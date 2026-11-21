use std::collections::HashSet;
use std::sync::LazyLock;

use regex::Regex;

use super::expand::variants;
use super::store::{Hit, Note};
use super::text::{oneline, similar, stop_word};

const MIN_TERM: usize = 2;
const BODY_WEIGHT: f64 = 0.6;
const COVER_FLOOR: f64 = 0.3;
const FOLD_SIM: f64 = 0.6;
const SEMANTIC_SURE: f32 = 0.55;
const EXTRA_STOP: &[&str] = &[
    "fuer",
    "sein",
    "sind",
    "haben",
    "waren",
    "denn",
    "weil",
    "ob",
    "wieviel",
    "wieviele",
    "gibt",
    "eigentlich",
    "irgendwie",
    "damals",
    "nochmal",
    "beim",
    "ueber",
    "ohne",
    "kein",
    "keine",
    "nicht",
    "sondern",
    "unter",
];
const VARIANT_WEIGHT: f64 = 0.85;

pub type Group = Vec<String>;

pub fn terms(query: &str) -> Vec<String> {
    let lower = query.to_lowercase();
    let mut seen = HashSet::new();
    let all: Vec<String> = lower
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.chars().count() >= MIN_TERM && seen.insert(t.to_string()))
        .map(String::from)
        .collect();
    let kept: Vec<String> =
        all.iter().filter(|t| !stop_word(t) && !EXTRA_STOP.contains(&t.as_str())).cloned().collect();
    if kept.is_empty() { all } else { kept }
}

fn has(hay: &str, term: &str) -> bool {
    hay.split(|c: char| !c.is_alphanumeric()).any(|w| w.starts_with(term))
}

pub fn groups(terms: &[String]) -> Vec<Group> {
    terms
        .iter()
        .map(|t| {
            let mut group = vec![t.clone()];
            group.extend(variants(t).into_iter().map(String::from));
            group
        })
        .collect()
}

pub fn coverage(note: &Note, groups: &[Group], sim: Option<f32>) -> f64 {
    if sim.is_some_and(|s| s >= SEMANTIC_SURE) {
        return 1.0;
    }
    if groups.is_empty() {
        return 1.0;
    }
    let title = note.title.to_lowercase();
    let body = note.body.to_lowercase();
    let got: f64 = groups
        .iter()
        .map(|g| {
            g.iter()
                .enumerate()
                .map(|(i, t)| {
                    let k = if i == 0 { 1.0 } else { VARIANT_WEIGHT };
                    if has(&title, t) {
                        k
                    } else if has(&body, t) {
                        k * BODY_WEIGHT
                    } else {
                        0.0
                    }
                })
                .fold(0.0, f64::max)
        })
        .sum();
    got / groups.len() as f64
}

pub fn weight(cover: f64) -> f64 {
    COVER_FLOOR + (1.0 - COVER_FLOOR) * cover * cover
}

fn sentences(body: &str) -> Vec<&str> {
    let mut out = Vec::new();
    for row in body.lines() {
        let mut start = 0;
        let mut prev = ' ';
        for (i, ch) in row.char_indices() {
            if ch.is_whitespace() && matches!(prev, '.' | '!' | '?') {
                out.push(&row[start..i]);
                start = i;
            }
            prev = ch;
        }
        out.push(&row[start..]);
    }
    out
}

fn drop_title_echo<'a>(part: &'a str, base: &str) -> &'a str {
    let blen = base.chars().count();
    if blen < 12 {
        return part;
    }
    let mut end = 0;
    let mut matched = 0;
    for (bc, (i, c)) in base.chars().zip(part.char_indices()) {
        if !c.eq_ignore_ascii_case(&bc) {
            break;
        }
        end = i + c.len_utf8();
        matched += 1;
    }
    if matched * 5 < blen * 4 {
        return part;
    }
    let mut tail = &part[end..];
    if part[..end].chars().next_back().is_some_and(char::is_alphanumeric) && tail.starts_with(char::is_alphanumeric) {
        tail = tail.find(char::is_whitespace).map_or("", |s| &tail[s..]);
    }
    let tail = tail.trim_start_matches(['…', ':', '-', '>', ' ', ',', '.', ';', '*', '(', ')']).trim();
    if tail.chars().count() < 6 { "" } else { tail }
}

static WIKI: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[\[(?:[^\]|]*\|)?([^\]|]+)\]\]").expect("wiki link"));
static LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"!?\[([^\]]*)\]\([^)]*\)").expect("md link"));
static CODE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"`([^`]+)`").expect("code span"));
static BOLD: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\*\*([^*]+)\*\*").expect("bold span"));

fn strip_md(s: &str) -> String {
    let wiki = WIKI.replace_all(s, "${1}");
    let link = LINK.replace_all(&wiki, "${1}");
    let code = CODE.replace_all(&link, "${1}");
    BOLD.replace_all(&code, "${1}").into_owned()
}

pub fn snippet(body: &str, title: &str, terms: &[String], n: usize) -> String {
    let title = title.to_lowercase();
    let mut best: Option<(usize, &str)> = None;
    let mut first: Option<&str> = None;
    for part in sentences(body) {
        let part = part.trim();
        let probe = part.trim_start_matches(['#', '-', '*', '>', ' ']);
        if probe.chars().count() < 6 {
            continue;
        }
        let low = probe.to_lowercase();
        if title.contains(&low) {
            continue;
        }
        first.get_or_insert(part);
        let n_hit = terms.iter().filter(|t| has(&low, t)).count();
        if n_hit > 0 && best.is_none_or(|(b, _)| n_hit > b) {
            best = Some((n_hit, part));
        }
    }
    let base = title.trim_end_matches(['…', '.', ' ']);
    best.map(|(_, p)| p)
        .or(first)
        .map(|p| {
            let clean = strip_md(p);
            let clean = clean.trim_start_matches(['#', '-', '*', '>', ' ']);
            oneline(drop_title_echo(clean, base), n)
        })
        .unwrap_or_default()
}

pub fn fold(hits: Vec<Hit>, limit: usize) -> Vec<Hit> {
    let mut out: Vec<Hit> = Vec::new();
    for h in hits {
        if out.len() == limit {
            break;
        }
        let twin = out.iter().any(|k| {
            k.note.title == h.note.title
                || (k.note.kind == h.note.kind && similar(&k.note.title, &h.note.title) >= FOLD_SIM)
        });
        if !twin {
            out.push(h);
        }
    }
    out
}

pub fn words(query: &str) -> Vec<String> {
    let mut seen = HashSet::new();
    query
        .to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|t| t.chars().count() >= 3 && seen.insert(t.to_string()))
        .map(String::from)
        .collect()
}

pub fn mentions(text: &str, words: &[String]) -> bool {
    let low = text.to_lowercase();
    !words.is_empty() && words.iter().all(|w| has(&low, w))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(id: i64, title: &str, body: &str) -> Note {
        Note { id, title: title.into(), body: body.into(), kind: "fact".into(), ..Note::default() }
    }

    fn hit(id: i64, title: &str, score: f64) -> Hit {
        Hit { note: note(id, title, ""), score, sim: None }
    }

    #[test]
    fn coverage_counts_title_more_than_body() {
        let t = terms("Sniper Worktree");
        assert_eq!(t, ["sniper", "worktree"]);
        let g = groups(&t);
        let both = coverage(&note(1, "Sniper worktree plan", ""), &g, None);
        let one = coverage(&note(2, "Sniper plan", "worktrees"), &g, None);
        let none = coverage(&note(3, "other", ""), &g, None);
        assert!((both - 1.0).abs() < 1e-9 && one > none && one < both, "{both} {one} {none}");
        assert!(weight(both) > weight(one) && weight(one) > weight(none));
        assert!((coverage(&note(4, "x", ""), &g, Some(0.9)) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn snippet_strips_inline_markdown() {
        let body = "See the **ship** script via [[git-flow|flow]] and `cargo test` in [docs](http://x).";
        let t = terms("ship script");
        assert_eq!(snippet(body, "", &t, 120), "See the ship script via flow and cargo test in docs.");
    }

    #[test]
    fn snippet_strips_leading_bold_label() {
        let body = "**Why:** the ship script keeps logs tidy for review.";
        let t = terms("ship script");
        assert_eq!(snippet(body, "", &t, 120), "Why: the ship script keeps logs tidy for review.");
    }

    #[test]
    fn snippet_picks_the_sentence_with_the_words() {
        let body = "# Plan\nIntro line about nothing. The hopper moves 10 items per transfer. Last bit.";
        let t = terms("hopper items");
        assert_eq!(snippet(body, "", &t, 80), "The hopper moves 10 items per transfer.");
        assert_eq!(snippet(body, "", &terms("zzz"), 80), "Intro line about nothing.");
        assert_eq!(snippet("", "", &t, 80), "");
        assert_eq!(snippet("Same words. Extra detail here", "Same words.", &terms("words"), 80), "Extra detail here");
    }

    #[test]
    fn snippet_surfaces_the_sentence_that_carries_the_query() {
        let cases: &[(&str, &str, &str)] = &[
            (
                "The daemon forwards recall to a worker. Snippets are stripped of markdown before display.",
                "markdown display",
                "stripped of markdown",
            ),
            (
                "**Why:** the HDD spins down after twenty minutes. First access waits for spinup.",
                "spins minutes",
                "spins down after twenty",
            ),
            (
                "Config lives in the config file. Messages live in the messages file. Layouts live in the gui folder.",
                "layouts folder",
                "gui folder",
            ),
            (
                "Arena bounds are defined in FactionPVPCore. The yml files sit under the arenas directory.",
                "arenas directory",
                "arenas directory",
            ),
            (
                "Redis crashed on startup last week. The fix was to restart the worker after Redis boots.",
                "restart worker",
                "restart the worker",
            ),
            (
                "This plugin supports Folia on Paper. All disk and network IO runs async off the main thread.",
                "async network",
                "async off the main thread",
            ),
            (
                "The token hint shows an estimate. It only appears above the configured threshold value.",
                "threshold configured",
                "above the configured threshold",
            ),
            (
                "Overview of the deploy flow goes here. The deploy script runs after the CI pipeline turns green.",
                "deploy script pipeline",
                "deploy script runs after",
            ),
            (
                "Baltop caches offline heads. The head texture loads from the Mojang profile endpoint.",
                "texture profile",
                "texture loads from the Mojang profile",
            ),
            (
                "Intro sentence with no keywords at all. The scoreboard updates every server tick without flicker.",
                "scoreboard flicker",
                "scoreboard updates every server tick",
            ),
        ];
        for (body, query, want) in cases {
            let got = snippet(body, "", &terms(query), 200);
            assert!(got.contains(want), "query {query:?} -> snippet {got:?} is missing {want:?}");
        }
    }

    #[test]
    fn fold_drops_exact_and_near_duplicate_notes_within_kind() {
        let h = |id: i64, title: &str, kind: &str, score: f64| Hit {
            note: Note { id, title: title.into(), kind: kind.into(), ..Note::default() },
            score,
            sim: None,
        };
        let hits = vec![
            h(1, "redis crash fix", "fact", 1.0),
            h(2, "redis crash fix", "note", 0.9),
            h(3, "redis crash patch", "fact", 0.8),
            h(4, "redis crash fix now", "fact", 0.7),
            h(5, "redis crash fix now", "note", 0.6),
            h(6, "totally other subject", "fact", 0.5),
        ];
        let kept: Vec<i64> = fold(hits.clone(), 10).into_iter().map(|x| x.note.id).collect();
        assert_eq!(
            kept,
            [1, 3, 5, 6],
            "exact title folds across kinds, same-kind near-dup folds, kind-guarded near-dup and sub-threshold stay"
        );
        let capped: Vec<i64> = fold(hits, 2).into_iter().map(|x| x.note.id).collect();
        assert_eq!(capped, [1, 3], "limit caps the folded output");
    }

    #[test]
    fn recall_ranking_puts_the_on_topic_note_first() {
        let corpus = [
            note(1, "Sniper live build rules", "Build the sniper jar obfuscated then push it to the live folder."),
            note(
                2,
                "Redis crash on the fernhost server",
                "The async team worker crashed on startup; restart it once redis boots again.",
            ),
            note(3, "FactionPVP arena bounds", "Arena bounds live in FactionPVPCore under the arenas directory."),
            note(4, "Wisply recall snippet stripping", "Snippets strip markdown yet keep bold labels before display."),
            note(
                5,
                "cpvpgens economy coins",
                "Coin one, compressed value, heavy token as items; the heavy price is the item itself.",
            ),
            note(6, "Video edit pipeline watermark", "Edits follow the headbop style with a watermark on export."),
            note(
                7,
                "Bewerbung Ausbildung apprenticeship",
                "The apprenticeship application has a deadline at the district office.",
            ),
            note(8, "Discord friends mapping", "Franz maps to Froxotron and Lukas to the lukfighter handle."),
            note(9, "External disk spindown", "The external disk spins down after twenty minutes before spinup."),
            note(10, "ThumbnailEngine renderer", "Thumbnails use a procedural renderer with no image model at all."),
        ];
        let cases: &[(&str, i64)] = &[
            ("sniper obfuscated jar", 1),
            ("redis worker restart", 2),
            ("arena bounds factionpvpcore", 3),
            ("snippet markdown bold", 4),
            ("economy coins heavy", 5),
            ("watermark edit export", 6),
            ("apprenticeship deadline office", 7),
            ("disk spindown spinup", 9),
        ];
        for (query, want) in cases {
            let g = groups(&terms(query));
            let mut ranked: Vec<(i64, f64)> = corpus.iter().map(|n| (n.id, weight(coverage(n, &g, None)))).collect();
            ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).expect("finite scores"));
            assert_eq!(
                ranked[0].0, *want,
                "query {query:?} ranked {:?} first, want {want}; scores {ranked:?}",
                ranked[0].0
            );
            assert!(ranked[0].1 > ranked[1].1, "query {query:?} top tied: {ranked:?}");
        }
    }

    #[test]
    fn snippet_drops_truncated_title_echo() {
        let body = "cpvpgens premium kit blocker war falsch asynckits kits yml sind editierbar und dann folgt noch viel mehr neuer text als neue information";
        let title = "cpvpgens premium kit blocker war falsch…";
        let out = snippet(body, title, &terms("asynckits"), 56);
        assert!(!out.is_empty() && !out.starts_with("cpvpgens") && out.starts_with("asynckits"), "{out}");
        assert_eq!(drop_title_echo("Deploy target changed to X", "deploy target"), "changed to X");
        assert_eq!(drop_title_echo("short", "deploy target now here"), "short");
    }

    #[test]
    fn terms_keep_stop_words_only_when_nothing_else_is_left() {
        assert_eq!(terms("the and"), ["the", "and"]);
        assert_eq!(terms("wie deploye ich"), ["deploye"]);
    }

    #[test]
    fn near_twins_fold_into_the_better_hit() {
        let hits = vec![
            hit(1, "Headroom integration for Lighting", 0.9),
            hit(2, "Lighting: headroom integration", 0.8),
            hit(3, "Statusline cache", 0.7),
            hit(4, "Statusline cache", 0.6),
        ];
        let kept: Vec<i64> = fold(hits, 8).iter().map(|h| h.note.id).collect();
        assert_eq!(kept, [1, 3]);
    }
}
