use std::fs;
use std::path::Path;

use super::report::{Finding, Sev};

struct Need {
    entry: &'static str,
    probe: &'static str,
    when: &'static [&'static str],
    sev: Sev,
}

const NEEDS: &[Need] = &[
    Need { entry: ".env", probe: ".env", when: &[".env"], sev: Sev::Medium },
    Need { entry: "*.pem", probe: "server.pem", when: &[".pem"], sev: Sev::Medium },
    Need { entry: "*.key", probe: "server.key", when: &[".key"], sev: Sev::Medium },
    Need { entry: "*.jks", probe: "store.jks", when: &[".jks", ".keystore"], sev: Sev::Medium },
    Need { entry: "target/", probe: "target/x", when: &["pom.xml", "cargo.toml"], sev: Sev::Low },
    Need { entry: "build/", probe: "build/x", when: &["build.gradle", "build.gradle.kts"], sev: Sev::Low },
    Need { entry: "node_modules/", probe: "node_modules/x", when: &["package.json"], sev: Sev::Low },
    Need {
        entry: "__pycache__/",
        probe: "__pycache__/x",
        when: &["requirements.txt", "pyproject.toml"],
        sev: Sev::Low,
    },
    Need { entry: ".venv/", probe: ".venv/x", when: &["requirements.txt", "pyproject.toml"], sev: Sev::Low },
];

fn wild(pattern: &str, text: &str) -> bool {
    match pattern.split_once('*') {
        None => pattern == text,
        Some((head, tail)) => {
            text.len() >= head.len() + tail.len()
                && text.starts_with(head)
                && (tail.contains('*') && wild(tail, &text[head.len()..]) || text.ends_with(tail))
        }
    }
}

fn ignored(lines: &[String], path: &str) -> bool {
    let name = path.rsplit('/').next().unwrap_or(path);
    lines.iter().any(|raw| {
        let neg = raw.starts_with('!');
        let pat = raw.trim_start_matches('!').trim_start_matches('/');
        if neg || pat.is_empty() {
            return false;
        }
        match pat.strip_suffix('/') {
            Some(dir) => path.split('/').any(|part| wild(dir, part)) && path.contains('/'),
            None if pat.contains('/') => wild(pat, path),
            None => wild(pat, name) || path.split('/').any(|part| wild(pat, part)),
        }
    })
}

pub fn check(root: &Path, names: &[String]) -> Vec<Finding> {
    if !root.join(".git").exists() {
        return Vec::new();
    }
    let present = |marks: &[&str]| {
        names.iter().any(|n| {
            let low = n.to_lowercase();
            let file = low.rsplit('/').next().unwrap_or(&low).to_string();
            marks.iter().any(|m| {
                file == *m
                    || (m.starts_with('.') && m.len() > 1 && file.ends_with(m))
                    || file.starts_with(&format!("{m}."))
            })
        })
    };
    let Ok(text) = fs::read_to_string(root.join(".gitignore")) else {
        return vec![Finding::new(
            Sev::Medium,
            "hygiene",
            ".gitignore",
            0,
            "the project has no .gitignore",
            "create one with .env, key files and build folders so secrets and junk never get committed",
        )];
    };
    let lines: Vec<String> =
        text.lines().map(|l| l.trim().to_string()).filter(|l| !l.is_empty() && !l.starts_with('#')).collect();
    NEEDS
        .iter()
        .filter(|n| present(n.when) && !ignored(&lines, n.probe))
        .map(|n| {
            Finding::new(
                n.sev,
                "hygiene",
                ".gitignore",
                0,
                format!("{} is not ignored", n.entry),
                format!("add the line {} to .gitignore", n.entry),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignore_lines_match_like_git() {
        let lines: Vec<String> = [".env*", "target/", "*.pem", "/build"].iter().map(|s| s.to_string()).collect();
        assert!(ignored(&lines, ".env") && ignored(&lines, "target/x") && ignored(&lines, "a/b/server.pem"));
        assert!(ignored(&lines, "build/x") && !ignored(&lines, "server.key") && !ignored(&lines, "node_modules/x"));
    }

    #[test]
    fn missing_entries_are_named() {
        let dir = std::env::temp_dir().join(format!("wsp-hyg-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join(".git")).unwrap();
        fs::write(dir.join(".gitignore"), "target/\n").unwrap();
        let names: Vec<String> = [".env", "pom.xml", "keys/a.pem"].iter().map(|s| s.to_string()).collect();
        let why: Vec<String> = check(&dir, &names).into_iter().map(|f| f.why).collect();
        assert_eq!(why, vec![".env is not ignored", "*.pem is not ignored"]);
        fs::remove_file(dir.join(".gitignore")).unwrap();
        assert_eq!(check(&dir, &names).len(), 1);
        fs::remove_dir_all(&dir).unwrap();
    }
}
