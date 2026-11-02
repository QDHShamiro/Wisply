use std::path::Path;

use serde_json::{Value, json};

use super::claude_env;

const KEY: &str = "autoCompactWindow";
const ENV_KEY: &str = "CLAUDE_CODE_AUTO_COMPACT_WINDOW";
const MIN: i64 = 100_000;
const MAX: i64 = 1_000_000;
const KILO: i64 = 1000;
const MEGA: i64 = 1_000_000;
const BARE_FROM: i64 = 100;
const BARE_TO: i64 = 1000;
const OFF_WORDS: [&str; 4] = ["off", "auto", "none", "default"];

pub fn parse(raw: &str) -> Result<Option<i64>, String> {
    let word = raw.trim().to_lowercase();
    if OFF_WORDS.contains(&word.as_str()) {
        return Ok(None);
    }
    let (digits, unit) = match word.chars().last() {
        Some('k') => (&word[..word.len() - 1], KILO),
        Some('m') => (&word[..word.len() - 1], MEGA),
        _ => (word.as_str(), 1),
    };
    let n: f64 = digits.parse().map_err(|_| format!("'{raw}' is not a size, use 250k, 400000 or off"))?;
    let mut tokens = (n * unit as f64).round() as i64;
    if unit == 1 && (BARE_FROM..=BARE_TO).contains(&tokens) {
        tokens *= KILO;
    }
    if !(MIN..=MAX).contains(&tokens) {
        return Err(format!("{raw} is outside 100k to 1M, the range Claude Code accepts"));
    }
    Ok(Some(tokens))
}

pub fn short(tokens: i64) -> String {
    if tokens % MEGA == 0 {
        format!("{}M", tokens / MEGA)
    } else if tokens % KILO == 0 {
        format!("{}k", tokens / KILO)
    } else {
        tokens.to_string()
    }
}

pub fn current(file: &Path) -> Result<Option<i64>, String> {
    let map = claude_env::load(file)?;
    Ok(map.get(KEY).and_then(Value::as_i64))
}

fn overrides(map: &serde_json::Map<String, Value>) -> Vec<String> {
    let mut out = Vec::new();
    if let Some(value) = map.get("env").and_then(|e| e.get(ENV_KEY)) {
        out.push(format!("env {ENV_KEY} = {value} in the same file beats {KEY}"));
    }
    if let Some(Value::Object(models)) = map.get("modelSettings") {
        let names: Vec<&str> =
            models.iter().filter(|(_, v)| v.get(KEY).is_some()).map(|(name, _)| name.as_str()).collect();
        if !names.is_empty() {
            out.push(format!("modelSettings sets its own window for {} and wins for that model", names.join(", ")));
        }
    }
    out
}

pub fn apply(file: &Path, window: Option<i64>) -> Result<String, String> {
    if !file.parent().is_some_and(Path::is_dir) {
        return Err(format!("{} has no folder, is Claude Code installed?", file.display()));
    }
    let mut map = claude_env::load(file)?;
    let before = map.get(KEY).and_then(Value::as_i64);
    let mut lines = Vec::new();
    match window {
        Some(w) if before == Some(w) => lines.push(format!("{KEY} is already {} in {}", short(w), file.display())),
        Some(w) => {
            map.insert(KEY.into(), json!(w));
            claude_env::save(file, map.clone())?;
            lines.push(format!("set {KEY} = {w} ({}) in {}", short(w), file.display()));
        }
        None if before.is_none() => lines.push(format!("{KEY} is not set in {}, nothing changed", file.display())),
        None => {
            map.shift_remove(KEY);
            claude_env::save(file, map.clone())?;
            lines.push(format!("removed {KEY} from {}, Claude Code uses its own window again", file.display()));
        }
    }
    if window.is_some() {
        lines.extend(overrides(&map));
    }
    Ok(lines.join("\n"))
}

pub fn status(file: &Path) -> String {
    match current(file) {
        Ok(Some(w)) => format!("compact = {} ({KEY} in {})", short(w), file.display()),
        Ok(None) => "compact = off (Claude Code decides, nothing set)".into(),
        Err(e) => e,
    }
}

pub fn command(file: &Path, args: &[String]) -> Result<String, String> {
    match args.first() {
        None => Ok(status(file)),
        Some(raw) => apply(file, parse(raw)?),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn sizes_are_read_like_claude_code_reads_them() {
        assert_eq!(parse("250k"), Ok(Some(250_000)));
        assert_eq!(parse("400000"), Ok(Some(400_000)));
        assert_eq!(parse("250"), Ok(Some(250_000)));
        assert_eq!(parse("1M"), Ok(Some(1_000_000)));
        assert_eq!(parse("0.5m"), Ok(Some(500_000)));
        assert_eq!(parse("off"), Ok(None));
        assert!(parse("50k").is_err() && parse("2M").is_err() && parse("abc").is_err() && parse("").is_err());
        assert_eq!((short(250_000), short(1_000_000), short(123_456)), ("250k".into(), "1M".into(), "123456".into()));
    }

    #[test]
    fn the_window_lands_in_the_settings_file_and_leaves_the_rest() {
        let dir = std::env::temp_dir().join(format!("lt-compact-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("settings.json");
        fs::write(
            &file,
            r#"{"model":"opus","env":{"CLAUDE_CODE_AUTO_COMPACT_WINDOW":"300000"},"modelSettings":{"opus":{"autoCompactWindow":500000}}}"#,
        )
        .unwrap();
        assert_eq!(status(&file), "compact = off (Claude Code decides, nothing set)");
        let said = command(&file, &["250k".into()]).unwrap();
        assert!(said.contains("= 250000 (250k)") && said.contains("env CLAUDE_CODE_AUTO_COMPACT_WINDOW"), "{said}");
        assert!(said.contains("modelSettings sets its own window for opus"), "{said}");
        let map = claude_env::load(&file).unwrap();
        assert_eq!(map["autoCompactWindow"], 250_000);
        assert_eq!(map["model"], "opus");
        assert!(command(&file, &["250k".into()]).unwrap().contains("already 250k"));
        assert!(status(&file).starts_with("compact = 250k"));
        assert!(command(&file, &["off".into()]).unwrap().contains("removed autoCompactWindow"));
        assert!(!claude_env::load(&file).unwrap().contains_key("autoCompactWindow"));
        assert!(command(&file, &["off".into()]).unwrap().contains("nothing changed"));
        assert!(command(&dir.join("nope").join("settings.json"), &["250k".into()]).is_err());
        let _ = fs::remove_dir_all(&dir);
    }
}
