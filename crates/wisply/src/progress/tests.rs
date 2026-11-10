use std::path::PathBuf;

use super::*;

fn temp(name: &str) -> PathBuf {
    let home = env::temp_dir().join(format!("wisply-progress-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&home);
    home
}

fn tool(sid: &str, name: &str, input: Value, response: Value) -> Value {
    json!({"session_id": sid, "cwd": "C:/work/app", "tool_name": name, "tool_input": input, "tool_response": response})
}

fn shown(home: &Path, sid: &str) -> String {
    field(home, Some(sid), None, now(), true).map(|l| plain(&l)).unwrap_or_default()
}

#[test]
fn a_todo_list_gives_percent_count_and_the_running_step() {
    let home = temp("todo");
    let todos = json!({"todos": [
        {"content": "Read the code", "status": "completed", "activeForm": "Reading the code"},
        {"content": "Write the fix", "status": "in_progress", "activeForm": "Writing the fix"},
        {"content": "Run the tests", "status": "pending", "activeForm": "Running the tests"},
        {"content": "Ship", "status": "pending", "activeForm": "Shipping"}
    ]});
    assert!(note(&home, &tool("s1", "TodoWrite", todos, Value::Null)));
    assert_eq!(shown(&home, "s1"), "##-------- 25% 1/4 Writing the fix");
    let colored = field(&home, Some("s1"), None, now(), false).unwrap();
    assert!(colored.contains("\u{1b}[38;5;220m25%\u{1b}[0m"), "{colored:?}");
    assert_eq!(shown(&home, "s2"), "");
    let plan = json!({"plan": [{"step": "Look", "status": "completed"}, {"step": "Build", "status": "in_progress"}]});
    assert!(note(&home, &tool("s2", "update_plan", plan, Value::Null)));
    assert_eq!(shown(&home, "s2"), "#####----- 50% 1/2 Build");
    assert_eq!(shown(&home, "s1"), "##-------- 25% 1/4 Writing the fix");
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn task_tools_build_the_plan_step_by_step() {
    let home = temp("tasks");
    let made = |id: &str| json!({"task": {"id": id}});
    let first = json!({"subject": "Write the tests", "description": "x", "activeForm": "Writing the tests"});
    assert!(note(&home, &tool("s1", "TaskCreate", first, made("1"))));
    assert!(note(&home, &tool("s1", "TaskCreate", json!({"subject": "Ship it"}), made("2"))));
    assert_eq!(shown(&home, "s1"), "---------- 0% 0/2 Write the tests");
    assert!(note(&home, &tool("s1", "TaskUpdate", json!({"taskId": "1", "status": "in_progress"}), Value::Null)));
    assert_eq!(shown(&home, "s1"), "---------- 0% 0/2 Writing the tests");
    assert!(note(&home, &tool("s1", "TaskUpdate", json!({"taskId": "1", "status": "completed"}), Value::Null)));
    assert_eq!(shown(&home, "s1"), "#####----- 50% 1/2 Ship it");
    assert!(note(&home, &tool("s1", "TaskUpdate", json!({"taskId": "2", "status": "deleted"}), Value::Null)));
    assert_eq!(shown(&home, "s1"), "########## 100% 1/1 done");
    assert!(note(&home, &tool("s1", "TaskUpdate", json!({"taskId": "9", "status": "completed"}), Value::Null)));
    assert_eq!(shown(&home, "s1"), "########## 100% 1/1 done");
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn the_last_tool_is_shown_next_to_the_plan() {
    let home = temp("last");
    let command = json!({"command": "cd /e/x && TOKEN=abc rtk cargo test --workspace -- --nocapture"});
    assert!(!note(&home, &tool("s1", "Bash", command, Value::Null)));
    assert_eq!(shown(&home, "s1"), "> Bash cargo test");
    let todos = json!({"todos": [{"content": "Fix", "status": "in_progress", "activeForm": "Fixing"}]});
    assert!(note(&home, &tool("s1", "TodoWrite", todos, Value::Null)));
    assert_eq!(shown(&home, "s1"), "---------- 0% 0/1 Fixing > Bash cargo test");
    let later = now() + LAST_FRESH + 1;
    assert_eq!(field(&home, Some("s1"), None, later, true).map(|l| plain(&l)).unwrap(), "---------- 0% 0/1 Fixing");
    assert!(!note(&home, &json!({"session_id": "s1"})));
    assert!(!note(&home, &json!({"tool_name": "Bash"})));
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn the_tool_label_leaves_out_arguments_secrets_and_folders() {
    let label = |name: &str, input: Value| activity(name, &input);
    assert_eq!(label("Read", json!({"file_path": "C:\\Users\\me\\private\\notes.txt"})), "Read notes.txt");
    assert_eq!(label("Edit", json!({"file_path": "/home/me/app/src/main.rs"})), "Edit main.rs");
    assert_eq!(label("WebFetch", json!({"url": "https://user:pw@example.com/a?token=1"})), "WebFetch example.com");
    assert_eq!(label("Bash", json!({"command": "curl -H 'Authorization: x' https://a.example"})), "Bash curl");
    assert_eq!(label("Bash", json!({"command": "export KEY=1; echo $KEY"})), "Bash echo");
    assert_eq!(label("Bash", json!({"command": "\n  git push origin main --force\n"})), "Bash git push origin");
    assert_eq!(label("Bash", json!({"command": "mysql -psecret db"})), "Bash mysql");
    assert_eq!(label("Bash", json!({"command": "cd /x"})), "Bash");
    assert_eq!(label("Grep", json!({"pattern": "a\u{1b}[31mb\nc"})), "Grep a[31mb c");
    assert_eq!(label("WebSearch", json!({"query": "rust file lock"})), "WebSearch rust file lock");
    assert_eq!(label("mcp__x__y", json!({"anything": 1})), "mcp__x__y");
    assert_eq!(label("Grep", json!({"pattern": "x".repeat(500)})).chars().count(), TEXT_MAX);
}

#[test]
fn a_reported_amount_wins_and_keeps_its_label() {
    let home = temp("report");
    assert_eq!(report(&home, "k", &["3/8", "Fixing", "tests"]).unwrap(), "###------- 37% 3/8 Fixing tests");
    assert_eq!(report(&home, "k", &["40"]).unwrap(), "####------ 40% Fixing tests");
    assert_eq!(report(&home, "k", &["Writing docs"]).unwrap(), "####------ 40% Writing docs");
    assert_eq!(report(&home, "k", &["100%"]).unwrap(), "########## 100% Writing docs");
    assert_eq!(report(&home, "k", &["9/3"]).unwrap(), "########## 100% 3/3 Writing docs");
    for bad in ["140", "3/0", "3/x", "1.5"] {
        assert_eq!(report(&home, "k", &[bad]), Err(BAD_AMOUNT.to_string()), "{bad}");
    }
    assert_eq!(report(&home, "new", &["Looking around"]).unwrap(), "Looking around");
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn old_progress_is_not_shown_any_more() {
    let home = temp("fresh");
    let todos = json!({"todos": [{"content": "A", "status": "completed"}, {"content": "B", "status": "pending"}]});
    note(&home, &tool("s1", "TodoWrite", todos, Value::Null));
    let at = now();
    let line = |time: u64| field(&home, Some("s1"), None, time, true).map(|l| plain(&l));
    assert_eq!(line(at + DONE_FRESH + 1).as_deref(), Some("#####----- 50% 1/2 B"));
    assert_eq!(line(at + PLAN_FRESH + 5), None);
    let done = json!({"todos": [{"content": "A", "status": "completed"}, {"content": "B", "status": "completed"}]});
    note(&home, &tool("s1", "TodoWrite", done, Value::Null));
    assert_eq!(line(now()).as_deref(), Some("########## 100% 2/2 done"));
    assert_eq!(line(now() + DONE_FRESH + 5), None);
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn a_folder_stands_in_when_there_is_no_session() {
    let home = temp("folder");
    assert_eq!(keys(None, Some("C:\\Work\\App\\")), keys(None, Some("c:/work/app")));
    assert_eq!(keys(Some(" "), None), Vec::<String>::new());
    let key = keys(None, Some("C:/work/app")).remove(0);
    report(&home, &key, &["2/4", "Building"]).unwrap();
    let line = field(&home, Some("unknown"), Some("c:\\work\\app"), now(), true).map(|l| plain(&l));
    assert_eq!(line.as_deref(), Some("#####----- 50% 2/4 Building"));
    assert_eq!(field(&home, Some("unknown"), Some("C:/other"), now(), true), None);
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn sessions_writing_at_once_lose_nothing() {
    let home = temp("race");
    let file = dir(&home).join("s.json");
    let workers: Vec<_> = (0..8)
        .map(|n| {
            let file = file.clone();
            thread::spawn(move || {
                for i in 0..5 {
                    let input = json!({"subject": format!("step {n}-{i}")});
                    let made = json!({"id": format!("{n}-{i}")});
                    assert!(change(&file, |p| p.add(&input, &made, now())).is_some());
                }
            })
        })
        .collect();
    for w in workers {
        w.join().unwrap();
    }
    let plan = Plan::parse(&fs::read_to_string(&file).unwrap());
    assert_eq!(plan.steps.len(), 40);
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn a_broken_file_is_ignored_and_then_replaced() {
    let home = temp("broken");
    let key = keys(Some("s1"), None).remove(0);
    fs::create_dir_all(dir(&home)).unwrap();
    fs::write(dir(&home).join(format!("{key}.json")), "{\"steps\": 7, \"t\": \"x\"").unwrap();
    fs::write(dir(&home).join(format!("{key}.now")), "no tab here").unwrap();
    assert_eq!(shown(&home, "s1"), "");
    let todos = json!({"todos": [{"content": "A", "status": "pending"}, {"bad": 1}, "text"]});
    assert!(note(&home, &tool("s1", "TodoWrite", todos, Value::Null)));
    assert_eq!(shown(&home, "s1"), "---------- 0% 0/1 A");
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn the_line_is_drawn_with_colours_by_how_far_it_is() {
    let line = |percent: i64| draw(Some(percent), "", "", None, false).unwrap();
    assert!(line(10).contains("\u{1b}[38;5;220m10%"));
    assert!(line(40).contains("\u{1b}[38;5;39m40%"));
    assert!(line(70).contains("\u{1b}[38;5;70m70%"));
    assert!(line(100).contains("\u{1b}[38;5;46m100%"));
    assert_eq!(
        plain(&line(100)),
        "\u{2501}\u{2501}\u{2501}\u{2501}\u{2501}\u{2501}\u{2501}\u{2501}\u{2501}\u{2501} 100% \u{2713}"
    );
    let long = "A very long description of the current step that does not fit";
    assert_eq!(plain(&draw(None, "", long, None, true).unwrap()), "A very long description of the curre...");
    assert_eq!(plain(&draw(None, "", long, None, false).unwrap()), "A very long description of the curre\u{2026}");
    assert_eq!(draw(None, "", "", None, true), None);
    assert_eq!(plain(&draw(None, "", "", Some("Bash cargo test"), false).unwrap()), "\u{203a} Bash cargo test");
}
