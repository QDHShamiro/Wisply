mod code;
mod docs;
mod image;
mod plugin;
mod web;

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::time::{Duration, Instant};

use regex::Regex;

use crate::files::{self, Pack};
use crate::personal::Rule;

pub use code::{Function, functions, smells_on};
pub use docs::drop_repeats;
pub use image::{png_smaller, recompress};
pub use plugin::{entries, unquote};

const IMAGE_BUDGET: Duration = Duration::from_secs(20);
pub const SCAN_BUDGET: Duration = Duration::from_secs(60);

static NUMBER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b\d+(?:\.\d+)?x?\b").expect("number pattern"));

fn normal(why: &str) -> String {
    NUMBER.replace_all(why, "#").replace(" (# times)", "").replace(" (+# more)", "")
}

fn amount(why: &str) -> f64 {
    NUMBER.find(why).and_then(|m| m.as_str().trim_end_matches('x').parse().ok()).unwrap_or(0.0)
}

pub fn key_of(rule: &str, file: &str, why: &str) -> String {
    format!("{rule}|{file}|{}", normal(why))
}

pub struct Report {
    pub found: Vec<Finding>,
    pub checked: BTreeSet<String>,
    pub left: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    pub rule: String,
    pub pack: Pack,
    pub file: String,
    pub line: usize,
    pub why: String,
    pub fix: String,
    pub impact: u8,
    pub sure: u8,
    pub effort: u8,
    pub fixer: Option<&'static str>,
}

impl Finding {
    pub fn new(rule: &str, pack: Pack, line: usize, why: impl Into<String>, fix: impl Into<String>) -> Finding {
        Finding {
            rule: rule.to_string(),
            pack,
            file: String::new(),
            line,
            why: why.into(),
            fix: fix.into(),
            impact: 2,
            sure: 2,
            effort: 2,
            fixer: None,
        }
    }

    pub fn rate(mut self, impact: u8, sure: u8, effort: u8) -> Finding {
        self.impact = impact;
        self.sure = sure;
        self.effort = effort;
        self
    }

    pub fn fixed_by(mut self, fixer: &'static str) -> Finding {
        self.fixer = Some(fixer);
        self
    }

    pub fn place(&self) -> String {
        if self.line > 0 { format!("{}:{}", self.file, self.line) } else { self.file.clone() }
    }

    pub fn key(&self) -> String {
        key_of(&self.rule, &self.file, &self.why)
    }

    pub fn exact(&self) -> String {
        format!("{}|{}|{}", self.rule, self.file, self.why)
    }

    pub fn known(&self, told: &BTreeSet<String>) -> bool {
        let head = format!("{}|{}|", self.rule, self.file);
        let (why, n) = (normal(&self.why), amount(&self.why));
        told.iter().filter_map(|t| t.strip_prefix(&head)).any(|w| normal(w) == why && amount(w) >= n)
    }

    pub fn points(&self) -> f64 {
        f64::from(self.impact) * f64::from(self.sure) / f64::from(self.effort.max(1))
    }
}

pub struct Scope<'a> {
    pub root: &'a Path,
    pub packs: &'a BTreeSet<Pack>,
    pub rules: &'a [Rule],
}

fn text_of(path: &Path) -> Option<String> {
    files::has_ext(path, files::TEXT_EXT).then(|| files::read(path)).flatten()
}

pub fn file(scope: &Scope, path: &Path, deep: bool) -> Vec<Finding> {
    checks(scope, path, &files::rel(scope.root, path), text_of(path).as_deref(), deep)
}

pub fn text_file(scope: &Scope, path: &Path, text: &str) -> Vec<Finding> {
    let text = files::has_ext(path, files::TEXT_EXT).then_some(text);
    checks(scope, path, &files::rel(scope.root, path), text, false)
}

fn checks(scope: &Scope, path: &Path, rel: &str, text: Option<&str>, deep: bool) -> Vec<Finding> {
    let mut out = Vec::new();
    let e = files::ext(path);
    let on = |p: Pack| scope.packs.contains(&p);
    if files::has_ext(path, files::IMAGE_EXT) && on(Pack::Images) {
        out.extend(image::check(path, &e, deep));
    }
    if let Some(text) = text.filter(|_| !files::is_minified(path)) {
        let test = files::is_test(path);
        if files::has_ext(path, files::CODE_EXT) && on(Pack::Code) {
            out.extend(code::check(&e, text, test, scope.packs));
        }
        if on(Pack::Web) {
            out.extend(web::check(&e, text));
        }
        if e == "md" && on(Pack::Docs) {
            out.extend(docs::check(path, scope.root, text));
        }
        if on(Pack::JavaPlugin) || on(Pack::McGui) {
            out.extend(plugin::check(path, text, scope.packs));
        }
        let (mine, waived) = crate::yours::check(scope.rules, path, &e, text, test, scope.packs);
        out.retain(|f| !waived.contains(f.rule.as_str()));
        out.extend(mine);
        if !matches!(e.as_str(), "md" | "rs" | "svg") && text.lines().any(|l| l.ends_with(' ') || l.ends_with('\t')) {
            let n = text.lines().filter(|l| l.ends_with(' ') || l.ends_with('\t')).count();
            out.push(
                Finding::new("trailing-spaces", Pack::Code, 0, format!("{n} lines end with spaces"), "strip them")
                    .rate(1, 3, 1)
                    .fixed_by("spaces"),
            );
        }
    }
    for f in &mut out {
        f.file = rel.to_string();
    }
    out
}

pub fn scan(scope: &Scope, list: &[PathBuf]) -> Report {
    scan_for(scope, list, SCAN_BUDGET)
}

fn scan_for(scope: &Scope, list: &[PathBuf], budget: Duration) -> Report {
    let started = Instant::now();
    let dups = scope.packs.contains(&Pack::Code);
    let (mut found, mut checked, mut code, mut left) = (Vec::new(), BTreeSet::new(), Vec::new(), 0);
    for (i, path) in list.iter().enumerate() {
        if started.elapsed() >= budget {
            left = list.len() - i;
            break;
        }
        let deep = started.elapsed() < IMAGE_BUDGET;
        let rel = files::rel(scope.root, path);
        let text = text_of(path);
        found.extend(checks(scope, path, &rel, text.as_deref(), deep));
        if deep || files::ext(path) != "png" {
            checked.insert(rel.clone());
        }
        if let Some(t) = text
            && dups
            && files::has_ext(path, files::CODE_EXT)
            && !files::is_test(path)
        {
            code.push((rel, t));
        }
    }
    found.extend(code::duplicates(&code));
    found.sort_by(|a, b| b.points().total_cmp(&a.points()).then(a.file.cmp(&b.file)).then(a.line.cmp(&b.line)));
    Report { found, checked, left }
}

pub fn line_of(text: &str, at: usize) -> usize {
    text[..at.min(text.len())].bytes().filter(|b| *b == b'\n').count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(why: &str) -> Finding {
        let mut f = Finding::new("yours", Pack::Code, 3, why, "x");
        f.file = "src/A.java".into();
        f
    }

    #[test]
    fn keys_ignore_counts_but_know_when_it_got_worse() {
        assert_eq!(at("`HashMap` used 2x").key(), at("`HashMap` used 7x").key());
        assert_eq!(at("unwrap() (3 times)").key(), at("unwrap()").key());
        assert_eq!(at("link to a.md goes nowhere (+2 more)").key(), at("link to a.md goes nowhere").key());
        assert_ne!(at("`foo1` is 80 lines").key(), at("`foo2` is 80 lines").key());
        let told: BTreeSet<String> = [at("`HashMap` used 2x").exact()].into();
        assert!(at("`HashMap` used 2x").known(&told));
        assert!(at("`HashMap` used 1x").known(&told));
        assert!(!at("`HashMap` used 3x").known(&told));
        assert!(!at("`tick()` used 1x").known(&told));
    }

    #[test]
    fn each_problem_is_reported_once() {
        let claude = "# Minecraft Plugin\n## Code Rules\n3. `FoliaScheduler` always — never `Bukkit.getScheduler()`.\n\
            5. `ConcurrentHashMap` for shared state — never `HashMap`.\n6. No `System.out` — use `DebugLogger`.\n";
        let rules = crate::personal::parse(claude, "c.md", &BTreeSet::new());
        let packs: BTreeSet<Pack> = [Pack::Code, Pack::JavaPlugin].into();
        let root = Path::new("p");
        let scope = Scope { root, packs: &packs, rules: &rules };
        let found = |name: &str, body: &str| -> Vec<String> {
            text_file(&scope, &root.join(name), body).into_iter().map(|f| f.rule).collect()
        };
        let a = "import java.util.HashMap;\nclass A {\n  // a HashMap is fine here\n\
            \x20 private final Map<A, B> m = new HashMap<>();\n  void p() { System.out.println(1); }\n}\n";
        assert_eq!(found("A.java", a), ["debug-output", "shared-hashmap"]);
        let b = "import java.util.HashMap;\nclass B {\n  Object copy(Map<A, B> n) { return new HashMap<>(n); }\n}\n";
        assert_eq!(found("B.java", b), ["yours"]);
        let run = "class S {\n  void run() { Bukkit.getScheduler().runTask(p, r); }\n}\n";
        assert!(found("FoliaScheduler.java", run).is_empty());
        assert_eq!(found("Task.java", run), ["bukkit-scheduler"]);
    }

    #[test]
    fn minified_files_are_left_alone() {
        let packs: BTreeSet<Pack> = [Pack::Code, Pack::Web].into();
        let scope = Scope { root: Path::new("p"), packs: &packs, rules: &[] };
        let css = ".a{color:#777;background:#888}";
        assert_eq!(text_file(&scope, Path::new("p/site.css"), css).len(), 1);
        assert!(text_file(&scope, Path::new("p/site.min.css"), css).is_empty());
    }

    #[test]
    fn scan_stops_at_the_budget_and_says_what_it_checked() {
        let dir = std::env::temp_dir().join(format!("wisply-think-budget-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let list: Vec<PathBuf> = (0..3).map(|i| dir.join(format!("{i}.md"))).collect();
        for p in &list {
            std::fs::write(p, "This is is wrong.\n").unwrap();
        }
        let packs: BTreeSet<Pack> = [Pack::Docs].into();
        let scope = Scope { root: &dir, packs: &packs, rules: &[] };
        let full = scan_for(&scope, &list, SCAN_BUDGET);
        assert_eq!((full.found.len(), full.checked.len(), full.left), (3, 3, 0));
        let none = scan_for(&scope, &list, Duration::ZERO);
        assert_eq!((none.found.len(), none.checked.len(), none.left), (0, 0, 3));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
