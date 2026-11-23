use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use super::super::config;

pub const GEMINI_URL: &str = "https://generativelanguage.googleapis.com/v1beta/openai/chat/completions";
const KEY_NAME: &str = "GEMINI_API_KEY";
const KEY_FILE: [&str; 3] = [".config", "watch", ".env"];
const CLOUD_WAIT: Duration = Duration::from_secs(120);
const LOCAL_WAIT: Duration = Duration::from_secs(600);
const GEMINI_GAP: Duration = Duration::from_secs(7);
pub const ANSWER_TOKENS: u32 = 1500;
const TEMPERATURE: f64 = 0.2;
const DETAIL_CHARS: usize = 200;
const LOCAL_HOSTS: &[&str] = &["://127.0.0.1", "://localhost", "://[::1]"];
const BUSY: [u16; 5] = [429, 500, 502, 503, 504];

static AGENT: LazyLock<ureq::Agent> =
    LazyLock::new(|| ureq::Agent::config_builder().http_status_as_error(false).build().into());
static LAST: LazyLock<Mutex<HashMap<String, Instant>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Debug, Default)]
pub struct Provider {
    pub link: String,
    pub url: String,
    pub model: String,
    pub key: Option<String>,
    pub local: bool,
    pub tokens: u32,
    pub chars: usize,
    pub gap: Duration,
    pub day: u32,
    pub extra: Value,
}

pub enum ChatError {
    NotReady(String),
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct Miss {
    pub status: u16,
    pub retry: Option<f64>,
    pub why: String,
}

impl Miss {
    pub fn busy(&self) -> bool {
        self.status == 0 || BUSY.contains(&self.status)
    }
}

pub fn loopback(url: &str) -> bool {
    LOCAL_HOSTS.iter().any(|h| url.contains(h))
}

pub fn host(url: &str) -> String {
    let host = url.split_once("://").map_or(url, |(_, r)| r).split('/').next().unwrap_or("");
    host.rsplit_once('@').map_or(host, |(_, h)| h).to_string()
}

impl Provider {
    pub fn api(base: &str, model: &str) -> Provider {
        let base = base.trim().trim_end_matches('/');
        let base = base.strip_suffix("/v1").unwrap_or(base);
        Provider {
            link: "local".into(),
            url: format!("{base}/v1/chat/completions"),
            model: model.to_string(),
            local: loopback(base),
            tokens: ANSWER_TOKENS,
            chars: wisply_memory::CHUNK_CHARS,
            extra: json!({"chat_template_kwargs": {"enable_thinking": false}}),
            ..Provider::default()
        }
    }

    pub fn name(&self) -> String {
        if self.link == "gemini" {
            format!("Gemini {}", self.model)
        } else {
            format!("{} at {}", self.model, host(&self.url))
        }
    }
}

pub fn gemini_key() -> Option<String> {
    api_key(KEY_NAME)
}

pub fn api_key(name: &str) -> Option<String> {
    if let Some(k) = env::var(name).ok().filter(|k| !k.trim().is_empty()) {
        return Some(k.trim().to_string());
    }
    let file = KEY_FILE.iter().fold(wisply_proto::user_dir(), |p, part| p.join(part));
    let text = fs::read_to_string(file).ok()?;
    text.lines().find_map(|line| {
        let (k, v) = line.split_once('=')?;
        let v = v.trim().trim_matches(['"', '\'']);
        (k.trim() == name && !v.is_empty()).then(|| v.to_string())
    })
}

pub fn provider_local(home: &Path) -> Option<Provider> {
    let cfg = config::load(home);
    let base = config::text(&cfg, "memory_api");
    let model = config::text(&cfg, "memory_api_model");
    if base.trim().is_empty() || model.trim().is_empty() {
        return None;
    }
    let p = Provider::api(&base, &model);
    p.local.then_some(p)
}

fn wait_gap(link: &str, gap: Duration) {
    if gap.is_zero() {
        return;
    }
    let left = LAST.lock().ok().and_then(|m| m.get(link).map(|at| gap.saturating_sub(at.elapsed())));
    if let Some(left) = left.filter(|l| !l.is_zero()) {
        std::thread::sleep(left);
    }
    if let Ok(mut m) = LAST.lock() {
        m.insert(link.to_string(), Instant::now());
    }
}

pub fn wait_turn() {
    wait_gap("gemini", GEMINI_GAP);
}

pub fn answer_text(raw: &str) -> String {
    let mut out = raw.to_string();
    while let (Some(a), Some(b)) = (out.find("<think>"), out.find("</think>")) {
        if b < a {
            break;
        }
        out.replace_range(a..b + "</think>".len(), "");
    }
    out.trim().to_string()
}

fn seconds(text: &str) -> Option<f64> {
    let (mut total, mut num, mut seen) = (0.0, String::new(), false);
    let mut it = text.chars().peekable();
    while let Some(c) = it.next() {
        if c.is_ascii_digit() || c == '.' {
            num.push(c);
            continue;
        }
        let n: f64 = num.parse().ok()?;
        num.clear();
        let scale = match c {
            'h' => 3600.0,
            'm' if it.peek() == Some(&'s') => {
                it.next();
                0.001
            }
            'm' => 60.0,
            's' => 1.0,
            _ => return None,
        };
        total += n * scale;
        seen = true;
        if it.peek().is_none_or(|c| !c.is_ascii_digit()) {
            break;
        }
    }
    (seen && num.is_empty()).then_some(total)
}

pub fn retry_hint(header: Option<&str>, body: &str) -> Option<f64> {
    if let Some(n) = header.and_then(|h| h.trim().parse::<f64>().ok()) {
        return Some(n);
    }
    let after = |mark: &str| body.find(mark).map(|i| &body[i + mark.len()..]);
    if let Some(rest) = after("\"retryDelay\"") {
        let v = rest.trim_start_matches([':', ' ', '"']);
        let end = v.find('"').unwrap_or(v.len());
        if let Some(s) = seconds(&v[..end]) {
            return Some(s);
        }
    }
    let rest = after("try again in ")?;
    let end = rest.find(|c: char| !(c.is_ascii_alphanumeric() || c == '.')).unwrap_or(rest.len());
    seconds(rest[..end].trim_end_matches('.'))
}

pub fn ask(p: &Provider, system: &str, user: &str, schema: &Value) -> Result<String, Miss> {
    let messages = json!([{"role": "system", "content": system}, {"role": "user", "content": user}]);
    let format = json!({"type": "json_schema", "json_schema": {"name": "memory", "schema": schema}});
    let tokens = if p.tokens == 0 { ANSWER_TOKENS } else { p.tokens };
    let mut body = json!({"model": p.model, "temperature": TEMPERATURE, "max_tokens": tokens,
        "messages": messages, "response_format": format});
    if let (Some(add), Some(out)) = (p.extra.as_object(), body.as_object_mut()) {
        out.extend(add.iter().map(|(k, v)| (k.clone(), v.clone())));
    }
    wait_gap(&p.link, p.gap);
    let wait = if p.local { LOCAL_WAIT } else { CLOUD_WAIT };
    let mut req =
        AGENT.post(&p.url).config().timeout_global(Some(wait)).build().header("Content-Type", "application/json");
    if let Some(key) = &p.key {
        req = req.header("Authorization", format!("Bearer {key}"));
    }
    let miss = |status, retry, why| Miss { status, retry, why };
    let mut resp =
        req.send(&body.to_string()).map_err(|e| miss(0, None, format!("{}: not reachable ({e})", p.name())))?;
    let status = resp.status().as_u16();
    let header = resp.headers().get("retry-after").and_then(|v| v.to_str().ok()).map(String::from);
    let text = resp.body_mut().read_to_string().unwrap_or_default();
    if status != 200 {
        let detail: String = text.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(DETAIL_CHARS).collect();
        let retry = retry_hint(header.as_deref(), &text);
        return Err(miss(status, retry, format!("{} answered {status}: {detail}", p.name())));
    }
    let data: Value = serde_json::from_str(&text).map_err(|_| miss(200, None, format!("{} sent no JSON", p.name())))?;
    let content = data.pointer("/choices/0/message/content").and_then(Value::as_str).unwrap_or("");
    Ok(answer_text(content))
}

pub fn chat(p: &Provider, system: &str, user: &str, schema: &Value) -> Result<String, ChatError> {
    ask(p, system, user, schema)
        .map_err(|m| if m.busy() { ChatError::NotReady(m.why) } else { ChatError::Failed(m.why) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_lose_their_thinking() {
        assert_eq!(answer_text("<think>hmm</think> {\"a\":1} "), "{\"a\":1}");
        assert_eq!(answer_text("plain"), "plain");
        let api = Provider::api("https://u:k@api.example.com/v1/", "brain");
        assert_eq!(
            (api.name().as_str(), api.url.as_str()),
            ("brain at api.example.com", "https://u:k@api.example.com/v1/chat/completions")
        );
        let near = Provider::api("http://127.0.0.1:9930", "brain");
        assert!(!api.local && near.local);
        let gem = Provider { link: "gemini".into(), model: "m".into(), ..Provider::default() };
        assert!(!gem.local && gem.name() == "Gemini m");
    }

    #[test]
    fn busy_models_say_when_to_come_back() {
        assert_eq!(retry_hint(Some("120"), ""), Some(120.0));
        assert_eq!(retry_hint(None, r#"{"details": [{"retryDelay": "34s"}]}"#), Some(34.0));
        assert_eq!(retry_hint(None, "Rate limit reached. Please try again in 7.5s. Need more?"), Some(7.5));
        assert_eq!(retry_hint(None, "Please try again in 1m2.5s."), Some(62.5));
        assert_eq!(retry_hint(None, "Please try again in 2h3m"), Some(7380.0));
        assert_eq!(retry_hint(None, "Please try again in 450ms"), Some(0.45));
        assert_eq!(retry_hint(None, "quota exceeded"), None);
        assert_eq!(retry_hint(None, "Please try again in a bit"), None);
        let m = |status| Miss { status, retry: None, why: String::new() };
        assert!(m(0).busy() && m(429).busy() && m(503).busy() && !m(401).busy() && !m(413).busy());
    }
}
