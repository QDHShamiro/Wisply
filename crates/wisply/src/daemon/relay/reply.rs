use std::collections::BTreeMap;

use serde_json::{Value, json};

const CHARS_PER_TOKEN: usize = 4;

pub fn stop_reason(finish: &str, has_tools: bool) -> &'static str {
    if has_tools {
        return "tool_use";
    }
    match finish {
        "length" => "max_tokens",
        "tool_calls" | "function_call" => "tool_use",
        _ => "end_turn",
    }
}

pub fn message_id(seed: u64) -> String {
    format!("msg_relay{seed:016x}")
}

pub fn to_anthropic(resp: &Value, model: &str, seed: u64) -> Value {
    let choice = &resp["choices"][0];
    let message = &choice["message"];
    let mut content = Vec::new();
    if let Some(text) = message["content"].as_str().filter(|t| !t.is_empty()) {
        content.push(json!({"type": "text", "text": text}));
    }
    let calls = message["tool_calls"].as_array().cloned().unwrap_or_default();
    for call in &calls {
        let raw = call["function"]["arguments"].as_str().unwrap_or("{}");
        let input = serde_json::from_str::<Value>(raw).ok().filter(Value::is_object).unwrap_or_else(|| json!({}));
        content.push(json!({
            "type": "tool_use",
            "id": call["id"],
            "name": call["function"]["name"],
            "input": input
        }));
    }
    if content.is_empty() {
        content.push(json!({"type": "text", "text": ""}));
    }
    json!({
        "id": message_id(seed),
        "type": "message",
        "role": "assistant",
        "model": model,
        "content": content,
        "stop_reason": stop_reason(choice["finish_reason"].as_str().unwrap_or(""), !calls.is_empty()),
        "stop_sequence": null,
        "usage": {
            "input_tokens": resp["usage"]["prompt_tokens"].as_u64().unwrap_or(0),
            "output_tokens": resp["usage"]["completion_tokens"].as_u64().unwrap_or(0)
        }
    })
}

fn event(kind: &str, data: &Value) -> String {
    format!("event: {kind}\ndata: {data}\n\n")
}

pub struct Streamer {
    model: String,
    id: String,
    started: bool,
    finished: bool,
    open: Option<usize>,
    text_block: Option<usize>,
    tools: BTreeMap<u64, usize>,
    next: usize,
    finish: String,
    prompt_tokens: u64,
    completion_tokens: u64,
    out_chars: usize,
    estimate: u64,
}

impl Streamer {
    pub fn new(model: &str, seed: u64, prompt_chars: usize) -> Streamer {
        Streamer {
            model: model.to_string(),
            id: message_id(seed),
            started: false,
            finished: false,
            open: None,
            text_block: None,
            tools: BTreeMap::new(),
            next: 0,
            finish: String::new(),
            prompt_tokens: 0,
            completion_tokens: 0,
            out_chars: 0,
            estimate: (prompt_chars / CHARS_PER_TOKEN) as u64,
        }
    }

    fn start(&mut self, out: &mut Vec<String>) {
        if self.started {
            return;
        }
        self.started = true;
        out.push(event(
            "message_start",
            &json!({"type": "message_start", "message": {
                "id": self.id, "type": "message", "role": "assistant", "model": self.model,
                "content": [], "stop_reason": null, "stop_sequence": null,
                "usage": {"input_tokens": self.estimate, "output_tokens": 0}
            }}),
        ));
    }

    fn close(&mut self, out: &mut Vec<String>) {
        if let Some(i) = self.open.take() {
            out.push(event("content_block_stop", &json!({"type": "content_block_stop", "index": i})));
        }
    }

    fn open_block(&mut self, block: Value, out: &mut Vec<String>) -> usize {
        self.close(out);
        let i = self.next;
        self.next += 1;
        self.open = Some(i);
        out.push(event(
            "content_block_start",
            &json!({"type": "content_block_start", "index": i, "content_block": block}),
        ));
        i
    }

    fn text(&mut self, text: &str, out: &mut Vec<String>) {
        let i = match self.text_block.filter(|i| self.open == Some(*i)) {
            Some(i) => i,
            None => {
                let i = self.open_block(json!({"type": "text", "text": ""}), out);
                self.text_block = Some(i);
                i
            }
        };
        self.out_chars += text.len();
        out.push(event(
            "content_block_delta",
            &json!({"type": "content_block_delta", "index": i, "delta": {"type": "text_delta", "text": text}}),
        ));
    }

    fn call(&mut self, call: &Value, position: u64, out: &mut Vec<String>) {
        let slot = call["index"].as_u64().unwrap_or(position);
        let name = call["function"]["name"].as_str().unwrap_or("");
        if !self.tools.contains_key(&slot) && (call["id"].is_string() || !name.is_empty()) {
            let id = call["id"].as_str().map_or_else(|| format!("toolu_relay{slot}"), str::to_string);
            let i = self.open_block(json!({"type": "tool_use", "id": id, "name": name, "input": {}}), out);
            self.tools.insert(slot, i);
        }
        let fragment = call["function"]["arguments"].as_str().unwrap_or("");
        if let (Some(i), false) = (self.tools.get(&slot), fragment.is_empty()) {
            self.out_chars += fragment.len();
            out.push(event(
                "content_block_delta",
                &json!({"type": "content_block_delta", "index": i, "delta": {"type": "input_json_delta", "partial_json": fragment}}),
            ));
        }
    }

    pub fn feed(&mut self, data: &str) -> Vec<String> {
        let mut out = Vec::new();
        if self.finished || data.trim() == "[DONE]" {
            return out;
        }
        let Ok(chunk) = serde_json::from_str::<Value>(data) else { return out };
        self.start(&mut out);
        if let Some(n) = chunk["usage"]["prompt_tokens"].as_u64() {
            self.prompt_tokens = n;
        }
        if let Some(n) = chunk["usage"]["completion_tokens"].as_u64() {
            self.completion_tokens = n;
        }
        let choice = &chunk["choices"][0];
        if let Some(text) = choice["delta"]["content"].as_str().filter(|t| !t.is_empty()) {
            self.text(text, &mut out);
        }
        for (n, call) in choice["delta"]["tool_calls"].as_array().into_iter().flatten().enumerate() {
            self.call(call, n as u64, &mut out);
        }
        if let Some(reason) = choice["finish_reason"].as_str() {
            self.finish = reason.to_string();
        }
        out
    }

    pub fn end(&mut self) -> Vec<String> {
        let mut out = Vec::new();
        if self.finished {
            return out;
        }
        self.finished = true;
        self.start(&mut out);
        if self.next == 0 {
            self.open_block(json!({"type": "text", "text": ""}), &mut out);
        }
        self.close(&mut out);
        let output =
            if self.completion_tokens > 0 { self.completion_tokens } else { (self.out_chars / CHARS_PER_TOKEN) as u64 };
        let reason = stop_reason(&self.finish, !self.tools.is_empty());
        let mut usage = json!({"output_tokens": output});
        if self.prompt_tokens > 0 {
            usage["input_tokens"] = json!(self.prompt_tokens);
        }
        out.push(event(
            "message_delta",
            &json!({"type": "message_delta", "delta": {"stop_reason": reason, "stop_sequence": null}, "usage": usage}),
        ));
        out.push(event("message_stop", &json!({"type": "message_stop"})));
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(events: &[String]) -> Vec<String> {
        events.iter().map(|e| e.lines().next().unwrap().trim_start_matches("event: ").to_string()).collect()
    }

    #[test]
    fn a_plain_answer_maps_text_usage_and_end_turn() {
        let resp = json!({"choices": [{"message": {"content": "hi"}, "finish_reason": "stop"}],
            "usage": {"prompt_tokens": 12, "completion_tokens": 3}});
        let out = to_anthropic(&resp, "claude-sonnet-5-5", 1);
        assert_eq!(out["content"][0]["text"], "hi");
        assert_eq!(out["stop_reason"], "end_turn");
        assert_eq!(out["usage"]["input_tokens"], 12);
        assert_eq!(out["model"], "claude-sonnet-5-5");
    }

    #[test]
    fn tool_calls_become_tool_use_blocks_and_broken_arguments_become_an_empty_object() {
        let resp = json!({"choices": [{"message": {"content": null, "tool_calls": [
            {"id": "c1", "function": {"name": "Bash", "arguments": "{\"command\":\"ls\"}"}},
            {"id": "c2", "function": {"name": "Read", "arguments": "{broken"}}
        ]}, "finish_reason": "stop"}]});
        let out = to_anthropic(&resp, "m", 1);
        assert_eq!(out["stop_reason"], "tool_use");
        assert_eq!(out["content"][0]["input"]["command"], "ls");
        assert_eq!(out["content"][1]["input"], json!({}));
    }

    #[test]
    fn length_maps_to_max_tokens() {
        let resp = json!({"choices": [{"message": {"content": "x"}, "finish_reason": "length"}]});
        assert_eq!(to_anthropic(&resp, "m", 1)["stop_reason"], "max_tokens");
    }

    #[test]
    fn a_streamed_text_then_tool_call_opens_two_blocks_in_order() {
        let mut s = Streamer::new("m", 1, 400);
        let mut all = Vec::new();
        all.extend(s.feed(r#"{"choices":[{"delta":{"content":"he"}}]}"#));
        all.extend(s.feed(r#"{"choices":[{"delta":{"content":"llo"}}]}"#));
        all.extend(s.feed(
            r#"{"choices":[{"delta":{"tool_calls":[{"index":0,"id":"c1","function":{"name":"Bash","arguments":"{\"a\""}}]}}]}"#,
        ));
        all.extend(s.feed(r#"{"choices":[{"delta":{"tool_calls":[{"index":0,"function":{"arguments":":1}"}}]}}]}"#));
        all.extend(s.feed(r#"{"choices":[{"delta":{},"finish_reason":"tool_calls"}],"usage":{"prompt_tokens":9,"completion_tokens":4}}"#));
        all.extend(s.feed("[DONE]"));
        all.extend(s.end());
        assert_eq!(
            kinds(&all),
            [
                "message_start",
                "content_block_start",
                "content_block_delta",
                "content_block_delta",
                "content_block_stop",
                "content_block_start",
                "content_block_delta",
                "content_block_delta",
                "content_block_stop",
                "message_delta",
                "message_stop"
            ]
        );
        let last = all[all.len() - 2].clone();
        assert!(last.contains("\"stop_reason\":\"tool_use\"") && last.contains("\"output_tokens\":4"));
    }

    #[test]
    fn an_empty_stream_still_ends_with_one_empty_text_block() {
        let mut s = Streamer::new("m", 1, 0);
        assert_eq!(
            kinds(&s.end()),
            ["message_start", "content_block_start", "content_block_stop", "message_delta", "message_stop"]
        );
    }
}
