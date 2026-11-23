use std::path::Path;
use std::sync::atomic::Ordering;

use super::engine::Verdict;
use super::{Ask, execute, explain, gate, scan_first, shell};

pub struct Plan {
    check: Option<String>,
    pub dry: bool,
    rounds: usize,
}

#[derive(Default)]
pub struct Done {
    pub text: String,
    pub ok: bool,
    pub halted: bool,
    pub calls: usize,
    pub seen: usize,
    pub secret: bool,
    pub checked: bool,
    pub rounds: usize,
}

pub struct Env<'a> {
    pub home: &'a Path,
    pub cwd: &'a Path,
    pub rules: &'a gate::Rules,
    pub shell: &'a shell::Shell,
}

pub fn plan(home: &Path, ask: &Ask) -> Plan {
    let check = ask.routine.as_deref().and_then(|name| crate::daemon::script_check(home, name));
    let dry = ask.args.get("dry").is_some_and(|v| v == "1");
    let rounds = if dry && check.is_some() { crate::daemon::PRACTICE_ROUNDS } else { 1 };
    Plan { check, dry, rounds }
}

fn once(env: &Env, ask: &Ask) -> Done {
    let bash = env.shell.kind != shell::Kind::Power;
    if let Some(stop) = scan_first(ask, env.cwd, env.rules, bash) {
        return Done { text: stop, halted: true, ..Done::default() };
    }
    let (ran, shared) = execute(env.home, ask, env.cwd.to_path_buf(), env.rules.clone(), env.shell.clone());
    let finished = shared.finished.load(Ordering::SeqCst);
    let worked = finished.saturating_sub(shared.failed.load(Ordering::SeqCst));
    let (text, ok) = explain(&ran, &ask.code, worked, ask.limit);
    Done {
        text,
        ok,
        halted: matches!(ran.verdict, Verdict::Halted(_)),
        calls: finished,
        seen: shared.seen.load(Ordering::Relaxed),
        secret: shared.secret.load(Ordering::SeqCst),
        ..Done::default()
    }
}

fn checked(env: &Env, ask: &Ask, check: &str, result: &str) -> Result<(), String> {
    let mut args = ask.args.clone();
    args.insert("result".into(), result.to_string());
    let probe = Ask { yes: false, limit: ask.limit, code: check.to_string(), args, routine: None };
    let got = once(env, &probe);
    let first = got.text.lines().next().unwrap_or("").trim();
    if !got.ok {
        return Err(first.strip_prefix("err: ").unwrap_or(first).to_string());
    }
    if got.text.trim() == "false" {
        return Err("the check returned false".into());
    }
    Ok(())
}

pub fn go(env: &Env, ask: &Ask, plan: &Plan) -> Done {
    let mut done = Done::default();
    for round in 0..plan.rounds {
        let got = once(env, ask);
        let (calls, seen, secret) = (done.calls + got.calls, done.seen + got.seen, done.secret || got.secret);
        done = Done { calls, seen, secret, rounds: done.rounds, ..got };
        let Some(check) = plan.check.as_deref().filter(|_| done.ok) else { break };
        if let Err(why) = checked(env, ask, check, &done.text) {
            done.ok = false;
            done.text = format!("err: the check failed: {why}\n{}", done.text);
            break;
        }
        done.rounds = round + 1;
    }
    done.checked = plan.check.is_some() && done.ok;
    if done.checked && plan.dry {
        done.text.push_str(&format!("\npractice: {}/{} dry runs passed the check", done.rounds, plan.rounds));
    }
    done
}
