use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use crate::files::{self, Pack};
use crate::personal::{Kind, Rule, re};
use crate::rules::{Finding, entries, smells_on, unquote};

static COLOR: LazyLock<Regex> = LazyLock::new(|| re(r"(?i)&#[0-9a-f]{6}|&[0-9a-fk-or]|<#[0-9a-f]{6}>"));
static PLACEHOLDER: LazyLock<Regex> = LazyLock::new(|| re(r"%[^%\s]+%|\{[^}\s]+\}|<[^>\s]+>"));
static DENIAL_KEY: LazyLock<Regex> = LazyLock::new(|| {
    re(r"(?i)no[-_]?perm|denied|not[-_]?enough|cannot|can-?not|invalid|error|fail|not[-_]?found|no[-_]?money")
});

fn applies(r: &Rule, packs: &BTreeSet<Pack>) -> bool {
    r.scope.is_empty() || r.scope.iter().any(|p| packs.contains(p))
}

fn comment_line(ext: &str, t: &str) -> bool {
    match ext {
        "py" | "sh" => t.starts_with('#') && !t.starts_with("#!"),
        "lua" => t.starts_with("--"),
        _ => t.starts_with("//") || t.starts_with("/*") || t.starts_with("* ") || t == "*" || t.starts_with("*/"),
    }
}

fn uses(line: &str, token: &str) -> bool {
    let word = |c: Option<char>| c.is_some_and(|c| c.is_alphanumeric() || c == '_');
    let (open, close) = (token.starts_with(char::is_alphanumeric), token.ends_with(char::is_alphanumeric));
    line.match_indices(token).any(|(i, _)| {
        (!open || !word(line[..i].chars().next_back())) && (!close || !word(line[i + token.len()..].chars().next()))
    })
}

fn import_line(l: &str) -> bool {
    let t = l.trim_start();
    ["import ", "use ", "from ", "#include"].iter().any(|p| t.starts_with(p))
}

fn first_and_count(hits: &[usize]) -> Option<(usize, usize)> {
    hits.first().map(|f| (*f, hits.len()))
}

fn yours(r: &Rule, line: usize, why: String) -> Finding {
    Finding::new("yours", Pack::Code, line, why, format!("your rule ({}): {}", r.source, cut(&r.text, 90)))
        .rate(2, 3, 1)
}

fn cut(t: &str, n: usize) -> String {
    if t.chars().count() <= n { t.to_string() } else { format!("{}…", t.chars().take(n - 1).collect::<String>()) }
}

fn visible(v: &str) -> String {
    PLACEHOLDER.replace_all(&COLOR.replace_all(unquote(v), ""), "").to_string()
}

fn gui_file(path: &Path) -> bool {
    let s = files::slashed(path).to_lowercase();
    files::has_ext(path, &["yml", "yaml"]) && (s.contains("/gui/") || s.contains("/guis/") || s.contains("/menus/"))
}

fn message_file(path: &Path) -> bool {
    let n = path.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
    files::has_ext(path, &["yml", "yaml"]) && (n.contains("messages") || n.starts_with("lang"))
}

fn lore_heads(text: &str) -> Vec<(usize, &str)> {
    let list = entries(text);
    let mut out = Vec::new();
    for (i, e) in list.iter().enumerate() {
        if e.key != "lore" {
            continue;
        }
        if e.value.starts_with('[') {
            let first = e.value.trim_start_matches('[').split(',').next().unwrap_or("");
            out.push((e.line, first));
        } else if let Some(next) = list.get(i + 1).filter(|n| n.key == "-" && n.indent >= e.indent) {
            out.push((next.line, next.value));
        }
    }
    out
}

pub fn check(
    rules: &[Rule],
    path: &Path,
    ext: &str,
    text: &str,
    test: bool,
    packs: &BTreeSet<Pack>,
) -> (Vec<Finding>, BTreeSet<&'static str>) {
    let mut out = Vec::new();
    let mut waived = BTreeSet::new();
    let code = files::has_ext(path, files::CODE_EXT);
    for r in rules.iter().filter(|r| applies(r, packs)) {
        match &r.kind {
            Kind::Ban(token, allow) if code && !test => {
                let hits: Vec<(usize, bool, Vec<&'static str>)> = text
                    .lines()
                    .enumerate()
                    .filter(|(_, l)| uses(l, token) && !comment_line(ext, l.trim_start()))
                    .map(|(i, l)| (i + 1, import_line(l), smells_on(ext, l, packs)))
                    .collect();
                let built_in: BTreeSet<&'static str> = hits.iter().flat_map(|h| h.2.iter().copied()).collect();
                if allow.iter().any(|a| path.file_stem() == Some(a.as_ref())) {
                    waived.extend(built_in);
                    continue;
                }
                let own: Vec<usize> =
                    hits.iter().filter(|h| built_in.is_empty() || !h.1 && h.2.is_empty()).map(|h| h.0).collect();
                if let Some((at, n)) = first_and_count(&own) {
                    out.push(yours(r, at, format!("`{token}` used {n}x")));
                }
            }
            Kind::NoComments if code && !test => {
                let hits: Vec<usize> = text
                    .lines()
                    .enumerate()
                    .filter(|(_, l)| comment_line(ext, l.trim_start()))
                    .map(|(i, _)| i + 1)
                    .collect();
                if let Some((at, n)) = first_and_count(&hits) {
                    out.push(yours(r, at, format!("{n} comment lines")));
                }
            }
            Kind::FileLines(n) if code => {
                let lines = text.lines().count();
                if lines > *n {
                    let mut f = yours(r, 0, format!("{lines} lines (over {n})"));
                    f.rule = "long-file".into();
                    out.push(f);
                }
            }
            Kind::Denial(color) if message_file(path) => {
                let bad: Vec<usize> = entries(text)
                    .iter()
                    .filter(|e| DENIAL_KEY.is_match(e.key) && !e.value.is_empty())
                    .filter(|e| {
                        let v = unquote(e.value);
                        let v =
                            PLACEHOLDER.find(v).filter(|m| m.start() == 0).map_or(v, |m| &v[m.end()..]).trim_start();
                        let colors: Vec<String> = COLOR.find_iter(v).map(|m| m.as_str().to_uppercase()).collect();
                        let other = colors.iter().any(|c| {
                            c.starts_with("&#") && c != color || c.len() == 2 && !"&L&O&N&M&K".contains(c.as_str())
                        });
                        !v.to_uppercase().starts_with(color.as_str()) || other
                    })
                    .map(|e| e.line)
                    .collect();
                if let Some((at, n)) = first_and_count(&bad) {
                    out.push(yours(r, at, format!("{n} denial messages are not fully {color}")));
                }
            }
            Kind::LoreHeader(head) if gui_file(path) => {
                let bad: Vec<usize> =
                    lore_heads(text).into_iter().filter(|(_, v)| !v.contains(head.as_str())).map(|(l, _)| l).collect();
                if let Some((at, n)) = first_and_count(&bad) {
                    out.push(yours(r, at, format!("{n} lores do not start with {head}")));
                }
            }
            Kind::Filler(mat) if gui_file(path) => {
                if text.contains("material:") && !text.contains(mat.as_str()) {
                    out.push(yours(r, 0, format!("no {mat} filler for empty slots")));
                }
            }
            Kind::OneGlow if gui_file(path) => {
                let glows: Vec<usize> = entries(text)
                    .iter()
                    .filter(|e| e.key == "glow" && unquote(e.value) == "true")
                    .map(|e| e.line)
                    .collect();
                if glows.len() > 1 {
                    out.push(yours(r, glows[1], format!("{} items glow", glows.len())));
                }
            }
            Kind::TitleColor(color) if gui_file(path) => {
                for e in entries(text).iter().filter(|e| e.key == "title") {
                    let v = unquote(e.value);
                    let colors: Vec<String> = COLOR.find_iter(v).map(|m| m.as_str().to_uppercase()).collect();
                    if !v.to_uppercase().starts_with(color.as_str()) || colors.iter().any(|c| c != color) {
                        out.push(yours(r, e.line, format!("title is not plain {color}")));
                        break;
                    }
                }
            }
            Kind::SmallCapsNames if gui_file(path) => {
                let bad: Vec<usize> = entries(text)
                    .iter()
                    .filter(|e| e.key == "name" && visible(e.value).trim().chars().any(|c| c.is_ascii_lowercase()))
                    .map(|e| e.line)
                    .collect();
                if let Some((at, n)) = first_and_count(&bad) {
                    out.push(yours(r, at, format!("{n} item names are not in small caps")));
                }
            }
            _ => {}
        }
    }
    (out, waived)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bans_match_whole_tokens() {
        assert!(uses("m = new HashMap<>()", "HashMap") && !uses("new ConcurrentHashMap<>()", "HashMap"));
        assert!(!uses("Map<A, B> my_HashMap2", "HashMap") && uses("x.tick();", "tick()"));
    }
}
