use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

use serde_json::Value;

use crate::chat;
use crate::config::Config;

const DATA: &str = "data:";
const DONE: &str = "[DONE]";
const SHORT: usize = 400;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Acc {
    pub text: String,
    pub thought: String,
    pub finish: String,
    pub done: bool,
    pub prompt_tokens: u64,
    pub cached_tokens: u64,
    pub answer_tokens: u64,
    pub prompt_per_second: f64,
    pub answer_per_second: f64,
    pub first_seconds: f64,
    pub stopped: bool,
    pub error: Option<String>,
    pub probs: Vec<Value>,
}

pub enum Piece<'a> {
    Word(&'a str),
    Thought(&'a str),
}

fn timings(value: &Value, acc: &mut Acc) {
    let Some(timings) = value.get("timings") else {
        return;
    };
    let count = |key: &str| timings.get(key).and_then(Value::as_u64).unwrap_or(0);
    let rate = |key: &str| timings.get(key).and_then(Value::as_f64).unwrap_or(0.0);
    acc.prompt_tokens = count("prompt_n");
    acc.cached_tokens = count("cache_n");
    acc.answer_tokens = count("predicted_n");
    acc.prompt_per_second = rate("prompt_per_second");
    acc.answer_per_second = rate("predicted_per_second");
}

pub fn feed(line: &str, acc: &mut Acc) -> String {
    let Some(rest) = line.trim().strip_prefix(DATA) else {
        return String::new();
    };
    let rest = rest.trim();
    if rest == DONE {
        acc.done = true;
        return String::new();
    }
    let Ok(value) = serde_json::from_str::<Value>(rest) else {
        return String::new();
    };
    if let Some(error) = value.get("error") {
        let message = error.get("message").and_then(Value::as_str);
        acc.error = Some(message.map_or_else(|| error.to_string(), str::to_string));
        return String::new();
    }
    let choice = value.get("choices").and_then(Value::as_array).and_then(|list| list.first());
    let part =
        |key: &str| choice.and_then(|c| c.get("delta")).and_then(|d| d.get(key)).and_then(Value::as_str).unwrap_or("");
    let fresh = part("content").to_string();
    acc.thought.push_str(part("reasoning_content"));
    if let Some(finish) = choice.and_then(|c| c.get("finish_reason")).and_then(Value::as_str) {
        acc.finish = finish.to_string();
    }
    if let Some(list) = choice.and_then(|c| c.get("logprobs")).and_then(|l| l.get("content")).and_then(Value::as_array)
    {
        acc.probs.extend(list.iter().cloned());
    }
    timings(&value, acc);
    acc.text.push_str(&fresh);
    fresh
}

pub fn send(config: &Config, body: &Value, show: &mut dyn FnMut(Piece) -> bool) -> Result<Acc, String> {
    let server = &config.server;
    let url = format!("http://{}:{}/v1/chat/completions", server.host, server.port);
    let agent = chat::agent(Duration::from_secs(server.request_timeout_seconds));
    let begun = Instant::now();
    let mut response = agent.post(&url).send_json(body).map_err(|e| e.to_string())?;
    let status = response.status().as_u16();
    if status != 200 {
        let raw = response.body_mut().read_to_string().unwrap_or_default();
        let short: String = raw.chars().take(SHORT).collect();
        return Err(format!("HTTP {status}: {short}"));
    }
    let mut acc = Acc::default();
    let reader = BufReader::new(response.body_mut().as_reader());
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let thought = acc.thought.len();
        let fresh = feed(&line, &mut acc);
        let mut wanted = thought == acc.thought.len() || show(Piece::Thought(&acc.thought[thought..]));
        if !fresh.is_empty() {
            if acc.first_seconds == 0.0 {
                acc.first_seconds = begun.elapsed().as_secs_f64();
            }
            wanted = show(Piece::Word(&fresh)) && wanted;
        }
        if let Some(error) = acc.error.take() {
            return Err(error);
        }
        if !wanted {
            acc.stopped = true;
            break;
        }
        if acc.done {
            break;
        }
    }
    Ok(acc)
}

#[cfg(test)]
mod tests {
    use super::{Acc, feed};

    #[test]
    fn a_stream_adds_up_to_the_answer_and_its_numbers() {
        let lines = [
            r#"data: {"choices":[{"delta":{"role":"assistant","content":null}}]}"#,
            "",
            r#"data: {"choices":[{"delta":{"reasoning_content":"hm"}}]}"#,
            r#"data: {"choices":[{"delta":{"content":"Hal"}}]}"#,
            ": keep-alive",
            r#"data: {"choices":[{"delta":{"content":"lo"}}]}"#,
            r#"data: {"choices":[{"delta":{},"finish_reason":"stop"}],"timings":{"prompt_n":12,"cache_n":30,"predicted_n":2,"prompt_per_second":500.0,"predicted_per_second":58.5}}"#,
            "data: [DONE]",
        ];
        let mut acc = Acc::default();
        let shown: Vec<String> = lines.iter().map(|line| feed(line, &mut acc)).collect();
        assert_eq!(shown.concat(), "Hallo");
        assert_eq!(acc.text, "Hallo");
        assert_eq!(acc.thought, "hm");
        assert_eq!(acc.finish, "stop");
        assert!(acc.done);
        assert_eq!((acc.prompt_tokens, acc.cached_tokens, acc.answer_tokens), (12, 30, 2));
        assert_eq!(acc.answer_per_second, 58.5);
        assert_eq!(acc.error, None);
    }

    #[test]
    fn token_odds_in_the_stream_are_collected_in_order() {
        let mut acc = Acc::default();
        for line in [
            r#"data: {"choices":[{"delta":{"content":"B"},"logprobs":{"content":[{"token":"B","logprob":-0.2}]}}]}"#,
            r#"data: {"choices":[{"delta":{"content":"."}}]}"#,
            r#"data: {"choices":[{"delta":{"content":"!"},"logprobs":{"content":[{"token":"!","logprob":-1.5}]}}]}"#,
        ] {
            feed(line, &mut acc);
        }
        let tokens: Vec<&str> = acc.probs.iter().filter_map(|p| p["token"].as_str()).collect();
        assert_eq!((acc.text.as_str(), tokens), ("B.!", vec!["B", "!"]));
    }

    #[test]
    fn a_server_error_in_the_stream_is_kept() {
        let mut acc = Acc::default();
        feed(r#"data: {"error":{"code":500,"message":"out of memory"}}"#, &mut acc);
        assert_eq!(acc.error.as_deref(), Some("out of memory"));
        let mut acc = Acc::default();
        feed("data: {broken", &mut acc);
        feed("event: ping", &mut acc);
        assert_eq!(acc, Acc::default());
    }
}
