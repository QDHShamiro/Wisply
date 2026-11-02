use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::LazyLock;

use regex::{Regex, RegexSet};

use super::report::{Finding, Sev};
use super::rules::{self, Scope};

const SECRETS: &[(&str, &str)] = &[
    ("GitHub token", r"\bgh[pousr]_[A-Za-z0-9]{20,}"),
    ("API key (sk-/pk-)", r"\b(?:sk|pk|rk)-[A-Za-z0-9_-]{16,}"),
    ("Groq key", r"\bgsk_[A-Za-z0-9]{20,}"),
    ("AWS access key", r"\bAKIA[0-9A-Z]{16}\b"),
    ("Slack token", r"\bxox[abpr]-[A-Za-z0-9-]{10,}"),
    ("Google API key", r"\bAIza[0-9A-Za-z_-]{35}\b"),
    ("Discord bot token", r"\b[MN][A-Za-z\d]{23,25}\.[\w-]{6}\.[\w-]{27,}"),
    ("private key", r"-----BEGIN (?:RSA |EC |OPENSSH |DSA |ENCRYPTED )?PRIVATE KEY-----"),
    ("JWT", r"\beyJ[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}"),
    ("Discord webhook", r"https://(?:ptb\.|canary\.)?discord(?:app)?\.com/api/webhooks/\d+/[\w-]{20,}"),
    ("password in a database URL", r#"(?i)jdbc:\w+://[^\s"']*[?&;]password=[^\s&"';]{4,}"#),
];
const KEY_FILES: &[&str] = &[".pem", ".key", ".p12", ".pfx", ".jks", ".keystore"];
const KEY_NAMES: &[&str] = &["id_rsa", "id_ed25519", "id_ecdsa"];
const SECRET_FIX: &str = "move it to an environment variable or an ignored file, then rotate the key";
const CONFIG_EXT: &[&str] = &["yml", "yaml", "json", "properties", "env", "toml", "ini", "conf", "cfg", "xml"];
const PLACEHOLDER: &[&str] = &[
    "changeme",
    "change_me",
    "password",
    "secret",
    "token",
    "example",
    "your",
    "xxx",
    "***",
    "<",
    "${",
    "{{",
    "%s",
    "null",
    "none",
    "todo",
];
const MAX_BYTES: u64 = 1_000_000;
const HISTORY_LINES: usize = 2_000_000;

static FOUND: LazyLock<Vec<(&'static str, Regex)>> =
    LazyLock::new(|| SECRETS.iter().map(|(name, p)| (*name, Regex::new(p).expect("secret"))).collect());
static SECRET_SET: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new(SECRETS.iter().map(|(_, p)| *p)).expect("secret set"));
static ASSIGN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?i)\b(password|passwd|pwd|secret|token|api[_-]?key|license[_-]?key)["']?\s*[:=]\s*["']?([^\s"',;]{6,})"#,
    )
    .expect("assign")
});
fn masked(value: &str) -> String {
    let head: String = value.chars().take(4).collect();
    format!("{head}…")
}

fn placeholder(value: &str) -> bool {
    let low = value.to_lowercase();
    PLACEHOLDER.iter().any(|p| low.starts_with(p))
        || value.contains(['(', ')', '$'])
        || value.chars().all(|c| c == '*' || c == 'x')
}

pub fn secrets_in_line(line: &str, config: bool) -> Vec<String> {
    let mut out: Vec<String> = SECRET_SET
        .matches(line)
        .iter()
        .filter_map(|i| FOUND[i].1.find(line).map(|m| format!("{} ({})", FOUND[i].0, masked(m.as_str()))))
        .collect();
    if out.is_empty()
        && config
        && let Some(m) = ASSIGN.captures(line)
        && !placeholder(&m[2])
    {
        out.push(format!("{} in plain text ({})", m[1].to_lowercase(), masked(&m[2])));
    }
    out
}

fn git(root: &Path, args: &[&str]) -> Command {
    let mut cmd = Command::new("git");
    cmd.args(["-c", "core.quotePath=false", "-C"]).arg(root).args(args).stdin(Stdio::null()).stderr(Stdio::null());
    crate::v2::hide(&mut cmd);
    cmd
}

pub fn files(root: &Path) -> Vec<String> {
    let out = git(root, &["ls-files", "-z", "-co", "--exclude-standard"]).output().ok().filter(|o| o.status.success());
    match out {
        Some(o) => String::from_utf8_lossy(&o.stdout).split('\0').filter(|n| !n.is_empty()).map(String::from).collect(),
        None => {
            let mut found = Vec::new();
            walk(root, root, &mut found);
            found
        }
    }
}

fn walk(root: &Path, dir: &Path, found: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            if !name.starts_with('.')
                && !["node_modules", "target", "build", "out", "dist", "venv"].contains(&name.as_str())
            {
                walk(root, &path, found);
            }
        } else if let Ok(rel) = path.strip_prefix(root) {
            found.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}

fn config_file(name: &str) -> bool {
    let low = name.to_lowercase();
    low.rsplit('/').next().is_some_and(|f| f.starts_with(".env"))
        || low.rsplit_once('.').is_some_and(|(_, ext)| CONFIG_EXT.contains(&ext))
}

pub fn secret_sev(why: &str) -> Sev {
    if why.starts_with("Google API key") || why.starts_with("JWT") { Sev::Medium } else { Sev::High }
}

pub fn scan_history(root: &Path) -> Vec<Finding> {
    let mut cmd = git(root, &["log", "-p", "--all", "--no-color", "-U0", "--format=commit %h"]);
    cmd.stdout(Stdio::piped());
    let Ok(mut child) = cmd.spawn() else { return Vec::new() };
    let Some(out) = child.stdout.take() else { return Vec::new() };
    let (mut found, mut commit, mut file) = (Vec::new(), String::new(), String::new());
    for line in BufReader::new(out).lines().map_while(Result::ok).take(HISTORY_LINES) {
        if let Some(h) = line.strip_prefix("commit ") {
            commit = h.trim().to_string();
        } else if let Some(f) = line.strip_prefix("+++ ") {
            file = f.strip_prefix("b/").unwrap_or(f).to_string();
        } else if let Some(added) = line.strip_prefix('+') {
            for why in secrets_in_line(added, config_file(&file)) {
                found.push(Finding::new(
                    secret_sev(&why),
                    "history",
                    format!("{file} (commit {commit})"),
                    0,
                    why,
                    "rotate the key; removing it needs a history rewrite (git filter-repo)",
                ));
            }
        }
    }
    let _ = child.kill();
    let _ = child.wait();
    found
}

fn key_file(low: &str) -> bool {
    let file = low.rsplit('/').next().unwrap_or(low);
    KEY_FILES.iter().any(|e| file.ends_with(e)) || KEY_NAMES.contains(&file)
}

pub fn scan_files(root: &Path, names: &[String]) -> Vec<Finding> {
    let mut found = Vec::new();
    for name in names {
        let low = name.to_lowercase();
        let file = low.rsplit('/').next().unwrap_or(&low);
        if key_file(&low) && !low.contains("/test") {
            found.push(Finding::new(
                Sev::High,
                "secret",
                name.clone(),
                0,
                "a key file is in the project folder",
                "keep it out of git (git rm --cached, add it to .gitignore) and rotate it if it was ever pushed",
            ));
        }
        if low.ends_with(".lock") || low.ends_with("package-lock.json") || low.contains("/test") {
            continue;
        }
        let path = root.join(name);
        if !std::fs::metadata(&path).is_ok_and(|m| m.len() <= MAX_BYTES) {
            continue;
        }
        let Ok(raw) = std::fs::read(&path) else { continue };
        if raw.iter().take(8192).any(|b| *b == 0) {
            continue;
        }
        let text = String::from_utf8_lossy(&raw);
        let config = config_file(name);
        let scope = if config { Scope::Config } else { Scope::Code };
        let judged = config || rules::code_file(name);
        if file == ".env" || (file.starts_with(".env.") && !file.ends_with("example") && !file.ends_with("sample")) {
            found.push(Finding::new(
                Sev::High,
                "secret",
                name.clone(),
                1,
                "an .env file is in the repo",
                "add it to .gitignore, keep an .env.example without values",
            ));
        }
        for (i, line) in text.lines().enumerate() {
            for why in secrets_in_line(line, config) {
                found.push(Finding::new(secret_sev(&why), "secret", name.clone(), i + 1, why, SECRET_FIX));
            }
            if judged && let Some(rule) = rules::judge(line, scope) {
                found.push(Finding::new(rule.sev, "code", name.clone(), i + 1, rule.why, rule.fix));
            }
        }
        if (low.ends_with(".java") || low.ends_with(".kt"))
            && text.contains("onCommand(")
            && !text.contains("hasPermission(")
            && let Some(at) = text.lines().position(|l| l.contains("onCommand("))
        {
            found.push(Finding::new(
                Sev::Low,
                "code",
                name.clone(),
                at + 1,
                "command without a permission check in this file",
                "check sender.hasPermission(...) or set permission: in plugin.yml",
            ));
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secrets_are_found_without_placeholders() {
        let fake = format!("token = \"ghp_{}\"", "a1B2".repeat(6));
        assert_eq!(secrets_in_line(&fake, false), vec!["GitHub token (ghp_…)"]);
        assert_eq!(secret_sev("Google API key (AIza…)"), Sev::Medium);
        assert_eq!(secrets_in_line("password: hunter2secret", true), vec!["password in plain text (hunt…)"]);
        assert!(secrets_in_line("password: changeme", true).is_empty());
        assert!(secrets_in_line("password: ${DB_PASS}", true).is_empty());
        assert!(secrets_in_line("password: hunter2secret", false).is_empty());
        let hook = format!("url: https://discord.com/api/webhooks/{}/{}", "1".repeat(18), "aB3-".repeat(8));
        assert_eq!(secrets_in_line(&hook, true), vec!["Discord webhook (http…)"]);
        let jdbc = ["jdbc:mysql://db:3306/x?user=a&pass", "word=hunter2secret"].concat();
        assert_eq!(secrets_in_line(&jdbc, false), vec!["password in a database URL (jdbc…)"]);
        assert!(config_file("plugins/x/config.yml") && config_file(".env.local") && !config_file("Main.java"));
        assert!(key_file("keys/server.pem") && key_file("id_rsa") && !key_file("monkey.java"));
    }
}
