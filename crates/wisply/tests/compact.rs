use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

const EXE: &str = env!("CARGO_BIN_EXE_wisply-full");

struct Home(PathBuf);

impl Home {
    fn new(tag: &str) -> Home {
        let base = if cfg!(windows) { std::env::temp_dir() } else { PathBuf::from("/tmp") };
        let path = base.join(format!("ltc-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(path.join("claude")).unwrap();
        fs::write(path.join("v3"), "").unwrap();
        Home(path)
    }

    fn settings(&self) -> PathBuf {
        self.0.join("claude").join("settings.json")
    }

    fn run(&self, args: &[&str]) -> (i32, String) {
        let out = Command::new(EXE)
            .args(args)
            .env("WISPLY_HOME", &self.0)
            .env(if cfg!(windows) { "USERPROFILE" } else { "HOME" }, &self.0)
            .env("CLAUDE_CONFIG_DIR", self.0.join("claude"))
            .env("CLAUDE_CODE_SESSION_ID", "cc-1")
            .env_remove("MSYSTEM")
            .stdin(Stdio::null())
            .output()
            .unwrap();
        let text = String::from_utf8_lossy(&out.stdout).trim_end().to_string() + &String::from_utf8_lossy(&out.stderr);
        (out.status.code().unwrap_or(-1), text.trim_end().to_string())
    }
}

impl Drop for Home {
    fn drop(&mut self) {
        let _ = self.run(&["stop"]);
        for _ in 0..20 {
            if fs::remove_dir_all(&self.0).is_ok() || !self.0.exists() {
                return;
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn transcript(home: &Home) -> PathBuf {
    let row = |v: Value| v.to_string();
    let say = |t: &str| row(json!({"type": "assistant", "message": {"content": [{"type": "text", "text": t}]}}));
    let edit = |f: &str| {
        row(
            json!({"type": "assistant", "message": {"content": [{"type": "tool_use", "name": "Edit", "input": {"file_path": f}}]}}),
        )
    };
    let rows = [
        row(
            json!({"type": "user", "message": {"content": "Move the export job to the new queue so reports stop timing out"}}),
        ),
        say("Root cause: the export job holds the lock while it renders, so I chose a separate worker queue instead."),
        edit("/work/reports/src/export.rs"),
        edit("/work/reports/src/queue.rs"),
        row(json!({"type": "user", "message": {"content": "keep the old endpoint working for one release"}})),
        say("Decided to keep /v1/export as a thin wrapper because clients cannot update at once."),
    ];
    let path = home.0.join("session.jsonl");
    fs::write(&path, rows.join("\n") + "\n").unwrap();
    path
}

#[test]
fn the_compact_window_is_written_and_removed_in_a_test_settings_file() {
    let home = Home::new("window");
    fs::write(home.settings(), r#"{"model":"opus","env":{"A":"1"}}"#).unwrap();
    assert_eq!(home.run(&["config", "compact"]).1.split(" (").next(), Some("compact = off"));
    let (code, said) = home.run(&["config", "compact", "250k"]);
    assert_eq!(code, 0, "{said}");
    let v: Value = serde_json::from_str(&fs::read_to_string(home.settings()).unwrap()).unwrap();
    assert_eq!((&v["autoCompactWindow"], &v["model"], &v["env"]["A"]), (&json!(250_000), &json!("opus"), &json!("1")));
    let (code, said) = home.run(&["setup", "--compact", "400k"]);
    assert_eq!(code, 0, "{said}");
    let v: Value = serde_json::from_str(&fs::read_to_string(home.settings()).unwrap()).unwrap();
    assert_eq!(v["autoCompactWindow"], 400_000);
    assert!(home.run(&["config", "compact"]).1.starts_with("compact = 400k"));
    assert_eq!(home.run(&["config", "compact", "50k"]).0, 1);
    home.run(&["config", "compact", "off"]);
    let v: Value = serde_json::from_str(&fs::read_to_string(home.settings()).unwrap()).unwrap();
    assert!(v.get("autoCompactWindow").is_none() && v["model"] == "opus");
}

#[test]
fn the_precompact_hook_writes_a_handoff_that_comes_back_after_the_compact() {
    let home = Home::new("handoff");
    assert_eq!(home.run(&["todo", "add", "write the migration note"]).1, "to-do #1 added");
    let path = transcript(&home);
    let cwd = std::env::current_dir().unwrap().display().to_string();
    let data = |extra: Value| {
        let mut v = json!({"cwd": cwd, "session_id": "cc-1", "transcript_path": path.display().to_string()});
        v.as_object_mut().unwrap().extend(extra.as_object().unwrap().clone());
        v.to_string()
    };
    let t = Instant::now();
    let (code, out) = home.run(&["hook", "compact", &data(json!({"trigger": "auto"}))]);
    let took = t.elapsed();
    assert_eq!((code, out.as_str()), (0, ""));
    assert!(took < Duration::from_millis(1500), "{took:?}");
    let (_, after) = home.run(&["hook", "start", &data(json!({"source": "compact"}))]);
    assert!(after.starts_with("Wisply handoff"), "{after}");
    assert!(after.contains("Goal: Move the export job to the new queue"), "{after}");
    assert!(after.contains("Latest request: keep the old endpoint working for one release"), "{after}");
    assert!(after.contains("separate worker queue") && after.contains("thin wrapper"), "{after}");
    assert!(after.contains("/work/reports/src/export.rs") && after.contains("write the migration note"), "{after}");
    assert!(after.contains("Wisply after the summary ("), "{after}");
    let (_, again) = home.run(&["hook", "start", &data(json!({"source": "compact"}))]);
    assert!(again.starts_with("Wisply after the summary ("), "{again}");
}
