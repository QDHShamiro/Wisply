use super::*;
use std::env;
use std::fs::File;

const SLOW_CI: Duration = Duration::from_secs(20);

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
    let root = env::temp_dir().join(format!("lt-hook-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    Scratch(Paths::new(&root.join(".wisply")), root)
}

fn session(holdout: bool) -> String {
    (0..)
        .map(|i| format!("s{i}"))
        .find(|s| fnv(&[s.as_str()]).is_multiple_of(HOLDOUT_EVERY) == holdout)
        .expect("session id")
}

fn bash_event(sid: &str, cmd: &str, stdout: &str) -> Vec<u8> {
    json!({
        "session_id": sid, "tool_name": "Bash", "tool_use_id": "toolu_1", "tool_input": {"command": cmd},
        "tool_response": {"stdout": stdout, "stderr": "warn: x", "interrupted": false, "isImage": false}
    })
    .to_string()
    .into_bytes()
}

fn big_log() -> String {
    let mut lines: Vec<String> = (0..400).map(|i| format!("step {i} done in {} ms", i % 17)).collect();
    lines.insert(200, "ERROR upload failed: connection refused".into());
    lines.join("\n")
}

fn logged(paths: &Paths) -> Vec<Value> {
    fs::read_to_string(paths.state.join("log.jsonl"))
        .unwrap_or_default()
        .lines()
        .map(|l| serde_json::from_str(l).expect("log line"))
        .collect()
}

#[test]
fn a_long_log_is_squeezed_and_the_full_text_kept() {
    let paths = scratch("cut");
    let log = big_log();
    let (reply, kept) = run(&bash_event(&session(false), "npm test 2>&1", &log), &paths, Instant::now(), SLOW_CI);
    let reply: Value = serde_json::from_str(&reply.expect("reply")).expect("reply json");
    let out = &reply["hookSpecificOutput"];
    assert_eq!(out["hookEventName"], "PostToolUse");
    let updated = &out["updatedToolOutput"];
    let stdout = updated["stdout"].as_str().expect("stdout");
    assert!(stdout.starts_with("wisply squeezed 401 lines"), "{stdout}");
    assert!(stdout.contains("ERROR upload failed: connection refused"));
    assert!(stdout.chars().count() < log.chars().count());
    assert_eq!((updated["stderr"].as_str(), updated["interrupted"].as_bool()), (Some("warn: x"), Some(false)));
    let name = kept_ref(stdout).expect("reference in the header");
    assert_eq!(fs::read_to_string(paths.out.join(format!("{name}.txt"))).expect("full output"), log);
    assert!(kept);
    let entry = &logged(&paths)[0];
    assert_eq!((entry["act"].as_str(), entry["ref"].as_str()), (Some("cut"), Some(name.as_str())));
    assert_eq!(entry["in"].as_u64(), Some(log.chars().count() as u64));
    assert_eq!(entry["kind"], "log");
}

#[test]
fn holdout_sessions_get_the_full_output_and_are_measured() {
    let paths = scratch("holdout");
    let (reply, kept) = run(&bash_event(&session(true), "npm test", &big_log()), &paths, Instant::now(), SLOW_CI);
    assert!(reply.is_none() && !kept);
    assert!(!paths.out.exists());
    let entry = &logged(&paths)[0];
    assert_eq!(entry["act"], "holdout");
    assert!(entry["out"].as_u64() < entry["in"].as_u64());
}

#[test]
fn exact_reads_listings_and_own_commands_stay_whole() {
    for cmd in [
        "cat src/main.rs",
        "cd /e/x && rtk head -50 crates/a.rs",
        "rtk git diff",
        "git -C /e/Wisply --no-pager show HEAD:a.rs",
        "diff a b",
        "echo x; sed -n '1,80p' SPEC.md",
        "for f in *.rs; do cat $f; done",
        "find . -name '*.java'",
        "ls -la",
        "rtk git log --oneline -50",
        "grep -rl foo src",
        "echo hi\ncat notes.md",
    ] {
        assert_eq!(untouchable(cmd), Some("exact"), "{cmd}");
    }
    for cmd in ["wisply snap", "rtk wisply text -f x", "\"E:/Lighting/bin/lighting.exe\" recall x", "lighting snap"] {
        assert_eq!(untouchable(cmd), Some("own"), "{cmd}");
    }
    for cmd in
        ["npm test", "tail -100 server.log", "cargo build 2>&1 | tail -80", "cat a.log && grep x b.log", "python x.py"]
    {
        assert_eq!(untouchable(cmd), None, "{cmd}");
    }
    let paths = scratch("exact");
    let (reply, _) = run(&bash_event(&session(false), "cat big.rs", &big_log()), &paths, Instant::now(), SLOW_CI);
    assert!(reply.is_none());
    assert_eq!(logged(&paths)[0]["act"], "exact");
}

#[test]
fn small_and_image_outputs_are_left_alone() {
    let paths = scratch("small");
    let (reply, _) = run(&bash_event("s", "npm test", &"x\n".repeat(500)), &paths, Instant::now(), SLOW_CI);
    assert!(reply.is_none());
    let image = json!({"tool_name": "Bash", "tool_input": {"command": "x"}, "tool_response": {"stdout": big_log(), "isImage": true}});
    assert!(run(image.to_string().as_bytes(), &paths, Instant::now(), SLOW_CI).0.is_none());
    let acts: Vec<Value> = logged(&paths).iter().map(|e| e["act"].clone()).collect();
    assert_eq!(acts, [json!("small"), json!("image")]);
}

#[test]
fn reading_a_kept_output_counts_as_a_reload() {
    let paths = scratch("reload");
    let kept = "C:\\Users\\x\\.wisply\\headroom\\out\\hr-0123456789.txt";
    let read = json!({"tool_name": "Read", "session_id": "s", "tool_input": {"file_path": kept}});
    assert!(run(read.to_string().as_bytes(), &paths, Instant::now(), SLOW_CI).0.is_none());
    let grep = bash_event("s", "grep -n ERROR ~/.wisply/headroom/out/hr-abcdefabcd.txt", &big_log());
    assert!(run(&grep, &paths, Instant::now(), SLOW_CI).0.is_none());
    let more = bash_event("s", "wisply more hr-1234512345 --grep ERROR", &big_log());
    assert!(run(&more, &paths, Instant::now(), SLOW_CI).0.is_none());
    let refs: Vec<(Value, Value)> = logged(&paths).iter().map(|e| (e["act"].clone(), e["ref"].clone())).collect();
    let want = ["hr-0123456789", "hr-abcdefabcd", "hr-1234512345"].map(|r| (json!("reload"), json!(r)));
    assert_eq!(refs, want);
    let other = json!({"tool_name": "Read", "tool_input": {"file_path": "src/main.rs"}});
    assert!(run(other.to_string().as_bytes(), &paths, Instant::now(), SLOW_CI).0.is_none());
    assert_eq!(logged(&paths).len(), 3);
}

#[test]
fn three_errors_in_ten_minutes_turn_it_off_once() {
    let paths = scratch("errors");
    assert!(run(b"not json", &paths, Instant::now(), SLOW_CI).0.is_none());
    assert!(run(b"{", &paths, Instant::now(), SLOW_CI).0.is_none());
    assert!(!paths.state.join("off").exists());
    let reply: Value =
        serde_json::from_str(&run(b"", &paths, Instant::now(), SLOW_CI).0.expect("message")).expect("json");
    assert!(reply["systemMessage"].as_str().is_some_and(|m| m.contains("3 errors in 10 minutes")));
    assert!(paths.state.join("off").exists());
}

#[test]
fn tidy_drops_old_and_oversized_kept_outputs() {
    let paths = scratch("tidy");
    fs::create_dir_all(&paths.out).unwrap();
    let old = paths.out.join("hr-old0000000.txt");
    let fresh = paths.out.join("hr-new0000000.txt");
    let foreign = paths.out.join("153012-001-snap.txt");
    for f in [&old, &fresh, &foreign] {
        fs::write(f, "x").unwrap();
    }
    let age = |f: &Path| {
        let long_ago = SystemTime::now() - Duration::from_secs(8 * 86_400);
        File::options().write(true).open(f).unwrap().set_modified(long_ago).unwrap();
    };
    age(&old);
    age(&foreign);
    tidy(&paths);
    assert!(!old.exists() && fresh.exists() && foreign.exists());
    fs::write(&old, "x").unwrap();
    age(&old);
    tidy(&paths);
    assert!(old.exists(), "tidies at most once an hour");
}

struct Stuck;

impl Read for Stuck {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        thread::sleep(SLOW_CI);
        Ok(0)
    }
}

#[test]
fn the_hook_input_is_bounded_in_size_and_time() {
    let short = Duration::from_millis(100);
    assert_eq!(take_input(io::Cursor::new(b"{}".to_vec()), 2, SLOW_CI).as_deref(), Some(&b"{}"[..]));
    assert_eq!(take_input(io::Cursor::new(vec![b'x'; 9]), 8, SLOW_CI), None);
    assert_eq!(take_input(io::Cursor::new(Vec::new()), 8, SLOW_CI).as_deref(), Some(&b""[..]));
    let started = Instant::now();
    assert_eq!(take_input(Stuck, 8, short), None);
    assert!(started.elapsed() < SLOW_CI / 4, "a pipe that never closes must not hold the hook");
}
