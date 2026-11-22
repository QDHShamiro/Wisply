use std::fs;
use std::path::{Path, PathBuf};

use super::*;

fn base(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("wisply-cm-gate-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("build")).unwrap();
    dir
}

fn rules(deny: &[&str], ask: &[&str], dir: &Path) -> Rules {
    let mut rules = Rules::default();
    let from = dir.join(".claude/settings.json");
    for rule in deny {
        rules.add(rule, false, dir, &from, false);
    }
    for rule in ask {
        rules.add(rule, true, dir, &from, false);
    }
    rules
}

fn is_ask(gate: &Gate) -> bool {
    matches!(gate, Gate::Ask { .. })
}

fn is_block(gate: &Gate) -> bool {
    matches!(gate, Gate::Block { .. })
}

#[test]
fn codex_rules_are_read_and_nested_choices_are_skipped() {
    let text = r#"
prefix_rule(pattern=["git", "push"], decision="forbidden")
prefix_rule(pattern = ["npm", "publish"], decision = "prompt")
prefix_rule(pattern=["java", "-version"], decision="allow")
prefix_rule(pattern=["gh", ["pr", "issue"]], decision="forbidden")
"#;
    let found = codex_rules(text);
    assert_eq!(found.len(), 3);
    assert_eq!(found[0], (vec!["git".to_string(), "push".to_string()], "forbidden".to_string()));
    assert_eq!(found[1].1, "prompt");
}

#[test]
fn deny_rules_block_even_with_yes_and_ask_rules_wait_for_yes() {
    let dir = base("rules");
    let r = rules(&["Bash(git push:*)", "Read(./.env)"], &["Bash(npm publish)"], &dir);
    assert!(is_block(&shell(&r, true, "cd x && git push origin main", true, &dir, &dir)));
    assert!(is_block(&shell(&r, false, "GIT PUSH", true, &dir, &dir)));
    assert!(is_ask(&shell(&r, false, "npm publish", true, &dir, &dir)));
    assert_eq!(shell(&r, true, "npm publish", true, &dir, &dir), Gate::Pass);
    assert_eq!(shell(&r, false, "git status", true, &dir, &dir), Gate::Pass);
    assert!(is_block(&file(&r, true, Op::Read, &dir.join(".env"), &dir)));
    assert_eq!(file(&r, false, Op::Read, &dir.join("src/main.rs"), &dir), Gate::Pass);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn deleting_inside_the_project_passes_and_everything_else_asks() {
    let dir = base("delete");
    let r = Rules::default();
    let pass = ["rm -rf build", "rm -rf build/*.o target", "rm -r ./build/", "rmdir build && echo done"];
    for line in pass {
        assert_eq!(shell(&r, false, line, true, &dir, &dir), Gate::Pass, "{line}");
    }
    let ask = [
        "rm -rf .",
        "rm -rf *",
        "rm -rf ../other",
        "rm -rf /etc",
        "rm -rf ~/x",
        "rm -rf $HOME",
        "rm -rf C:/Windows",
        "cd .. && rm -rf build",
        "rm -rf build && git reset --hard",
        "rm -rf `pwd`",
    ];
    for line in ask {
        assert!(is_ask(&shell(&r, false, line, true, &dir, &dir)), "{line}");
    }
    let elsewhere = std::env::temp_dir();
    assert!(is_ask(&shell(&r, false, "rm -rf build", true, &elsewhere, &dir)));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn files_outside_the_project_and_secret_files_ask() {
    let dir = base("files");
    let outside = std::env::temp_dir().join(format!("wisply-cm-out-{}.txt", std::process::id()));
    let other = std::env::current_exe().unwrap();
    fs::write(&outside, "x").unwrap();
    let r = Rules::default();
    assert_eq!(file(&r, false, Op::Write, &dir.join("new.txt"), &dir), Gate::Pass);
    assert_eq!(file(&r, false, Op::Write, &outside, &dir), Gate::Pass);
    assert!(is_ask(&file(&r, false, Op::Write, &other, &dir)));
    let key = files::user_home().unwrap().join(".ssh/id_rsa");
    assert!(is_ask(&file(&r, false, Op::Read, &key, &dir)));
    assert_eq!(file(&r, true, Op::Read, &key, &dir), Gate::Pass);
    let _ = fs::remove_file(&outside);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn the_first_scan_finds_risky_literals_before_anything_runs() {
    let dir = base("scan");
    let r = rules(&["Bash(git push:*)"], &[], &dir);
    assert_eq!(scan(&r, false, "await $`git status`; await read(\"src/a.rs\")", &dir, true), Gate::Pass);
    assert!(is_block(&scan(&r, true, "const b = 'main';\nawait $`git push origin ${b}`", &dir, true)));
    assert!(is_ask(&scan(&r, false, "await $(\"rm -rf /\")", &dir, true)));
    assert!(is_ask(&scan(&r, false, "await write('~/.ssh/config', 'x')", &dir, true)));
    assert_eq!(scan(&r, false, "const t = `rm -rf /`; t.length", &dir, true), Gate::Pass);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn path_rules_follow_claude_code_patterns() {
    let base = Path::new("/work/app");
    assert!(path_fits(&path_glob(".env", base), "/work/app/sub/.env"));
    assert!(path_fits(&path_glob("/secrets/**", base), "/work/app/secrets/a/b.txt"));
    assert!(!path_fits(&path_glob("/secrets/**", base), "/work/other/secrets/a.txt"));
    assert!(path_fits(&path_glob("src/*.rs", base), "/work/app/src/main.rs"));
}
