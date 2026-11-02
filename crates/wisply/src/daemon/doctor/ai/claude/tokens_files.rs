use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::files::{front_of, skill_dirs};
use super::tokens_scan::LEGACY;
use super::{Env, read_json};

const FOLDED: [&str; 4] = ["|", ">", "|-", ">-"];
const OWN_PLUGIN: &str = "wisply@";
const NO_MODEL: &str = "disable-model-invocation";

pub struct SkillInfo {
    pub name: String,
    pub description: usize,
    pub body: usize,
}

pub struct MemoryFile {
    pub label: String,
    pub chars: usize,
    pub long_lines: usize,
}

pub struct Command {
    pub name: String,
    pub description: usize,
}

pub fn tokens(chars: usize) -> usize {
    chars.div_ceil(4)
}

fn split(text: &str) -> (String, usize) {
    let text = text.trim_start_matches('\u{feff}');
    let mut lines = text.lines();
    if lines.next().map(str::trim_end) != Some("---") {
        return (String::new(), text.chars().count());
    }
    let mut front = String::new();
    let mut body = 0;
    let mut closed = false;
    for line in lines {
        if closed {
            body += line.chars().count() + 1;
        } else if line.trim_end() == "---" {
            closed = true;
        } else {
            front.push_str(line);
            front.push('\n');
        }
    }
    (front, body)
}

fn description_chars(front: &str) -> usize {
    let mut lines = front.lines();
    while let Some(line) = lines.next() {
        let Some(rest) = line.strip_prefix("description:") else { continue };
        let rest = rest.trim();
        if !FOLDED.contains(&rest) {
            return rest.trim_matches(['"', '\'']).chars().count();
        }
        let mut total = 0;
        for next in lines.by_ref() {
            if !next.starts_with([' ', '\t']) {
                break;
            }
            total += next.trim().chars().count() + 1;
        }
        return total;
    }
    0
}

pub fn skills(env: &Env) -> Vec<SkillInfo> {
    let mut out = Vec::new();
    for dir in skill_dirs(env) {
        let Some(name) = dir.file_name().map(|n| n.to_string_lossy().into_owned()) else { continue };
        let Ok(text) = fs::read_to_string(dir.join("SKILL.md")) else { continue };
        let (front, body) = split(&text);
        out.push(SkillInfo { name, description: description_chars(&front), body });
    }
    out
}

fn memory_file(label: String, path: &Path, long: usize) -> Option<MemoryFile> {
    let text = fs::read_to_string(path).ok()?;
    let long_lines = text.lines().filter(|l| l.chars().count() > long).count();
    Some(MemoryFile { label, chars: text.chars().count(), long_lines })
}

pub fn memory_files(env: &Env, long: usize) -> Vec<MemoryFile> {
    let mut found: Vec<MemoryFile> = Vec::new();
    let fixed = [
        ("CLAUDE.md (user)".to_string(), env.claude.join("CLAUDE.md")),
        ("CLAUDE.md (project)".to_string(), env.cwd.join("CLAUDE.md")),
        ("MEMORY.md (user)".to_string(), env.claude.join("memory").join("MEMORY.md")),
    ];
    for (label, path) in fixed {
        found.extend(memory_file(label, &path, long));
    }
    let mut dirs: Vec<PathBuf> =
        fs::read_dir(env.claude.join("projects")).into_iter().flatten().flatten().map(|e| e.path()).collect();
    dirs.sort();
    for (n, dir) in dirs.iter().enumerate() {
        found.extend(memory_file(
            format!("MEMORY.md (project {})", n + 1),
            &dir.join("memory").join("MEMORY.md"),
            long,
        ));
    }
    found
}

fn hook_commands(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::String(s) => out.push(s.clone()),
        Value::Array(items) => items.iter().for_each(|v| hook_commands(v, out)),
        Value::Object(map) => map.values().for_each(|v| hook_commands(v, out)),
        _ => {}
    }
}

pub fn legacy_leftovers(env: &Env) -> Vec<(String, String)> {
    let mut found: BTreeSet<(String, String)> = BTreeSet::new();
    let [user, project, local] = env.settings();
    for file in [user, env.claude.join("settings.local.json"), project, local] {
        let Some(settings) = read_json(&file) else { continue };
        let mut commands = Vec::new();
        if let Some(hooks) = settings.get("hooks") {
            hook_commands(hooks, &mut commands);
        }
        for tag in LEGACY.iter().filter(|t| commands.iter().any(|c| c.contains(*t))) {
            found.insert(((*tag).to_string(), "hook".to_string()));
        }
        let plugins = settings.get("enabledPlugins").and_then(Value::as_object).into_iter().flatten();
        for (key, _) in plugins.filter(|(_, v)| v.as_bool() == Some(true)) {
            for tag in LEGACY.iter().filter(|t| key.contains(*t)) {
                found.insert(((*tag).to_string(), "enabled plugin".to_string()));
            }
        }
    }
    found.into_iter().collect()
}

fn command_dirs(env: &Env) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let installed = read_json(&env.claude.join("plugins").join("installed_plugins.json")).unwrap_or(Value::Null);
    let plugins = installed.get("plugins").unwrap_or(&installed);
    for (_, list) in plugins.as_object().into_iter().flatten().filter(|(k, _)| k.starts_with(OWN_PLUGIN)) {
        let paths = list.as_array().into_iter().flatten();
        dirs.extend(paths.filter_map(|e| e.get("installPath")?.as_str()).map(|p| Path::new(p).join("commands")));
    }
    if env.cwd.join(".claude-plugin").is_dir() {
        dirs.push(env.cwd.join("commands"));
    }
    dirs
}

pub fn open_commands(env: &Env) -> Vec<Command> {
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut out = Vec::new();
    for dir in command_dirs(env) {
        let mut files: Vec<PathBuf> = fs::read_dir(dir).into_iter().flatten().flatten().map(|e| e.path()).collect();
        files.sort();
        for file in files.iter().filter(|f| f.extension().is_some_and(|e| e == "md")) {
            let Some(name) = file.file_stem().map(|n| n.to_string_lossy().into_owned()) else { continue };
            let Ok(Some(front)) = front_of(file) else { continue };
            let blocked = front.value(NO_MODEL).is_some_and(|v| v == "true");
            if blocked || !seen.insert(name.clone()) {
                continue;
            }
            let description = front.value("description").map_or(0, |d| d.chars().count());
            out.push(Command { name, description });
        }
    }
    out
}
