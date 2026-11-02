use std::io::{self, Read, Write};
use std::panic;

use serde_json::{Value, json};
use wisply_proto::home;

const LIMIT_SECONDS: f64 = 60.0;
const INPUT_MAX: u64 = 1_000_000;
const OFF_FILE: &str = "sleep-guard-off";
const REASON: &str = "A sleep of a minute or more blocks this session. Use `wisply wait ci` (waits for the CI of this branch and prints one green/red line), `wisply wait until \"<command>\" --every 5s --timeout 10m` (repeats a check until it passes), or run the command with run_in_background and keep working. Short sleeps stay allowed. `touch ~/.wisply/sleep-guard-off` turns this check off.";

fn seconds(token: &str) -> Option<f64> {
    let split = token.find(|c: char| !(c.is_ascii_digit() || c == '.')).unwrap_or(token.len());
    let (digits, unit) = token.split_at(split);
    let n: f64 = digits.parse().ok()?;
    let scale = match unit {
        "" | "s" => 1.0,
        "m" => 60.0,
        "h" => 3600.0,
        "d" => 86_400.0,
        _ => return None,
    };
    Some(n * scale)
}

fn is_sleep(token: &str) -> bool {
    let name = token.rsplit(['/', '\\']).next().unwrap_or(token);
    name == "sleep" || name == "sleep.exe"
}

pub fn long_sleep(command: &str) -> Option<f64> {
    let tokens: Vec<&str> =
        command.split(|c: char| c.is_whitespace() || matches!(c, ';' | '&' | '|' | '(' | ')' | '{' | '}' | '`')).filter(|t| !t.is_empty()).collect();
    let mut worst: Option<f64> = None;
    for (i, token) in tokens.iter().enumerate() {
        if !is_sleep(token) {
            continue;
        }
        let total: f64 = tokens[i + 1..]
            .iter()
            .map_while(|t| seconds(t.trim_matches(|c| c == '"' || c == '\'')))
            .sum();
        if total >= LIMIT_SECONDS {
            worst = Some(worst.map_or(total, |w| w.max(total)));
        }
    }
    worst
}

pub fn verdict(input: &[u8]) -> Option<String> {
    let event: Value = serde_json::from_slice(input).ok()?;
    if event["tool_name"].as_str() != Some("Bash") || event["tool_input"]["run_in_background"].as_bool() == Some(true) {
        return None;
    }
    long_sleep(event["tool_input"]["command"].as_str()?)?;
    Some(
        json!({"hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "deny",
            "permissionDecisionReason": REASON,
        }})
        .to_string(),
    )
}

pub fn pre() {
    panic::set_hook(Box::new(|_| {}));
    let mut input = Vec::new();
    if io::stdin().lock().take(INPUT_MAX).read_to_end(&mut input).is_err() {
        return;
    }
    if home().join(OFF_FILE).exists() {
        return;
    }
    if let Some(reply) = verdict(&input) {
        let mut out = io::stdout().lock();
        let _ = out.write_all(reply.as_bytes());
        let _ = out.flush();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bash(command: &str) -> Vec<u8> {
        json!({"tool_name": "Bash", "tool_input": {"command": command}}).to_string().into_bytes()
    }

    #[test]
    fn long_sleeps_are_found_anywhere() {
        assert_eq!(long_sleep("sleep 60"), Some(60.0));
        assert_eq!(long_sleep("gh run list && sleep 90 && echo x"), Some(90.0));
        assert_eq!(long_sleep("for i in 1 2 3; do sleep 120; done"), Some(120.0));
        assert_eq!(long_sleep("while true; do x; sleep 1m; done"), Some(60.0));
        assert_eq!(long_sleep("sleep 1m 30s"), Some(90.0));
        assert_eq!(long_sleep("/usr/bin/sleep 3600"), Some(3600.0));
        assert_eq!(long_sleep("sleep 30 ; sleep 45"), None);
        assert_eq!(long_sleep("sleep 59.9"), None);
        assert_eq!(long_sleep("sleep 0.5"), None);
        assert_eq!(long_sleep("echo sleeping 600"), None);
        assert_eq!(long_sleep("sleep \"$N\""), None);
    }

    #[test]
    fn deny_names_the_alternatives() {
        let reply: Value = serde_json::from_str(&verdict(&bash("sleep 600")).expect("deny")).expect("json");
        assert_eq!(reply["hookSpecificOutput"]["permissionDecision"], "deny");
        let why = reply["hookSpecificOutput"]["permissionDecisionReason"].as_str().unwrap_or_default();
        assert!(why.contains("wisply wait ci") && why.contains("run_in_background"));
    }

    #[test]
    fn short_sleeps_other_tools_and_background_runs_pass() {
        assert!(verdict(&bash("sleep 5")).is_none());
        assert!(verdict(b"not json").is_none());
        let read = json!({"tool_name": "Read", "tool_input": {"command": "sleep 600"}});
        assert!(verdict(read.to_string().as_bytes()).is_none());
        let bg = json!({"tool_name": "Bash", "tool_input": {"command": "sleep 600", "run_in_background": true}});
        assert!(verdict(bg.to_string().as_bytes()).is_none());
    }
}
