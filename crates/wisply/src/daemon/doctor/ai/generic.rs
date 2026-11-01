use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use serde_json::Value;

use super::claude::frontmatter;
use super::claude::{clean_name, names_line};
use super::{Ctx, find_program};
use crate::daemon::doctor::{Level, Row, row};
use crate::daemon::media::tools;

const VERSION_WAIT: Duration = Duration::from_secs(15);
const VERSION_CHARS: usize = 60;
const DEFAULT_WARN: usize = 40_000;
const KILO: f64 = 1000.0;

#[derive(Clone, Copy, PartialEq)]
pub enum Format {
    Json,
    Toml,
    Opaque,
}

pub struct Cli {
    pub id: &'static str,
    pub programs: &'static [&'static str],
    pub dir: &'static [&'static str],
    pub config: &'static str,
    pub format: Format,
    pub mcp_key: &'static str,
    pub docs: &'static [&'static str],
    pub skills: &'static str,
    pub warn: usize,
}

pub const CLIS: [Cli; 6] = [
    Cli {
        id: "codex",
        programs: &["codex"],
        dir: &[".codex"],
        config: "config.toml",
        format: Format::Toml,
        mcp_key: "mcp_servers",
        docs: &["AGENTS.md"],
        skills: "skills",
        warn: 32_768,
    },
    Cli {
        id: "gemini",
        programs: &["gemini"],
        dir: &[".gemini"],
        config: "settings.json",
        format: Format::Json,
        mcp_key: "mcpServers",
        docs: &["GEMINI.md"],
        skills: "skills",
        warn: DEFAULT_WARN,
    },
    Cli {
        id: "qwen",
        programs: &["qwen"],
        dir: &[".qwen"],
        config: "settings.json",
        format: Format::Json,
        mcp_key: "mcpServers",
        docs: &["QWEN.md"],
        skills: "skills",
        warn: DEFAULT_WARN,
    },
    Cli {
        id: "opencode",
        programs: &["opencode"],
        dir: &[".config", "opencode"],
        config: "opencode.json",
        format: Format::Opaque,
        mcp_key: "",
        docs: &["AGENTS.md"],
        skills: "skills",
        warn: DEFAULT_WARN,
    },
    Cli {
        id: "cursor",
        programs: &["cursor-agent"],
        dir: &[".cursor"],
        config: "mcp.json",
        format: Format::Json,
        mcp_key: "mcpServers",
        docs: &[],
        skills: "",
        warn: DEFAULT_WARN,
    },
    Cli {
        id: "continue",
        programs: &["cn"],
        dir: &[".continue"],
        config: "config.yaml",
        format: Format::Opaque,
        mcp_key: "",
        docs: &[],
        skills: "",
        warn: DEFAULT_WARN,
    },
];

fn cli(id: &str) -> Option<&'static Cli> {
    CLIS.iter().find(|c| c.id == id)
}

fn dir_of(ctx: &Ctx, c: &Cli) -> PathBuf {
    c.dir.iter().fold(ctx.user.clone(), |p, part| p.join(part))
}

pub fn detected(ctx: &Ctx, id: &str) -> bool {
    let Some(c) = cli(id) else { return false };
    dir_of(ctx, c).join(c.config).is_file() || find_program(c.programs).is_some()
}

fn kilo(n: usize) -> String {
    if n as f64 >= KILO { format!("{:.1}k", n as f64 / KILO) } else { n.to_string() }
}

fn position(text: &str, offset: usize) -> (usize, usize) {
    let before = &text[..offset.min(text.len())];
    let line = before.matches('\n').count() + 1;
    let column = before.rsplit('\n').next().map_or(0, |l| l.chars().count()) + 1;
    (line, column)
}

pub fn parse_problem(text: &str, format: Format) -> Option<String> {
    match format {
        Format::Json => serde_json::from_str::<Value>(text)
            .err()
            .map(|e| format!("invalid JSON at line {} column {}", e.line(), e.column())),
        Format::Toml => toml::from_str::<toml::Value>(text).err().map(|e| {
            let (line, column) = position(text, e.span().map_or(0, |s| s.start));
            format!("invalid TOML at line {line} column {column}")
        }),
        Format::Opaque => None,
    }
}

pub fn mcp_names(text: &str, c: &Cli) -> Vec<String> {
    if c.mcp_key.is_empty() {
        return Vec::new();
    }
    match c.format {
        Format::Json => serde_json::from_str::<Value>(text)
            .ok()
            .and_then(|v| v.get(c.mcp_key).and_then(Value::as_object).map(|m| m.keys().cloned().collect()))
            .unwrap_or_default(),
        Format::Toml => toml::from_str::<toml::Value>(text)
            .ok()
            .and_then(|v| v.get(c.mcp_key).and_then(toml::Value::as_table).map(|t| t.keys().cloned().collect()))
            .unwrap_or_default(),
        Format::Opaque => Vec::new(),
    }
}

fn install_row(ctx: &Ctx, c: &Cli) -> Row {
    let Some(path) = find_program(c.programs) else {
        let folder = dir_of(ctx, c);
        return row(Level::Ok, "install", format!("config folder {} found, no command on PATH", folder.display()), "");
    };
    let ran = tools::run(&path, &["--version".to_string()], VERSION_WAIT).ok();
    let version = ran
        .and_then(|r| r.out.lines().next().map(|l| l.trim().chars().take(VERSION_CHARS).collect::<String>()))
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "version unknown".into());
    row(Level::Ok, "install", format!("{version} ({})", path.display()), "")
}

fn config_rows(ctx: &Ctx, c: &Cli) -> Vec<Row> {
    let file = dir_of(ctx, c).join(c.config);
    let Ok(text) = fs::read_to_string(&file) else { return Vec::new() };
    let mut rows = Vec::new();
    match parse_problem(&text, c.format) {
        Some(why) => {
            rows.push(row(
                Level::Fail,
                "config",
                format!("{}: {why}", file.display()),
                "repair the file at that position",
            ));
            return rows;
        }
        None if c.format == Format::Opaque => {
            rows.push(row(Level::Ok, "config", format!("{} present (format not checked)", file.display()), ""));
        }
        None => rows.push(row(Level::Ok, "config", format!("{} parses", file.display()), "")),
    }
    let names = mcp_names(&text, c);
    if !names.is_empty() {
        let clean: Vec<String> = names.iter().filter(|n| clean_name(n)).cloned().collect();
        let odd = names.len() - clean.len();
        let extra = if odd > 0 { format!(" (+{odd} with odd names)") } else { String::new() };
        rows.push(row(Level::Ok, "mcp", format!("{} MCP servers: {}{extra}", names.len(), names_line(&clean)), ""));
    }
    rows
}

fn doc_files(ctx: &Ctx, c: &Cli) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    for doc in c.docs {
        files.push(dir_of(ctx, c).join(doc));
        files.extend(ctx.cwd.ancestors().map(|d| d.join(doc)));
    }
    let mut seen: Vec<PathBuf> = Vec::new();
    for file in files.into_iter().filter(|f| f.is_file()) {
        let key = fs::canonicalize(&file).unwrap_or_else(|_| file.clone());
        if !seen.contains(&key) {
            seen.push(key);
        }
    }
    seen
}

fn doc_rows(ctx: &Ctx, c: &Cli) -> Vec<Row> {
    let mut rows = Vec::new();
    for file in doc_files(ctx, c) {
        let size = fs::metadata(&file).map_or(0, |m| m.len() as usize);
        if size > c.warn {
            rows.push(row(
                Level::Warn,
                "docs",
                format!(
                    "{} is {} bytes, over the {} this tool reads or warns at",
                    file.display(),
                    kilo(size),
                    kilo(c.warn)
                ),
                "cut what a session can read from the code itself",
            ));
        } else {
            rows.push(row(Level::Ok, "docs", format!("{} {} bytes", file.display(), kilo(size)), ""));
        }
    }
    rows
}

fn skill_rows(ctx: &Ctx, c: &Cli) -> Vec<Row> {
    if c.skills.is_empty() {
        return Vec::new();
    }
    let base = dir_of(ctx, c).join(c.skills);
    let mut dirs: Vec<PathBuf> = fs::read_dir(&base)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.join("SKILL.md").is_file())
        .collect();
    dirs.sort();
    let mut rows = Vec::new();
    for dir in &dirs {
        let file = dir.join("SKILL.md");
        let checked = fs::read_to_string(&file).map_err(|e| e.to_string()).and_then(|t| frontmatter::parse(&t));
        if let Err(why) = checked {
            rows.push(row(
                Level::Warn,
                "skills",
                format!("{}: {why}", file.display()),
                "fix the frontmatter line named above (quote the value)",
            ));
        }
    }
    if rows.is_empty() && !dirs.is_empty() {
        rows.push(row(Level::Ok, "skills", format!("{} skills, frontmatter parses", dirs.len()), ""));
    }
    rows
}

pub fn rows(ctx: &Ctx, id: &str) -> Vec<Row> {
    let Some(c) = cli(id) else { return Vec::new() };
    let mut rows = vec![install_row(ctx, c)];
    rows.extend(config_rows(ctx, c));
    rows.extend(doc_rows(ctx, c));
    rows.extend(skill_rows(ctx, c));
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scene(tag: &str) -> Ctx {
        let root = std::env::temp_dir().join(format!("lt-ai-generic-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join(".codex/skills/ok")).unwrap();
        fs::create_dir_all(root.join(".codex/skills/bad")).unwrap();
        fs::create_dir_all(root.join(".gemini")).unwrap();
        fs::create_dir_all(root.join("proj")).unwrap();
        Ctx { user: root.clone(), cwd: root.join("proj") }
    }

    #[test]
    fn toml_and_json_errors_name_the_position_and_mcp_names_are_listed() {
        let codex = cli("codex").unwrap();
        let gemini = cli("gemini").unwrap();
        let good = "model = \"x\"\n[mcp_servers.alpha]\ncommand = \"a\"\n[mcp_servers.beta]\ncommand = \"b\"\n";
        assert_eq!(parse_problem(good, Format::Toml), None);
        assert_eq!(mcp_names(good, codex), vec!["alpha", "beta"]);
        let bad = "model = \"x\"\n[mcp_servers\ncommand = 1\n";
        assert!(parse_problem(bad, Format::Toml).unwrap().starts_with("invalid TOML at line 2"));
        assert_eq!(mcp_names(r#"{"mcpServers":{"one":{},"two":{}}}"#, gemini), vec!["one", "two"]);
        assert!(parse_problem("{\"a\": 1,}", Format::Json).unwrap().starts_with("invalid JSON at line 1"));
        assert_eq!(parse_problem("anything: [", Format::Opaque), None);
        assert_eq!(position("a\nbc\ndef", 5), (3, 1));
    }

    #[test]
    fn a_tool_is_checked_by_its_own_files() {
        let ctx = scene("one");
        fs::write(ctx.user.join(".codex/config.toml"), "[mcp_servers.alpha]\ncommand = \"a\"\n").unwrap();
        fs::write(ctx.user.join(".codex/AGENTS.md"), "x".repeat(40_000)).unwrap();
        fs::write(ctx.cwd.join("AGENTS.md"), "short").unwrap();
        fs::write(ctx.user.join(".codex/skills/ok/SKILL.md"), "---\nname: ok\ndescription: fine\n---\n").unwrap();
        fs::write(ctx.user.join(".codex/skills/bad/SKILL.md"), "---\nname: bad\ndescription: Use when: x\n---\n")
            .unwrap();
        assert!(detected(&ctx, "codex"));
        assert!(!detected(&ctx, "gemini"));
        let rows = rows(&ctx, "codex");
        let all: String = rows.iter().map(|r| format!("{} {}\n", r.name, r.text)).collect();
        assert!(all.contains("config.toml parses") && all.contains("1 MCP servers: alpha"), "{all}");
        assert!(all.contains("AGENTS.md is 40.0k bytes, over the 32.8k"), "{all}");
        assert!(all.contains("AGENTS.md 5 bytes"), "{all}");
        assert!(all.contains("bad") && all.contains("unquoted value"), "{all}");
        fs::write(ctx.user.join(".codex/config.toml"), "[broken\n").unwrap();
        let broken = super::rows(&ctx, "codex");
        assert!(broken.iter().any(|r| r.level == Level::Fail && r.text.contains("invalid TOML")));
        fs::remove_dir_all(&ctx.user).unwrap();
    }
}
