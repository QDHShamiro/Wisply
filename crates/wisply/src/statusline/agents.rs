use std::fs;
use std::path::Path;

use serde_json::Value;

use super::render::{paint, tone};

const STALE_AFTER: u64 = 180;
const FILE: &str = "board.json";
const WAIT_TONE: i64 = 60;
const LIMIT_TONE: i64 = 100;
const CALM_TONE: i64 = 0;

pub fn field(home: &Path, now: u64) -> Option<String> {
    let text = fs::read_to_string(home.join("agents").join(FILE)).ok()?;
    part(&serde_json::from_str(&text).ok()?, now)
}

fn count(v: &Value, key: &str) -> i64 {
    v.get(key).and_then(Value::as_i64).unwrap_or(0).max(0)
}

fn part(v: &Value, now: u64) -> Option<String> {
    let at = v.get("t")?.as_u64()?;
    if now.saturating_sub(at) > STALE_AFTER {
        return None;
    }
    let (running, waiting, limit) = (count(v, "running"), count(v, "waiting"), count(v, "limit"));
    let mut bits = Vec::new();
    if running > 0 {
        bits.push(format!("{running} run"));
    }
    if waiting > 0 {
        bits.push(format!("{waiting} wait"));
    }
    if limit > 0 {
        let at = v.get("reset").and_then(Value::as_str).filter(|s| !s.is_empty());
        bits.push(at.map_or_else(|| format!("{limit} limit"), |t| format!("{limit} limit till {t}")));
    }
    if bits.is_empty() {
        return None;
    }
    let level = if limit > 0 {
        LIMIT_TONE
    } else if waiting > 0 {
        WAIT_TONE
    } else {
        CALM_TONE
    };
    Some(paint(tone(level), &format!("agents {}", bits.join(" "))))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn plain(s: &str) -> String {
        let mut out = String::new();
        let mut skip = false;
        for c in s.chars() {
            match (skip, c) {
                (_, '\u{1b}') => skip = true,
                (true, 'm') => skip = false,
                (false, c) => out.push(c),
                _ => {}
            }
        }
        out
    }

    #[test]
    fn the_segment_counts_what_needs_a_look_and_hides_when_quiet_or_old() {
        let v = json!({"t": 1000, "running": 2, "waiting": 1, "limit": 0});
        assert_eq!(plain(&part(&v, 1010).unwrap()), "agents 2 run 1 wait");
        let v = json!({"t": 1000, "running": 0, "waiting": 0, "limit": 3, "reset": "15:20"});
        assert_eq!(plain(&part(&v, 1010).unwrap()), "agents 3 limit till 15:20");
        assert!(part(&json!({"t": 1000, "running": 0, "waiting": 0, "limit": 0}), 1010).is_none());
        assert!(part(&json!({"t": 1000, "running": 1}), 1000 + STALE_AFTER + 1).is_none());
        assert!(part(&json!({"running": 1}), 10).is_none());
    }
}
