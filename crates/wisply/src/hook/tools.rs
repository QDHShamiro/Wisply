use super::*;
use std::env;

struct Scratch(Paths, PathBuf);

impl std::ops::Deref for Scratch {
    type Target = Paths;

    fn deref(&self) -> &Paths {
        &self.0
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.1);
    }
}

fn scratch(name: &str) -> Scratch {
    let root = env::temp_dir().join(format!("lt-hook-tools-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    Scratch(Paths::new(&root.join(".wisply")), root)
}

fn session() -> String {
    (0..).map(|i| format!("s{i}")).find(|s| !fnv(&[s.as_str()]).is_multiple_of(HOLDOUT_EVERY)).expect("session id")
}

fn event(tool: &str, input: Value, response: Value) -> Vec<u8> {
    json!({
        "session_id": session(), "tool_name": tool, "tool_use_id": "toolu_9", "tool_input": input,
        "tool_response": response
    })
    .to_string()
    .into_bytes()
}

fn reply_of(paths: &Paths, tool: &str, input: Value, response: Value) -> Option<Value> {
    let (reply, _) = run(&event(tool, input, response), paths, Instant::now(), Duration::from_secs(30));
    reply.map(|r| serde_json::from_str(&r).expect("reply json"))
}

fn grep_text() -> String {
    let mut lines: Vec<String> =
        (0..40).map(|i| format!("crates/app/src/main.rs:{}:    let v{i} = run({i});", i + 1)).collect();
    lines.push("crates/app/src/main.rs:99:    panic!(\"fatal: disk full\");".into());
    lines.join("\n")
}

#[test]
fn a_grep_result_is_grouped_and_the_shape_stays() {
    let paths = scratch("grep");
    let text = grep_text();
    let response = json!({"mode": "content", "numFiles": 0, "filenames": [], "content": text, "numLines": 41});
    let reply = reply_of(&paths, "Grep", json!({"pattern": "run"}), response).expect("reply");
    let updated = &reply["hookSpecificOutput"]["updatedToolOutput"];
    let content = updated["content"].as_str().expect("content");
    assert!(content.starts_with("wisply squeezed 41 lines"), "{content}");
    assert!(content.contains("crates/app/src/main.rs (41)"));
    assert!(content.contains("99:    panic!(\"fatal: disk full\");"));
    assert_eq!((updated["mode"].as_str(), updated["numLines"].as_u64()), (Some("content"), Some(41)));
    let name = kept_ref(content).expect("reference in the header");
    assert_eq!(fs::read_to_string(paths.out.join(format!("{name}.txt"))).expect("full output"), text);
}

#[test]
fn web_pages_lose_boilerplate_and_repeats() {
    let paths = scratch("web");
    let page: Vec<String> = (0..30)
        .map(|i| {
            format!("Section {i} explains how the {i}th feature works and what it needs to run properly in production.")
        })
        .collect();
    let text =
        format!("Skip to main content\nSign in\n{}\n\n{}\nPrivacy Policy", page.join("\n"), page[..12].join("\n"));
    let response = json!({"bytes": 5000, "code": 200, "codeText": "OK", "result": text, "durationMs": 10, "url": "https://x.example"});
    let reply = reply_of(&paths, "WebFetch", json!({"url": "https://x.example"}), response).expect("reply");
    let updated = &reply["hookSpecificOutput"]["updatedToolOutput"];
    let result = updated["result"].as_str().expect("result");
    assert!(result.starts_with("wisply squeezed"), "{result}");
    assert!(!result.contains("Skip to main content") && !result.contains("Privacy Policy"));
    assert_eq!(result.matches("Section 3 explains").count(), 1);
    assert_eq!((updated["code"].as_u64(), updated["url"].as_str()), (Some(200), Some("https://x.example")));
}

#[test]
fn a_search_keeps_its_links_and_gets_one_summary() {
    let paths = scratch("search");
    let part: Vec<String> = (0..30)
        .map(|i| format!("Result {i} says the migration guide covers the {i}th breaking change in detail."))
        .collect();
    let text = format!("Skip to main content\n{}\nSign in\n{}", part.join("\n"), part[..12].join("\n"));
    let response = json!({
        "query": "migration", "durationSeconds": 2.0,
        "results": [{"tool_use_id": "t", "content": [{"title": "A", "url": "https://a"}]}, text]
    });
    let reply = reply_of(&paths, "WebSearch", json!({"query": "migration"}), response).expect("reply");
    let updated = &reply["hookSpecificOutput"]["updatedToolOutput"];
    assert_eq!(updated["results"][0]["content"][0]["url"], "https://a");
    let summary = updated["results"][1].as_str().expect("summary");
    assert!(summary.starts_with("wisply squeezed") && !summary.contains("Sign in"));
    assert_eq!(updated["durationSeconds"], json!(2.0));
}

#[test]
fn short_and_odd_tool_results_are_left_alone() {
    let paths = scratch("small");
    let small = json!({"mode": "content", "content": "a.rs:1:x\na.rs:2:y", "numLines": 2});
    assert!(reply_of(&paths, "Grep", json!({"pattern": "x"}), small).is_none());
    let files = json!({"mode": "files_with_matches", "numFiles": 2, "filenames": ["a", "b"]});
    assert!(reply_of(&paths, "Grep", json!({"pattern": "x"}), files).is_none());
    let reload = json!({"result": "x ".repeat(2000)});
    assert!(reply_of(&paths, "WebFetch", json!({"url": "wisply more hr-1234512345"}), reload).is_none());
}
