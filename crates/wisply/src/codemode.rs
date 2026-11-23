mod engine;
pub(crate) mod files;
mod gate;
mod host;
mod learned;
mod saved;
mod shell;

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::{Duration, Instant};

use wisply_core::{Log, OUT_CHARS, Row};

use engine::{Ran, Verdict};
use gate::Gate;
use host::{Setup, Shared};

pub const HELP: &str = include_str!("codemode/help.txt");
const LIMIT: Duration = Duration::from_secs(600);
const CODE_MAX: u64 = 1 << 20;
const STACK: usize = 16 << 20;
const SETTLE: Duration = Duration::from_secs(2);
const LINE_MAX: usize = 120;
const CALL_ERRORS: [&str; 2] = ["ShellError", "WisplyError"];
const AGAIN: &str = "ask the user, then run the same code again with --yes (\"yes\": true over MCP)";
const OFF: &str = "err: code mode is turned off on this computer -> try: single wisply commands, or the user turns it on with: wisply config codemode on";
const NESTED: &str = "err: code mode cannot start inside code mode -> try: put those steps into the outer script";

#[derive(Debug, PartialEq, Eq, Default)]
struct Ask {
    yes: bool,
    limit: Duration,
    code: String,
    args: BTreeMap<String, String>,
    routine: Option<String>,
}

fn pair(text: &str) -> Result<(String, String), String> {
    let bad = format!("--arg takes name=value, got '{text}'");
    let (k, v) = text.split_once('=').ok_or(bad.clone())?;
    let mut chars = k.chars();
    let starts = chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_');
    if !starts || !chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(bad);
    }
    Ok((k.to_string(), v.to_string()))
}

fn span(text: &str) -> Option<Duration> {
    let text = text.trim().to_lowercase();
    let at = text.find(|c: char| !c.is_ascii_digit() && c != '.').unwrap_or(text.len());
    let (number, unit) = text.split_at(at);
    let n = number.parse::<f64>().ok().filter(|n| *n > 0.0)?;
    let secs = match unit.trim() {
        "ms" => n / 1000.0,
        "" | "s" | "sec" => n,
        "m" | "min" => n * 60.0,
        _ => return None,
    };
    Some(Duration::from_secs_f64(secs).min(LIMIT))
}

fn parse(args: &[String], stdin: impl FnOnce() -> Result<String, String>) -> Result<Option<Ask>, String> {
    let mut ask = Ask { limit: LIMIT, ..Ask::default() };
    let (mut words, mut from, mut piped) = (Vec::new(), None::<String>, false);
    let bad_time = "--timeout takes a time like 90s or 5m";
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        let (flag, inline) = a.split_once('=').map_or((a, None), |(f, v)| (f, Some(v.to_string())));
        let takes = matches!(flag, "--timeout" | "--file" | "--arg" | "--routine");
        let value = match (takes, inline) {
            (false, _) => String::new(),
            (true, Some(v)) => v,
            (true, None) => {
                i += 1;
                args.get(i).cloned().ok_or(format!("{a} needs a value"))?
            }
        };
        match flag {
            "--yes" | "-y" => ask.yes = true,
            "--timeout" => ask.limit = span(&value).ok_or(bad_time)?,
            "--file" => from = Some(value),
            "--arg" => {
                let (k, v) = pair(&value)?;
                ask.args.insert(k, v);
            }
            "--routine" => ask.routine = Some(value),
            "-" => piped = true,
            _ => words.push(a),
        }
        i += 1;
    }
    if words.len() == 1 && matches!(words[0], "help" | "-h" | "--help") && from.is_none() && !piped {
        return Ok(None);
    }
    ask.code = match (from, piped) {
        (Some(path), _) => {
            let full = files::resolve(&env::current_dir().unwrap_or_default(), &path);
            fs::read_to_string(&full).map_err(|e| format!("cannot read {} ({e})", files::shown(&full)))?
        }
        (None, true) => stdin()?,
        (None, false) => words.join(" "),
    };
    match (&ask.routine, ask.code.trim().is_empty()) {
        (Some(_), false) => Err("--routine runs saved code, so leave the code out".into()),
        (Some(_), true) | (None, false) => Ok(Some(ask)),
        (None, true) => Ok(None),
    }
}

fn read_stdin() -> Result<String, String> {
    let mut raw = Vec::new();
    io::stdin()
        .take(CODE_MAX + 1)
        .read_to_end(&mut raw)
        .map_err(|e| format!("cannot read the code from stdin ({e})"))?;
    if raw.len() as u64 > CODE_MAX {
        return Err(format!("the code on stdin is over {} MB", CODE_MAX >> 20));
    }
    Ok(String::from_utf8_lossy(&raw).into_owned())
}

fn plural(n: usize, one: &str) -> String {
    if n == 1 { format!("1 {one}") } else { format!("{n} {one}s") }
}

fn progress(finished: usize) -> String {
    match finished {
        0 => "Nothing was done.".into(),
        1 => "1 earlier call already finished and stays done, nothing after this ran.".into(),
        n => format!("{n} earlier calls already finished and stay done, nothing after this ran."),
    }
}

fn source_line(code: &str, line: usize) -> Option<String> {
    let text = code.lines().nth(line.checked_sub(1)?)?.trim();
    if text.is_empty() {
        return None;
    }
    let flat: String = text.chars().take(LINE_MAX).collect();
    Some(if text.chars().count() > LINE_MAX { format!("{flat}...") } else { flat })
}

fn halted(text: &str, finished: usize) -> String {
    let (reason, hint) = text.split_once('\0').unwrap_or((text, "ask the user"));
    format!("err: code mode stopped: {reason}. {} -> try: {hint}", progress(finished))
}

fn threw(t: &engine::Thrown, code: &str, finished: usize) -> String {
    let (head, rest) = t.message.split_once('\n').unwrap_or((&t.message, ""));
    let place = t.line.map(|n| format!(" at line {n}")).unwrap_or_default();
    let mut out = format!("err: {}{place}: {head}", t.name);
    if let Some(src) = t.line.and_then(|n| source_line(code, n)) {
        out.push_str(&format!("\n  > {src}"));
    }
    if !rest.trim().is_empty() {
        out.push('\n');
        out.push_str(rest.trim_end());
    }
    let fix = t.line.map_or("fix the code".to_string(), |n| format!("fix line {n}"));
    let finished = if CALL_ERRORS.contains(&t.name.as_str()) { finished.saturating_sub(1) } else { finished };
    let done = match finished {
        0 => String::new(),
        1 => " (1 call finished and stays done)".to_string(),
        n => format!(" ({n} calls finished and stay done)"),
    };
    out.push_str(&format!("\n-> try: {fix} and run it again{done}"));
    out
}

fn explain(ran: &Ran, code: &str, finished: usize, limit: Duration) -> (String, bool) {
    let (out, ok) = match &ran.verdict {
        Verdict::Value(Some(text)) => (text.clone(), true),
        Verdict::Value(None) if ran.prints.trim().is_empty() => ("ok".into(), true),
        Verdict::Value(None) => (String::new(), true),
        Verdict::Threw(t) => (threw(t, code, finished), false),
        Verdict::Halted(text) => (halted(text, finished), false),
        Verdict::TimedOut => {
            let secs = limit.as_secs();
            let took = if secs >= 60 && secs.is_multiple_of(60) { format!("{} min", secs / 60) } else { format!("{secs} s") };
            let running = ran.started.saturating_sub(ran.finished);
            let done = match (finished.saturating_sub(running), running) {
                (0, 0) => "Nothing had finished.".to_string(),
                (n, 0) => format!("{} had finished before the stop.", plural(n, "call")),
                (n, r) => format!("{} had finished before the stop, {r} still running got stopped.", plural(n, "call")),
            };
            let text = format!(
                "err: code mode ran out of time after {took} and stopped what was still running. {done} -> try: a longer --timeout (up to 10m), or split the work"
            );
            (text, false)
        }
        Verdict::Stuck => (
            "err: the code waits for something that never comes (a promise nobody resolves) -> try: await only $, wisply, read, write, edit, glob and sleep".into(),
            false,
        ),
        Verdict::Broken(why) => (format!("err: {why} -> try: run the same code again"), false),
    };
    let mut text = ran.prints.trim_end().to_string();
    if !out.is_empty() {
        if !text.is_empty() {
            text.push('\n');
        }
        text.push_str(&out);
    }
    for lost in &ran.lost {
        text.push_str(&format!("\n! a call nobody awaited failed: {lost}"));
    }
    (text, ok)
}

fn drain(shared: &Shared, until: Instant) {
    while shared.busy.load(Ordering::SeqCst) > 0 && Instant::now() < until {
        thread::sleep(Duration::from_millis(20));
    }
}

fn settle(shared: &Shared, verdict: &Verdict, deadline: Instant) {
    if matches!(verdict, Verdict::TimedOut | Verdict::Broken(_)) {
        shared.stop.store(true, Ordering::SeqCst);
    }
    drain(shared, deadline.max(Instant::now() + SETTLE));
    if shared.busy.load(Ordering::SeqCst) > 0 {
        shared.stop.store(true, Ordering::SeqCst);
        drain(shared, Instant::now() + SETTLE);
    }
}

fn execute(home: &Path, ask: &Ask, cwd: PathBuf, rules: gate::Rules, shell: shell::Shell) -> (Ran, Arc<Shared>) {
    let deadline = Instant::now() + ask.limit;
    let sid = env::var("CLAUDE_CODE_SESSION_ID").unwrap_or_default();
    let args = ask.args.clone();
    let setup = Setup { home: home.to_path_buf(), cwd, sid, yes: ask.yes, shell, rules, deadline, args };
    let (done_tx, done_rx) = mpsc::channel();
    let (runner, shared) = host::start(setup, done_tx);
    let code = ask.code.clone();
    let worker = thread::Builder::new().name("wisply-codemode".into()).stack_size(STACK).spawn(move || {
        let host: Rc<dyn engine::Host> = Rc::new(runner);
        engine::run(&code, host, &done_rx, deadline)
    });
    let joined = worker.map_err(|e| e.to_string()).and_then(|h| h.join().map_err(|_| "it crashed".to_string()));
    let ran = joined.unwrap_or_else(|why| Ran {
        verdict: Verdict::Broken(format!("the script engine stopped ({why})")),
        prints: String::new(),
        lost: Vec::new(),
        started: 0,
        finished: 0,
    });
    settle(&shared, &ran.verdict, deadline);
    (ran, shared)
}

fn scan_first(ask: &Ask, cwd: &Path, rules: &gate::Rules, bash: bool) -> Option<String> {
    let (reason, hint) = match gate::scan(rules, ask.yes, &ask.code, cwd, bash) {
        Gate::Pass => return None,
        Gate::Ask { what, risk: Some(risk), mark } => {
            (format!("{what} {} ('{mark}'), so the user decides", risk.what()), AGAIN)
        }
        Gate::Ask { what, risk: None, mark } => (format!("{what} needs the user's ok first ({mark})"), AGAIN),
        Gate::Block { what, rule, from } => {
            (format!("{what} is forbidden by the user's rule {rule} in {from}"), "leave that step out, the rule stays")
        }
    };
    Some(halted(&format!("{reason}\0{hint}"), 0))
}

fn for_routine(text: String, ask: &Ask) -> String {
    match &ask.routine {
        Some(name) => text.replace(AGAIN, &format!("ask the user, then run: wisply run {name} --yes")),
        None => text,
    }
}

fn emit(home: &Path, text: &str, seen: usize, code: i32, started: Instant) -> ExitCode {
    let (shown, _) = wisply_core::cap(&home.join("out"), text, "codemode", None, OUT_CHARS);
    println!("{shown}");
    let argv = ["codemode".to_string()];
    let sid = env::var("CLAUDE_CODE_SESSION_ID").unwrap_or_default();
    let row = Row {
        name: "codemode",
        argv: &argv,
        code,
        ms: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        out: &shown,
        test: false,
        cut: seen.saturating_sub(shown.chars().count()),
        sid: &sid,
        id: "",
        queued_ms: 0,
        stages: &[],
    };
    Log::new(home).write(&row);
    ExitCode::from(u8::try_from(code).unwrap_or(1))
}

pub fn run(home: &Path, args: &[String]) -> ExitCode {
    let started = Instant::now();
    if env::var_os(host::NESTED).is_some() {
        println!("{NESTED}");
        return ExitCode::from(1);
    }
    let mut ask = match parse(args, read_stdin) {
        Ok(Some(ask)) => ask,
        Ok(None) => {
            println!("{}", HELP.trim_end());
            return ExitCode::SUCCESS;
        }
        Err(why) => {
            println!("err: {why} -> try: wisply codemode help");
            return ExitCode::from(2);
        }
    };
    if !crate::daemon::config_on(&crate::daemon::load_config(home), "codemode") {
        println!("{OFF}");
        return ExitCode::from(1);
    }
    if let Some(name) = &ask.routine {
        match saved::code_of(home, name) {
            Ok(code) => ask.code = code,
            Err(why) => return emit(home, &format!("err: {why}"), 0, 1, started),
        }
    }
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let rules = gate::load(&cwd);
    let shell = shell::find();
    let plan = learned::plan(home, &ask);
    let done = learned::go(&learned::Env { home, cwd: &cwd, rules: &rules, shell: &shell }, &ask, &plan);
    let mut text = for_routine(done.text, &ask);
    if done.halted {
        return emit(home, &text, done.seen, 1, started);
    }
    let report = saved::Report {
        code: &ask.code,
        args: &ask.args,
        routine: ask.routine.as_deref(),
        calls: done.calls,
        ms: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        ok: done.ok,
        secret: done.secret,
        error: (!done.ok).then(|| text.lines().next().unwrap_or("")),
        checked: done.checked,
        rounds: done.rounds,
        dry: plan.dry,
    };
    let sid = env::var("CLAUDE_CODE_SESSION_ID").unwrap_or_default();
    let note = saved::tell(home, &cwd, &sid, &report);
    if let Some(note) = note {
        text.push_str(&format!("\n{note}"));
    }
    emit(home, &text, done.seen, if done.ok { 0 } else { 1 }, started)
}

#[cfg(test)]
#[path = "codemode/tests.rs"]
mod tests;
