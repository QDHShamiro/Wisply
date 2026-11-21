use serde_json::{Map, Value};

use crate::guard::{REQUESTS_PER_DAY, REQUESTS_PER_MINUTE, TOKENS_PER_MINUTE};
use crate::reply;

const TOO_MANY_REQUESTS: u16 = 429;
const REQUEST_TIMEOUT: u16 = 408;
const UNAUTHORIZED: u16 = 401;
const NOT_FOUND: u16 = 404;
const FIRST_SERVER_ERROR: u16 = 500;
const KEY_REASONS: [&str; 7] = [
    "API_KEY_INVALID",
    "API_KEY_EXPIRED",
    "API_KEY_SERVICE_BLOCKED",
    "API_KEY_HTTP_REFERRER_BLOCKED",
    "API_KEY_IP_ADDRESS_BLOCKED",
    "SERVICE_DISABLED",
    "UNAUTHENTICATED",
];
const KEY_MESSAGE_MARKER: &str = "api key";
const DAY_MARKER: &str = "perday";
const MINUTE_MARKER: &str = "perminute";
const TOKEN_MARKER: &str = "token";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    RateLimited,
    Server,
    Network,
    KeyRejected,
    NotFound,
    Rejected,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Trouble {
    pub class: Class,
    pub status: Option<u16>,
    pub message: String,
    pub retry_seconds: Option<f64>,
    pub daily: bool,
    pub learned: Vec<(&'static str, f64)>,
}

fn one_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn violations(body: &Value) -> impl Iterator<Item = &Map<String, Value>> {
    let lists = reply::error_details(body).filter_map(|detail| detail.get("violations")?.as_array());
    lists.flatten().filter_map(Value::as_object)
}

fn rule_for_quota(quota: &str) -> Option<&'static str> {
    let per_token = quota.contains(TOKEN_MARKER);
    if quota.contains(DAY_MARKER) {
        (!per_token).then_some(REQUESTS_PER_DAY)
    } else if quota.contains(MINUTE_MARKER) {
        Some(if per_token { TOKENS_PER_MINUTE } else { REQUESTS_PER_MINUTE })
    } else {
        None
    }
}

fn positive(value: Option<&Value>) -> Option<f64> {
    let number = match value? {
        Value::String(text) => text.trim().parse().ok()?,
        other => other.as_f64()?,
    };
    (number.is_finite() && number > 0.0).then_some(number)
}

fn quota_facts(body: &Value) -> (bool, Vec<(&'static str, f64)>) {
    let mut daily = false;
    let mut learned = Vec::new();
    for violation in violations(body) {
        let quota = violation.get("quotaId").and_then(Value::as_str).unwrap_or_default().to_lowercase();
        daily |= quota.contains(DAY_MARKER);
        if let (Some(rule), Some(value)) = (rule_for_quota(&quota), positive(violation.get("quotaValue"))) {
            learned.push((rule, value));
        }
    }
    (daily, learned)
}

fn retry_delay(body: &Value) -> Option<f64> {
    let mut waits =
        reply::error_details(body).filter_map(|detail| reply::seconds_from_duration(detail.get("retryDelay")));
    waits.find(|seconds| *seconds > 0.0)
}

fn key_problem(status: u16, body: &Value, message: &str) -> bool {
    status == UNAUTHORIZED
        || reply::error_reasons(body).iter().any(|reason| KEY_REASONS.contains(reason))
        || message.to_lowercase().contains(KEY_MESSAGE_MARKER)
}

fn class(status: u16, body: &Value, message: &str) -> Class {
    match status {
        TOO_MANY_REQUESTS => Class::RateLimited,
        REQUEST_TIMEOUT => Class::Server,
        status if status >= FIRST_SERVER_ERROR => Class::Server,
        status if key_problem(status, body, message) => Class::KeyRejected,
        NOT_FOUND => Class::NotFound,
        _ => Class::Rejected,
    }
}

pub fn answered(status: u16, retry_after: Option<&str>, text: &str, now: f64) -> Trouble {
    let body: Value = serde_json::from_str(text).unwrap_or(Value::Null);
    let named = reply::error_message(&body);
    let message = if named.is_empty() { one_line(text) } else { named };
    let retry_seconds = reply::seconds_from_header(retry_after, now).or_else(|| retry_delay(&body));
    let (daily, learned) = quota_facts(&body);
    Trouble { class: class(status, &body, &message), status: Some(status), message, retry_seconds, daily, learned }
}

pub fn no_answer(message: &str) -> Trouble {
    Trouble {
        class: Class::Network,
        status: None,
        message: one_line(message),
        retry_seconds: None,
        daily: false,
        learned: Vec::new(),
    }
}

#[cfg(test)]
#[path = "trouble_tests.rs"]
mod tests;
