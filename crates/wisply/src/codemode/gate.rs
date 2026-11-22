use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde_json::Value;

use super::files;
use crate::risk::{self, Risk};

static RULE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"prefix_rule\(\s*pattern\s*=\s*\[((?:[^\]"]|"(?:[^"\\]|\\.)*")*)\]\s*,\s*decision\s*=\s*"(\w+)""#)
        .expect("rule")
});
static WORD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#""((?:[^"\\]|\\.)*)""#).expect("word"));
static SPLIT: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"&&|\|\||[;|&\n\r]").expect("split"));
static SHELL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$`((?:[^`\\]|\\.)*)`").expect("shell"));
static PLAIN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\$\(\s*(?:"([^"\\]*)"|'([^'\\]*)')\s*\)"#).expect("plain"));
static FILE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"\b(write|edit|read)\(\s*(?:"([^"\\]+)"|'([^'\\]+)')"#).expect("file"));
static HOLE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$\{[^}]*\}").expect("hole"));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Read,
    Write,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pat {
    All,
    Exact(String),
    Prefix(Vec<String>),
    Wild(String),
}

#[derive(Debug, Clone)]
struct Rule {
    pat: Pat,
    shell: Option<bool>,
    shown: String,
    from: String,
}

#[derive(Debug, Clone)]
struct PathRule {
    op: Op,
    glob: String,
    shown: String,
    from: String,
}

#[derive(Debug, Default, Clone)]
pub struct Rules {
    deny: Vec<Rule>,
    ask: Vec<Rule>,
    deny_paths: Vec<PathRule>,
    ask_paths: Vec<PathRule>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Gate {
    Pass,
    Ask { what: String, risk: Option<Risk>, mark: String },
    Block { what: String, rule: String, from: String },
}

fn shown_from(path: &Path) -> String {
    let full = files::shown(path);
    match files::user_home().map(|h| files::shown(&h)) {
        Some(home) if full.starts_with(&home) => format!("~{}", &full[home.len()..]),
        _ => full,
    }
}

fn json(path: &Path) -> Option<Value> {
    serde_json::from_str(&fs::read_to_string(path).ok()?).ok()
}

fn strings(v: &Value) -> Vec<String> {
    v.as_array().map(|a| a.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
}

fn shell_pat(inner: &str, prefix_words: bool) -> Pat {
    let inner = inner.trim();
    if let Some(head) = inner.strip_suffix(":*") {
        return Pat::Prefix(vec![head.trim().to_lowercase()]);
    }
    if inner.contains('*') {
        return Pat::Wild(inner.to_lowercase());
    }
    if prefix_words { Pat::Prefix(vec![inner.to_lowercase()]) } else { Pat::Exact(inner.to_lowercase()) }
}

fn split_rule(rule: &str) -> (&str, Option<&str>) {
    match rule.find('(') {
        Some(open) if rule.ends_with(')') => (rule[..open].trim(), Some(&rule[open + 1..rule.len() - 1])),
        _ => (rule.trim(), None),
    }
}

impl Rules {
    fn add(&mut self, rule: &str, ask: bool, base: &Path, from: &Path, prefix_words: bool) {
        let (tool, inner) = split_rule(rule);
        let from = shown_from(from);
        let shown = rule.to_string();
        let shell = match tool {
            "Bash" | "Shell" | "run_shell_command" => Some(Some(true)),
            "PowerShell" => Some(Some(false)),
            _ => None,
        };
        if let Some(shell) = shell {
            let pat = inner.map_or(Pat::All, |i| shell_pat(i, prefix_words || tool != "Bash" && tool != "PowerShell"));
            let shell =
                if matches!(pat, Pat::All) && tool != "Shell" && tool != "run_shell_command" { shell } else { None };
            let list = if ask { &mut self.ask } else { &mut self.deny };
            list.push(Rule { pat, shell, shown, from });
            return;
        }
        let op = match tool {
            "Read" => Op::Read,
            "Edit" | "Write" | "MultiEdit" | "NotebookEdit" => Op::Write,
            _ => return,
        };
        let Some(inner) = inner.filter(|i| !i.trim().is_empty()) else { return };
        let glob = path_glob(inner.trim(), base);
        let list = if ask { &mut self.ask_paths } else { &mut self.deny_paths };
        list.push(PathRule { op, glob, shown, from });
    }

    fn claude(&mut self, file: &Path, base: &Path) {
        let Some(v) = json(file) else { return };
        for (key, ask) in [("deny", false), ("ask", true)] {
            for rule in strings(&v["permissions"][key]) {
                self.add(&rule, ask, base, file, false);
            }
        }
    }

    fn gemini(&mut self, file: &Path, base: &Path) {
        let Some(v) = json(file) else { return };
        for rule in strings(&v["tools"]["exclude"]).into_iter().chain(strings(&v["excludeTools"])) {
            self.add(&rule, false, base, file, true);
        }
    }

    fn cursor(&mut self, file: &Path, base: &Path) {
        let Some(v) = json(file) else { return };
        for rule in strings(&v["permissions"]["deny"]) {
            self.add(&rule, false, base, file, true);
        }
    }

    fn codex(&mut self, dir: &Path) {
        let Ok(read) = fs::read_dir(dir) else { return };
        for entry in read.filter_map(Result::ok).filter(|e| e.path().extension().is_some_and(|x| x == "rules")) {
            let path = entry.path();
            let Ok(text) = fs::read_to_string(&path) else { continue };
            for (words, decision) in codex_rules(&text) {
                let ask = match decision.as_str() {
                    "forbidden" => false,
                    "prompt" => true,
                    _ => continue,
                };
                let shown = format!("prefix_rule({})", words.join(" "));
                let rule = Rule { pat: Pat::Prefix(words), shell: None, shown, from: shown_from(&path) };
                if ask { self.ask.push(rule) } else { self.deny.push(rule) }
            }
        }
    }
}

fn codex_rules(text: &str) -> Vec<(Vec<String>, String)> {
    RULE.captures_iter(text)
        .filter(|c| !c[1].contains('['))
        .map(|c| {
            let words = WORD
                .captures_iter(&c[1])
                .map(|w| serde_json::from_str::<String>(&format!("\"{}\"", &w[1])).unwrap_or_else(|_| w[1].to_string()))
                .map(|w| w.to_lowercase())
                .collect();
            (words, c[2].to_string())
        })
        .collect()
}

fn path_glob(pattern: &str, base: &Path) -> String {
    let pattern = pattern.replace('\\', "/");
    let full = if let Some(abs) = pattern.strip_prefix("//") {
        if cfg!(windows) && abs.chars().nth(1) != Some(':') {
            format!("{}:/{}", abs.chars().next().unwrap_or('c'), abs.get(2..).unwrap_or_default())
        } else {
            format!("/{abs}")
        }
    } else if let Some(rest) = pattern.strip_prefix("~/") {
        files::user_home().map(|h| format!("{}/{rest}", files::shown(&h))).unwrap_or(pattern.clone())
    } else if let Some(rest) = pattern.strip_prefix('/') {
        format!("{}/{rest}", files::shown(base))
    } else if !pattern.trim_start_matches("./").contains('/') {
        format!("**/{}", pattern.trim_start_matches("./"))
    } else {
        format!("{}/{}", files::shown(base), pattern.trim_start_matches("./"))
    };
    full.to_lowercase()
}

fn path_fits(glob: &str, path: &str) -> bool {
    let pat: Vec<&str> = glob.split('/').filter(|s| !s.is_empty()).collect();
    let parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
    fn go(pat: &[&str], parts: &[&str]) -> bool {
        match (pat.first(), parts.first()) {
            (None, None) => true,
            (Some(&"**"), _) => go(&pat[1..], parts) || (!parts.is_empty() && go(pat, &parts[1..])),
            (Some(p), Some(n)) => files::fits(p, n) && go(&pat[1..], &parts[1..]),
            _ => false,
        }
    }
    go(&pat, &parts) || (pat.last() != Some(&"**") && go(&[pat.as_slice(), &["**"]].concat(), &parts))
}

pub fn load(cwd: &Path) -> Rules {
    let mut rules = Rules::default();
    let home = files::user_home();
    if let Some(home) = &home {
        rules.claude(&home.join(".claude/settings.json"), home);
        rules.gemini(&home.join(".gemini/settings.json"), home);
        rules.cursor(&home.join(".cursor/cli-config.json"), home);
        rules.codex(&home.join(".codex/rules"));
    }
    let mut dir: Option<&Path> = Some(cwd);
    while let Some(here) = dir {
        if home.as_deref() != Some(here) {
            rules.claude(&here.join(".claude/settings.json"), here);
            rules.claude(&here.join(".claude/settings.local.json"), here);
            rules.gemini(&here.join(".gemini/settings.json"), here);
            rules.cursor(&here.join(".cursor/cli.json"), here);
        }
        if here.join(".git").exists() {
            break;
        }
        dir = here.parent();
    }
    rules
}

fn segments(line: &str) -> Vec<String> {
    SPLIT
        .split(line)
        .map(|s| s.trim().trim_start_matches(['(', '{']).trim_end_matches([')', '}']).trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

fn bare(word: &str) -> String {
    let name = word.rsplit(['/', '\\']).next().unwrap_or(word);
    name.strip_suffix(".exe").unwrap_or(name).to_string()
}

fn hits(pat: &Pat, seg: &str) -> bool {
    let words = wisply_proto::words(seg).unwrap_or_else(|_| seg.split_whitespace().map(str::to_string).collect());
    match pat {
        Pat::All => true,
        Pat::Exact(want) => seg == want,
        Pat::Wild(glob) => files::fits(glob, seg),
        Pat::Prefix(want) if want.len() == 1 => {
            let head = &want[0];
            seg == head || seg.starts_with(&format!("{head} ")) || words.first().is_some_and(|w| bare(w) == *head)
        }
        Pat::Prefix(want) => {
            words.len() >= want.len()
                && words.iter().zip(want).enumerate().all(|(i, (w, p))| w == p || (i == 0 && bare(w) == bare(p)))
        }
    }
}

fn shell_rule<'a>(list: &'a [Rule], line: &str, bash: bool) -> Option<&'a Rule> {
    let segs = segments(line);
    list.iter().filter(|r| r.shell.is_none_or(|b| b == bash)).find(|r| segs.iter().any(|s| hits(&r.pat, s)))
}

fn short(text: &str) -> String {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() > 120 { format!("{}...", flat.chars().take(117).collect::<String>()) } else { flat }
}

const DELETERS: &[&str] = &["rm", "rmdir", "unlink", "del", "erase", "rd", "remove-item", "ri"];
const MOVES: &[&str] = &["cd", "pushd", "popd", "chdir", "set-location", "sl"];
const UNSAFE: &[char] = &['$', '`', '~', '%', '\\', ':', '\'', '"', '(', ')', '{', '}', '<', '>', '!', '|', '&', ';'];
const GLOB: &[char] = &['*', '?', '['];

fn local_target(word: &str, dir: &Path, root: &Path) -> bool {
    if word.contains(UNSAFE) || word.starts_with('/') {
        return false;
    }
    let parts: Vec<&str> = word.split('/').filter(|p| !p.is_empty() && *p != ".").collect();
    let Some(first) = parts.first() else { return false };
    if parts.contains(&"..") || first.contains(GLOB) {
        return false;
    }
    let mut at = dir.to_path_buf();
    for part in parts.iter().take_while(|p| !p.contains(GLOB)) {
        at.push(part);
        if fs::symlink_metadata(&at).is_ok_and(|m| m.file_type().is_symlink()) {
            return false;
        }
    }
    files::inside(&at, &[root.to_path_buf()]) && at != root
}

fn local_delete(line: &str, dir: &Path, root: &Path) -> bool {
    if !files::inside(dir, &[root.to_path_buf()]) {
        return false;
    }
    let mut any = false;
    for seg in SPLIT.split(line).map(str::trim).filter(|s| !s.is_empty()) {
        let Ok(words) = wisply_proto::words(seg) else { return false };
        let Some(verb) = words.first().map(|w| bare(w).to_lowercase()) else { continue };
        if MOVES.contains(&verb.as_str()) {
            return false;
        }
        if !DELETERS.contains(&verb.as_str()) {
            if risk::of(seg).is_some() {
                return false;
            }
            continue;
        }
        let dos = matches!(verb.as_str(), "del" | "erase" | "rd");
        let flag = |w: &&String| w.starts_with('-') || (dos && w.starts_with('/') && w.len() == 2);
        let targets: Vec<&String> = words[1..].iter().filter(|w| !flag(w)).collect();
        if targets.is_empty() || !targets.iter().all(|t| local_target(t, dir, root)) {
            return false;
        }
        any = true;
    }
    any
}

pub fn shell(rules: &Rules, yes: bool, line: &str, bash: bool, dir: &Path, root: &Path) -> Gate {
    let place = if dir == root { String::new() } else { format!(" in {}", shown_from(dir)) };
    let what = format!("the command '{}'{place}", short(line));
    if let Some(rule) = shell_rule(&rules.deny, line, bash) {
        return Gate::Block { what, rule: rule.shown.clone(), from: rule.from.clone() };
    }
    if yes {
        return Gate::Pass;
    }
    if let Some(rule) = shell_rule(&rules.ask, line, bash) {
        return Gate::Ask { what, risk: None, mark: format!("{} in {}", rule.shown, rule.from) };
    }
    match risk::of(line) {
        Some(found) if found.risk == Risk::Destroy && local_delete(line, dir, root) => Gate::Pass,
        Some(found) => Gate::Ask { what, risk: Some(found.risk), mark: found.mark },
        None => Gate::Pass,
    }
}

fn outside(path: &Path, cwd: &Path) -> bool {
    !files::inside(path, &[cwd.to_path_buf(), std::env::temp_dir()])
}

pub fn file(rules: &Rules, yes: bool, op: Op, path: &Path, cwd: &Path) -> Gate {
    let full = files::shown(path).to_lowercase();
    let verb = if op == Op::Read { "reading" } else { "writing" };
    let what = format!("{verb} {}", shown_from(path));
    if let Some(rule) = rules.deny_paths.iter().find(|r| r.op == op && path_fits(&r.glob, &full)) {
        return Gate::Block { what, rule: rule.shown.clone(), from: rule.from.clone() };
    }
    if yes {
        return Gate::Pass;
    }
    if let Some(rule) = rules.ask_paths.iter().find(|r| r.op == op && path_fits(&r.glob, &full)) {
        return Gate::Ask { what, risk: None, mark: format!("{} in {}", rule.shown, rule.from) };
    }
    let found = risk::of(&format!("open {}", files::shown(path)));
    let found = found.filter(|f| f.risk == Risk::Secrets || (op == Op::Write && f.risk == Risk::Rights));
    if let Some(found) = found {
        return Gate::Ask { what, risk: Some(found.risk), mark: found.mark };
    }
    if op == Op::Write && path.is_file() && outside(path, cwd) {
        return Gate::Ask { what, risk: Some(Risk::Destroy), mark: "an existing file outside the project".into() };
    }
    Gate::Pass
}

pub fn scan(rules: &Rules, yes: bool, source: &str, cwd: &Path, bash: bool) -> Gate {
    let bodies = SHELL.captures_iter(source).map(|c| HOLE.replace_all(&c[1], "x").into_owned()).chain(
        PLAIN.captures_iter(source).filter_map(|c| c.get(1).or_else(|| c.get(2)).map(|m| m.as_str().to_string())),
    );
    for body in bodies {
        let gate = shell(rules, yes, &body, bash, cwd, cwd);
        if gate != Gate::Pass {
            return gate;
        }
    }
    for c in FILE.captures_iter(source) {
        let op = if &c[1] == "read" { Op::Read } else { Op::Write };
        let Some(path) = c.get(2).or_else(|| c.get(3)) else { continue };
        let gate = file(rules, yes, op, &files::resolve(cwd, path.as_str()), cwd);
        if gate != Gate::Pass {
            return gate;
        }
    }
    Gate::Pass
}

#[cfg(test)]
#[path = "gate_tests.rs"]
mod tests;
