use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

use super::Ctx;
use super::grade::{Tally, week};

const DEPTH: usize = 5;
const MAX_LINE: usize = 3_000_000;
const MAX_FILE: u64 = 200_000_000;
const CODEX_CALLS: [&str; 3] = ["function_call", "custom_tool_call", "local_shell_call"];
const CODEX_FAILS: [&str; 4] = ["execution error", "script failed", "script terminated", "verification failed"];
const EXIT: &str = "exit code:";
const CHATS: &str = "chats";

struct Source {
    id: &'static str,
    roots: &'static [&'static [&'static str]],
    codex: bool,
}

const SOURCES: [Source; 3] = [
    Source { id: "codex", roots: &[&[".codex", "sessions"]], codex: true },
    Source { id: "gemini", roots: &[&[".gemini", "tmp"]], codex: false },
    Source { id: "qwen", roots: &[&[".qwen", "tmp"], &[".qwen", "projects"]], codex: false },
];

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

pub fn modified(path: &Path) -> Option<u64> {
    let meta = fs::metadata(path).ok()?;
    (meta.len() <= MAX_FILE).then_some(())?;
    Some(meta.modified().ok()?.duration_since(UNIX_EPOCH).ok()?.as_secs())
}

fn session_file(path: &Path, codex: bool) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if codex {
        return name.ends_with(".jsonl") || name.ends_with(".jsonl.zst");
    }
    let in_chats = path.parent().and_then(Path::file_name).is_some_and(|n| n == CHATS);
    in_chats && (name.ends_with(".json") || name.ends_with(".jsonl"))
}

fn walk(dir: &Path, depth: usize, codex: bool, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            if depth > 0 {
                walk(&path, depth - 1, codex, out);
            }
        } else if session_file(&path, codex) {
            out.push(path);
        }
    }
}

pub fn open(path: &Path) -> Option<Box<dyn Read>> {
    let file = File::open(path).ok()?;
    if path.extension().is_some_and(|e| e == "zst") {
        return Some(Box::new(zstd::stream::read::Decoder::new(file).ok()?.take(MAX_FILE)));
    }
    Some(Box::new(file))
}

fn output_text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Array(items) => {
            items.iter().filter_map(|i| i.get("text").and_then(Value::as_str)).collect::<Vec<_>>().join("\n")
        }
        _ => String::new(),
    }
}

pub fn codex_failed(text: &str) -> Option<String> {
    if text.trim_start().starts_with('{')
        && let Ok(inner) = serde_json::from_str::<Value>(text)
    {
        let code = inner.get("metadata").and_then(|m| m.get("exit_code")).and_then(Value::as_i64);
        let body = inner.get("output").map(output_text).unwrap_or_default();
        return (code.is_some_and(|c| c != 0)).then_some(body);
    }
    let first = text.lines().next().unwrap_or("").trim().to_lowercase();
    let code = first.strip_prefix(EXIT).and_then(|c| c.trim().parse::<i64>().ok());
    let failed = code.is_some_and(|c| c != 0) || CODEX_FAILS.iter().any(|f| first.contains(f));
    failed.then(|| text.to_string())
}

fn codex_line(line: &str, tally: &mut Tally, tokens: &mut u64) {
    let Ok(entry) = serde_json::from_str::<Value>(line) else { return };
    let Some(p) = entry.get("payload") else { return };
    let kind = p.get("type").and_then(Value::as_str).unwrap_or("");
    if CODEX_CALLS.contains(&kind) {
        tally.calls += 1;
    } else if kind.ends_with("_output") && CODEX_CALLS.iter().any(|c| kind.starts_with(c)) {
        if let Some(text) = codex_failed(&p.get("output").map(output_text).unwrap_or_default()) {
            tally.fail(&text);
        }
    } else if kind == "token_count" {
        let total = p.pointer("/info/total_token_usage/total_tokens").and_then(Value::as_u64).unwrap_or(0);
        *tokens = (*tokens).max(total);
    }
}

fn codex(path: &Path) -> Tally {
    let mut tally = Tally::default();
    let Some(reader) = open(path) else { return tally };
    let mut tokens = 0;
    for line in BufReader::new(reader).lines().map_while(Result::ok) {
        if line.len() <= MAX_LINE && (line.contains("call") || line.contains("token_count")) {
            codex_line(&line, &mut tally, &mut tokens);
        }
    }
    tally.tokens = tokens;
    tally.sessions = usize::from(tally.calls > 0 || tokens > 0);
    tally
}

#[derive(Default)]
struct Chat {
    listed: Tally,
    parts: Tally,
    tokens: u64,
    seen: bool,
}

fn error_text(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Object(o) => o.get("message").and_then(Value::as_str).map_or_else(|| v.to_string(), str::to_string),
        _ => v.to_string(),
    }
}

fn visit(v: &Value, chat: &mut Chat) {
    match v {
        Value::Array(items) => items.iter().for_each(|i| visit(i, chat)),
        Value::Object(o) => {
            if let Some(calls) = o.get("toolCalls").and_then(Value::as_array) {
                for call in calls {
                    chat.listed.calls += 1;
                    if call.get("status").and_then(Value::as_str) == Some("error") {
                        let shown = call.get("resultDisplay").or_else(|| call.get("result")).unwrap_or(&Value::Null);
                        chat.listed.fail(&error_text(shown));
                    }
                }
            }
            if o.contains_key("functionCall") {
                chat.parts.calls += 1;
            }
            if let Some(err) = o.get("functionResponse").and_then(|r| r.get("response")).and_then(|r| r.get("error")) {
                chat.parts.fail(&error_text(err));
            }
            let total = o.get("tokens").and_then(|t| t.get("total")).and_then(Value::as_u64);
            let usage = o.get("usageMetadata").and_then(|u| u.get("totalTokenCount")).and_then(Value::as_u64);
            chat.tokens += total.or(usage).unwrap_or(0);
            chat.seen |= o.contains_key("type") || o.contains_key("role");
            o.iter()
                .filter(|(k, _)| !matches!(k.as_str(), "toolCalls" | "functionCall" | "functionResponse"))
                .for_each(|(_, child)| visit(child, chat));
        }
        _ => {}
    }
}

fn chat(path: &Path) -> Tally {
    let mut chat = Chat::default();
    let Some(mut reader) = open(path) else { return Tally::default() };
    if path.extension().is_some_and(|e| e == "jsonl") {
        for line in BufReader::new(reader).lines().map_while(Result::ok).filter(|l| l.len() <= MAX_LINE) {
            if let Ok(v) = serde_json::from_str::<Value>(&line) {
                visit(&v, &mut chat);
            }
        }
    } else {
        let mut text = String::new();
        if reader.read_to_string(&mut text).is_ok()
            && let Ok(v) = serde_json::from_str::<Value>(&text)
        {
            visit(&v, &mut chat);
        }
    }
    let mut tally = if chat.listed.calls > 0 { chat.listed } else { chat.parts };
    tally.tokens = chat.tokens;
    tally.sessions = usize::from(chat.seen);
    tally
}

fn files_of(user: &Path, source: &Source) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for root in source.roots {
        let dir = root.iter().fold(user.to_path_buf(), |p, part| p.join(part));
        walk(&dir, DEPTH, source.codex, &mut files);
    }
    files
}

pub fn codex_files(user: &Path) -> Vec<PathBuf> {
    SOURCES.iter().filter(|s| s.codex).flat_map(|s| files_of(user, s)).collect()
}

pub fn weeks_in(user: &Path, id: &str, now: u64) -> [Tally; 2] {
    let mut weeks: [Tally; 2] = Default::default();
    let Some(source) = SOURCES.iter().find(|s| s.id == id) else { return weeks };
    for path in files_of(user, source) {
        let Some(w) = modified(&path).and_then(|secs| week(now, secs)) else { continue };
        let one = if source.codex { codex(&path) } else { chat(&path) };
        weeks[w].add(&one);
    }
    weeks
}

pub fn reads(id: &str) -> bool {
    SOURCES.iter().any(|s| s.id == id)
}

pub fn weeks(ctx: &Ctx, id: &str) -> [Tally; 2] {
    weeks_in(&ctx.user, id, now())
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    fn line(payload: &str) -> String {
        format!(r#"{{"timestamp":"t","type":"response_item","payload":{payload}}}"#)
    }

    #[test]
    fn codex_rollouts_are_counted_plain_and_compressed() {
        let root = std::env::temp_dir().join(format!("lt-ai-sessions-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let day = root.join(".codex/sessions/2026/11/23");
        fs::create_dir_all(&day).unwrap();
        let lines = [
            line(r#"{"type":"function_call","name":"shell_command","call_id":"a"}"#),
            line(r#"{"type":"function_call_output","call_id":"a","output":"Exit code: 1\nWall time: 0.2 seconds\nOutput:\nfatal: not a git repository"}"#),
            line(r#"{"type":"function_call","name":"shell_command","call_id":"b"}"#),
            line(r#"{"type":"function_call_output","call_id":"b","output":"Exit code: 0\nOutput:\nok"}"#),
            line(r#"{"type":"custom_tool_call","name":"apply_patch","call_id":"c"}"#),
            line(r#"{"type":"custom_tool_call_output","call_id":"c","output":"apply_patch verification failed: Failed to find expected lines"}"#),
            line(r#"{"type":"function_call","name":"apply_patch","call_id":"d"}"#),
            line(r#"{"type":"function_call_output","call_id":"d","output":"{\"output\":\"Success.\",\"metadata\":{\"exit_code\":0}}"}"#),
            r#"{"type":"event_msg","payload":{"type":"token_count","info":{"total_token_usage":{"total_tokens":1500}}}}"#.into(),
            r#"{"type":"event_msg","payload":{"type":"token_count","info":{"total_token_usage":{"total_tokens":4000}}}}"#.into(),
        ];
        let text = lines.join("\n");
        fs::write(day.join("rollout-a.jsonl"), &text).unwrap();
        let mut packed =
            zstd::stream::write::Encoder::new(File::create(day.join("rollout-b.jsonl.zst")).unwrap(), 3).unwrap();
        packed.write_all(text.as_bytes()).unwrap();
        packed.finish().unwrap();
        let [now, before] = weeks_in(&root, "codex", now());
        assert_eq!((now.sessions, now.calls, now.errors, now.tokens), (2, 8, 4, 8000));
        assert_eq!(now.fails["fatal: not a git repository"], 2);
        assert_eq!(before, Tally::default());
        assert_eq!(codex_failed("{\"output\":\"x\",\"metadata\":{\"exit_code\":2}}").as_deref(), Some("x"));
        assert!(codex_failed("Script completed\nWall time 1 seconds").is_none());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn gemini_and_qwen_chats_count_tool_calls_once() {
        let root = std::env::temp_dir().join(format!("lt-ai-chats-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let gem = root.join(".gemini/tmp/abc/chats");
        let qwen = root.join(".qwen/projects/x/chats");
        fs::create_dir_all(&gem).unwrap();
        fs::create_dir_all(&qwen).unwrap();
        fs::write(
            gem.join("session-1.json"),
            r#"{"sessionId":"s","messages":[{"type":"user","content":"hi"},{"type":"gemini","tokens":{"total":700},
            "toolCalls":[{"name":"run_shell_command","status":"success"},{"name":"read_file","status":"error","resultDisplay":"File not found: a.rs"}]}]}"#,
        )
        .unwrap();
        fs::write(root.join(".gemini/tmp/abc/logs.json"), r#"[{"type":"user","message":"x"}]"#).unwrap();
        let q = [
            r#"{"type":"assistant","message":{"role":"model","parts":[{"functionCall":{"name":"read_file","args":{}}}]},"usageMetadata":{"totalTokenCount":300}}"#,
            r#"{"type":"tool_result","message":{"role":"user","parts":[{"functionResponse":{"name":"read_file","response":{"error":"File not found: b.rs"}}}]}}"#,
            r#"{"type":"assistant","message":{"role":"model","parts":[{"functionCall":{"name":"ls","args":{}}}]},"usageMetadata":{"totalTokenCount":500}}"#,
        ];
        fs::write(qwen.join("s1.jsonl"), q.join("\n")).unwrap();
        let [g, _] = weeks_in(&root, "gemini", now());
        assert_eq!((g.sessions, g.calls, g.errors, g.tokens), (1, 2, 1, 700));
        let [w, _] = weeks_in(&root, "qwen", now());
        assert_eq!((w.sessions, w.calls, w.errors, w.tokens), (1, 2, 1, 800));
        assert_eq!(w.fails["File not found: b.rs"], 1);
        assert!(reads("qwen") && !reads("cursor"));
        let _ = fs::remove_dir_all(&root);
    }
}
