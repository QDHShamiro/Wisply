use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};
use wisply_core::Fail;

use super::cfg;
use super::idle;
use super::text::{now, quiet_now};

const MIN_S: u64 = 60;

fn file(home: &Path) -> PathBuf {
    cfg::dir(home).join("state.json")
}

pub fn state(home: &Path) -> Map<String, Value> {
    fs::read_to_string(file(home))
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default()
}

pub fn remember(home: &Path, kv: Value) {
    let mut cur = state(home);
    if let Some(obj) = kv.as_object() {
        cur.extend(obj.clone());
    }
    let _ = fs::create_dir_all(cfg::dir(home));
    let _ = wisply_core::save(file(home), Value::Object(cur).to_string());
}

pub fn manual(home: &Path) -> bool {
    state(home).get("away").and_then(Value::as_bool).unwrap_or(false)
}

pub fn gone(home: &Path, settings: &Map<String, Value>) -> Option<String> {
    if manual(home) {
        return Some("away is on".to_string());
    }
    let after = cfg::num(settings, "away_after_min") * MIN_S;
    let idle = idle::secs()?;
    (after > 0 && idle >= after).then(|| format!("no input for {} min", idle / MIN_S))
}

pub fn gap_left(home: &Path, settings: &Map<String, Value>) -> u64 {
    let last = state(home).get("last_notice").and_then(Value::as_f64).unwrap_or(0.0);
    let gap = cfg::num(settings, "min_gap_s") as f64;
    (last + gap - now()).max(0.0).ceil() as u64
}

pub fn stamp_notice(home: &Path) {
    remember(home, json!({"last_notice": now()}));
}

pub fn run(home: &Path, args: &[String]) -> Result<String, Fail> {
    let settings = cfg::load(home);
    match args.first().map(String::as_str) {
        Some(word @ ("on" | "off")) => {
            remember(home, json!({"away": word == "on"}));
            Ok(format!("call: away is {word}{}", if word == "on" { " (the hook may ring now)" } else { "" }))
        }
        Some("status") | None => Ok(status(home, &settings)),
        Some(other) => Err(Fail::hint(format!("unknown away option '{other}'"), "wisply call away on|off|status")),
    }
}

pub fn status(home: &Path, settings: &Map<String, Value>) -> String {
    let after = cfg::num(settings, "away_after_min");
    let idle = idle::secs().map_or_else(|| "unknown".to_string(), |s| format!("{} min", s / MIN_S));
    let now_away = gone(home, settings).map_or_else(|| "you are here".to_string(), |why| format!("away, {why}"));
    format!(
        "call: {} (manual {}, automatic after {after} min without input, idle now {idle})\nquiet hours {}-{}: {}\ngap between hook notices: {} s ({} s left)",
        now_away,
        if manual(home) { "on" } else { "off" },
        cfg::text(settings, "quiet_from"),
        cfg::text(settings, "quiet_to"),
        if quiet_now(settings) { "now" } else { "not now" },
        cfg::num(settings, "min_gap_s"),
        gap_left(home, settings)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("wisply-call-away-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn away_switches_by_hand_and_the_gap_counts_down() {
        let home = temp("switch");
        let c = cfg::defaults();
        assert!(!manual(&home));
        assert!(run(&home, &["on".into()]).unwrap().contains("away is on"));
        assert!(manual(&home) && gone(&home, &c).is_some());
        run(&home, &["off".into()]).unwrap();
        assert!(!manual(&home));
        assert_eq!(gap_left(&home, &c), 0);
        stamp_notice(&home);
        assert!(gap_left(&home, &c) > 290);
        assert!(run(&home, &["maybe".into()]).is_err());
        assert!(status(&home, &c).contains("quiet hours 23:00-07:00"));
        let _ = fs::remove_dir_all(&home);
    }
}
