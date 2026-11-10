use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const EXE: &str = env!("CARGO_BIN_EXE_wisply-full");

struct Home(PathBuf);

impl Home {
    fn new(tag: &str) -> Home {
        let base = if cfg!(windows) { std::env::temp_dir() } else { PathBuf::from("/tmp") };
        let path = base.join(format!("lte-{tag}-{}", std::process::id()));
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
            .env("WISPLY_NO_UPDATE", "1")
            .env("CLAUDE_CODE_SESSION_ID", "cc-errors")
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

const WRONG: [&[&str]; 24] = [
    &["zzz"],
    &["config", "bogus"],
    &["config", "cleanup", "maybe"],
    &["config", "update", "sometimes"],
    &["keys", "add"],
    &["keys", "rm"],
    &["todo", "bogus"],
    &["todo", "add"],
    &["dialog", "maybe"],
    &["log", "abc"],
    &["record", "bogus"],
    &["routine", "bogus"],
    &["wiki", "bogus"],
    &["memory", "bogus"],
    &["mobile", "bogus"],
    &["notify", "bogus"],
    &["viewport", "abc"],
    &["tokens", "bogus"],
    &["headless", "bogus"],
    &["statusline", "bogus"],
    &["statusline", "set", "bogus", "on"],
    &["statusline", "set", "tokens"],
    &["update", "--bogus"],
    &["progress", "150"],
];

#[test]
fn a_wrong_command_says_what_is_wrong_and_what_to_try() {
    let home = Home::new("try");
    for args in WRONG {
        let (code, said) = home.run(args);
        let line = said.lines().next().unwrap_or_default();
        assert!(code == 1 && line.starts_with("err: ") && line.contains(" -> try: "), "{args:?}: {code} {said}");
        assert!(!line.contains("usage:"), "{args:?}: {said}");
    }
}

#[test]
fn a_wrong_value_changes_nothing() {
    let home = Home::new("keep");
    let before = home.run(&["config", "cleanup"]);
    assert_eq!(before.0, 0, "{before:?}");
    assert_eq!(
        home.run(&["config", "cleanup", "maybe"]),
        (1, "err: cleanup is on or off -> try: wisply config cleanup on".into())
    );
    assert_eq!(home.run(&["config", "cleanup"]), before);
    assert_eq!(
        home.run(&["keys", "rm"]),
        (1, "err: keys rm needs an app and a key -> try: wisply keys rm discord.exe ctrl+k".into())
    );
    assert_eq!(
        home.run(&["dialog", "maybe"]),
        (1, "err: dialog needs accept or dismiss -> try: wisply dialog accept|dismiss [text]".into())
    );
    assert_eq!(home.run(&["record", "bogus"]).1.split(" -> ").next(), Some("err: unknown record command 'bogus'"));
    assert_eq!(home.run(&["routine", "bogus"]).1.split(" -> ").next(), Some("err: unknown routine command 'bogus'"));
    assert!(home.run(&["todo", "bogus"]).1.contains("todo done|doing|drop|open <id>"));
    let (code, said) = home.run(&["install", "nope"]);
    assert!(code == 1 && said.contains("unknown AI app 'nope' -> try: wisply install <antigravity|"), "{said}");
}
