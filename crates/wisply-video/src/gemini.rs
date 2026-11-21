use std::sync::LazyLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{Map, Value};

use crate::config;
use crate::fail::{Fail, Kind};
use crate::guard::{self, Ask, Clock, Guard};
use crate::key;
use crate::redact;
use crate::reply::{self, Usage};
use crate::trouble::{self, Class, Trouble};

const ADDRESS: &str = "https://generativelanguage.googleapis.com/v1beta/interactions";
const KEY_HEADER: &str = "x-goog-api-key";
const JSON: &str = "application/json";
const RETRY_AFTER: &str = "retry-after";
const MODEL: &str = "model";
const STATUS: &str = "status";
const CONTINUATION: &str = "continuation_token";
const HIDDEN_KEY: &str = "<key>";
const FIRST_GOOD: u16 = 200;
const FIRST_BAD: u16 = 300;
const SHORTEST_WAIT: f64 = 1.0;
const GROWTH: f64 = 2.0;
const STEADY_SHARE: f64 = 0.5;
const NANOS: f64 = 1e9;
const NETWORK_PROBLEM: &str = "a network problem";
const UNKNOWN: &str = "unknown";

static AGENT: LazyLock<ureq::Agent> =
    LazyLock::new(|| ureq::Agent::config_builder().http_status_as_error(false).build().into());

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sent {
    pub status: u16,
    pub retry_after: Option<String>,
    pub text: String,
}

pub trait Transport {
    fn post(&self, body: &str, seconds: f64) -> Result<Sent, String>;
}

pub struct Http {
    key: String,
}

pub struct Request<'a> {
    pub body: &'a Map<String, Value>,
    pub estimate: Ask,
    pub models: &'a [String],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Miss {
    pub model: String,
    pub problem: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reply {
    pub text: String,
    pub model: String,
    pub status: String,
    pub complete: bool,
    pub usage: Usage,
    pub requests: u32,
    pub hops: usize,
    pub seconds: f64,
    pub misses: Vec<Miss>,
}

pub struct Gemini<'a> {
    pub settings: &'a config::Gemini,
    pub guard: &'a Guard<'a>,
    pub clock: &'a dyn Clock,
    pub transport: &'a dyn Transport,
    pub jitter: &'a dyn Fn() -> f64,
}

struct Call {
    interaction: Value,
    model: String,
    tries: u32,
    misses: Vec<Miss>,
}

pub fn jitter() -> f64 {
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |since| since.subsec_nanos());
    f64::from(nanos) / NANOS
}

fn with_model(body: &Map<String, Value>, model: &str) -> String {
    let mut full = Map::new();
    full.insert(MODEL.to_string(), Value::from(model));
    full.extend(body.iter().map(|(name, value)| (name.clone(), value.clone())));
    Value::Object(full).to_string()
}

fn read(text: &str) -> Result<Value, Fail> {
    match serde_json::from_str::<Value>(text) {
        Ok(interaction) if interaction.is_object() => Ok(interaction),
        _ => Err(Fail::hint(Kind::Service, "Gemini sent an answer that cannot be read", "try again later")),
    }
}

fn times(tries: u32) -> String {
    if tries == 1 { "1 try".to_string() } else { format!("{tries} tries") }
}

fn problem(trouble: &Trouble) -> String {
    trouble.status.map_or_else(|| NETWORK_PROBLEM.to_string(), |status| format!("HTTP {status}"))
}

impl Http {
    pub fn new(key: String) -> Http {
        Http { key }
    }

    fn hidden(&self, problem: &ureq::Error) -> String {
        problem.to_string().replace(&self.key, HIDDEN_KEY)
    }
}

impl Transport for Http {
    fn post(&self, body: &str, seconds: f64) -> Result<Sent, String> {
        let wait = Duration::from_secs_f64(seconds.max(SHORTEST_WAIT));
        let request = AGENT.post(ADDRESS).config().timeout_global(Some(wait)).build();
        let request = request.header(KEY_HEADER, &self.key).header("Content-Type", JSON).header("Accept", JSON);
        let mut answer = request.send(body).map_err(|problem| self.hidden(&problem))?;
        let status = answer.status().as_u16();
        let retry_after = answer.headers().get(RETRY_AFTER).and_then(|value| value.to_str().ok()).map(str::to_string);
        let text = answer.body_mut().with_config().limit(u64::MAX).read_to_string();
        Ok(Sent { status, retry_after, text: text.map_err(|problem| self.hidden(&problem))? })
    }
}

impl<T: Transport + ?Sized> Transport for &T {
    fn post(&self, body: &str, seconds: f64) -> Result<Sent, String> {
        (**self).post(body, seconds)
    }
}

impl Gemini<'_> {
    fn error_text(&self, trouble: &Trouble) -> String {
        redact::hide_links(&trouble.message).chars().take(self.settings.error_text_chars).collect()
    }

    fn backoff(&self, tries: u32) -> f64 {
        let retry = &self.settings.retry;
        let grown = retry.first_delay_seconds * GROWTH.powf(f64::from(tries.saturating_sub(1)));
        retry.max_delay_seconds.min(grown) * (STEADY_SHARE + (1.0 - STEADY_SHARE) * (self.jitter)())
    }

    fn after_rate_limit(&self, trouble: &Trouble, scope: &str, tries: u32) -> Result<f64, Fail> {
        let until = self.guard.note_rejection(scope, trouble.retry_seconds, trouble.daily)?;
        for (rule, value) in &trouble.learned {
            self.guard.learn_limit(scope, rule, *value)?;
        }
        if tries >= self.settings.retry.attempts { Err(guard::blocked(scope, until)) } else { Ok(0.0) }
    }

    fn after_outage(&self, trouble: &Trouble, tries: u32, deadline: f64) -> Result<f64, Fail> {
        let delay = trouble.retry_seconds.unwrap_or_else(|| self.backoff(tries));
        if tries < self.settings.retry.attempts && self.clock.now() + delay < deadline {
            return Ok(delay);
        }
        let problem = problem(trouble);
        let text = self.error_text(trouble);
        let message =
            format!("Gemini did not answer ({problem}: {text}) after {}, nothing more was sent", times(tries));
        Err(Fail::hint(Kind::Service, message, "try again in a few minutes"))
    }

    fn refusal(&self, trouble: &Trouble, scope: &str) -> Fail {
        let text = self.error_text(trouble);
        match trouble.class {
            Class::KeyRejected => key::rejected(),
            Class::NotFound => {
                let hint = format!("check the model names under [gemini] in {}", crate::CONFIG_FILE);
                Fail::hint(Kind::Service, format!("Gemini answered 404 for {scope}: {text}"), hint)
            }
            _ => {
                let status = trouble.status.unwrap_or_default();
                let message = format!("Gemini refused the request (HTTP {status}): {text}");
                Fail { status: trouble.status, ..Fail::new(Kind::Rejected, message) }
            }
        }
    }

    fn next_delay(&self, trouble: &Trouble, scope: &str, tries: u32, deadline: f64) -> Result<f64, Fail> {
        match trouble.class {
            Class::RateLimited => self.after_rate_limit(trouble, scope, tries),
            Class::Server | Class::Network => self.after_outage(trouble, tries, deadline),
            Class::KeyRejected | Class::NotFound | Class::Rejected => Err(self.refusal(trouble, scope)),
        }
    }

    fn call(&self, models: &[String], body: &Map<String, Value>, estimate: &Ask, deadline: f64) -> Result<Call, Fail> {
        let mut models: Vec<&str> = models.iter().map(String::as_str).collect();
        let mut overloaded: Vec<String> = Vec::new();
        let mut misses: Vec<Miss> = Vec::new();
        let mut tries = 0;
        loop {
            tries += 1;
            let reservation = self.guard.reserve(&models, estimate)?;
            let scope = reservation.scope.as_str();
            let wait = self.settings.request_timeout_seconds.min((deadline - self.clock.now()).max(SHORTEST_WAIT));
            let trouble = match self.transport.post(&with_model(body, scope), wait) {
                Ok(sent) if (FIRST_GOOD..FIRST_BAD).contains(&sent.status) => {
                    let interaction = read(&sent.text)?;
                    let used = reply::total_tokens(&reply::usage(&interaction));
                    self.guard.settle(&reservation, if used > 0 { used } else { estimate.tokens })?;
                    return Ok(Call { interaction, model: reservation.scope, tries, misses });
                }
                Ok(sent) => trouble::answered(sent.status, sent.retry_after.as_deref(), &sent.text, self.clock.now()),
                Err(problem) => trouble::no_answer(&problem),
            };
            misses.push(Miss { model: scope.to_string(), problem: problem(&trouble) });
            let mut delay = self.next_delay(&trouble, scope, tries, deadline)?;
            if trouble.class == Class::Server {
                overloaded.push(scope.to_string());
                let failed = overloaded.iter().filter(|model| model.as_str() == scope).count();
                let rest: Vec<&str> = models.iter().copied().filter(|model| *model != scope).collect();
                if !rest.is_empty() && failed >= self.settings.retry.tries_per_model {
                    models = rest;
                    delay = 0.0;
                }
            }
            if delay > 0.0 {
                self.clock.sleep(delay);
            }
        }
    }

    fn next_token(&self, call: &Call, hops: usize, deadline: f64) -> Option<String> {
        let token = reply::word(&call.interaction, CONTINUATION);
        let open = reply::word(&call.interaction, STATUS) == reply::REQUIRES_CONTINUATION && !token.is_empty();
        (open && hops < self.settings.max_hops && self.clock.now() < deadline).then(|| token.to_string())
    }

    fn reply(&self, calls: &[Call], started: f64) -> Result<Reply, Fail> {
        let last = calls.last().map(|call| (call.model.as_str(), reply::word(&call.interaction, STATUS)));
        let (model, status) = last.unwrap_or_default();
        let text: String = calls.iter().map(|call| reply::output_text(&call.interaction)).collect();
        if text.is_empty() && status != reply::COMPLETED {
            let named = if status.is_empty() { UNKNOWN } else { status };
            let message = format!("Gemini stopped with status {named} and gave no answer");
            return Err(Fail::hint(Kind::Service, message, "try again later or with a shorter part"));
        }
        let usages: Vec<Usage> = calls.iter().map(|call| reply::usage(&call.interaction)).collect();
        Ok(Reply {
            text,
            model: model.to_string(),
            status: status.to_string(),
            complete: status == reply::COMPLETED,
            usage: reply::merge_usage(&usages),
            requests: calls.iter().map(|call| call.tries).sum(),
            hops: calls.len().saturating_sub(1),
            seconds: self.clock.now() - started,
            misses: calls.iter().flat_map(|call| call.misses.iter().cloned()).collect(),
        })
    }

    pub fn send(&self, request: &Request) -> Result<Reply, Fail> {
        let started = self.clock.now();
        let deadline = started + self.settings.retry.total_seconds;
        let mut models = request.models.to_vec();
        let mut body = request.body.clone();
        let mut calls: Vec<Call> = Vec::new();
        loop {
            let call = match self.call(&models, &body, &request.estimate, deadline) {
                Ok(call) => call,
                Err(_) if !calls.is_empty() => break,
                Err(fail) => return Err(fail),
            };
            let token = self.next_token(&call, calls.len(), deadline);
            models = vec![call.model.clone()];
            calls.push(call);
            let Some(token) = token else { break };
            body = request.body.clone();
            body.insert(CONTINUATION.to_string(), Value::from(token));
        }
        self.reply(&calls, started)
    }
}

#[cfg(test)]
#[path = "gemini_tests.rs"]
mod tests;
