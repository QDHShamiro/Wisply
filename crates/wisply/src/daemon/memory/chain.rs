use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::{Map, Value, json};
use wisply_memory::{CHUNK_CHARS, now, oneline};

use super::super::{config, game};
use super::chat::{self, GEMINI_URL, Miss, Provider};

const DEFAULT_CHAIN: &str = "gemini,local";
const BOOK: &str = "chain.json";
const DAY_S: f64 = 86_400.0;
const BUSY_S: f64 = 900.0;
const DAILY_S: f64 = 3600.0;
const DOWN_S: f64 = 300.0;
const DOWN_MAX_S: f64 = 3600.0;
const BROKEN_S: f64 = 21_600.0;
const HINT_MIN_S: f64 = 30.0;
const MAX_S: f64 = 21_600.0;
const MIN_CHARS: usize = 1000;
const WHY_CHARS: usize = 120;
const STEM_CHARS: usize = 4;
const TOO_BIG: [u16; 2] = [413, 422];
const TOO_BIG_WORDS: &[&str] = &["too large", "context length", "context_length", "maximum context"];
const DAILY_WORDS: &[&str] = &["per day", "perday", "daily", "(tpd)", "(rpd)"];
const NO_MODEL: &str = "no model to learn with (GEMINI_API_KEY, GROQ_API_KEY, OPENROUTER_API_KEY in wisply config \
memory_chain, or wisply config memory_api + memory_api_model)";

struct Preset {
    name: &'static str,
    url: &'static str,
    model: &'static str,
    key: &'static str,
    chars: usize,
    tokens: u32,
    gap: u64,
    day: u32,
    extra: &'static str,
}

const PRESETS: &[Preset] = &[
    Preset {
        name: "gemini",
        url: GEMINI_URL,
        model: "",
        key: "GEMINI_API_KEY",
        chars: CHUNK_CHARS,
        tokens: 1500,
        gap: 7,
        day: 0,
        extra: r#"{"reasoning_effort": "none"}"#,
    },
    Preset {
        name: "groq",
        url: "https://api.groq.com/openai/v1/chat/completions",
        model: "openai/gpt-oss-120b",
        key: "GROQ_API_KEY",
        chars: 10_000,
        tokens: 2500,
        gap: 60,
        day: 1000,
        extra: r#"{"reasoning_effort": "low"}"#,
    },
    Preset {
        name: "openrouter",
        url: "https://openrouter.ai/api/v1/chat/completions",
        model: "openrouter/free",
        key: "OPENROUTER_API_KEY",
        chars: CHUNK_CHARS,
        tokens: 2500,
        gap: 4,
        day: 50,
        extra: r#"{"provider": {"data_collection": "deny"}}"#,
    },
];

pub struct Planned {
    pub label: String,
    pub link: Result<Provider, String>,
}

pub enum Halt {
    Wait(String, Option<f64>),
    Fail(String),
}

fn entries(cfg: &Map<String, Value>) -> Vec<Value> {
    let named = |s: &str| -> Vec<Value> {
        s.split([',', ' ', '>', ';']).map(str::trim).filter(|w| !w.is_empty() && *w != "-").map(|w| json!(w)).collect()
    };
    match cfg.get("memory_chain") {
        Some(Value::Array(a)) => a.clone(),
        Some(Value::String(s)) if s.trim_start().starts_with('[') => serde_json::from_str(s).unwrap_or_default(),
        Some(Value::String(s)) if !s.trim().is_empty() => named(s),
        _ => named(DEFAULT_CHAIN),
    }
}

fn word(v: &Value, key: &str) -> Option<String> {
    v.get(key).and_then(Value::as_str).map(str::trim).filter(|s| !s.is_empty()).map(String::from)
}

fn number(e: &Value, key: &str, preset: Option<u64>, default: u64) -> u64 {
    e.get(key).and_then(Value::as_u64).or(preset).unwrap_or(default)
}

fn forced_cloud(e: &Value) -> bool {
    e.get("cloud").and_then(Value::as_bool) == Some(true)
}

fn local_link(e: &Value, cfg: &Map<String, Value>) -> Result<Provider, String> {
    let (base, model) = (config::text(cfg, "memory_api"), config::text(cfg, "memory_api_model"));
    if base.trim().is_empty() || model.trim().is_empty() {
        return Err("no memory_api + memory_api_model".into());
    }
    let mut p = Provider::api(&base, &word(e, "model").unwrap_or(model));
    p.local = p.local && !forced_cloud(e);
    Ok(p)
}

fn key_of(e: &Value, preset: Option<&Preset>) -> Result<Option<String>, String> {
    let Some(name) = word(e, "key").or_else(|| preset.map(|p| p.key.to_string())) else { return Ok(None) };
    chat::api_key(&name).map(Some).ok_or(format!("no {name}"))
}

fn remote_link(e: &Value, cfg: &Map<String, Value>, name: &str) -> Result<Provider, String> {
    if name == "gemini" && !config::on(cfg, "gemini") {
        return Err("gemini off".into());
    }
    let preset = PRESETS.iter().find(|p| p.name == name);
    let url = word(e, "url").or_else(|| preset.map(|p| p.url.to_string())).ok_or("no url")?;
    let own = Some(config::text(cfg, "gemini_model")).filter(|m| name == "gemini" && !m.trim().is_empty());
    let model = word(e, "model").or_else(|| preset.map(|p| p.model.to_string()).filter(|m| !m.is_empty()));
    let model = model.or(own).ok_or("no model")?;
    let key = key_of(e, preset)?;
    let extra = e.get("extra").cloned().or_else(|| preset.and_then(|p| serde_json::from_str(p.extra).ok()));
    let chars = number(e, "chars", preset.map(|p| p.chars as u64), CHUNK_CHARS as u64) as usize;
    let tokens = number(e, "tokens", preset.map(|p| u64::from(p.tokens)), u64::from(chat::ANSWER_TOKENS));
    Ok(Provider {
        link: name.to_string(),
        local: preset.is_none() && !forced_cloud(e) && chat::loopback(&url),
        url,
        model,
        key,
        chars: chars.max(MIN_CHARS),
        tokens: tokens as u32,
        gap: Duration::from_secs(number(e, "gap", preset.map(|p| p.gap), 0)),
        day: number(e, "day", preset.map(|p| u64::from(p.day)), 0) as u32,
        extra: extra.unwrap_or(Value::Null),
    })
}

fn build(entry: &Value, cfg: &Map<String, Value>) -> Planned {
    let e = if entry.is_string() { json!({"name": entry}) } else { entry.clone() };
    let label = word(&e, "name").unwrap_or_else(|| "custom".into()).to_lowercase();
    let link =
        if label == "local" && word(&e, "url").is_none() { local_link(&e, cfg) } else { remote_link(&e, cfg, &label) };
    Planned { label, link }
}

pub fn plan(home: &Path) -> Vec<Planned> {
    let cfg = config::load(home);
    let mut out: Vec<Planned> = Vec::new();
    for p in entries(&cfg).iter().map(|e| build(e, &cfg)) {
        if !out.iter().any(|o| o.label == p.label) {
            out.push(p);
        }
    }
    out
}

pub fn links(home: &Path) -> Vec<Provider> {
    plan(home).into_iter().filter_map(|p| p.link.ok()).collect()
}

fn tokens(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_lowercase).collect()
}

pub fn private_words(home: &Path) -> Vec<Vec<String>> {
    let text = config::text(&config::load(home), "memory_private");
    text.split([',', '\n', ';']).map(tokens).filter(|t| !t.is_empty()).collect()
}

pub fn private(words: &[Vec<String>], hays: &[&str]) -> bool {
    !words.is_empty()
        && hays.iter().any(|h| {
            let toks = tokens(h);
            words.iter().any(|w| toks.windows(w.len()).any(|win| hit(w, win)))
        })
}

fn hit(word: &[String], win: &[String]) -> bool {
    let last = word.len().saturating_sub(1);
    word.iter()
        .zip(win)
        .enumerate()
        .all(|(i, (a, b))| a == b || (i == last && a.chars().count() >= STEM_CHARS && b.starts_with(a.as_str())))
}

fn book_path(home: &Path) -> PathBuf {
    home.join("memory").join(BOOK)
}

fn today(t: f64) -> u64 {
    (t / DAY_S).floor().max(0.0) as u64
}

fn load(home: &Path, t: f64) -> Value {
    let raw = fs::read_to_string(book_path(home)).ok().and_then(|s| serde_json::from_str::<Value>(&s).ok());
    let mut book = raw.filter(Value::is_object).unwrap_or_else(|| json!({}));
    if book.get("day").and_then(Value::as_u64) != Some(today(t)) {
        for mark in book.get_mut("links").and_then(Value::as_object_mut).into_iter().flat_map(|m| m.values_mut()) {
            mark["used"] = json!(0);
        }
        book["day"] = json!(today(t));
    }
    book
}

fn save(home: &Path, book: &Value) {
    let path = book_path(home);
    let _ = fs::create_dir_all(path.parent().unwrap_or(home)).and_then(|()| wisply_core::save(&path, book.to_string()));
}

fn mark<'a>(book: &'a mut Value, link: &str) -> &'a mut Value {
    if !book.get("links").is_some_and(Value::is_object) {
        book["links"] = json!({});
    }
    let links = &mut book["links"];
    if !links.get(link).is_some_and(Value::is_object) {
        links[link] = json!({"used": 0, "until": 0.0, "why": "", "misses": 0});
    }
    &mut links[link]
}

fn f(v: &Value, key: &str) -> f64 {
    v.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

pub fn blocked(home: &Path, p: &Provider) -> Option<(f64, String)> {
    let t = now();
    let mut book = load(home, t);
    let m = mark(&mut book, &p.link);
    if f(m, "until") > t {
        return Some((f(m, "until"), m["why"].as_str().unwrap_or("").to_string()));
    }
    let used = f(m, "used") as u32;
    (p.day > 0 && used >= p.day)
        .then(|| ((today(t) + 1) as f64 * DAY_S, format!("{used} of {} asks used today", p.day)))
}

pub fn used_today(home: &Path, p: &Provider) -> u32 {
    let mut book = load(home, now());
    f(mark(&mut book, &p.link), "used") as u32
}

pub fn ok(home: &Path, p: &Provider) {
    let mut book = load(home, now());
    let m = mark(&mut book, &p.link);
    m["used"] = json!(f(m, "used") as u64 + 1);
    m["misses"] = json!(0);
    m["until"] = json!(0.0);
    m["why"] = json!("");
    save(home, &book);
}

pub fn too_big(m: &Miss) -> bool {
    let low = m.why.to_lowercase();
    TOO_BIG.contains(&m.status) || (m.status == 400 && TOO_BIG_WORDS.iter().any(|w| low.contains(w)))
}

pub fn pause_for(m: &Miss, misses: u32) -> Option<f64> {
    let grow = |base: f64, cap: f64| (base * f64::from(1u32 << misses.min(8))).min(cap);
    if too_big(m) || m.status == 200 {
        return None;
    }
    if m.status == 0 || m.status >= 500 {
        return Some(grow(DOWN_S, DOWN_MAX_S));
    }
    if m.status != 429 {
        return Some(BROKEN_S);
    }
    let low = m.why.to_lowercase();
    let daily = DAILY_WORDS.iter().any(|w| low.contains(w));
    Some(match m.retry {
        Some(h) if !daily => h.clamp(HINT_MIN_S, MAX_S),
        Some(h) => h.clamp(DAILY_S, MAX_S),
        None => grow(if daily { DAILY_S } else { BUSY_S }, MAX_S),
    })
}

pub fn miss(home: &Path, p: &Provider, m: &Miss) -> Option<f64> {
    let t = now();
    let mut book = load(home, t);
    let mk = mark(&mut book, &p.link);
    let misses = f(mk, "misses") as u32;
    let pause = pause_for(m, misses)?;
    mk["until"] = json!(t + pause);
    mk["misses"] = json!(misses + 1);
    mk["why"] = json!(oneline(&m.why, WHY_CHARS));
    save(home, &book);
    Some(t + pause)
}

pub struct Links<'a> {
    home: &'a Path,
    pub all: Vec<Provider>,
    skip: Vec<usize>,
    bad: String,
    pub last: Option<usize>,
}

impl<'a> Links<'a> {
    pub fn new(home: &'a Path, all: Vec<Provider>) -> Links<'a> {
        Links { home, all, skip: Vec::new(), bad: String::new(), last: None }
    }

    pub fn pick(&self) -> Result<usize, Halt> {
        let (mut soon, mut why, mut game_on): (Option<f64>, String, Option<String>) = (None, String::new(), None);
        for (i, p) in self.all.iter().enumerate().filter(|(i, _)| !self.skip.contains(i)) {
            if p.local {
                let g = game_on.get_or_insert_with(game::running);
                if !g.is_empty() {
                    if soon.is_none() {
                        why = format!("{g} runs and {} is on this PC", p.link);
                    }
                    continue;
                }
            }
            match blocked(self.home, p) {
                None => return Ok(i),
                Some((until, w)) if soon.is_none_or(|s| until < s) => {
                    soon = Some(until);
                    why = format!("{} paused: {w}", p.link);
                }
                Some(_) => {}
            }
        }
        if soon.is_none() && why.is_empty() {
            let bad = if self.bad.is_empty() { NO_MODEL.to_string() } else { self.bad.clone() };
            return Err(Halt::Fail(bad));
        }
        Err(Halt::Wait(why, soon))
    }

    pub fn ask(&mut self, i: usize, system: &str, user: &str, schema: &Value) -> Option<String> {
        let p = &self.all[i];
        match chat::ask(p, system, user, schema) {
            Ok(answer) => {
                ok(self.home, p);
                self.last = Some(i);
                Some(answer)
            }
            Err(m) if too_big(&m) && p.chars > MIN_CHARS => {
                self.all[i].chars = (p.chars / 2).max(MIN_CHARS);
                None
            }
            Err(m) => {
                miss(self.home, p, &m);
                self.refuse(i, m.why);
                None
            }
        }
    }

    pub fn refuse(&mut self, i: usize, why: String) {
        self.skip.push(i);
        self.bad = why;
    }

    pub fn done(&mut self) {
        self.skip.clear();
        self.bad.clear();
    }
}

pub fn report(home: &Path) -> Vec<String> {
    let t = now();
    plan(home)
        .iter()
        .map(|pl| match &pl.link {
            Err(why) => format!("  {}: off ({why})", pl.label),
            Ok(p) => {
                let state = blocked(home, p).map_or_else(
                    || "ready".to_string(),
                    |(until, why)| format!("paused {} more: {why}", wisply_core::ago((until - t).max(0.0) as u64)),
                );
                let today = match (p.day, used_today(home, p)) {
                    (_, 0) => String::new(),
                    (0, n) => format!(", {n} asks today"),
                    (d, n) => format!(", {n}/{d} asks today"),
                };
                let place = if p.local { "this PC" } else { "cloud" };
                format!("  {}: {} ({place}), {state}{today}", pl.label, p.name())
            }
        })
        .collect()
}

#[cfg(test)]
#[path = "chain_tests.rs"]
mod tests;
