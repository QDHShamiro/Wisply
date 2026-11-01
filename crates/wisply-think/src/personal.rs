use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;

use crate::files::{self, Pack};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Ban(String, Vec<String>),
    NoComments,
    FileLines(usize),
    Denial(String),
    LoreHeader(String),
    Filler(String),
    OneGlow,
    TitleColor(String),
    SmallCapsNames,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    pub kind: Kind,
    pub scope: BTreeSet<Pack>,
    pub source: String,
    pub text: String,
}

pub(crate) fn re(p: &str) -> Regex {
    Regex::new(p).expect("personal rule pattern")
}

static NO_COMMENTS: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?i)\b(?:zero|no|keine?|null)\s+(?:code\s+)?(?:comments?|kommentare)\b"));
static FILE_LINES: LazyLock<Regex> = LazyLock::new(|| {
    re(
        r"(?i)files?\s+(?:under|below|unter)\s+(\d{2,4})\s+(?:lines|zeilen)|max(?:imal|imum)?\.?\s+(\d{2,4})\s+(?:lines|zeilen)\s+(?:per|pro|je)\s+(?:file|datei)",
    )
});
static DENIAL: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?i)(?:denial|verbots?|deny|no-permission)[\w\s-]*?(?:messages?|nachrichten?).*?(&#[0-9a-f]{6})")
});
static LORE: LazyLock<Regex> = LazyLock::new(|| re(r#"(?i)lore[^\n]*?["`](&[0-9a-fk-or][^"`]*?:)["`]"#));
static FILLER: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?i)(?:unused|empty|leere?n?|freie?n?)\s+slots?.*?\b([A-Z_]+_GLASS_PANE)\b"));
static GLOW: LazyLock<Regex> = LazyLock::new(|| re(r"(?i)max\.?\s*1\s*`?glow"));
static TITLE: LazyLock<Regex> = LazyLock::new(|| re(r"(?i)(?:gui[- ]?tit(?:el|le)s?|\btitel\b)[^\n]*?(&#[0-9a-f]{6})"));
static CAPS: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?i)item[- ]?names?[^\n]*small[- ]?caps|small[- ]?caps[^\n]*item[- ]?names?"));
static NEGATION: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?i)\b(?:never|no|nie|niemals|kein|keine|keinen|avoid|verboten|don't|do not)\b"));
static TICK: LazyLock<Regex> = LazyLock::new(|| re(r"`([^`]+)`"));
static GIT_HEAD: LazyLock<Regex> = LazyLock::new(|| re(r"(?i)\bgit(?:hub)?\b|\bcommits?\b|\bpull requests?\b"));

const BRIDGE: &[&str] = &["use", "call", "import", "write", "add", "using", "nutze", "nutzen", "verwende", "benutze"];
const STOPS: &[&str] =
    &["—", " – ", " - ", ";", "→", "->", " use ", " instead", " always", " statt", " immer", " nutze"];
const FILE_ENDS: &[&str] = &[".json", ".yml", ".yaml", ".md", ".env", ".toml", ".txt", ".exe", ".jar", ".xml"];

fn code_like(t: &str) -> bool {
    let camel = t.as_bytes().windows(2).any(|w| w[0].is_ascii_lowercase() && w[1].is_ascii_uppercase());
    t.len() >= 3
        && !t.contains([' ', '/', '\\'])
        && !t.starts_with(['-', '.', '<', '&', '~'])
        && !FILE_ENDS.iter().any(|e| t.to_lowercase().ends_with(e))
        && (camel || t.contains(['.', '(', '_', ':']))
}

fn instead(line: &str, banned: &[String]) -> Vec<String> {
    TICK.captures_iter(line)
        .map(|c| c[1].trim().to_string())
        .filter(|t| code_like(t) && !banned.contains(t))
        .map(|t| t.split(['.', '(']).next().unwrap_or_default().to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

fn bans(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    for neg in NEGATION.find_iter(line) {
        let mut rest = &line[neg.end()..];
        if let Some(cut) = STOPS.iter().filter_map(|s| rest.find(s)).min() {
            rest = &rest[..cut];
        }
        let mut gap = rest.find('`').map_or(rest, |i| &rest[..i]).split_whitespace();
        let first_ok = match (gap.next(), gap.next()) {
            (None, _) => true,
            (Some(w), None) => BRIDGE.contains(&w.to_lowercase().as_str()),
            _ => false,
        };
        if !first_ok {
            continue;
        }
        for c in TICK.captures_iter(rest) {
            let t = c[1].trim();
            if code_like(t) && !out.iter().any(|o| o == t) {
                out.push(t.to_string());
            }
        }
    }
    out
}

pub fn scope_of(text: &str) -> BTreeSet<Pack> {
    let t = text.to_lowercase();
    let mut out = BTreeSet::new();
    let has = |words: &[&str]| words.iter().any(|w| t.contains(w));
    if has(&["minecraft", "plugin", "paper", "folia", "bukkit", "spigot"]) {
        out.insert(Pack::JavaPlugin);
        out.insert(Pack::McGui);
    }
    if has(&["website", "web ", "html", "css", "frontend"]) {
        out.insert(Pack::Web);
    }
    if has(&["rust", "cargo"]) {
        out.insert(Pack::Rust);
    }
    if has(&["python"]) {
        out.insert(Pack::Python);
    }
    out
}

fn kinds(line: &str) -> Vec<Kind> {
    let mut out = Vec::new();
    if NO_COMMENTS.is_match(line) {
        out.push(Kind::NoComments);
    }
    if let Some(c) = FILE_LINES.captures(line)
        && let Some(n) = c.get(1).or(c.get(2)).and_then(|m| m.as_str().parse().ok())
    {
        out.push(Kind::FileLines(n));
    }
    if let Some(c) = DENIAL.captures(line) {
        out.push(Kind::Denial(c[1].to_uppercase()));
    }
    if let Some(c) = LORE.captures(line) {
        out.push(Kind::LoreHeader(c[1].to_string()));
    }
    if let Some(c) = FILLER.captures(line) {
        out.push(Kind::Filler(c[1].to_string()));
    }
    if GLOW.is_match(line) {
        out.push(Kind::OneGlow);
    }
    if let Some(c) = TITLE.captures(line) {
        out.push(Kind::TitleColor(c[1].to_uppercase()));
    }
    if CAPS.is_match(line) {
        out.push(Kind::SmallCapsNames);
    }
    let banned = bans(line);
    let allow = instead(line, &banned);
    out.extend(banned.into_iter().map(|t| Kind::Ban(t, allow.clone())));
    out
}

fn same(a: &Kind, b: &Kind) -> bool {
    match (a, b) {
        (Kind::Ban(x, _), Kind::Ban(y, _)) => x == y,
        _ => a == b,
    }
}

pub fn parse(text: &str, source: &str, fallback: &BTreeSet<Pack>) -> Vec<Rule> {
    let body = text.strip_prefix("---").and_then(|r| r.split_once("\n---").map(|(_, b)| b)).unwrap_or(text);
    let whole = scope_of(text);
    let mut heads: Vec<(usize, String)> = Vec::new();
    let mut code = false;
    let mut out: Vec<Rule> = Vec::new();
    let skip = body.len() < text.len();
    let offset = text[..text.len() - body.len()].matches('\n').count();
    for (i, line) in body.lines().enumerate() {
        let t = line.trim_start();
        if t.starts_with("```") {
            code = !code;
            continue;
        }
        if code {
            continue;
        }
        let level = t.chars().take_while(|c| *c == '#').count();
        if level > 0 && t[level..].starts_with(' ') {
            heads.retain(|(l, _)| *l < level);
            heads.push((level, t[level..].trim().to_string()));
            continue;
        }
        let found = kinds(line);
        if found.is_empty() {
            continue;
        }
        let chain: String = heads.iter().map(|(_, h)| h.as_str()).collect::<Vec<_>>().join(" ");
        let mut scope = scope_of(&format!("{chain} {line}"));
        if scope.is_empty() {
            scope = if skip { whole.clone() } else { fallback.clone() };
        }
        for kind in found {
            if matches!(kind, Kind::Ban(..)) && (scope.is_empty() || GIT_HEAD.is_match(&chain)) {
                continue;
            }
            if out.iter().any(|r| same(&r.kind, &kind)) {
                continue;
            }
            out.push(Rule {
                kind,
                scope: scope.clone(),
                source: format!("{source}:{}", i + 1 + offset),
                text: t.trim().to_string(),
            });
        }
    }
    out
}

fn encoded(p: &Path) -> String {
    let s: String = p.to_string_lossy().chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '-' }).collect();
    s.trim_end_matches('-').to_lowercase()
}

pub fn sources(user: &Path, root: &Path) -> Vec<PathBuf> {
    let mut out = vec![user.join(".claude").join("CLAUDE.md")];
    for d in root.ancestors() {
        out.push(d.join("CLAUDE.md"));
        out.push(d.join(".claude").join("CLAUDE.md"));
        if d == user {
            break;
        }
    }
    let mine: BTreeSet<String> = std::iter::once(user).chain(root.ancestors()).map(encoded).collect();
    if let Ok(list) = fs::read_dir(user.join(".claude").join("projects")) {
        for p in list.flatten().filter(|p| mine.contains(&encoded(Path::new(&p.file_name())))) {
            if let Ok(mem) = fs::read_dir(p.path().join("memory")) {
                out.extend(
                    mem.flatten()
                        .map(|e| e.path())
                        .filter(|f| f.file_name().is_some_and(|n| n.to_string_lossy().starts_with("feedback_"))),
                );
            }
        }
    }
    let mut seen = BTreeSet::new();
    out.retain(|p| p.is_file() && seen.insert(p.clone()));
    out
}

pub fn load(user: &Path, root: &Path, fallback: &BTreeSet<Pack>) -> Vec<Rule> {
    let mut out: Vec<Rule> = Vec::new();
    for f in sources(user, root) {
        let Ok(text) = fs::read_to_string(&f) else { continue };
        let name = files::slashed(f.strip_prefix(user).unwrap_or(&f));
        for r in parse(&text, &format!("~/{name}"), fallback) {
            if !out.iter().any(|o| same(&o.kind, &r.kind)) {
                out.push(r);
            }
        }
    }
    out
}

pub fn describe(r: &Rule) -> String {
    let what = match &r.kind {
        Kind::Ban(t, allow) if allow.is_empty() => format!("never `{t}`"),
        Kind::Ban(t, allow) => format!("never `{t}` (except in {})", allow.join(", ")),
        Kind::NoComments => "no comments".into(),
        Kind::FileLines(n) => format!("files under {n} lines"),
        Kind::Denial(c) => format!("denial messages fully {c}"),
        Kind::LoreHeader(h) => format!("lore starts with {h}"),
        Kind::Filler(m) => format!("empty slots = {m}"),
        Kind::OneGlow => "max 1 glow per GUI".into(),
        Kind::TitleColor(c) => format!("GUI titles {c}, no other colors"),
        Kind::SmallCapsNames => "item names in small caps".into(),
    };
    let scope: Vec<&str> = r.scope.iter().map(|p| p.name()).collect();
    let scope = if scope.is_empty() { "all".to_string() } else { scope.join(",") };
    format!("{what} [{scope}] <- {}", r.source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::yours::check;

    const RULES: &str = "# Minecraft Plugin — Shamiro\n\n## Absolute Code Rules\n1. Zero comments (no `//`, `/* */`, JavaDoc, TODO).\n\
        3. `FoliaScheduler` always — never `Bukkit.getScheduler()`.\n5. `ConcurrentHashMap` for shared state — never `HashMap`.\n\
        6. No `System.out` — use `DebugLogger` gated by `debug: false`.\n2. Never regenerate boilerplate (`FoliaScheduler`, `ColorUtil`) — copy.\n\
        - Never commit or echo creds from `servers.json`.\n\n## Design Constraints\n- Denial messages fully `&#FF0000` — no mid-line color change.\n\
        - GUI titles + item names in Unicode Small Caps (ᴀʙᴄ).\n- Unused slots = `GRAY_STAINED_GLASS_PANE` + `name: \" \"`; max 1 `glow: true` per layer.\n\
        - Every lore has `\"&8Description:\"` header + CTA line.\n";

    #[test]
    fn rules_come_out_of_claude_md() {
        let rules = parse(RULES, "~/.claude/CLAUDE.md", &BTreeSet::new());
        let kinds: Vec<&Kind> = rules.iter().map(|r| &r.kind).collect();
        assert_eq!(
            kinds,
            [
                &Kind::NoComments,
                &Kind::Ban("Bukkit.getScheduler()".into(), vec!["FoliaScheduler".into()]),
                &Kind::Ban("HashMap".into(), vec!["ConcurrentHashMap".into()]),
                &Kind::Ban("System.out".into(), vec!["DebugLogger".into()]),
                &Kind::Denial("&#FF0000".into()),
                &Kind::SmallCapsNames,
                &Kind::Filler("GRAY_STAINED_GLASS_PANE".into()),
                &Kind::OneGlow,
                &Kind::LoreHeader("&8Description:".into()),
            ]
        );
        assert!(rules.iter().all(|r| r.scope.contains(&Pack::JavaPlugin)));
        assert_eq!(rules[1].source, "~/.claude/CLAUDE.md:5");
        let memory =
            "---\nname: gui\n---\n\nGUI-Titel nur &#444444 + normaler Text, keine Small Caps im Minecraft-Plugin.\n";
        let m = parse(memory, "m.md", &BTreeSet::new());
        assert_eq!(m[0].kind, Kind::TitleColor("&#444444".into()));
        assert_eq!(m[0].source, "m.md:5");
    }

    #[test]
    fn git_rules_and_other_projects_stay_out() {
        let git = "# Minecraft Plugin\n## Git Workflow\n- No `Claude-Session:` lines, never `Git.push()`.\n";
        assert!(parse(git, "c.md", &BTreeSet::new()).is_empty());
        let base = std::env::temp_dir().join(format!("wisply-think-mem-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let user = base.join("user");
        let proj = base.join("work").join("proj");
        for (owner, token) in [(&user, "homeBan()"), (&proj, "projBan()"), (&base.join("other"), "otherBan()")] {
            let dir = user.join(".claude").join("projects").join(encoded(owner)).join("memory");
            fs::create_dir_all(&dir).unwrap();
            fs::write(dir.join("feedback_x.md"), format!("Minecraft plugin: never `{token}`.\n")).unwrap();
        }
        let bans = |root: &Path| {
            let mut out: Vec<String> = load(&user, root, &BTreeSet::new())
                .into_iter()
                .filter_map(|r| if let Kind::Ban(t, _) = r.kind { Some(t) } else { None })
                .filter(|t| t.ends_with("Ban()"))
                .collect();
            out.sort();
            out
        };
        assert_eq!(bans(&proj), ["homeBan()", "projBan()"]);
        assert_eq!(bans(&proj.join("sub")), ["homeBan()", "projBan()"]);
        assert_eq!(bans(&base.join("work")), ["homeBan()"]);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn your_rules_are_checked_in_the_right_files() {
        let rules = parse(RULES, "c.md", &BTreeSet::new());
        let packs: BTreeSet<Pack> = [Pack::Code, Pack::JavaPlugin, Pack::McGui].into_iter().collect();
        let java = "class A {\n  // hi\n  Map<A, B> m = new ConcurrentHashMap<>();\n  Map<A, B> n = new HashMap<>();\n  Object copy() { return new HashMap<>(n); }\n}\n";
        let got: Vec<String> =
            check(&rules, Path::new("src/A.java"), "java", java, false, &packs).0.into_iter().map(|f| f.why).collect();
        assert_eq!(got, ["1 comment lines", "`HashMap` used 1x"]);
        let wrapper = "class FoliaScheduler {\n  void run() { Bukkit.getScheduler().runTask(p, r); }\n}\n";
        let (got, waived) = check(&rules, Path::new("src/FoliaScheduler.java"), "java", wrapper, false, &packs);
        assert!(got.is_empty() && waived.contains("bukkit-scheduler"), "{got:?} {waived:?}");
        let other = "class Task {\n  void run() { Bukkit.getScheduler().runTask(p, r); }\n}\n";
        let (got, waived) = check(&rules, Path::new("src/Task.java"), "java", other, false, &packs);
        assert!(got.is_empty() && waived.is_empty(), "{got:?} {waived:?}");
        let rust_only: BTreeSet<Pack> = [Pack::Code, Pack::Rust].into_iter().collect();
        assert!(
            check(&rules, Path::new("src/a.rs"), "rs", "// x\nlet m = HashMap::new();\n", false, &rust_only)
                .0
                .is_empty()
        );
        let gui = "title: '&#444444Shop'\nitems:\n  a:\n    material: STONE\n    name: '&fStone'\n    glow: true\n    lore:\n      - '&7Cheap'\n  b:\n    glow: true\n";
        let got: Vec<String> = check(&rules, Path::new("p/gui/shop.yml"), "yml", gui, false, &packs)
            .0
            .into_iter()
            .map(|f| f.why)
            .collect();
        assert_eq!(
            got,
            [
                "1 item names are not in small caps",
                "no GRAY_STAINED_GLASS_PANE filler for empty slots",
                "2 items glow",
                "1 lores do not start with &8Description:"
            ]
        );
        let msgs = "no-permission: '&#FF0000You can not do that.'\nnot-enough-money: '&#FF0000You need &#FFAA00$5'\nwelcome: '&aHi'\n";
        let got: Vec<String> = check(&rules, Path::new("p/messages.yml"), "yml", msgs, false, &packs)
            .0
            .into_iter()
            .map(|f| f.why)
            .collect();
        assert_eq!(got, ["1 denial messages are not fully &#FF0000"]);
    }
}
