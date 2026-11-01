use std::fs;
use std::path::{Path, PathBuf};

use super::files::{front_of, skill_dirs};
use super::usage::{is_off, skill_modes};
use super::{Env, Level, Row, row};

const CHARS_PER_TOKEN: usize = 4;
const WARN_CHARS: usize = 40_000;
const NAME_ONLY: &str = "name-only";
const KILO: f64 = 1000.0;

fn sanitized(cwd: &Path) -> String {
    cwd.to_string_lossy().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect()
}

pub fn instruction_files(env: &Env) -> Vec<PathBuf> {
    let mut files = vec![env.claude.join("CLAUDE.md")];
    for dir in env.cwd.ancestors() {
        files.extend([dir.join("CLAUDE.md"), dir.join("CLAUDE.local.md"), dir.join(".claude").join("CLAUDE.md")]);
    }
    files.push(env.claude.join("projects").join(sanitized(&env.cwd)).join("memory").join("MEMORY.md"));
    let mut seen: Vec<PathBuf> = Vec::new();
    for file in files.into_iter().filter(|f| f.is_file()) {
        let key = fs::canonicalize(&file).unwrap_or_else(|_| file.clone());
        if !seen.contains(&key) {
            seen.push(key);
        }
    }
    seen
}

fn chars_of(path: &Path) -> usize {
    fs::read_to_string(path).map_or(0, |t| t.chars().count())
}

fn tokens(chars: usize) -> usize {
    chars / CHARS_PER_TOKEN
}

fn k(n: usize) -> String {
    if n as f64 >= KILO { format!("{:.1}k", n as f64 / KILO) } else { n.to_string() }
}

fn skill_listing(env: &Env) -> (usize, usize) {
    let modes = skill_modes(env);
    let (mut count, mut chars) = (0, 0);
    for dir in skill_dirs(env) {
        let folder = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        if is_off(&modes, &folder) {
            continue;
        }
        let front = front_of(&dir.join("SKILL.md")).ok().flatten();
        let name = front.as_ref().and_then(|f| f.value("name")).unwrap_or(&folder).to_string();
        let name_only = modes.get(&folder).is_some_and(|m| m == NAME_ONLY);
        let description = front.as_ref().and_then(|f| f.value("description")).unwrap_or("");
        chars += name.chars().count() + if name_only { 0 } else { description.chars().count() };
        count += 1;
    }
    (count, chars)
}

pub fn rows(env: &Env) -> Vec<Row> {
    let files = instruction_files(env);
    let mut rows = Vec::new();
    let mut total = 0;
    let mut parts = Vec::new();
    for file in &files {
        let chars = chars_of(file);
        total += chars;
        let name = file.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        parts.push(format!("{name} {} chars (~{} tokens est.)", k(chars), k(tokens(chars))));
        if chars > WARN_CHARS {
            rows.push(row(
                Level::Warn,
                "context",
                format!(
                    "{} is {} chars, over the {} size Claude Code warns about",
                    file.display(),
                    k(chars),
                    k(WARN_CHARS)
                ),
                "cut what a session can read from the code itself, move task workflows into a skill",
            ));
        }
    }
    let (skills, chars) = skill_listing(env);
    parts.push(format!("{skills} listed skills ~{} tokens est.", k(tokens(chars))));
    let all = k(tokens(total + chars));
    rows.insert(
        0,
        row(
            Level::Ok,
            "context",
            format!("{} | always loaded ~{all} tokens est. (/context is exact)", parts.join(", ")),
            "",
        ),
    );
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_files_skills_and_size_warnings_are_counted() {
        let root = std::env::temp_dir().join(format!("lt-cd-context-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let claude = root.join("claude");
        let cwd = root.join("proj");
        fs::create_dir_all(claude.join("skills/a")).unwrap();
        fs::create_dir_all(claude.join("skills/b")).unwrap();
        fs::create_dir_all(&cwd).unwrap();
        fs::write(claude.join("CLAUDE.md"), "x".repeat(400)).unwrap();
        fs::write(cwd.join("CLAUDE.md"), "y".repeat(40_001)).unwrap();
        fs::write(claude.join("skills/a/SKILL.md"), "---\nname: a\ndescription: aaaa\n---\n").unwrap();
        fs::write(claude.join("skills/b/SKILL.md"), "---\nname: b\ndescription: bbbbbbbb\n---\n").unwrap();
        fs::write(claude.join("settings.json"), r#"{"skillOverrides":{"b":"name-only"}}"#).unwrap();
        let env = Env { user: root.clone(), claude, config: root.join(".claude.json"), cwd };
        let rows = rows(&env);
        assert!(rows[0].text.contains("CLAUDE.md 400 chars (~100 tokens est.)"), "{}", rows[0].text);
        assert!(rows[0].text.contains("2 listed skills ~1 tokens est."), "{}", rows[0].text);
        assert!(rows.iter().any(|r| r.level == Level::Warn && r.text.contains("40.0k chars")), "{}", rows[1].text);
        assert_eq!(sanitized(Path::new("C:\\Users\\Gigabyte")), "C--Users-Gigabyte");
        fs::remove_dir_all(&root).unwrap();
    }
}
