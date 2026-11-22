use std::sync::LazyLock;

use regex::{Captures, Regex};
use serde_json::Value;

use crate::fail::{Fail, Kind};

const FORMS: &str = "seconds (83), \"1:23\", \"1:02:03\" or \"1m23s\"";
const MINUTE: f64 = 60.0;
const HOUR: f64 = 3600.0;
const WRAPPERS: &str = "\"'`[]";

static PLAIN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([0-9]+(?:\.[0-9]+)?)\s*(?:s|sec|seconds)?$").expect("plain"));
static COLONS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:([0-9]+):)?([0-9]+):([0-9]{1,2}(?:\.[0-9]+)?)$").expect("colons"));
static UNITS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:([0-9]+)\s*h)?\s*(?:([0-9]+)\s*m(?:in)?)?\s*(?:([0-9]+(?:\.[0-9]+)?)\s*s(?:ec)?)?$")
        .expect("units")
});
static CLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:([0-9]+):)?([0-9]{1,3}):([0-9]{2}(?:\.[0-9]+)?)").expect("clock"));
static SEPARATOR: LazyLock<Regex> = LazyLock::new(|| Regex::new("[,;]").expect("separator"));

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Range {
    pub start: f64,
    pub end: Option<f64>,
}

impl Range {
    pub fn start_offset(&self) -> String {
        format!("{}s", self.start.floor() as u64)
    }

    pub fn end_offset(&self) -> Option<String> {
        self.end.map(|end| format!("{}s", end.ceil() as u64))
    }

    pub fn label(&self) -> String {
        let end = self.end.map_or_else(|| "end".to_string(), format);
        format!("{}-{end}", format(self.start))
    }
}

fn refuse(shown: &str, reason: &str) -> Fail {
    Fail::hint(Kind::Time, format!("time '{}' {reason}", shown.trim()), format!("use {FORMS}"))
}

fn part(found: &Captures, index: usize) -> f64 {
    found.get(index).and_then(|m| m.as_str().parse::<f64>().ok()).unwrap_or(0.0)
}

fn join(found: &Captures) -> f64 {
    part(found, 1) * HOUR + part(found, 2) * MINUTE + part(found, 3)
}

fn from_colons(shown: &str, found: &Captures) -> Result<f64, Fail> {
    if part(found, 3) >= MINUTE {
        return Err(refuse(shown, "has 60 or more seconds after the colon"));
    }
    if found.get(1).is_some() && part(found, 2) >= MINUTE {
        return Err(refuse(shown, "has 60 or more minutes after the hours"));
    }
    Ok(join(found))
}

fn checked(shown: &str, seconds: f64) -> Result<f64, Fail> {
    if !seconds.is_finite() {
        return Err(refuse(shown, "is not a finite number"));
    }
    if seconds < 0.0 {
        return Err(refuse(shown, "is negative"));
    }
    Ok(seconds)
}

pub fn parse(value: &str) -> Result<f64, Fail> {
    let text = value.trim_matches(|c: char| WRAPPERS.contains(c) || c.is_whitespace()).to_lowercase();
    if text.is_empty() {
        return Err(refuse(value, "is empty"));
    }
    if let Some(found) = PLAIN.captures(&text) {
        return checked(value, part(&found, 1));
    }
    if let Some(found) = COLONS.captures(&text) {
        return checked(value, from_colons(value, &found)?);
    }
    match UNITS.captures(&text) {
        Some(found) if (1..=3).any(|index| found.get(index).is_some()) => checked(value, join(&found)),
        _ => Err(refuse(value, "is not a time")),
    }
}

pub fn parse_value(value: &Value) -> Result<f64, Fail> {
    match value {
        Value::String(text) => parse(text),
        Value::Number(number) => checked(&number.to_string(), number.as_f64().unwrap_or(f64::NAN)),
        other => Err(refuse(&other.to_string(), "is not a time")),
    }
}

pub fn find(value: &Value) -> Option<f64> {
    if let Value::String(text) = value
        && let Some(found) = CLOCK.captures(text)
    {
        return Some(join(&found));
    }
    parse_value(value).ok()
}

pub fn format(seconds: f64) -> String {
    let whole = if seconds.is_finite() && seconds > 0.0 { seconds.floor() as u64 } else { 0 };
    let (hours, minutes, secs) = (whole / 3600, whole % 3600 / 60, whole % 60);
    if hours > 0 { format!("{hours}:{minutes:02}:{secs:02}") } else { format!("{minutes:02}:{secs:02}") }
}

pub fn range(start: Option<&str>, end: Option<&str>, duration: Option<f64>) -> Result<Option<Range>, Fail> {
    if start.is_none() && end.is_none() {
        return Ok(None);
    }
    let from = start.map(parse).transpose()?.unwrap_or(0.0);
    let mut to = end.map(parse).transpose()?;
    if let Some(to) = to
        && to <= from
    {
        return Err(Fail::new(Kind::Time, format!("end {} is not after start {}", format(to), format(from))));
    }
    if let Some(length) = duration {
        if from >= length {
            let message = format!("start {} is past the end of the video ({})", format(from), format(length));
            return Err(Fail::new(Kind::Time, message));
        }
        to = to.filter(|to| *to < length);
    }
    Ok(if from == 0.0 && to.is_none() { None } else { Some(Range { start: from, end: to }) })
}

pub fn stamps(values: &str, most: usize, duration: Option<f64>) -> Result<Vec<f64>, Fail> {
    let entries: Vec<&str> = SEPARATOR.split(values).filter(|entry| !entry.trim().is_empty()).collect();
    if entries.is_empty() {
        return Err(Fail::hint(Kind::Time, "no timestamps were given", format!("use {FORMS}")));
    }
    let mut seconds = entries.into_iter().map(parse).collect::<Result<Vec<f64>, Fail>>()?;
    seconds.sort_by(f64::total_cmp);
    seconds.dedup();
    if seconds.len() > most {
        let message = format!("{} timestamps were given, at most {most} fit into one call", seconds.len());
        return Err(Fail::hint(Kind::Time, message, "ask for the rest in a second call"));
    }
    if let Some(length) = duration {
        let late: Vec<String> = seconds.iter().filter(|value| **value >= length).map(|value| format(*value)).collect();
        if !late.is_empty() {
            let message =
                format!("these timestamps are past the end of the video ({}): {}", format(length), late.join(", "));
            return Err(Fail::new(Kind::Time, message));
        }
    }
    Ok(seconds)
}

#[cfg(test)]
#[path = "timecode_tests.rs"]
mod tests;
