use crate::junk::{self, Junk};
use crate::readstate::{Seen, append, load, normal, now_s, record, stat, state_file, valid};
use crate::readview::{BIG_LINES, cut_view};
use regex::Regex;
use serde_json::{Map, Value, json};
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

const MIN_STUB_CHARS: usize = 600;
const BIG_CHARS: usize = 12_000;
const JUNK_CHARS: usize = 2_000;
const GARBLED_SAMPLE: usize = 4_000;

static CAT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*(?:rtk\s+)?cat\s+(?:-n\s+)?("[^"]+"|'[^']+'|[^\s|;&<>$`*?()"']+)\s*$"#).expect("cat pattern")
});
static MSYS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^/([a-zA-Z])/(.*)$").expect("msys pattern"));
fn off(state: &Path) -> bool {
    state.join("reads-off").exists()
}

fn junk_off(state: &Path) -> bool {
    state.join("junk-off").exists()
}

fn sid(event: &Value) -> Option<&str> {
    event["session_id"].as_str().filter(|s| !s.is_empty())
}

fn edited_path(event: &Value) -> Option<&str> {
    let input = &event["tool_input"];
    input["file_path"].as_str().or_else(|| input["notebook_path"].as_str())
}

pub fn forget(state: &Path, event: &Value) {
    let (Some(sid), Some(path)) = (sid(event), edited_path(event)) else { return };
    let path = normal(path);
    if valid(&path) {
        append(&state_file(state, sid), &[format!("E\t{path}")]);
    }
}

pub fn other_event(state: &Path, event: &Value) -> Option<String> {
    let sid = sid(event)?;
    let file = state_file(state, sid);
    match event["hook_event_name"].as_str()? {
        "PreCompact" => {
            if file.exists() {
                append(&file, &["C".to_string()]);
            }
            None
        }
        "PreToolUse" if event["tool_name"].as_str() == Some("Write") && !off(state) => {
            let raw = event["tool_input"]["file_path"].as_str()?;
            if !fs::metadata(raw).is_ok_and(|m| m.is_file()) {
                return None;
            }
            let path = normal(raw);
            let table = load(&file);
            let known = table.edited.contains(&path) || table.seen.keys().any(|k| k.0 == path);
            if !known {
                return None;
            }
            let text = format!(
                "wisply: {raw} already exists and you read or changed it in this session. Edit sends only the changed lines, Write sends the whole file again; use Edit unless you really replace everything."
            );
            Some(json!({"hookSpecificOutput": {"hookEventName": "PreToolUse", "additionalContext": text}}).to_string())
        }
        _ => None,
    }
}

enum Verdict {
    Pass,
    Stub(String, String),
    Cut(String, String),
    Junk(String, Option<String>),
}

struct View<'a> {
    state: &'a Path,
    sid: &'a str,
    path: &'a str,
    start: u64,
    count: u64,
    content: &'a str,
    whole: bool,
    asked: bool,
    total: u64,
}

fn quick_hash(content: &str) -> u64 {
    crate::hook::quick(&[content]) ^ content.len() as u64
}

fn keep(state: &Path, path: &str, content: &str) -> Option<String> {
    let name = format!("hr-{:010x}", crate::hook::quick(&[path, content]) >> 24);
    let dir = state.join("out");
    let file = dir.join(format!("{name}.txt"));
    if !file.exists() {
        let _ = fs::create_dir_all(&dir);
        fs::write(&file, content).ok()?;
    }
    Some(name)
}

fn ago(secs: u64) -> String {
    match secs {
        0..=59 => format!("{secs} s"),
        60..=3599 => format!("{} min", secs / 60),
        _ => format!("{} h", secs / 3600),
    }
}

fn file_name(path: &str) -> &str {
    path.rsplit(['/', '\\']).next().unwrap_or(path)
}

fn garbled(content: &str) -> bool {
    content
        .chars()
        .take(GARBLED_SAMPLE)
        .any(|c| c == char::REPLACEMENT_CHARACTER || (c.is_control() && !c.is_whitespace()))
}

fn no_text(path: &str, kind: Junk, size: u64) -> String {
    let name = file_name(path);
    let list = junk::inside(path).map(|tool| format!(" Its file list: {tool} {name}")).unwrap_or_default();
    format!("[wisply] {name} is {} ({}) - nothing to read as text.{list}", kind.what(), junk::size(size))
}

fn rarely_read(v: &View, kind: Junk, name: &str) -> String {
    let tokens = v.content.len() / 4;
    let (lines, back) = if v.total > v.count {
        (format!("{} lines, its first {} are ~{tokens} tokens", v.total, v.count), "Those lines")
    } else {
        (format!("{} lines, ~{tokens} tokens", v.count), "Whole text")
    };
    format!(
        "[wisply] {} is {} ({lines}), rarely worth reading whole. For one part: Read with offset/limit or Grep. {back}: Read it again, or wisply more {name}.",
        file_name(v.path),
        kind.what()
    )
}

fn junk_kind(v: &View, size: u64) -> Result<Option<Junk>, Verdict> {
    if !v.asked || v.content.len() <= JUNK_CHARS || junk_off(v.state) {
        return Ok(None);
    }
    match junk::of(v.path) {
        Some(kind) if kind.text() => Ok(Some(kind)),
        Some(kind) if garbled(v.content) => Err(Verdict::Junk(no_text(v.path, kind, size), None)),
        _ => Ok(None),
    }
}

fn decide(v: &View) -> Verdict {
    let Some((size, mtime)) = stat(v.path) else { return Verdict::Pass };
    let path = normal(v.path);
    if !valid(&path) {
        return Verdict::Pass;
    }
    let kind = match junk_kind(v, size) {
        Ok(kind) => kind,
        Err(replaced) => return replaced,
    };
    let hash = quick_hash(v.content);
    let file = state_file(v.state, v.sid);
    let table = load(&file);
    let key = (path.clone(), v.start, v.count);
    let now = now_s();
    let fresh = |cut: bool, at: u64| Seen { size, mtime, hash, at, cut };
    if let Some(seen) = table.seen.get(&key)
        && seen.hash == hash
        && seen.size == size
        && seen.mtime == mtime
    {
        if seen.cut {
            append(&file, &[record(&path, v.start, v.count, &fresh(false, now))]);
            return Verdict::Pass;
        }
        if v.content.len() < MIN_STUB_CHARS {
            return Verdict::Pass;
        }
        let Some(name) = keep(v.state, &path, v.content) else { return Verdict::Pass };
        let last = v.start + v.count.saturating_sub(1);
        let text = format!(
            "[wisply] unchanged since your read {} ago, lines {}-{}; wisply more {name} for the text",
            ago(now.saturating_sub(seen.at)),
            v.start,
            last
        );
        return Verdict::Stub(text, name);
    }
    let untouched = !table.edited.contains(&path);
    if let Some(kind) = kind.filter(|_| untouched)
        && let Some(name) = keep(v.state, &path, v.content)
    {
        append(&file, &[record(&path, v.start, v.count, &fresh(true, now))]);
        return Verdict::Junk(rarely_read(v, kind, &name), Some(name));
    }
    let big = v.whole && untouched && v.content.len() > BIG_CHARS && v.count as usize >= BIG_LINES;
    if big
        && let Some(name) = keep(v.state, &path, v.content)
        && let Some(view) = cut_view(v.content, &name)
    {
        append(&file, &[record(&path, v.start, v.count, &fresh(true, now))]);
        return Verdict::Cut(view, name);
    }
    append(&file, &[record(&path, v.start, v.count, &fresh(false, now))]);
    Verdict::Pass
}

pub fn post_tool(state: &Path, event: &mut Value, entry: &mut Map<String, Value>) -> Option<String> {
    if off(state) {
        return None;
    }
    let sid = sid(event)?.to_string();
    let tool = event["tool_name"].as_str()?.to_string();
    let (replacement, chars, kind) = match tool.as_str() {
        "Read" => (read(state, event, &sid)?, event["tool_response"]["file"]["content"].as_str()?.len(), "read"),
        "Bash" => (bash(state, event, &sid)?, event["tool_response"]["stdout"].as_str()?.len(), "view"),
        _ => return None,
    };
    let (text, name, act) = match replacement {
        Verdict::Stub(t, n) => (t, Some(n), "stub"),
        Verdict::Cut(t, n) => (t, Some(n), "cut"),
        Verdict::Junk(t, n) => (t, n, "junk"),
        Verdict::Pass => return None,
    };
    entry.insert("act".into(), json!(act));
    entry.insert("kind".into(), json!(kind));
    entry.insert("in".into(), json!(chars));
    entry.insert("out".into(), json!(text.len()));
    if let Some(name) = name {
        entry.insert("ref".into(), json!(name));
    }
    let mut updated = event["tool_response"].take();
    if tool == "Read" {
        updated["file"]["content"] = Value::String(text.clone());
        updated["file"]["numLines"] = json!(text.lines().count());
    } else {
        updated["stdout"] = Value::String(text);
    }
    Some(json!({"hookSpecificOutput": {"hookEventName": "PostToolUse", "updatedToolOutput": updated}}).to_string())
}

fn read(state: &Path, event: &Value, sid: &str) -> Option<Verdict> {
    let resp = &event["tool_response"];
    if resp["type"].as_str() != Some("text") {
        return None;
    }
    let file = resp.get("file")?;
    let content = file["content"].as_str()?;
    let path = file["filePath"].as_str().or_else(|| event["tool_input"]["file_path"].as_str())?;
    let (start, count) = (file["startLine"].as_u64()?, file["numLines"].as_u64()?);
    let ranged = !event["tool_input"]["offset"].is_null() || !event["tool_input"]["limit"].is_null();
    let total = file["totalLines"].as_u64().unwrap_or(count);
    let asked = !ranged && start == 1;
    let whole = asked && total == count;
    Some(decide(&View { state, sid, path, start, count, content, whole, asked, total }))
}

fn resolve(token: &str, cwd: &str) -> String {
    let bare = token.trim_matches(['"', '\'']);
    let bare = match MSYS.captures(bare) {
        Some(c) if cfg!(windows) => format!("{}:/{}", &c[1], &c[2]),
        _ => bare.to_string(),
    };
    let absolute = Path::new(&bare).is_absolute() || bare.starts_with('/');
    if absolute || cwd.is_empty() { bare } else { format!("{}/{bare}", cwd.trim_end_matches(['/', '\\'])) }
}

fn bash(state: &Path, event: &Value, sid: &str) -> Option<Verdict> {
    let cmd = event["tool_input"]["command"].as_str()?;
    let token = CAT.captures(cmd)?.get(1)?.as_str().to_string();
    let stdout = event["tool_response"]["stdout"].as_str()?;
    if event["tool_response"]["isImage"].as_bool() == Some(true) || stdout.is_empty() {
        return None;
    }
    let path = resolve(&token, event["cwd"].as_str().unwrap_or_default());
    let count = stdout.lines().count() as u64;
    let view =
        View { state, sid, path: &path, start: 1, count, content: stdout, whole: true, asked: true, total: count };
    Some(decide(&view))
}

#[cfg(test)]
#[path = "reads/tests.rs"]
mod tests;
