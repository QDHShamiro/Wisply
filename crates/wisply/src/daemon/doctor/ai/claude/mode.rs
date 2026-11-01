use std::path::Path;

use serde_json::Value;

use super::{Env, Level, Row, at, read_json, row};

const AUTO: &str = "auto";

fn mode_in(file: &Path) -> Option<String> {
    let settings = read_json(file)?;
    at(&settings, &["permissions", "defaultMode"]).and_then(Value::as_str).map(str::to_string)
}

fn auto_disabled(env: &Env) -> bool {
    env.settings().iter().filter_map(|f| read_json(f)).any(|s| {
        at(&s, &["permissions", "disableAutoMode"]).is_some_and(|v| !v.is_null())
            || s.get("disableAutoMode").is_some_and(|v| !v.is_null())
    })
}

pub fn rows(env: &Env) -> Vec<Row> {
    let [user, project, local] = env.settings();
    let mine = mode_in(&user);
    let pinned = mode_in(&project).or_else(|| mode_in(&local));
    if auto_disabled(env) {
        return vec![row(Level::Ok, "mode", "auto mode is turned off on purpose (disableAutoMode)", "")];
    }
    if pinned.as_deref() == Some(AUTO) && mine.as_deref() != Some(AUTO) {
        return vec![row(
            Level::Ok,
            "mode",
            "auto is the default here (set by this project, not by your user settings)",
            "",
        )];
    }
    if let Some(pin) = pinned.filter(|p| p != AUTO) {
        return vec![row(Level::Ok, "mode", format!("this project pins its own default mode ({pin})"), "")];
    }
    if mine.as_deref() == Some(AUTO) {
        return vec![row(Level::Ok, "mode", "auto is your default permission mode", "")];
    }
    vec![row(
        Level::Warn,
        "mode",
        format!("default mode is {}, not auto", mine.as_deref().unwrap_or("default")),
        "add \"permissions\": {\"defaultMode\": \"auto\"} to ~/.claude/settings.json (applies to every project)",
    )]
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn env(tag: &str) -> Env {
        let root = std::env::temp_dir().join(format!("lt-cd-mode-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("claude")).unwrap();
        fs::create_dir_all(root.join("proj/.claude")).unwrap();
        Env {
            user: root.clone(),
            claude: root.join("claude"),
            config: root.join(".claude.json"),
            cwd: root.join("proj"),
        }
    }

    #[test]
    fn auto_pins_and_the_kill_switch_are_told_apart() {
        let e = env("a");
        let first = |e: &Env| rows(e).remove(0);
        assert_eq!(first(&e).level, Level::Warn);
        fs::write(e.claude.join("settings.json"), r#"{"permissions":{"defaultMode":"auto"}}"#).unwrap();
        assert!(first(&e).text.starts_with("auto is your default"));
        fs::write(e.cwd.join(".claude/settings.local.json"), r#"{"permissions":{"defaultMode":"plan"}}"#).unwrap();
        assert!(first(&e).text.contains("pins its own default mode (plan)"));
        fs::write(e.cwd.join(".claude/settings.json"), r#"{"permissions":{"disableAutoMode":"disable"}}"#).unwrap();
        assert!(first(&e).text.contains("turned off on purpose"));
        fs::remove_dir_all(e.user).unwrap();
    }
}
