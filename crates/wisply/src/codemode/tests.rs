use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use super::engine::{self, Host, Ran, Step, Thrown, Verdict};

#[derive(Default)]
struct Fake {
    seen: RefCell<Vec<String>>,
}

impl Host for Fake {
    fn start(&self, _id: i32, kind: &str, payload: &str) -> Step {
        self.seen.borrow_mut().push(format!("{kind} {payload}"));
        match kind {
            "wisply" if payload.contains("boom") => {
                Step::Now(true, r#"{"code":1,"out":"err: no page -> try: open one"}"#.into())
            }
            "wisply" => Step::Now(true, format!(r#"{{"code":0,"out":"ran {}"}}"#, payload.len())),
            "sh" if payload.contains("risky") => Step::Halt("asks first".into()),
            "sh" if payload.contains("fails") => Step::Now(true, r#"{"code":3,"stdout":"","stderr":"bad\n"}"#.into()),
            "sh" => Step::Now(true, r#"{"code":0,"stdout":"a\nb\n","stderr":""}"#.into()),
            "read" => Step::Now(true, "file text".into()),
            _ => Step::Now(false, format!("no {kind} here")),
        }
    }

    fn facts(&self) -> String {
        r#"{"shell":"bash","os":"test","cwd":"/work"}"#.into()
    }
}

fn run_for(code: &str, limit: Duration) -> (Ran, Vec<String>) {
    let fake = Rc::new(Fake::default());
    let (_tx, rx) = mpsc::channel();
    let ran = engine::run(code, fake.clone(), &rx, Instant::now() + limit);
    let seen = fake.seen.borrow().clone();
    (ran, seen)
}

fn value(code: &str) -> String {
    match run_for(code, Duration::from_secs(20)).0.verdict {
        Verdict::Value(Some(text)) => text,
        other => format!("{other:?}"),
    }
}

#[test]
fn the_last_expression_is_the_result() {
    assert_eq!(value("1 + 1"), "2");
    assert_eq!(value("const a = await wisply('snap -f x'); a"), "ran 20");
    assert_eq!(value("({ value: 7 })"), r#"{"value":7}"#);
    assert_eq!(value("['a', 'b']"), "a\nb");
    assert_eq!(value("const x = 5; return x * 2"), "10");
    assert_eq!(value("await $`ls`.lines()"), "a\nb");
    assert_eq!(value("({ out: $`ls`, n: sleep(5).then(() => 3) })"), r#"{"out":"a\nb","n":3}"#);
    assert_eq!(value("shell + ' ' + os + ' ' + cwd"), "bash test /work");
}

#[test]
fn errors_name_the_line_and_stops_are_told_apart() {
    let (ran, _) = run_for("const a = 1;\nawait wisply('boom');\n3", Duration::from_secs(20));
    assert_eq!(
        ran.verdict,
        Verdict::Threw(Thrown {
            name: "WisplyError".into(),
            message: "err: no page -> try: open one".into(),
            line: Some(2)
        })
    );
    let (ran, _) = run_for("const a = ;", Duration::from_secs(20));
    assert!(
        matches!(ran.verdict, Verdict::Threw(Thrown { ref name, line: Some(1), .. }) if name == "SyntaxError"),
        "{:?}",
        ran.verdict
    );
    let (ran, seen) = run_for("await $`risky`; await $`ls`", Duration::from_secs(20));
    assert_eq!(ran.verdict, Verdict::Halted("asks first".into()));
    assert_eq!((seen.len(), ran.started, ran.finished), (1, 0, 0));
    assert_eq!(run_for("while (true) {}", Duration::from_millis(300)).0.verdict, Verdict::TimedOut);
    assert_eq!(run_for("await new Promise(() => {})", Duration::from_secs(20)).0.verdict, Verdict::Stuck);
}

#[test]
fn lost_failures_and_shell_errors_are_reported() {
    let (ran, _) = run_for("$`fails`; 1", Duration::from_secs(20));
    assert_eq!(ran.verdict, Verdict::Value(Some("1".into())));
    assert_eq!(ran.lost, vec!["exit 3 from: fails".to_string()]);
    let (ran, _) = run_for("const r = await $`fails`.nothrow();\nr.exitCode", Duration::from_secs(20));
    assert_eq!(ran.verdict, Verdict::Value(Some("3".into())));
    let (ran, _) = run_for("\n\nawait $`fails`", Duration::from_secs(20));
    assert!(
        matches!(ran.verdict, Verdict::Threw(Thrown { ref name, line: Some(3), .. }) if name == "ShellError"),
        "{:?}",
        ran.verdict
    );
    let (ran, _) = run_for("console.log('one', {a: 1}); console.log([1, 2])", Duration::from_secs(20));
    assert_eq!(ran.prints, "one {\"a\":1}\n1\n2\n");
}

#[test]
fn values_are_quoted_for_the_shell() {
    use super::shell::{Kind, line};
    let parts = |v: serde_json::Value| v.as_array().unwrap().clone();
    let p = parts(serde_json::json!([{"raw": "echo "}, {"arg": "it's $HOME"}, {"raw": " "}, {"args": ["a b", "c"]}]));
    assert_eq!(line(Kind::Bash, &p), r"echo 'it'\''s $HOME' 'a b' c");
    let p = parts(serde_json::json!([{"raw": "echo \"x "}, {"arg": "a\"b$c"}, {"raw": "\""}]));
    assert_eq!(line(Kind::Bash, &p), r#"echo "x a\"b\$c""#);
    let p = parts(serde_json::json!([{"raw": "Write-Output "}, {"arg": "it's"}]));
    assert_eq!(line(Kind::Power, &p), "Write-Output 'it''s'");
    let p = parts(serde_json::json!([{"raw": "ls "}, {"arg": "src/main.rs"}]));
    assert_eq!(line(Kind::Bash, &p), "ls src/main.rs");
}

#[test]
fn the_command_line_is_read() {
    use super::{Ask, LIMIT, parse};
    let args = |w: &[&str]| w.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
    let none = || Err("no stdin".to_string());
    let ask = parse(&args(&["--yes", "--timeout", "90s", "1 +", "1"]), none).unwrap().unwrap();
    assert_eq!(ask, Ask { yes: true, limit: Duration::from_secs(90), code: "1 + 1".into(), ..Ask::default() });
    let ask = parse(&args(&["--arg", "pr=101", "--arg=note=a=b", "args.pr"]), none).unwrap().unwrap();
    assert_eq!((ask.args["pr"].as_str(), ask.args["note"].as_str(), ask.code.as_str()), ("101", "a=b", "args.pr"));
    assert_eq!(parse(&args(&["--arg", "1x=2", "x"]), none), Err("--arg takes name=value, got '1x=2'".into()));
    assert_eq!(parse(&args(&["--arg", "x"]), none), Err("--arg takes name=value, got 'x'".into()));
    let replay = parse(&args(&["--routine", "ship-pr", "--arg", "pr=7"]), none).unwrap().unwrap();
    assert_eq!((replay.routine.as_deref(), replay.code.as_str(), replay.args.len()), (Some("ship-pr"), "", 1));
    assert_eq!(
        parse(&args(&["--routine=ship-pr", "1"]), none),
        Err("--routine runs saved code, so leave the code out".into())
    );
    assert_eq!(
        parse(&args(&["--timeout=1h", "x"]), none).map(|a| a.map(|a| a.limit)),
        Err("--timeout takes a time like 90s or 5m".into())
    );
    assert_eq!(parse(&args(&["--timeout=30m", "x"]), none).unwrap().unwrap().limit, LIMIT);
    assert_eq!(parse(&args(&["help"]), none), Ok(None));
    assert_eq!(parse(&args(&[]), none), Ok(None));
    let piped = parse(&args(&["-y", "-"]), || Ok("await sleep(1)".into())).unwrap().unwrap();
    assert_eq!((piped.yes, piped.code.as_str()), (true, "await sleep(1)"));
}

#[test]
fn errors_show_the_line_and_what_already_ran() {
    use super::explain;
    let ran = Ran {
        verdict: Verdict::Threw(Thrown {
            name: "ShellError".into(),
            message: "exit 2 from: cargo test\nfailed".into(),
            line: Some(2),
        }),
        prints: "step one\n".into(),
        lost: Vec::new(),
        started: 2,
        finished: 2,
    };
    let (text, ok) = explain(&ran, "const a = 1;\nawait $`cargo test`", 2, Duration::from_secs(600));
    assert!(!ok);
    assert_eq!(
        text,
        "step one\nerr: ShellError at line 2: exit 2 from: cargo test\n  > await $`cargo test`\nfailed\n-> try: fix line 2 and run it again (2 calls finished and stay done)"
    );
    let ran = Ran {
        verdict: Verdict::Value(None),
        prints: String::new(),
        lost: vec!["boom".into()],
        started: 1,
        finished: 1,
    };
    assert_eq!(
        explain(&ran, "", 1, Duration::from_secs(600)),
        ("ok\n! a call nobody awaited failed: boom".into(), true)
    );
}

#[test]
fn real_shell_and_files_run_through_the_workers() {
    use super::host::{self, Setup};
    use super::{gate, shell};
    let dir = std::env::temp_dir().join(format!("wisply-cm-e2e-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let deadline = Instant::now() + Duration::from_secs(60);
    let setup = Setup {
        home: dir.join("home"),
        cwd: dir.clone(),
        sid: String::new(),
        yes: false,
        shell: shell::find(),
        rules: gate::Rules::default(),
        deadline,
        args: std::collections::BTreeMap::from([("pr".to_string(), "101".to_string())]),
    };
    let (tx, rx) = mpsc::channel();
    let (runner, shared) = host::start(setup, tx);
    let code = r#"
const name = "it's";
await write("notes/a.txt", "one\ntwo\n");
const n = await edit("notes/a.txt", "two", "three");
const [said, list, text] = await Promise.all([$`echo ${name}`.text(), glob("notes/*.txt"), read("notes/a.txt")]);
const bad = await $`exit 4`.nothrow();
({ said: said.trim(), list, n, text, bad: bad.exitCode, pr: args.pr, frozen: Object.isFrozen(args) })
"#;
    let ran = engine::run(code, Rc::new(runner), &rx, deadline);
    assert_eq!(
        ran.verdict,
        Verdict::Value(Some(
            r#"{"said":"it's","list":["notes/a.txt"],"n":1,"text":"one\nthree\n","bad":4,"pr":"101","frozen":true}"#
                .into()
        ))
    );
    assert_eq!(shared.finished.load(std::sync::atomic::Ordering::SeqCst), 6);
    let _ = std::fs::remove_dir_all(&dir);
}
