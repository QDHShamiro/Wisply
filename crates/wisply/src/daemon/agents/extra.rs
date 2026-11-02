use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::model::Row;
use super::secret::scrub;
use crate::daemon::config;

const RESULT_FILE: &str = "RESULT.md";
const WORK_FOLDER: &str = "agents-work";
const NAME_PREFIX: &str = "agent-";
const RESULT_LINES: usize = 400;
const LINE_MAX: usize = 400;
const OWNER_MAX: usize = 120;

fn user_home() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(PathBuf::from)
}

fn work_dir(home: &Path) -> Option<PathBuf> {
    let set = config::text(&config::load(home), "agents_work");
    if set.is_empty() { user_home().map(|h| h.join(WORK_FOLDER)) } else { Some(PathBuf::from(set)) }
}

fn stem_ok(stem: &str) -> bool {
    !stem.is_empty()
        && stem.len() <= 64
        && !stem.starts_with('.')
        && stem.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
        && !stem.contains("..")
}

pub fn result_path(home: &Path, name: &str) -> Option<PathBuf> {
    let stem = name.strip_prefix(NAME_PREFIX).unwrap_or(name);
    if !stem_ok(stem) {
        return None;
    }
    let file = work_dir(home)?.join(stem).join(RESULT_FILE);
    file.is_file().then_some(file)
}

pub fn has_result(home: &Path, name: &str) -> bool {
    result_path(home, name).is_some()
}

pub fn result_text(home: &Path, name: &str) -> Option<String> {
    let raw = fs::read_to_string(result_path(home, name)?).ok()?;
    Some(raw.lines().take(RESULT_LINES).map(|l| scrub(l, LINE_MAX)).collect::<Vec<_>>().join("\n"))
}

pub fn build_owner(home: &Path) -> Option<String> {
    let lock = config::text(&config::load(home), "agents_lock");
    if lock.is_empty() {
        return None;
    }
    let dir = Path::new(&lock);
    if !dir.exists() {
        return None;
    }
    let raw = fs::read_to_string(dir.join("owner")).unwrap_or_default();
    let line = scrub(raw.lines().next().unwrap_or(""), OWNER_MAX);
    Some(if line.is_empty() { "busy".into() } else { line })
}

pub fn attach(v: &mut Value, home: &Path, row: &Row) {
    if let Some(text) = result_text(home, &row.name) {
        v["result"] = json!(text);
    }
}

pub fn flag(v: &mut Value, home: &Path, row: &Row) {
    v["has_result"] = json!(has_result(home, &row.name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_plain_names_reach_the_disk() {
        for bad in ["", ".hidden", "a/b", r"a\b", "..", "a..b", "x y", &"x".repeat(65)] {
            assert!(!stem_ok(bad), "{bad}");
        }
        assert!(stem_ok("ar-amc") && stem_ok("w_web.2"));
    }

    #[test]
    fn a_result_file_is_read_scrubbed_and_matched_by_name() {
        let base = std::env::temp_dir().join(format!("wisply-agents-extra-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let work = base.join("work");
        fs::create_dir_all(work.join("job-one")).unwrap();
        fs::write(work.join("job-one").join(RESULT_FILE), "done\ntoken = abcdef123456\nsecond\n").unwrap();
        fs::create_dir_all(base.join("agents")).unwrap();
        let cfg = format!("{{\"agents_work\": {}}}", json!(work.display().to_string()));
        fs::write(base.join("config.json"), cfg).unwrap();
        let text = result_text(&base, "agent-job-one");
        let _ = fs::remove_dir_all(&base);
        let text = text.unwrap_or_default();
        assert!(text.starts_with("done\n") && text.ends_with("second"), "{text}");
        assert!(!text.contains("abcdef123456"), "{text}");
    }
}
