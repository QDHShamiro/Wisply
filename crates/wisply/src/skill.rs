use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::atomic::{AtomicU32, Ordering};

use serde_json::json;
use wisply_core::Fail;
use wisply_think::skill::head;

use crate::risk;
use crate::setup::place::copy_tree;

mod look;
mod web;

use look::{Checked, inspect, is_script, walk};

const USAGE: &str = "wisply skill | find <words> [--web] | new <name> \"what it does, use when ...\" --file text.md | \
                     add <folder> | get <owner/repo/path> | check <folder or name> | rm <name>";
const GUIDE: &str = "a good skill, part by part:
1. name: a-z, 0-9 and dashes, and it says the task (pdf-forms, not helper)
2. description: one or two sentences in the third person: what it does, then \"Use when ...\" with the words a user \
                     would say. Every AI picks the skill by this line alone
3. text: the steps in the order they are done, as orders, with the real commands and file names, what done looks \
                     like and how to check it, under 500 lines
4. long detail goes to references/<topic>.md and the text names each file; work that must be exact goes to scripts/
build it: wisply skill new <name> \"<description>\" --file text.md   (a whole folder: wisply skill add <folder>)
try it first: wisply skill check <folder>";
const SKILL: &str = "SKILL.md";
const MARK: &str = ".wisply.json";
const STORE: &str = "skills";
const SCRIPT_FOLDER: &str = "scripts/";
const SCRIPT_KINDS: &[&str] = &[
    "py", "sh", "bash", "zsh", "fish", "ps1", "bat", "cmd", "js", "mjs", "cjs", "ts", "rb", "pl", "php", "lua", "vbs",
    "wsf", "exe", "dll", "so", "dylib", "jar", "msi", "com", "scr", "bin", "command",
];
const WHEN_WORDS: &[&str] = &["when", "use ", "for ", "wenn", "bei "];
const NAME_MAX: usize = 64;
const FILES_MAX: usize = 200;
const BYTES_MAX: u64 = 5 * 1024 * 1024;
const TEXT_MAX: u64 = 1024 * 1024;
const ABOUT_SHORT: usize = 40;
const ABOUT_SHOWN: usize = 120;
const FOUND_MAX: usize = 8;
const NAMED_MAX: usize = 3;

static NEXT: AtomicU32 = AtomicU32::new(0);

pub fn run(home: &Path, args: &[String]) -> ExitCode {
    match command(home, &wisply_proto::user_dir(), args) {
        Ok(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Err(fail) => {
            println!("{}", fail.text());
            ExitCode::from(1)
        }
    }
}

fn command(home: &Path, user: &Path, args: &[String]) -> Result<String, Fail> {
    let rest = args.get(1..).unwrap_or_default();
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(list(home, user)),
        Some("find") => find(home, user, rest),
        Some("new") => new(home, user, rest),
        Some("add") => add(home, user, rest),
        Some("get") => get(home, user, rest),
        Some("check") => check(home, rest),
        Some("rm") => remove(home, user, rest),
        Some(other) => Err(Fail::hint(format!("skill does not know '{other}'"), USAGE)),
    }
}

fn store(home: &Path) -> PathBuf {
    home.join(STORE)
}

fn slash(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn places(user: &Path) -> Vec<(&'static str, PathBuf)> {
    let gemini = user.join(".gemini");
    let all = [
        ("claude", user.join(".claude").join("skills")),
        ("codex", user.join(".codex").join("skills")),
        ("cursor", user.join(".cursor").join("skills")),
        ("gemini", gemini.join("skills")),
        ("antigravity", gemini.join("antigravity").join("skills")),
        ("antigravity", gemini.join("antigravity").join("global_skills")),
        ("antigravity-cli", gemini.join("antigravity-cli").join("skills")),
        ("lmstudio", user.join(".lmstudio").join("skills")),
        ("agents", user.join(".agents").join("skills")),
    ];
    all.into_iter().filter(|(_, dir)| dir.parent().is_some_and(Path::is_dir)).collect()
}

struct Seen {
    name: String,
    about: String,
    dir: PathBuf,
    at: Vec<&'static str>,
}

fn folders(dir: &Path) -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| !e.file_name().to_string_lossy().starts_with('.'))
        .map(|e| e.path())
        .filter(|p| p.join(SKILL).is_file())
        .collect();
    out.sort();
    out
}

fn leaf(dir: &Path) -> String {
    dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()
}

fn seen(home: &Path, user: &Path) -> Vec<Seen> {
    let mut out: Vec<Seen> = Vec::new();
    for (label, dir) in std::iter::once(("wisply", store(home))).chain(places(user)) {
        for folder in folders(&dir) {
            let Some(h) = fs::read_to_string(folder.join(SKILL)).ok().as_deref().and_then(head) else { continue };
            let name = if h.name.is_empty() { leaf(&folder) } else { h.name };
            match out.iter_mut().find(|s| s.name == name) {
                Some(s) if s.at.contains(&label) => {}
                Some(s) => s.at.push(label),
                None => out.push(Seen { name, about: h.description, dir: folder, at: vec![label] }),
            }
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

fn short(text: &str) -> String {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= ABOUT_SHOWN { flat } else { flat.chars().take(ABOUT_SHOWN).collect::<String>() + "..." }
}

fn row(s: &Seen) -> String {
    format!("  {} - {} | in {}", s.name, short(&s.about), s.at.join(", "))
}

fn list(home: &Path, user: &Path) -> String {
    let all = seen(home, user);
    if all.is_empty() {
        return "no skills on this PC yet. Look on the internet: wisply skill find <words> --web, or build one: wisply \
                skill new"
            .into();
    }
    let mut lines = vec![format!("{} skill(s) on this PC, look for one with: wisply skill find <words>", all.len())];
    lines.extend(all.iter().map(row));
    lines.join("\n")
}

fn score(words: &[String], name: &str, about: &str) -> usize {
    let (name, about) = (name.to_lowercase(), about.to_lowercase());
    words.iter().map(|w| 3 * usize::from(name.contains(w.as_str())) + usize::from(about.contains(w.as_str()))).sum()
}

fn find(home: &Path, user: &Path, args: &[String]) -> Result<String, Fail> {
    let online = args.iter().any(|a| a == "--web");
    let words: Vec<String> = args
        .iter()
        .filter(|a| !a.starts_with("--"))
        .flat_map(|a| a.split_whitespace())
        .map(str::to_lowercase)
        .collect();
    if words.is_empty() {
        return Err(Fail::hint("find needs the words to look for", "wisply skill find \"pdf forms\" [--web]"));
    }
    let mut hits: Vec<(usize, Seen)> = seen(home, user)
        .into_iter()
        .filter_map(|s| Some((score(&words, &s.name, &s.about), s)).filter(|(points, _)| *points > 0))
        .collect();
    hits.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.name.cmp(&b.1.name)));
    let mut lines = Vec::new();
    if !hits.is_empty() {
        lines.push(format!("{} on this PC:", hits.len()));
        lines.extend(hits.iter().take(FOUND_MAX).map(|(_, s)| format!("{} | {}", row(s), slash(&s.dir))));
    }
    if online || hits.is_empty() {
        lines.push(web::find(home, &words).unwrap_or_else(|fail| fail.text()));
    } else {
        lines.push("the internet too: the same call with --web".into());
    }
    Ok(lines.join("\n"))
}

fn named(name: Option<&String>) -> Result<&str, Fail> {
    let name = name.map(String::as_str).unwrap_or_default();
    let fine = !name.is_empty()
        && name.len() <= NAME_MAX
        && name.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
    if fine { Ok(name) } else { Err(Fail::hint("a skill name is a-z, 0-9 and dashes, like pdf-forms", USAGE)) }
}

struct Wish<'a> {
    plain: Vec<&'a String>,
    file: Option<&'a str>,
    replace: bool,
    yes: bool,
}

fn wish(args: &[String]) -> Result<Wish<'_>, Fail> {
    let mut w = Wish { plain: Vec::new(), file: None, replace: false, yes: false };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--replace" => w.replace = true,
            "--yes" | "-y" => w.yes = true,
            "--file" => w.file = it.next().map(String::as_str),
            _ if a.starts_with("--") => return Err(Fail::hint(format!("skill does not know {a}"), USAGE)),
            _ => w.plain.push(a),
        }
    }
    Ok(w)
}

fn check(home: &Path, args: &[String]) -> Result<String, Fail> {
    let w = wish(args)?;
    let Some(path) = w.plain.first() else {
        return Err(Fail::hint("check needs the folder or the name of a skill", USAGE));
    };
    let (given, kept) = (folder_of(path), store(home).join(path));
    let known = !given.exists() && named(Some(path)).is_ok() && kept.join(SKILL).is_file();
    let c = inspect(if known { &kept } else { &given })?;
    if !c.errs.is_empty() {
        return Err(Fail::new(format!(
            "the skill '{}' is not right yet: {}{}",
            c.name,
            c.errs.join("; "),
            better(&c.warns)
        )));
    }
    let scripts = if c.scripts.is_empty() { String::new() } else { format!(", {} script(s)", c.scripts.len()) };
    Ok(format!("ok: the skill '{}' is right{scripts}{}", c.name, better(&c.warns)))
}

fn better(warns: &[String]) -> String {
    if warns.is_empty() { String::new() } else { format!("\nbetter still: {}", warns.join("; ")) }
}

fn folder_of(path: &str) -> PathBuf {
    let path = Path::new(path);
    if path.is_file() { path.parent().map(Path::to_path_buf).unwrap_or_default() } else { path.to_path_buf() }
}

fn fresh(home: &Path) -> PathBuf {
    store(home).join(format!(".new-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)))
}

fn mirror(src: &Path, dst: &Path) -> io::Result<()> {
    copy_tree(src, dst)?;
    let mut there = Vec::new();
    walk(dst, dst, &mut there);
    for (rel, _) in there {
        if !src.join(&rel).is_file() {
            fs::remove_file(dst.join(&rel))?;
        }
    }
    Ok(())
}

fn doubts(c: &Checked, from: &str, strangers: &[&str]) -> Vec<String> {
    let mut why = Vec::new();
    let named = c.scripts.iter().take(NAMED_MAX).cloned().collect::<Vec<_>>().join(", ");
    if from.starts_with(web::ORIGIN) && !c.scripts.is_empty() {
        why.push(format!("brings {} script(s) from the internet ({named})", c.scripts.len()));
    } else if let Some(found) = risk::of(&c.code) {
        why.push(format!("has a script that {} ('{}')", found.risk.what(), found.mark));
    }
    if let Some(veiled) = c.veiled.filter(|_| from.starts_with(web::ORIGIN)) {
        why.push(veiled.to_string());
    }
    if !strangers.is_empty() {
        why.push(format!("would overwrite another skill of that name in {}", strangers.join(", ")));
    }
    why
}

fn strangers(spots: &[(&'static str, PathBuf)], name: &str) -> Vec<&'static str> {
    let mut out: Vec<&'static str> = Vec::new();
    for (label, place) in spots {
        let there = place.join(name);
        if there.exists() && !there.join(MARK).is_file() && !out.contains(label) {
            out.push(label);
        }
    }
    out
}

fn keep(home: &Path, src: &Path, dir: &Path, same: bool, mark: &[u8]) -> io::Result<()> {
    if same {
        return wisply_core::save(dir.join(MARK), mark);
    }
    let new = fresh(home);
    let filled = copy_tree(src, &new).and_then(|()| wisply_core::save(new.join(MARK), mark));
    let cleared = filled.and_then(|()| if dir.exists() { fs::remove_dir_all(dir) } else { Ok(()) });
    let moved = cleared.and_then(|()| fs::rename(&new, dir));
    if moved.is_err() {
        let _ = fs::remove_dir_all(&new);
    }
    moved
}

fn spread(spots: &[(&'static str, PathBuf)], dir: &Path, name: &str) -> String {
    let (mut done, mut failed): (Vec<&str>, Vec<String>) = (Vec::new(), Vec::new());
    for (label, place) in spots {
        match mirror(dir, &place.join(name)) {
            Ok(()) if done.contains(label) => {}
            Ok(()) => done.push(label),
            Err(e) => failed.push(format!("{label} ({e})")),
        }
    }
    let ais = if done.is_empty() { "no AI yet (none of their folders is here)".to_string() } else { done.join(", ") };
    let missed = if failed.is_empty() {
        String::new()
    } else {
        format!(
            "
not installed for: {}",
            failed.join(", ")
        )
    };
    format!("installed for {ais}; new sessions see it{missed}")
}

fn take(home: &Path, user: &Path, src: &Path, from: &str, w: &Wish<'_>) -> Result<String, Fail> {
    let c = inspect(src)?;
    if !c.errs.is_empty() {
        let fix = "fix that, then the same call (wisply skill new shows what a skill needs)";
        return Err(Fail::hint(format!("the skill '{}' is not right yet: {}", c.name, c.errs.join("; ")), fix));
    }
    let dir = store(home).join(&c.name);
    let kept = dir.join(SKILL).is_file();
    let same = kept && fs::canonicalize(src).ok() == fs::canonicalize(&dir).ok();
    if kept && !same && !w.replace {
        let hint = "the same call with --replace, or another name";
        return Err(Fail::hint(format!("wisply already keeps a skill named '{}'", c.name), hint));
    }
    let spots = places(user);
    let why = doubts(&c, from, &strangers(&spots, &c.name));
    if !why.is_empty() && !w.yes {
        return Err(Fail::hint(
            format!(
                "the skill '{}' {}, so the user decides: nothing was done, ask them first",
                c.name,
                why.join(" and ")
            ),
            "the same call with --yes once they agreed",
        ));
    }
    let mark = serde_json::to_vec_pretty(&json!({"name": c.name, "from": from, "agreed": why})).unwrap_or_default();
    keep(home, src, &dir, same, &mark)
        .map_err(|e| Fail::new(format!("cannot save the skill in {}: {e}", slash(&dir))))?;
    let agreed = if why.is_empty() {
        String::new()
    } else {
        format!(
            "
the user agreed: it {}",
            why.join(" and ")
        )
    };
    let spread = spread(&spots, &dir, &c.name);
    Ok(format!("ok: skill '{}' kept in {} and {spread}{agreed}{}", c.name, slash(&dir), better(&c.warns)))
}

fn yaml(text: &str) -> String {
    let plain =
        text.starts_with(char::is_alphanumeric) && !text.contains(": ") && !text.contains(" #") && !text.ends_with(':');
    if plain { text.to_string() } else { format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\"")) }
}

fn written(file: Option<&str>) -> Result<String, Fail> {
    let Some(path) = file else {
        let mut text = String::new();
        if !io::stdin().is_terminal() {
            let _ = io::stdin().take(TEXT_MAX).read_to_string(&mut text);
        }
        return Ok(text);
    };
    let small = fs::metadata(path).is_ok_and(|m| m.is_file() && m.len() <= TEXT_MAX);
    small
        .then(|| fs::read_to_string(path).ok())
        .flatten()
        .ok_or_else(|| Fail::new(format!("{path} is not a text file under 1 MB that can be read")))
}

fn new(home: &Path, user: &Path, args: &[String]) -> Result<String, Fail> {
    let w = wish(args)?;
    if w.plain.is_empty() {
        return Ok(GUIDE.into());
    }
    let name = named(w.plain.first().copied())?;
    let about = w.plain.get(1).map(|a| a.split_whitespace().collect::<Vec<_>>().join(" ")).unwrap_or_default();
    if about.is_empty() {
        return Err(Fail::hint(format!("say what '{name}' does and when to use it"), USAGE));
    }
    let text = written(w.file)?;
    if text.trim().is_empty() {
        let hint = "write its steps into a file, then the same call with --file text.md (wisply skill new shows what a \
                    skill needs)";
        return Err(Fail::hint(format!("the skill '{name}' has no text yet"), hint));
    }
    if text.trim_start().starts_with("---") {
        return Err(Fail::hint("that text already starts with frontmatter", "wisply skill add <its folder>"));
    }
    let draft = fresh(home).join(name);
    let body = format!("---\nname: {name}\ndescription: {}\n---\n\n{}\n", yaml(&about), text.trim());
    let made = fs::create_dir_all(&draft).and_then(|()| fs::write(draft.join(SKILL), body));
    let done = match made {
        Ok(()) => take(home, user, &draft, "built here", &w),
        Err(e) => Err(Fail::new(format!("cannot write the skill in {}: {e}", slash(&draft)))),
    };
    if let Some(parent) = draft.parent() {
        let _ = fs::remove_dir_all(parent);
    }
    done
}

fn add(home: &Path, user: &Path, args: &[String]) -> Result<String, Fail> {
    let w = wish(args)?;
    let Some(path) = w.plain.first() else { return Err(Fail::hint("add needs the folder of a skill", USAGE)) };
    take(home, user, &folder_of(path), "a folder on this PC", &w)
}

fn get(home: &Path, user: &Path, args: &[String]) -> Result<String, Fail> {
    let w = wish(args)?;
    let Some(spot) = w.plain.first() else {
        return Err(Fail::hint(
            "get needs the place of a skill",
            "wisply skill get <owner/repo/path>, as find --web prints it",
        ));
    };
    let room = fresh(home);
    let done = web::fetch(spot, &room).and_then(|(dir, from)| take(home, user, &dir, &from, &w));
    let _ = fs::remove_dir_all(&room);
    done
}

fn remove(home: &Path, user: &Path, args: &[String]) -> Result<String, Fail> {
    let name = named(args.first())?;
    let mut gone: Vec<&str> = Vec::new();
    for (label, place) in places(user) {
        let there = place.join(name);
        if there.join(MARK).is_file() && fs::remove_dir_all(&there).is_ok() && !gone.contains(&label) {
            gone.push(label);
        }
    }
    let dir = store(home).join(name);
    let kept = dir.join(SKILL).is_file() && fs::remove_dir_all(&dir).is_ok();
    if !kept && gone.is_empty() {
        let hint = "wisply skill (skills that other tools installed stay where they are)";
        return Err(Fail::hint(format!("wisply keeps no skill named '{name}'"), hint));
    }
    let from = if gone.is_empty() { String::new() } else { format!(", also from {}", gone.join(", ")) };
    Ok(format!("ok: skill '{name}' removed{from}"))
}

#[cfg(test)]
#[path = "skill/tests.rs"]
mod tests;
