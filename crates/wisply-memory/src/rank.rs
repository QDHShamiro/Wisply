use std::collections::HashSet;

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

pub fn snippet(body: &str, title: &str, terms: &[String], n: usize) -> String {
    let title = title.to_lowercase();
    let mut best: Option<(usize, &str)> = None;
    let mut first: Option<&str> = None;
    for part in sentences(body) {
        let part = part.trim().trim_start_matches(['#', '-', '*', '>', ' ']);
        if part.chars().count() < 6 {
            continue;
        }
        let low = part.to_lowercase();
        if title.contains(&low) {
            continue;
        }
        first.get_or_insert(part);
        let n_hit = terms.iter().filter(|t| has(&low, t)).count();
        if n_hit > 0 && best.is_none_or(|(b, _)| n_hit > b) {
            best = Some((n_hit, part));
        }
    }
    best.map(|(_, p)| p).or(first).map(|p| oneline(p, n)).unwrap_or_default()
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
    fn snippet_picks_the_sentence_with_the_words() {
        let body = "# Plan\nIntro line about nothing. The hopper moves 10 items per transfer. Last bit.";
        let t = terms("hopper items");
        assert_eq!(snippet(body, "", &t, 80), "The hopper moves 10 items per transfer.");
        assert_eq!(snippet(body, "", &terms("zzz"), 80), "Intro line about nothing.");
        assert_eq!(snippet("", "", &t, 80), "");
        assert_eq!(snippet("Same words. Extra detail here", "Same words.", &terms("words"), 80), "Extra detail here");
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
