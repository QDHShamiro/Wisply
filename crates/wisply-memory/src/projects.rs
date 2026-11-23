use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use super::store::now;
use super::text::stop_word;
use super::vault::front;

pub const INDEX: &str = "MEMORY.md";
pub const PROJECTS: &str = "PROJECTS.md";
const KIND: &str = "project";
const PREFIX: &str = "project_";
const POINTER: &str = "- [Project notes](PROJECTS.md) — project lines moved here by `wisply memory slim`; Wisply names \
                       the matching ones when a project comes up, open this file for the rest";
const HEAD: &str = "# Project notes\n\nMoved out of MEMORY.md by `wisply memory slim`, so they cost tokens only when \
                    a project comes up. `wisply memory slim --undo` puts them back.\n";
const GENERIC: &[&str] = &[
    "ai", "alt", "api", "app", "apps", "async", "body", "bot", "build", "chat", "code", "critical", "data", "dev",
    "discord", "draft", "edit", "edits", "fix", "game", "hook", "hooks", "java", "ki", "live", "local", "lokale",
    "lokaler", "memory", "mod", "mods", "money", "new", "pc", "plan", "plugin", "plugins", "release", "repo", "server",
    "setup", "skill", "skills", "spiel", "task", "terminal", "test", "tool", "tools", "video", "web",
];
const MIN_KEY: usize = 3;
const MIN_WORD: usize = 4;
const PAIR: usize = 2;
const STRONG: usize = 10;
const PLURAL_FROM: usize = 5;
const BACKUP: &str = "MEMORY";

static LINK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*[-*]\s*\[([^\]]+)\]\(([^)\s]+\.md)\)").expect("link pattern"));
static TOKEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^\W_]+").expect("token pattern"));

pub struct Entry {
    pub line: String,
    pub file: String,
    keys: Vec<Vec<String>>,
    words: HashSet<String>,
}

#[derive(Default)]
pub struct Slim {
    pub moved: Vec<String>,
    pub chars: usize,
    pub backup: Option<PathBuf>,
}

fn tokens(text: &str) -> Vec<String> {
    TOKEN.find_iter(&text.to_lowercase()).map(|m| m.as_str().to_string()).collect()
}

fn generic(word: &str) -> bool {
    GENERIC.contains(&word) || stop_word(word) || word.chars().all(|c| c.is_ascii_digit())
}

fn key(words: &[String]) -> Option<Vec<String>> {
    let first = words.first()?;
    if first.chars().count() < MIN_KEY || first.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if !generic(first) {
        return Some(vec![first.clone()]);
    }
    let second = words.get(1).filter(|w| w.chars().count() >= MIN_KEY)?;
    Some(vec![first.clone(), second.clone()])
}

fn link(line: &str) -> Option<(String, String)> {
    let m = LINK.captures(line)?;
    Some((m[1].to_string(), m[2].to_string()))
}

fn entry(line: &str) -> Option<Entry> {
    let (title, file) = link(line)?;
    if file == PROJECTS {
        return None;
    }
    let stem = Path::new(&file).file_stem().map_or(String::new(), |s| s.to_string_lossy().into_owned());
    let slug = tokens(stem.strip_prefix(PREFIX).unwrap_or(&stem));
    let head = tokens(&title);
    let mut keys: Vec<Vec<String>> = [&slug, &head].into_iter().filter_map(|w| key(w)).collect();
    keys.dedup();
    let words = slug.into_iter().chain(head).filter(|w| w.chars().count() >= MIN_WORD && !generic(w)).collect();
    Some(Entry { line: line.trim_end().to_string(), file, keys, words })
}

fn said(text: &str) -> HashSet<String> {
    let mut out = HashSet::new();
    for t in tokens(text) {
        if t.chars().count() >= PLURAL_FROM
            && let Some(one) = t.strip_suffix('s')
        {
            out.insert(one.to_string());
        }
        out.insert(t);
    }
    out
}

pub fn entries(dir: &Path) -> Vec<Entry> {
    fs::read_to_string(dir.join(PROJECTS)).map(|t| t.lines().filter_map(entry).collect()).unwrap_or_default()
}

pub fn matching<'a>(all: &'a [Entry], text: &str, max: usize) -> Vec<&'a Entry> {
    let heard = said(text);
    let mut scored: Vec<(usize, usize, &Entry)> = all
        .iter()
        .enumerate()
        .filter_map(|(i, e)| {
            let strong = e.keys.iter().filter(|k| k.iter().all(|w| heard.contains(w))).count();
            let shared = e.words.iter().filter(|w| heard.contains(*w)).count();
            let score = strong * STRONG + shared;
            (strong > 0 || shared >= PAIR).then_some((score, i, e))
        })
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    scored.into_iter().take(max).map(|(_, _, e)| e).collect()
}

pub fn memory_dir(claude: &Path, cwd: &str, transcript: &str) -> PathBuf {
    if let Some(parent) = Path::new(transcript).parent().filter(|_| !transcript.is_empty()) {
        return parent.join("memory");
    }
    let slug: String = cwd.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    claude.join("projects").join(slug).join("memory")
}

fn kind_of(dir: &Path, file: &str) -> Option<String> {
    let text = fs::read_to_string(dir.join(file)).ok()?;
    front(&text).0.get("type").map(|t| t.trim().to_lowercase())
}

fn newline(text: &str) -> &'static str {
    if text.contains("\r\n") { "\r\n" } else { "\n" }
}

fn write(path: &Path, lines: &[String], nl: &str) -> io::Result<()> {
    let mut text = lines.join(nl);
    text.push_str(nl);
    wisply_core::save(path, text)
}

fn is_pointer(line: &str) -> bool {
    link(line).is_some_and(|(_, f)| f == PROJECTS)
}

fn with_pointer(mut lines: Vec<String>) -> Vec<String> {
    if lines.iter().any(|l| is_pointer(l)) {
        return lines;
    }
    let at = lines.iter().position(|l| link(l).is_some()).unwrap_or(lines.len());
    lines.insert(at, POINTER.to_string());
    lines
}

fn merged(old: &str, moved: &[String]) -> Vec<String> {
    let mut lines: Vec<String> = if old.trim().is_empty() {
        HEAD.lines().map(String::from).collect()
    } else {
        old.lines().map(String::from).collect()
    };
    let mut at: HashMap<String, usize> = HashMap::new();
    for (i, l) in lines.iter().enumerate() {
        if let Some((_, f)) = link(l) {
            at.insert(f, i);
        }
    }
    for line in moved {
        let Some((_, file)) = link(line) else { continue };
        match at.get(&file) {
            Some(&i) => lines[i] = line.clone(),
            None => {
                at.insert(file, lines.len());
                lines.push(line.clone());
            }
        }
    }
    lines
}

fn backup(dir: &Path, archive: &Path, text: &str) -> io::Result<PathBuf> {
    fs::create_dir_all(archive)?;
    let owner = dir.parent().and_then(Path::file_name).map_or(String::new(), |n| n.to_string_lossy().into_owned());
    let path = archive.join(format!("{BACKUP}-{owner}-{}.md", now() as u64));
    fs::write(&path, text)?;
    Ok(path)
}

pub fn slim(dir: &Path, archive: Option<&Path>, dry: bool) -> io::Result<Slim> {
    let index = fs::read_to_string(dir.join(INDEX))?;
    let nl = newline(&index);
    let (mut kept, mut moved) = (Vec::new(), Vec::new());
    for line in index.lines() {
        let project =
            link(line).filter(|(_, f)| f != PROJECTS).is_some_and(|(_, f)| kind_of(dir, &f).as_deref() == Some(KIND));
        if project { moved.push(line.trim_end().to_string()) } else { kept.push(line.to_string()) }
    }
    let chars = moved.iter().map(|l| l.chars().count() + 1).sum();
    let out = Slim { moved, chars, backup: None };
    let has_projects = dir.join(PROJECTS).is_file();
    let pointed = kept.iter().any(|l| is_pointer(l));
    if dry || (out.moved.is_empty() && (pointed || !has_projects)) {
        return Ok(out);
    }
    let backup = match archive {
        Some(a) if !out.moved.is_empty() => Some(backup(dir, a, &index)?),
        _ => None,
    };
    if !out.moved.is_empty() {
        let old = fs::read_to_string(dir.join(PROJECTS)).unwrap_or_default();
        write(&dir.join(PROJECTS), &merged(&old, &out.moved), nl)?;
    }
    write(&dir.join(INDEX), &with_pointer(kept), nl)?;
    Ok(Slim { backup, ..out })
}

pub fn auto(dir: &Path) -> io::Result<usize> {
    if !dir.join(PROJECTS).is_file() || !dir.join(INDEX).is_file() {
        return Ok(0);
    }
    Ok(slim(dir, None, false)?.moved.len())
}

pub fn undo(dir: &Path) -> io::Result<usize> {
    let projects = fs::read_to_string(dir.join(PROJECTS))?;
    let index = fs::read_to_string(dir.join(INDEX)).unwrap_or_default();
    let nl = newline(&index);
    let mut lines: Vec<String> = index.lines().filter(|l| !is_pointer(l)).map(String::from).collect();
    let have: HashSet<String> = lines.iter().filter_map(|l| link(l).map(|(_, f)| f)).collect();
    let back: Vec<String> = projects
        .lines()
        .filter(|l| link(l).is_some_and(|(_, f)| f != PROJECTS && !have.contains(&f)))
        .map(|l| l.trim_end().to_string())
        .collect();
    while lines.last().is_some_and(|l| l.trim().is_empty()) {
        lines.pop();
    }
    lines.extend(back.iter().cloned());
    write(&dir.join(INDEX), &lines, nl)?;
    fs::remove_file(dir.join(PROJECTS))?;
    Ok(back.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(dir: &Path, file: &str, kind: &str) {
        fs::write(dir.join(file), format!("---\nname: n\nmetadata:\n  type: {kind}\n---\nbody")).unwrap();
    }

    fn line(title: &str, file: &str, hook: &str) -> String {
        format!("- [{title}]({file}) — {hook}")
    }

    #[test]
    fn slim_moves_project_lines_and_undo_puts_them_back() {
        let base = std::env::temp_dir().join(format!("wisply-projects-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let dir = base.join("C--x").join("memory");
        fs::create_dir_all(&dir).unwrap();
        note(&dir, "feedback_short.md", "feedback");
        note(&dir, "project_sniper.md", "project");
        note(&dir, "project_body_detector.md", "project");
        let rule = line("Short answers", "feedback_short.md", "keep it short");
        let sniper = line("Sniper Live-Stand", "project_sniper.md", "E:\\Sniper on main");
        let body = line("Body Detector Hand-Maus", "project_body_detector.md", "MediaPipe");
        fs::write(dir.join(INDEX), format!("# Memory Index\r\n\r\n{rule}\r\n{sniper}\r\n{body}\r\n")).unwrap();
        let dry = slim(&dir, Some(&base.join("archive")), true).unwrap();
        assert_eq!(dry.moved.len(), 2);
        assert!(!dir.join(PROJECTS).exists());
        assert_eq!(auto(&dir).unwrap(), 0);
        let done = slim(&dir, Some(&base.join("archive")), false).unwrap();
        assert_eq!(done.moved, vec![sniper.clone(), body.clone()]);
        assert!(done.backup.as_ref().is_some_and(|b| b.is_file()));
        let index = fs::read_to_string(dir.join(INDEX)).unwrap();
        assert_eq!(index, format!("# Memory Index\r\n\r\n{POINTER}\r\n{rule}\r\n"));
        let all = entries(&dir);
        assert_eq!(all.len(), 2);
        let found = |text: &str| matching(&all, text, 3).iter().map(|e| e.file.clone()).collect::<Vec<_>>();
        assert_eq!(found("schau mal in E:/Sniper nach"), vec!["project_sniper.md"]);
        assert_eq!(found("snipers sind kaputt"), vec!["project_sniper.md"]);
        assert!(found("der body vom request").is_empty());
        assert_eq!(found("body detector startet nicht"), vec!["project_body_detector.md"]);
        assert_eq!(found("die hand maus zittert"), vec!["project_body_detector.md"]);
        let newer = line("Sniper Live-Stand", "project_sniper.md", "now on dev");
        fs::write(dir.join(INDEX), format!("{index}{newer}\r\n")).unwrap();
        assert_eq!(auto(&dir).unwrap(), 1);
        let projects = fs::read_to_string(dir.join(PROJECTS)).unwrap();
        assert!(projects.contains("now on dev") && !projects.contains("E:\\Sniper on main"), "{projects}");
        assert_eq!(fs::read_to_string(dir.join(INDEX)).unwrap(), index);
        assert_eq!(undo(&dir).unwrap(), 2);
        assert!(!dir.join(PROJECTS).exists());
        let back = fs::read_to_string(dir.join(INDEX)).unwrap();
        assert_eq!(back, format!("# Memory Index\r\n\r\n{rule}\r\n{newer}\r\n{body}\r\n"));
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn keys_skip_generic_first_words() {
        let e = entry(&line("Pilot Minecraft-KI", "project_pilot.md", "x")).unwrap();
        assert_eq!(e.keys, vec![vec!["pilot".to_string()]]);
        let e = entry(&line("Wisply Code Mode", "project_wisply_codemode.md", "x")).unwrap();
        assert_eq!(e.keys, vec![vec!["wisply".to_string(), "codemode".into()], vec!["wisply".into(), "code".into()]]);
        assert!(entry(&line("Notes", "PROJECTS.md", "x")).is_none());
        assert_eq!(
            memory_dir(Path::new("/h/.claude"), "C:\\Users\\Me", ""),
            Path::new("/h/.claude").join("projects").join("C--Users-Me").join("memory")
        );
        assert_eq!(memory_dir(Path::new("/h"), "x", "/h/projects/p/s.jsonl"), Path::new("/h/projects/p/memory"));
    }
}
