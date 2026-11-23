use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

use super::claude::frontmatter;
use crate::daemon::doctor::{Level, Row, row};

pub const FIX: &str = "--fix";
pub const UNDO: &str = "--undo";
const BACKUP: [&str; 2] = ["doctor", "backup"];
const MANIFEST: &str = "manifest.json";
const UNDO_HINT: &str = "wisply doctor ai --undo puts the old files back";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Check,
    Fix,
    Undo,
}

pub fn split(rest: &[String]) -> (Mode, Vec<String>) {
    let mode = if rest.iter().any(|a| a == UNDO) {
        Mode::Undo
    } else if rest.iter().any(|a| a == FIX) {
        Mode::Fix
    } else {
        Mode::Check
    };
    (mode, rest.iter().filter(|a| *a != FIX && *a != UNDO).cloned().collect())
}

fn backups(home: &Path) -> PathBuf {
    BACKUP.iter().fold(home.to_path_buf(), |p, part| p.join(part))
}

fn fresh_dir(home: &Path) -> PathBuf {
    let mut secs = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    loop {
        let dir = backups(home).join(secs.to_string());
        if !dir.exists() {
            return dir;
        }
        secs += 1;
    }
}

fn save_backup(home: &Path, broken: &[(PathBuf, String, String)]) -> Result<PathBuf, String> {
    let dir = fresh_dir(home);
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let mut list = Vec::new();
    for (n, (path, old, _)) in broken.iter().enumerate() {
        let copy = format!("{n}.md");
        fs::write(dir.join(&copy), old).map_err(|e| e.to_string())?;
        list.push(json!({ "path": path.to_string_lossy(), "copy": copy }));
    }
    fs::write(dir.join(MANIFEST), Value::Array(list).to_string()).map_err(|e| e.to_string())?;
    Ok(dir)
}

pub fn fix_files(home: &Path, files: &[PathBuf]) -> Vec<Row> {
    let broken: Vec<(PathBuf, String, String)> = files
        .iter()
        .filter_map(|p| {
            let old = fs::read_to_string(p).ok()?;
            let new = frontmatter::repair(&old)?;
            Some((p.clone(), old, new))
        })
        .collect();
    if broken.is_empty() {
        return vec![row(
            Level::Ok,
            "fix",
            format!("{} frontmatter files checked, nothing to repair", files.len()),
            "",
        )];
    }
    let dir = match save_backup(home, &broken) {
        Ok(dir) => dir,
        Err(why) => {
            return vec![row(
                Level::Fail,
                "fix",
                format!("backup failed ({why}), nothing changed"),
                "check free disk space",
            )];
        }
    };
    let mut rows = Vec::new();
    let mut fixed = 0;
    for (path, _, new) in &broken {
        match fs::write(path, new) {
            Ok(()) => {
                fixed += 1;
                rows.push(row(
                    Level::Ok,
                    "fix",
                    format!("quoted the broken frontmatter value in {}", path.display()),
                    "",
                ));
            }
            Err(e) => rows.push(row(
                Level::Fail,
                "fix",
                format!("{}: {e}", path.display()),
                "close the program that holds the file and run it again",
            )),
        }
    }
    rows.push(row(Level::Ok, "fix", format!("repaired {fixed} files, backup in {}", dir.display()), UNDO_HINT));
    rows
}

fn newest(home: &Path) -> Option<PathBuf> {
    fs::read_dir(backups(home))
        .ok()?
        .flatten()
        .filter_map(|e| e.file_name().to_str()?.parse::<u64>().ok().map(|n| (n, e.path())))
        .max_by_key(|(n, _)| *n)
        .map(|(_, p)| p)
}

pub fn undo(home: &Path) -> Vec<Row> {
    let Some(dir) = newest(home) else { return vec![row(Level::Ok, "undo", "no backup from --fix to put back", "")] };
    let list: Vec<Value> = fs::read_to_string(dir.join(MANIFEST))
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default();
    let mut rows = Vec::new();
    let mut failed = false;
    for item in &list {
        let (Some(path), Some(copy)) =
            (item.get("path").and_then(Value::as_str), item.get("copy").and_then(Value::as_str))
        else {
            continue;
        };
        let back = fs::read(dir.join(copy)).and_then(|bytes| fs::write(path, bytes));
        match back {
            Ok(()) => rows.push(row(Level::Ok, "undo", format!("put back {path}"), "")),
            Err(e) => {
                failed = true;
                rows.push(row(Level::Fail, "undo", format!("{path}: {e}"), "close the program that holds the file"));
            }
        }
    }
    if !failed {
        let _ = fs::remove_dir_all(&dir);
    }
    if rows.is_empty() {
        rows.push(row(
            Level::Warn,
            "undo",
            format!("backup {} is empty or broken", dir.display()),
            "delete that folder",
        ));
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_are_split_from_tool_names() {
        let args = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(split(&args(&["codex", "--fix"])), (Mode::Fix, args(&["codex"])));
        assert_eq!(split(&args(&["--undo"])), (Mode::Undo, Vec::new()));
        assert_eq!(split(&args(&["claude"])), (Mode::Check, args(&["claude"])));
    }

    #[test]
    fn fix_keeps_a_backup_and_undo_puts_it_back() {
        let home = std::env::temp_dir().join(format!("wisply-fix-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(&home).unwrap();
        let bad = home.join("SKILL.md");
        let good = home.join("ok.md");
        let old = "---\nname: x\ndescription: Use when: it breaks\n---\nbody\n";
        fs::write(&bad, old).unwrap();
        fs::write(&good, "---\nname: y\n---\n").unwrap();
        let rows = fix_files(&home, &[bad.clone(), good.clone()]);
        assert_eq!(rows.len(), 2);
        assert!(rows[1].text.starts_with("repaired 1 files, backup in "));
        assert_eq!(
            fs::read_to_string(&bad).unwrap(),
            "---\nname: x\ndescription: \"Use when: it breaks\"\n---\nbody\n"
        );
        assert!(fix_files(&home, &[bad.clone(), good])[0].text.contains("nothing to repair"));
        let back = undo(&home);
        assert_eq!(back.len(), 1);
        assert_eq!(fs::read_to_string(&bad).unwrap(), old);
        assert!(newest(&home).is_none());
        assert_eq!(undo(&home)[0].text, "no backup from --fix to put back");
        let _ = fs::remove_dir_all(&home);
    }
}
