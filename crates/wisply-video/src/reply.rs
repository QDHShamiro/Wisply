use std::sync::LazyLock;

use regex::Regex;
use serde_json::{Map, Value};

use crate::pacific;

pub const COMPLETED: &str = "completed";
pub const REQUIRES_CONTINUATION: &str = "requires_continuation";
const USER_INPUT: &str = "user_input";
const THOUGHT: &str = "thought";
const MODEL_OUTPUT: &str = "model_output";
const TEXT: &str = "text";
const FIRST_HOP_ONLY: [&str; 1] = ["initial_input_tokens"];
const TOTAL: &str = "total_tokens";
const TOKEN_PARTS: [&str; 3] = ["total_input_tokens", "total_output_tokens", "total_thought_tokens"];
const MONTHS: [&str; 12] = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];
const ZONES: [&str; 2] = ["gmt", "utc"];
const MINUTE: i64 = 60;
const HOUR: i64 = 3600;
const DAY: i64 = 86_400;

static DURATION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\s*(\d+(?:\.\d+)?)s\s*$").expect("duration"));

pub type Usage = Map<String, Value>;

fn kind(value: &Value) -> &str {
    value.get("type").and_then(Value::as_str).unwrap_or_default()
}

fn list(value: Option<&Value>) -> &[Value] {
    value.and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
}

fn error(body: &Value) -> Option<&Map<String, Value>> {
    body.get("error")?.as_object()
}

pub fn word<'a>(interaction: &'a Value, name: &str) -> &'a str {
    interaction.get(name).and_then(Value::as_str).unwrap_or_default()
}

pub fn output_text(interaction: &Value) -> String {
    let mut parts: Vec<&str> = Vec::new();
    'steps: for step in list(interaction.get("steps")).iter().rev() {
        match kind(step) {
            USER_INPUT => break,
            THOUGHT => continue,
            MODEL_OUTPUT => {}
            _ if parts.is_empty() => continue,
            _ => break,
        }
        for item in list(step.get("content")).iter().rev() {
            if kind(item) == TEXT {
                parts.push(word(item, TEXT));
            } else if !parts.is_empty() {
                break 'steps;
            }
        }
    }
    parts.reverse();
    parts.concat()
}

pub fn usage(interaction: &Value) -> Usage {
    let found = interaction.get("usage").and_then(Value::as_object);
    found
        .into_iter()
        .flatten()
        .filter(|(_, value)| !value.is_null())
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect()
}

fn whole(usage: &Usage, name: &str) -> i64 {
    usage.get(name).and_then(Value::as_i64).unwrap_or(0)
}

pub fn total_tokens(usage: &Usage) -> i64 {
    match whole(usage, TOTAL) {
        0 => TOKEN_PARTS.iter().map(|name| whole(usage, name)).sum(),
        total => total,
    }
}

fn joined(held: &Value, more: &Value) -> Option<Value> {
    match (held, more) {
        (Value::Number(_), Value::Number(_)) => Some(Value::from(held.as_i64()? + more.as_i64()?)),
        (Value::Array(first), Value::Array(second)) => {
            Some(Value::Array([first.as_slice(), second.as_slice()].concat()))
        }
        _ => None,
    }
}

pub fn merge_usage(parts: &[Usage]) -> Usage {
    let mut merged = Usage::new();
    for (name, value) in parts.iter().flatten() {
        let next = match merged.get(name) {
            None => Some(value.clone()),
            Some(_) if FIRST_HOP_ONLY.contains(&name.as_str()) => None,
            Some(held) => joined(held, value),
        };
        if let Some(next) = next {
            merged.insert(name.clone(), next);
        }
    }
    merged
}

pub fn error_message(body: &Value) -> String {
    let message = error(body).and_then(|error| error.get("message")).and_then(Value::as_str).unwrap_or_default();
    message.split_whitespace().collect::<Vec<&str>>().join(" ")
}

pub fn error_details(body: &Value) -> impl Iterator<Item = &Map<String, Value>> {
    list(error(body).and_then(|error| error.get("details"))).iter().filter_map(Value::as_object)
}

pub fn error_reasons(body: &Value) -> Vec<&str> {
    let status = error(body).and_then(|error| error.get("status")).and_then(Value::as_str);
    let reasons = error_details(body).filter_map(|detail| detail.get("reason").and_then(Value::as_str));
    status.into_iter().chain(reasons).collect()
}

pub fn seconds_from_duration(value: Option<&Value>) -> Option<f64> {
    DURATION.captures(value?.as_str()?)?[1].parse().ok()
}

fn http_date(value: &str) -> Option<f64> {
    let parts: Vec<&str> = value.split_whitespace().collect();
    let [_, day, month, year, clock, zone] = parts.as_slice() else {
        return None;
    };
    if !ZONES.iter().any(|known| zone.eq_ignore_ascii_case(known)) {
        return None;
    }
    let month = MONTHS.iter().position(|name| month.eq_ignore_ascii_case(name))? as i64 + 1;
    let mut clock = clock.split(':').map(|part| part.parse::<i64>().ok());
    let (hour, minute, second) = (clock.next()??, clock.next()??, clock.next()??);
    let days = pacific::days_from_civil(year.parse().ok()?, month, day.parse().ok()?);
    Some((days * DAY + hour * HOUR + minute * MINUTE + second) as f64)
}

pub fn seconds_from_header(value: Option<&str>, now: f64) -> Option<f64> {
    let value = value?.trim();
    let seconds = match value.parse::<f64>() {
        Ok(seconds) => seconds,
        Err(_) => http_date(value)? - now,
    };
    (seconds.is_finite() && seconds > 0.0).then_some(seconds)
}

#[cfg(test)]
#[path = "reply_tests.rs"]
mod tests;
