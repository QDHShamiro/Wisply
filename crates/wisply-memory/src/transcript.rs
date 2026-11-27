use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde_json::Value;

use super::asked::asked;
use super::project::{fold, project_of};
use super::text::scrub;

pub const CHUNK_CHARS: usize = 24_000;
const CHUNKS: usize = 8;
const TEXT_CHARS: usize = 1500;
const TOOL_CHARS: usize = 240;
const ARG_CHARS: usize = 160;
const ARG_KEYS: &[&str] = &["file_path", "path", "pattern", "skill", "url", "query", "prompt"];
const EDITS: &[&str] = &["Edit", "Write", "NotebookEdit"];
const EDIT_WEIGHT: usize = 3;
const LEAN_USER: usize = 700;
const LEAN_CLAUDE: usize = 320;
const LEAN_ERROR: usize = 160;
const LEAN_MIN_CLAUDE: usize = 40;
const LEAN_CMD: usize = 120;
const KEEP_CMDS: &[&str] =
    &["git commit", "git merge", "git push", "gh pr", "git revert", "git reset", "git checkout -b"];

pub(super) static REMINDER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?s)<system-reminder>.*?</system-reminder>|<local-command-caveat>.*?</local-command-caveat>")
        .expect("reminder pattern")
});
pub(super) static TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"</?[a-zA-Z][\w-]*(?:\s[^<>]*)?>").expect("tag pattern"));
static FILLER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(?:now |ok,? |okay,? |sure,? |alright,? |good,? |great,? )*(?:let me|let's|i'll|i will|i'm going to|lass mich|ich werde|jetzt|gleich|first,? )")
        .expect("filler pattern")
});
static DECISION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)because|root cause|instead|decid|chose|weil|ursache|stattdessen|entschied|fix|bug|found|finding|error|fehler|forbidden|never|nie ")
        .expect("decision pattern")
});
static FENCE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?s)```.*?(?:```|$)").expect("fence pattern"));
static SERVER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"--(?:profile|server)[ =]+([A-Za-z0-9][\w.-]*)").expect("server pattern"));
static ABSOLUTE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:[A-Za-z]:[\\/]|/)").expect("path pattern"));

#[derive(Debug, Default)]
pub struct Condensed {
    pub text: String,
    pub end: u64,
    pub title: String,
    pub paths: Vec<String>,
    pub servers: Vec<String>,
    pub more: bool,
    pub users: usize,
    pub edits: usize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Style {
    Full,
    Lean,
}

pub fn cut(text: &str, n: usize) -> String {
    let s = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if s.chars().count() <= n { s } else { format!("{} …", s.chars().take(n).collect::<String>()) }
}

pub fn clean(text: &str) -> String {
    TAG.replace_all(&REMINDER.replace_all(text, ""), "").trim().to_string()
}

pub(super) fn user_text(text: &str, style: Style) -> String {
    match style {
        Style::Full => cut(text, TEXT_CHARS),
        Style::Lean => cut(&FENCE.replace_all(text, "[code]"), LEAN_USER),
    }
}

fn user_line(text: &str, style: Style) -> String {
    let text = user_text(text, style);
    if text.is_empty() { String::new() } else { format!("USER: {text}") }
}

fn shown(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

pub(super) fn truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
        Value::Number(n) => n.as_f64().is_some_and(|x| x != 0.0),
    }
}

fn tool_line(name: &str, input: &Value) -> String {
    if name == "Bash" || name == "PowerShell" {
        return format!("{name}: {}", cut(&input.get("command").map(shown).unwrap_or_default(), TOOL_CHARS));
    }
    if let Some(v) = ARG_KEYS.iter().find_map(|k| input.get(*k).filter(|v| truthy(v))) {
        return format!("{name} {}", cut(&shown(v), ARG_CHARS));
    }
    format!("{name} {}", cut(&input.to_string(), ARG_CHARS))
}

fn result_text(c: &Value) -> String {
    match c.get("content") {
        Some(Value::Array(items)) => {
            items.iter().filter_map(|x| x.get("text").and_then(Value::as_str)).collect::<Vec<_>>().join(" ")
        }
        Some(Value::Null) | None => String::new(),
        Some(v) => shown(v),
    }
}

fn said(text: &str, lean: bool) -> String {
    let mut text = clean(text);
    if lean {
        text = FENCE.replace_all(&text, "[code]").into_owned();
    }
    let skip =
        lean && (text.chars().count() < LEAN_MIN_CLAUDE || (FILLER.is_match(&text) && !DECISION.is_match(&text)));
    if text.is_empty() || skip {
        String::new()
    } else {
        format!("CLAUDE: {}", cut(&text, if lean { LEAN_CLAUDE } else { TEXT_CHARS }))
    }
}

fn used(c: &Value, paths: &mut Vec<String>, servers: &mut Vec<String>, lean: bool) -> String {
    let name = c.get("name").and_then(Value::as_str).unwrap_or("");
    let input = c.get("input").cloned().unwrap_or(Value::Null);
    let file = ["file_path", "notebook_path", "path"].iter().find_map(|k| input.get(*k)).and_then(Value::as_str);
    if let Some(f) = file.filter(|f| ABSOLUTE.is_match(f)) {
        let times = if EDITS.contains(&name) { EDIT_WEIGHT } else { 1 };
        paths.extend(std::iter::repeat_n(f.to_string(), times));
    }
    if name == "Bash" || name == "PowerShell" {
        let cmd = input.get("command").map(shown).unwrap_or_default();
        servers.extend(SERVER.captures_iter(&cmd).map(|m| m[1].to_string()));
    }
    if !lean {
        return format!("TOOL {}", tool_line(name, &input));
    }
    if EDITS.contains(&name) {
        return file.map(|f| format!("EDIT {}", cut(f, ARG_CHARS))).unwrap_or_default();
    }
    if name != "Bash" && name != "PowerShell" {
        return String::new();
    }
    let cmd = input.get("command").map(shown).unwrap_or_default();
    let kept = KEEP_CMDS.iter().filter_map(|k| cmd.find(k)).min();
    kept.map(|at| format!("RAN {}", cut(&cmd[at..], LEAN_CMD))).unwrap_or_default()
}

fn pieces(d: &Value, kind: &str, paths: &mut Vec<String>, servers: &mut Vec<String>, style: Style) -> Vec<String> {
    let lean = style == Style::Lean;
    let content = d.get("message").and_then(|m| m.get("content"));
    let mut out = vec![user_line(&asked(d), style)];
    if let Some(text) = content.and_then(Value::as_str).filter(|_| kind == "assistant") {
        out.push(said(text, lean));
    }
    for c in content.and_then(Value::as_array).into_iter().flatten() {
        out.push(match c.get("type").and_then(Value::as_str) {
            Some("text") if kind == "assistant" => said(c.get("text").and_then(Value::as_str).unwrap_or(""), lean),
            Some("tool_use") => used(c, paths, servers, lean),
            Some("tool_result") if c.get("is_error").is_some_and(truthy) => {
                format!("ERROR: {}", cut(&result_text(c), if lean { LEAN_ERROR } else { TOOL_CHARS }))
            }
            _ => String::new(),
        });
    }
    out.retain(|piece| !piece.is_empty());
    out
}

fn turn(d: &Value, kind: &str, found: &[String]) -> bool {
    let text = |v: Option<&Value>| !clean(v.and_then(Value::as_str).unwrap_or("")).is_empty();
    match (kind, d.get("message").and_then(|m| m.get("content"))) {
        ("attachment", _) => !found.is_empty(),
        ("user", Some(Value::Array(items))) => {
            items.iter().any(|c| c.get("type").and_then(Value::as_str) == Some("text") && text(c.get("text")))
        }
        ("user", content) => text(content),
        _ => false,
    }
}

pub fn condense(path: &Path, offset: u64, limit: Option<usize>) -> std::io::Result<Condensed> {
    condense_as(path, offset, limit, Style::Lean)
}

pub fn condense_as(path: &Path, offset: u64, limit: Option<usize>, style: Style) -> std::io::Result<Condensed> {
    let limit = limit.unwrap_or(CHUNK_CHARS * CHUNKS);
    let mut file = BufReader::new(File::open(path)?);
    file.seek(SeekFrom::Start(offset))?;
    let mut out = Condensed { end: offset, ..Condensed::default() };
    let (mut parts, mut size, mut raw) = (Vec::new(), 0usize, Vec::new());
    loop {
        raw.clear();
        if file.read_until(b'\n', &mut raw)? == 0 || !raw.ends_with(b"\n") {
            break;
        }
        if size >= limit {
            out.more = true;
            break;
        }
        out.end += raw.len() as u64;
        let Ok(d) = serde_json::from_slice::<Value>(&raw) else { continue };
        let kind = d.get("type").and_then(Value::as_str).unwrap_or("");
        if kind == "ai-title" {
            if let Some(t) = d.get("aiTitle").and_then(Value::as_str).filter(|t| !t.is_empty()) {
                out.title = t.to_string();
            }
            continue;
        }
        let side = ["isMeta", "isSidechain"].iter().any(|k| d.get(*k).is_some_and(truthy));
        if !["user", "assistant", "attachment"].contains(&kind) || side {
            continue;
        }
        let found = pieces(&d, kind, &mut out.paths, &mut out.servers, style);
        out.users += usize::from(turn(&d, kind, &found));
        for piece in found {
            if style == Style::Lean && parts.last() == Some(&piece) {
                continue;
            }
            if style == Style::Lean && piece.starts_with("EDIT ") && parts.contains(&piece) {
                continue;
            }
            out.edits += usize::from(piece.starts_with("EDIT "));
            size += piece.chars().count() + 1;
            parts.push(piece);
        }
    }
    out.text = scrub(&parts.join("\n"));
    Ok(out)
}

pub fn chunks(text: &str) -> Vec<String> {
    chunks_of(text, CHUNK_CHARS)
}

pub fn chunks_of(text: &str, limit: usize) -> Vec<String> {
    let limit = limit.max(1);
    let (mut out, mut cur, mut size) = (Vec::new(), Vec::<String>::new(), 0usize);
    let pieces = text.split('\n').flat_map(|line| {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() <= limit {
            vec![line.to_string()]
        } else {
            chars.chunks(limit).map(|c| c.iter().collect()).collect()
        }
    });
    for line in pieces {
        let n = line.chars().count();
        if size + n > limit && !cur.is_empty() {
            out.push(cur.join("\n"));
            cur.clear();
            size = 0;
        }
        cur.push(line);
        size += n + 1;
    }
    if !cur.is_empty() {
        out.push(cur.join("\n"));
    }
    out
}

fn vote(seen: &mut Vec<(String, usize)>, name: String) {
    if name.is_empty() {
        return;
    }
    match seen.iter_mut().find(|(n, _)| *n == name) {
        Some((_, c)) => *c += 1,
        None => seen.push((name, 1)),
    }
}

fn winner(seen: Vec<(String, usize)>) -> String {
    let best = seen.iter().map(|(_, c)| *c).max().unwrap_or(0);
    seen.into_iter().find(|(_, c)| *c == best).map(|(n, _)| n).unwrap_or_default()
}

pub fn project_of_files(paths: &[String]) -> String {
    project_of_work(paths, &[])
}

pub fn project_of_work(paths: &[String], servers: &[String]) -> String {
    let mut seen: Vec<(String, usize)> = Vec::new();
    for f in paths {
        let Some(parent) = Path::new(f).parent().filter(|_| Path::new(f).is_absolute()) else { continue };
        vote(&mut seen, project_of(&parent.to_string_lossy()));
    }
    for name in servers {
        vote(&mut seen, fold(name.clone()));
    }
    winner(seen)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn transcripts_are_condensed_like_2x() {
        let dir = std::env::temp_dir().join(format!("ltm-transcript-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let lines = [
            r#"{"type":"ai-title","aiTitle":"Cart fix"}"#,
            r#"{"type":"user","message":{"content":"fix the <b>cart</b><system-reminder>noise</system-reminder>"}}"#,
            r#"{"type":"assistant","message":{"content":[{"type":"text","text":"On it"},{"type":"tool_use","name":"Edit","input":{"file_path":"C:/x/a.rs"}}]}}"#,
            r#"{"type":"user","isMeta":true,"message":{"content":"skip"}}"#,
            r#"{"type":"user","message":{"content":[{"type":"tool_result","is_error":true,"content":[{"type":"text","text":"boom   token: abcdef123"}]}]}}"#,
            r#"{"type":"assistant","message":{"content":[{"type":"tool_use","name":"Bash","input":{"command":"cargo   test"}}]}}"#,
        ];
        let body = lines.join("\n") + "\n{\"type\":\"user\"";
        let path = dir.join("t.jsonl");
        fs::write(&path, &body).unwrap();
        let c = condense_as(&path, 0, None, Style::Full).unwrap();
        let want = "USER: fix the cart\nCLAUDE: On it\nTOOL Edit C:/x/a.rs\nERROR: boom token: [secret]\nTOOL Bash: cargo test";
        assert_eq!(c.text, want);
        assert_eq!((c.title.as_str(), c.more, c.paths.len()), ("Cart fix", false, 3));
        assert_eq!(c.end as usize, body.len() - "{\"type\":\"user\"".len());
        let again = condense(&path, c.end, None).unwrap();
        assert!(again.text.is_empty() && again.end == c.end);
        let small = condense(&path, 0, Some(10)).unwrap();
        assert!(small.more && small.text.starts_with("USER: fix the cart") && small.end < c.end);
        assert_eq!(cut("a  b\nc", 10), "a b c");
        assert_eq!(cut("abcdef", 3), "abc …");
        let long = format!("{}\n{}", "a".repeat(CHUNK_CHARS - 5), "b".repeat(10));
        assert_eq!(chunks(&long).len(), 2);
        assert_eq!(chunks("x\ny"), ["x\ny"]);
        assert_eq!(chunks_of("abcdefg\nhi", 3), ["abc", "def", "g", "hi"]);
        assert_eq!(chunks_of("ab\ncd\nefgh", 6), ["ab\ncd", "efgh"]);
        assert!(chunks_of(&"ü".repeat(25), 10).iter().all(|p| p.chars().count() <= 10));
        let lean = condense(&path, 0, None).unwrap();
        assert_eq!(
            lean.text,
            "USER: fix the cart
EDIT C:/x/a.rs
ERROR: boom token: [secret]"
        );
        assert_eq!((lean.users, lean.edits), (1, 1));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn shell_commands_name_their_server_profile() {
        let dir = std::env::temp_dir().join(format!("ltm-servers-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let lines = [
            r#"{"type":"assistant","message":{"content":[{"type":"tool_use","name":"Bash","input":{"command":"python sftp.py get x.yml --profile alpha-srv"}}]}}"#,
            r#"{"type":"assistant","message":{"content":[{"type":"tool_use","name":"Bash","input":{"command":"sftp.py worker status --server=alpha-srv"}}]}}"#,
            r#"{"type":"assistant","message":{"content":[{"type":"tool_use","name":"Read","input":{"file_path":"C:/x/a.rs","command":"--profile nope"}}]}}"#,
        ];
        let path = dir.join("t.jsonl");
        fs::write(
            &path,
            lines.join(
                "
",
            ) + "
",
        )
        .unwrap();
        for style in [Style::Full, Style::Lean] {
            let c = condense_as(&path, 0, None, style).unwrap();
            assert_eq!(c.servers, vec!["alpha-srv".to_string(), "alpha-srv".to_string()]);
        }
        assert_eq!(project_of_work(&[], &["alpha-srv".to_string()]), "alpha-srv");
        assert_eq!(project_of_work(&[], &[]), "");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    #[ignore]
    fn measure_transcripts() {
        let list = std::env::var("WISPLY_MEASURE").unwrap_or_default();
        for f in list.split(',').filter(|f| !f.is_empty()) {
            let path = Path::new(f);
            let name = path.file_name().map(|n| n.to_string_lossy().chars().take(8).collect::<String>());
            let t = std::time::Instant::now();
            let full = match condense_as(path, 0, None, Style::Full) {
                Ok(c) => c,
                Err(e) => {
                    println!("MEASURE skip {f}: {e}");
                    continue;
                }
            };
            let full_ms = t.elapsed().as_millis();
            let span = std::env::temp_dir().join(format!("measure-span-{}.jsonl", std::process::id()));
            let mut head = Vec::new();
            std::io::Read::read_to_end(&mut std::io::Read::take(File::open(path).unwrap(), full.end), &mut head)
                .unwrap();
            std::fs::write(&span, head).unwrap();
            let t = std::time::Instant::now();
            let lean = condense_as(&span, 0, None, Style::Lean).unwrap();
            let lean_ms = t.elapsed().as_millis();
            let _ = std::fs::remove_file(&span);
            if std::env::var("WISPLY_SHOW").is_ok() {
                println!("SHOW {}", lean.text.chars().take(1400).collect::<String>());
            }
            let row = [(&full, full_ms), (&lean, lean_ms)].map(|(c, ms)| {
                format!(
                    "{} chars ~{} tok {}ms users={} edits={}",
                    c.text.chars().count(),
                    c.text.len() / 4,
                    ms,
                    c.users,
                    c.edits
                )
            });
            println!("MEASURE {} | full {} | lean {}", name.unwrap_or_default(), row[0], row[1]);
        }
    }
}
