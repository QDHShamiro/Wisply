use std::fs;
use std::path::Path;

use serde_json::{Map, Value, json};

use super::config::{Config, dir};
use crate::setup::claude_env::{load, save};

const KEY: &str = "statusLine";
const WORD: &str = "statusline";
const PROGRAM: &str = "wisply";

#[derive(Debug, PartialEq)]
pub enum Owner {
    Nobody,
    Ours,
    Foreign(String),
}

#[derive(Debug, PartialEq)]
pub enum On {
    Set,
    Same,
    Updated,
    Replaced,
    Refused(String),
}

#[derive(Debug, PartialEq)]
pub enum Off {
    Removed,
    Restored,
    NotOurs,
    Nothing,
}

pub fn command_line(exe: &Path) -> String {
    let path = exe.to_string_lossy().replace('\\', "/");
    if path.contains(' ') { format!("\"{path}\" {WORD}") } else { format!("{path} {WORD}") }
}

pub fn is_ours(command: &str) -> bool {
    let Some(rest) = command.trim_end().strip_suffix(WORD) else { return false };
    let program = rest.trim_end().trim_matches('"').replace('\\', "/");
    let name = program.rsplit('/').next().unwrap_or("").to_ascii_lowercase();
    !program.is_empty() && (name == PROGRAM || name == format!("{PROGRAM}.exe"))
}

fn command_of(map: &Map<String, Value>) -> Option<String> {
    map.get(KEY)?.get("command")?.as_str().map(str::to_string)
}

pub fn owner(file: &Path) -> Result<Owner, String> {
    let map = load(file)?;
    Ok(match command_of(&map) {
        None if map.get(KEY).is_some_and(|v| !v.is_null()) => Owner::Foreign(map[KEY].to_string()),
        None => Owner::Nobody,
        Some(cmd) if is_ours(&cmd) => Owner::Ours,
        Some(cmd) => Owner::Foreign(cmd),
    })
}

fn previous(home: &Path) -> std::path::PathBuf {
    dir(home).join("previous.json")
}

fn told(home: &Path) -> std::path::PathBuf {
    dir(home).join("told")
}

pub fn turn_on(file: &Path, home: &Path, exe: &Path, force: bool) -> Result<On, String> {
    let mut map = load(file)?;
    let wanted = command_line(exe);
    let current = command_of(&map);
    let mut outcome = On::Set;
    match (&current, map.get(KEY)) {
        (Some(cmd), _) if *cmd == wanted => return Ok(On::Same),
        (Some(cmd), _) if is_ours(cmd) => outcome = On::Updated,
        (_, Some(old)) if !old.is_null() => {
            if !force {
                return Ok(On::Refused(current.clone().unwrap_or_else(|| old.to_string())));
            }
            fs::create_dir_all(dir(home))
                .and_then(|()| fs::write(previous(home), serde_json::to_string_pretty(old).unwrap_or_default()))
                .map_err(|e| format!("cannot save the old status line: {e}"))?;
            outcome = On::Replaced;
        }
        _ => {}
    }
    map.insert(KEY.into(), json!({"type": "command", "command": wanted}));
    save(file, map)?;
    Ok(outcome)
}

pub fn turn_off(file: &Path, home: &Path) -> Result<Off, String> {
    let mut map = load(file)?;
    match owner(file)? {
        Owner::Nobody => return Ok(Off::Nothing),
        Owner::Foreign(_) => return Ok(Off::NotOurs),
        Owner::Ours => {}
    }
    let saved = fs::read_to_string(previous(home)).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok());
    let outcome = match saved {
        Some(old) => {
            map.insert(KEY.into(), old);
            let _ = fs::remove_file(previous(home));
            Off::Restored
        }
        None => {
            map.shift_remove(KEY);
            Off::Removed
        }
    };
    save(file, map)?;
    Ok(outcome)
}

#[derive(Debug, PartialEq)]
pub enum Health {
    Off,
    Unset,
    Own,
    Fine,
    Missing(String),
    Broken(String),
}

pub fn program_of(command: &str) -> Option<std::path::PathBuf> {
    let rest = command.trim_end().strip_suffix(WORD)?;
    let program = rest.trim().trim_matches('"');
    (!program.is_empty()).then(|| std::path::PathBuf::from(program))
}

pub fn health(home: &Path, file: &Path) -> Health {
    let enabled = Config::load(home).enabled;
    let map = match load(file) {
        Ok(map) => map,
        Err(e) => return Health::Broken(e),
    };
    match command_of(&map) {
        Some(cmd) if is_ours(&cmd) => match program_of(&cmd) {
            Some(exe) if !exe.is_file() => Health::Missing(exe.display().to_string()),
            _ if !enabled => Health::Off,
            _ => Health::Fine,
        },
        Some(_) => Health::Own,
        None if map.get(KEY).is_some_and(|v| !v.is_null()) => Health::Own,
        None if enabled => Health::Unset,
        None => Health::Off,
    }
}

pub fn auto(home: &Path, file: &Path, exe: &Path) -> Option<String> {
    if !Config::load(home).enabled || !file.parent().is_some_and(Path::is_dir) {
        return None;
    }
    match turn_on(file, home, exe, false) {
        Ok(On::Set) => Some(format!("status line on in {} (switch off: wisply statusline off)", file.display())),
        Ok(On::Updated) => Some(format!("status line in {} now points to this wisply", file.display())),
        Ok(On::Refused(_)) if !told(home).is_file() => {
            let _ = fs::create_dir_all(dir(home)).and_then(|()| fs::write(told(home), ""));
            Some("kept your own status line (wisply statusline on --force switches to the Wisply one)".into())
        }
        Err(e) => Some(e),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn scene(tag: &str) -> (PathBuf, PathBuf, PathBuf) {
        let root = std::env::temp_dir().join(format!("lt-sl-install-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let claude = root.join("claude");
        fs::create_dir_all(&claude).unwrap();
        (claude.join("settings.json"), root.join("home"), root.join("bin").join("wisply.exe"))
    }

    fn read(file: &Path) -> Value {
        serde_json::from_str(&fs::read_to_string(file).unwrap()).unwrap()
    }

    #[test]
    fn ownership_is_decided_by_the_command() {
        assert!(is_ours("C:/Users/x/.wisply/bin/wisply.exe statusline"));
        assert!(is_ours("\"C:\\Users\\x y\\.wisply\\bin\\wisply.exe\" statusline"));
        assert!(is_ours("/home/x/.wisply/bin/wisply statusline"));
        assert!(!is_ours("powershell -File statusline.ps1"));
        assert!(!is_ours("wisply statusline --other"));
        assert!(!is_ours("/usr/bin/other statusline"));
        assert_eq!(command_line(Path::new("C:\\a b\\wisply.exe")), "\"C:/a b/wisply.exe\" statusline");
    }

    #[test]
    fn on_sets_off_removes_and_other_keys_stay() {
        let (file, home, exe) = scene("basic");
        fs::write(&file, r#"{"env":{"A":"1"},"model":"x"}"#).unwrap();
        assert_eq!(turn_on(&file, &home, &exe, false).unwrap(), On::Set);
        assert_eq!(turn_on(&file, &home, &exe, false).unwrap(), On::Same);
        let v = read(&file);
        assert_eq!(v["env"]["A"], "1");
        assert_eq!(v["model"], "x");
        assert_eq!(v["statusLine"]["type"], "command");
        assert_eq!(owner(&file).unwrap(), Owner::Ours);
        let moved = exe.with_file_name("new").join("wisply.exe");
        assert_eq!(turn_on(&file, &home, &moved, false).unwrap(), On::Updated);
        assert_eq!(turn_off(&file, &home).unwrap(), Off::Removed);
        assert!(read(&file).get("statusLine").is_none() && read(&file)["env"]["A"] == "1");
        assert_eq!(turn_off(&file, &home).unwrap(), Off::Nothing);
        fs::remove_dir_all(file.parent().unwrap().parent().unwrap()).unwrap();
    }

    #[test]
    fn a_foreign_status_line_is_kept_until_forced_and_comes_back_on_off() {
        let (file, home, exe) = scene("foreign");
        let own = r#"{"statusLine":{"type":"command","command":"powershell -File x.ps1","padding":1}}"#;
        fs::write(&file, own).unwrap();
        assert_eq!(turn_on(&file, &home, &exe, false).unwrap(), On::Refused("powershell -File x.ps1".into()));
        assert_eq!(read(&file)["statusLine"]["command"], "powershell -File x.ps1");
        assert_eq!(turn_off(&file, &home).unwrap(), Off::NotOurs);
        assert_eq!(turn_on(&file, &home, &exe, true).unwrap(), On::Replaced);
        assert!(is_ours(read(&file)["statusLine"]["command"].as_str().unwrap()));
        assert_eq!(turn_off(&file, &home).unwrap(), Off::Restored);
        assert_eq!(read(&file)["statusLine"]["padding"], 1);
        assert!(!previous(&home).is_file());
        fs::remove_dir_all(file.parent().unwrap().parent().unwrap()).unwrap();
    }

    #[test]
    fn health_tells_off_unset_own_fine_and_missing_apart() {
        let (file, home, exe) = scene("health");
        fs::create_dir_all(exe.parent().unwrap()).unwrap();
        fs::write(&exe, "").unwrap();
        fs::write(&file, "{}").unwrap();
        assert_eq!(health(&home, &file), Health::Unset);
        turn_on(&file, &home, &exe, false).unwrap();
        assert_eq!(health(&home, &file), Health::Fine);
        fs::remove_file(&exe).unwrap();
        assert!(matches!(health(&home, &file), Health::Missing(_)));
        fs::write(&file, r#"{"statusLine":{"type":"command","command":"x.sh"}}"#).unwrap();
        assert_eq!(health(&home, &file), Health::Own);
        fs::write(&file, "{ nope").unwrap();
        assert!(matches!(health(&home, &file), Health::Broken(_)));
        assert_eq!(program_of("\"C:/a b/wisply.exe\" statusline"), Some(PathBuf::from("C:/a b/wisply.exe")));
        let mut cfg = Config::load(&home);
        cfg.enabled = false;
        cfg.save(&home).unwrap();
        fs::write(&file, "{}").unwrap();
        assert_eq!(health(&home, &file), Health::Off);
        fs::remove_dir_all(file.parent().unwrap().parent().unwrap()).unwrap();
    }

    #[test]
    fn broken_json_is_left_alone_and_auto_respects_the_off_switch() {
        let (file, home, exe) = scene("broken");
        fs::write(&file, "{ nope").unwrap();
        assert!(turn_on(&file, &home, &exe, false).is_err());
        assert_eq!(fs::read_to_string(&file).unwrap(), "{ nope");
        fs::write(&file, "{}").unwrap();
        assert!(auto(&home, &file, &exe).unwrap().starts_with("status line on in"));
        assert_eq!(auto(&home, &file, &exe), None);
        turn_off(&file, &home).unwrap();
        let mut cfg = Config::load(&home);
        cfg.enabled = false;
        cfg.save(&home).unwrap();
        assert_eq!(auto(&home, &file, &exe), None);
        assert!(read(&file).get("statusLine").is_none());
        fs::remove_dir_all(file.parent().unwrap().parent().unwrap()).unwrap();
    }
}
