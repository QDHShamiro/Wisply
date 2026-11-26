use std::io::Write;
use std::net::TcpListener;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

use serde_json::{Value, json};

use super::server::{Request, read_request};
use super::{serve, state};

const TOOL_REPLY: &str = r#"{"choices":[{"finish_reason":"tool_calls","message":{"content":null,"tool_calls":[{"id":"c1","type":"function","function":{"name":"Bash","arguments":"{\"command\":\"ls\"}"}}]}}],"usage":{"prompt_tokens":11,"completion_tokens":5}}"#;
const STREAM_REPLY: &str = "data: {\"choices\":[{\"delta\":{\"content\":\"hel\"}}]}\n\ndata: {\"choices\":[{\"delta\":{\"content\":\"lo\"},\"finish_reason\":\"stop\"}],\"usage\":{\"prompt_tokens\":4,\"completion_tokens\":2}}\n\ndata: [DONE]\n\n";

fn listen() -> (TcpListener, u16) {
    let l = TcpListener::bind("127.0.0.1:0").unwrap();
    let p = l.local_addr().unwrap().port();
    (l, p)
}

fn limited_anthropic(hits: Arc<AtomicUsize>, auth: Arc<std::sync::Mutex<String>>) -> u16 {
    let (l, port) = listen();
    thread::spawn(move || {
        for conn in l.incoming().flatten() {
            if let Ok(req) = read_request(&conn) {
                hits.fetch_add(1, Ordering::SeqCst);
                *auth.lock().unwrap() = req
                    .headers
                    .iter()
                    .find(|(k, _)| k.eq_ignore_ascii_case("authorization"))
                    .map(|(_, v)| v.clone())
                    .unwrap_or_default();
            }
            let _ = (&conn).write_all(
                b"HTTP/1.1 429 Too Many Requests\r\nretry-after: 7\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}",
            );
        }
    });
    port
}

fn openai(seen: Arc<std::sync::Mutex<Vec<Value>>>) -> u16 {
    let (l, port) = listen();
    thread::spawn(move || {
        for conn in l.incoming().flatten() {
            let Ok(req) = read_request(&conn) else { continue };
            let body: Value = serde_json::from_slice(&req.body).unwrap_or(Value::Null);
            let stream = body["stream"] == true;
            seen.lock().unwrap().push(body);
            let (kind, text) =
                if stream { ("text/event-stream", STREAM_REPLY) } else { ("application/json", TOOL_REPLY) };
            let head = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: {kind}\r\ncontent-length: {}\r\nconnection: close\r\n\r\n",
                text.len()
            );
            let _ = (&conn).write_all(head.as_bytes());
            let _ = (&conn).write_all(text.as_bytes());
        }
    });
    port
}

fn call(home: &std::path::Path, body: Value) -> (String, Value) {
    let req = Request {
        method: "POST".into(),
        path: "/v1/messages".into(),
        headers: vec![("authorization".into(), "Bearer subscription-token".into())],
        body: body.to_string().into_bytes(),
    };
    let mut out = Vec::new();
    serve::handle(home, &req, &mut out);
    let text = String::from_utf8_lossy(&out).to_string();
    let (head, rest) = text.split_once("\r\n\r\n").unwrap_or((&text, ""));
    (head.to_string(), serde_json::from_str(rest).unwrap_or(Value::String(rest.to_string())))
}

#[test]
fn a_claude_limit_switches_to_the_fallback_and_back_to_claude_after_the_reset() {
    let hits = Arc::new(AtomicUsize::new(0));
    let auth = Arc::new(std::sync::Mutex::new(String::new()));
    let seen = Arc::new(std::sync::Mutex::new(Vec::new()));
    let anthropic = limited_anthropic(hits.clone(), auth.clone());
    let provider = openai(seen.clone());
    let home = std::env::temp_dir().join(format!("wisply-relay-flow-{}", std::process::id()));
    std::fs::create_dir_all(&home).unwrap();
    let custom = json!([{"name": "fake", "url": format!("http://127.0.0.1:{provider}/v1/chat/completions"), "model": "m", "local": true, "max_prompt_tokens": 50000}]);
    let cfg = json!({"fallback_upstream": format!("http://127.0.0.1:{anthropic}"), "fallback_custom": custom.to_string(),
                     "fallback_small_chain": "fake", "fallback_big_chain": "fake"});
    std::fs::write(home.join("config.json"), cfg.to_string()).unwrap();
    state::set_enabled(true);
    state::clear_limit();

    let tools = json!([{"name": "Bash", "description": "run", "input_schema": {"type": "object", "properties": {"command": {"type": "string"}}}}]);
    let ask = json!({"model": "claude-opus-5-5", "max_tokens": 100, "tools": tools,
                     "messages": [{"role": "user", "content": "list files"}]});
    let (head, body) = call(&home, ask.clone());
    assert!(head.starts_with("HTTP/1.1 200"), "{head}");
    assert_eq!(body["content"][0]["type"], "tool_use");
    assert_eq!(body["content"][0]["input"]["command"], "ls");
    assert_eq!(body["stop_reason"], "tool_use");
    assert_eq!(body["model"], "claude-opus-5-5");
    assert_eq!(hits.load(Ordering::SeqCst), 1);
    assert_eq!(auth.lock().unwrap().as_str(), "Bearer subscription-token");
    assert!(state::limited());
    assert_eq!(seen.lock().unwrap()[0]["tools"][0]["function"]["name"], "Bash");

    let mut streamed = ask.clone();
    streamed["stream"] = json!(true);
    streamed["tools"] = json!([]);
    let (head, text) = call(&home, streamed);
    let text = text.as_str().unwrap_or("").to_string();
    assert!(head.contains("text/event-stream"), "{head}");
    assert!(text.contains("message_start") && text.contains("\"hel\"") && text.contains("message_stop"), "{text}");
    assert_eq!(hits.load(Ordering::SeqCst), 1);

    state::clear_limit();
    let _ = call(&home, ask);
    assert_eq!(hits.load(Ordering::SeqCst), 2);
    state::set_enabled(false);
    let _ = std::fs::remove_dir_all(&home);
}
