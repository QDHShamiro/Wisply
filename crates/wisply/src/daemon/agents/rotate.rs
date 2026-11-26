use std::path::Path;

use serde_json::{Map, Value};

use super::transcript::{self, Stats};
use crate::daemon::memory::handoff;

pub const DEFAULT_TOKENS: i64 = 100_000;
const MIN_TOKENS: i64 = 20_000;
const PROMPT_CHARS: usize = 3_900;
const K: i64 = 1000;

#[derive(Debug, PartialEq)]
pub struct Plan {
    pub prompt: String,
    pub context: i64,
    pub model: String,
    pub effort: String,
}

pub fn threshold(cfg: &Map<String, Value>) -> i64 {
    cfg.get("agents_rotate_tokens").and_then(Value::as_i64).filter(|n| *n >= MIN_TOKENS).unwrap_or(DEFAULT_TOKENS)
}

pub fn enabled(cfg: &Map<String, Value>) -> bool {
    cfg.get("agents_rotate").and_then(Value::as_bool).unwrap_or(false)
}

pub fn prompt(handoff: &str, goal: &str, old: &Path, context: i64, text: &str) -> String {
    let mut head = vec![format!(
        "Wisply session rotation: you are a fresh session that continues the work of an earlier one in this directory. It stopped at the Claude limit with about {}k tokens of context. The handoff below replaces that context.",
        context / K
    )];
    if !goal.is_empty() {
        head.push(format!("Goal condition (keep working until it is met): {goal}"));
    }
    head.push(format!("Old transcript, read it only if the handoff lacks something: {}", old.display()));
    let head = head.join("\n");
    let tail = format!("\n{text}");
    let fixed = head.chars().count() + tail.chars().count() + 1;
    let kept: String = handoff.chars().take(PROMPT_CHARS.saturating_sub(fixed)).collect();
    let mut out = head;
    if !kept.trim().is_empty() {
        out.push('\n');
        out.push_str(kept.trim_end());
    }
    out.push_str(&tail);
    out
}

pub fn build(cfg: &Map<String, Value>, stats: &Stats, path: &Path, text: &str) -> Option<Plan> {
    if !enabled(cfg) || stats.context < threshold(cfg) {
        return None;
    }
    let told = handoff::for_rotation(path);
    if told.is_empty() && stats.goal.is_empty() {
        return None;
    }
    Some(Plan {
        prompt: prompt(&told, &stats.goal, path, stats.context, text),
        context: stats.context,
        model: stats.model.clone(),
        effort: stats.effort.clone(),
    })
}

pub fn plan(cfg: &Map<String, Value>, sid: &str, text: &str) -> Option<Plan> {
    if !enabled(cfg) {
        return None;
    }
    let stats = transcript::stats_of(sid);
    if stats.context < threshold(cfg) {
        return None;
    }
    build(cfg, &stats, &transcript::path_of(sid)?, text)
}

pub fn name(old: &str) -> String {
    let base = old.trim().trim_start_matches('-').trim();
    if base.is_empty() { "rotated".to_string() } else { format!("{base} (rotated)") }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn cfg(pairs: &[(&str, Value)]) -> Map<String, Value> {
        pairs.iter().map(|(k, v)| ((*k).to_string(), v.clone())).collect()
    }

    #[test]
    fn it_is_off_until_switched_on_and_the_threshold_has_a_floor() {
        assert!(!enabled(&cfg(&[])));
        assert!(enabled(&cfg(&[("agents_rotate", json!(true))])));
        assert_eq!(threshold(&cfg(&[])), DEFAULT_TOKENS);
        assert_eq!(threshold(&cfg(&[("agents_rotate_tokens", json!(5))])), DEFAULT_TOKENS);
        assert_eq!(threshold(&cfg(&[("agents_rotate_tokens", json!(150_000))])), 150_000);
    }

    #[test]
    fn the_prompt_carries_goal_path_handoff_and_the_closing_word_inside_the_limit() {
        let long = "x".repeat(9_000);
        let p = prompt(&long, "fix every issue", Path::new("/t/s.jsonl"), 187_500, "weiter");
        assert!(p.contains("about 187k tokens"));
        assert!(p.contains("Goal condition (keep working until it is met): fix every issue"));
        assert!(p.contains("/t/s.jsonl") && p.ends_with("\nweiter"));
        assert!(p.chars().count() <= PROMPT_CHARS + 1, "{}", p.chars().count());
        let bare = prompt("", "", Path::new("/t/s.jsonl"), 100_000, "weiter");
        assert!(!bare.contains("Goal condition") && bare.ends_with("\nweiter"));
    }

    #[test]
    fn a_rotated_session_keeps_its_name_marked() {
        assert_eq!(name("build relay"), "build relay (rotated)");
        assert_eq!(name("--x"), "x (rotated)");
        assert_eq!(name(" "), "rotated");
    }

    #[test]
    fn only_a_big_enough_session_with_something_to_hand_over_rotates() {
        let dir = std::env::temp_dir().join(format!("wisply-rotate-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("s.jsonl");
        let rows = [
            json!({"type": "user", "message": {"content": "Build a cache for the shop prices so the page loads faster"}}),
            json!({"type": "assistant", "message": {"content": [{"type": "text", "text": "Root cause: the price query runs once per item, so I chose one batched query instead."}]}}),
        ];
        let lines: Vec<String> = rows.iter().map(Value::to_string).collect();
        std::fs::write(&file, lines.join("\n") + "\n").unwrap();
        let on = cfg(&[("agents_rotate", json!(true))]);
        let big = Stats {
            context: 150_000,
            goal: "keep it fast".into(),
            model: "claude-sonnet-5-5".into(),
            effort: "high".into(),
            ..Stats::default()
        };
        let plan = build(&on, &big, &file, "weiter").unwrap();
        assert_eq!((plan.context, plan.model.as_str(), plan.effort.as_str()), (150_000, "claude-sonnet-5-5", "high"));
        assert!(plan.prompt.contains("Goal condition (keep working until it is met): keep it fast"));
        assert!(plan.prompt.contains("Build a cache for the shop prices") && plan.prompt.ends_with("\nweiter"));
        let small = Stats { context: 40_000, ..big.clone() };
        assert_eq!(build(&on, &small, &file, "weiter"), None);
        assert_eq!(build(&cfg(&[]), &big, &file, "weiter"), None);
        let empty = dir.join("none.jsonl");
        std::fs::write(&empty, "").unwrap();
        let bare = Stats { context: 150_000, ..Stats::default() };
        assert_eq!(build(&on, &bare, &empty, "weiter"), None);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
