use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde_json::Value;

use super::text::scrub;
use super::transcript::{REMINDER, Style, TAG, truthy, user_text};

const BY_PERSON: &[&str] = &["human", "sdk"];
const NOT_A_TURN: &[&str] = &["isMeta", "isSidechain", "isCompactSummary"];
const COMMAND_WORDS: usize = 2;
const USER_ROW: &str = r#""type":"user""#;
const TOOL_ROW: &str = r#""tool_use_id""#;
const QUEUED_ROW: &str = r#""queued_command""#;

static MACHINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:<[a-z][a-z0-9]*(?:-[a-z0-9]+)+[\s>]|\[Request interrupted|\[SYSTEM NOTIFICATION|This session is being continued)",
    )
    .expect("machine pattern")
});
static COMMAND: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<command-name>\s*(.*?)\s*</command-name>").expect("command pattern"));
static COMMAND_ARGS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)<command-args>\s*(.*?)\s*</command-args>").expect("args pattern"));

fn by_person(d: &Value) -> bool {
    ["turnOrigin", "origin"].iter().all(|key| match d.get(*key) {
        None | Some(Value::Null) => true,
        Some(v) => v.get("kind").unwrap_or(v).as_str().is_some_and(|k| BY_PERSON.contains(&k)),
    })
}

fn typed(raw: &str) -> String {
    let text = REMINDER.replace_all(raw, "");
    let text = text.trim();
    if text.starts_with("<command-") {
        let name = COMMAND.captures(text).map(|c| c[1].to_string()).unwrap_or_default();
        let args = COMMAND_ARGS
            .captures(text)
            .map(|c| c[1].split_whitespace().collect::<Vec<_>>().join(" "))
            .unwrap_or_default();
        let setting = text.starts_with("<command-name") && args.split_whitespace().count() < COMMAND_WORDS;
        return if name.is_empty() || setting { String::new() } else { format!("{name} {args}").trim().to_string() };
    }
    if MACHINE.is_match(text) { String::new() } else { TAG.replace_all(text, "").trim().to_string() }
}

fn queued(d: &Value) -> String {
    let Some(a) = d.get("attachment").filter(|a| a.get("type").and_then(Value::as_str) == Some("queued_command"))
    else {
        return String::new();
    };
    if a.get("commandMode").and_then(Value::as_str) != Some("prompt")
        || a.get("isMeta").is_some_and(truthy)
        || !by_person(a)
    {
        return String::new();
    }
    match a.get("prompt") {
        Some(Value::String(s)) => typed(s),
        Some(Value::Array(items)) => {
            typed(&items.iter().filter_map(|x| x.get("text").and_then(Value::as_str)).collect::<Vec<_>>().join(" "))
        }
        _ => String::new(),
    }
}

pub(super) fn asked(d: &Value) -> String {
    match d.get("type").and_then(Value::as_str) {
        Some("attachment") => queued(d),
        Some("user") if by_person(d) && !NOT_A_TURN.iter().any(|k| d.get(*k).is_some_and(truthy)) => {
            match d.get("message").and_then(|m| m.get("content")) {
                Some(Value::String(s)) => typed(s),
                Some(Value::Array(items)) => items
                    .iter()
                    .filter(|c| c.get("type").and_then(Value::as_str) == Some("text"))
                    .filter_map(|c| c.get("text").and_then(Value::as_str))
                    .map(typed)
                    .filter(|t| !t.is_empty())
                    .collect::<Vec<_>>()
                    .join(" "),
                _ => String::new(),
            }
        }
        _ => String::new(),
    }
}

pub fn typed_in(path: &Path, from: u64, to: u64) -> std::io::Result<(String, String)> {
    let mut file = BufReader::new(File::open(path)?);
    let start = from.saturating_sub(1);
    file.seek(SeekFrom::Start(start))?;
    let (mut at, mut raw) = (start, Vec::new());
    if from > 0 {
        at += file.read_until(b'\n', &mut raw)? as u64;
    }
    let (mut first, mut last) = (String::new(), String::new());
    while at < to {
        raw.clear();
        let n = file.read_until(b'\n', &mut raw)?;
        if n == 0 || !raw.ends_with(b"\n") {
            break;
        }
        at += n as u64;
        let Ok(line) = std::str::from_utf8(&raw) else { continue };
        if !line.contains(QUEUED_ROW) && (!line.contains(USER_ROW) || line.contains(TOOL_ROW)) {
            continue;
        }
        let Ok(d) = serde_json::from_str::<Value>(line) else { continue };
        let text = scrub(&user_text(&asked(&d), Style::Lean));
        if text.is_empty() {
            continue;
        }
        if first.is_empty() {
            first.clone_from(&text);
        }
        last = text;
    }
    Ok((first, last))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::super::transcript::{condense, condense_as};
    use super::*;

    #[test]
    fn only_typed_messages_become_user_lines() {
        use serde_json::json;
        let dir = std::env::temp_dir().join(format!("ltm-typed-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let user =
            |origin: &str, text: &str| json!({"type": "user", "turnOrigin": origin, "message": {"content": text}});
        let queued = |origin: &str, prompt: Value| {
            json!({"type": "attachment", "attachment": {"type": "queued_command", "commandMode": "prompt",
                "origin": {"kind": origin}, "prompt": prompt}})
        };
        let rows = [
            user("human", "add a discount field to the order form"),
            user("task_notification", "<task-notification><status>completed</status></task-notification>"),
            json!({"type": "user", "message": {"content": "<task-notification>oven done</task-notification>"}}),
            json!({"type": "user", "isCompactSummary": true,
                "message": {"content": "This session is being continued from a previous conversation."}}),
            json!({"type": "user", "message": {"content": [{"type": "text", "text": "[Request interrupted by user]"}]}}),
            json!({"type": "user", "message": {"content": "<local-command-stdout>Set model to quick</local-command-stdout>"}}),
            json!({"type": "user", "message": {"content":
                "<command-name>/model</command-name>\n<command-message>model</command-message>\n<command-args>quick</command-args>"}}),
            json!({"type": "user", "message": {"content":
                "<command-name>/goal</command-name>\n<command-message>goal</command-message>\n<command-args>ship the order form today</command-args>"}}),
            user("human", "<command-message>tidy</command-message>\n<command-name>/tidy</command-name>"),
            json!({"type": "user", "turnOrigin": "peer", "origin": {"kind": "peer"},
                "message": {"content": "the other session moved the main branch"}}),
            queued("human", json!("and make it required")),
            queued("task-notification", json!("<task-notification>oven done</task-notification>")),
            queued("human", json!([{"type": "text", "text": "use the red style"}])),
            user("human", "<bash-stdout>3 files</bash-stdout>"),
            json!({"type": "attachment", "attachment": {"type": "hook_success", "content": "noise"}}),
            json!({"type": "assistant", "message": {"content": [{"type": "tool_use", "name": "Bash",
                "input": {"command": "cd /work/shop && export A=1 && git commit -m \"add discount field\""}}]}}),
        ]
        .map(|row| row.to_string());
        let path = dir.join("t.jsonl");
        fs::write(&path, rows.join("\n") + "\n").unwrap();
        let lean = condense(&path, 0, None).unwrap();
        let want = [
            "USER: add a discount field to the order form",
            "USER: /goal ship the order form today",
            "USER: /tidy",
            "USER: and make it required",
            "USER: use the red style",
            "RAN git commit -m \"add discount field\"",
        ];
        assert_eq!(lean.text, want.join("\n"));
        assert_eq!((lean.text.matches("USER: ").count(), lean.users), (5, 13));
        let full = condense_as(&path, 0, None, Style::Full).unwrap();
        assert_eq!(full.text.matches("USER: ").count(), 5, "{}", full.text);
        let at = |n: usize| rows[..n].iter().map(|row| row.len() as u64 + 1).sum::<u64>();
        let whole = typed_in(&path, 0, u64::MAX).unwrap();
        assert_eq!(whole, ("add a discount field to the order form".to_string(), "use the red style".to_string()));
        let span = typed_in(&path, at(8), at(11)).unwrap();
        assert_eq!(span, ("/tidy".to_string(), "and make it required".to_string()));
        let inside = typed_in(&path, at(8) + 5, at(11)).unwrap();
        assert_eq!(inside, ("and make it required".to_string(), "and make it required".to_string()));
        assert_eq!(typed_in(&path, at(1), at(7)).unwrap(), (String::new(), String::new()));
        let _ = fs::remove_dir_all(&dir);
    }
}
