use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

const FILE: &str = "config.json";

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub enabled: bool,
    pub act: bool,
    pub warn: bool,
    pub notify: bool,
    pub statusline: bool,
    pub tick_secs: u64,
    pub ram_warn_pct: f32,
    pub ram_suspend_pct: f32,
    pub cpu_warn_pct: f32,
    pub busy_secs: u64,
    pub disk_busy_pct: f32,
    pub grow_mb: u64,
    pub grow_secs: u64,
    pub notify_gap_secs: u64,
    pub afk_secs: u64,
    pub clean_min_free_gb: u64,
    pub clean_weekly_secs: u64,
    pub stale_days: u64,
    pub log_days: u64,
    pub temp_hours: u64,
    pub trim_min_mb: u64,
    pub trim_every_secs: u64,
    pub ping_host: String,
    pub ping_ms: u64,
    pub builders: Vec<String>,
    pub background: Vec<String>,
    pub protect: Vec<String>,
    pub protect_paths: Vec<String>,
    pub project_roots: Vec<String>,
    pub game_exes: Vec<(String, Vec<String>)>,
}

fn strings(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| (*s).to_string()).collect()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            enabled: true,
            act: false,
            warn: true,
            notify: true,
            statusline: true,
            tick_secs: 5,
            ram_warn_pct: 85.0,
            ram_suspend_pct: 92.0,
            cpu_warn_pct: 80.0,
            busy_secs: 30,
            disk_busy_pct: 95.0,
            grow_mb: 800,
            grow_secs: 120,
            notify_gap_secs: 1800,
            afk_secs: 300,
            clean_min_free_gb: 25,
            clean_weekly_secs: 604_800,
            stale_days: 30,
            log_days: 14,
            temp_hours: 24,
            trim_min_mb: 300,
            trim_every_secs: 300,
            ping_host: "1.1.1.1".into(),
            ping_ms: 1500,
            builders: strings(&[
                "cargo", "rustc", "gradle", "gradlew", "node", "mvn", "javac", "claude", "kotlinc", "tsc",
            ]),
            background: strings(&["onedrive", "searchindexer", "searchhost", "mousocoreworker", "tiworker", "spotify"]),
            protect: strings(&[
                "javaw",
                "java",
                "wisply",
                "wisply-task",
                "explorer",
                "svchost",
                "system",
                "csrss",
                "winlogon",
                "lsass",
                "services",
                "dwm",
                "smss",
                "wininit",
                "init",
                "systemd",
                "launchd",
                "kernel_task",
                "sshd",
                "modrinthapp",
                "minecraft",
            ]),
            protect_paths: Vec::new(),
            project_roots: Vec::new(),
            game_exes: vec![("minecraft".into(), strings(&["javaw", "java"]))],
        }
    }
}

pub fn dir(home: &Path) -> PathBuf {
    home.join("pc")
}

fn list(m: &Map<String, Value>, key: &str) -> Option<Vec<String>> {
    let items = m.get(key)?.as_array()?;
    Some(items.iter().filter_map(|v| v.as_str().map(str::to_lowercase)).collect())
}

fn paths(m: &Map<String, Value>, key: &str) -> Option<Vec<String>> {
    let items = m.get(key)?.as_array()?;
    Some(items.iter().filter_map(|v| v.as_str().map(String::from)).collect())
}

impl Config {
    pub fn parse(text: &str) -> Config {
        let mut c = Config::default();
        let Ok(Value::Object(m)) = serde_json::from_str::<Value>(text) else { return c };
        let flag = |k: &str, to: &mut bool| {
            if let Some(v) = m.get(k).and_then(Value::as_bool) {
                *to = v;
            }
        };
        let num = |k: &str, to: &mut u64| {
            if let Some(v) = m.get(k).and_then(Value::as_u64) {
                *to = v;
            }
        };
        let pct = |k: &str, to: &mut f32| {
            if let Some(v) = m.get(k).and_then(Value::as_f64) {
                *to = v as f32;
            }
        };
        flag("enabled", &mut c.enabled);
        flag("act", &mut c.act);
        flag("warn", &mut c.warn);
        flag("notify", &mut c.notify);
        flag("statusline", &mut c.statusline);
        num("tick_secs", &mut c.tick_secs);
        num("busy_secs", &mut c.busy_secs);
        num("grow_mb", &mut c.grow_mb);
        num("grow_secs", &mut c.grow_secs);
        num("notify_gap_secs", &mut c.notify_gap_secs);
        num("afk_secs", &mut c.afk_secs);
        num("clean_min_free_gb", &mut c.clean_min_free_gb);
        num("clean_weekly_secs", &mut c.clean_weekly_secs);
        num("stale_days", &mut c.stale_days);
        num("log_days", &mut c.log_days);
        num("temp_hours", &mut c.temp_hours);
        num("trim_min_mb", &mut c.trim_min_mb);
        num("trim_every_secs", &mut c.trim_every_secs);
        num("ping_ms", &mut c.ping_ms);
        pct("ram_warn_pct", &mut c.ram_warn_pct);
        pct("ram_suspend_pct", &mut c.ram_suspend_pct);
        pct("cpu_warn_pct", &mut c.cpu_warn_pct);
        pct("disk_busy_pct", &mut c.disk_busy_pct);
        if let Some(host) = m.get("ping_host").and_then(Value::as_str).filter(|h| !h.trim().is_empty()) {
            c.ping_host = host.trim().to_string();
        }
        if let Some(v) = list(&m, "builders") {
            c.builders = v;
        }
        if let Some(v) = list(&m, "background") {
            c.background = v;
        }
        if let Some(v) = list(&m, "protect") {
            c.protect.extend(v);
        }
        if let Some(v) = paths(&m, "protect_paths") {
            c.protect_paths = v;
        }
        if let Some(v) = paths(&m, "project_roots") {
            c.project_roots = v;
        }
        if let Some(Value::Object(games)) = m.get("game_exes") {
            for (game, exes) in games {
                let names: Option<Vec<String>> =
                    exes.as_array().map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_lowercase)).collect());
                if let Some(names) = names {
                    let game = game.to_lowercase();
                    c.game_exes.retain(|(g, _)| *g != game);
                    c.game_exes.push((game, names));
                }
            }
        }
        c.tick_secs = c.tick_secs.max(1);
        c
    }

    pub fn load(home: &Path) -> Config {
        fs::read_to_string(dir(home).join(FILE)).map(|t| Config::parse(&t)).unwrap_or_default()
    }

    pub fn set_flag(home: &Path, key: &str, on: bool) -> Result<(), String> {
        let path = dir(home).join(FILE);
        let mut m = match fs::read_to_string(&path).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok()) {
            Some(Value::Object(m)) => m,
            _ => Map::new(),
        };
        m.insert(key.into(), json!(on));
        let text = serde_json::to_string_pretty(&Value::Object(m)).map_err(|e| e.to_string())?;
        fs::create_dir_all(dir(home))
            .and_then(|()| fs::write(&path, text))
            .map_err(|e| format!("cannot save {}: {e}", path.display()))
    }

    pub fn game_procs(&self, game: &str) -> Vec<String> {
        let key = game.trim_start_matches("fullscreen ").to_lowercase();
        let key = key.trim_end_matches(".exe");
        match self.game_exes.iter().find(|(g, _)| g == key) {
            Some((_, exes)) => exes.clone(),
            None => vec![key.to_string()],
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "act {} | warn {} | notify {} | tick {} s | ram {}% | cpu {}% for {} s | afk {} s | clean below {} GB free | ping {}",
            on(self.act),
            on(self.warn),
            on(self.notify),
            self.tick_secs,
            self.ram_warn_pct,
            self.cpu_warn_pct,
            self.busy_secs,
            self.afk_secs,
            self.clean_min_free_gb,
            self.ping_host
        )
    }
}

pub fn on(v: bool) -> &'static str {
    if v { "on" } else { "off" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_users_watch_but_do_not_act() {
        let c = Config::default();
        assert!(c.enabled && c.warn && !c.act && c.tick_secs == 5);
    }

    #[test]
    fn the_file_overrides_defaults_and_keeps_the_rest() {
        let c = Config::parse(
            r#"{"act":true,"tick_secs":0,"ram_warn_pct":70,"protect":["MyGame"],"game_exes":{"Minecraft":["Javaw","X"]},"ping_host":" hugosmp "}"#,
        );
        assert!(c.act && c.tick_secs == 1 && c.ram_warn_pct == 70.0 && c.ping_host == "hugosmp");
        assert!(c.protect.contains(&"mygame".into()) && c.protect.contains(&"javaw".into()));
        assert_eq!(c.game_procs("minecraft"), ["javaw", "x"]);
        assert_eq!(c.game_procs("fullscreen Doom.exe"), ["doom"]);
        assert_eq!(Config::parse("not json"), Config::default());
    }

    #[test]
    fn switching_act_is_saved() {
        let home = std::env::temp_dir().join(format!("wpc-cfg-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        Config::set_flag(&home, "act", true).unwrap();
        assert!(Config::load(&home).act);
        Config::set_flag(&home, "act", false).unwrap();
        assert!(!Config::load(&home).act);
        fs::remove_dir_all(&home).unwrap();
    }
}
