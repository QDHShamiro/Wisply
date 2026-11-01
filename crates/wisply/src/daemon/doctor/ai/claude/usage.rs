use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::files::skill_dirs;
use super::transcripts::Scan;
use super::{Env, Level, Row, at, clean_name, names_line, read_json, row};

const OFF: &str = "off";

fn count(map: Option<&Value>, key: &str) -> u64 {
    map.and_then(|m| m.get(key)).and_then(|e| e.get("usageCount")).and_then(Value::as_u64).unwrap_or(0)
}

pub fn skill_modes(env: &Env) -> BTreeMap<String, String> {
    let mut modes = BTreeMap::new();
    for file in env.settings() {
        let Some(settings) = read_json(&file) else { continue };
        let Some(map) = settings.get("skillOverrides").and_then(Value::as_object) else { continue };
        for (name, value) in map {
            if let Some(mode) = value.as_str() {
                modes.insert(name.clone(), mode.to_string());
            }
        }
    }
    modes
}

pub fn is_off(modes: &BTreeMap<String, String>, name: &str) -> bool {
    modes.get(name).is_some_and(|m| m == OFF)
}

fn skill_rows(env: &Env, config: &Value, scan: &Scan) -> Vec<Row> {
    let modes = skill_modes(env);
    let usage = config.get("skillUsage");
    let (mut unused, mut odd, mut active) = (Vec::new(), 0, 0);
    for dir in skill_dirs(env) {
        let Some(name) = dir.file_name().map(|n| n.to_string_lossy().into_owned()) else { continue };
        if !clean_name(&name) {
            odd += 1;
            continue;
        }
        if is_off(&modes, &name) {
            continue;
        }
        active += 1;
        if count(usage, &name) == 0 && !scan.skills.contains_key(&name) {
            unused.push(name);
        }
    }
    unused.sort();
    let mut rows = Vec::new();
    if unused.is_empty() {
        rows.push(row(Level::Ok, "skills", format!("{active} active skills, each used at least once"), ""));
    } else {
        rows.push(row(
            Level::Warn,
            "skills",
            format!("{} of {active} active skills never used (0 since install): {}", unused.len(), names_line(&unused)),
            "set \"skillOverrides\": {\"<name>\": \"off\"} in ~/.claude/settings.json (undo: remove the entry)",
        ));
    }
    if odd > 0 {
        rows.push(row(Level::Warn, "skills", format!("{odd} skill folders with odd names skipped"), "rename them"));
    }
    rows
}

fn plugin_rows(env: &Env, config: &Value) -> Vec<Row> {
    let mut enabled: BTreeSet<String> = BTreeSet::new();
    for file in env.settings() {
        let Some(settings) = read_json(&file) else { continue };
        for (key, value) in settings.get("enabledPlugins").and_then(Value::as_object).into_iter().flatten() {
            if value.as_bool() == Some(true) {
                enabled.insert(key.clone());
            } else {
                enabled.remove(key);
            }
        }
    }
    let usage = config.get("pluginUsage");
    let idle: Vec<String> = enabled.iter().filter(|k| clean_name(k) && count(usage, k) == 0).cloned().collect();
    if idle.is_empty() {
        return vec![row(Level::Ok, "plugins", format!("{} enabled plugins, none at zero uses", enabled.len()), "")];
    }
    vec![row(
        Level::Warn,
        "plugins",
        format!(
            "{} of {} enabled plugins at 0 uses since install: {} (passive plugins never count, check before removing)",
            idle.len(),
            enabled.len(),
            names_line(&idle)
        ),
        "set \"enabledPlugins\": {\"<name>@<marketplace>\": false} in ~/.claude/settings.json or run /plugin",
    )]
}

fn normalized(name: &str) -> String {
    name.chars().map(|c| if c.is_ascii_alphanumeric() || c == '_' || c == '-' { c } else { '_' }).collect()
}

fn mcp_names(env: &Env, config: &Value) -> Vec<String> {
    let mut names: BTreeSet<String> = BTreeSet::new();
    let project = at(config, &["projects", &env.project_key()]);
    for map in [config.get("mcpServers"), project.and_then(|p| p.get("mcpServers"))] {
        names.extend(map.and_then(Value::as_object).into_iter().flat_map(|m| m.keys().cloned()));
    }
    if let Some(file) = read_json(&env.cwd.join(".mcp.json")) {
        names.extend(file.get("mcpServers").and_then(Value::as_object).into_iter().flat_map(|m| m.keys().cloned()));
    }
    for gone in project.and_then(|p| p.get("disabledMcpServers")).and_then(Value::as_array).into_iter().flatten() {
        if let Some(name) = gone.as_str() {
            names.remove(name);
        }
    }
    names.into_iter().collect()
}

fn mcp_rows(env: &Env, config: &Value, scan: &Scan) -> Vec<Row> {
    let names = mcp_names(env, config);
    let idle: Vec<String> =
        names.iter().filter(|n| clean_name(n) && !scan.mcp.contains_key(&normalized(n))).cloned().collect();
    if idle.is_empty() {
        return vec![row(
            Level::Ok,
            "mcp",
            format!("{} enabled MCP servers, each called in the window", names.len()),
            "",
        )];
    }
    vec![row(
        Level::Warn,
        "mcp",
        format!(
            "{} of {} enabled MCP servers never called in {}: {}",
            idle.len(),
            names.len(),
            scan.window(),
            names_line(&idle)
        ),
        "/mcp disable <name> (this project only, /mcp enable undoes it); deferred tools cost no context, this is only less clutter",
    )]
}

pub fn rows(env: &Env, scan: &Scan) -> Vec<Row> {
    let config = read_json(&env.config).unwrap_or(Value::Null);
    let mut rows = skill_rows(env, &config, scan);
    rows.extend(plugin_rows(env, &config));
    rows.extend(mcp_rows(env, &config, scan));
    rows
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn unused_skills_plugins_and_servers_are_named_and_overrides_respected() {
        let root = std::env::temp_dir().join(format!("lt-cd-usage-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let claude = root.join("claude");
        for name in ["used", "never", "muted", "viaskill"] {
            fs::create_dir_all(claude.join("skills").join(name)).unwrap();
            fs::write(claude.join("skills").join(name).join("SKILL.md"), "---\nname: x\n---\n").unwrap();
        }
        fs::create_dir_all(root.join("proj")).unwrap();
        fs::write(
            claude.join("settings.json"),
            r#"{"skillOverrides":{"muted":"off"},"enabledPlugins":{"a@m":true,"b@m":true,"c@m":false}}"#,
        )
        .unwrap();
        let cwd = root.join("proj");
        let key = cwd.to_string_lossy().replace('\\', "/");
        let config = serde_json::json!({
            "skillUsage": {"used": {"usageCount": 4}},
            "pluginUsage": {"a@m": {"usageCount": 7}, "b@m": {"usageCount": 0}},
            "mcpServers": {"alive.one": {}, "idle": {}, "off": {}},
            "projects": {key: {"disabledMcpServers": ["off"]}}
        });
        fs::write(root.join(".claude.json"), config.to_string()).unwrap();
        let env = Env { user: root.clone(), claude, config: root.join(".claude.json"), cwd };
        let mut scan = Scan::default();
        scan.skills.insert("viaskill".into(), 1);
        scan.mcp.insert("alive_one".into(), 3);
        let rows = rows(&env, &scan);
        let all: String = rows.iter().map(|r| format!("{} {}\n", r.name, r.text)).collect();
        assert!(all.contains("1 of 3 active skills never used (0 since install): never"), "{all}");
        assert!(all.contains("1 of 2 enabled plugins at 0 uses since install: b@m"), "{all}");
        assert!(all.contains("1 of 2 enabled MCP servers never called") && all.contains(": idle"), "{all}");
        assert!(!all.contains("muted") && !all.contains("c@m"), "{all}");
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn server_names_are_normalized_like_tool_names() {
        assert_eq!(normalized("claude.ai Canva"), "claude_ai_Canva");
        assert_eq!(normalized("my-server_1"), "my-server_1");
    }
}
