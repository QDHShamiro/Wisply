use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

use super::away;
use super::cfg::{self, fill};
use super::text::{clip, now, safe};

const DISCORD_EPOCH_MS: u64 = 1_420_070_400_000;
const SNOWFLAKE_SHIFT: u32 = 22;
const ITEM_CHARS: usize = 90;
const HOUR_S: f64 = 3600.0;

fn albion_dir(settings: &Map<String, Value>) -> PathBuf {
    let own = cfg::text(settings, "albion_dir");
    if own.trim().is_empty() {
        wisply_proto::user_dir().join("agents-work").join("albion-watch")
    } else {
        PathBuf::from(own.trim())
    }
}

fn made_ms(id: &str) -> Option<u64> {
    id.parse::<u64>().ok().map(|n| (n >> SNOWFLAKE_SHIFT) + DISCORD_EPOCH_MS)
}

pub fn albion(settings: &Map<String, Value>, since_ms: u64) -> Vec<(String, String)> {
    let Ok(raw) = fs::read_to_string(albion_dir(settings).join("inbox").join("messages.jsonl")) else {
        return Vec::new();
    };
    raw.lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter(|m| m["id"].as_str().and_then(made_ms).is_some_and(|ms| ms > since_ms))
        .map(|m| (m["author"].as_str().unwrap_or("").to_string(), m["text"].as_str().unwrap_or("").to_string()))
        .collect()
}

fn items(text: &str) -> Vec<String> {
    text.lines()
        .filter_map(|l| {
            let rest = l.trim().strip_prefix('#')?;
            let (num, task) = rest.split_once(' ')?;
            num.chars().all(|c| c.is_ascii_digit()).then(|| task.trim_start_matches("[doing] ").to_string())
        })
        .collect()
}

fn todos(home: &Path, settings: &Map<String, Value>) -> Option<String> {
    let (code, out) = crate::client::ask(home, &["todo"]).ok()?;
    let all = items(&out);
    if code != 0 || all.is_empty() {
        return None;
    }
    let shown: Vec<String> =
        all.iter().take(cfg::num(settings, "briefing_items") as usize).map(|t| clip(t, ITEM_CHARS)).collect();
    let head = fill(&cfg::text(settings, "text_briefing_todo"), &[("n", &all.len().to_string())]);
    Some(format!("{head}: {}", shown.join("; ")))
}

pub fn tally(raw: &str) -> (usize, usize) {
    let rows: Vec<Value> = serde_json::from_str(raw).unwrap_or_default();
    let of = |w: &str| rows.iter().filter(|r| r["status"] == w).count();
    (of("running"), of("waiting"))
}

fn agents(home: &Path, settings: &Map<String, Value>) -> Option<String> {
    let (code, out) = crate::client::ask(home, &["agents", "--json"]).ok()?;
    let (running, waiting) = tally(&out);
    (code == 0 && running + waiting > 0).then(|| {
        fill(
            &cfg::text(settings, "text_briefing_agents"),
            &[("running", &running.to_string()), ("waiting", &waiting.to_string())],
        )
    })
}

fn albion_line(home: &Path, settings: &Map<String, Value>) -> Option<String> {
    let last = away::state(home).get("briefed_ms").and_then(Value::as_u64);
    let floor = ((now() - cfg::num(settings, "albion_hours") as f64 * HOUR_S) * 1000.0) as u64;
    let fresh = albion(settings, last.unwrap_or(floor));
    let (author, text) = fresh.last()?;
    let head = fill(&cfg::text(settings, "text_briefing_albion"), &[("n", &fresh.len().to_string())]);
    Some(format!("{head}, zuletzt {author}: {}", clip(text, ITEM_CHARS)))
}

pub fn compose(home: &Path, settings: &Map<String, Value>) -> String {
    let parts: Vec<String> =
        [todos(home, settings), agents(home, settings), albion_line(home, settings)].into_iter().flatten().collect();
    if parts.is_empty() {
        return cfg::text(settings, "text_briefing_empty");
    }
    safe(&parts.join(". "), cfg::num(settings, "text_max") as usize)
}

pub fn done(home: &Path) {
    away::remember(home, json!({"briefed_ms": (now() * 1000.0) as u64}));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("wisply-call-brief-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn snowflakes_become_times_and_only_new_albion_messages_count() {
        assert_eq!(made_ms("1574520071961714811").map(|ms| ms / 86_400_000), Some(20_780));
        let dir = temp("albion");
        fs::create_dir_all(dir.join("inbox")).unwrap();
        let old = r#"{"id":"1573873573624283156","author":"A","text":"alt"}"#;
        let new = r#"{"id":"1574533066716872876","author":"xALBIONx","text":"neu"}"#;
        fs::write(dir.join("inbox").join("messages.jsonl"), format!("{old}\n{new}\nkaputt\n")).unwrap();
        let mut c = cfg::defaults();
        c.insert("albion_dir".into(), json!(dir.display().to_string()));
        let since = made_ms("1574000000000000000").unwrap();
        assert_eq!(albion(&c, since), vec![("xALBIONx".to_string(), "neu".to_string())]);
        assert_eq!(albion(&c, 0).len(), 2);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn todo_and_agent_listings_are_counted() {
        let listing =
            "to-dos:\n  #12 [doing] Plugin bauen\n  #13 Server updaten (Notiz)\nwisply todo <id> shows one in full";
        assert_eq!(items(listing), ["Plugin bauen", "Server updaten (Notiz)"]);
        let rows = r#"[{"status":"running"},{"status":"waiting"},{"status":"running"},{"status":"done"}]"#;
        assert_eq!(tally(rows), (2, 1));
        assert_eq!(tally("kein json"), (0, 0));
    }

    #[test]
    fn a_missing_albion_folder_adds_nothing() {
        let home = temp("empty");
        let dir = temp("none");
        let mut c = cfg::defaults();
        c.insert("albion_dir".into(), json!(dir.display().to_string()));
        assert_eq!(albion_line(&home, &c), None);
        let _ = fs::remove_dir_all(&home);
        let _ = fs::remove_dir_all(&dir);
    }
}
