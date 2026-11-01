use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

use super::{Env, Level, Row, at, read_json, row};
use crate::daemon::media::tools;

const PROGRAMS: [&str; 3] = ["claude.exe", "claude.cmd", "claude"];
const VERSION_WAIT: Duration = Duration::from_secs(15);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Native,
    Npm,
    Other,
}

impl Kind {
    fn word(self) -> &'static str {
        match self {
            Kind::Native => "native",
            Kind::Npm => "npm global",
            Kind::Other => "unknown type",
        }
    }
}

pub fn find_on_path() -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    env::split_paths(&path).find_map(|dir| PROGRAMS.iter().map(|p| dir.join(p)).find(|p| p.is_file()))
}

pub fn classify(path: &Path) -> Kind {
    let text = path.to_string_lossy().replace('\\', "/").to_ascii_lowercase();
    if text.contains("/.local/bin/") {
        Kind::Native
    } else if text.contains("node_modules") || text.contains("/npm/") || text.contains("/pnpm/") {
        Kind::Npm
    } else {
        Kind::Other
    }
}

fn expected(method: &str) -> Option<Kind> {
    match method {
        "native" => Some(Kind::Native),
        "global" => Some(Kind::Npm),
        _ => None,
    }
}

fn version(exe: &Path) -> Option<String> {
    let ran = tools::run(exe, &["--version".to_string()], VERSION_WAIT).ok()?;
    ran.out.split_whitespace().next().map(str::to_string)
}

fn launcher(env: &Env) -> PathBuf {
    env.user.join(".local").join("bin").join(if cfg!(windows) { "claude.exe" } else { "claude" })
}

fn on_path(dir: &Path) -> bool {
    let norm = |p: &Path| p.to_string_lossy().replace('\\', "/").trim_end_matches('/').to_ascii_lowercase();
    env::var_os("PATH").is_some_and(|path| env::split_paths(&path).any(|p| norm(&p) == norm(dir)))
}

pub fn rows(env: &Env) -> Vec<Row> {
    let mut rows = Vec::new();
    let method = read_json(&env.config)
        .and_then(|c| at(&c, &["installMethod"]).and_then(|v| v.as_str().map(str::to_string)))
        .unwrap_or_default();
    match find_on_path() {
        None => rows.push(row(Level::Warn, "install", "no claude program found on PATH", "reinstall Claude Code")),
        Some(path) => {
            let kind = classify(&path);
            let ver = version(&path).unwrap_or_else(|| "version unknown".into());
            let text = format!("{ver}, {} ({}), installMethod {}", kind.word(), path.display(), method_word(&method));
            match expected(&method) {
                Some(want) if want != kind && kind != Kind::Other => rows.push(row(
                    Level::Warn,
                    "install",
                    format!("{text}: the running type disagrees with the config"),
                    "claude install",
                )),
                _ => rows.push(row(Level::Ok, "install", text, "")),
            }
            if kind != Kind::Other && env.claude.join("local").is_dir() {
                rows.push(row(
                    Level::Warn,
                    "install",
                    format!("leftover npm-local install in {}", env.claude.join("local").display()),
                    format!("delete the folder {} (reinstalling brings it back)", env.claude.join("local").display()),
                ));
            }
        }
    }
    let native = launcher(env);
    if native.is_file()
        && let Some(dir) = native.parent()
        && !on_path(dir)
    {
        rows.push(row(
            Level::Warn,
            "path",
            format!("{} is not in PATH", dir.display()),
            "add this line to your shell config: export PATH=\"$HOME/.local/bin:$PATH\"",
        ));
    }
    rows
}

fn method_word(method: &str) -> &str {
    if method.is_empty() { "not set" } else { method }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_install_type_comes_from_the_path() {
        assert_eq!(classify(Path::new("C:\\Users\\x\\.local\\bin\\claude.exe")), Kind::Native);
        assert_eq!(classify(Path::new("C:\\Users\\x\\AppData\\Roaming\\npm\\claude.cmd")), Kind::Npm);
        assert_eq!(classify(Path::new("/usr/lib/node_modules/.bin/claude")), Kind::Npm);
        assert_eq!(classify(Path::new("/opt/claude/bin/claude")), Kind::Other);
        assert_eq!(expected("global"), Some(Kind::Npm));
        assert_eq!(expected("native"), Some(Kind::Native));
        assert_eq!(expected("weird"), None);
        assert_eq!(method_word(""), "not set");
    }
}
