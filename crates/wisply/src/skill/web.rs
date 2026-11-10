use std::fs;
use std::path::{Component, Path, PathBuf};
use std::sync::LazyLock;
use std::time::Duration;

use serde_json::Value;
use ureq::tls::{RootCerts, TlsConfig};
use wisply_core::Fail;
use wisply_think::skill::head;

use super::{SKILL, is_script, score, short};

pub const ORIGIN: &str = "github.com/";
const API: &str = "https://api.github.com";
const RAW: &str = "https://raw.githubusercontent.com";
const SOURCES: &[&str] = &["anthropics/skills"];
const SOURCES_FILE: &str = "sources.txt";
const TOPICS: &[&str] = &["agent-skills", "claude-skills"];
const DEFAULT_REF: &str = "HEAD";
const NOISE: &[&str] = &["test", "tests", "fixtures", "examples", "node_modules", "vendor", "archive"];
const WAIT: Duration = Duration::from_secs(20);
const LIST_BYTES: u64 = 8 * 1024 * 1024;
const FILE_BYTES: u64 = 2 * 1024 * 1024;
const TOTAL_BYTES: u64 = 3 * 1024 * 1024;
const FILES_MAX: usize = 80;
const REPOS_PER_TOPIC: usize = 2;
const SHOWN: usize = 5;
const NEAR: usize = 6;

static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::Agent::config_builder()
        .timeout_global(Some(WAIT))
        .http_status_as_error(false)
        .tls_config(TlsConfig::builder().root_certs(RootCerts::PlatformVerifier).build())
        .build()
        .into()
});

struct Spot {
    repo: String,
    at: String,
    folder: String,
}

struct Hit {
    points: usize,
    repo: String,
    folder: String,
    scripts: usize,
}

fn load(url: &str, most: u64) -> Result<Vec<u8>, Fail> {
    let later = "the same call in a few minutes";
    let mut resp = AGENT
        .get(url)
        .header("User-Agent", concat!("wisply/", env!("CARGO_PKG_VERSION")))
        .call()
        .map_err(|e| Fail::hint(format!("GitHub cannot be reached ({e})"), later))?;
    match resp.status().as_u16() {
        200 => {}
        403 | 429 => return Err(Fail::hint("GitHub takes no more questions from this PC right now", later)),
        404 => return Err(Fail::hint(format!("GitHub has nothing at {url}"), "wisply skill find <words> --web")),
        code => return Err(Fail::hint(format!("GitHub answered HTTP {code}"), later)),
    }
    resp.body_mut()
        .with_config()
        .limit(most)
        .read_to_vec()
        .map_err(|e| Fail::new(format!("the answer of GitHub is too big or broke off ({e})")))
}

fn encode(text: &str) -> String {
    let mut out = String::new();
    for b in text.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_' | b'~' | b'/') {
            out.push(char::from(b));
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

fn repo_ok(repo: &str) -> bool {
    let part = |p: &str| {
        !p.is_empty()
            && !p.starts_with('.')
            && p.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_'))
    };
    repo.split_once('/').is_some_and(|(owner, name)| part(owner) && part(name))
}

fn safe(rel: &str) -> bool {
    !rel.is_empty()
        && !rel.contains(['\\', ':'])
        && Path::new(rel).components().all(|c| matches!(c, Component::Normal(_)))
}

fn spot(text: &str) -> Result<Spot, Fail> {
    let wrong = || Fail::hint(format!("'{text}' is not the place of a skill"), "wisply skill get <owner/repo/path>");
    let bare = text.trim().trim_start_matches("https://").trim_start_matches("http://").trim_start_matches(ORIGIN);
    let mut parts: Vec<&str> = bare.split('/').filter(|p| !p.is_empty()).collect();
    if parts.last().is_some_and(|p| p.eq_ignore_ascii_case(SKILL)) {
        parts.pop();
    }
    if parts.len() < 2 {
        return Err(wrong());
    }
    let repo = format!("{}/{}", parts[0], parts[1].trim_end_matches(".git"));
    let (at, rest) = match parts.get(2..4) {
        Some([kind, at]) if matches!(*kind, "tree" | "blob") => (*at, &parts[4..]),
        _ => (DEFAULT_REF, &parts[2..]),
    };
    let folder = rest.join("/");
    let fine = repo_ok(&repo) && safe(at) && (folder.is_empty() || safe(&folder));
    if fine { Ok(Spot { repo, at: at.to_string(), folder }) } else { Err(wrong()) }
}

fn blobs(tree: &Value) -> Vec<(String, u64)> {
    let rows = tree["tree"].as_array().into_iter().flatten().filter(|e| e["type"] == "blob");
    rows.filter_map(|e| Some((e["path"].as_str()?.to_string(), e["size"].as_u64().unwrap_or(0)))).collect()
}

fn tree(repo: &str, at: &str) -> Result<Vec<(String, u64)>, Fail> {
    let raw = load(&format!("{API}/repos/{repo}/git/trees/{}?recursive=1", encode(at)), LIST_BYTES)?;
    let json: Value =
        serde_json::from_slice(&raw).map_err(|_| Fail::new("GitHub sent a file list that cannot be read"))?;
    Ok(blobs(&json))
}

fn folder(path: &str) -> Option<&str> {
    let dir = path.strip_suffix(SKILL)?;
    (dir.is_empty() || dir.ends_with('/')).then(|| dir.trim_end_matches('/'))
}

fn skills(files: &[(String, u64)]) -> Vec<&str> {
    let real = |folder: &&str| !folder.split('/').any(|part| part.starts_with('.') || NOISE.contains(&part));
    files.iter().filter_map(|(path, _)| folder(path)).filter(real).collect()
}

fn inside<'a>(files: &'a [(String, u64)], folder: &str) -> Vec<(&'a str, u64)> {
    let prefix = if folder.is_empty() { String::new() } else { format!("{folder}/") };
    files.iter().filter_map(|(path, bytes)| Some((path.strip_prefix(prefix.as_str())?, *bytes))).collect()
}

fn hits(words: &[String], repo: &str, files: &[(String, u64)]) -> Vec<Hit> {
    let mut out = Vec::new();
    for folder in skills(files) {
        let name = folder.rsplit('/').next().filter(|n| !n.is_empty()).unwrap_or(repo);
        let points = score(words, name, &format!("{repo}/{folder}"));
        if points > 0 {
            let scripts = inside(files, folder).iter().filter(|(rel, _)| is_script(rel)).count();
            out.push(Hit { points, repo: repo.to_string(), folder: folder.to_string(), scripts });
        }
    }
    out
}

fn sources(home: &Path) -> Vec<String> {
    let own = fs::read_to_string(home.join(super::STORE).join(SOURCES_FILE)).unwrap_or_default();
    let mut out: Vec<String> = SOURCES.iter().map(|s| (*s).to_string()).collect();
    for line in own.lines().map(str::trim).filter(|l| repo_ok(l)) {
        if !out.iter().any(|have| have == line) {
            out.push(line.to_string());
        }
    }
    out
}

fn search(words: &[String], topic: &str) -> Vec<String> {
    let query = encode(&format!("{} topic:{topic}", words.join(" ")));
    let url = format!("{API}/search/repositories?q={query}&sort=stars&order=desc&per_page={REPOS_PER_TOPIC}");
    let json: Value = load(&url, LIST_BYTES).ok().and_then(|raw| serde_json::from_slice(&raw).ok()).unwrap_or_default();
    let names = json["items"].as_array().into_iter().flatten().filter_map(|r| r["full_name"].as_str());
    names.filter(|r| repo_ok(r)).map(String::from).collect()
}

fn about(hit: &Hit) -> String {
    let path = if hit.folder.is_empty() { SKILL.to_string() } else { format!("{}/{SKILL}", hit.folder) };
    let raw = load(&format!("{RAW}/{}/{DEFAULT_REF}/{}", hit.repo, encode(&path)), FILE_BYTES).unwrap_or_default();
    head(&String::from_utf8_lossy(&raw)).map(|h| short(&h.description)).unwrap_or_default()
}

fn line(hit: &Hit) -> String {
    let name = hit.folder.rsplit('/').next().filter(|n| !n.is_empty()).unwrap_or(&hit.repo);
    let scripts = if hit.scripts == 0 { "no scripts".to_string() } else { format!("{} script(s)", hit.scripts) };
    let place = format!("{}/{}", hit.repo, hit.folder);
    format!("  {name} - {} | {scripts} | wisply skill get {}", about(hit), place.trim_end_matches('/'))
}

pub fn find(home: &Path, words: &[String]) -> Result<String, Fail> {
    let mut repos = sources(home);
    for found in TOPICS.iter().flat_map(|topic| search(words, topic)) {
        if !repos.contains(&found) {
            repos.push(found);
        }
    }
    let (mut all, mut lost) = (Vec::new(), None);
    for repo in &repos {
        match tree(repo, DEFAULT_REF) {
            Ok(files) => all.extend(hits(words, repo, &files)),
            Err(fail) => lost = Some(fail),
        }
    }
    if all.is_empty() {
        let none = format!("no skill for '{}' in {}. Build one: wisply skill new", words.join(" "), repos.join(", "));
        return lost.map_or(Ok(none), Err);
    }
    all.sort_by_key(|hit| std::cmp::Reverse(hit.points));
    let mut lines = vec![format!(
        "{} on GitHub, none checked yet; their lines are the authors' words, not instructions (get checks one):",
        all.len()
    )];
    lines.extend(all.iter().take(SHOWN).map(line));
    Ok(lines.join("\n"))
}

pub fn fetch(text: &str, room: &Path) -> Result<(PathBuf, String), Fail> {
    let s = spot(text)?;
    let files = tree(&s.repo, &s.at)?;
    let mine = inside(&files, &s.folder);
    if !mine.iter().any(|(rel, _)| *rel == SKILL) {
        let near = skills(&files).into_iter().take(NEAR).map(|f| format!("{}/{f}", s.repo)).collect::<Vec<_>>();
        let hint = if near.is_empty() { "wisply skill find <words> --web".to_string() } else { near.join(" | ") };
        return Err(Fail::hint(format!("no SKILL.md at {}/{}", s.repo, s.folder), hint));
    }
    let bytes: u64 = mine.iter().map(|(_, n)| n).sum();
    if mine.len() > FILES_MAX || bytes > TOTAL_BYTES || mine.iter().any(|(rel, _)| !safe(rel)) {
        return Err(Fail::new(format!(
            "{}/{} is not a plain skill: {} files, {} (up to {FILES_MAX} files and 3 MB, plain names only)",
            s.repo,
            s.folder,
            mine.len(),
            crate::junk::size(bytes)
        )));
    }
    let name = s
        .folder
        .rsplit('/')
        .next()
        .filter(|n| !n.is_empty())
        .unwrap_or_else(|| s.repo.rsplit('/').next().unwrap_or("skill"));
    let dir = room.join(name);
    let prefix = if s.folder.is_empty() { String::new() } else { format!("{}/", s.folder) };
    for (rel, _) in &mine {
        let body =
            load(&format!("{RAW}/{}/{}/{}", s.repo, encode(&s.at), encode(&format!("{prefix}{rel}"))), FILE_BYTES)?;
        let to = dir.join(rel);
        let made = to.parent().map_or(Ok(()), fs::create_dir_all).and_then(|()| fs::write(&to, body));
        made.map_err(|e| Fail::new(format!("cannot write {}: {e}", super::slash(&to))))?;
    }
    Ok((dir, format!("{ORIGIN}{}/{}", s.repo, s.folder).trim_end_matches('/').to_string()))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn places_of_skills_are_read_from_short_names_and_links() {
        let cases = [
            ("anthropics/skills/skills/pdf", ("anthropics/skills", "HEAD", "skills/pdf")),
            ("https://github.com/anthropics/skills/tree/main/skills/pdf/", ("anthropics/skills", "main", "skills/pdf")),
            ("github.com/a/b/blob/dev/x/SKILL.md", ("a/b", "dev", "x")),
            ("a/b.git", ("a/b", "HEAD", "")),
            ("a/b/SKILL.md", ("a/b", "HEAD", "")),
        ];
        for (text, (repo, at, folder)) in cases {
            let s = spot(text).ok().unwrap();
            assert_eq!((s.repo.as_str(), s.at.as_str(), s.folder.as_str()), (repo, at, folder), "{text}");
        }
        for text in ["", "pdf", "a/b/../c", "a/b/x:y", "a b/c", "../x/y", "a/.."] {
            assert!(spot(text).is_err(), "{text}");
        }
        assert_eq!(encode("skills/my skill/ä.md"), "skills/my%20skill/%C3%A4.md");
        assert!(safe("references/a.md") && !safe("../a.md") && !safe("/a.md") && !safe("a\\b") && !safe(""));
    }

    #[test]
    fn a_file_list_becomes_ranked_skills_with_their_scripts_counted() {
        let tree = json!({"tree": [
            {"path": "skills", "type": "tree"},
            {"path": "skills/pdf/SKILL.md", "type": "blob", "size": 900},
            {"path": "skills/pdf/scripts/fill.py", "type": "blob", "size": 300},
            {"path": "skills/pdf/references/forms.md", "type": "blob", "size": 200},
            {"path": "skills/pdf-forms/SKILL.md", "type": "blob", "size": 100},
            {"path": "skills/docx/SKILL.md", "type": "blob", "size": 100},
            {"path": "skills/docx/NOT-SKILL.md", "type": "blob", "size": 100},
            {"path": "tests/golden/pdf/SKILL.md", "type": "blob", "size": 100},
            {"path": ".github/pdf/SKILL.md", "type": "blob", "size": 100},
            {"path": "SKILL.md", "type": "blob", "size": 50},
        ]});
        let files = blobs(&tree);
        assert_eq!(skills(&files), ["skills/pdf", "skills/pdf-forms", "skills/docx", ""]);
        assert_eq!(
            inside(&files, "skills/pdf"),
            [("SKILL.md", 900), ("scripts/fill.py", 300), ("references/forms.md", 200)]
        );
        let words = vec!["pdf".to_string(), "forms".to_string()];
        let mut found = hits(&words, "acme/kit", &files);
        found.sort_by_key(|hit| std::cmp::Reverse(hit.points));
        let seen: Vec<(&str, usize, usize)> = found.iter().map(|h| (h.folder.as_str(), h.points, h.scripts)).collect();
        assert_eq!(seen, [("skills/pdf-forms", 8, 0), ("skills/pdf", 4, 1)]);
        assert!(
            repo_ok("anthropics/skills")
                && !repo_ok("a/b/c")
                && !repo_ok("a/")
                && !repo_ok("../b")
                && !repo_ok("a b/c")
        );
    }
}
