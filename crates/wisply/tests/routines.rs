use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use serde_json::{Value, json};

const EXE: &str = env!("CARGO_BIN_EXE_wisply-full");

struct Home(PathBuf);

impl Home {
    fn new(tag: &str) -> Home {
        let base = if cfg!(windows) { std::env::temp_dir() } else { PathBuf::from("/tmp") };
        let path = base.join(format!("ltd-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        fs::write(path.join("v3"), "").unwrap();
        Home(path)
    }

    fn run(&self, args: &[&str]) -> (i32, String) {
        let out = Command::new(EXE)
            .args(args)
            .env("WISPLY_HOME", &self.0)
            .env(if cfg!(windows) { "USERPROFILE" } else { "HOME" }, &self.0)
            .env("CLAUDE_CODE_SESSION_ID", "it-1")
            .env_remove("MSYSTEM")
            .stdin(Stdio::null())
            .output()
            .unwrap();
        (out.status.code().unwrap_or(-1), String::from_utf8_lossy(&out.stdout).trim_end().to_string())
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

#[test]
fn routines_from_2x_run_with_params_and_count_their_runs() {
    let home = Home::new("routines");
    fs::create_dir_all(home.0.join("routines")).unwrap();
    let routine = json!({"name": "local-check", "version": 1, "source": "saved", "created": "2026-10-30 18:00",
        "params": {"word": "x"}, "steps": [
            {"cmd": "remember", "args": ["routine note {word}"], "flags": {}},
            {"cmd": "wait", "args": ["200"], "flags": {}},
            {"cmd": "nonsense", "args": [], "flags": {}, "optional": true},
            {"cmd": "recall", "args": ["routine note {word}"], "flags": {}}],
        "check": null, "cost": 400, "stats": {"runs": 0, "ok": 0, "fail": 0, "ms": 0, "saved": 0, "seen": 1, "recent": []},
        "flaky": false});
    fs::write(home.0.join("routines").join("local-check.json"), routine.to_string()).unwrap();
    let (code, out) = home.run(&["run", "local-check", "word=alpha"]);
    assert!(code == 0 && out.starts_with("ok local-check: 3 steps in "), "{out}");
    let (code, out) = home.run(&["run", "local"]);
    assert!(code == 1 && out == "err: local-check needs word -> try: wisply run local-check word=x", "{out}");
    let (_, out) = home.run(&["routines"]);
    assert!(out.starts_with("local-check word= | 1x ok | "), "{out}");
    let (_, out) = home.run(&["routine", "show", "local-check"]);
    assert!(out.contains("\n3 nonsense (optional)\n4 recall ") && out.contains("\nruns 1, ok 1, fail 0, "), "{out}");
    let saved: Value =
        serde_json::from_str(&fs::read_to_string(home.0.join("routines").join("local-check.json")).unwrap()).unwrap();
    assert_eq!(saved["stats"]["recent"], json!([1]));
    let (code, out) = home.run(&["run", "nothing-like-this"]);
    assert!(code == 1 && out.starts_with("err: no routine"), "{out}");
}

#[test]
fn tasks_become_episodes_that_can_be_saved_offered_and_forgotten() {
    let home = Home::new("episodes");
    assert_eq!(home.run(&["wait", "50"]).0, 0);
    assert_eq!(home.run(&["wait", "60"]).0, 0);
    assert_eq!(home.run(&["routine", "learn"]), (0, "nothing new to learn".into()));
    let kept = fs::read_to_string(home.0.join("episodes.jsonl")).unwrap();
    let ep: Value = serde_json::from_str(kept.lines().next().unwrap()).unwrap();
    assert_eq!(ep["steps"].as_array().unwrap().len(), 2, "{kept}");
    assert_eq!((ep["steps"][0]["cmd"].as_str(), ep["steps"][1]["args"].clone()), (Some("wait"), json!(["60"])));
    let (code, out) = home.run(&["routine", "save", "two waits", "ms=60"]);
    assert_eq!((code, out.as_str()), (0, "saved routine two-waits (2 steps) -> wisply run two-waits ms=60"));
    let (_, shown) = home.run(&["routine", "show", "two-waits"]);
    assert!(shown.contains("\n1 wait 50\n2 wait {ms}\n"), "{shown}");
    let (code, out) = home.run(&["run", "two-waits", "ms=70"]);
    assert!(code == 0 && out.starts_with("ok two-waits: 2 steps in "), "{out}");
    let routine = json!({"name": "github-issues", "version": 1, "source": "learned", "created": "2026-10-30 18:00",
        "params": {"page": "x"}, "steps": [{"cmd": "open", "args": ["https://github.com/QDHShamiro/{page}"], "flags": {}}],
        "check": null, "cost": 900, "tags": ["issues"],
        "stats": {"runs": 0, "ok": 0, "fail": 0, "ms": 0, "saved": 0, "seen": 2, "recent": []}, "flaky": false});
    fs::write(home.0.join("routines").join("github-issues.json"), routine.to_string()).unwrap();
    let ask = json!({"prompt": "zeig mir die github issues bitte"}).to_string();
    let (_, offered) = home.run(&["suggest", &ask]);
    let line = "Wisply routine fits this request (one call, swap the values): wisply run github-issues page=x";
    assert!(offered.starts_with(line), "{offered}");
    assert!(!home.run(&["suggest", &ask]).1.contains("github-issues"));
    assert_eq!(home.run(&["routine", "forget"]), (0, "task history cleared (routines stay)".into()));
    assert!(!home.0.join("episodes.jsonl").exists());
}

#[test]
fn a_failed_routine_is_repaired_from_the_steps_done_by_hand() {
    let home = Home::new("repair");
    fs::create_dir_all(home.0.join("routines")).unwrap();
    let routine = json!({"name": "fixme", "version": 1, "source": "saved", "created": "2026-10-30 18:00",
        "params": {}, "steps": [{"cmd": "wait", "args": ["20"], "flags": {}}, {"cmd": "nonsense", "args": [], "flags": {}}],
        "check": null, "cost": 300, "stats": {"runs": 0, "ok": 0, "fail": 0, "ms": 0, "saved": 0, "seen": 1, "recent": []},
        "flaky": false});
    fs::write(home.0.join("routines").join("fixme.json"), routine.to_string()).unwrap();
    let (code, out) = home.run(&["run", "fixme"]);
    assert!(code == 1 && out.starts_with("err: fixme stopped at step 2/2 (nonsense): "), "{out}");
    home.run(&["wait", "30"]);
    home.run(&["wait", "40"]);
    let (_, out) = home.run(&["routine", "learn"]);
    assert_eq!(out, "learned fixme\n! repaired routine fixme (v2) from your fix");
    let (_, shown) = home.run(&["routine", "show", "fixme"]);
    assert!(shown.starts_with("fixme v2 (saved, ") && shown.contains("\n1 wait 20\n2 wait 30\n3 wait 40\n"), "{shown}");
}
