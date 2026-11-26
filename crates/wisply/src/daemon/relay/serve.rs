use std::io::{self, Write};
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use super::super::agents::events;
use super::super::config;
use super::super::memory::chain;
use super::fallback;
use super::pick::{self, Game, Need};
use super::providers::{self, DEFAULT_BIG_CHAIN, DEFAULT_SMALL_CHAIN};
use super::server::Request;
use super::state;
use super::upstream::{self, ANTHROPIC};
use super::wire;

const FREE_PREFIX: &str = "/free";
const OVERLOADED: u16 = 529;
const RATE_LIMITED: u16 = 429;
const ASK_GAP_S: u64 = 600;
const ASK_POLL: Duration = Duration::from_secs(2);
const ASK_WAIT_S: u64 = 90;
const ASK_MINUTES: u64 = 60;

enum Tried {
    Done,
    Limited,
    Down,
}

pub fn plain_error(out: &mut impl Write, status: u16, kind: &str, message: &str) -> io::Result<()> {
    let body = wire::error_body(kind, message).to_string();
    fallback::head(out, status, "application/json", &[("content-length", body.len().to_string())])?;
    out.write_all(body.as_bytes())
}

fn upstream_base(home: &Path) -> String {
    let custom = config::text(&config::load(home), "fallback_upstream");
    if custom.trim().is_empty() { ANTHROPIC.to_string() } else { custom.trim().to_string() }
}

fn relay(out: &mut impl Write, resp: upstream::Reply) -> io::Result<()> {
    let status = resp.status().as_u16();
    let headers = upstream::reply_headers(&resp);
    let reason = resp.status().canonical_reason().unwrap_or("OK");
    write!(out, "HTTP/1.1 {status} {reason}\r\nconnection: close\r\n")?;
    for (k, v) in &headers {
        write!(out, "{k}: {v}\r\n")?;
    }
    write!(out, "\r\n")?;
    let mut reader = resp.into_body().into_reader();
    let mut buf = [0u8; 8192];
    loop {
        let n = io::Read::read(&mut reader, &mut buf).unwrap_or(0);
        if n == 0 {
            return out.flush();
        }
        out.write_all(&buf[..n])?;
        out.flush()?;
    }
}

fn try_upstream(home: &Path, req: &Request, path: &str, out: &mut impl Write) -> Tried {
    let guarded = state::enabled();
    match upstream::send(&upstream_base(home), &req.method, path, &req.headers, &req.body) {
        Ok(resp) => {
            let status = resp.status().as_u16();
            if guarded && (status == RATE_LIMITED || status == OVERLOADED) {
                let wait = if status == OVERLOADED {
                    state::OVERLOAD_S
                } else {
                    state::retry_wait(resp.headers().get("retry-after").and_then(|v| v.to_str().ok()))
                };
                state::set_limit(wait);
                return Tried::Limited;
            }
            if relay(out, resp).is_ok() && status < 400 {
                state::clear_limit();
            }
            Tried::Done
        }
        Err(why) => {
            if guarded {
                return Tried::Down;
            }
            let _ = plain_error(out, 502, "api_error", &format!("api.anthropic.com is not reachable: {why}"));
            Tried::Done
        }
    }
}

#[cfg(not(test))]
fn gaming() -> bool {
    !super::super::game::running().is_empty()
}

#[cfg(test)]
fn gaming() -> bool {
    false
}

fn need_for(home: &Path, body: &Value, model: &str) -> Need {
    let words = chain::private_words(home);
    let text = format!("{}{}", body["system"], body["messages"]);
    let tokens = pick::tokens_of(wire::prompt_chars(body));
    Need {
        tokens,
        tools: body["tools"].as_array().is_some_and(|t| !t.is_empty()),
        small: pick::small(model, tokens),
        private: chain::private(&words, &[&text]),
        game: gaming(),
        free_only_cloud: false,
    }
}

fn ask_local(home: &Path, cfg: &serde_json::Map<String, Value>) -> bool {
    if state::should_ask(ASK_GAP_S) {
        events::emit(
            home,
            "waiting",
            None,
            "game running, local model wanted",
            &format!(
                "wisply needs the local model while Claude is limited. Allow it: wisply fallback local {ASK_MINUTES}"
            ),
            true,
        );
    }
    let wait = cfg.get("fallback_ask_wait").and_then(Value::as_u64).filter(|n| *n > 0).unwrap_or(ASK_WAIT_S);
    let until = Instant::now() + Duration::from_secs(wait);
    while Instant::now() < until && !state::local_ok() {
        thread::sleep(ASK_POLL);
    }
    state::local_ok()
}

fn settle_game(
    home: &Path,
    cfg: &serde_json::Map<String, Value>,
    all: &[providers::Provider],
    names: &[String],
    need: &mut Need,
) {
    if !need.game {
        return;
    }
    let open = Need { game: false, ..*need };
    let wanted = pick::order(all, names, &open).iter().any(|p| p.local);
    match pick::game_rule(&config::text(cfg, "fallback_game_local"), state::local_ok(), wanted) {
        Game::Open => need.game = false,
        Game::Blocked => {}
        Game::Ask => need.game = !ask_local(home, cfg),
    }
}

fn answer_with_fallback(home: &Path, body: &Value, out: &mut impl Write) {
    let cfg = config::load(home);
    let all = providers::all(&cfg);
    let model = body["model"].as_str().unwrap_or("");
    let mut need = need_for(home, body, model);
    let key = if need.small {
        ("fallback_small_chain", DEFAULT_SMALL_CHAIN)
    } else {
        ("fallback_big_chain", DEFAULT_BIG_CHAIN)
    };
    let names = providers::chain(&cfg, key.0, key.1);
    settle_game(home, &cfg, &all, &names, &mut need);
    let order = pick::order(&all, &names, &need);
    if order.is_empty() {
        let _ = plain_error(
            out,
            OVERLOADED,
            "overloaded_error",
            &format!("wisply fallback: {}", pick::why_none(&all, &names, &need)),
        );
        return;
    }
    if let Err(details) = fallback::run(body, &order, out) {
        let _ = plain_error(out, OVERLOADED, "overloaded_error", &format!("wisply fallback failed: {details}"));
    }
}

fn messages(home: &Path, req: &Request, path: &str, free: bool, out: &mut impl Write) {
    let Ok(body) = serde_json::from_slice::<Value>(&req.body) else {
        let _ = plain_error(out, 400, "invalid_request_error", "the body is not JSON");
        return;
    };
    if !free {
        let skip = state::enabled() && state::limited();
        if !skip && !matches!(try_upstream(home, req, path, out), Tried::Limited | Tried::Down) {
            return;
        }
    }
    answer_with_fallback(home, &body, out);
}

fn count_tokens(home: &Path, req: &Request, path: &str, free: bool, out: &mut impl Write) {
    let body: Value = serde_json::from_slice(&req.body).unwrap_or(Value::Null);
    if !free {
        let skip = state::enabled() && state::limited();
        if !skip && matches!(try_upstream(home, req, path, out), Tried::Done) {
            return;
        }
    }
    let n = pick::tokens_of(wire::prompt_chars(&body));
    let text = json!({"input_tokens": n}).to_string();
    let _ = fallback::head(out, 200, "application/json", &[("content-length", text.len().to_string())]);
    let _ = out.write_all(text.as_bytes());
}

fn health(out: &mut impl Write) {
    let text = json!({"ok": true, "fallback": state::enabled(), "limited": state::limited()}).to_string();
    let _ = fallback::head(out, 200, "application/json", &[("content-length", text.len().to_string())]);
    let _ = out.write_all(text.as_bytes());
}

pub fn handle(home: &Path, req: &Request, out: &mut impl Write) {
    let (free, path) = match req.path.strip_prefix(FREE_PREFIX) {
        Some(rest) if rest.starts_with('/') => (true, rest),
        _ => (false, req.path.as_str()),
    };
    let route = path.split('?').next().unwrap_or(path);
    match (req.method.as_str(), route) {
        ("GET", "/health") => health(out),
        ("POST", "/v1/messages") => messages(home, req, path, free, out),
        ("POST", "/v1/messages/count_tokens") => count_tokens(home, req, path, free, out),
        _ if free => {
            let _ = plain_error(out, 404, "not_found_error", "the free lane only answers /v1/messages");
        }
        _ => {
            let _ = try_upstream(home, req, path, out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_use_the_anthropic_error_shape() {
        let mut buf = Vec::new();
        plain_error(&mut buf, 529, "overloaded_error", "all busy").unwrap();
        let text = String::from_utf8(buf).unwrap();
        let body: Value = serde_json::from_str(text.split("\r\n\r\n").nth(1).unwrap()).unwrap();
        assert!(text.starts_with("HTTP/1.1 529 "));
        assert_eq!((body["type"].as_str(), body["error"]["type"].as_str()), (Some("error"), Some("overloaded_error")));
        assert_eq!(body["error"]["message"], "all busy");
    }
}
