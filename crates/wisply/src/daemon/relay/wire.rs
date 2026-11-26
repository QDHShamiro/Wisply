use serde_json::{Map, Value, json};

const ALWAYS_STRIPPED: &[&str] = &["$schema"];
const IMAGE_OMITTED: &str = "[image omitted]";
const ERROR_PREFIX: &str = "Error: ";

pub struct Opts<'a> {
    pub vision: bool,
    pub max_tokens: u64,
    pub strip: &'a [String],
}

fn blocks_text(content: &Value) -> String {
    match content {
        Value::String(s) => s.clone(),
        Value::Array(parts) => parts
            .iter()
            .filter(|p| p["type"] == "text")
            .filter_map(|p| p["text"].as_str())
            .collect::<Vec<_>>()
            .join("\n"),
        _ => String::new(),
    }
}

fn cleaned(schema: &Value, strip: &[String]) -> Value {
    match schema {
        Value::Object(map) => Value::Object(
            map.iter()
                .filter(|(k, _)| !ALWAYS_STRIPPED.contains(&k.as_str()) && !strip.iter().any(|s| s == *k))
                .map(|(k, v)| (k.clone(), cleaned(v, strip)))
                .collect::<Map<_, _>>(),
        ),
        Value::Array(items) => Value::Array(items.iter().map(|v| cleaned(v, strip)).collect()),
        other => other.clone(),
    }
}

fn image_part(block: &Value, vision: bool) -> Value {
    let source = &block["source"];
    let url = match source["type"].as_str() {
        Some("base64") => format!(
            "data:{};base64,{}",
            source["media_type"].as_str().unwrap_or("image/png"),
            source["data"].as_str().unwrap_or("")
        ),
        _ => source["url"].as_str().unwrap_or("").to_string(),
    };
    if !vision || url.is_empty() {
        return json!({"type": "text", "text": IMAGE_OMITTED});
    }
    json!({"type": "image_url", "image_url": {"url": url}})
}

fn tool_text(block: &Value) -> String {
    let body = blocks_text(&block["content"]);
    if block["is_error"] == true { format!("{ERROR_PREFIX}{body}") } else { body }
}

fn assistant_message(blocks: &[Value]) -> Option<Value> {
    let text = blocks_text(&Value::Array(blocks.to_vec()));
    let calls: Vec<Value> = blocks
        .iter()
        .filter(|b| b["type"] == "tool_use")
        .map(|b| {
            json!({
                "id": b["id"],
                "type": "function",
                "function": {"name": b["name"], "arguments": b["input"].to_string()}
            })
        })
        .collect();
    if text.is_empty() && calls.is_empty() {
        return None;
    }
    let mut msg = json!({"role": "assistant", "content": if text.is_empty() { Value::Null } else { json!(text) }});
    if !calls.is_empty() {
        msg["tool_calls"] = Value::Array(calls);
    }
    Some(msg)
}

fn user_messages(blocks: &[Value], vision: bool, out: &mut Vec<Value>) {
    for b in blocks.iter().filter(|b| b["type"] == "tool_result") {
        out.push(json!({"role": "tool", "tool_call_id": b["tool_use_id"], "content": tool_text(b)}));
    }
    let parts: Vec<Value> = blocks
        .iter()
        .filter_map(|b| match b["type"].as_str() {
            Some("text") => Some(json!({"type": "text", "text": b["text"]})),
            Some("image") => Some(image_part(b, vision)),
            _ => None,
        })
        .collect();
    if parts.is_empty() {
        return;
    }
    if parts.iter().all(|p| p["type"] == "text") {
        let text: Vec<&str> = parts.iter().filter_map(|p| p["text"].as_str()).collect();
        out.push(json!({"role": "user", "content": text.join("\n")}));
    } else {
        out.push(json!({"role": "user", "content": parts}));
    }
}

fn messages(req: &Value, vision: bool) -> Vec<Value> {
    let mut out = Vec::new();
    let system = blocks_text(&req["system"]);
    if !system.is_empty() {
        out.push(json!({"role": "system", "content": system}));
    }
    for m in req["messages"].as_array().into_iter().flatten() {
        let role = m["role"].as_str().unwrap_or("user");
        match (&m["content"], role) {
            (Value::String(s), _) => out.push(json!({"role": role, "content": s})),
            (Value::Array(blocks), "assistant") => out.extend(assistant_message(blocks)),
            (Value::Array(blocks), _) => user_messages(blocks, vision, &mut out),
            _ => {}
        }
    }
    out
}

fn tools(req: &Value, strip: &[String]) -> Option<Value> {
    let list: Vec<Value> = req["tools"]
        .as_array()?
        .iter()
        .map(|t| {
            json!({
                "type": "function",
                "function": {
                    "name": t["name"],
                    "description": t["description"].as_str().unwrap_or(""),
                    "parameters": cleaned(&t["input_schema"], strip)
                }
            })
        })
        .collect();
    (!list.is_empty()).then(|| Value::Array(list))
}

fn tool_choice(req: &Value) -> Option<Value> {
    let c = req.get("tool_choice")?;
    match c["type"].as_str()? {
        "auto" => Some(json!("auto")),
        "any" => Some(json!("required")),
        "none" => Some(json!("none")),
        "tool" => Some(json!({"type": "function", "function": {"name": c["name"]}})),
        _ => None,
    }
}

pub fn to_openai(req: &Value, model: &str, opts: &Opts) -> Value {
    let mut body = json!({"model": model, "messages": messages(req, opts.vision)});
    if let Some(t) = tools(req, opts.strip) {
        body["tools"] = t;
        if let Some(choice) = tool_choice(req) {
            body["tool_choice"] = choice;
        }
    }
    if let Some(max) = req["max_tokens"].as_u64() {
        body["max_tokens"] = json!(if opts.max_tokens > 0 { max.min(opts.max_tokens) } else { max });
    }
    for key in ["temperature", "top_p"] {
        if let Some(v) = req.get(key).filter(|v| v.is_number()) {
            body[key] = v.clone();
        }
    }
    if let Some(stop) = req.get("stop_sequences").filter(|v| v.as_array().is_some_and(|a| !a.is_empty())) {
        body["stop"] = stop.clone();
    }
    if req["stream"] == true {
        body["stream"] = json!(true);
        body["stream_options"] = json!({"include_usage": true});
    }
    body
}

pub fn prompt_chars(req: &Value) -> usize {
    let mut n = blocks_text(&req["system"]).len();
    for m in req["messages"].as_array().into_iter().flatten() {
        n += m["content"].to_string().len();
    }
    n + req["tools"].to_string().len()
}

pub fn error_body(kind: &str, message: &str) -> Value {
    json!({"type": "error", "error": {"type": kind, "message": message}})
}

#[cfg(test)]
mod tests {
    use super::*;

    fn opts() -> Opts<'static> {
        Opts { vision: false, max_tokens: 0, strip: &[] }
    }

    #[test]
    fn a_tool_round_trip_becomes_assistant_tool_calls_then_tool_messages_then_user_text() {
        let req = json!({
            "system": [{"type": "text", "text": "be brief"}],
            "messages": [
                {"role": "user", "content": "list files"},
                {"role": "assistant", "content": [
                    {"type": "text", "text": "ok"},
                    {"type": "tool_use", "id": "t1", "name": "Bash", "input": {"command": "ls"}}
                ]},
                {"role": "user", "content": [
                    {"type": "tool_result", "tool_use_id": "t1", "content": [{"type": "text", "text": "a.txt"}]},
                    {"type": "text", "text": "thanks"}
                ]}
            ]
        });
        let out = to_openai(&req, "m", &opts());
        let roles: Vec<&str> =
            out["messages"].as_array().unwrap().iter().map(|m| m["role"].as_str().unwrap()).collect();
        assert_eq!(roles, ["system", "user", "assistant", "tool", "user"]);
        assert_eq!(out["messages"][2]["tool_calls"][0]["function"]["arguments"], "{\"command\":\"ls\"}");
        assert_eq!(out["messages"][3]["tool_call_id"], "t1");
        assert_eq!(out["messages"][3]["content"], "a.txt");
    }

    #[test]
    fn tool_schemas_lose_dollar_schema_and_configured_keys_at_every_depth() {
        let req = json!({"messages": [], "tools": [{"name": "t", "input_schema": {
            "$schema": "x", "type": "object", "additionalProperties": false,
            "properties": {"a": {"type": "string", "additionalProperties": false}}
        }}]});
        let strip = vec!["additionalProperties".to_string()];
        let out = to_openai(&req, "m", &Opts { vision: false, max_tokens: 0, strip: &strip });
        let p = &out["tools"][0]["function"]["parameters"];
        assert!(p.get("$schema").is_none() && p.get("additionalProperties").is_none());
        assert!(p["properties"]["a"].get("additionalProperties").is_none());
        assert_eq!(p["type"], "object");
    }

    #[test]
    fn max_tokens_is_capped_and_images_are_dropped_without_vision() {
        let req = json!({"max_tokens": 64000, "stream": true, "messages": [{"role": "user", "content": [
            {"type": "image", "source": {"type": "base64", "media_type": "image/png", "data": "AA"}}
        ]}]});
        let out = to_openai(&req, "m", &Opts { vision: false, max_tokens: 8192, strip: &[] });
        assert_eq!(out["max_tokens"], 8192);
        assert_eq!(out["stream_options"]["include_usage"], true);
        assert_eq!(out["messages"][0]["content"], IMAGE_OMITTED);
    }

    #[test]
    fn tool_choice_any_means_required() {
        let req = json!({"messages": [], "tools": [{"name": "t", "input_schema": {}}], "tool_choice": {"type": "any"}});
        assert_eq!(to_openai(&req, "m", &opts())["tool_choice"], "required");
    }
}
