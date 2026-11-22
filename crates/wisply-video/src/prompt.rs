use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::captions::Segment;
use crate::config;
use crate::fail::{Fail, Kind};
use crate::schema::{self, Mode};
use crate::timecode::{self, Range};
use crate::youtube::Chapter;

const VERSION: u64 = 2;
const FINGERPRINT_CHARS: usize = 12;
const OPENING: &str = "You watch a video for an assistant that can neither see nor hear it. \
    Report only what is really shown or heard, nothing from general knowledge. \
    Copy values, shortcuts, menu names and code exactly as they appear on screen. \
    When something is too small or blurry to read, mark it unsure instead of guessing. \
    Name a key or mouse action only when it is visible (key overlay, cursor, menu) or said \
    aloud, never infer it.";
const STEPS_TASK: &str = "This is a tutorial. List every step the presenter performs, in order, one entry per \
    action, so that someone can repeat it exactly. If the video is a talk or slides rather than hands-on work, \
    give one entry per idea with what the slide shows and says, not slide changes. Also list notable \
    sounds other than speech.";
const SUMMARY_TASK: &str = "Summarise the video: the gist, the key points with their times, and notable sounds \
    other than speech such as music, alerts or effects.";
const FULL_TASK: &str = "Summarise the video, then list every step the presenter performs, in order, one entry \
    per action, so that someone can repeat it exactly. If the video is a talk or slides rather than hands-on \
    work, give one entry per idea with what the slide shows and says, not slide changes. Also list \
    notable sounds other than speech.";
const FOCUS_TEMPLATE: &str = "Focus on: {focus}. Leave out what is unrelated to it.";
const QUIET_TEMPLATE: &str = "Nobody speaks at {spans}. Watch these parts closely, silent actions are easy to miss.";
const WINDOW_TEMPLATE: &str = "You see only the part from {start} to {end}.";
const TITLE_TEMPLATE: &str = "Video: \"{title}\"";
const CHANNEL_TEMPLATE: &str = " by {channel}";
const LENGTH_TEMPLATE: &str = ", length {length}";
const CHAPTERS_PREFIX: &str = "Chapters: ";
const DESCRIPTION_TEMPLATE: &str = "Description from the uploader: \"\"\"{description}\"\"\"";
const OPEN_END: &str = "the end";
const FOCUS_HINT: &str = "name the topic in a few words";

#[derive(Debug, Clone, Default)]
pub struct Facts<'a> {
    pub title: &'a str,
    pub channel: &'a str,
    pub duration: Option<f64>,
    pub chapters: &'a [Chapter],
    pub description: &'a str,
    pub window: Option<Range>,
    pub quiet: &'a [(f64, f64)],
    pub focus: Option<&'a str>,
}

fn task(mode: Mode) -> &'static str {
    match mode {
        Mode::Summary => SUMMARY_TASK,
        Mode::Steps => STEPS_TASK,
        Mode::Full => FULL_TASK,
    }
}

pub fn clean_focus(focus: Option<&str>, settings: &config::Analysis) -> Result<Option<String>, Fail> {
    let words: Vec<&str> = focus.unwrap_or_default().split_whitespace().collect();
    let cleaned = words.join(" ").trim_end_matches('.').to_string();
    let length = cleaned.chars().count();
    if length > settings.max_focus_chars {
        let message = format!("focus is {length} characters long, at most {} fit", settings.max_focus_chars);
        return Err(Fail::hint(Kind::Input, message, FOCUS_HINT));
    }
    Ok(if cleaned.is_empty() { None } else { Some(cleaned) })
}

pub fn quiet_spans(segments: &[Segment], start: f64, end: f64, settings: &config::Analysis) -> Vec<(f64, f64)> {
    let mut spoken: Vec<&Segment> = segments.iter().filter(|part| part.end > start && part.start < end).collect();
    spoken.sort_by(|a, b| a.start.total_cmp(&b.start));
    let mut gaps = Vec::new();
    let mut cursor = start;
    for part in spoken {
        if part.start - cursor >= settings.quiet_min_seconds {
            gaps.push((cursor, part.start));
        }
        cursor = cursor.max(part.end);
    }
    if end - cursor >= settings.quiet_min_seconds {
        gaps.push((cursor, end));
    }
    gaps.sort_by(|a, b| (b.1 - b.0).total_cmp(&(a.1 - a.0)));
    gaps.truncate(settings.max_quiet_spans);
    gaps.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    gaps
}

fn chapters_in_window<'a>(facts: &Facts<'a>, most: usize) -> Vec<&'a Chapter> {
    let (first, last) = match facts.window {
        Some(window) => (window.start, window.end.unwrap_or(f64::INFINITY)),
        None => (f64::NEG_INFINITY, f64::INFINITY),
    };
    facts.chapters.iter().filter(|chapter| chapter.start < last && chapter.end > first).take(most).collect()
}

fn video_facts(facts: &Facts, settings: &config::Analysis) -> Vec<String> {
    let mut lines = Vec::new();
    if !facts.title.is_empty() {
        let mut line = TITLE_TEMPLATE.replace("{title}", facts.title);
        if !facts.channel.is_empty() {
            line.push_str(&CHANNEL_TEMPLATE.replace("{channel}", facts.channel));
        }
        if let Some(length) = facts.duration.filter(|length| *length != 0.0) {
            line.push_str(&LENGTH_TEMPLATE.replace("{length}", &timecode::format(length)));
        }
        lines.push(line + ".");
    }
    let chapters = chapters_in_window(facts, settings.max_chapters);
    if !chapters.is_empty() {
        let listed: Vec<String> =
            chapters.iter().map(|item| format!("{} {}", timecode::format(item.start), item.title)).collect();
        lines.push(format!("{CHAPTERS_PREFIX}{}", listed.join("; ")));
    }
    if !facts.description.is_empty() {
        lines.push(DESCRIPTION_TEMPLATE.replace("{description}", facts.description));
    }
    lines
}

fn window_line(facts: &Facts) -> Option<String> {
    let window = facts.window?;
    let end = window.end.or(facts.duration).map_or_else(|| OPEN_END.to_string(), timecode::format);
    Some(WINDOW_TEMPLATE.replace("{start}", &timecode::format(window.start)).replace("{end}", &end))
}

pub fn build(mode: Mode, facts: &Facts, settings: &config::Analysis) -> String {
    let mut lines = vec![OPENING.to_string()];
    lines.extend(video_facts(facts, settings));
    lines.extend(window_line(facts));
    lines.push(task(mode).to_string());
    if let Some(focus) = facts.focus {
        lines.push(FOCUS_TEMPLATE.replace("{focus}", focus));
    }
    if !facts.quiet.is_empty() {
        let span = |(start, end): &(f64, f64)| format!("{}-{}", timecode::format(*start), timecode::format(*end));
        let spans: Vec<String> = facts.quiet.iter().map(span).collect();
        lines.push(QUIET_TEMPLATE.replace("{spans}", &spans.join(", ")));
    }
    lines.join("\n")
}

fn sorted_json(value: &Value, out: &mut String) {
    match value {
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                out.push_str(if index > 0 { ", " } else { "" });
                sorted_json(item, out);
            }
            out.push(']');
        }
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            out.push('{');
            for (index, key) in keys.into_iter().enumerate() {
                out.push_str(if index > 0 { ", " } else { "" });
                out.push_str(&Value::String(key.clone()).to_string());
                out.push_str(": ");
                sorted_json(&map[key], out);
            }
            out.push('}');
        }
        other => out.push_str(&other.to_string()),
    }
}

pub fn fingerprint(mode: Mode, settings: &config::Analysis) -> String {
    let parts = json!([
        VERSION,
        OPENING,
        task(mode),
        FOCUS_TEMPLATE,
        QUIET_TEMPLATE,
        WINDOW_TEMPLATE,
        TITLE_TEMPLATE,
        CHANNEL_TEMPLATE,
        LENGTH_TEMPLATE,
        CHAPTERS_PREFIX,
        DESCRIPTION_TEMPLATE,
        settings.max_chapters,
        schema::answer(mode, settings),
    ]);
    let mut encoded = String::new();
    sorted_json(&parts, &mut encoded);
    let digest: String = Sha256::digest(encoded.as_bytes()).iter().map(|byte| format!("{byte:02x}")).collect();
    format!("{VERSION}-{}", &digest[..FINGERPRINT_CHARS])
}

#[cfg(test)]
#[path = "prompt_tests.rs"]
mod tests;
