use std::collections::HashSet;

use serde::Serialize;
use serde_json::Value;

use crate::chat;
use crate::config::{Bench, Weights};
use crate::job::Segment;
use crate::timecode;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Item {
    pub time: String,
    pub at: Option<f64>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Checks {
    pub valid: bool,
    pub complete: bool,
    pub items: usize,
    pub times: Option<f64>,
    pub order: Option<f64>,
    pub spread: Option<f64>,
    pub unique: Option<f64>,
    pub coverage: Option<f64>,
    pub grounded: Option<f64>,
    pub score: f64,
}

pub struct Window {
    pub start: f64,
    pub end: f64,
}

fn collect_text(value: &Value, skip: Option<&str>, out: &mut Vec<String>) {
    match value {
        Value::String(text) => out.push(text.clone()),
        Value::Array(list) => list.iter().for_each(|v| collect_text(v, None, out)),
        Value::Object(map) => {
            map.iter().filter(|(key, _)| Some(key.as_str()) != skip).for_each(|(_, v)| collect_text(v, None, out))
        }
        _ => {}
    }
}

pub fn items(value: &Value) -> Vec<Item> {
    let mut found = Vec::new();
    walk(value, &mut found);
    found
}

fn walk(value: &Value, found: &mut Vec<Item>) {
    match value {
        Value::Array(list) => list.iter().for_each(|v| walk(v, found)),
        Value::Object(map) => {
            if let Some(Value::String(time)) = map.get("t") {
                let mut parts = Vec::new();
                collect_text(value, Some("t"), &mut parts);
                found.push(Item { time: time.clone(), at: timecode::parse(time), text: parts.join(" ") });
                return;
            }
            map.values().for_each(|v| walk(v, found));
        }
        _ => {}
    }
}

pub fn words(text: &str, bench: &Bench) -> HashSet<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .map(str::to_lowercase)
        .filter(|w| w.chars().count() >= bench.min_word_chars)
        .filter(|w| !bench.stop_words.iter().any(|s| s == w))
        .map(|w| w.chars().take(bench.stem_chars.max(1)).collect())
        .collect()
}

fn ratio(hits: usize, count: usize) -> Option<f64> {
    (count > 0).then(|| hits as f64 / count as f64)
}

fn normalised(text: &str) -> String {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
        .collect::<Vec<_>>()
        .join(" ")
}

fn grounded(found: &[Item], transcript: &[Segment], bench: &Bench) -> Option<f64> {
    let mut checkable = 0;
    let mut hits = 0;
    for item in found {
        let Some(at) = item.at else { continue };
        let said: String = transcript
            .iter()
            .filter(|s| s.end >= at - bench.ground_seconds && s.start <= at + bench.ground_seconds)
            .map(|s| s.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        let heard = words(&said, bench);
        let own = words(&item.text, bench);
        if heard.is_empty() || own.is_empty() {
            continue;
        }
        checkable += 1;
        if !own.is_disjoint(&heard) {
            hits += 1;
        }
    }
    ratio(hits, checkable)
}

fn coverage(times: &[f64], window: &Window, bench: &Bench) -> Option<f64> {
    let length = window.end - window.start;
    if times.is_empty() || length <= 0.0 || bench.block_seconds <= 0.0 {
        return None;
    }
    let cap = bench.max_blocks.max(1) as f64;
    let block = bench.block_seconds.max(length / cap);
    let blocks = (length / block).ceil().clamp(1.0, cap) as usize;
    let covered: HashSet<usize> =
        times.iter().map(|t| (((t - window.start) / block).floor().max(0.0) as usize).min(blocks - 1)).collect();
    ratio(covered.len(), blocks)
}

fn score(parts: &[(Option<f64>, f64)]) -> f64 {
    let (sum, weight) = parts
        .iter()
        .filter_map(|(value, weight)| value.map(|v| (v * weight, *weight)))
        .fold((0.0, 0.0), |acc, (v, w)| (acc.0 + v, acc.1 + w));
    if weight > 0.0 { (sum / weight * 100.0).round() } else { 0.0 }
}

impl Checks {
    fn invalid(complete: bool) -> Checks {
        Checks {
            valid: false,
            complete,
            items: 0,
            times: None,
            order: None,
            spread: None,
            unique: None,
            coverage: None,
            grounded: None,
            score: 0.0,
        }
    }
}

fn measure(found: &[Item], complete: bool, window: &Window, transcript: &[Segment], bench: &Bench) -> Checks {
    let slack = bench.time_slack_seconds;
    let parsed: Vec<f64> = found.iter().filter_map(|i| i.at).collect();
    let inside: Vec<f64> =
        parsed.iter().copied().filter(|t| *t >= window.start - slack && *t <= window.end + slack).collect();
    let ordered = parsed.windows(2).filter(|p| p[1] >= p[0]).count();
    let distinct_times: HashSet<u64> = parsed.iter().map(|t| t.to_bits()).collect();
    let distinct_texts: HashSet<String> = found.iter().map(|i| normalised(&i.text)).collect();
    let times = if found.is_empty() { Some(0.0) } else { ratio(inside.len(), found.len()) };
    let order = ratio(ordered, parsed.len().saturating_sub(1));
    let spread = ratio(distinct_times.len(), parsed.len());
    let unique = ratio(distinct_texts.len(), found.len());
    let coverage = coverage(&inside, window, bench);
    let grounded = grounded(found, transcript, bench);
    let w: Weights = bench.weights;
    let score = score(&[
        (Some(if complete { 1.0 } else { 0.0 }), w.complete),
        (times, w.times),
        (order, w.order),
        (spread, w.spread),
        (unique, w.unique),
        (coverage.or(Some(0.0)), w.coverage),
        (grounded, w.grounded),
    ]);
    Checks { valid: true, complete, items: found.len(), times, order, spread, unique, coverage, grounded, score }
}

pub fn check(
    text: &str,
    finish: &str,
    window: &Window,
    transcript: &[Segment],
    bench: &Bench,
) -> (Option<Value>, Vec<Item>, Checks) {
    let complete = finish == chat::FINISH_DONE;
    let Some(value) = serde_json::from_str::<Value>(text).ok().filter(Value::is_object) else {
        return (None, Vec::new(), Checks::invalid(complete));
    };
    let found = items(&value);
    let checks = measure(&found, complete, window, transcript, bench);
    (Some(value), found, checks)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use serde_json::json;

    use super::{Window, check, items, words};
    use crate::config::{Bench, Weights};
    use crate::job::Segment;

    fn bench() -> Bench {
        Bench {
            cases: PathBuf::new(),
            out_dir: PathBuf::new(),
            template: PathBuf::new(),
            data_marker: String::new(),
            latest_name: String::new(),
            models: Vec::new(),
            thumb_long_side: 240,
            block_seconds: 30.0,
            max_blocks: 8,
            time_slack_seconds: 1.0,
            ground_seconds: 45.0,
            min_word_chars: 4,
            stem_chars: 5,
            stop_words: vec!["then".into(), "with".into()],
            gpu_query: Vec::new(),
            game_error: String::new(),
            game_wait_seconds: 0.0,
            game_poll_seconds: 1.0,
            weights: Weights {
                complete: 1.0,
                times: 1.0,
                order: 1.0,
                spread: 1.0,
                unique: 1.0,
                coverage: 1.0,
                grounded: 2.0,
            },
            cold_start: false,
            history_runs: 0,
            reference: None,
            measurements: None,
            style: None,
            style_marker: String::new(),
            card: PathBuf::new(),
            write_tries: 1,
            write_wait_ms: 0,
        }
    }

    fn said(start: f64, text: &str) -> Segment {
        Segment { start, end: start + 5.0, text: text.into() }
    }

    fn window() -> Window {
        Window { start: 0.0, end: 120.0 }
    }

    #[test]
    fn items_are_every_object_with_a_time() {
        let value = json!({"sum": "x", "steps": [
            {"t": "00:05", "do": "Select cube", "keys": ["Tab"]},
            {"t": "bad", "do": "Bevel", "vals": [{"n": "Width", "v": "0.5"}]}
        ]});
        let found = items(&value);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].at, Some(5.0));
        assert_eq!(found[0].text, "Select cube Tab");
        assert_eq!(found[1].at, None);
        assert!(found[1].text.contains("Width") && found[1].text.contains("0.5"));
    }

    #[test]
    fn words_drop_short_and_stop_words_and_share_stems() {
        let set = words("Then select the SELECTION with bevels", &bench());
        assert!(set.contains("selec"));
        assert!(set.contains("bevel"));
        assert!(!set.contains("then") && !set.contains("the"));
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn broken_json_scores_zero() {
        let (value, _, checks) = check("{\"steps\": [", "length", &window(), &[], &bench());
        assert!(value.is_none());
        assert!(!checks.valid && !checks.complete);
        assert_eq!(checks.score, 0.0);
    }

    #[test]
    fn a_clean_answer_scores_full_marks() {
        let text = json!({"steps": [
            {"t": "00:10", "do": "Select the cube"},
            {"t": "00:40", "do": "Bevel the edges"},
            {"t": "01:15", "do": "Render the scene"},
            {"t": "01:45", "do": "Export the image"}
        ]})
        .to_string();
        let transcript = [
            said(8.0, "first we select our cube"),
            said(38.0, "now bevel those edges"),
            said(70.0, "and render it"),
            said(100.0, "finally export the image"),
        ];
        let (_, found, checks) = check(&text, "stop", &window(), &transcript, &bench());
        assert_eq!(found.len(), 4);
        assert_eq!(checks.times, Some(1.0));
        assert_eq!(checks.order, Some(1.0));
        assert_eq!(checks.spread, Some(1.0));
        assert_eq!(checks.unique, Some(1.0));
        assert_eq!(checks.coverage, Some(1.0));
        assert_eq!(checks.grounded, Some(1.0));
        assert_eq!(checks.score, 100.0);
    }

    #[test]
    fn repeats_wrong_times_and_made_up_steps_cost_points() {
        let text = json!({"steps": [
            {"t": "00:10", "do": "Select the cube"},
            {"t": "00:10", "do": "Select the cube"},
            {"t": "00:05", "do": "Paint a dragon"},
            {"t": "05:00", "do": "Export"}
        ]})
        .to_string();
        let transcript = [said(8.0, "first we select our cube")];
        let (_, _, checks) = check(&text, "stop", &window(), &transcript, &bench());
        assert_eq!(checks.times, Some(0.75));
        assert_eq!(checks.unique, Some(0.75));
        assert_eq!(checks.spread, Some(0.75));
        assert!(checks.order.unwrap() < 1.0);
        assert!(checks.grounded.unwrap() < 1.0);
        assert!(checks.score < 90.0);
    }

    #[test]
    fn no_transcript_means_grounding_is_not_judged() {
        let text = json!({"pts": [{"t": "00:10", "x": "A slide about loss"}]}).to_string();
        let (_, _, checks) = check(&text, "stop", &window(), &[], &bench());
        assert_eq!(checks.grounded, None);
        assert!(checks.score > 0.0);
    }

    #[test]
    fn two_steps_for_a_long_video_do_not_cover_it() {
        let text = json!({"steps": [
            {"t": "00:10", "do": "Select the cube"},
            {"t": "00:20", "do": "Bevel the edges"}
        ]})
        .to_string();
        let (_, _, checks) = check(&text, "stop", &window(), &[], &bench());
        assert_eq!(checks.coverage, Some(0.25));
    }

    #[test]
    fn an_empty_list_is_a_bad_answer() {
        let (_, _, checks) = check("{\"steps\": []}", "stop", &window(), &[], &bench());
        assert_eq!(checks.items, 0);
        assert_eq!(checks.times, Some(0.0));
        assert!(checks.score < 50.0);
    }
}
