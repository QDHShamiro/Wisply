use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

pub const SEGMENTS: [&str; 11] =
    ["branch", "sync", "github", "repo", "model", "effort", "context", "tokens", "limit5h", "limitweek", "pc"];
const OFF_BY_DEFAULT: [&str; 1] = ["repo"];
const ASCII: &str = "ascii";

#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    pub enabled: bool,
    pub ascii: bool,
    segments: Map<String, Value>,
}

pub fn dir(home: &Path) -> PathBuf {
    home.join("statusline")
}

fn file(home: &Path) -> PathBuf {
    dir(home).join("config.json")
}

impl Config {
    pub fn defaults() -> Self {
        let segments = SEGMENTS.iter().map(|s| ((*s).to_string(), json!(!OFF_BY_DEFAULT.contains(s)))).collect();
        Config { enabled: true, ascii: false, segments }
    }

    pub fn load(home: &Path) -> Self {
        let mut cfg = Self::defaults();
        let Ok(text) = fs::read_to_string(file(home)) else { return cfg };
        let Ok(Value::Object(saved)) = serde_json::from_str::<Value>(&text) else { return cfg };
        if let Some(on) = saved.get("enabled").and_then(Value::as_bool) {
            cfg.enabled = on;
        }
        if let Some(on) = saved.get(ASCII).and_then(Value::as_bool) {
            cfg.ascii = on;
        }
        if let Some(Value::Object(segs)) = saved.get("segments") {
            for (name, value) in segs {
                if let (true, Some(on)) = (SEGMENTS.contains(&name.as_str()), value.as_bool()) {
                    cfg.segments.insert(name.clone(), json!(on));
                }
            }
        }
        cfg
    }

    pub fn save(&self, home: &Path) -> Result<(), String> {
        let value = json!({"enabled": self.enabled, ASCII: self.ascii, "segments": self.segments});
        let text = serde_json::to_string_pretty(&value).map_err(|e| e.to_string())? + "\n";
        let path = file(home);
        fs::create_dir_all(dir(home))
            .and_then(|()| fs::write(&path, text))
            .map_err(|e| format!("cannot save {}: {e}", path.display()))
    }

    pub fn shown(&self, name: &str) -> bool {
        self.segments.get(name).and_then(Value::as_bool).unwrap_or(false)
    }

    pub fn set(&mut self, name: &str, on: bool) -> Result<(), String> {
        if name == ASCII {
            self.ascii = on;
        } else if SEGMENTS.contains(&name) {
            self.segments.insert(name.to_string(), json!(on));
        } else {
            return Err(format!("unknown segment '{name}', one of: {}, {ASCII}", SEGMENTS.join(", ")));
        }
        Ok(())
    }

    pub fn listing(&self) -> String {
        let word = |on: bool| if on { "on" } else { "off" };
        let mut parts: Vec<String> = SEGMENTS.iter().map(|s| format!("{s} {}", word(self.shown(s)))).collect();
        parts.push(format!("{ASCII} {}", word(self.ascii)));
        parts.join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn home(tag: &str) -> PathBuf {
        let home = std::env::temp_dir().join(format!("lt-sl-config-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(&home).unwrap();
        home
    }

    #[test]
    fn defaults_show_everything_but_the_repo_and_survive_a_round_trip() {
        let h = home("rt");
        let mut cfg = Config::load(&h);
        assert!(cfg.enabled && !cfg.ascii && cfg.shown("branch") && cfg.shown("github") && !cfg.shown("repo"));
        cfg.set("repo", true).unwrap();
        cfg.set("tokens", false).unwrap();
        cfg.set("ascii", true).unwrap();
        cfg.enabled = false;
        cfg.save(&h).unwrap();
        let back = Config::load(&h);
        assert_eq!(back, cfg);
        assert!(back.shown("repo") && !back.shown("tokens") && back.ascii && !back.enabled);
        assert!(cfg.set("nope", true).unwrap_err().starts_with("unknown segment 'nope'"));
        fs::write(file(&h), "not json").unwrap();
        assert_eq!(Config::load(&h), Config::defaults());
        fs::remove_dir_all(&h).unwrap();
    }
}
