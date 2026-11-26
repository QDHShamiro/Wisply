use std::io::{BufRead, BufReader, Read, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use serde_json::Value;

use super::providers::Provider;
use super::reply::{self, Streamer};
use super::state;
use super::upstream;
use super::wire::{self, Opts};

const LOCAL_FIRST_BYTE: Duration = Duration::from_secs(900);
const CLOUD_FIRST_BYTE: Duration = Duration::from_secs(120);
const DETAIL_CHARS: usize = 160;
const TOO_BIG: [u16; 2] = [413, 422];
const COOL_FAIL_S: u64 = 60;
const COOL_LOCAL_S: u64 = 20;
const COOL_BIG_S: u64 = 300;
const ERROR_MAX: usize = 64 * 1024;

static SEED: AtomicU64 = AtomicU64::new(1);

fn seed() -> u64 {
    state::now().wrapping_mul(2_654_435_761).wrapping_add(SEED.fetch_add(1, Ordering::Relaxed))
}

fn request_body(req: &Value, p: &Provider) -> Value {
    let opts = Opts { vision: p.vision, max_tokens: p.max_output, strip: &p.strip };
    let mut body = wire::to_openai(req, &p.model, &opts);
    if let (Some(add), Some(out)) = (p.extra.as_object(), body.as_object_mut()) {
        out.extend(add.iter().map(|(k, v)| (k.clone(), v.clone())));
    }
    body
}

fn short(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(DETAIL_CHARS).collect()
}

pub fn head(out: &mut impl Write, status: u16, kind: &str, extra: &[(&str, String)]) -> std::io::Result<()> {
    let reason = ureq::http::StatusCode::from_u16(status).ok().and_then(|s| s.canonical_reason()).unwrap_or("OK");
    write!(
        out,
        "HTTP/1.1 {status} {reason}\r\ncontent-type: {kind}\r\nconnection: close\r\ncache-control: no-cache\r\n"
    )?;
    for (k, v) in extra {
        write!(out, "{k}: {v}\r\n")?;
    }
    write!(out, "\r\n")
}

fn stream_reply(out: &mut impl Write, body: impl Read, model: &str, chars: usize) -> std::io::Result<()> {
    head(out, 200, "text/event-stream", &[])?;
    let mut streamer = Streamer::new(model, seed(), chars);
    let mut lines = BufReader::new(body);
    let mut line = String::new();
    loop {
        line.clear();
        match lines.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                if let Some(data) = line.trim_end().strip_prefix("data:") {
                    for event in streamer.feed(data.trim_start()) {
                        out.write_all(event.as_bytes())?;
                    }
                    out.flush()?;
                }
            }
            Err(_) => {
                let err = wire::error_body("overloaded_error", "the fallback model stopped answering");
                out.write_all(format!("event: error\ndata: {err}\n\n").as_bytes())?;
                return out.flush();
            }
        }
    }
    for event in streamer.end() {
        out.write_all(event.as_bytes())?;
    }
    out.flush()
}

fn plain_reply(out: &mut impl Write, mut body: impl Read, model: &str) -> std::io::Result<()> {
    let mut text = String::new();
    body.read_to_string(&mut text)?;
    let resp: Value = serde_json::from_str(&text).unwrap_or(Value::Null);
    let answer = reply::to_anthropic(&resp, model, seed()).to_string();
    head(out, 200, "application/json", &[("content-length", answer.len().to_string())])?;
    out.write_all(answer.as_bytes())?;
    out.flush()
}

fn cool_for(p: &Provider, status: u16, retry: Option<&str>, text: &str) -> u64 {
    if status == 429 {
        return state::retry_wait(retry);
    }
    let big = TOO_BIG.contains(&status) || text.to_lowercase().contains("context length");
    if big {
        COOL_BIG_S
    } else if p.local {
        COOL_LOCAL_S
    } else {
        COOL_FAIL_S
    }
}

pub fn run(req: &Value, order: &[&Provider], out: &mut impl Write) -> Result<String, String> {
    let model = req["model"].as_str().unwrap_or("claude").to_string();
    let stream = req["stream"] == true;
    let chars = wire::prompt_chars(req);
    let mut failures = Vec::new();
    for p in order {
        let body = request_body(req, p);
        let wait = if p.local { LOCAL_FIRST_BYTE } else { CLOUD_FIRST_BYTE };
        let mut call = upstream::agent()
            .post(&p.url)
            .config()
            .timeout_recv_response(Some(wait))
            .build()
            .header("content-type", "application/json");
        if let Some(key) = &p.key {
            call = call.header("authorization", format!("Bearer {key}"));
        }
        let mut resp = match call.send(body.to_string().as_bytes()) {
            Ok(r) => r,
            Err(e) => {
                state::cool(&p.name, if p.local { COOL_LOCAL_S } else { COOL_FAIL_S });
                state::count(&p.name, false);
                failures.push(format!("{}: not reachable ({})", p.name, short(&e.to_string())));
                continue;
            }
        };
        let status = resp.status().as_u16();
        if status != 200 {
            let retry = resp.headers().get("retry-after").and_then(|v| v.to_str().ok()).map(String::from);
            let mut text = String::new();
            let _ = resp.body_mut().as_reader().take(ERROR_MAX as u64).read_to_string(&mut text);
            state::cool(&p.name, cool_for(p, status, retry.as_deref(), &text));
            state::count(&p.name, false);
            failures.push(format!("{} answered {status}: {}", p.name, short(&text)));
            continue;
        }
        state::count(&p.name, true);
        let reader = resp.into_body().into_reader();
        let _ = if stream { stream_reply(out, reader, &model, chars) } else { plain_reply(out, reader, &model) };
        return Ok(p.name.clone());
    }
    Err(failures.join("; "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn extra_fields_of_the_provider_land_in_the_request_and_the_output_is_capped() {
        let p = Provider {
            name: "x".into(),
            url: String::new(),
            model: "m".into(),
            key_env: String::new(),
            key: None,
            max_prompt: 1,
            max_output: 100,
            vision: false,
            local: false,
            code_ok: true,
            strip: vec![],
            extra: json!({"reasoning_effort": "low"}),
        };
        let body = request_body(&json!({"max_tokens": 5000, "messages": []}), &p);
        assert_eq!((body["reasoning_effort"].as_str(), body["max_tokens"].as_u64()), (Some("low"), Some(100)));
        assert_eq!(body["model"], "m");
    }

    #[test]
    fn the_head_names_the_status_and_closes_the_connection() {
        let mut buf = Vec::new();
        head(&mut buf, 529, "application/json", &[("x-a", "1".into())]).unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert!(
            text.starts_with("HTTP/1.1 529 ") && text.contains("connection: close") && text.contains("x-a: 1\r\n\r\n")
        );
    }
}
