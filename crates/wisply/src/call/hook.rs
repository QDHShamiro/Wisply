use std::io::Read;
use std::path::Path;

use serde_json::{Map, Value, json};

use super::answer::{self, Opts};
use super::away;
use super::cfg::{self, fill};
use super::ring::{self, Kind, Spec};
use super::text::{quiet_now, safe};

const SKIPPED_TOOLS: [&str; 2] = ["AskUserQuestion", "ExitPlanMode"];
const SLACK_S: u64 = 30;

pub fn describe(tool: &str, input: &Value, chars: usize) -> String {
    let pick = |k: &str| input.get(k).and_then(Value::as_str).map(str::to_string);
    let raw = match tool {
        "Bash" | "PowerShell" => pick("command"),
        "Edit" | "Write" | "Read" | "NotebookEdit" => pick("file_path"),
        _ => None,
    };
    safe(&raw.unwrap_or_else(|| input.to_string()), chars)
}

fn decision(allow: bool, why: &str) -> Value {
    let mut verdict = json!({"behavior": if allow { "allow" } else { "deny" }});
    if !allow {
        verdict["message"] = json!(why);
    }
    json!({"hookSpecificOutput": {"hookEventName": "PermissionRequest", "decision": verdict}})
}

fn permission(home: &Path, settings: &Map<String, Value>, input: &Value) -> Option<Value> {
    let tool = input["tool_name"].as_str().unwrap_or("");
    if tool.is_empty() || SKIPPED_TOOLS.contains(&tool) {
        return None;
    }
    let what = describe(tool, &input["tool_input"], cfg::num(settings, "hook_tool_chars") as usize);
    let question = fill(&cfg::text(settings, "text_hook_permission"), &[("tool", tool), ("what", &what)]);
    let choices = vec![cfg::text(settings, "hook_yes"), cfg::text(settings, "hook_no")];
    let opts = Opts {
        question,
        choices: choices.clone(),
        wait_s: Some(cfg::num(settings, "hook_wait_s")),
        channels: None,
        force: false,
    };
    let (done, _) = answer::ask(home, &opts).ok()?;
    let yes = done.answer.as_deref() == Some(choices[0].as_str());
    Some(decision(yes, &cfg::text(settings, "text_hook_denied")))
}

fn notice(home: &Path, settings: &Map<String, Value>, key: &str, what: &str) {
    if away::gap_left(home, settings) > 0 {
        return;
    }
    away::stamp_notice(home);
    let clipped = safe(what, cfg::num(settings, "hook_stop_chars") as usize);
    let spec = Spec {
        kind: Kind::Notice,
        title: cfg::text(settings, "title_notice"),
        text: fill(&cfg::text(settings, key), &[("what", &clipped)]),
        ask: None,
        channels: ring::pick(settings, "channels", None),
    };
    let _ = ring::send(home, settings, &spec);
}

pub fn handle(home: &Path, input: &Value) -> Option<Value> {
    let settings = cfg::load(home);
    if !cfg::on(&settings, "enabled") || quiet_now(&settings) {
        return None;
    }
    let events = cfg::list(&settings, "hook_events");
    let wants = |name: &str| events.iter().any(|e| e == name);
    let event = input["hook_event_name"].as_str().unwrap_or("");
    let relevant = match event {
        "PermissionRequest" => wants("permission"),
        "Stop" => wants("stop"),
        "Notification" => wants("idle") && input["notification_type"] == "idle_prompt",
        _ => false,
    };
    if !relevant || away::gone(home, &settings).is_none() {
        return None;
    }
    match event {
        "PermissionRequest" => permission(home, &settings, input),
        "Stop" => {
            notice(home, &settings, "text_hook_stop", input["last_assistant_message"].as_str().unwrap_or(""));
            None
        }
        _ => {
            notice(home, &settings, "text_hook_idle", input["message"].as_str().unwrap_or(""));
            None
        }
    }
}

pub fn run(home: &Path) -> String {
    let mut raw = String::new();
    let _ = std::io::stdin().read_to_string(&mut raw);
    let Ok(input) = serde_json::from_str::<Value>(&raw) else { return String::new() };
    handle(home, &input).map(|v| v.to_string()).unwrap_or_default()
}

pub fn snippet(home: &Path) -> String {
    let settings = cfg::load(home);
    let exe =
        std::env::current_exe().map(|p| p.display().to_string().replace('\\', "/")).unwrap_or_else(|_| "wisply".into());
    let command = format!("\"{exe}\" call hook");
    let long = cfg::num(&settings, "hook_wait_s") + SLACK_S;
    let entry = |matcher: Option<&str>, secs: u64| {
        let mut e = json!({"hooks": [{"type": "command", "command": command, "timeout": secs}]});
        if let Some(m) = matcher {
            e["matcher"] = json!(m);
        }
        e
    };
    let doc = json!({"hooks": {
        "PermissionRequest": [entry(None, long)],
        "Notification": [entry(Some("idle_prompt"), SLACK_S)],
        "Stop": [entry(None, SLACK_S)]
    }});
    serde_json::to_string_pretty(&doc).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call::net::mock;
    use crate::call::text::now;
    use std::path::PathBuf;

    fn temp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("wisply-call-hook-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn ready(home: &Path, server: &str) {
        cfg::set(
            home,
            &[
                format!("ntfy_server={server}"),
                "ntfy_topic=t-hook".into(),
                "ntfy_poll_s=1".into(),
                "quiet_from=00:00".into(),
                "quiet_to=00:00".into(),
                "hook_wait_s=10".into(),
                "away_after_min=0".into(),
            ],
        )
        .unwrap();
    }

    fn permission_input() -> Value {
        json!({"hook_event_name": "PermissionRequest", "tool_name": "Bash", "tool_input": {"command": "rm -rf build"}})
    }

    fn reply(word: &str, id: &str) -> String {
        json!({"event": "message", "id": id, "time": now() as u64 + 5, "message": word}).to_string()
    }

    #[test]
    fn tools_are_described_short_and_without_secrets() {
        assert_eq!(describe("Bash", &json!({"command": "ls -la"}), 50), "ls -la");
        assert_eq!(describe("Edit", &json!({"file_path": "a/b.rs", "old_string": "x"}), 50), "a/b.rs");
        let key = "ghp_abcdefghijklmnopqrstuvwxyz0123456789";
        let shown = describe("Bash", &json!({"command": format!("git push https://{key}@github.com/x/y")}), 200);
        assert!(!shown.contains(key));
        assert!(describe("Bash", &json!({"command": "x".repeat(500)}), 40).chars().count() <= 40);
    }

    #[test]
    fn the_hook_stays_silent_while_he_is_here() {
        let home = temp("here");
        let srv = mock::start();
        ready(&home, &srv.url);
        assert!(handle(&home, &permission_input()).is_none());
        assert!(srv.seen.lock().unwrap().is_empty());
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn away_rings_and_the_answer_becomes_the_permission_decision() {
        let home = temp("away");
        let srv = mock::start();
        ready(&home, &srv.url);
        away::run(&home, &["on".into()]).unwrap();
        srv.replies.lock().unwrap().push(reply("nein", "m9"));
        let out = handle(&home, &permission_input()).unwrap();
        assert_eq!(out["hookSpecificOutput"]["decision"]["behavior"], "deny");
        assert!(out["hookSpecificOutput"]["decision"]["message"].as_str().unwrap().contains("abgelehnt"));
        srv.replies.lock().unwrap().push(reply("ja", "m10"));
        let out = handle(&home, &permission_input()).unwrap();
        assert_eq!(out["hookSpecificOutput"]["decision"]["behavior"], "allow");
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn stop_notices_respect_the_gap_and_never_block() {
        let home = temp("stop");
        let srv = mock::start();
        ready(&home, &srv.url);
        away::run(&home, &["on".into()]).unwrap();
        let stop = json!({"hook_event_name": "Stop", "last_assistant_message": "Build fertig, 3 Tests gruen"});
        assert!(handle(&home, &stop).is_none());
        assert!(handle(&home, &stop).is_none());
        let seen = srv.seen.lock().unwrap();
        assert_eq!(seen.iter().filter(|(m, ..)| m == "POST").count(), 1);
        let body: Value = serde_json::from_str(&seen[0].2).unwrap();
        assert!(body["message"].as_str().unwrap().contains("Build fertig"));
        assert_eq!(body["priority"], 3);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn the_settings_snippet_lists_the_three_hooks() {
        let home = temp("snippet");
        let doc: Value = serde_json::from_str(&snippet(&home)).unwrap();
        assert_eq!(doc["hooks"]["Notification"][0]["matcher"], "idle_prompt");
        assert!(doc["hooks"]["PermissionRequest"][0]["hooks"][0]["command"].as_str().unwrap().ends_with("call hook"));
        assert_eq!(doc["hooks"]["PermissionRequest"][0]["hooks"][0]["timeout"], 120);
        let _ = std::fs::remove_dir_all(&home);
    }
}
