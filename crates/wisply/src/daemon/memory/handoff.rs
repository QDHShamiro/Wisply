use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use serde_json::Value;
use wisply_memory::{condense, oneline};

const TAIL_BYTES: u64 = 1_000_000;
const HEAD_BYTES: u64 = 400_000;
const USER_TAG: &str = r#""type":"user""#;
const CONTINUED: &str = "This session is being continued";
const TAIL_CHARS: usize = 600_000;
const GOAL_CHARS: usize = 260;
const LATEST_CHARS: usize = 260;
const DECISION_CHARS: usize = 200;
const DECISIONS: usize = 6;
const FILES: usize = 12;
const COMMANDS: usize = 4;
const ERRORS: usize = 2;
const ERROR_CHARS: usize = 160;
const TODOS: usize = 8;
const HARD_CAP: usize = 3_500;
const KEEP: Duration = Duration::from_secs(172_800);
const DIR: &str = "handoff";
const USER: &str = "USER: ";
const CLAUDE: &str = "CLAUDE: ";
const EDIT: &str = "EDIT ";
const RAN: &str = "RAN ";
const ERROR: &str = "ERROR: ";

fn lines<'a>(text: &'a str, prefix: &str) -> Vec<&'a str> {
    text.lines().filter_map(|l| l.strip_prefix(prefix)).collect()
}

fn last_unique(items: Vec<&str>, n: usize) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for item in items.into_iter().rev() {
        if !out.iter().any(|o| o == item) {
            out.push(item.to_string());
        }
        if out.len() == n {
            break;
        }
    }
    out.reverse();
    out
}

fn bullets(head: &str, items: Vec<String>, width: usize, out: &mut Vec<String>) {
    if items.is_empty() {
        return;
    }
    out.push(head.to_string());
    out.extend(items.iter().map(|i| format!("  - {}", oneline(i, width))));
}

pub fn compose(head: &str, tail: &str, title: &str, todos: &[String]) -> String {
    let goal = Some(title)
        .filter(|t| !t.is_empty())
        .or_else(|| lines(head, USER).first().copied())
        .or_else(|| lines(tail, USER).first().copied())
        .unwrap_or("");
    let latest = lines(tail, USER).last().copied().unwrap_or("");
    let mut out = vec!["Wisply handoff, written before the compact from your transcript (no model):".to_string()];
    if !goal.is_empty() {
        out.push(format!("Goal: {}", oneline(goal, GOAL_CHARS)));
    }
    if !latest.is_empty() && latest != goal {
        out.push(format!("Latest request: {}", oneline(latest, LATEST_CHARS)));
    }
    bullets("Decisions and findings:", last_unique(lines(tail, CLAUDE), DECISIONS), DECISION_CHARS, &mut out);
    let open: Vec<String> = todos.iter().take(TODOS).cloned().collect();
    bullets("Open to-dos:", open, DECISION_CHARS, &mut out);
    bullets("Last errors:", last_unique(lines(tail, ERROR), ERRORS), ERROR_CHARS, &mut out);
    bullets("Changed files:", last_unique(lines(tail, EDIT), FILES), DECISION_CHARS, &mut out);
    bullets("Commands run:", last_unique(lines(tail, RAN), COMMANDS), DECISION_CHARS, &mut out);
    if out.len() == 1 {
        return String::new();
    }
    let text = out.join("\n");
    if text.chars().count() <= HARD_CAP { text } else { text.chars().take(HARD_CAP).collect() }
}

fn tail_offset(path: &Path) -> std::io::Result<u64> {
    let size = fs::metadata(path)?.len();
    if size <= TAIL_BYTES {
        return Ok(0);
    }
    let mut file = BufReader::new(File::open(path)?);
    file.seek(SeekFrom::Start(size - TAIL_BYTES))?;
    let mut skipped = Vec::new();
    let n = file.read_until(b'\n', &mut skipped)?;
    Ok(size - TAIL_BYTES + n as u64)
}

fn first_request(path: &Path) -> String {
    let Ok(file) = File::open(path) else { return String::new() };
    let mut reader = BufReader::new(file.take(HEAD_BYTES));
    let mut raw = Vec::new();
    while reader.read_until(b'\n', &mut raw).is_ok_and(|n| n > 0) {
        let line = std::mem::take(&mut raw);
        if !line.windows(USER_TAG.len()).any(|w| w == USER_TAG.as_bytes()) || !line.ends_with(b"\n") {
            continue;
        }
        let Ok(v) = serde_json::from_slice::<Value>(&line) else { continue };
        if ["isMeta", "isSidechain"].iter().any(|k| v.get(*k).and_then(Value::as_bool) == Some(true)) {
            continue;
        }
        let content = v.get("message").and_then(|m| m.get("content"));
        let text = match content {
            Some(Value::String(s)) => s.as_str(),
            Some(Value::Array(a)) => a.iter().find_map(|b| b.get("text").and_then(Value::as_str)).unwrap_or(""),
            _ => "",
        };
        let text = text.trim();
        if !text.is_empty() && !text.starts_with(['<', '[']) && !text.starts_with(CONTINUED) {
            return oneline(text, GOAL_CHARS);
        }
    }
    String::new()
}

pub fn from_transcript(path: &Path, todos: &[String]) -> String {
    let at = tail_offset(path).unwrap_or(0);
    let Ok(tail) = condense(path, at, Some(TAIL_CHARS)) else { return String::new() };
    let first = if at == 0 { String::new() } else { first_request(path) };
    let head = if first.is_empty() { String::new() } else { format!("{USER}{first}") };
    compose(&head, &tail.text, &tail.title, todos)
}

fn file_of(home: &Path, sid: &str) -> Option<PathBuf> {
    let name: String = sid.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_').collect();
    (!name.is_empty()).then(|| home.join(DIR).join(format!("{name}.md")))
}

fn tidy(dir: &Path) {
    let now = SystemTime::now();
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        let old =
            entry.metadata().and_then(|m| m.modified()).is_ok_and(|t| now.duration_since(t).unwrap_or_default() > KEEP);
        if old {
            let _ = fs::remove_file(entry.path());
        }
    }
}

pub fn save(home: &Path, sid: &str, text: &str) {
    let Some(file) = file_of(home, sid) else { return };
    if text.is_empty() {
        let _ = fs::remove_file(&file);
        return;
    }
    if let Some(dir) = file.parent() {
        let _ = fs::create_dir_all(dir);
        tidy(dir);
    }
    let _ = fs::write(&file, text);
}

pub fn take(home: &Path, sid: &str) -> Option<String> {
    let file = file_of(home, sid)?;
    let text = fs::read_to_string(&file).ok()?;
    let _ = fs::remove_file(&file);
    Some(text).filter(|t| !t.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_transcript(dir: &Path, rows: &[String]) -> PathBuf {
        let path = dir.join("t.jsonl");
        fs::write(&path, rows.join("\n") + "\n").unwrap();
        path
    }

    fn user(text: &str) -> String {
        serde_json::json!({"type": "user", "message": {"content": text}}).to_string()
    }

    fn said(text: &str) -> String {
        serde_json::json!({"type": "assistant", "message": {"content": [{"type": "text", "text": text}]}}).to_string()
    }

    fn edit(path: &str) -> String {
        serde_json::json!({"type": "assistant", "message": {"content": [{"type": "tool_use", "name": "Edit", "input": {"file_path": path}}]}})
            .to_string()
    }

    #[test]
    fn a_made_up_transcript_gives_goal_decisions_files_and_open_points() {
        let dir = std::env::temp_dir().join(format!("lt-handoff-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let rows = vec![
            user("Build a cache for the shop prices so the page loads faster"),
            said("Let me look at the code first."),
            said("Root cause: the price query runs once per item, so I chose one batched query instead."),
            edit("/work/shop/src/prices.rs"),
            edit("/work/shop/src/prices.rs"),
            edit("/work/shop/src/cache.rs"),
            user("also keep the cache for five minutes only"),
            said("Decided to expire entries after 300 s because stale prices cost orders."),
        ];
        let path = write_transcript(&dir, &rows);
        let text = from_transcript(&path, &["#4 add a test for the expiry".to_string()]);
        assert!(text.contains("Goal: Build a cache for the shop prices"), "{text}");
        assert!(text.contains("Latest request: also keep the cache for five minutes only"), "{text}");
        assert!(text.contains("one batched query instead") && text.contains("expire entries after 300 s"), "{text}");
        assert!(!text.contains("Let me look at the code"), "{text}");
        assert!(text.contains("  - /work/shop/src/prices.rs\n  - /work/shop/src/cache.rs"), "{text}");
        assert_eq!(text.matches("prices.rs").count(), 1, "{text}");
        assert!(text.contains("Open to-dos:\n  - #4 add a test for the expiry"), "{text}");
        assert!(text.chars().count() <= HARD_CAP);
        save(&dir, "sess-1", &text);
        assert_eq!(take(&dir, "sess-1").as_deref(), Some(text.as_str()));
        assert_eq!(take(&dir, "sess-1"), None);
        assert_eq!(take(&dir, "../x"), None);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_long_transcript_is_read_from_the_end_and_stays_fast() {
        let dir = std::env::temp_dir().join(format!("lt-handoff-big-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let mut rows = vec![user("Port the importer to the new parser")];
        for n in 0..40_000 {
            rows.push(edit(&format!("/work/imp/src/f{}.rs", n % 50)));
            rows.push(serde_json::json!({"type": "user", "message": {"content": [{"type": "tool_result", "content": "x".repeat(120)}]}}).to_string());
        }
        rows.push(said("Found the bug: the old parser dropped empty rows, so the totals differed."));
        rows.push(user("now run the tests"));
        let path = write_transcript(&dir, &rows);
        assert!(fs::metadata(&path).unwrap().len() > TAIL_BYTES);
        let t = std::time::Instant::now();
        let text = from_transcript(&path, &[]);
        assert!(t.elapsed() < Duration::from_millis(300), "{:?}", t.elapsed());
        assert!(
            text.contains("Goal: Port the importer") && text.contains("Latest request: now run the tests"),
            "{text}"
        );
        assert!(text.contains("old parser dropped empty rows"), "{text}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn nothing_to_say_gives_nothing() {
        assert_eq!(compose("", "", "", &[]), "");
    }
}
