use super::*;
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("wisply-reads-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn source(name: &str, fns: usize) -> String {
    let mut s = format!("// module {name}\nuse std::fmt;\n\n");
    for i in 0..fns {
        s.push_str(&format!(
            "pub fn handler_{i}(input: &str) -> usize {{\n    let mut total = {i};\n    for c in input.chars() {{\n        total += c as usize;\n    }}\n    total\n}}\n\n"
        ));
    }
    s
}

fn read_event(sid: &str, path: &Path, content: &str, ranged: bool) -> Value {
    let n = content.lines().count();
    let input = if ranged { json!({"file_path": path, "offset": 1, "limit": n}) } else { json!({"file_path": path}) };
    json!({
        "session_id": sid, "tool_name": "Read", "tool_input": input,
        "tool_response": {"type": "text", "file": {"filePath": path, "content": content, "numLines": n, "startLine": 1, "totalLines": n}}
    })
}

fn run(state: &Path, mut event: Value) -> (Option<Value>, Map<String, Value>) {
    let mut entry = Map::new();
    let reply = post_tool(state, &mut event, &mut entry).map(|r| serde_json::from_str::<Value>(&r).unwrap());
    (reply, entry)
}

fn text_of(reply: &Value) -> String {
    reply["hookSpecificOutput"]["updatedToolOutput"]["file"]["content"].as_str().unwrap().to_string()
}

#[test]
fn second_unchanged_read_becomes_a_stub() {
    let dir = scratch("stub");
    let file = dir.join("a.rs");
    let body = source("a", 8);
    fs::write(&file, &body).unwrap();
    let state = dir.join("headroom");
    assert!(run(&state, read_event("s1", &file, &body, true)).0.is_none());
    let (reply, entry) = run(&state, read_event("s1", &file, &body, true));
    let text = text_of(&reply.unwrap());
    assert!(text.contains("unchanged since your read"), "{text}");
    assert!(text.contains("wisply more hr-"));
    assert_eq!(entry["act"], "stub");
}

#[test]
fn edit_clears_the_file_and_changed_content_never_stubs() {
    let dir = scratch("edit");
    let file = dir.join("b.rs");
    let body = source("b", 8);
    fs::write(&file, &body).unwrap();
    let state = dir.join("headroom");
    run(&state, read_event("s2", &file, &body, true));
    forget(&state, &json!({"session_id": "s2", "tool_name": "Edit", "tool_input": {"file_path": file}}));
    assert!(run(&state, read_event("s2", &file, &body, true)).0.is_none());
    let changed = body.replace("total = 1;", "total = 2;");
    fs::write(&file, &changed).unwrap();
    assert!(run(&state, read_event("s2", &file, &changed, true)).0.is_none());
    let same_again = run(&state, read_event("s2", &file, &changed, true)).0;
    assert!(same_again.is_some());
    fs::write(&file, &body).unwrap();
    assert!(run(&state, read_event("s2", &file, &body, true)).0.is_none());
}

#[test]
fn compact_and_other_sessions_reset_the_table() {
    let dir = scratch("compact");
    let file = dir.join("c.rs");
    let body = source("c", 8);
    fs::write(&file, &body).unwrap();
    let state = dir.join("headroom");
    run(&state, read_event("s3", &file, &body, true));
    assert!(run(&state, read_event("other", &file, &body, true)).0.is_none());
    other_event(&state, &json!({"session_id": "s3", "hook_event_name": "PreCompact"}));
    assert!(run(&state, read_event("s3", &file, &body, true)).0.is_none());
}

#[test]
fn big_whole_file_is_cut_once_then_given_in_full() {
    let dir = scratch("cut");
    let file = dir.join("d.rs");
    let body = source("d", 110);
    fs::write(&file, &body).unwrap();
    let state = dir.join("headroom");
    let (reply, entry) = run(&state, read_event("s4", &file, &body, false));
    let text = text_of(&reply.unwrap());
    assert!(text.contains("--- outline") && text.contains("x, every 8 lines: pub fn handler_"), "{text}");
    assert!(text.len() < body.len() / 2);
    assert_eq!(entry["act"], "cut");
    assert!(run(&state, read_event("s4", &file, &body, false)).0.is_none());
    let (stub, _) = run(&state, read_event("s4", &file, &body, false));
    assert!(text_of(&stub.unwrap()).contains("unchanged"));
}

#[test]
fn ranges_small_files_and_edited_files_are_not_cut() {
    let dir = scratch("nocut");
    let file = dir.join("e.rs");
    let body = source("e", 110);
    fs::write(&file, &body).unwrap();
    let state = dir.join("headroom");
    assert!(run(&state, read_event("s5", &file, &body, true)).0.is_none());
    let small = dir.join("f.rs");
    let tiny = source("f", 2);
    fs::write(&small, &tiny).unwrap();
    assert!(run(&state, read_event("s5", &small, &tiny, false)).0.is_none());
    forget(&state, &json!({"session_id": "s6", "tool_name": "Write", "tool_input": {"file_path": file}}));
    assert!(run(&state, read_event("s6", &file, &body, false)).0.is_none());
}

#[test]
fn cat_of_a_whole_file_is_cut_and_stubbed() {
    let dir = scratch("cat");
    let file = dir.join("g.rs");
    let body = source("g", 110);
    fs::write(&file, &body).unwrap();
    let state = dir.join("headroom");
    let event = json!({
        "session_id": "s7", "tool_name": "Bash", "cwd": dir, "tool_input": {"command": "cat g.rs"},
        "tool_response": {"stdout": body, "stderr": "", "interrupted": false, "isImage": false}
    });
    let (reply, entry) = run(&state, event.clone());
    let out = reply.unwrap()["hookSpecificOutput"]["updatedToolOutput"]["stdout"].as_str().unwrap().to_string();
    assert!(out.contains("--- outline"));
    assert_eq!(entry["kind"], "view");
    let piped = json!({
        "session_id": "s7", "tool_name": "Bash", "cwd": dir, "tool_input": {"command": "cat g.rs | head -5"},
        "tool_response": {"stdout": body, "stderr": "", "interrupted": false, "isImage": false}
    });
    assert!(run(&state, piped).0.is_none());
}

#[test]
fn write_over_a_known_file_gets_a_hint_only() {
    let dir = scratch("write");
    let file = dir.join("h.rs");
    let body = source("h", 8);
    fs::write(&file, &body).unwrap();
    let state = dir.join("headroom");
    let pre = json!({"session_id": "s8", "hook_event_name": "PreToolUse", "tool_name": "Write", "tool_input": {"file_path": file}});
    assert!(other_event(&state, &pre).is_none());
    run(&state, read_event("s8", &file, &body, true));
    let hint: Value = serde_json::from_str(&other_event(&state, &pre).unwrap()).unwrap();
    assert!(hint["hookSpecificOutput"]["additionalContext"].as_str().unwrap().contains("use Edit"));
    assert!(hint["hookSpecificOutput"].get("permissionDecision").is_none());
    let fresh = json!({"session_id": "s8", "hook_event_name": "PreToolUse", "tool_name": "Write", "tool_input": {"file_path": dir.join("new.rs")}});
    assert!(other_event(&state, &fresh).is_none());
}

#[test]
fn each_read_is_written_once_and_a_full_table_keeps_the_newest_entry() {
    let dir = scratch("once");
    let state = dir.join("headroom");
    let first = dir.join("first.rs");
    let body = source("first", 8);
    fs::write(&first, &body).unwrap();
    run(&state, read_event("s9", &first, &body, true));
    let table = state_file(&state, "s9");
    assert_eq!(fs::read_to_string(&table).unwrap().lines().count(), 1);
    for i in 0..crate::readstate::STATE_MAX_LINES as u64 {
        append(&table, &[format!("R	/filler/{i}.rs	1	1	1	1	1	1	0"), "E	/filler/gone.rs".to_string()]);
    }
    let big = dir.join("big.rs");
    let long = source("big", 110);
    fs::write(&big, &long).unwrap();
    let (cut, _) = run(&state, read_event("s9", &big, &long, false));
    assert!(text_of(&cut.unwrap()).contains("--- outline"));
    let lines = fs::read_to_string(&table).unwrap().lines().count();
    assert!(lines <= crate::readstate::STATE_MAX_LINES + 2, "{lines}");
    assert!(run(&state, read_event("s9", &big, &long, false)).0.is_none());
    let (stub, _) = run(&state, read_event("s9", &first, &body, true));
    assert!(text_of(&stub.unwrap()).contains("unchanged"));
}

#[test]
fn parallel_hooks_of_one_session_lose_no_entry() {
    let dir = scratch("parallel");
    let table = state_file(&dir.join("headroom"), "s10");
    let workers: Vec<_> = (0..8)
        .map(|w| {
            let table = table.clone();
            std::thread::spawn(move || {
                for i in 0..40 {
                    append(&table, &[format!("R	/w{w}/{i}.rs	1	1	1	1	1	1	0")]);
                }
            })
        })
        .collect();
    workers.into_iter().for_each(|w| w.join().unwrap());
    assert_eq!(load(&table).seen.len(), 320);
}

#[test]
fn rebuilt_sessions_save_tokens_within_the_time_budget() {
    let dir = scratch("measure");
    let state = dir.join("headroom");
    let names = ["engine", "store", "router"];
    let bodies: Vec<(PathBuf, String)> = names
        .iter()
        .map(|n| {
            let p = dir.join(format!("{n}.rs"));
            let b = source(n, 110);
            fs::write(&p, &b).unwrap();
            (p, b)
        })
        .collect();
    let (mut before, mut after, mut times) = (0usize, 0usize, Vec::<Duration>::new());
    let mut go = |event: Value, content: &str| {
        before += content.len() / 4;
        let t = Instant::now();
        let (reply, _) = run(&state, event);
        times.push(t.elapsed());
        after += reply.map_or(content.len() / 4, |r| text_of(&r).len() / 4);
    };
    for (p, b) in &bodies {
        go(read_event("m", p, b, false), b);
        go(read_event("m", p, b, false), b);
        go(read_event("m", p, b, false), b);
    }
    let (p, b) = &bodies[0];
    forget(&state, &json!({"session_id": "m", "tool_name": "Edit", "tool_input": {"file_path": p}}));
    let edited = b.replace("total = 7;", "total = 700;");
    fs::write(p, &edited).unwrap();
    go(read_event("m", p, &edited, false), &edited);
    go(read_event("m", p, &edited, false), &edited);
    times.sort();
    let (median, worst) = (times[times.len() / 2], times[times.len() - 1]);
    println!(
        "tokens before {before} after {after} saved {}% median call {median:?} worst call {worst:?} over {} calls",
        100 - after * 100 / before,
        times.len()
    );
    assert!(after * 10 < before * 6);
    if !cfg!(debug_assertions) {
        assert!(median < Duration::from_millis(2), "{median:?}");
    }
}

fn cat_event(sid: &str, dir: &Path, name: &str, stdout: &str) -> Value {
    json!({
        "session_id": sid, "tool_name": "Bash", "cwd": dir, "tool_input": {"command": format!("cat {name}")},
        "tool_response": {"stdout": stdout, "stderr": "", "interrupted": false, "isImage": false}
    })
}

#[test]
fn a_lock_file_gets_one_line_once_then_the_whole_text() {
    let dir = scratch("junk");
    let file = dir.join("Cargo.lock");
    let body: String =
        (0..300).map(|i| format!("[[package]]\nname = \"crate-{i}\"\nversion = \"1.0.{i}\"\n\n")).collect();
    fs::write(&file, &body).unwrap();
    let state = dir.join("headroom");
    assert!(run(&state, read_event("j0", &file, &body, true)).0.is_none());
    let (reply, entry) = run(&state, read_event("j1", &file, &body, false));
    let text = text_of(&reply.unwrap());
    assert!(text.starts_with("[wisply] Cargo.lock is a lock file (1200 lines, ~"), "{text}");
    assert!(text.contains("Read it again, or wisply more hr-") && text.lines().count() == 1, "{text}");
    assert_eq!((entry["act"].as_str(), entry["kind"].as_str()), (Some("junk"), Some("read")));
    let kept = state.join("out").join(format!("{}.txt", entry["ref"].as_str().unwrap()));
    assert_eq!(fs::read_to_string(kept).unwrap(), body);
    assert!(run(&state, read_event("j1", &file, &body, false)).0.is_none());
    assert!(text_of(&run(&state, read_event("j1", &file, &body, false)).0.unwrap()).contains("unchanged"));
    let small = dir.join("yarn.lock");
    let few = "left-pad@1.3.0:\n  version \"1.3.0\"\n";
    fs::write(&small, few).unwrap();
    assert!(run(&state, read_event("j1", &small, few, false)).0.is_none());
    forget(&state, &json!({"session_id": "j2", "tool_name": "Edit", "tool_input": {"file_path": file}}));
    assert!(run(&state, read_event("j2", &file, &body, false)).0.is_none());
    let mut capped = read_event("j4", &file, &body, false);
    capped["tool_response"]["file"]["totalLines"] = json!(5000);
    capped["tool_response"]["file"]["truncatedByTokenCap"] = json!(true);
    let (reply, entry) = run(&state, capped.clone());
    let reply = reply.unwrap();
    let text = text_of(&reply);
    assert!(text.starts_with("[wisply] Cargo.lock is a lock file (5000 lines, its first 1200 are ~"), "{text}");
    assert!(text.contains("Those lines: Read it again, or wisply more hr-") && text.lines().count() == 1, "{text}");
    assert_eq!(reply["hookSpecificOutput"]["updatedToolOutput"]["file"]["totalLines"], json!(5000));
    assert_eq!(entry["act"].as_str(), Some("junk"));
    assert!(run(&state, capped).0.is_none());
    fs::write(state.join("junk-off"), "").unwrap();
    assert!(run(&state, read_event("j3", &file, &body, false)).0.is_none());
}

#[test]
fn a_binary_file_shown_as_text_becomes_one_line_every_time() {
    let dir = scratch("binary");
    let state = dir.join("headroom");
    let noise = "PK\u{3}\u{4}\u{fffd}\u{fffd}META-INF\u{0}".repeat(200);
    fs::write(dir.join("plugin.jar"), &noise).unwrap();
    for _ in 0..2 {
        let (reply, entry) = run(&state, cat_event("b1", &dir, "plugin.jar", &noise));
        let out = reply.unwrap()["hookSpecificOutput"]["updatedToolOutput"]["stdout"].as_str().unwrap().to_string();
        assert!(out.starts_with("[wisply] plugin.jar is a compiled or packed file ("), "{out}");
        assert!(out.ends_with("Its file list: unzip -l plugin.jar"), "{out}");
        assert_eq!((entry["act"].as_str(), entry.get("ref")), (Some("junk"), None));
    }
    let words = "plain words in a file that only has a binary name\n".repeat(60);
    fs::write(dir.join("notes.bin"), &words).unwrap();
    assert!(run(&state, cat_event("b1", &dir, "notes.bin", &words)).0.is_none());
}
