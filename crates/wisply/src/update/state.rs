use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

use super::release::newer;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const FILE: &str = "update.json";
const DAY: u64 = 86_400;

pub fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

pub fn load(home: &Path) -> Value {
    fs::read_to_string(home.join(FILE))
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .filter(Value::is_object)
        .unwrap_or_else(|| json!({}))
}

pub fn save(home: &Path, state: &Value) {
    let _ = fs::write(home.join(FILE), state.to_string());
}

pub fn set(home: &Path, key: &str, value: Value) {
    let mut state = load(home);
    state[key] = value;
    save(home, &state);
}

fn drop_key(state: &mut Value, key: &str) {
    if let Some(map) = state.as_object_mut() {
        map.remove(key);
    }
}

pub fn mode(home: &Path) -> String {
    let cfg = fs::read_to_string(home.join("config.json"))
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v["update"].as_str().map(str::to_lowercase));
    match cfg.as_deref() {
        Some(m @ ("notify" | "off")) => m.to_string(),
        _ => "auto".into(),
    }
}

pub fn finished(home: &Path, from: &str, to: &str, ok: bool, why: &str) {
    let mut state = load(home);
    state["last"] = json!({"from": from, "to": to, "at": now(), "ok": ok, "why": why, "shown": false});
    drop_key(&mut state, "staged");
    if ok {
        drop_key(&mut state, "failed");
    } else {
        let count = state["failed"]["count"].as_u64().filter(|_| state["failed"]["version"] == json!(to));
        state["failed"] = json!({"version": to, "count": count.unwrap_or(0) + 1});
    }
    save(home, &state);
}

pub fn gave_up(state: &Value, version: &str) -> bool {
    state["failed"]["version"] == json!(version) && state["failed"]["count"].as_u64().unwrap_or(0) >= 3
}

pub fn notice(home: &Path) -> Option<String> {
    let mut state = load(home);
    let line = if state["last"]["shown"] == json!(false) {
        let last = state["last"].clone();
        state["last"]["shown"] = json!(true);
        let (from, to) = (last["from"].as_str().unwrap_or("?"), last["to"].as_str().unwrap_or("?"));
        if last["ok"] == json!(true) {
            format!("Wisply updated {from} -> {to} by itself (wisply update --rollback undoes it)")
        } else {
            let why = last["why"].as_str().unwrap_or("unknown");
            format!(
                "Wisply update {from} -> {to} failed ({why}); the old version still runs. wisply update tries again"
            )
        }
    } else {
        let latest = state["latest"].as_str().unwrap_or("").to_string();
        let day = now() / DAY;
        if mode(home) != "notify" || !newer(VERSION, &latest) || state["notified"].as_u64() == Some(day) {
            return None;
        }
        state["notified"] = json!(day);
        format!("Wisply {latest} is available, you run {VERSION}: wisply update (or config update auto)")
    };
    save(home, &state);
    Some(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn home(tag: &str) -> std::path::PathBuf {
        let home = std::env::temp_dir().join(format!("wisply-update-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(&home).unwrap();
        home
    }

    #[test]
    fn mode_reads_the_config_and_defaults_to_auto() {
        let h = home("mode");
        assert_eq!(mode(&h), "auto");
        fs::write(h.join("config.json"), r#"{"update": "Notify"}"#).unwrap();
        assert_eq!(mode(&h), "notify");
        fs::write(h.join("config.json"), r#"{"update": "sometimes"}"#).unwrap();
        assert_eq!(mode(&h), "auto");
        fs::remove_dir_all(&h).unwrap();
    }

    #[test]
    fn a_result_is_shown_once_and_a_version_is_given_up_after_three_failures() {
        let h = home("last");
        finished(&h, "3.0.1", "3.1.0", true, "");
        let line = notice(&h).unwrap();
        assert!(line.contains("3.0.1 -> 3.1.0") && line.contains("--rollback"), "{line}");
        assert_eq!(notice(&h), None);
        for _ in 0..2 {
            finished(&h, "3.0.1", "3.2.0", false, "selftest");
            assert!(!gave_up(&load(&h), "3.2.0"));
        }
        finished(&h, "3.0.1", "3.2.0", false, "selftest");
        assert!(gave_up(&load(&h), "3.2.0") && !gave_up(&load(&h), "3.3.0"));
        assert!(notice(&h).unwrap().contains("failed (selftest)"));
        finished(&h, "3.0.1", "3.3.0", false, "x");
        assert_eq!(load(&h)["failed"]["count"], json!(1));
        fs::remove_dir_all(&h).unwrap();
    }

    #[test]
    fn notify_mentions_a_new_version_once_a_day() {
        let h = home("notify");
        set(&h, "latest", json!("99.0.0"));
        assert_eq!(notice(&h), None);
        fs::write(h.join("config.json"), r#"{"update": "notify"}"#).unwrap();
        assert!(notice(&h).unwrap().starts_with("Wisply 99.0.0 is available"));
        assert_eq!(notice(&h), None);
        set(&h, "notified", json!(1));
        assert!(notice(&h).is_some());
        set(&h, "latest", json!("0.0.1"));
        set(&h, "notified", json!(1));
        assert_eq!(notice(&h), None);
        fs::remove_dir_all(&h).unwrap();
    }
}
