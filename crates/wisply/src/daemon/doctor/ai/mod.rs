mod claude;
mod generic;

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use super::{Level, Row, print, row};

const CLAUDE: &str = "claude";
const GROUP: &str = "ai";

pub struct Ctx {
    pub user: PathBuf,
    pub cwd: PathBuf,
}

impl Ctx {
    pub fn detect() -> Self {
        let user =
            env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(PathBuf::from).unwrap_or_default();
        Ctx { user, cwd: env::current_dir().unwrap_or_default() }
    }
}

pub fn ids() -> Vec<&'static str> {
    std::iter::once(CLAUDE).chain(generic::CLIS.iter().map(|c| c.id)).collect()
}

pub fn handles(word: &str) -> bool {
    word == GROUP || ids().contains(&word)
}

pub fn find_program(names: &[&str]) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    let suffixes: &[&str] = if cfg!(windows) { &[".exe", ".cmd", ".bat", ""] } else { &[""] };
    for dir in env::split_paths(&path) {
        for name in names {
            for suffix in suffixes {
                let candidate = dir.join(format!("{name}{suffix}"));
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

fn wanted(first: &str, rest: &[String]) -> Result<Vec<String>, String> {
    let mut names: Vec<String> = Vec::new();
    if first != GROUP {
        names.push(first.to_string());
    }
    names.extend(rest.iter().cloned());
    if let Some(bad) = names.iter().find(|n| !ids().contains(&n.as_str())) {
        return Err(format!("unknown tool '{bad}' -> try one of: {}", ids().join(", ")));
    }
    Ok(names)
}

fn tag(rows: Vec<Row>, id: &str) -> Vec<Row> {
    rows.into_iter()
        .map(|mut r| {
            r.text = format!("{id}: {}", r.text);
            r
        })
        .collect()
}

pub fn run(first: &str, rest: &[String]) -> ExitCode {
    let names = match wanted(first, rest) {
        Ok(names) => names,
        Err(why) => {
            eprintln!("err: {why}");
            return ExitCode::from(2);
        }
    };
    let ctx = Ctx::detect();
    let explicit = !names.is_empty();
    let mut rows: Vec<Row> = Vec::new();
    let (mut found, mut missing, mut window) = (Vec::new(), Vec::new(), String::new());
    for id in ids().into_iter().filter(|id| !explicit || names.iter().any(|n| n == id)) {
        let present = if id == CLAUDE { claude::installed() } else { generic::detected(&ctx, id) };
        if !present {
            missing.push(id);
            if explicit {
                rows.push(row(
                    Level::Warn,
                    "ai",
                    format!("{id}: not found on this machine"),
                    "install it or check the name",
                ));
            }
            continue;
        }
        found.push(id);
        let got = if id == CLAUDE {
            let (rows, win) = claude::rows();
            window = win;
            rows
        } else {
            generic::rows(&ctx, id)
        };
        rows.extend(tag(got, id));
    }
    let seen = if missing.is_empty() { String::new() } else { format!(" | not found: {}", missing.join(", ")) };
    let win = if window.is_empty() { String::new() } else { format!(" | claude window: {window}") };
    println!(
        "wisply doctor ai {} | found: {}{seen}{win}",
        env!("CARGO_PKG_VERSION"),
        if found.is_empty() { "none".to_string() } else { found.join(", ") }
    );
    if rows.iter().all(|r| r.level == Level::Ok) {
        rows.push(row(Level::Ok, "ai", "nothing to clean up", ""));
    }
    print(&rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_words_are_checked_and_aliases_work() {
        assert!(handles("ai") && handles("claude") && handles("codex") && !handles("--fix") && !handles("nope"));
        assert_eq!(wanted("ai", &[]).unwrap(), Vec::<String>::new());
        assert_eq!(wanted("codex", &["gemini".to_string()]).unwrap(), vec!["codex", "gemini"]);
        assert!(wanted("ai", &["nope".to_string()]).unwrap_err().starts_with("unknown tool 'nope'"));
    }

    #[test]
    fn rows_are_tagged_with_the_tool() {
        let rows = tag(vec![row(Level::Ok, "config", "fine", "")], "codex");
        assert_eq!(rows[0].text, "codex: fine");
    }
}
