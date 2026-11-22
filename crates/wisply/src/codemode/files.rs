use std::fs;
use std::path::{Component, Path, PathBuf};

const READ_MAX: u64 = 16 << 20;
const GLOB_MAX: usize = 5_000;
const SKIP_DIRS: &[&str] = &[".git", "node_modules", "target", ".venv", "__pycache__", ".next", "dist"];

pub fn resolve(cwd: &Path, path: &str) -> PathBuf {
    let path = path.trim();
    let raw = match path.strip_prefix("~/").or_else(|| path.strip_prefix("~\\")) {
        Some(rest) => user_home().map(|h| h.join(rest)).unwrap_or_else(|| PathBuf::from(path)),
        None => PathBuf::from(path),
    };
    let full = if raw.is_absolute() { raw } else { cwd.join(raw) };
    let mut out = PathBuf::new();
    for part in full.components() {
        match part {
            Component::ParentDir => {
                out.pop();
            }
            Component::CurDir => {}
            other => out.push(other.as_os_str()),
        }
    }
    out
}

pub fn user_home() -> Option<PathBuf> {
    std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(PathBuf::from)
}

pub fn shown(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

pub fn inside(path: &Path, roots: &[PathBuf]) -> bool {
    let low = |p: &Path| shown(p).to_lowercase();
    let path = low(path);
    roots
        .iter()
        .map(|r| low(r))
        .any(|root| path == root || path.starts_with(&format!("{}/", root.trim_end_matches('/'))))
}

pub fn read(path: &Path) -> Result<String, String> {
    let meta = fs::metadata(path).map_err(|e| format!("cannot read {} ({e})", shown(path)))?;
    if meta.is_dir() {
        return Err(format!("{} is a folder, use glob('{}/*')", shown(path), shown(path)));
    }
    if meta.len() > READ_MAX {
        return Err(format!("{} is over {} MB, read a part with $`head` or $`sed`", shown(path), READ_MAX >> 20));
    }
    let raw = fs::read(path).map_err(|e| format!("cannot read {} ({e})", shown(path)))?;
    Ok(wisply_squeeze::decode(&raw))
}

pub fn write(path: &Path, text: &str) -> Result<String, String> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| format!("cannot make the folder {} ({e})", shown(dir)))?;
    }
    fs::write(path, text).map_err(|e| format!("cannot write {} ({e})", shown(path)))?;
    Ok(shown(path))
}

pub fn edit(path: &Path, from: &str, to: &str, all: bool) -> Result<String, String> {
    if from.is_empty() {
        return Err("edit needs the old text, use write() for a new file".into());
    }
    let text = read(path)?;
    let crlf = text.contains("\r\n") && !from.contains("\r\n");
    let (from, to) =
        if crlf { (from.replace('\n', "\r\n"), to.replace('\n', "\r\n")) } else { (from.into(), to.into()) };
    let count = text.matches(from.as_str()).count();
    if count == 0 {
        return Err(format!("the old text was not found in {}, read() it first", shown(path)));
    }
    if count > 1 && !all {
        return Err(format!(
            "the old text is in {} {count} times, give more of the lines around it or pass {{all: true}}",
            shown(path)
        ));
    }
    let next = if all { text.replace(from.as_str(), &to) } else { text.replacen(from.as_str(), &to, 1) };
    fs::write(path, next).map_err(|e| format!("cannot write {} ({e})", shown(path)))?;
    Ok(count.to_string())
}

fn options(segment: &str) -> Vec<String> {
    let (Some(open), Some(close)) = (segment.find('{'), segment.find('}')) else {
        return vec![segment.to_string()];
    };
    if close < open {
        return vec![segment.to_string()];
    }
    let (head, tail) = (&segment[..open], &segment[close + 1..]);
    segment[open + 1..close].split(',').flat_map(|mid| options(&format!("{head}{mid}{tail}"))).collect()
}

fn matches(pattern: &[char], name: &[char]) -> bool {
    match (pattern.first(), name.first()) {
        (None, None) => true,
        (Some('*'), _) => matches(&pattern[1..], name) || (!name.is_empty() && matches(pattern, &name[1..])),
        (Some('?'), Some(_)) => matches(&pattern[1..], &name[1..]),
        (Some(p), Some(n)) if p.eq_ignore_ascii_case(n) => matches(&pattern[1..], &name[1..]),
        _ => false,
    }
}

pub(super) fn fits(segment: &str, name: &str) -> bool {
    let name: Vec<char> = name.chars().collect();
    options(segment).iter().any(|option| matches(&option.chars().collect::<Vec<_>>(), &name))
}

fn walk(dir: &Path, segments: &[&str], out: &mut Vec<PathBuf>) {
    if out.len() >= GLOB_MAX {
        return;
    }
    let Some((first, rest)) = segments.split_first() else { return };
    if *first == "**" {
        walk(dir, rest, out);
        for entry in entries(dir).into_iter().filter(|p| p.is_dir()) {
            walk(&entry, segments, out);
        }
        return;
    }
    let plain = !first.contains(['*', '?', '{']);
    let found = if plain { vec![dir.join(first)] } else { entries(dir) };
    for path in found {
        let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        if !(plain && path.exists()) && !(!plain && fits(first, &name)) {
            continue;
        }
        if rest.is_empty() {
            if out.len() < GLOB_MAX {
                out.push(path);
            }
        } else if path.is_dir() {
            walk(&path, rest, out);
        }
    }
}

fn entries(dir: &Path) -> Vec<PathBuf> {
    let Ok(read) = fs::read_dir(dir) else { return Vec::new() };
    let mut list: Vec<PathBuf> = read
        .filter_map(Result::ok)
        .filter(|e| {
            !e.file_type().is_ok_and(|t| t.is_dir()) || !SKIP_DIRS.contains(&e.file_name().to_string_lossy().as_ref())
        })
        .map(|e| e.path())
        .collect();
    list.sort();
    list
}

pub fn glob(cwd: &Path, pattern: &str) -> Result<Vec<String>, String> {
    let pattern = pattern.trim().replace('\\', "/");
    if pattern.is_empty() {
        return Err("glob needs a pattern like 'src/**/*.rs'".into());
    }
    let (root, rest) = split_root(cwd, &pattern);
    let segments: Vec<&str> = rest.split('/').filter(|s| !s.is_empty() && *s != ".").collect();
    let mut out = Vec::new();
    walk(&root, &segments, &mut out);
    let base = shown(cwd).trim_end_matches('/').to_string() + "/";
    let relative = !Path::new(&pattern).is_absolute() && !pattern.starts_with('~');
    Ok(out
        .iter()
        .map(|p| {
            let full = shown(p);
            match full.strip_prefix(&base) {
                Some(short) if relative => short.to_string(),
                _ => full,
            }
        })
        .collect())
}

fn split_root(cwd: &Path, pattern: &str) -> (PathBuf, String) {
    let parts: Vec<&str> = pattern.split('/').collect();
    let fixed = parts.iter().take_while(|p| !p.contains(['*', '?', '{'])).count().min(parts.len().saturating_sub(1));
    let head = parts[..fixed].join("/");
    let rest = parts[fixed..].join("/");
    let root = if head.is_empty() && pattern.starts_with('/') {
        PathBuf::from("/")
    } else {
        resolve(cwd, if head.is_empty() { "." } else { &head })
    };
    (root, rest)
}
