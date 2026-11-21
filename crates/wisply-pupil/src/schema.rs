use serde_json::Value;

use crate::config::Text;

fn guide_lines(value: &Value, parent: &str, text: &Text, lines: &mut Vec<String>) {
    let Some(properties) = value.get("properties").and_then(Value::as_object) else {
        return;
    };
    for (name, field) in properties {
        let path = if parent.is_empty() { name.clone() } else { format!("{parent}{}{name}", text.field_joiner) };
        if let Some(description) = field.get("description").and_then(Value::as_str) {
            lines.push(text.field_line.replace("{name}", &path).replace("{text}", description.trim()));
        }
        guide_lines(field, &path, text, lines);
        if let Some(items) = field.get("items") {
            guide_lines(items, &path, text, lines);
        }
    }
}

pub fn field_guide(schema: &Value, text: &Text) -> String {
    if text.field_line.is_empty() {
        return String::new();
    }
    let mut lines = Vec::new();
    guide_lines(schema, "", text, &mut lines);
    if lines.is_empty() {
        return String::new();
    }
    format!("{}\n{}", text.fields_header, lines.join("\n"))
}

pub fn max_items(description: &str, phrase: &str) -> Option<u64> {
    if phrase.is_empty() {
        return None;
    }
    let lower = description.to_lowercase();
    let at = lower.find(&phrase.to_lowercase())? + phrase.len();
    let digits: String = lower[at..].trim_start().chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok().filter(|n| *n > 0)
}

fn add_limits(value: &mut Value, text: &Text) {
    match value {
        Value::Object(map) => {
            let time_on = !text.time_key.is_empty() && !text.time_pattern.is_empty();
            if time_on
                && let Some(Value::Object(time)) = map.get_mut("properties").and_then(|p| p.get_mut(&text.time_key))
                && time.get("type").and_then(Value::as_str) == Some("string")
                && !time.contains_key("pattern")
            {
                time.insert("pattern".into(), Value::String(text.time_pattern.clone()));
            }
            let limit = (map.get("type").and_then(Value::as_str) == Some("array") && !map.contains_key("maxItems"))
                .then(|| map.get("description").and_then(Value::as_str))
                .flatten()
                .and_then(|d| max_items(d, &text.max_items_phrase));
            if let Some(limit) = limit {
                map.insert("maxItems".into(), Value::from(limit));
            }
            map.values_mut().for_each(|v| add_limits(v, text));
        }
        Value::Array(list) => list.iter_mut().for_each(|v| add_limits(v, text)),
        _ => {}
    }
}

fn require_items(schema: &mut Value) {
    let required: Vec<String> = schema
        .get("required")
        .and_then(Value::as_array)
        .map(|names| names.iter().filter_map(Value::as_str).map(String::from).collect())
        .unwrap_or_default();
    let Some(Value::Object(properties)) = schema.get_mut("properties") else {
        return;
    };
    for name in required {
        if let Some(Value::Object(list)) = properties.get_mut(&name)
            && list.get("type").and_then(Value::as_str) == Some("array")
        {
            let least = list.get("minItems").and_then(Value::as_u64).unwrap_or(0);
            list.insert("minItems".into(), Value::from(least.max(1)));
        }
    }
}

pub fn harden(schema: &Value, text: &Text) -> Value {
    let mut copy = schema.clone();
    add_limits(&mut copy, text);
    require_items(&mut copy);
    copy
}

#[cfg(test)]
pub mod tests {
    use super::{field_guide, harden, max_items};
    use crate::config::Text;
    use crate::grammar;
    use serde_json::json;

    pub fn text() -> Text {
        Text {
            transcript_header: "Said:".into(),
            transcript_line: "[{time}] {text}".into(),
            no_transcript: "None.".into(),
            frames_header: "Frames:".into(),
            frame_label: "[{time}]".into(),
            answer_rule: "JSON only.".into(),
            time_key: "t".into(),
            time_pattern: "^[0-9]{2}:[0-9]{2}$".into(),
            max_items_phrase: "At most".into(),
            fields_header: "Fields:".into(),
            field_line: "- {name}: {text}".into(),
            field_joiner: ".".into(),
            span_rule: "From {start} to {end}.".into(),
            spread_rule: "One every {step}.".into(),
            question_turn: false,
            seconds_decimals: 0,
        }
    }

    #[test]
    fn the_model_is_told_what_every_field_means() {
        let schema = json!({"type": "object", "properties": {
            "sum": {"type": "string", "description": " The gist. "},
            "pts": {"type": "array", "description": "Points.", "items": {"type": "object", "properties": {
                "t": {"type": "string", "description": "Time."},
                "x": {"type": "string", "description": "The point."},
                "raw": {"type": "string"}
            }}}
        }});
        assert_eq!(
            field_guide(&schema, &text()),
            "Fields:\n- sum: The gist.\n- pts: Points.\n- pts.t: Time.\n- pts.x: The point."
        );
        assert_eq!(field_guide(&json!({"type": "string"}), &text()), "");
        let off = Text { field_line: String::new(), ..text() };
        assert_eq!(field_guide(&schema, &off), "");
    }

    #[test]
    fn list_limits_come_from_the_description() {
        assert_eq!(max_items("The key points in order. At most 10.", "At most"), Some(10));
        assert_eq!(max_items("every step. at most 80", "At most"), Some(80));
        assert_eq!(max_items("At most none", "At most"), None);
        assert_eq!(max_items("At most 0.", "At most"), None);
        assert_eq!(max_items("At most 5.", ""), None);
        let schema = json!({"type": "object", "properties": {
            "pts": {"type": "array", "description": "Points. At most 10.", "items": {"type": "string"}},
            "fixed": {"type": "array", "description": "At most 4.", "maxItems": 2},
            "free": {"type": "array", "items": {"type": "string"}}
        }});
        let hard = harden(&schema, &text());
        assert_eq!(hard["properties"]["pts"]["maxItems"], 10);
        assert_eq!(hard["properties"]["fixed"]["maxItems"], 2);
        assert!(hard["properties"]["free"].get("maxItems").is_none());
    }

    #[test]
    fn required_lists_cannot_stay_empty() {
        let schema = json!({"type": "object", "required": ["steps", "sum", "gone"], "properties": {
            "steps": {"type": "array", "items": {"type": "string"}},
            "sum": {"type": "string"},
            "more": {"type": "array", "items": {"type": "string"}},
            "many": {"type": "array", "minItems": 3}
        }});
        let hard = harden(&schema, &text());
        assert_eq!(hard["properties"]["steps"]["minItems"], 1);
        assert!(hard["properties"]["sum"].get("minItems").is_none());
        assert!(hard["properties"]["more"].get("minItems").is_none());
        let wide =
            json!({"type": "object", "required": ["many"], "properties": {"many": {"type": "array", "minItems": 3}}});
        assert_eq!(harden(&wide, &text())["properties"]["many"]["minItems"], 3);
        let plain = json!({"type": "object", "required": ["steps"], "properties": {
            "steps": {"type": "array", "items": {"type": "string"}}
        }});
        let loose = grammar::compact(&plain).expect("grammar");
        let tight = grammar::compact(&harden(&plain, &text())).expect("grammar");
        assert!(loose.contains(r#")? "]""#));
        assert!(!tight.contains(r#")? "]""#));
    }

    #[test]
    fn time_fields_get_a_pattern_everywhere() {
        let schema = json!({
            "type": "object",
            "properties": {
                "sum": {"type": "string"},
                "steps": {"type": "array", "items": {"type": "object", "properties": {
                    "t": {"type": "string"},
                    "do": {"type": "string"}
                }}},
                "at": {"type": "object", "properties": {"t": {"type": "number"}}},
                "keep": {"type": "object", "properties": {"t": {"type": "string", "pattern": "x"}}}
            }
        });
        let hard = harden(&schema, &text());
        let step = &hard["properties"]["steps"]["items"]["properties"];
        assert_eq!(step["t"]["pattern"], "^[0-9]{2}:[0-9]{2}$");
        assert!(step["do"].get("pattern").is_none());
        assert!(hard["properties"]["at"]["properties"]["t"].get("pattern").is_none());
        assert_eq!(hard["properties"]["keep"]["properties"]["t"]["pattern"], "x");
        let keys: Vec<&String> = step.as_object().expect("object").keys().collect();
        assert_eq!(keys, ["t", "do"]);
        let off = Text { time_pattern: String::new(), ..text() };
        assert_eq!(harden(&schema, &off), schema);
    }
}
