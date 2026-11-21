use std::time::Duration;

use base64::Engine;
use serde_json::{Value, json};

use crate::config::{Budget, Config, Text};
use crate::dry;
use crate::grammar;
use crate::job::Segment;
use crate::schema::{field_guide, harden};
use crate::timecode;
use crate::video::Picture;

pub const FINISH_DONE: &str = "stop";

#[derive(Debug, Clone)]
pub struct Answer {
    pub text: String,
    pub finish: String,
    pub prompt_tokens: u64,
    pub answer_tokens: u64,
    pub prompt_per_second: f64,
    pub answer_per_second: f64,
    pub probs: Option<Value>,
}

pub fn transcript_block(text: &Text, segments: &[Segment], start: f64, end: f64) -> String {
    let lines: Vec<String> = segments
        .iter()
        .filter(|s| s.end >= start && s.start <= end && !s.text.trim().is_empty())
        .map(|s| text.transcript_line.replace("{time}", &timecode::format(s.start)).replace("{text}", s.text.trim()))
        .collect();
    if lines.is_empty() {
        text.no_transcript.clone()
    } else {
        format!("{}\n{}", text.transcript_header, lines.join("\n"))
    }
}

pub fn estimate_tokens(text: &str, budget: &Budget) -> u32 {
    let chars = text.chars().count() as f64;
    (chars / budget.chars_per_token.max(0.5)).ceil() as u32
}

pub fn fit_chars(tokens: u32, budget: &Budget) -> usize {
    (f64::from(tokens) * budget.chars_per_token.max(0.5)).floor() as usize
}

pub fn frame_budget(
    context: u32,
    max_tokens: u32,
    spare: u32,
    text_tokens: u32,
    per_frame: u32,
    cap: usize,
) -> Result<usize, u32> {
    let used = max_tokens.saturating_add(spare).saturating_add(text_tokens);
    let left = context.saturating_sub(used);
    if per_frame == 0 || left < per_frame {
        return Err(used);
    }
    Ok(((left / per_frame) as usize).min(cap))
}

pub fn minute_cap(cap: usize, seconds: f64, per_minute: f64) -> usize {
    if per_minute <= 0.0 || !seconds.is_finite() {
        return cap;
    }
    let allowed = (seconds.max(0.0) / 60.0 * per_minute).ceil().max(1.0);
    cap.min(allowed as usize)
}

pub fn heard_block(config: &Config, segments: &[Segment], start: f64, end: f64, hears: bool) -> String {
    let block = transcript_block(&config.text, segments, start, end);
    if hears && block == config.text.no_transcript && !config.audio.no_transcript.is_empty() {
        return config.audio.no_transcript.clone();
    }
    block
}

fn timed_cap(schema: &Value, text: &Text) -> Option<u64> {
    let hard = harden(schema, text);
    let pointer = format!("/items/properties/{}", text.time_key);
    hard.get("properties")?
        .as_object()?
        .values()
        .filter(|field| field.get("type").and_then(Value::as_str) == Some("array"))
        .filter(|field| !text.time_key.is_empty() && field.pointer(&pointer).is_some())
        .filter_map(|field| field.get("maxItems").and_then(Value::as_u64))
        .max()
}

pub fn span_text(text: &Text, schema: &Value, start: f64, end: f64) -> String {
    if text.span_rule.is_empty() || end <= start {
        return String::new();
    }
    let mut said = text.span_rule.replace("{start}", &timecode::format(start)).replace("{end}", &timecode::format(end));
    let cap = timed_cap(schema, text).filter(|_| !text.spread_rule.is_empty());
    if let Some(cap) = cap {
        let step = (end - start) / cap.max(1) as f64;
        said.push(' ');
        said.push_str(&text.spread_rule.replace("{step}", &timecode::format(step)));
    }
    said
}

pub fn tail_text(config: &Config, prompt: &str, transcript: &str, schema: &Value, window: (f64, f64)) -> String {
    let guide = field_guide(schema, &config.text);
    let span = span_text(&config.text, schema, window.0, window.1);
    let parts = [transcript, prompt.trim(), guide.as_str(), config.text.answer_rule.as_str(), span.as_str()];
    parts.iter().filter(|p| !p.is_empty()).copied().collect::<Vec<_>>().join("\n\n")
}

pub fn body(config: &Config, model: &str, tail: &str, pictures: &[Picture], schema: &Value) -> Value {
    let engine = base64::engine::general_purpose::STANDARD;
    let mut content =
        if pictures.is_empty() { Vec::new() } else { vec![json!({"type": "text", "text": config.text.frames_header})] };
    for picture in pictures {
        let label = timecode::stamp(&config.text.frame_label, picture.at, config.text.seconds_decimals);
        content.push(json!({"type": "text", "text": label}));
        let url = format!("data:image/jpeg;base64,{}", engine.encode(&picture.jpeg));
        content.push(json!({"type": "image_url", "image_url": {"url": url}}));
    }
    let turn = config.text.question_turn && !pictures.is_empty();
    let messages = if turn {
        json!([
            {"role": "user", "content": content},
            {"role": "user", "content": [{"type": "text", "text": tail}]}
        ])
    } else {
        content.push(json!({"type": "text", "text": tail}));
        json!([{"role": "user", "content": content}])
    };
    let schema = harden(schema, &config.text);
    let mut sent = base(config, model, messages, config.sampling.max_tokens);
    match config.sampling.compact_grammar.then(|| grammar::compact(&schema)).flatten() {
        Some(rules) => sent["grammar"] = json!(rules),
        None => {
            sent["response_format"] =
                json!({"type": "json_schema", "json_schema": {"name": "answer", "schema": schema}})
        }
    }
    sent
}

pub fn base(config: &Config, model: &str, messages: Value, max_tokens: u32) -> Value {
    let sampling = &config.sampling;
    let mut sent = json!({
        "model": model,
        "messages": messages,
        "stream": false,
        "cache_prompt": true,
        "temperature": sampling.temperature,
        "top_p": sampling.top_p,
        "top_k": sampling.top_k,
        "presence_penalty": sampling.presence_penalty,
        "max_tokens": max_tokens,
        "chat_template_kwargs": {"enable_thinking": sampling.enable_thinking},
    });
    if let Some(seed) = sampling.seed {
        sent["seed"] = json!(seed);
    }
    dry::set(&mut sent, sampling, sampling.dry_multiplier);
    sent
}

pub fn add_sound(config: &Config, sent: &mut Value, sound: &[u8]) {
    let turns = sent.get("messages").and_then(Value::as_array).map_or(0, Vec::len);
    let Some(content) = sent.pointer_mut("/messages/0/content").and_then(Value::as_array_mut) else {
        return;
    };
    let engine = base64::engine::general_purpose::STANDARD;
    let at = if turns > 1 { content.len() } else { content.len().saturating_sub(1) };
    let parts = [
        json!({"type": "text", "text": config.audio.header}),
        json!({"type": "input_audio", "input_audio": {"data": engine.encode(sound), "format": config.audio.format}}),
    ];
    content.splice(at..at, parts);
}

pub fn read_answer(value: &Value) -> Option<Answer> {
    let choice = value.get("choices")?.as_array()?.first()?;
    let text = choice.get("message")?.get("content")?.as_str().unwrap_or("").to_string();
    let finish = choice.get("finish_reason").and_then(Value::as_str).unwrap_or("").to_string();
    let usage = value.get("usage");
    let count = |key: &str| usage.and_then(|u| u.get(key)).and_then(Value::as_u64).unwrap_or(0);
    let timings = value.get("timings");
    let rate = |key: &str| timings.and_then(|t| t.get(key)).and_then(Value::as_f64).unwrap_or(0.0);
    Some(Answer {
        text,
        finish,
        prompt_tokens: count("prompt_tokens"),
        answer_tokens: count("completion_tokens"),
        prompt_per_second: rate("prompt_per_second"),
        answer_per_second: rate("predicted_per_second"),
        probs: choice.get("logprobs").and_then(|l| l.get("content")).filter(|c| c.is_array()).cloned(),
    })
}

pub fn add_probs(sent: &mut Value, top: u32) {
    sent["logprobs"] = json!(true);
    sent["top_logprobs"] = json!(top);
}

pub fn agent(timeout: Duration) -> ureq::Agent {
    ureq::Agent::new_with_config(
        ureq::Agent::config_builder().timeout_global(Some(timeout)).http_status_as_error(false).proxy(None).build(),
    )
}

pub fn ask(config: &Config, body: &Value) -> Result<Answer, String> {
    let server = &config.server;
    let url = format!("http://{}:{}/v1/chat/completions", server.host, server.port);
    let agent = agent(Duration::from_secs(server.request_timeout_seconds));
    let mut response = agent.post(&url).send_json(body).map_err(|e| e.to_string())?;
    let status = response.status().as_u16();
    let raw = response.body_mut().with_config().limit(64 * 1024 * 1024).read_to_string().map_err(|e| e.to_string())?;
    if status != 200 {
        let short: String = raw.chars().take(400).collect();
        return Err(format!("HTTP {status}: {short}"));
    }
    let value: Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    read_answer(&value).ok_or_else(|| raw.chars().take(400).collect())
}

#[cfg(test)]
#[path = "chat_tests.rs"]
mod tests;
