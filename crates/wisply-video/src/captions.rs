use std::sync::LazyLock;

use regex::Regex;
use serde_json::Value;

use crate::youtube::{ORIGINAL_SUFFIX, VideoInfo};

pub const JSON3_SUFFIX: &str = "json3";
pub const VTT_SUFFIX: &str = "vtt";
const MILLISECONDS: f64 = 1000.0;
const LANGUAGE_SEPARATOR: char = '-';
const VTT_ARROW: &str = "-->";
const BYTE_ORDER_MARK: char = '\u{feff}';
const FRACTION_DIGITS: usize = 3;
const ENTITIES: [(&str, &str); 6] =
    [("amp", "&"), ("lt", "<"), ("gt", ">"), ("quot", "\""), ("apos", "'"), ("nbsp", "\u{a0}")];

static VTT_TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new("<[^>]*>").expect("tag"));
static VTT_TIME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:(\d+):)?(\d{1,2}):(\d{2})[.,](\d{1,3})$").expect("time"));
static BLOCK_GAP: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\r?\n\s*\r?\n").expect("gap"));
static ENTITY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"&(?:#(\d{1,7})|#[xX]([0-9A-Fa-f]{1,6})|([A-Za-z]+));").expect("entity"));

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Track {
    pub key: String,
    pub automatic: bool,
}

impl Segment {
    pub fn new(start: f64, end: f64, text: impl Into<String>) -> Segment {
        Segment { start, end, text: text.into() }
    }
}

impl Track {
    pub fn language(&self) -> String {
        base_language(self.key.strip_suffix(ORIGINAL_SUFFIX).unwrap_or(&self.key))
    }
}

pub fn base_language(code: &str) -> String {
    code.split(LANGUAGE_SEPARATOR).next().unwrap_or_default().to_lowercase()
}

pub fn choose_track(info: &VideoInfo) -> Option<Track> {
    let language = base_language(&info.language);
    let manual = &info.subtitle_languages;
    let automatic = &info.caption_languages;
    let track = |key: &String, automatic: bool| Some(Track { key: key.clone(), automatic });
    let originals: Vec<&String> = automatic.iter().filter(|key| key.ends_with(ORIGINAL_SUFFIX)).collect();
    if !language.is_empty() {
        if let Some(key) = manual.iter().find(|key| base_language(key) == language) {
            return track(key, false);
        }
        if let Some(key) = originals.iter().find(|key| base_language(key) == language) {
            return track(key, true);
        }
    }
    if let Some(key) = originals.first() {
        return track(key, true);
    }
    if let Some(key) = manual.first() {
        return track(key, false);
    }
    if !language.is_empty() && automatic.contains(&language) {
        return track(&language, true);
    }
    match automatic.as_slice() {
        [only] => track(only, true),
        _ => None,
    }
}

fn unescape(text: &str) -> String {
    let decoded = ENTITY.replace_all(text, |found: &regex::Captures| {
        let number =
            |group: usize, radix: u32| found.get(group).and_then(|m| u32::from_str_radix(m.as_str(), radix).ok());
        let named = || found.get(3).and_then(|name| ENTITIES.iter().find(|(known, _)| *known == name.as_str()));
        match number(1, 10).or_else(|| number(2, 16)).and_then(char::from_u32) {
            Some(letter) => letter.to_string(),
            None => named().map_or_else(|| found[0].to_string(), |(_, letter)| letter.to_string()),
        }
    });
    decoded.into_owned()
}

pub fn tidy_text(text: &str) -> String {
    unescape(text).split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn spoken(parts: &[Value]) -> String {
    let words = parts.iter().filter_map(|part| part.get("utf8").and_then(Value::as_str));
    tidy_text(&words.collect::<String>())
}

pub fn parse_json3(data: &Value) -> Vec<Segment> {
    let Some(events) = data.get("events").and_then(Value::as_array) else { return Vec::new() };
    let mut cues = Vec::new();
    for event in events {
        let parts = event.get("segs").and_then(Value::as_array);
        let start = event.get("tStartMs").and_then(Value::as_f64);
        let (Some(parts), Some(start)) = (parts, start) else { continue };
        let text = spoken(parts);
        if text.is_empty() {
            continue;
        }
        let seconds = event.get("dDurationMs").and_then(Value::as_f64).unwrap_or(0.0) / MILLISECONDS;
        let begin = start / MILLISECONDS;
        cues.push(Segment::new(begin, begin + seconds.max(0.0), text));
    }
    clip_overlaps(cues)
}

pub fn vtt_seconds(stamp: &str) -> Option<f64> {
    let found = VTT_TIME.captures(stamp.split_whitespace().next().unwrap_or_default())?;
    let whole = |group: usize| found.get(group).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0);
    let fraction = format!("{:0<FRACTION_DIGITS$}", &found[4]).parse::<f64>().ok()?;
    Some(whole(1) * 3600.0 + whole(2) * 60.0 + whole(3) + fraction / MILLISECONDS)
}

pub fn parse_vtt(text: &str) -> Vec<Segment> {
    let mut cues = Vec::new();
    let mut last_line = String::new();
    for block in BLOCK_GAP.split(text.strip_prefix(BYTE_ORDER_MARK).unwrap_or(text)) {
        let lines: Vec<&str> = block.lines().filter(|line| !line.trim().is_empty()).collect();
        let Some(timing) = lines.iter().position(|line| line.contains(VTT_ARROW)) else { continue };
        let Some((begin, end)) = lines[timing].split_once(VTT_ARROW) else { continue };
        let (Some(begin), Some(end)) = (vtt_seconds(begin), vtt_seconds(end)) else { continue };
        let mut fresh: Vec<String> = Vec::new();
        for raw in &lines[timing + 1..] {
            let line = tidy_text(&VTT_TAG.replace_all(raw, ""));
            if !line.is_empty() && line != last_line {
                fresh.push(line.clone());
                last_line = line;
            }
        }
        if !fresh.is_empty() {
            cues.push(Segment::new(begin, begin.max(end), fresh.join(" ")));
        }
    }
    clip_overlaps(cues)
}

pub fn clip_overlaps(mut cues: Vec<Segment>) -> Vec<Segment> {
    cues.sort_by(|first, second| first.start.total_cmp(&second.start));
    let starts: Vec<f64> = cues.iter().map(|cue| cue.start).collect();
    for (index, cue) in cues.iter_mut().enumerate() {
        let following = starts.get(index + 1).copied().unwrap_or(cue.end);
        let end = if following > cue.start { cue.end.min(following) } else { cue.end };
        cue.end = cue.start.max(end);
    }
    cues
}

pub fn group_segments(cues: &[Segment], max_seconds: f64, max_chars: usize) -> Vec<Segment> {
    let mut grouped: Vec<Segment> = Vec::new();
    for cue in cues {
        if let Some(current) = grouped.last_mut() {
            let joined = format!("{} {}", current.text, cue.text);
            if cue.end - current.start <= max_seconds && joined.chars().count() <= max_chars {
                current.end = cue.end;
                current.text = joined;
                continue;
            }
        }
        grouped.push(cue.clone());
    }
    grouped
}

pub fn within(segments: &[Segment], start: Option<f64>, end: Option<f64>) -> Vec<Segment> {
    let low = start.unwrap_or(0.0);
    let kept = segments.iter().filter(|segment| segment.end >= low && end.is_none_or(|end| segment.start <= end));
    kept.cloned().collect()
}

#[cfg(test)]
#[path = "captions_tests.rs"]
mod tests;
