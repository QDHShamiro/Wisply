use crate::readstate::{Seen, append, load, normal, now_s, record, shrink, stat, state_file, valid};
use crate::readview::{BIG_LINES, cut_view};
use regex::Regex;
use serde_json::{Map, Value, json};
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

const MIN_STUB_CHARS: usize = 600;
const BIG_CHARS: usize = 12_000;

static CAT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^\s*(?:rtk\s+)?cat\s+(?:-n\s+)?("[^"]+"|'[^']+'|[^\s|;&<>$`*?()"']+)\s*$"#).expect("cat pattern")
});
static MSYS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^/([a-zA-Z])/(.*)$").expect("msys pattern"));
fn off(state: &Path) -> bool {
    state.join("reads-off").exists()
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
            let (table, _) = load(&file);
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
}

struct View<'a> {
    state: &'a Path,
    sid: &'a str,
    path: &'a str,
    start: u64,
    count: u64,
    content: &'a str,
    whole: bool,
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

fn decide(v: &View) -> Verdict {
    let Some((size, mtime)) = stat(v.path) else { return Verdict::Pass };
    let path = normal(v.path);
    if !valid(&path) {
        return Verdict::Pass;
    }
    let hash = quick_hash(v.content);
    let file = state_file(v.state, v.sid);
    let (table, lines) = load(&file);
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
    let big = v.whole && !table.edited.contains(&path) && v.content.len() > BIG_CHARS && v.count as usize >= BIG_LINES;
    if big
        && let Some(name) = keep(v.state, &path, v.content)
        && let Some(view) = cut_view(v.content, &name)
    {
        append(&file, &[record(&path, v.start, v.count, &fresh(true, now))]);
        shrink(&file, &table, lines + 1);
        return Verdict::Cut(view, name);
    }
    append(&file, &[record(&path, v.start, v.count, &fresh(false, now))]);
    append(&file, &[record(&path, v.start, v.count, &fresh(false, now))]);
    shrink(&file, &table, lines + 1);
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
        Verdict::Stub(t, n) => (t, n, "stub"),
        Verdict::Cut(t, n) => (t, n, "cut"),
        Verdict::Pass => return None,
    };
    entry.insert("act".into(), json!(act));
    entry.insert("kind".into(), json!(kind));
    entry.insert("in".into(), json!(chars));
    entry.insert("out".into(), json!(text.len()));
    entry.insert("ref".into(), json!(name));
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
    let whole = !ranged && start == 1 && file["totalLines"].as_u64() == Some(count);
    Some(decide(&View { state, sid, path, start, count, content, whole }))
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
    Some(decide(&View { state, sid, path: &path, start: 1, count, content: stdout, whole: true }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::{Duration, Instant};

    fn scratch(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("wisply-reads-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn source(name: &str, fns: usize) -> String {
        let mut s = format!("// module {name}\nuse std::fmt;\n\n");
        for i in 0..fns {
            s.push_str(&format!(
                "pub fn handler_{i}(input: &str) -> usize {{\n    let mut total = {i};\n    for c in input.chars() {{\n        total += c as usize;\n    }}\n    total\n}}\n\n"
            ));
        }
        s
    }

    fn read_event(sid: &str, path: &Path, content: &str, ranged: bool) -> Value {
        let n = content.lines().count();
        let input =
            if ranged { json!({"file_path": path, "offset": 1, "limit": n}) } else { json!({"file_path": path}) };
        json!({
            "session_id": sid, "tool_name": "Read", "tool_input": input,
            "tool_response": {"type": "text", "file": {"filePath": path, "content": content, "numLines": n, "startLine": 1, "totalLines": n}}
        })
    }

    fn run(state: &Path, mut event: Value) -> (Option<Value>, Map<String, Value>) {
        let mut entry = Map::new();
        let reply = post_tool(state, &mut event, &mut entry).map(|r| serde_json::from_str::<Value>(&r).unwrap());
        (reply, entry)
    }

    fn text_of(reply: &Value) -> String {
        reply["hookSpecificOutput"]["updatedToolOutput"]["file"]["content"].as_str().unwrap().to_string()
    }

    #[test]
    fn second_unchanged_read_becomes_a_stub() {
        let dir = scratch("stub");
        let file = dir.join("a.rs");
        let body = source("a", 8);
        fs::write(&file, &body).unwrap();
        let state = dir.join("headroom");
        assert!(run(&state, read_event("s1", &file, &body, true)).0.is_none());
        let (reply, entry) = run(&state, read_event("s1", &file, &body, true));
        let text = text_of(&reply.unwrap());
        assert!(text.contains("unchanged since your read"), "{text}");
        assert!(text.contains("wisply more hr-"));
        assert_eq!(entry["act"], "stub");
    }

    #[test]
    fn edit_clears_the_file_and_changed_content_never_stubs() {
        let dir = scratch("edit");
        let file = dir.join("b.rs");
        let body = source("b", 8);
        fs::write(&file, &body).unwrap();
        let state = dir.join("headroom");
        run(&state, read_event("s2", &file, &body, true));
        forget(&state, &json!({"session_id": "s2", "tool_name": "Edit", "tool_input": {"file_path": file}}));
        assert!(run(&state, read_event("s2", &file, &body, true)).0.is_none());
        let changed = body.replace("total = 1;", "total = 2;");
        fs::write(&file, &changed).unwrap();
        assert!(run(&state, read_event("s2", &file, &changed, true)).0.is_none());
        let same_again = run(&state, read_event("s2", &file, &changed, true)).0;
        assert!(same_again.is_some());
        fs::write(&file, &body).unwrap();
        assert!(run(&state, read_event("s2", &file, &body, true)).0.is_none());
    }

    #[test]
    fn compact_and_other_sessions_reset_the_table() {
        let dir = scratch("compact");
        let file = dir.join("c.rs");
        let body = source("c", 8);
        fs::write(&file, &body).unwrap();
        let state = dir.join("headroom");
        run(&state, read_event("s3", &file, &body, true));
        assert!(run(&state, read_event("other", &file, &body, true)).0.is_none());
        other_event(&state, &json!({"session_id": "s3", "hook_event_name": "PreCompact"}));
        assert!(run(&state, read_event("s3", &file, &body, true)).0.is_none());
    }

    #[test]
    fn big_whole_file_is_cut_once_then_given_in_full() {
        let dir = scratch("cut");
        let file = dir.join("d.rs");
        let body = source("d", 110);
        fs::write(&file, &body).unwrap();
        let state = dir.join("headroom");
        let (reply, entry) = run(&state, read_event("s4", &file, &body, false));
        let text = text_of(&reply.unwrap());
        assert!(text.contains("--- outline") && text.contains("fn handler_20"), "{text}");
        assert!(text.len() < body.len() / 2);
        assert_eq!(entry["act"], "cut");
        assert!(run(&state, read_event("s4", &file, &body, false)).0.is_none());
        let (stub, _) = run(&state, read_event("s4", &file, &body, false));
        assert!(text_of(&stub.unwrap()).contains("unchanged"));
    }

    #[test]
    fn ranges_small_files_and_edited_files_are_not_cut() {
        let dir = scratch("nocut");
        let file = dir.join("e.rs");
        let body = source("e", 110);
        fs::write(&file, &body).unwrap();
        let state = dir.join("headroom");
        assert!(run(&state, read_event("s5", &file, &body, true)).0.is_none());
        let small = dir.join("f.rs");
        let tiny = source("f", 2);
        fs::write(&small, &tiny).unwrap();
        assert!(run(&state, read_event("s5", &small, &tiny, false)).0.is_none());
        forget(&state, &json!({"session_id": "s6", "tool_name": "Write", "tool_input": {"file_path": file}}));
        assert!(run(&state, read_event("s6", &file, &body, false)).0.is_none());
    }

    #[test]
    fn cat_of_a_whole_file_is_cut_and_stubbed() {
        let dir = scratch("cat");
        let file = dir.join("g.rs");
        let body = source("g", 110);
        fs::write(&file, &body).unwrap();
        let state = dir.join("headroom");
        let event = json!({
            "session_id": "s7", "tool_name": "Bash", "cwd": dir, "tool_input": {"command": "cat g.rs"},
            "tool_response": {"stdout": body, "stderr": "", "interrupted": false, "isImage": false}
        });
        let (reply, entry) = run(&state, event.clone());
        let out = reply.unwrap()["hookSpecificOutput"]["updatedToolOutput"]["stdout"].as_str().unwrap().to_string();
        assert!(out.contains("--- outline"));
        assert_eq!(entry["kind"], "view");
        let piped = json!({
            "session_id": "s7", "tool_name": "Bash", "cwd": dir, "tool_input": {"command": "cat g.rs | head -5"},
            "tool_response": {"stdout": body, "stderr": "", "interrupted": false, "isImage": false}
        });
        assert!(run(&state, piped).0.is_none());
    }

    #[test]
    fn write_over_a_known_file_gets_a_hint_only() {
        let dir = scratch("write");
        let file = dir.join("h.rs");
        let body = source("h", 8);
        fs::write(&file, &body).unwrap();
        let state = dir.join("headroom");
        let pre = json!({"session_id": "s8", "hook_event_name": "PreToolUse", "tool_name": "Write", "tool_input": {"file_path": file}});
        assert!(other_event(&state, &pre).is_none());
        run(&state, read_event("s8", &file, &body, true));
        let hint: Value = serde_json::from_str(&other_event(&state, &pre).unwrap()).unwrap();
        assert!(hint["hookSpecificOutput"]["additionalContext"].as_str().unwrap().contains("use Edit"));
        assert!(hint["hookSpecificOutput"].get("permissionDecision").is_none());
        let fresh = json!({"session_id": "s8", "hook_event_name": "PreToolUse", "tool_name": "Write", "tool_input": {"file_path": dir.join("new.rs")}});
        assert!(other_event(&state, &fresh).is_none());
    }

    #[test]
    fn rebuilt_sessions_save_tokens_within_the_time_budget() {
        let dir = scratch("measure");
        let state = dir.join("headroom");
        let names = ["engine", "store", "router"];
        let bodies: Vec<(PathBuf, String)> = names
            .iter()
            .map(|n| {
                let p = dir.join(format!("{n}.rs"));
                let b = source(n, 110);
                fs::write(&p, &b).unwrap();
                (p, b)
            })
            .collect();
        let (mut before, mut after, mut times) = (0usize, 0usize, Vec::<Duration>::new());
        let mut go = |event: Value, content: &str| {
            before += content.len() / 4;
            let t = Instant::now();
            let (reply, _) = run(&state, event);
            times.push(t.elapsed());
            after += reply.map_or(content.len() / 4, |r| text_of(&r).len() / 4);
        };
        for (p, b) in &bodies {
            go(read_event("m", p, b, false), b);
            go(read_event("m", p, b, false), b);
            go(read_event("m", p, b, false), b);
        }
        let (p, b) = &bodies[0];
        forget(&state, &json!({"session_id": "m", "tool_name": "Edit", "tool_input": {"file_path": p}}));
        let edited = b.replace("total = 7;", "total = 700;");
        fs::write(p, &edited).unwrap();
        go(read_event("m", p, &edited, false), &edited);
        go(read_event("m", p, &edited, false), &edited);
        times.sort();
        let (median, worst) = (times[times.len() / 2], times[times.len() - 1]);
        println!(
            "tokens before {before} after {after} saved {}% median call {median:?} worst call {worst:?} over {} calls",
            100 - after * 100 / before,
            times.len()
        );
        assert!(after * 10 < before * 6);
        if !cfg!(debug_assertions) {
            assert!(median < Duration::from_millis(2), "{median:?}");
        }
    }
}
