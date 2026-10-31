use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub const SKIP_DIRS: &[&str] = &["target", "build", "out", "dist", "node_modules", "__pycache__", "venv", "coverage"];
const KEEP_DOT_DIRS: &[&str] = &[".github"];
pub const CODE_EXT: &[&str] =
    &["rs", "java", "kt", "py", "js", "ts", "tsx", "jsx", "mjs", "cjs", "go", "cs", "lua", "vue", "svelte", "sh"];
pub const WEB_EXT: &[&str] = &["html", "htm", "css", "scss", "vue", "svelte", "jsx", "tsx"];
pub const TEXT_EXT: &[&str] = &[
    "rs",
    "java",
    "kt",
    "py",
    "js",
    "ts",
    "tsx",
    "jsx",
    "mjs",
    "cjs",
    "go",
    "cs",
    "lua",
    "vue",
    "svelte",
    "sh",
    "html",
    "htm",
    "css",
    "scss",
    "md",
    "yml",
    "yaml",
    "json",
    "toml",
    "xml",
    "properties",
    "sql",
    "ps1",
    "bat",
    "cmd",
    "txt",
    "gradle",
    "kts",
    "svg",
];
pub const IMAGE_EXT: &[&str] = &["png", "jpg", "jpeg", "webp", "gif", "svg"];
const MARKERS: &[&str] = &[
    "Cargo.toml",
    "pom.xml",
    "build.gradle",
    "build.gradle.kts",
    "package.json",
    "pyproject.toml",
    "requirements.txt",
    "setup.py",
    "SKILL.md",
    "fabric.mod.json",
    ".git",
];
pub const WALK_MAX: usize = 20_000;
const READ_MAX: u64 = 2_000_000;

#[derive(Clone)]
struct Ignore {
    pat: String,
    ext: bool,
    dir_only: bool,
    keep: bool,
    anchored: bool,
}

fn parse_ignore(text: &str) -> Vec<Ignore> {
    let glob = ['*', '?', '[', '/', '\\'];
    text.lines()
        .filter_map(|l| {
            let l = l.trim();
            if l.is_empty() || l.starts_with('#') {
                return None;
            }
            let (keep, l) = l.strip_prefix('!').map_or((false, l), |r| (true, r));
            let (dir_only, l) = l.strip_suffix('/').map_or((false, l), |r| (true, r));
            let l = l.strip_prefix("**/").unwrap_or(l);
            let (anchored, l) = l.strip_prefix('/').map_or((false, l), |r| (true, r));
            if let Some(e) = l.strip_prefix("*.")
                && !e.is_empty()
                && !e.contains(glob)
            {
                return Some(Ignore { pat: format!(".{e}"), ext: true, dir_only, keep, anchored });
            }
            (!l.is_empty() && !l.contains(glob)).then(|| Ignore {
                pat: l.to_string(),
                ext: false,
                dir_only,
                keep,
                anchored,
            })
        })
        .collect()
}

fn ignored(list: &[Ignore], name: &str, dir: bool) -> bool {
    list.iter()
        .rev()
        .find(|i| (!i.dir_only || dir) && if i.ext { name.ends_with(&i.pat) } else { name == i.pat })
        .is_some_and(|i| !i.keep)
}

type Rules = Rc<Vec<Ignore>>;

fn with_local(dir: &Path, names: &[(PathBuf, String, bool)], up: &Rules) -> (Rules, Rules) {
    let has = |n: &str, d: bool| names.iter().any(|(_, name, is_dir)| name == n && *is_dir == d);
    let mut extra = Vec::new();
    for (on, file) in [
        (has(".git", true), dir.join(".git").join("info").join("exclude")),
        (has(".gitignore", false), dir.join(".gitignore")),
    ] {
        if on && let Ok(t) = fs::read_to_string(file) {
            extra.extend(parse_ignore(&t));
        }
    }
    if extra.is_empty() {
        return (Rc::clone(up), Rc::clone(up));
    }
    let here: Vec<Ignore> = up.iter().chain(&extra).cloned().collect();
    let down: Vec<Ignore> = up.iter().chain(extra.iter().filter(|i| !i.anchored)).cloned().collect();
    (Rc::new(here), Rc::new(down))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Pack {
    Code,
    Rust,
    JavaPlugin,
    Mod,
    Web,
    Python,
    Docs,
    Images,
    McGui,
    Skill,
}

impl Pack {
    pub const ALL: [Pack; 10] = [
        Pack::Code,
        Pack::Rust,
        Pack::JavaPlugin,
        Pack::Mod,
        Pack::Web,
        Pack::Python,
        Pack::Docs,
        Pack::Images,
        Pack::McGui,
        Pack::Skill,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Pack::Code => "code",
            Pack::Rust => "rust",
            Pack::JavaPlugin => "java-plugin",
            Pack::Mod => "mod",
            Pack::Web => "web",
            Pack::Python => "python",
            Pack::Docs => "docs",
            Pack::Images => "images",
            Pack::McGui => "mc-gui",
            Pack::Skill => "skill",
        }
    }

    pub fn from_name(name: &str) -> Option<Pack> {
        Pack::ALL.into_iter().find(|p| p.name() == name.trim().to_lowercase())
    }
}

pub fn ext(p: &Path) -> String {
    p.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default()
}

pub fn has_ext(p: &Path, list: &[&str]) -> bool {
    list.contains(&ext(p).as_str())
}

pub fn walk(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut todo: Vec<(PathBuf, Rules)> = Vec::new();
    for p in paths {
        if p.is_file() {
            out.push(p.clone());
        } else {
            todo.push((p.clone(), Rc::default()));
        }
    }
    while let Some((dir, up)) = todo.pop() {
        if out.len() >= WALK_MAX {
            break;
        }
        let Ok(list) = fs::read_dir(&dir) else { continue };
        let names: Vec<(PathBuf, String, bool)> = list
            .flatten()
            .filter_map(|e| {
                let t = e.file_type().ok()?;
                (t.is_dir() || t.is_file()).then(|| (e.path(), e.file_name().to_string_lossy().to_string(), t.is_dir()))
            })
            .collect();
        let (here, down) = with_local(&dir, &names, &up);
        for (path, name, is_dir) in names {
            if ignored(&here, &name, is_dir) {
                continue;
            }
            if !is_dir {
                out.push(path);
            } else if !SKIP_DIRS.contains(&name.as_str())
                && (!name.starts_with('.') || KEEP_DOT_DIRS.contains(&name.as_str()))
            {
                todo.push((path, Rc::clone(&down)));
            }
        }
    }
    out.sort();
    out
}

pub fn read(p: &Path) -> Option<String> {
    if fs::metadata(p).ok()?.len() > READ_MAX {
        return None;
    }
    let raw = fs::read(p).ok()?;
    if raw.iter().take(8192).any(|b| *b == 0) {
        return None;
    }
    Some(String::from_utf8(raw).unwrap_or_else(|e| e.into_bytes().iter().map(|b| *b as char).collect()))
}

pub fn root_of(p: &Path) -> PathBuf {
    let start = if p.is_dir() { p.to_path_buf() } else { p.parent().map(Path::to_path_buf).unwrap_or_default() };
    for d in start.ancestors() {
        if MARKERS.iter().any(|m| d.join(m).exists()) {
            return d.to_path_buf();
        }
    }
    start
}

fn workspace(d: &Path) -> bool {
    let has = |f: &str, needle: &str| fs::read_to_string(d.join(f)).is_ok_and(|t| t.contains(needle));
    has("Cargo.toml", "[workspace]")
        || has("pom.xml", "<modules>")
        || has("settings.gradle", "include")
        || has("settings.gradle.kts", "include")
        || has("package.json", "\"workspaces\"")
}

pub fn root_for(paths: &[PathBuf]) -> Option<PathBuf> {
    let first = paths.first()?;
    let mut common = if first.is_dir() { first.clone() } else { first.parent()?.to_path_buf() };
    for p in &paths[1..] {
        while !p.starts_with(&common) && common.pop() {}
    }
    let base = root_of(&common);
    if base.join(".git").exists() {
        return Some(base);
    }
    for d in base.ancestors().skip(1) {
        if workspace(d) {
            return Some(d.to_path_buf());
        }
        if d.join(".git").exists() {
            break;
        }
    }
    Some(base)
}

pub fn is_test(p: &Path) -> bool {
    let s = slashed(p).to_lowercase();
    s.contains("/tests/")
        || s.contains("/test/")
        || s.contains(".test.")
        || s.contains("_test.")
        || s.contains(".spec.")
}

pub fn is_minified(p: &Path) -> bool {
    p.file_name().is_some_and(|n| n.to_string_lossy().to_lowercase().contains(".min."))
}

pub fn slashed(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

pub fn rel(root: &Path, p: &Path) -> String {
    slashed(p.strip_prefix(root).unwrap_or(p))
}

pub fn packs_of(root: &Path, files: &[PathBuf]) -> BTreeSet<Pack> {
    let mut out = BTreeSet::new();
    let at = |m: &str| root.join(m).exists();
    if at("Cargo.toml") {
        out.insert(Pack::Rust);
    }
    if at("package.json") || at("index.html") {
        out.insert(Pack::Web);
    }
    if at("fabric.mod.json") || at("src/main/resources/fabric.mod.json") {
        out.insert(Pack::Mod);
    }
    if at("SKILL.md") {
        out.insert(Pack::Skill);
    }
    for f in files {
        let e = ext(f);
        let name = f.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
        let path = slashed(f).to_lowercase();
        match e.as_str() {
            "rs" => {
                out.insert(Pack::Rust);
            }
            "py" => {
                out.insert(Pack::Python);
            }
            "md" => {
                out.insert(Pack::Docs);
            }
            _ => {}
        }
        if CODE_EXT.contains(&e.as_str()) {
            out.insert(Pack::Code);
        }
        if WEB_EXT.contains(&e.as_str()) {
            out.insert(Pack::Web);
        }
        if IMAGE_EXT.contains(&e.as_str()) {
            out.insert(Pack::Images);
        }
        if name == "plugin.yml" || name == "paper-plugin.yml" {
            out.insert(Pack::JavaPlugin);
        }
        if name == "fabric.mod.json" {
            out.insert(Pack::Mod);
        }
        if (e == "yml" || e == "yaml") && (path.contains("/gui/") || path.contains("/guis/") || name == "messages.yml")
        {
            out.insert(Pack::McGui);
        }
    }
    out
}

pub fn quick_packs(root: &Path) -> BTreeSet<Pack> {
    let mut out = BTreeSet::new();
    let at = |m: &str| root.join(m).exists();
    if at("Cargo.toml") {
        out.insert(Pack::Rust);
    }
    if at("package.json") || at("index.html") {
        out.insert(Pack::Web);
    }
    if at("src/main/resources/plugin.yml") || at("src/main/resources/paper-plugin.yml") || at("plugin.yml") {
        out.insert(Pack::JavaPlugin);
        out.insert(Pack::McGui);
    }
    if at("fabric.mod.json") || at("src/main/resources/fabric.mod.json") {
        out.insert(Pack::Mod);
    }
    if at("pyproject.toml") || at("requirements.txt") || at("setup.py") {
        out.insert(Pack::Python);
    }
    if at("SKILL.md") {
        out.insert(Pack::Skill);
    }
    if !out.is_empty() {
        out.insert(Pack::Code);
        out.insert(Pack::Docs);
        out.insert(Pack::Images);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packs_follow_the_files() {
        let root = Path::new("/nowhere");
        let files: Vec<PathBuf> =
            ["a/src/main.rs", "a/src/main/resources/plugin.yml", "a/gui/shop.yml", "a/x.png", "a/README.md"]
                .iter()
                .map(PathBuf::from)
                .collect();
        let got: Vec<&str> = packs_of(root, &files).into_iter().map(Pack::name).collect();
        assert_eq!(got, ["code", "rust", "java-plugin", "docs", "images", "mc-gui"]);
        assert_eq!(Pack::from_name(" Web "), Some(Pack::Web));
        assert!(is_test(Path::new("crates/x/tests/a.rs")) && !is_test(Path::new("src/a.rs")));
        assert_eq!(rel(Path::new("/p"), Path::new("/p/src/a.rs")), "src/a.rs");
    }

    #[test]
    fn walk_skips_ignored_and_dot_folders_and_roots_find_the_workspace() {
        let base = std::env::temp_dir().join(format!("wisply-think-walk-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let put = |p: &str, body: &str| {
            let f = base.join(p);
            fs::create_dir_all(f.parent().unwrap()).unwrap();
            fs::write(f, body).unwrap();
        };
        put("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]\n");
        put(".gitignore", "# c\n/gen/\n*.log\n!keep.log\nnotes\n");
        put(".git/info/exclude", "SPEC.md\n");
        put(".github/workflows/ci.yml", "x");
        put(".graphify/a.js", "x");
        put("gen/a.rs", "x");
        put("SPEC.md", "x");
        put("a.log", "x");
        put("keep.log", "x");
        put("notes/n.md", "x");
        put("crates/a/Cargo.toml", "[package]\n");
        put("crates/a/src/lib.rs", "x");
        put("crates/a/.gitignore", "lib.rs\n");
        put("crates/b/Cargo.toml", "[package]\n");
        put("crates/b/src/main.rs", "x");
        put("crates/b/gen/x.rs", "x");
        let got: Vec<String> = walk(std::slice::from_ref(&base)).iter().map(|p| rel(&base, p)).collect();
        assert_eq!(
            got,
            [
                ".github/workflows/ci.yml".to_string(),
                ".gitignore".into(),
                "Cargo.toml".into(),
                "crates/a/.gitignore".into(),
                "crates/a/Cargo.toml".into(),
                "crates/b/Cargo.toml".into(),
                "crates/b/gen/x.rs".into(),
                "crates/b/src/main.rs".into(),
                "keep.log".into(),
            ]
        );
        let a = base.join("crates/a/src/lib.rs");
        assert_eq!(root_for(std::slice::from_ref(&a)), Some(base.clone()));
        assert_eq!(root_for(&[a, base.join("crates/b/src/main.rs")]), Some(base.clone()));
        assert_eq!(root_of(&base.join("crates/a/src/lib.rs")), base.join("crates/a"));
        let _ = fs::remove_dir_all(&base);
    }
}
