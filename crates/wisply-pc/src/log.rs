use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::model::{secs, size};

const FILE: &str = "actions.jsonl";
const KEEP_BYTES: u64 = 512 * 1024;

#[derive(Clone, Debug)]
pub struct Log {
    path: PathBuf,
}

impl Log {
    pub fn new(dir: &Path) -> Log {
        Log { path: dir.join(FILE) }
    }

    pub fn add(&self, kind: &str, what: &str, place: &str, bytes: u64) {
        let row = json!({"t": secs(), "kind": kind, "what": what, "where": place, "bytes": bytes});
        if let Some(parent) = self.path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if fs::metadata(&self.path).is_ok_and(|m| m.len() > KEEP_BYTES) {
            let _ = fs::rename(&self.path, self.path.with_extension("old"));
        }
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&self.path) {
            let _ = writeln!(f, "{row}");
        }
    }

    pub fn tail(&self, n: usize) -> Vec<Value> {
        let text = fs::read_to_string(&self.path).unwrap_or_default();
        let rows: Vec<Value> = text.lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
        rows[rows.len().saturating_sub(n)..].to_vec()
    }

    pub fn show(&self, n: usize) -> String {
        let rows = self.tail(n);
        if rows.is_empty() {
            return "pc log | nothing done yet".into();
        }
        let text = |r: &Value, k: &str| r.get(k).and_then(Value::as_str).unwrap_or("").to_string();
        let lines: Vec<String> = rows
            .iter()
            .map(|r| {
                let bytes = r.get("bytes").and_then(Value::as_u64).unwrap_or(0);
                let when = r.get("t").and_then(Value::as_u64).unwrap_or(0);
                let age = secs().saturating_sub(when);
                let tail = if bytes > 0 { format!(" ({})", size(bytes)) } else { String::new() };
                format!("{} ago | {} | {} | {}{tail}", ago(age), text(r, "kind"), text(r, "what"), text(r, "where"))
            })
            .collect();
        lines.join("\n")
    }
}

pub fn ago(s: u64) -> String {
    match s {
        0..60 => format!("{s}s"),
        60..3600 => format!("{}m", s / 60),
        3600..86_400 => format!("{}h", s / 3600),
        _ => format!("{}d", s / 86_400),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actions_are_kept_and_shown_newest_last() {
        let dir = std::env::temp_dir().join(format!("wpc-log-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let log = Log::new(&dir);
        assert_eq!(log.show(5), "pc log | nothing done yet");
        log.add("clean", "deleted", "C:/tmp/a", 2048);
        log.add("priority", "cargo lowered", "pid 4", 0);
        let shown = log.show(5);
        let lines: Vec<&str> = shown.lines().collect();
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("clean | deleted | C:/tmp/a (2 KB)"), "{shown}");
        assert!(lines[1].ends_with("priority | cargo lowered | pid 4"), "{shown}");
        assert_eq!(log.show(1).lines().count(), 1);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn ages_read_short() {
        assert_eq!(
            (ago(5).as_str(), ago(130).as_str(), ago(7300).as_str(), ago(200_000).as_str()),
            ("5s", "2m", "2h", "2d")
        );
    }
}
