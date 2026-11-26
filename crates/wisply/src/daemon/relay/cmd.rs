use std::path::Path;
use std::time::Instant;

use serde_json::{Value, json};
use wisply_core::{Call, Fail};

use super::super::config;
use super::fallback;
use super::pick::{self, Need};
use super::providers::{self, DEFAULT_BIG_CHAIN, DEFAULT_SMALL_CHAIN};
use super::server;
use super::state;

const SIGNUP: &[(&str, &str)] = &[
    ("groq", "https://console.groq.com/keys"),
    ("openrouter", "https://openrouter.ai/settings/keys"),
    ("nvidia", "https://build.nvidia.com/settings/api-keys"),
    ("gemini", "https://aistudio.google.com/apikey"),
];
const KEY_HELP: &str = "put KEY=value lines into ~/.config/watch/.env (never into a repo)";
const TEST_PROMPT: &str = "Reply with the single word: ok";
const DEFAULT_ALLOW_MIN: u64 = 60;

fn usage() -> Fail {
    Fail::hint("fallback takes on, off, status, keys, test or local", "wisply fallback status")
}

fn set(home: &Path, on: bool) -> Result<(), Fail> {
    config::command(home, &["fallback".to_string(), if on { "on" } else { "off" }.to_string()]).map(|_| ())
}

fn wait_text(secs: u64) -> String {
    if secs >= 3600 {
        format!("{}h {}m", secs / 3600, secs % 3600 / 60)
    } else if secs >= 60 {
        format!("{}m", secs / 60)
    } else {
        format!("{secs}s")
    }
}

fn policy_word(policy: &str) -> &'static str {
    match policy.trim() {
        "always" => "always allowed",
        "never" => "never",
        _ => "asks first (wisply fallback local)",
    }
}

fn allow(c: &Call) -> Result<String, Fail> {
    match c.pos.get(1).map(String::as_str) {
        Some("off") => {
            state::revoke_local();
            Ok("local model during a game: asks first again".to_string())
        }
        Some(word) => {
            let minutes: u64 = word
                .parse()
                .ok()
                .filter(|m| *m > 0)
                .ok_or_else(|| Fail::hint("local takes minutes or off", "wisply fallback local 60"))?;
            state::allow_local(minutes * 60);
            Ok(format!("local model allowed during a game for {minutes} minutes"))
        }
        None => {
            state::allow_local(DEFAULT_ALLOW_MIN * 60);
            Ok(format!("local model allowed during a game for {DEFAULT_ALLOW_MIN} minutes"))
        }
    }
}

fn status(home: &Path) -> String {
    let cfg = config::load(home);
    let mut lines = Vec::new();
    let port = server::port();
    lines.push(format!("fallback: {}", if state::enabled() { "on" } else { "off" }));
    lines.push(if port == 0 {
        "relay: not listening".to_string()
    } else {
        format!("relay: http://127.0.0.1:{port}  (free lane: /free)")
    });
    let allowed = state::local_ok_until().saturating_sub(state::now());
    let policy = config::text(&cfg, "fallback_game_local");
    lines.push(format!(
        "game: {}  local model during a game: {}",
        if super::super::game::running().is_empty() { "not running" } else { "running" },
        if allowed > 0 { format!("allowed for {}", wait_text(allowed)) } else { policy_word(&policy).to_string() }
    ));
    lines.push(if state::limited() {
        format!(
            "claude: limit hit, fallback active for {}",
            wait_text(state::limit_until().saturating_sub(state::now()))
        )
    } else {
        "claude: reachable".to_string()
    });
    lines.push(format!(
        "small chain: {}",
        providers::chain(&cfg, "fallback_small_chain", DEFAULT_SMALL_CHAIN).join(" > ")
    ));
    lines.push(format!("big chain: {}", providers::chain(&cfg, "fallback_big_chain", DEFAULT_BIG_CHAIN).join(" > ")));
    for p in providers::all(&cfg) {
        let state_word = if !p.ready() {
            "no key".to_string()
        } else if state::cooling(&p.name) > 0 {
            format!("cooling {}", wait_text(state::cooling(&p.name)))
        } else {
            "ready".to_string()
        };
        let (ok, bad) = state::counts(&p.name);
        lines.push(format!(
            "  {:<11} {:<12} up to {}k tokens  {} ok / {} failed",
            p.name,
            state_word,
            p.max_prompt / 1000,
            ok,
            bad
        ));
    }
    lines.join("\n")
}

fn keys(home: &Path) -> String {
    let cfg = config::load(home);
    let mut lines = vec![KEY_HELP.to_string()];
    for p in providers::all(&cfg).iter().filter(|p| !p.local) {
        let link = SIGNUP.iter().find(|(n, _)| *n == p.name).map_or("", |(_, u)| *u);
        let have = if p.key.is_some() { "set" } else { "missing" };
        lines.push(format!("  {:<11} {:<20} {:<8} {}", p.name, p.key_env, have, link));
    }
    lines.join("\n")
}

fn test(home: &Path, only: Option<&str>) -> Result<String, Fail> {
    let cfg = config::load(home);
    let all = providers::all(&cfg);
    let order: Vec<&providers::Provider> = match only {
        Some(name) => all.iter().filter(|p| p.name == name && p.ready()).collect(),
        None => {
            let need =
                Need { tokens: 10, tools: false, small: true, private: false, game: false, free_only_cloud: false };
            let names = providers::chain(&cfg, "fallback_small_chain", DEFAULT_SMALL_CHAIN);
            pick::order(&all, &names, &need)
        }
    };
    if order.is_empty() {
        return Err(Fail::hint("no ready provider to test", "wisply fallback keys"));
    }
    let req =
        json!({"model": "claude-haiku-5-5", "max_tokens": 16, "messages": [{"role": "user", "content": TEST_PROMPT}]});
    let mut sink = Vec::new();
    let started = Instant::now();
    let name = fallback::run(&req, &order, &mut sink).map_err(|why| Fail::hint(&why, "wisply fallback keys"))?;
    let text = String::from_utf8_lossy(&sink);
    let body: Value = serde_json::from_str(text.split("\r\n\r\n").nth(1).unwrap_or("")).unwrap_or(Value::Null);
    let answer = body["content"][0]["text"].as_str().unwrap_or("").trim().to_string();
    Ok(format!("{name} answered in {} ms: {answer}", started.elapsed().as_millis()))
}

pub fn run(home: &Path, c: &Call) -> Result<String, Fail> {
    match c.pos.first().map(String::as_str).unwrap_or("status") {
        "status" => Ok(status(home)),
        "keys" => Ok(keys(home)),
        "local" => allow(c),
        "test" => test(home, c.pos.get(1).map(String::as_str)),
        "on" => {
            set(home, true)?;
            state::set_enabled(true);
            let port = server::ensure(home).map_err(|e| Fail::hint(&e, "wisply config fallback_port <free port>"))?;
            Ok(format!(
                "fallback on: relay listens on http://127.0.0.1:{port}\nuse it: ANTHROPIC_BASE_URL=http://127.0.0.1:{port} claude\nterminal agents and routines: ANTHROPIC_BASE_URL=http://127.0.0.1:{port}/free"
            ))
        }
        "off" => {
            set(home, false)?;
            state::set_enabled(false);
            Ok("fallback off: the relay only passes requests through to Claude".to_string())
        }
        _ => Err(usage()),
    }
}

pub fn start(home: &Path) {
    if config::on(&config::load(home), "fallback") {
        state::set_enabled(true);
        let _ = server::ensure(home);
    }
}
