use std::collections::BTreeMap;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use super::engine::{Done, Host, Step};
use super::files;
use super::gate::{self, Gate, Op, Rules};
use super::shell::{self, Job, Shell};

const WORKERS: usize = 8;
pub const NESTED: &str = "WISPLY_CODEMODE";
const BLOCKED: &[&str] = &[
    "daemon",
    "host",
    "mcp",
    "install",
    "uninstall",
    "codemode",
    "statusline",
    "hook",
    "ui",
    "update",
    "voice-session",
    "voice-watch",
    "testapp",
    "claude-start",
];
const OWN: &[&str] = &[
    "more", "peek", "progress", "kit", "skill", "lint", "safe", "review", "analyze", "setup", "pupil", "headless",
    "doctor", "squeeze", "call", "local",
];
const HOLE_OPEN: char = '\u{E000}';
const HOLE_CLOSE: char = '\u{E001}';

pub struct Setup {
    pub home: PathBuf,
    pub cwd: PathBuf,
    pub sid: String,
    pub yes: bool,
    pub shell: Shell,
    pub rules: Rules,
    pub deadline: Instant,
    pub args: BTreeMap<String, String>,
}

#[derive(Default)]
pub struct Shared {
    pub stop: AtomicBool,
    pub busy: AtomicUsize,
    pub seen: AtomicUsize,
    pub finished: AtomicUsize,
    pub failed: AtomicUsize,
    pub secret: AtomicBool,
    marked: AtomicBool,
}

enum Work {
    Shell { line: String, cwd: PathBuf, env: Vec<(String, String)>, stdin: Option<String>, until: Instant },
    Wisply { argv: Vec<String>, secret: Option<String> },
    Read(PathBuf),
    Write(PathBuf, String),
    Edit { path: PathBuf, from: String, to: String, all: bool },
    Glob(PathBuf, String),
}

pub struct Runner {
    setup: Arc<Setup>,
    jobs: Sender<(i32, Work)>,
}

pub fn start(setup: Setup, done: Sender<Done>) -> (Runner, Arc<Shared>) {
    let setup = Arc::new(setup);
    let shared = Arc::new(Shared::default());
    let (jobs, queue) = mpsc::channel::<(i32, Work)>();
    let queue = Arc::new(Mutex::new(queue));
    for _ in 0..WORKERS {
        let (setup, shared, queue, done) = (setup.clone(), shared.clone(), queue.clone(), done.clone());
        thread::spawn(move || work(&setup, &shared, &queue, &done));
    }
    (Runner { setup, jobs }, shared)
}

fn work(setup: &Setup, shared: &Shared, queue: &Mutex<Receiver<(i32, Work)>>, done: &Sender<Done>) {
    loop {
        let next = queue.lock().map_err(|_| ()).and_then(|q| q.recv().map_err(|_| ()));
        let Ok((id, task)) = next else { return };
        shared.busy.fetch_add(1, Ordering::SeqCst);
        let (ok, text, seen) = match perform(setup, shared, task) {
            Ok((text, seen)) => (true, text, seen),
            Err(text) => (false, text, 0),
        };
        shared.seen.fetch_add(seen, Ordering::Relaxed);
        if !ok {
            shared.failed.fetch_add(1, Ordering::SeqCst);
        }
        shared.finished.fetch_add(1, Ordering::SeqCst);
        shared.busy.fetch_sub(1, Ordering::SeqCst);
        if done.send(Done { id, ok, text }).is_err() {
            return;
        }
    }
}

fn perform(setup: &Setup, shared: &Shared, task: Work) -> Result<(String, usize), String> {
    match task {
        Work::Shell { line, cwd, env, stdin, until } => {
            let job = Job { line: &line, cwd: &cwd, env: &env, stdin: stdin.as_deref(), until, stop: &shared.stop };
            let out = shell::run(&setup.shell, &job)?;
            let seen = out.stdout.chars().count() + out.stderr.chars().count();
            Ok((json!({"code": out.code, "stdout": out.stdout, "stderr": out.stderr}).to_string(), seen))
        }
        Work::Wisply { argv, secret } => {
            if secret.is_some() {
                shared.secret.store(true, Ordering::SeqCst);
            }
            let (code, out) = wisply(setup, shared, &argv, secret.as_deref())?;
            if out.starts_with(wisply_proto::HANDOFF) {
                return Err(super::saved::INSIDE.to_string());
            }
            let seen = out.chars().count();
            Ok((json!({"code": code, "out": out}).to_string(), seen))
        }
        Work::Read(path) => files::read(&path).map(|text| {
            let seen = text.chars().count();
            (text, seen)
        }),
        Work::Write(path, text) => files::write(&path, &text).map(|p| (p, 0)),
        Work::Edit { path, from, to, all } => files::edit(&path, &from, &to, all).map(|n| (n, 0)),
        Work::Glob(cwd, pattern) => files::glob(&cwd, &pattern).map(|list| {
            let text = Value::from(list).to_string();
            let seen = text.chars().count();
            (text, seen)
        }),
    }
}

fn wisply(setup: &Setup, shared: &Shared, argv: &[String], secret: Option<&str>) -> Result<(i32, String), String> {
    let first = argv.first().map(String::as_str).unwrap_or("");
    let own = OWN.contains(&first)
        || (first == "memory" && argv.get(1).is_some_and(|a| a == "bench"))
        || crate::waitfor::wanted(argv);
    if own {
        let exe = env::current_exe().map_err(|e| format!("cannot find wisply itself ({e})"))?;
        let mut cmd = Command::new(exe);
        cmd.args(argv).current_dir(&setup.cwd).env(NESTED, "1");
        let until = setup.deadline;
        let out = shell::wait_for(cmd, None, until, &shared.stop)?;
        let text = if out.stdout.trim().is_empty() { out.stderr } else { out.stdout };
        return Ok((out.code, text.trim_end().to_string()));
    }
    if !shared.marked.swap(true, Ordering::SeqCst) {
        let mark = ["codemode".to_string(), "mark".to_string()];
        let _ = crate::client::ask_as(&setup.home, &mark, &setup.sid, None, &setup.cwd);
    }
    crate::client::ask_as(&setup.home, argv, &setup.sid, secret, &setup.cwd)
}

fn holes(parts: &[Value]) -> Result<Vec<String>, String> {
    let mut line = String::new();
    let mut values: Vec<Vec<String>> = Vec::new();
    for part in parts {
        if let Some(raw) = part.get("raw").and_then(Value::as_str) {
            line.push_str(raw);
            continue;
        }
        let list = match (part.get("arg").and_then(Value::as_str), part.get("args").and_then(Value::as_array)) {
            (Some(arg), _) => vec![arg.to_string()],
            (None, Some(args)) => args.iter().map(|a| a.as_str().unwrap_or_default().to_string()).collect(),
            _ => continue,
        };
        line.push_str(&format!("{HOLE_OPEN}{}{HOLE_CLOSE}", values.len()));
        values.push(list);
    }
    let fill = |word: &str| {
        let mut out = word.to_string();
        for (i, list) in values.iter().enumerate() {
            out = out.replace(&format!("{HOLE_OPEN}{i}{HOLE_CLOSE}"), &list.join(" "));
        }
        out
    };
    let mut argv = Vec::new();
    for word in wisply_proto::words(&line)? {
        let whole =
            word.strip_prefix(HOLE_OPEN).and_then(|w| w.strip_suffix(HOLE_CLOSE)).and_then(|n| n.parse::<usize>().ok());
        match whole.and_then(|i| values.get(i)) {
            Some(list) => argv.extend(list.iter().cloned()),
            None => argv.push(fill(&word)),
        }
    }
    Ok(argv)
}

fn argv_of(payload: &Value) -> Result<Vec<String>, String> {
    let mut argv = if let Some(list) = payload.get("argv").and_then(Value::as_array) {
        list.iter().map(|a| a.as_str().unwrap_or_default().to_string()).collect()
    } else if let Some(parts) = payload.get("parts").and_then(Value::as_array) {
        holes(parts)?
    } else {
        wisply_proto::words(payload.get("line").and_then(Value::as_str).unwrap_or("").trim())?
    };
    if argv.first().is_some_and(|a| a == "wisply" || a == "lighting") {
        argv.remove(0);
    }
    if argv.is_empty() {
        argv.push("help".into());
    }
    Ok(argv)
}

fn take_env(argv: &mut Vec<String>) -> Result<Option<String>, String> {
    let Some(at) = argv.iter().position(|a| a == "--env") else { return Ok(None) };
    let name = argv.get(at + 1).cloned().ok_or("--env needs the name of an environment variable")?;
    argv.drain(at..at + 2);
    env::var(&name).map(Some).map_err(|_| format!("environment variable {name} is not set"))
}

fn halt(gate: &Gate) -> Option<Step> {
    let again = super::AGAIN;
    let (reason, hint) = match gate {
        Gate::Pass => return None,
        Gate::Ask { what, risk: Some(risk), mark } => {
            (format!("{what} {} ('{mark}'), so the user decides", risk.what()), again)
        }
        Gate::Ask { what, risk: None, mark } => (format!("{what} needs the user's ok first ({mark})"), again),
        Gate::Block { what, rule, from } => {
            (format!("{what} is forbidden by the user's rule {rule} in {from}"), "leave that step out, the rule stays")
        }
    };
    Some(Step::Halt(format!("{reason}\0{hint}")))
}

impl Runner {
    fn shell_work(&self, payload: &Value) -> Result<Work, Step> {
        let parts = payload.get("parts").and_then(Value::as_array).cloned().unwrap_or_default();
        let line = shell::line(self.setup.shell.kind, &parts);
        let bash = self.setup.shell.kind != shell::Kind::Power;
        let cwd = match payload.get("cwd").and_then(Value::as_str) {
            Some(dir) if !dir.trim().is_empty() => files::resolve(&self.setup.cwd, dir),
            _ => self.setup.cwd.clone(),
        };
        if let Some(step) = halt(&gate::shell(&self.setup.rules, self.setup.yes, &line, bash, &cwd, &self.setup.cwd)) {
            return Err(step);
        }
        let mut env: Vec<(String, String)> = payload
            .get("env")
            .and_then(Value::as_object)
            .map(|m| m.iter().map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_string())).collect())
            .unwrap_or_default();
        env.push((NESTED.into(), "1".into()));
        let ms = payload.get("timeout").and_then(Value::as_f64).filter(|ms| *ms > 0.0);
        let until = ms.map_or(self.setup.deadline, |ms| {
            (Instant::now() + Duration::from_millis(ms as u64)).min(self.setup.deadline)
        });
        let stdin = payload.get("stdin").and_then(Value::as_str).map(str::to_string);
        Ok(Work::Shell { line, cwd, env, stdin, until })
    }

    fn file_work(&self, kind: &str, payload: &Value) -> Result<Work, Step> {
        let text = |key: &str| payload.get(key).and_then(Value::as_str).unwrap_or("").to_string();
        let path = files::resolve(&self.setup.cwd, &text("path"));
        if kind == "glob" {
            let dir = text("cwd");
            let cwd =
                if dir.trim().is_empty() { self.setup.cwd.clone() } else { files::resolve(&self.setup.cwd, &dir) };
            return Ok(Work::Glob(cwd, text("pattern")));
        }
        let op = if kind == "read" { Op::Read } else { Op::Write };
        if let Some(step) = halt(&gate::file(&self.setup.rules, self.setup.yes, op, &path, &self.setup.cwd)) {
            return Err(step);
        }
        Ok(match kind {
            "read" => Work::Read(path),
            "write" => Work::Write(path, text("text")),
            _ => Work::Edit {
                path,
                from: text("from"),
                to: text("to"),
                all: payload.get("all").and_then(Value::as_bool).unwrap_or(false),
            },
        })
    }

    fn wisply_work(payload: &Value) -> Result<Work, Step> {
        let mut argv = argv_of(payload).map_err(|why| Step::Now(false, format!("err: {why}")))?;
        let first = argv[0].as_str();
        if BLOCKED.contains(&first) {
            let why =
                format!("err: wisply {first} cannot run inside code mode -> try: run it on its own, outside the code");
            return Err(Step::Now(false, why));
        }
        let secret = take_env(&mut argv).map_err(|why| Step::Now(false, format!("err: {why}")))?;
        Ok(Work::Wisply { argv, secret })
    }
}

impl Host for Runner {
    fn start(&self, id: i32, kind: &str, payload: &str) -> Step {
        let payload: Value = serde_json::from_str(payload).unwrap_or(Value::Null);
        let work = match kind {
            "sh" => self.shell_work(&payload),
            "wisply" => Runner::wisply_work(&payload),
            "read" | "write" | "edit" | "glob" => self.file_work(kind, &payload),
            _ => Err(Step::Now(false, format!("code mode has no call named {kind}"))),
        };
        match work {
            Ok(work) => match self.jobs.send((id, work)) {
                Ok(()) => Step::Going,
                Err(_) => Step::Now(false, "the workers of code mode stopped".into()),
            },
            Err(step) => step,
        }
    }

    fn facts(&self) -> String {
        let cwd = files::shown(&self.setup.cwd);
        json!({"shell": self.setup.shell.name(), "os": env::consts::OS, "cwd": cwd, "args": self.setup.args})
            .to_string()
    }
}
