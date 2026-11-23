use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use serde_json::{Value, json};

use super::release::{REPO, newer, parse};
use super::state::{self, VERSION};
use crate::daemon::{claude_exe, clean_claude_env, run_cmd};

const NAME: &str = "wisply";
const LIMIT: Duration = Duration::from_secs(120);
const TRIES: u64 = 3;
const WHY_CHARS: usize = 160;

#[derive(Debug, PartialEq)]
pub struct Install {
    pub market: String,
    pub version: String,
}

#[derive(Debug, PartialEq)]
pub enum Step {
    Current,
    Update,
    Skip(String),
}

pub fn claude_dir() -> PathBuf {
    env::var_os("CLAUDE_CONFIG_DIR").map(PathBuf::from).unwrap_or_else(|| wisply_proto::user_dir().join(".claude"))
}

pub fn codex_dir() -> PathBuf {
    env::var_os("CODEX_HOME").map(PathBuf::from).unwrap_or_else(|| wisply_proto::user_dir().join(".codex"))
}

fn read_json(path: &Path) -> Value {
    fs::read_to_string(path).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(Value::Null)
}

fn behind(have: &str) -> bool {
    parse(have).is_none() || newer(have, VERSION)
}

fn plain(version: (u64, u64, u64)) -> String {
    format!("{}.{}.{}", version.0, version.1, version.2)
}

pub fn installed(plugins: &Value) -> Vec<Install> {
    let prefix = format!("{NAME}@");
    let Some(map) = plugins["plugins"].as_object() else { return Vec::new() };
    map.iter()
        .filter_map(|(key, list)| {
            let market = key.strip_prefix(&prefix)?;
            let user = list.as_array()?.iter().find(|i| i["scope"] == json!("user"))?;
            Some(Install { market: market.to_string(), version: user["version"].as_str().unwrap_or("").to_string() })
        })
        .collect()
}

pub fn offered(location: &Path) -> Option<String> {
    let market = read_json(&location.join(".claude-plugin").join("marketplace.json"));
    let entry = market["plugins"].as_array()?.iter().find(|p| p["name"] == json!(NAME))?;
    let source = &entry["source"];
    if let Some(path) = source.as_str() {
        let manifest = read_json(&location.join(path).join(".claude-plugin").join("plugin.json"));
        return manifest["version"].as_str().and_then(parse).map(plain);
    }
    parse(source["ref"].as_str()?).map(plain)
}

pub fn decide(have: &str, offered: Option<&str>, market: &str, folder: Option<&str>) -> Step {
    if !behind(have) {
        return Step::Current;
    }
    if offered.and_then(parse).is_some_and(|o| Some(o) == parse(VERSION)) {
        return Step::Update;
    }
    let offers = offered.map_or_else(|| "no fixed version".to_string(), |o| format!("only {o}"));
    Step::Skip(match folder {
        Some(dir) => format!(
            "its marketplace {market} is the folder {dir} and offers {offers}; to follow releases: claude plugin marketplace remove {market}, claude plugin marketplace add {REPO}, claude plugin install {NAME}@{NAME}"
        ),
        None => format!("its marketplace {market} offers {offers}"),
    })
}

pub fn codex_markets(config: &str) -> Vec<String> {
    let prefix = format!("{NAME}@");
    let Ok(table) = config.parse::<toml::Table>() else { return Vec::new() };
    let markets = table.get("marketplaces").and_then(toml::Value::as_table);
    let Some(plugins) = table.get("plugins").and_then(toml::Value::as_table) else { return Vec::new() };
    plugins
        .iter()
        .filter_map(|(key, plugin)| {
            let market = key.strip_prefix(&prefix)?;
            let on = plugin.get("enabled").and_then(toml::Value::as_bool).unwrap_or(true);
            let git = markets?.get(market)?.get("source_type")?.as_str()? == "git";
            (on && git).then(|| market.to_string())
        })
        .collect()
}

fn on_path(names: &[&str]) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    env::split_paths(&path).flat_map(|dir| names.iter().map(move |n| dir.join(n))).find(|p| p.is_file())
}

fn codex_cli() -> Option<PathBuf> {
    if cfg!(windows) { on_path(&["codex.exe", "codex.cmd"]) } else { on_path(&["codex"]) }
}

pub fn short(text: &str) -> String {
    let lines: Vec<&str> = text.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    let line = ["denied", "not found", "fatal", "error", "fail"]
        .iter()
        .find_map(|word| lines.iter().find(|l| l.to_lowercase().contains(word)))
        .or(lines.last())
        .copied()
        .unwrap_or("no output");
    let line = line.trim_start_matches(['✘', '×', ' ']);
    line.chars().take(WHY_CHARS).collect()
}

fn git_spots() -> Vec<PathBuf> {
    let base = |var: &str| env::var_os(var).map(PathBuf::from);
    let mut spots: Vec<PathBuf> = [("ProgramFiles", "Git"), ("LOCALAPPDATA", "Programs/Git")]
        .into_iter()
        .filter_map(|(var, dir)| Some(base(var)?.join(dir).join("cmd")))
        .collect();
    if let Some(desktop) = base("LOCALAPPDATA").map(|l| l.join("GitHubDesktop")) {
        let mut apps: Vec<PathBuf> = fs::read_dir(desktop).into_iter().flatten().flatten().map(|e| e.path()).collect();
        apps.retain(|a| a.file_name().is_some_and(|n| n.to_string_lossy().starts_with("app-")));
        apps.sort();
        spots.extend(apps.into_iter().rev().map(|a| a.join("resources").join("app").join("git").join("cmd")));
    }
    spots
}

fn git_dir() -> Option<PathBuf> {
    if on_path(&[if cfg!(windows) { "git.exe" } else { "git" }]).is_some() || !cfg!(windows) {
        return None;
    }
    git_spots().into_iter().find(|dir| dir.join("git.exe").is_file())
}

fn git_missing() -> bool {
    on_path(&[if cfg!(windows) { "git.exe" } else { "git" }]).is_none() && git_dir().is_none()
}

fn cli(exe: &Path, args: &[&str]) -> Result<String, String> {
    let mut cmd = Command::new(exe);
    cmd.args(args).current_dir(wisply_proto::user_dir());
    clean_claude_env(&mut cmd);
    if let Some(dir) = git_dir() {
        let path = env::var_os("PATH").unwrap_or_default();
        let joined = env::join_paths(std::iter::once(dir).chain(env::split_paths(&path))).unwrap_or(path);
        cmd.env("PATH", joined);
    }
    let hint = |why: String| if git_missing() { format!("{why}; git is needed (winget install Git.Git)") } else { why };
    match run_cmd(cmd, LIMIT) {
        Ok(ran) if ran.ok => Ok(ran.out),
        Ok(ran) => Err(hint(short(&format!("{}\n{}", ran.err, ran.out)))),
        Err(fail) => Err(hint(fail.text())),
    }
}

fn record(home: &Path, ai: &str, fields: Value) {
    let mut s = state::load(home);
    if s["plugins"][ai]["to"] != json!(VERSION) {
        s["plugins"][ai] = json!({"to": VERSION});
    }
    if let (Some(entry), Some(add)) = (s["plugins"][ai].as_object_mut(), fields.as_object()) {
        entry.extend(add.clone());
    }
    state::save(home, &s);
}

fn tried_out(home: &Path, ai: &str) -> bool {
    let entry = &state::load(home)["plugins"][ai];
    entry["to"] == json!(VERSION) && entry["ok"] == json!(false) && entry["tries"].as_u64().unwrap_or(0) >= TRIES
}

fn failed(home: &Path, ai: &str, from: &str, why: &str) {
    let tries = state::load(home)["plugins"][ai]["tries"].as_u64().unwrap_or(0);
    record(home, ai, json!({"from": from, "ok": false, "why": why, "tries": tries + 1, "skip": null}));
}

pub fn catch_up(home: &Path, manual: bool) -> Vec<String> {
    let mut said = claude(home, manual);
    said.extend(codex(home, manual));
    said
}

fn claude(home: &Path, manual: bool) -> Vec<String> {
    let dir = claude_dir().join("plugins");
    let known = read_json(&dir.join("known_marketplaces.json"));
    let mut said = Vec::new();
    for install in installed(&read_json(&dir.join("installed_plugins.json"))) {
        let key = format!("{NAME}@{}", install.market);
        if !behind(&install.version) {
            said.push(format!("claude {key} {} is current", install.version));
        } else if manual || !tried_out(home, "claude") {
            said.push(claude_one(home, &dir, &known[&install.market], &install, &key));
        }
    }
    said
}

fn claude_one(home: &Path, dir: &Path, market: &Value, install: &Install, key: &str) -> String {
    let have = install.version.as_str();
    let exe = claude_exe();
    if !exe.is_file() {
        return format!("claude {key} {have}: the claude command was not found");
    }
    let folder =
        (market["source"]["source"] == json!("directory")).then(|| market["source"]["path"].as_str().unwrap_or("?"));
    if folder.is_none()
        && let Err(why) = cli(&exe, &["plugin", "marketplace", "update", &install.market])
    {
        failed(home, "claude", have, &why);
        return format!("claude {key} {have}: marketplace update failed ({why})");
    }
    let offers = market["installLocation"].as_str().map(Path::new).and_then(offered);
    match decide(have, offers.as_deref(), &install.market, folder) {
        Step::Current => format!("claude {key} {have} is current"),
        Step::Skip(why) => {
            record(home, "claude", json!({"from": have, "skip": why}));
            format!("claude {key} {have}: not updated, {why}")
        }
        Step::Update => {
            let ran = cli(&exe, &["plugin", "update", key, "--scope", "user"]);
            let now = installed(&read_json(&dir.join("installed_plugins.json")))
                .into_iter()
                .find(|i| i.market == install.market)
                .map(|i| i.version)
                .unwrap_or_default();
            if behind(&now) {
                let why = ran.err().unwrap_or_else(|| format!("the plugin still reports {now}"));
                failed(home, "claude", have, &why);
                return format!("claude {key} {have}: update to {VERSION} failed ({why})");
            }
            record(
                home,
                "claude",
                json!({"from": have, "market": install.market, "ok": true, "shown": false, "skip": null, "why": null, "tries": 0}),
            );
            format!("claude {key} {have} -> {now}, new Claude sessions use it")
        }
    }
}

fn codex(home: &Path, manual: bool) -> Vec<String> {
    let Ok(config) = fs::read_to_string(codex_dir().join("config.toml")) else { return Vec::new() };
    let markets = codex_markets(&config);
    let entry = &state::load(home)["plugins"]["codex"];
    let done = entry["to"] == json!(VERSION) && entry["ok"] == json!(true);
    if markets.is_empty() || (!manual && (done || tried_out(home, "codex"))) {
        return Vec::new();
    }
    let Some(exe) = codex_cli() else { return vec!["codex: the codex command was not found".into()] };
    let mut said = Vec::new();
    for market in markets {
        match cli(&exe, &["plugin", "marketplace", "upgrade", &market]) {
            Ok(_) => {
                record(home, "codex", json!({"market": market, "ok": true, "shown": false, "why": null, "tries": 0}));
                said.push(format!("codex {NAME}@{market} upgraded for {VERSION}, new Codex sessions use it"));
            }
            Err(why) => {
                failed(home, "codex", "", &why);
                said.push(format!("codex {NAME}@{market}: marketplace upgrade failed ({why})"));
            }
        }
    }
    said
}

pub fn summary(home: &Path) -> Option<String> {
    let s = state::load(home);
    let claude = &s["plugins"]["claude"];
    let fresh = |entry: &Value| entry["to"] == json!(VERSION);
    let mut parts = Vec::new();
    for install in installed(&read_json(&claude_dir().join("plugins").join("installed_plugins.json"))) {
        let extra = match (behind(&install.version), claude["skip"].as_str(), claude["ok"].as_bool()) {
            (false, _, _) => String::new(),
            (true, Some(skip), _) if fresh(claude) => format!(" (not updated: {skip})"),
            (true, _, Some(false)) if fresh(claude) => {
                format!(" (update failed: {})", claude["why"].as_str().unwrap_or("?"))
            }
            _ => format!(" (updates to {VERSION} on the next check)"),
        };
        parts.push(format!("claude {NAME}@{} {}{extra}", install.market, install.version));
    }
    let codex = &s["plugins"]["codex"];
    if let Ok(config) = fs::read_to_string(codex_dir().join("config.toml")) {
        for market in codex_markets(&config) {
            parts.push(match codex["ok"].as_bool() {
                Some(true) if fresh(codex) => format!("codex {NAME}@{market} upgraded for {VERSION}"),
                Some(false) if fresh(codex) => {
                    format!("codex {NAME}@{market} upgrade failed: {}", codex["why"].as_str().unwrap_or("?"))
                }
                _ => format!("codex {NAME}@{market} upgrades on the next check"),
            });
        }
    }
    (!parts.is_empty()).then(|| format!("plugins {}", parts.join(" | ")))
}

#[cfg(test)]
#[path = "plugins_tests.rs"]
mod tests;
