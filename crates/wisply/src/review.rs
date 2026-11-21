use std::fs;
use std::path::Path;
use std::process::ExitCode;

use serde_json::{Value, json};

use wisply_core::Fail;

use crate::daemon::memory::chat::{self, ChatError};

const FILE_MAX: usize = 12_000;
const SYSTEM: &str = "You are a senior code reviewer. Given a goal and optional code, return concrete, specific improvements. Each improvement names the area, the concrete change, and why it helps. Plain engineering only, no security framing. Reply only as the given JSON schema.";

fn schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "improvements": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "area": {"type": "string"},
                        "idea": {"type": "string"},
                        "why": {"type": "string"}
                    },
                    "required": ["area", "idea", "why"]
                }
            }
        },
        "required": ["improvements"]
    })
}

fn clip(body: &str, max: usize) -> String {
    if body.len() <= max {
        return body.to_string();
    }
    let mut end = max;
    while end > 0 && !body.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}\n... (trimmed)", &body[..end])
}

fn build_user(goal: &str, file: Option<(&str, &str)>) -> String {
    let mut out = format!("Goal: {goal}");
    if let Some((name, body)) = file {
        out.push_str(&format!("\n\nFile {name}:\n{}", clip(body, FILE_MAX)));
    }
    out
}

fn render(raw: &str, who: &str) -> String {
    let parsed: Option<Value> = serde_json::from_str(raw).ok();
    let items = parsed.as_ref().and_then(|v| v.get("improvements")).and_then(Value::as_array);
    let Some(items) = items.filter(|a| !a.is_empty()) else {
        return format!("{who} (local) answered, but with no clear list:\n{}", raw.trim());
    };
    let mut out = format!("{} idea(s) from {who} (local, nothing left your machine):", items.len());
    for (i, it) in items.iter().enumerate() {
        let area = it.get("area").and_then(Value::as_str).unwrap_or("");
        let idea = it.get("idea").and_then(Value::as_str).unwrap_or("");
        let why = it.get("why").and_then(Value::as_str).unwrap_or("");
        out.push_str(&format!("\n\n{}. [{area}] {idea}", i + 1));
        if !why.is_empty() {
            out.push_str(&format!("\n   why: {why}"));
        }
    }
    out
}

fn work(home: &Path, args: &[String]) -> Result<String, Fail> {
    let mut file: Option<String> = None;
    let mut words: Vec<&str> = Vec::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--file" | "-f" => file = it.next().map(|s| s.to_string()),
            other if other.starts_with("--file=") => file = Some(other.trim_start_matches("--file=").to_string()),
            other => words.push(other),
        }
    }
    let goal = words.join(" ");
    let goal = goal.trim();
    if goal.is_empty() && file.is_none() {
        return Err(Fail::hint(
            "review needs a goal, and optionally a file to look at",
            "wisply review \"what can i improve in wisply\"",
        ));
    }
    let Some(p) = chat::provider_local(home) else {
        return Err(Fail::hint(
            "no local model set: point memory_api at your own server so nothing leaves your machine",
            "wisply config memory_api http://127.0.0.1:11434",
        ));
    };
    let body = match &file {
        Some(path) => Some(fs::read_to_string(path).map_err(|e| Fail::new(format!("cannot read {path}: {e}")))?),
        None => None,
    };
    let goal = if goal.is_empty() { "review this file for concrete improvements" } else { goal };
    let user = build_user(goal, file.as_deref().zip(body.as_deref()));
    let who = p.name();
    match chat::chat(&p, SYSTEM, &user, &schema()) {
        Ok(raw) => Ok(render(&raw, &who)),
        Err(ChatError::NotReady(why)) => Err(Fail::new(format!("local model not ready: {why}"))),
        Err(ChatError::Failed(why)) => Err(Fail::new(format!("local review failed: {why}"))),
    }
}

pub fn run(home: &Path, args: &[String]) -> ExitCode {
    match work(home, args) {
        Ok(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Err(fail) => {
            println!("{}", fail.text());
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_prompt_carries_goal_and_trimmed_file() {
        assert_eq!(build_user("make it faster", None), "Goal: make it faster");
        let long = "x".repeat(FILE_MAX + 50);
        let u = build_user("review", Some(("a.rs", &long)));
        assert!(u.starts_with("Goal: review\n\nFile a.rs:\n"));
        assert!(u.contains("... (trimmed)"));
        assert!(u.len() < FILE_MAX + 200);
    }

    #[test]
    fn render_lists_items_and_survives_garbage() {
        let raw = r#"{"improvements":[{"area":"io","idea":"batch writes","why":"fewer syscalls"}]}"#;
        let out = render(raw, "brain");
        assert!(out.starts_with("1 idea(s) from brain (local"), "{out}");
        assert!(out.contains("1. [io] batch writes"));
        assert!(out.contains("why: fewer syscalls"));
        assert!(render("not json", "brain").contains("no clear list"));
        assert!(render(r#"{"improvements":[]}"#, "brain").contains("no clear list"));
    }

    #[test]
    fn schema_requires_the_improvement_fields() {
        let s = schema();
        assert_eq!(s["type"], "object");
        assert_eq!(s["properties"]["improvements"]["items"]["required"], json!(["area", "idea", "why"]));
    }
}
