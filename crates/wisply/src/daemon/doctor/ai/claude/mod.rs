mod context;
mod files;
pub(super) mod frontmatter;
mod install;
mod mode;
mod transcripts;
mod usage;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::super::{Level, Row, row};

const WINDOW: usize = 50;
const SHOWN_NAMES: usize = 8;

pub struct Env {
    pub user: PathBuf,
    pub claude: PathBuf,
    pub config: PathBuf,
    pub cwd: PathBuf,
}

impl Env {
    pub fn detect() -> Self {
        let user =
            env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(PathBuf::from).unwrap_or_default();
        let custom = env::var_os("CLAUDE_CONFIG_DIR").filter(|v| !v.is_empty()).map(PathBuf::from);
        let config = custom.as_ref().map_or_else(|| user.join(".claude.json"), |dir| dir.join(".claude.json"));
        let claude = custom.unwrap_or_else(|| user.join(".claude"));
        Env { user, claude, config, cwd: env::current_dir().unwrap_or_default() }
    }

    pub fn settings(&self) -> [PathBuf; 3] {
        [
            self.claude.join("settings.json"),
            self.cwd.join(".claude").join("settings.json"),
            self.cwd.join(".claude").join("settings.local.json"),
        ]
    }

    pub fn project_key(&self) -> String {
        self.cwd.to_string_lossy().replace('\\', "/")
    }
}

pub fn read_json(path: &Path) -> Option<Value> {
    serde_json::from_str(&fs::read_to_string(path).ok()?).ok()
}

pub fn at<'a>(v: &'a Value, path: &[&str]) -> Option<&'a Value> {
    path.iter().try_fold(v, |node, key| node.get(*key))
}

pub fn clean_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || "_-.:@ ".contains(c))
}

pub fn names_line(names: &[String]) -> String {
    let mut shown: Vec<&str> = names.iter().take(SHOWN_NAMES).map(String::as_str).collect();
    if names.len() > SHOWN_NAMES {
        shown.push("...");
    }
    shown.join(", ")
}

pub fn installed() -> bool {
    let env = Env::detect();
    env.claude.is_dir() || install::find_on_path().is_some()
}

pub fn rows() -> (Vec<Row>, String) {
    let env = Env::detect();
    let scan = transcripts::scan(&env, WINDOW);
    let mut rows: Vec<Row> = Vec::new();
    rows.extend(install::rows(&env));
    rows.extend(files::rows(&env));
    rows.extend(usage::rows(&env, &scan));
    rows.extend(transcripts::rows(&scan));
    rows.extend(context::rows(&env));
    rows.extend(mode::rows(&env));
    (rows, scan.window())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_checked_and_listed_short() {
        assert!(clean_name("git-flow") && clean_name("wisply@wisply") && clean_name("claude.ai Canva"));
        assert!(!clean_name("a\"b") && !clean_name("x$(y)") && !clean_name("") && !clean_name("a;b"));
        let many: Vec<String> = (0..10).map(|n| format!("s{n}")).collect();
        assert_eq!(names_line(&many), "s0, s1, s2, s3, s4, s5, s6, s7, ...");
        assert_eq!(names_line(&many[..2]), "s0, s1");
    }

    #[test]
    fn json_paths_are_read_key_by_key() {
        let v: Value = serde_json::json!({"a": {"b": "c"}});
        assert_eq!(at(&v, &["a", "b"]).and_then(Value::as_str), Some("c"));
        assert!(at(&v, &["a", "x"]).is_none());
    }
}
