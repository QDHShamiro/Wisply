use crate::{Squeezed, squeeze, squeeze_web};
use serde_json::Value;
use std::collections::HashSet;

fn strings(results: &[Value]) -> Vec<&str> {
    results.iter().filter_map(Value::as_str).collect()
}

pub fn extract(tool: &str, response: &Value) -> Option<String> {
    match tool {
        "Grep" => response.get("content")?.as_str().map(str::to_owned),
        "WebFetch" => response.get("result")?.as_str().map(str::to_owned),
        "WebSearch" => {
            let parts = strings(response.get("results")?.as_array()?);
            (!parts.is_empty()).then(|| parts.join("\n\n"))
        }
        _ => None,
    }
}

pub fn squeeze_tool(tool: &str, text: &str) -> Option<Squeezed> {
    if tool == "Grep" { squeeze(text) } else { squeeze_web(text) }
}

fn dedupe_links(results: &mut [Value]) {
    let mut seen = HashSet::new();
    for item in results.iter_mut() {
        let Some(links) = item.get_mut("content").and_then(Value::as_array_mut) else { continue };
        links.retain(|l| l.get("url").and_then(Value::as_str).is_none_or(|u| seen.insert(u.to_owned())));
    }
}

pub fn apply(tool: &str, response: &mut Value, text: &str) {
    match tool {
        "Grep" => response["content"] = Value::String(text.to_owned()),
        "WebFetch" => response["result"] = Value::String(text.to_owned()),
        "WebSearch" => {
            let Some(results) = response.get_mut("results").and_then(Value::as_array_mut) else { return };
            let mut placed = false;
            results.retain_mut(|r| {
                if !r.is_string() {
                    return true;
                }
                if placed {
                    return false;
                }
                placed = true;
                *r = Value::String(text.to_owned());
                true
            });
            dedupe_links(results);
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn text_is_taken_from_and_put_back_into_each_tool_shape() {
        let mut grep = json!({"mode": "content", "content": "a.rs:1:x", "numLines": 1});
        assert_eq!(extract("Grep", &grep).as_deref(), Some("a.rs:1:x"));
        apply("Grep", &mut grep, "new");
        assert_eq!(grep, json!({"mode": "content", "content": "new", "numLines": 1}));
        let mut fetch = json!({"result": "page", "url": "u", "code": 200});
        apply("WebFetch", &mut fetch, "short");
        assert_eq!(fetch, json!({"result": "short", "url": "u", "code": 200}));
        assert!(extract("Read", &fetch).is_none());
        assert!(extract("Grep", &json!({"filenames": ["a"]})).is_none());
    }

    #[test]
    fn a_search_keeps_its_links_once_and_one_summary() {
        let link = |u: &str| json!({"title": "t", "url": u});
        let mut search = json!({
            "query": "q",
            "results": [
                {"tool_use_id": "x", "content": [link("https://a"), link("https://b"), link("https://a")]},
                "first part",
                {"tool_use_id": "y", "content": [link("https://b"), link("https://c")]},
                "second part"
            ],
            "durationSeconds": 1.5
        });
        assert_eq!(extract("WebSearch", &search).as_deref(), Some("first part\n\nsecond part"));
        apply("WebSearch", &mut search, "short summary");
        assert_eq!(
            search["results"],
            json!([
                {"tool_use_id": "x", "content": [link("https://a"), link("https://b")]},
                "short summary",
                {"tool_use_id": "y", "content": [link("https://c")]}
            ])
        );
        assert_eq!(search["durationSeconds"], json!(1.5));
    }
}
