use std::fs::{self, File};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use wisply_memory::{condense, oneline, typed_in};

const TAIL_BYTES: u64 = 1_000_000;
const HEAD_BYTES: u64 = 400_000;
const BACK_BLOCKS: usize = 12;
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
const LATEST: &str = "Latest request: ";

pub struct Early {
    pub title: String,
    pub body: String,
    pub paths: Vec<String>,
}

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

pub fn compose(first: &str, earlier: &str, tail: &str, title: &str, todos: &[String]) -> String {
    let typed = lines(tail, USER);
    let first = Some(first).filter(|f| !f.is_empty()).or_else(|| typed.first().copied()).unwrap_or("");
    let latest = typed.last().copied().or(Some(earlier).filter(|e| !e.is_empty())).unwrap_or(first);
    let goal = if title.is_empty() { first } else { title };
    let mut out = vec!["Wisply handoff, written before the compact from your transcript (no model):".to_string()];
    if !goal.is_empty() {
        out.push(format!("Goal: {}", oneline(goal, GOAL_CHARS)));
    }
    if !title.is_empty() && !first.is_empty() && first != latest {
        out.push(format!("First request: {}", oneline(first, GOAL_CHARS)));
    }
    if !latest.is_empty() && latest != goal {
        out.push(format!("{LATEST}{}", oneline(latest, LATEST_CHARS)));
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

fn earlier(path: &Path, at: u64) -> String {
    let mut to = at;
    for _ in 0..BACK_BLOCKS {
        if to == 0 {
            break;
        }
        let from = to.saturating_sub(TAIL_BYTES);
        match typed_in(path, from, to) {
            Ok((_, last)) if !last.is_empty() => return last,
            Ok(_) => to = from,
            Err(_) => break,
        }
    }
    String::new()
}

fn told(path: &Path, at: u64, tail: &str, title: &str, todos: &[String]) -> String {
    let first = typed_in(path, 0, at.min(HEAD_BYTES)).map(|found| found.0).unwrap_or_default();
    let before = if lines(tail, USER).is_empty() { earlier(path, at) } else { String::new() };
    compose(&first, &before, tail, title, todos)
}

pub fn from_transcript(path: &Path, todos: &[String]) -> String {
    let at = tail_offset(path).unwrap_or(0);
    let Ok(tail) = condense(path, at, Some(TAIL_CHARS)) else { return String::new() };
    told(path, at, &tail.text, &tail.title, todos)
}

pub fn early(path: &Path, min_chars: usize, min_users: usize) -> Option<Early> {
    let at = tail_offset(path).unwrap_or(0);
    let tail = condense(path, at, Some(TAIL_CHARS)).ok()?;
    if tail.text.chars().count() < min_chars || (tail.users < min_users && tail.edits == 0) {
        return None;
    }
    let text = told(path, at, &tail.text, &tail.title, &[]);
    let mut body: Vec<&str> = text.lines().skip(1).collect();
    if let Some(at) = body.iter().position(|l| l.starts_with(LATEST)) {
        let line = body.remove(at);
        body.insert(0, line);
    }
    Some(Early { title: tail.title, body: body.join("\n"), paths: tail.paths }).filter(|e| !e.body.is_empty())
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

    const TRIES: usize = 5;
    const SHARE_OF_DEADLINE: u32 = 8;

    fn timed(path: &Path) -> (String, Duration) {
        (0..TRIES)
            .map(|_| {
                let t = std::time::Instant::now();
                (from_transcript(path, &[]), t.elapsed())
            })
            .min_by_key(|(_, took)| *took)
            .unwrap()
    }

    fn in_time(took: Duration) -> bool {
        wisply_proto::hook_limit(&["hook", "compact"]).is_some_and(|limit| took < limit / SHARE_OF_DEADLINE)
    }

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
        let (text, best) = timed(&path);
        assert!(in_time(best), "{best:?}");
        assert!(
            text.contains("Goal: Port the importer") && text.contains("Latest request: now run the tests"),
            "{text}"
        );
        assert!(text.contains("old parser dropped empty rows"), "{text}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_latest_request_is_found_before_the_tail_and_machine_rows_are_no_requests() {
        use serde_json::json;
        let dir = std::env::temp_dir().join(format!("lt-handoff-mid-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let work = |rows: &mut Vec<String>| {
            for n in 0..6_000 {
                rows.push(edit(&format!("/work/inv/src/f{}.rs", n % 50)));
                rows.push(
                    json!({"type": "user", "message": {"content": [{"type": "tool_result",
                        "tool_use_id": format!("t{n}"), "content": "x".repeat(120)}]}})
                    .to_string(),
                );
            }
        };
        let mut rows = vec![user("Move the invoices to the new layout")];
        work(&mut rows);
        rows.push(user("print the tax as its own line"));
        work(&mut rows);
        rows.push(said("Found the cause: the tax line was summed twice, so the total was wrong."));
        rows.push(
            json!({"type": "user", "turnOrigin": "task_notification",
                "message": {"content": "<task-notification><status>completed</status></task-notification>"}})
            .to_string(),
        );
        rows.push(
            json!({"type": "user", "isCompactSummary": true,
                "message": {"content": "This session is being continued from a previous conversation."}})
            .to_string(),
        );
        rows.push(json!({"type": "ai-title", "aiTitle": "Invoice layout"}).to_string());
        let path = write_transcript(&dir, &rows);
        assert!(fs::metadata(&path).unwrap().len() > 3 * TAIL_BYTES);
        let (text, best) = timed(&path);
        assert!(in_time(best), "{best:?}");
        let top = "Goal: Invoice layout\nFirst request: Move the invoices to the new layout\nLatest request: print the tax as its own line\n";
        assert!(text.contains(top), "{text}");
        assert!(text.contains("the tax line was summed twice"), "{text}");
        assert!(!text.contains("completed") && !text.contains("being continued"), "{text}");
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn nothing_to_say_gives_nothing() {
        assert_eq!(compose("", "", "", "", &[]), "");
    }

    #[test]
    fn an_early_note_leads_with_the_latest_request_and_skips_small_talk() {
        let dir = std::env::temp_dir().join(format!("lt-handoff-early-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let rows = vec![
            user("Build a cache for the shop prices so the page loads faster"),
            said("Root cause: the price query runs once per item, so I chose one batched query instead."),
            edit("/work/shop/src/prices.rs"),
            user("also keep the cache for five minutes only"),
            said("Decided to expire entries after 300 s because stale prices cost orders."),
            serde_json::json!({"type": "ai-title", "aiTitle": "Price cache"}).to_string(),
        ];
        let path = write_transcript(&dir, &rows);
        let got = early(&path, 100, 2).unwrap();
        assert_eq!(got.title, "Price cache");
        let lead = "Latest request: also keep the cache for five minutes only\nGoal: Price cache\n";
        assert!(got.body.starts_with(lead), "{}", got.body);
        assert!(
            !got.body.contains("Wisply handoff") && got.body.contains("expire entries after 300 s"),
            "{}",
            got.body
        );
        assert!(!got.paths.is_empty() && got.paths.iter().all(|p| p == "/work/shop/src/prices.rs"), "{:?}", got.paths);
        assert!(early(&path, 100_000, 2).is_none());
        let small = write_transcript(&dir, &[user("hi there, are you awake"), said("Yes, I am here.")]);
        assert!(early(&small, 1, 2).is_none());
        let _ = fs::remove_dir_all(&dir);
    }
}
