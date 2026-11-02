use std::process::{Command, ExitCode, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::Value;
use wisply_squeeze::{clean, decode, read_limited, squeeze};

const CI_USAGE: &str = "wisply wait ci [--pr N|--branch B|--sha S] [--timeout 30m] [--ignore name,name]";
const UNTIL_USAGE: &str = "wisply wait until \"<command>\" [--every 5s] [--timeout 10m]";
const CI_TIMEOUT: Duration = Duration::from_secs(30 * 60);
const UNTIL_TIMEOUT: Duration = Duration::from_secs(10 * 60);
const UNTIL_EVERY: Duration = Duration::from_secs(5);
const POLL_FIRST: Duration = Duration::from_secs(10);
const POLL_MAX: Duration = Duration::from_secs(30);
const POLL_STEP: Duration = Duration::from_secs(5);
const RUN_LIMIT: &str = "30";
const FAIL_LINES: usize = 12;
const FAIL_CHARS: usize = 1_500;
const SHORT_SHA: usize = 7;
const OUT_MAX: usize = 4_000_000;
const GH_ENV: &str = "WISPLY_GH";

#[derive(Debug, PartialEq)]
pub enum State {
    Waiting(usize),
    Green(usize),
    Red(Vec<Failed>),
}

#[derive(Debug, PartialEq)]
pub struct Failed {
    pub run: u64,
    pub name: String,
}

fn text<'a>(v: &'a Value, key: &str) -> &'a str {
    v[key].as_str().unwrap_or_default()
}

pub fn judge(runs: &[Value]) -> State {
    if runs.is_empty() {
        return State::Waiting(0);
    }
    let open = runs.iter().filter(|r| text(r, "status") != "completed").count();
    let mut bad = Vec::new();
    for r in runs.iter().filter(|r| text(r, "status") == "completed") {
        if !matches!(text(r, "conclusion"), "success" | "skipped" | "neutral") {
            bad.push(Failed { run: r["databaseId"].as_u64().unwrap_or(0), name: text(r, "workflowName").to_string() });
        }
    }
    if !bad.is_empty() {
        return State::Red(bad);
    }
    if open > 0 { State::Waiting(open) } else { State::Green(runs.len()) }
}

pub fn failure_marks(message: &str, ignore: &[String]) -> Vec<String> {
    message
        .split_whitespace()
        .filter_map(|w| w.strip_suffix("=failure"))
        .filter(|name| !name.is_empty() && !ignore.iter().any(|i| i == name))
        .map(|name| format!("{name}=failure"))
        .collect()
}

pub fn parse_duration(raw: &str) -> Option<Duration> {
    let raw = raw.trim();
    let (digits, unit) = raw.split_at(raw.find(|c: char| !c.is_ascii_digit()).unwrap_or(raw.len()));
    let n: u64 = digits.parse().ok()?;
    let scale = match unit {
        "" | "s" => 1,
        "m" => 60,
        "h" => 3600,
        _ => return None,
    };
    Some(Duration::from_secs(n.checked_mul(scale)?))
}

pub fn next_poll(current: Duration) -> Duration {
    (current + POLL_STEP).min(POLL_MAX)
}

pub fn runs_of_newest_sha(all: Vec<Value>) -> Vec<Value> {
    let Some(sha) = all.first().map(|r| text(r, "headSha").to_string()) else { return all };
    all.into_iter().filter(|r| text(r, "headSha") == sha).collect()
}

struct Flags {
    pr: Option<String>,
    branch: Option<String>,
    sha: Option<String>,
    timeout: Duration,
    ignore: Vec<String>,
}

fn parse_ci(args: &[String]) -> Result<Flags, String> {
    let mut f = Flags { pr: None, branch: None, sha: None, timeout: CI_TIMEOUT, ignore: Vec::new() };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let (key, inline) = match a.split_once('=') {
            Some((k, v)) => (k, Some(v.to_string())),
            None => (a.as_str(), None),
        };
        let value = inline.or_else(|| it.next().cloned()).ok_or(format!("{key} needs a value"))?;
        match key {
            "--pr" => f.pr = Some(value),
            "--branch" => f.branch = Some(value),
            "--sha" => f.sha = Some(value),
            "--timeout" => f.timeout = parse_duration(&value).ok_or("timeout like 30m, 90s or 1h")?,
            "--ignore" => f.ignore = value.split(',').map(str::to_string).collect(),
            other => return Err(format!("unknown option {other}")),
        }
    }
    Ok(f)
}

fn capture(program: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(program).args(args).stdin(Stdio::null()).stderr(Stdio::null()).output().ok()?;
    out.status.success().then(|| decode(&out.stdout))
}

fn gh(args: &[&str]) -> Option<String> {
    let program = std::env::var(GH_ENV).unwrap_or_else(|_| "gh".into());
    capture(&program, args)
}

fn git(args: &[&str]) -> Option<String> {
    capture("git", args).map(|s| s.trim().to_string()).filter(|s| !s.is_empty())
}

fn json(raw: Option<String>) -> Option<Value> {
    serde_json::from_str(&raw?).ok()
}

struct Target {
    branch: Option<String>,
    sha: Option<String>,
}

fn target(f: &Flags) -> Result<Target, String> {
    if let Some(pr) = &f.pr {
        let v = json(gh(&["pr", "view", pr, "--json", "headRefName,headRefOid"]))
            .ok_or(format!("gh cannot read PR {pr}"))?;
        return Ok(Target {
            branch: Some(text(&v, "headRefName").to_string()),
            sha: Some(text(&v, "headRefOid").to_string()),
        });
    }
    if f.branch.is_some() || f.sha.is_some() {
        return Ok(Target { branch: f.branch.clone(), sha: f.sha.clone() });
    }
    let branch = git(&["rev-parse", "--abbrev-ref", "HEAD"]).ok_or("not in a git repository")?;
    Ok(Target { sha: git(&["rev-parse", "HEAD"]), branch: Some(branch) })
}

fn list_runs(t: &Target) -> Result<Vec<Value>, String> {
    let mut args =
        vec!["run", "list", "--limit", RUN_LIMIT, "--json", "databaseId,status,conclusion,headSha,workflowName"];
    if let Some(b) = &t.branch {
        args.extend(["--branch", b]);
    }
    let all = json(gh(&args)).ok_or("gh run list failed (is gh installed and logged in?)")?;
    let all = all.as_array().cloned().unwrap_or_default();
    Ok(match &t.sha {
        Some(sha) => all.into_iter().filter(|r| text(r, "headSha").starts_with(sha.as_str())).collect(),
        None => runs_of_newest_sha(all),
    })
}

fn failed_job_lines(run: u64) -> String {
    let log = gh(&["run", "view", &run.to_string(), "--log-failed"]).unwrap_or_default();
    let body = squeeze(&log).map_or_else(|| clean(&log), |s| s.plain);
    let lines: Vec<&str> = body.lines().filter(|l| !l.trim().is_empty()).collect();
    let tail = lines.len().saturating_sub(FAIL_LINES);
    let joined = lines[tail..].join("\n");
    joined.chars().take(FAIL_CHARS).collect()
}

fn annotation_failures(run: u64, ignore: &[String]) -> Vec<String> {
    let Some(v) = json(gh(&["run", "view", &run.to_string(), "--json", "jobs"])) else { return Vec::new() };
    let repo = gh(&["repo", "view", "--json", "nameWithOwner", "-q", ".nameWithOwner"]).unwrap_or_default();
    let mut marks = Vec::new();
    for job in v["jobs"].as_array().into_iter().flatten() {
        let Some(job_id) = job["databaseId"].as_u64() else { continue };
        let path = format!("repos/{}/check-runs/{job_id}/annotations", repo.trim());
        let Some(list) = json(gh(&["api", &path])) else { continue };
        for a in list.as_array().into_iter().flatten() {
            for m in failure_marks(text(a, "message"), ignore) {
                marks.push(format!("{} {m}", text(job, "name")));
            }
        }
    }
    marks
}

fn sha_label(t: &Target) -> String {
    t.sha.as_deref().map(|s| s.chars().take(SHORT_SHA).collect()).unwrap_or_default()
}

fn fail(msg: &str) -> ExitCode {
    println!("{msg}");
    ExitCode::from(2)
}

pub fn ci(args: &[String]) -> ExitCode {
    let flags = match parse_ci(args) {
        Ok(f) => f,
        Err(why) => return fail(&format!("err: {why} -> try: {CI_USAGE}")),
    };
    let target = match target(&flags) {
        Ok(t) => t,
        Err(why) => return fail(&format!("err: {why} -> try: {CI_USAGE}")),
    };
    let started = Instant::now();
    let mut pause = POLL_FIRST;
    let mut last;
    loop {
        match list_runs(&target) {
            Ok(runs) => match judge(&runs) {
                State::Green(n) => return finish_green(&runs, n, &target, &flags, started),
                State::Red(bad) => return finish_red(&bad, &target),
                State::Waiting(0) => last = "no runs yet".to_string(),
                State::Waiting(n) => last = format!("{n} run(s) still going"),
            },
            Err(why) => last = why,
        }
        if started.elapsed() + pause > flags.timeout {
            return fail(&format!("err: timeout after {} s: {last}", started.elapsed().as_secs()));
        }
        thread::sleep(pause);
        pause = next_poll(pause);
    }
}

fn finish_green(runs: &[Value], n: usize, t: &Target, f: &Flags, started: Instant) -> ExitCode {
    let marks: Vec<String> =
        runs.iter().flat_map(|r| annotation_failures(r["databaseId"].as_u64().unwrap_or(0), &f.ignore)).collect();
    if marks.is_empty() {
        println!("green: {n} run(s) {} in {} s", sha_label(t), started.elapsed().as_secs());
        return ExitCode::SUCCESS;
    }
    println!("red: {} {}", sha_label(t), marks.join(", "));
    ExitCode::from(1)
}

fn finish_red(bad: &[Failed], t: &Target) -> ExitCode {
    let names: Vec<&str> = bad.iter().map(|b| b.name.as_str()).collect();
    println!("red: {} {}", sha_label(t), names.join(", "));
    for b in bad {
        let jobs = json(gh(&["run", "view", &b.run.to_string(), "--json", "jobs"]));
        for job in jobs.iter().flat_map(|v| v["jobs"].as_array().into_iter().flatten()) {
            if text(job, "conclusion") == "failure" {
                println!("job: {}", text(job, "name"));
            }
        }
        println!("{}", failed_job_lines(b.run));
    }
    ExitCode::from(1)
}

fn shell(command: &str) -> Command {
    let mut c = Command::new("sh");
    c.arg("-c").arg(command);
    c
}

fn fallback_shell(command: &str) -> Command {
    let mut c = Command::new("cmd");
    c.arg("/C").arg(command);
    c
}

fn run_once(command: &str) -> (bool, String) {
    let started = std::io::pipe().and_then(|(reader, writer)| {
        let mut c = shell(command);
        c.stdin(Stdio::null()).stdout(writer.try_clone()?).stderr(writer.try_clone()?);
        let child = match c.spawn() {
            Ok(child) => child,
            Err(_) if cfg!(windows) => {
                let mut f = fallback_shell(command);
                f.stdin(Stdio::null()).stdout(writer.try_clone()?).stderr(writer);
                f.spawn()?
            }
            Err(e) => return Err(e),
        };
        drop(c);
        Ok((reader, child))
    });
    match started {
        Ok((reader, mut child)) => {
            let data = read_limited(reader, OUT_MAX);
            let ok = child.wait().is_ok_and(|s| s.success());
            (ok, decode(&data))
        }
        Err(e) => (false, format!("cannot start the command: {e}")),
    }
}

fn shown(out: &str) -> String {
    squeeze(out).map_or_else(|| clean(out), |s| s.plain)
}

pub fn until(args: &[String]) -> ExitCode {
    let mut command = None;
    let (mut every, mut timeout) = (UNTIL_EVERY, UNTIL_TIMEOUT);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        let (key, inline) = match a.split_once('=') {
            Some((k, v)) if k.starts_with("--") => (k, Some(v.to_string())),
            _ => (a.as_str(), None),
        };
        match key {
            "--every" | "--timeout" => {
                let parsed = inline.or_else(|| it.next().cloned()).as_deref().and_then(parse_duration);
                match (key, parsed) {
                    ("--every", Some(d)) if !d.is_zero() => every = d,
                    ("--timeout", Some(d)) => timeout = d,
                    _ => return fail(&format!("err: {key} needs a time like 5s or 10m -> try: {UNTIL_USAGE}")),
                }
            }
            _ if command.is_none() => command = Some(a.clone()),
            _ => return fail(&format!("err: one command in quotes -> try: {UNTIL_USAGE}")),
        }
    }
    let Some(command) = command else { return fail(&format!("err: which command? -> try: {UNTIL_USAGE}")) };
    let started = Instant::now();
    loop {
        let (ok, out) = run_once(&command);
        if ok {
            let body = shown(&out);
            if !body.trim().is_empty() {
                println!("{}", body.trim_end());
            }
            return ExitCode::SUCCESS;
        }
        if started.elapsed() + every > timeout {
            println!("err: timeout after {} s, last output:\n{}", started.elapsed().as_secs(), shown(&out).trim_end());
            return ExitCode::from(2);
        }
        thread::sleep(every);
    }
}

pub fn wanted(args: &[String]) -> bool {
    args.first().is_some_and(|a| a == "wait") && args.get(1).is_some_and(|a| a == "ci" || a == "until")
}

pub fn run(args: &[String]) -> ExitCode {
    if args.get(1).is_some_and(|a| a == "ci") { ci(&args[2..]) } else { until(&args[2..]) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn run(status: &str, conclusion: &str, id: u64) -> Value {
        json!({"databaseId": id, "status": status, "conclusion": conclusion, "headSha": "abc", "workflowName": "v3"})
    }

    #[test]
    fn runs_are_judged() {
        assert_eq!(judge(&[]), State::Waiting(0));
        assert_eq!(judge(&[run("in_progress", "", 1)]), State::Waiting(1));
        assert_eq!(judge(&[run("completed", "success", 1)]), State::Green(1));
        assert_eq!(judge(&[run("completed", "success", 1), run("in_progress", "", 2)]), State::Waiting(1));
        let red = judge(&[run("completed", "failure", 7), run("in_progress", "", 2)]);
        assert_eq!(red, State::Red(vec![Failed { run: 7, name: "v3".into() }]));
    }

    #[test]
    fn failure_marks_skip_the_ignored_steps() {
        let m = "fmt=success setup=failure selftest=failure lint=success";
        assert_eq!(failure_marks(m, &[]), ["setup=failure", "selftest=failure"]);
        assert_eq!(failure_marks(m, &["setup".to_string()]), ["selftest=failure"]);
        assert!(failure_marks("fmt=success", &[]).is_empty());
    }

    #[test]
    fn durations_and_backoff() {
        assert_eq!(parse_duration("30m"), Some(Duration::from_secs(1800)));
        assert_eq!(parse_duration("90"), Some(Duration::from_secs(90)));
        assert_eq!(parse_duration("2h"), Some(Duration::from_secs(7200)));
        assert_eq!(parse_duration("x"), None);
        assert_eq!(parse_duration("5q"), None);
        let mut p = POLL_FIRST;
        for _ in 0..10 {
            p = next_poll(p);
        }
        assert_eq!(p, POLL_MAX);
    }

    #[test]
    fn only_the_newest_commit_counts() {
        let mut old = run("completed", "failure", 1);
        old["headSha"] = json!("old");
        let kept = runs_of_newest_sha(vec![run("completed", "success", 2), old]);
        assert_eq!(kept.len(), 1);
    }

    #[test]
    fn wait_ci_and_until_are_ours() {
        let a = |s: &str| s.split(' ').map(String::from).collect::<Vec<_>>();
        assert!(wanted(&a("wait ci --pr 3")) && wanted(&a("wait until x")));
        assert!(!wanted(&a("wait text")) && !wanted(&a("snap")));
    }

    #[test]
    fn until_passes_and_times_out() {
        let ok = until(&["echo done".to_string(), "--every=1s".into(), "--timeout=5s".into()]);
        assert_eq!(format!("{ok:?}"), format!("{:?}", ExitCode::SUCCESS));
        let late = until(&["exit 1".to_string(), "--every=1s".into(), "--timeout=1s".into()]);
        assert_eq!(format!("{late:?}"), format!("{:?}", ExitCode::from(2)));
    }
}
