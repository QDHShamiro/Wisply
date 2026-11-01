use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::frontmatter::{self, Front};
use super::{Env, Level, Row, row};

const NAME: &str = "name";
const DESCRIPTION: &str = "description";

pub fn json_files(env: &Env) -> Vec<PathBuf> {
    let [user, project, local] = env.settings();
    vec![user, env.claude.join("settings.local.json"), env.config.clone(), project, local, env.cwd.join(".mcp.json")]
}

pub fn parse_problem(path: &Path) -> Option<String> {
    let text = fs::read_to_string(path).ok()?;
    serde_json::from_str::<serde_json::Value>(&text)
        .err()
        .map(|e| format!("{}: invalid JSON at line {} column {}", path.display(), e.line(), e.column()))
}

fn markdown_files(dir: &Path, deep: bool) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() && deep {
            out.extend(markdown_files(&path, deep));
        } else if path.extension().is_some_and(|e| e == "md") {
            out.push(path);
        }
    }
    out.sort();
    out
}

pub fn skill_dirs(env: &Env) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for base in [env.claude.join("skills"), env.cwd.join(".claude").join("skills")] {
        for entry in fs::read_dir(base).into_iter().flatten().flatten() {
            if entry.path().join("SKILL.md").is_file() {
                out.push(entry.path());
            }
        }
    }
    out.sort();
    out
}

pub fn front_of(path: &Path) -> Result<Option<Front>, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    frontmatter::parse(&text)
}

fn agent_rows(env: &Env) -> Vec<Row> {
    let mut rows = Vec::new();
    let dirs = [(env.claude.join("agents"), false), (env.cwd.join(".claude").join("agents"), true)];
    for (dir, deep) in dirs {
        let mut seen: BTreeMap<(PathBuf, String), Vec<PathBuf>> = BTreeMap::new();
        for file in markdown_files(&dir, deep) {
            let Ok(Some(front)) = front_of(&file) else { continue };
            let Some(name) = front.value(NAME) else { continue };
            if !front.has(DESCRIPTION) || front.value(DESCRIPTION).is_some_and(str::is_empty) {
                rows.push(row(
                    Level::Warn,
                    "agents",
                    format!("{} has a name but no description, Claude Code never loads it", file.display()),
                    "add a description: line to its frontmatter",
                ));
            }
            let parent = file.parent().map(Path::to_path_buf).unwrap_or_default();
            seen.entry((parent, name.to_string())).or_default().push(file);
        }
        for ((_, name), group) in seen.into_iter().filter(|(_, g)| g.len() > 1) {
            let list: Vec<String> = group.iter().map(|p| p.display().to_string()).collect();
            rows.push(row(
                Level::Warn,
                "agents",
                format!("{} files use the name '{name}': {}", group.len(), list.join(", ")),
                "rename or remove all but one so the name is unique",
            ));
        }
    }
    rows
}

fn skill_rows(env: &Env) -> Vec<Row> {
    let mut rows = Vec::new();
    for dir in skill_dirs(env) {
        let file = dir.join("SKILL.md");
        if let Err(why) = front_of(&file) {
            rows.push(row(
                Level::Warn,
                "skills",
                format!("{}: {why}, so every field is dropped", file.display()),
                "fix the frontmatter line named above (quote the value)",
            ));
        }
    }
    rows
}

pub fn rows(env: &Env) -> Vec<Row> {
    let mut rows = Vec::new();
    let present: Vec<PathBuf> = json_files(env).into_iter().filter(|p| p.is_file()).collect();
    let broken: Vec<String> = present.iter().filter_map(|p| parse_problem(p)).collect();
    if broken.is_empty() {
        rows.push(row(Level::Ok, "files", format!("{} settings and MCP files parse", present.len()), ""));
    }
    for why in broken {
        rows.push(row(
            Level::Fail,
            "files",
            format!("{why} (Claude Code ignores the whole file)"),
            "repair the file at that position",
        ));
    }
    let agents = agent_rows(env);
    let skills = skill_rows(env);
    if agents.is_empty() && skills.is_empty() {
        rows.push(row(Level::Ok, "agents", "agent and skill frontmatter parse, no name collisions", ""));
    }
    rows.extend(agents);
    rows.extend(skills);
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scene(tag: &str) -> Env {
        let root = std::env::temp_dir().join(format!("lt-cd-files-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let claude = root.join("claude");
        fs::create_dir_all(claude.join("agents")).unwrap();
        fs::create_dir_all(claude.join("skills/bad")).unwrap();
        fs::create_dir_all(claude.join("skills/good")).unwrap();
        fs::create_dir_all(root.join("proj")).unwrap();
        Env { user: root.clone(), claude: claude.clone(), config: root.join(".claude.json"), cwd: root.join("proj") }
    }

    #[test]
    fn broken_json_agents_and_skills_are_found_without_printing_contents() {
        let env = scene("all");
        fs::write(env.claude.join("settings.json"), "{\"env\": {\"SECRET\": \"hunter2\",}\n}").unwrap();
        fs::write(&env.config, "{}").unwrap();
        fs::write(env.claude.join("agents/a.md"), "---\nname: dup\ndescription: x\n---\n").unwrap();
        fs::write(env.claude.join("agents/b.md"), "---\nname: dup\ndescription: y\n---\n").unwrap();
        fs::write(env.claude.join("agents/c.md"), "---\nname: nodesc\n---\n").unwrap();
        fs::write(env.claude.join("agents/doc.md"), "# notes only\n").unwrap();
        fs::write(env.claude.join("skills/bad/SKILL.md"), "---\nname: bad\ndescription: Use when: x\n---\n").unwrap();
        fs::write(env.claude.join("skills/good/SKILL.md"), "---\nname: good\ndescription: fine\n---\n").unwrap();
        let rows = rows(&env);
        let text: Vec<String> = rows.iter().map(|r| format!("{} {}", r.name, r.text)).collect();
        let all = text.join("\n");
        assert!(all.contains("files") && all.contains("invalid JSON at line 1"), "{all}");
        assert!(!all.contains("hunter2"), "{all}");
        assert!(all.contains("2 files use the name 'dup'"), "{all}");
        assert!(all.contains("c.md has a name but no description"), "{all}");
        assert!(all.contains("skills") && all.contains("bad") && all.contains("unquoted value"), "{all}");
        assert!(!all.contains("good"), "{all}");
        assert!(!all.contains("doc.md"), "{all}");
        fs::remove_dir_all(env.user).unwrap();
    }

    #[test]
    fn a_clean_setup_gives_ok_rows() {
        let env = scene("clean");
        fs::write(env.claude.join("settings.json"), "{}").unwrap();
        let rows = rows(&env);
        assert!(
            rows.iter().all(|r| r.level == Level::Ok),
            "{}",
            rows.iter().map(|r| r.text.clone()).collect::<String>()
        );
        fs::remove_dir_all(env.user).unwrap();
    }
}
