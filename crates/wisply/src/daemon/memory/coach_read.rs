use std::io::{BufRead, BufReader};
use std::path::Path;

use serde_json::Value;

use crate::daemon::doctor::ai::grade::signature;
use crate::daemon::doctor::ai::sessions::{codex_failed, open};

const MAX_LINE: usize = 3_000_000;
const EDITS: [&str; 4] = ["Edit", "Write", "MultiEdit", "NotebookEdit"];
const SHELLS: [&str; 2] = ["Bash", "PowerShell"];
const READERS: [&str; 9] = ["cat", "sed", "head", "tail", "less", "nl", "type", "get-content", "bat"];
const SEPARATORS: [&str; 4] = ["&&", "||", "|", ";"];
const PATCH_FILES: [&str; 2] = ["*** Update File: ", "*** Add File: "];

#[derive(Debug, PartialEq)]
pub enum Step {
    Read(String, String),
    Edit(String),
    Run(String),
    Out(Option<String>, usize),
}

fn text_of(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Array(parts) => {
            parts.iter().filter_map(|p| p.get("text").and_then(Value::as_str)).collect::<Vec<_>>().join("\n")
        }
        _ => String::new(),
    }
}

fn str_at<'a>(v: &'a Value, key: &str) -> &'a str {
    v.get(key).and_then(Value::as_str).unwrap_or("")
}

fn shell(cmd: &str, out: &mut Vec<Step>) {
    let mut parts = vec![cmd];
    for sep in SEPARATORS {
        parts = parts.iter().flat_map(|p| p.split(sep)).collect();
    }
    for part in parts {
        let words: Vec<&str> = part.split_whitespace().collect();
        let Some((first, rest)) = words.split_first() else { continue };
        if !READERS.contains(&first.to_lowercase().as_str()) {
            continue;
        }
        if let Some(file) = rest.iter().rev().find(|w| !w.starts_with('-') && w.contains('.')) {
            out.push(Step::Read(file.trim_matches(['"', '\'']).to_string(), part.trim().to_string()));
        }
    }
    for line in cmd.lines() {
        for pre in PATCH_FILES {
            if let Some(path) = line.trim_start().strip_prefix(pre) {
                out.push(Step::Edit(path.trim().to_string()));
            }
        }
    }
    out.push(Step::Run(cmd.to_string()));
}

fn claude_line(v: &Value, out: &mut Vec<Step>) {
    let Some(blocks) = v.pointer("/message/content").and_then(Value::as_array) else { return };
    let denied = !str_at(v, "toolDenialKind").is_empty();
    for b in blocks {
        match str_at(b, "type") {
            "tool_use" => {
                let input = b.get("input").unwrap_or(&Value::Null);
                let name = str_at(b, "name");
                if name == "Read" && !str_at(input, "file_path").is_empty() {
                    let at = |k: &str| input.get(k).map(Value::to_string).unwrap_or_default();
                    out.push(Step::Read(
                        str_at(input, "file_path").to_string(),
                        format!("{}:{}", at("offset"), at("limit")),
                    ));
                } else if EDITS.contains(&name) {
                    let path = [str_at(input, "file_path"), str_at(input, "notebook_path")]
                        .into_iter()
                        .find(|p| !p.is_empty())
                        .unwrap_or("");
                    out.push(Step::Edit(path.to_string()));
                } else if SHELLS.contains(&name) {
                    shell(str_at(input, "command"), out);
                }
            }
            "tool_result" if !denied => {
                let text = text_of(b.get("content").unwrap_or(&Value::Null));
                let failed = b.get("is_error").and_then(Value::as_bool) == Some(true);
                out.push(Step::Out(failed.then(|| signature(&text)), text.len()));
            }
            _ => {}
        }
    }
}

fn command_of(v: &Value) -> Option<String> {
    match v.get("command").or_else(|| v.get("cmd")) {
        Some(Value::String(s)) => Some(s.clone()),
        Some(Value::Array(a)) => a.iter().filter_map(Value::as_str).next_back().map(str::to_string),
        _ => None,
    }
}

fn codex_line(v: &Value, out: &mut Vec<Step>, cwd: &mut String) {
    let Some(p) = v.get("payload") else { return };
    if str_at(v, "type") == "session_meta" {
        if cwd.is_empty() {
            *cwd = str_at(p, "cwd").to_string();
        }
        return;
    }
    match str_at(p, "type") {
        "function_call" | "local_shell_call" => {
            let args: Value = serde_json::from_str(str_at(p, "arguments")).unwrap_or(Value::Null);
            if let Some(cmd) = command_of(&args).or_else(|| command_of(p.get("action").unwrap_or(&Value::Null))) {
                shell(&cmd, out);
            }
        }
        "custom_tool_call" => {
            let input = str_at(p, "input");
            for line in input.lines() {
                for pre in PATCH_FILES {
                    if let Some(path) = line.strip_prefix(pre) {
                        out.push(Step::Edit(path.trim().to_string()));
                    }
                }
            }
        }
        kind if kind.ends_with("_output") => {
            let text = p.get("output").map(text_of).unwrap_or_default();
            out.push(Step::Out(codex_failed(&text).map(|t| signature(&t)), text.len()));
        }
        _ => {}
    }
}

pub fn steps(path: &Path, codex: bool) -> (Vec<Step>, String) {
    let mut out = Vec::new();
    let mut cwd = String::new();
    let Some(reader) = open(path) else { return (out, cwd) };
    for line in BufReader::new(reader).lines().map_while(Result::ok) {
        let wanted = if codex {
            line.contains("call") || line.contains("session_meta")
        } else {
            line.contains("tool_use") || line.contains("tool_result")
        };
        if line.len() > MAX_LINE || !wanted {
            continue;
        }
        let Ok(v) = serde_json::from_str::<Value>(&line) else { continue };
        if codex {
            codex_line(&v, &mut out, &mut cwd);
        } else {
            claude_line(&v, &mut out);
        }
    }
    (out, cwd)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::json;

    use super::*;

    fn file(tag: &str, lines: &[Value]) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!("wisply-coach-read-{tag}-{}.jsonl", std::process::id()));
        let text: Vec<String> = lines.iter().map(Value::to_string).collect();
        fs::write(&path, text.join("\n")).unwrap();
        path
    }

    fn tool(name: &str, input: Value) -> Value {
        json!({"type": "assistant", "message": {"content": [{"type": "tool_use", "name": name, "input": input}]}})
    }

    fn result(text: &str, error: bool, denied: bool) -> Value {
        let block = json!({"type": "tool_result", "is_error": error, "content": [{"type": "text", "text": text}]});
        let mut v = json!({"type": "user", "message": {"content": [block]}});
        if denied {
            v["toolDenialKind"] = json!("user");
        }
        v
    }

    #[test]
    fn claude_transcripts_turn_into_steps() {
        let err = "<tool_use_error>File has not been read yet</tool_use_error>";
        let path = file(
            "claude",
            &[
                tool("Read", json!({"file_path": "C:/p/a.rs", "offset": 10, "limit": 20})),
                result(err, true, false),
                result("no", true, true),
                tool("Bash", json!({"command": "cd x && sed -n 1,20p src/b.rs | head"})),
                tool("Edit", json!({"file_path": "C:/p/a.rs"})),
                result("ok", false, false),
            ],
        );
        let (got, cwd) = steps(&path, false);
        assert_eq!(cwd, "");
        assert_eq!(
            got,
            [
                Step::Read("C:/p/a.rs".into(), "10:20".into()),
                Step::Out(Some(signature(err)), err.len()),
                Step::Read("src/b.rs".into(), "sed -n 1,20p src/b.rs".into()),
                Step::Run("cd x && sed -n 1,20p src/b.rs | head".into()),
                Step::Edit("C:/p/a.rs".into()),
                Step::Out(None, 2),
            ]
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn codex_rollouts_turn_into_steps() {
        let call = |args: Value| json!({"type": "response_item", "payload": {"type": "function_call", "arguments": args.to_string()}});
        let failed = "Exit code: 1\nWall time: 0.1 seconds\nOutput:\nerror: nope";
        let patch = "*** Begin Patch\n*** Update File: src/c.rs\n@@\n-a\n+b\n*** End Patch";
        let path = file(
            "codex",
            &[
                json!({"type": "session_meta", "payload": {"cwd": "E:/demo"}}),
                call(json!({"command": ["bash", "-lc", "cat src/main.rs"]})),
                json!({"type": "response_item", "payload": {"type": "function_call_output", "output": failed}}),
                json!({"type": "response_item", "payload": {"type": "custom_tool_call", "input": patch}}),
                call(json!({"cmd": "cargo test"})),
                json!({"type": "response_item", "payload": {"type": "function_call_output", "output": "ok"}}),
            ],
        );
        let (got, cwd) = steps(&path, true);
        assert_eq!(cwd, "E:/demo");
        assert_eq!(
            got,
            [
                Step::Read("src/main.rs".into(), "cat src/main.rs".into()),
                Step::Run("cat src/main.rs".into()),
                Step::Out(Some(signature(failed)), failed.len()),
                Step::Edit("src/c.rs".into()),
                Step::Run("cargo test".into()),
                Step::Out(None, 2),
            ]
        );
        let _ = fs::remove_file(path);
    }
}
