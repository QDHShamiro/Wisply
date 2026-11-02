use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::config::Config;
use crate::guard::{Guard, inside, norm, path_of};
use crate::log::Log;
use crate::model::{Proc, short_name, size};
use crate::plat::shell;
use crate::walk::{mtime, weigh};

const DAY: u64 = 86_400;
const HOUR: u64 = 3600;
const GRADLE: [&str; 4] = ["build.gradle", "build.gradle.kts", "settings.gradle", "settings.gradle.kts"];
const LOCKS: [&str; 5] = ["package-lock.json", "yarn.lock", "pnpm-lock.yaml", "bun.lockb", "bun.lock"];
const JUNK: [(&str, &[&str]); 5] = [
    ("target", &["Cargo.toml"]),
    ("target", &["pom.xml"]),
    ("build", &GRADLE),
    (".gradle", &GRADLE),
    ("node_modules", &LOCKS),
];
const JUNK_NAMES: [&str; 4] = ["target", "build", ".gradle", "node_modules"];
const MARKERS: [&str; 7] = [
    "Cargo.toml",
    "package.json",
    "build.gradle",
    "build.gradle.kts",
    "settings.gradle",
    "settings.gradle.kts",
    "pom.xml",
];
const SKIP_DIRS: [&str; 12] = [
    "appdata",
    "library",
    ".cache",
    ".cargo",
    ".rustup",
    ".npm",
    ".m2",
    ".git",
    "node_modules",
    "target",
    "build",
    ".gradle",
];
const SCAN_DEPTH: usize = 6;
const GIT_WAIT: Duration = Duration::from_secs(8);
const LISTED: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Temp,
    Cache,
    Dump,
    Build,
    Log,
    Package,
}

impl Kind {
    pub fn word(self) -> &'static str {
        match self {
            Kind::Temp => "temp files",
            Kind::Cache => "caches",
            Kind::Dump => "crash dumps",
            Kind::Build => "old build folders",
            Kind::Log => "old logs",
            Kind::Package => "package caches",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Item {
    pub path: PathBuf,
    pub bytes: u64,
    pub kind: Kind,
    pub tool: Option<&'static str>,
}

#[derive(Clone, Debug, Default)]
pub struct Env {
    pub user: PathBuf,
    pub wisply: PathBuf,
    pub temps: Vec<PathBuf>,
    pub caches: Vec<(PathBuf, Kind)>,
    pub roots: Vec<PathBuf>,
    pub logs: Vec<PathBuf>,
    pub packages: Vec<(PathBuf, &'static str)>,
    pub now: u64,
}

#[derive(Debug, Default, PartialEq)]
pub struct Outcome {
    pub freed: u64,
    pub removed: usize,
    pub skipped: usize,
}

fn var(name: &str) -> Option<PathBuf> {
    std::env::var_os(name).map(PathBuf::from).filter(|p| !p.as_os_str().is_empty())
}

pub fn env_here(user: &Path, wisply: &Path, cfg: &Config, now: u64) -> Env {
    let mut env = Env { user: user.into(), wisply: wisply.into(), now, ..Env::default() };
    env.temps.push(std::env::temp_dir());
    env.roots = if cfg.project_roots.is_empty() {
        vec![user.into()]
    } else {
        cfg.project_roots.iter().map(PathBuf::from).collect()
    };
    env.logs = vec![wisply.join("out"), user.join("agents-work")];
    env.packages = vec![
        (path_of(user, &[".cargo", "registry", "cache"]), "cargo"),
        (path_of(user, &[".cargo", "registry", "src"]), "cargo"),
        (path_of(user, &[".gradle", "caches"]), "gradle"),
    ];
    if cfg!(windows) {
        let local = var("LOCALAPPDATA").unwrap_or_else(|| path_of(user, &["AppData", "Local"]));
        env.caches.push((local.join("CrashDumps"), Kind::Dump));
        env.caches.push((local.join("D3DSCache"), Kind::Cache));
        env.caches.push((path_of(&local, &["Microsoft", "Windows", "INetCache"]), Kind::Cache));
        env.packages.push((local.join("npm-cache"), "npm"));
        env.packages.push((path_of(&local, &["pip", "Cache"]), "pip"));
    } else if cfg!(target_os = "macos") {
        env.caches.push((path_of(user, &["Library", "Logs", "DiagnosticReports"]), Kind::Dump));
        env.packages.push((path_of(user, &[".npm", "_cacache"]), "npm"));
        env.packages.push((path_of(user, &["Library", "Caches", "pip"]), "pip"));
    } else {
        env.caches.push((path_of(user, &[".cache", "thumbnails"]), Kind::Cache));
        env.packages.push((path_of(user, &[".npm", "_cacache"]), "npm"));
        env.packages.push((path_of(user, &[".cache", "pip"]), "pip"));
    }
    env
}

fn tool_names(tool: &str) -> &'static [&'static str] {
    match tool {
        "cargo" => &["cargo", "rustc", "rust-analyzer", "rustup"],
        "gradle" => &["java", "javaw", "gradle", "gradlew", "kotlin-daemon"],
        "npm" => &["node", "npm", "npx", "yarn", "pnpm"],
        "pip" => &["python", "python3", "pip", "pip3", "pythonw"],
        _ => &[],
    }
}

fn busy(tool: &str, procs: &[Proc]) -> bool {
    let names = tool_names(tool);
    procs.iter().any(|p| names.contains(&short_name(&p.name).as_str()))
}

fn children(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir).map(|r| r.flatten().map(|e| e.path()).collect()).unwrap_or_default()
}

fn item(path: PathBuf, kind: Kind, tool: Option<&'static str>, g: &Guard) -> Option<(Item, u64)> {
    if g.blocked(&path).is_some() {
        return None;
    }
    let w = weigh(&path, &[]);
    if !w.complete || (path.is_dir() && g.holds_secret(&path)) {
        return None;
    }
    Some((Item { path, bytes: w.bytes, kind, tool }, w.newest))
}

fn aged(env: &Env, newest: u64, hours: u64) -> bool {
    env.now.saturating_sub(newest) >= hours * HOUR
}

fn from_dirs(env: &Env, dirs: &[(PathBuf, Kind)], cfg: &Config, g: &Guard) -> Vec<Item> {
    let mut out = Vec::new();
    for (dir, kind) in dirs {
        for child in children(dir) {
            if let Some((it, newest)) = item(child, *kind, None, g)
                && aged(env, newest, cfg.temp_hours)
            {
                out.push(it);
            }
        }
    }
    out
}

fn logs(env: &Env, cfg: &Config, g: &Guard) -> Vec<Item> {
    let mut out = Vec::new();
    for dir in &env.logs {
        let wisply_out = norm(dir) == norm(&env.wisply.join("out"));
        for file in children(dir) {
            let name = file.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
            let wanted = wisply_out || name.ends_with(".log") || name.ends_with(".json");
            let Ok(meta) = fs::symlink_metadata(&file) else { continue };
            if !wanted
                || meta.is_dir()
                || g.blocked(&file).is_some()
                || env.now.saturating_sub(mtime(&meta)) < cfg.log_days * DAY
            {
                continue;
            }
            out.push(Item { path: file, bytes: meta.len(), kind: Kind::Log, tool: None });
        }
    }
    out
}

fn packages(env: &Env, procs: &[Proc], g: &Guard) -> Vec<Item> {
    env.packages
        .iter()
        .filter(|(p, tool)| p.is_dir() && !busy(tool, procs))
        .filter_map(|(p, tool)| item(p.clone(), Kind::Package, Some(tool), g).map(|(i, _)| i))
        .collect()
}

fn is_project(dir: &Path) -> bool {
    MARKERS.iter().any(|m| dir.join(m).is_file())
}

fn clean_repo(dir: &Path) -> bool {
    if !dir.join(".git").exists() {
        return true;
    }
    let path = dir.to_string_lossy();
    shell::run("git", &["-C", &path, "status", "--porcelain"], GIT_WAIT).is_some_and(|out| out.trim().is_empty())
}

fn running_in(dir: &Path, procs: &[Proc]) -> bool {
    let d = norm(dir);
    procs.iter().any(|p| !p.exe.is_empty() && inside(&norm(Path::new(&p.exe)), &d))
}

fn junk_in(project: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for (name, needs) in JUNK {
        let dir = project.join(name);
        if !dir.is_dir() || !needs.iter().any(|m| project.join(m).is_file()) {
            continue;
        }
        if name == "build" && dir.join("libs").exists() {
            found.extend(children(&dir).into_iter().filter(|c| c.file_name().is_some_and(|n| n != "libs")));
        } else {
            found.push(dir);
        }
    }
    found
}

fn stale(env: &Env, project: &Path, cfg: &Config) -> bool {
    let w = weigh(project, &[JUNK_NAMES.as_slice(), &[".git"]].concat());
    let git = ["index", "HEAD"]
        .iter()
        .filter_map(|f| fs::metadata(project.join(".git").join(f)).ok())
        .map(|m| mtime(&m))
        .max()
        .unwrap_or(0);
    w.complete && env.now.saturating_sub(w.newest.max(git)) >= cfg.stale_days * DAY
}

fn walk_projects(env: &Env, dir: &Path, depth: usize, cfg: &Config, procs: &[Proc], g: &Guard, out: &mut Vec<Item>) {
    if depth > SCAN_DEPTH || g.blocked(dir).is_some() {
        return;
    }
    if is_project(dir) && stale(env, dir, cfg) && !running_in(dir, procs) && clean_repo(dir) {
        for path in junk_in(dir) {
            if let Some((it, _)) = item(path, Kind::Build, None, g) {
                out.push(it);
            }
        }
    }
    for child in children(dir) {
        let name = child.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
        let real = fs::symlink_metadata(&child).is_ok_and(|m| m.is_dir() && !m.file_type().is_symlink());
        if real && !name.starts_with('.') && !SKIP_DIRS.contains(&name.as_str()) {
            walk_projects(env, &child, depth + 1, cfg, procs, g, out);
        }
    }
}

pub fn plan(env: &Env, cfg: &Config, procs: &[Proc]) -> Vec<Item> {
    let g = Guard::new(&env.user, &env.wisply, cfg);
    let temps: Vec<(PathBuf, Kind)> = env.temps.iter().map(|t| (t.clone(), Kind::Temp)).collect();
    let mut out = from_dirs(env, &temps, cfg, &g);
    out.extend(from_dirs(env, &env.caches, cfg, &g));
    out.extend(logs(env, cfg, &g));
    out.extend(packages(env, procs, &g));
    for root in &env.roots {
        walk_projects(env, root, 0, cfg, procs, &g, &mut out);
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out.dedup_by(|a, b| a.path == b.path);
    out
}

pub fn execute(items: &[Item], env: &Env, cfg: &Config, procs: &[Proc], log: &Log) -> Outcome {
    let g = Guard::new(&env.user, &env.wisply, cfg);
    let mut out = Outcome::default();
    for it in items {
        let tool_busy = it.tool.is_some_and(|t| busy(t, procs));
        let meta = fs::symlink_metadata(&it.path);
        if g.blocked(&it.path).is_some() || tool_busy || meta.is_err() || running_in(&it.path, procs) {
            out.skipped += 1;
            continue;
        }
        let dir = meta.as_ref().is_ok_and(|m| m.is_dir() && !m.file_type().is_symlink());
        let done = if dir {
            fs::remove_dir_all(&it.path)
        } else {
            fs::remove_file(&it.path).or_else(|_| fs::remove_dir(&it.path))
        };
        let left = if done.is_ok() { 0 } else { weigh(&it.path, &[]).bytes };
        let freed = it.bytes.saturating_sub(left);
        if done.is_ok() || freed > 0 {
            out.removed += 1;
            out.freed += freed;
            log.add("clean", &format!("deleted {}", it.kind.word()), &it.path.to_string_lossy(), freed);
        } else {
            out.skipped += 1;
        }
    }
    out
}

pub fn render(items: &[Item], dry: bool) -> String {
    if items.is_empty() {
        return "pc clean | nothing to clean up".into();
    }
    let kinds = [Kind::Temp, Kind::Cache, Kind::Dump, Kind::Build, Kind::Log, Kind::Package];
    let total: u64 = items.iter().map(|i| i.bytes).sum();
    let head = if dry { "pc clean --dry | would free" } else { "pc clean | can free" };
    let mut lines = vec![format!("{head} {} in {} items", size(total), items.len())];
    for k in kinds {
        let of: Vec<&Item> = items.iter().filter(|i| i.kind == k).collect();
        if !of.is_empty() {
            lines.push(format!("{} | {} items | {}", k.word(), of.len(), size(of.iter().map(|i| i.bytes).sum())));
        }
    }
    let mut big: Vec<&Item> = items.iter().collect();
    big.sort_by_key(|i| std::cmp::Reverse(i.bytes));
    for i in big.iter().take(LISTED) {
        lines.push(format!("  {} | {}", size(i.bytes), i.path.display()));
    }
    lines.join("\n")
}

#[cfg(test)]
#[path = "clean_tests.rs"]
mod tests;
