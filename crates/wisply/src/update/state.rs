use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};
use wisply_core::ago;

use super::auto::{FORCE_AFTER, managed};
use super::plugins::summary;
use super::release::newer;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const FILE: &str = "update.json";
const DAY: u64 = 86_400;
const CHANGES: &str = include_str!("../../../../CHANGELOG.md");
const NEWS: usize = 3;
const NEWS_CHARS: usize = 110;

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
    let _ = wisply_core::save(home.join(FILE), state.to_string());
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

pub fn staged(home: &Path, version: &str, root: &Path) {
    set(home, "staged", json!({"version": version, "root": root.to_string_lossy(), "at": now()}));
}

pub fn news(changes: &str, version: &str) -> Vec<String> {
    let head = format!("## [{version}]");
    let Some(start) = changes.find(&head) else { return Vec::new() };
    let body = changes[start + head.len()..].split("\n## [").next().unwrap_or("");
    let mut items: Vec<String> = Vec::new();
    for line in body.lines() {
        if let Some(item) = line.strip_prefix("- ") {
            items.push(item.trim().to_string());
        } else if line.starts_with("  ")
            && let Some(last) = items.last_mut()
        {
            last.push(' ');
            last.push_str(line.trim());
        }
    }
    items.iter().take(NEWS).map(|item| first_sentence(item)).collect()
}

fn first_sentence(item: &str) -> String {
    let end = item.find(". ").map_or(item.len(), |at| at + 1);
    let sentence = &item[..end];
    if sentence.chars().count() <= NEWS_CHARS {
        return sentence.to_string();
    }
    let cut: String = sentence.chars().take(NEWS_CHARS - 1).collect();
    format!("{}…", cut.trim_end())
}

pub fn notice(home: &Path) -> Option<String> {
    let mut state = load(home);
    let mut lines: Vec<String> = result_line(&mut state).or_else(|| offer_line(home, &mut state)).into_iter().collect();
    lines.extend(plugin_lines(&mut state));
    if lines.is_empty() {
        return None;
    }
    save(home, &state);
    Some(lines.join("\n"))
}

fn result_line(state: &mut Value) -> Option<String> {
    if state["last"]["shown"] != json!(false) {
        return None;
    }
    state["last"]["shown"] = json!(true);
    let last = &state["last"];
    let (from, to) = (last["from"].as_str().unwrap_or("?"), last["to"].as_str().unwrap_or("?"));
    if last["ok"] != json!(true) {
        let why = last["why"].as_str().unwrap_or("unknown");
        return Some(format!(
            "Wisply update {from} -> {to} failed ({why}); the old version still runs. wisply update tries again"
        ));
    }
    let new = if to == VERSION { news(CHANGES, VERSION) } else { Vec::new() };
    let new = if new.is_empty() { String::new() } else { format!(". New: {}", new.join(" | ")) };
    Some(format!("Wisply updated {from} -> {to} by itself (wisply update --rollback undoes it){new}"))
}

fn offer_line(home: &Path, state: &mut Value) -> Option<String> {
    let latest = state["latest"].as_str().unwrap_or("").to_string();
    let day = now() / DAY;
    if mode(home) != "notify" || !newer(VERSION, &latest) || state["notified"].as_u64() == Some(day) {
        return None;
    }
    state["notified"] = json!(day);
    Some(format!("Wisply {latest} is available, you run {VERSION}: wisply update (or config update auto)"))
}

fn plugin_lines(state: &mut Value) -> Vec<String> {
    let mut lines = Vec::new();
    for (ai, name) in [("claude", "Claude"), ("codex", "Codex")] {
        let entry = &state["plugins"][ai];
        if entry["ok"] != json!(true) || entry["shown"] != json!(false) || entry["to"] != json!(VERSION) {
            continue;
        }
        let from = entry["from"].as_str().filter(|f| !f.is_empty()).map(|f| format!("{f} -> ")).unwrap_or_default();
        state["plugins"][ai]["shown"] = json!(true);
        lines.push(format!("{name} plugin wisply {from}{VERSION} is installed, new {name} sessions use it"));
    }
    lines
}

pub fn status_line(home: &Path) -> Vec<String> {
    let mut lines = vec![update_line(&load(home), &mode(home), managed(home), now())];
    lines.extend(summary(home));
    lines
}

pub fn update_line(state: &Value, mode: &str, managed: bool, at: u64) -> String {
    if mode == "off" {
        return "update off (config update auto turns it on)".into();
    }
    if !managed {
        return format!("update {mode} | not active here, only the installed wisply in ~/.wisply/bin updates itself");
    }
    let checked = match state["checked"].as_u64() {
        Some(when) => format!("checked {} ago", ago(at.saturating_sub(when))),
        None => "not checked yet".into(),
    };
    let mut parts = vec![format!("update {mode}"), waiting(state, mode, at), checked];
    parts.extend(last_part(state, at));
    parts.join(" | ")
}

fn waiting(state: &Value, mode: &str, at: u64) -> String {
    let staged = state["staged"]["version"].as_str().filter(|v| newer(VERSION, v));
    let latest = state["latest"].as_str().filter(|v| newer(VERSION, v));
    match (staged, latest) {
        (Some(v), _) => {
            let since = at.saturating_sub(state["staged"]["at"].as_u64().unwrap_or(0));
            let force = match FORCE_AFTER.saturating_sub(since) {
                0 => "now at the next free moment".to_string(),
                left => format!("at the latest in {} at a free moment", ago(left)),
            };
            format!("{v} ready, installs after 5 min without commands ({force})")
        }
        (None, Some(v)) if mode == "notify" => format!("{v} available, wisply update installs it"),
        (None, Some(v)) if gave_up(state, v) => format!("{v} failed 3 times, wisply update tries again"),
        (None, Some(v)) => format!("{v} found, downloads on the next check"),
        _ => format!("{VERSION} is current"),
    }
}

fn last_part(state: &Value, at: u64) -> Option<String> {
    let last = &state["last"];
    let (from, to) = (last["from"].as_str()?, last["to"].as_str()?);
    let when = ago(at.saturating_sub(last["at"].as_u64().unwrap_or(0)));
    Some(match last["ok"].as_bool() {
        Some(true) => format!("last {from} -> {to} ok {when} ago"),
        _ => format!("last {from} -> {to} failed {when} ago ({})", last["why"].as_str().unwrap_or("unknown")),
    })
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

    #[test]
    fn news_takes_the_first_sentence_of_the_first_three_items_of_a_version() {
        let log = "# Changelog\n\n## [9.1.0] - x\n\n### Added\n- `a`: first thing. More words.\n  still more.\n\
                   - Second one\n  goes on. Rest\n### Fixed\n- Third.\n- Fourth.\n\n## [9.0.0] - y\n- Old.\n";
        assert_eq!(news(log, "9.1.0"), vec!["`a`: first thing.", "Second one goes on.", "Third."]);
        assert_eq!(news(log, "9.0.0"), vec!["Old."]);
        assert!(news(log, "9.0").is_empty() && news(log, "8.0.0").is_empty());
        let long = format!("## [1.0.0]\n- {}\n", "word ".repeat(40));
        let item = &news(&long, "1.0.0")[0];
        assert!(item.ends_with('…') && item.chars().count() <= NEWS_CHARS, "{item}");
        assert!(!news(CHANGES, VERSION).is_empty(), "CHANGELOG.md needs a section for {VERSION}");
    }

    #[test]
    fn the_update_notice_names_the_news_and_each_updated_plugin_once() {
        let h = home("news");
        finished(&h, "0.0.1", VERSION, true, "");
        let mut s = load(&h);
        s["plugins"] = json!({
            "claude": {"to": VERSION, "from": "3.0.0", "ok": true, "shown": false},
            "codex": {"to": "0.0.1", "ok": true, "shown": false}
        });
        save(&h, &s);
        let text = notice(&h).unwrap();
        assert!(text.contains(&format!("0.0.1 -> {VERSION}")) && text.contains(". New: "), "{text}");
        assert!(text.contains(&format!("Claude plugin wisply 3.0.0 -> {VERSION} is installed")), "{text}");
        assert!(!text.contains("Codex plugin wisply"), "{text}");
        assert_eq!(notice(&h), None);
        fs::remove_dir_all(&h).unwrap();
    }

    #[test]
    fn the_status_line_says_when_a_staged_update_installs() {
        let t = 1_000_000;
        assert_eq!(update_line(&json!({}), "off", true, t), "update off (config update auto turns it on)");
        assert!(update_line(&json!({}), "auto", false, t).contains("not active here"));
        assert_eq!(
            update_line(&json!({}), "auto", true, t),
            format!("update auto | {VERSION} is current | not checked yet")
        );
        let staged = json!({"staged": {"version": "99.0.0", "at": t - 3_600}, "checked": t - 720});
        let line = update_line(&staged, "auto", true, t);
        assert!(
            line.contains("99.0.0 ready") && line.contains("at the latest in 5h") && line.contains("checked 12m ago"),
            "{line}"
        );
        let late = json!({"staged": {"version": "99.0.0", "at": t - FORCE_AFTER}});
        assert!(update_line(&late, "auto", true, t).contains("now at the next free moment"));
        let failed = json!({"latest": "99.0.0", "failed": {"version": "99.0.0", "count": 3},
                            "last": {"from": "1.0.0", "to": "99.0.0", "at": t - 60, "ok": false, "why": "selftest"}});
        let line = update_line(&failed, "auto", true, t);
        assert!(
            line.contains("99.0.0 failed 3 times") && line.contains("last 1.0.0 -> 99.0.0 failed 60s ago (selftest)"),
            "{line}"
        );
        assert!(update_line(&json!({"latest": "99.0.0"}), "notify", true, t).contains("99.0.0 available"));
    }
}
