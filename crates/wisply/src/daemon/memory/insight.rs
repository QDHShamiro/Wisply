use std::fmt::Write as _;

use serde_json::{Value, json};
use wisply_core::{Call, CmdStat, Fail, Failure, command_stats, failures};
use wisply_memory::{Todo, oneline};

use super::super::Daemon;
use super::with_store;

const USAGE: &str = "wisply insight | insight add \"title | why | proof\" | insight list | insight sessions";
const PROJECT: &str = "wisply";
const ORIGIN: &str = "insight";
const LAST: usize = 3000;
const LIGHT_LAST: usize = 200;
const MIN_RUNS: i64 = 8;
const ERR_PCT: i64 = 20;
const SLOW_MS: i64 = 3000;
const BIG_TOKENS: i64 = 5000;
const SNAP_PCT: i64 = 50;
const LIGHT_BAD_PCT: i64 = 30;
const LIGHT_WARN_PCT: i64 = 10;
const FAIL_SHOWN: usize = 8;
const TOKEN_SHOWN: usize = 5;
const LESSON_SHOWN: usize = 5;
const TASK_CHARS: usize = 120;
const PROOF_CHARS: usize = 160;
const SESSIONS_SHOWN: usize = 4;
const QUIET: &[&str] = &["board", "hook"];
const WAITS: &[&str] = &["wait", "listen", "watch", "research", "launch", "agents", "reply", "inbox"];

pub struct Finding {
    pub kind: &'static str,
    pub cmd: String,
    pub text: String,
    pub proof: String,
}

fn counted(stats: Vec<CmdStat>) -> Vec<CmdStat> {
    stats.into_iter().filter(|s| !QUIET.contains(&s.cmd.as_str())).collect()
}

fn pct(part: i64, whole: i64) -> i64 {
    if whole > 0 { 100 * part / whole } else { 0 }
}

fn top_failure<'a>(rows: &'a [Failure], cmd: &str) -> Option<&'a Failure> {
    rows.iter().find(|f| f.cmd == cmd)
}

pub fn findings(stats: &[CmdStat], fails: &[Failure]) -> Vec<Finding> {
    let mut out = Vec::new();
    for s in stats.iter().filter(|s| s.runs >= MIN_RUNS) {
        let waits = WAITS.contains(&s.cmd.as_str());
        let err = pct(s.err, s.runs);
        if err >= ERR_PCT && !waits {
            let proof = top_failure(fails, &s.cmd)
                .map_or_else(String::new, |f| format!("most common: {}x {}", f.count, oneline(&f.error, PROOF_CHARS)));
            out.push(Finding {
                kind: "errors",
                cmd: s.cmd.clone(),
                text: format!("{} fails {err}% of {} runs", s.cmd, s.runs),
                proof,
            });
        }
        if s.avg_ms >= SLOW_MS && !waits {
            out.push(Finding {
                kind: "slow",
                cmd: s.cmd.clone(),
                text: format!("{} takes {} ms on average over {} runs", s.cmd, s.avg_ms, s.runs),
                proof: String::new(),
            });
        }
        if s.avg_tokens >= BIG_TOKENS {
            out.push(Finding {
                kind: "tokens",
                cmd: s.cmd.clone(),
                text: format!("{} prints about {} tokens per run", s.cmd, s.avg_tokens),
                proof: format!("{} tokens in total", s.tokens),
            });
        }
        if let Some(after) = s.snap_after_pct.filter(|p| *p >= SNAP_PCT) {
            out.push(Finding {
                kind: "snap",
                cmd: s.cmd.clone(),
                text: format!("{} is followed by a full snap in {after}% of runs", s.cmd),
                proof: "the action result may not say what changed".into(),
            });
        }
    }
    out
}

fn light(stats: &[CmdStat]) -> Value {
    let runs: i64 = stats.iter().map(|s| s.runs).sum();
    let err: i64 = stats.iter().filter(|s| !WAITS.contains(&s.cmd.as_str())).map(|s| s.err).sum();
    let counted_runs: i64 = stats.iter().filter(|s| !WAITS.contains(&s.cmd.as_str())).map(|s| s.runs).sum();
    let ms: i64 = stats.iter().map(|s| s.avg_ms * s.runs).sum();
    let err_pct = pct(err, counted_runs);
    let tone = if err_pct >= LIGHT_BAD_PCT {
        "bad"
    } else if err_pct >= LIGHT_WARN_PCT {
        "warn"
    } else {
        "good"
    };
    json!({"tone": tone, "err_pct": err_pct, "avg_ms": if runs > 0 { ms / runs } else { 0 }, "runs": runs})
}

fn suggestions(rows: &[Todo]) -> Vec<&Todo> {
    rows.iter().filter(|t| t.origin == ORIGIN && t.status == "suggested").collect()
}

pub fn side(d: &Daemon) -> Result<Value, Fail> {
    let stats = counted(command_stats(&d.home, LIGHT_LAST));
    let rows = with_store(d, |s| s.entries(None, false))?;
    let todos = rows.iter().filter(|t| t.open()).count();
    let proposals = suggestions(&rows).len();
    Ok(json!({"light": light(&stats), "badges": {"todos": todos, "insight": proposals}}))
}

pub fn board(d: &Daemon) -> Result<Value, Fail> {
    let stats = counted(command_stats(&d.home, LAST));
    let fails: Vec<Failure> =
        failures(&d.home, LAST).into_iter().filter(|f| !QUIET.contains(&f.cmd.as_str())).collect();
    let found = findings(&stats, &fails);
    let commands: Vec<Value> = stats
        .iter()
        .map(|s| {
            json!({"cmd": s.cmd, "runs": s.runs, "err": s.err, "err_pct": pct(s.err, s.runs), "avg_ms": s.avg_ms,
                   "avg_tokens": s.avg_tokens, "tokens": s.tokens, "snap_after": s.snap_after_pct,
                   "waits": WAITS.contains(&s.cmd.as_str())})
        })
        .collect();
    let fail_rows: Vec<Value> =
        fails.iter().map(|f| json!({"cmd": f.cmd, "error": f.error, "count": f.count})).collect();
    let found_rows: Vec<Value> =
        found.iter().map(|f| json!({"kind": f.kind, "cmd": f.cmd, "text": f.text, "proof": f.proof})).collect();
    Ok(json!({"commands": commands, "failures": fail_rows, "findings": found_rows, "light": light(&stats)}))
}

fn report(d: &Daemon, with_sessions: bool) -> Result<String, Fail> {
    let stats = counted(command_stats(&d.home, LAST));
    if stats.is_empty() {
        return Ok("insight: the log is empty, nothing to read yet".into());
    }
    let fails: Vec<Failure> =
        failures(&d.home, LAST).into_iter().filter(|f| !QUIET.contains(&f.cmd.as_str())).collect();
    let runs: i64 = stats.iter().map(|s| s.runs).sum();
    let err: i64 = stats.iter().map(|s| s.err).sum();
    let mut out = format!("wisply insight: {runs} commands read, {}% failed\n", pct(err, runs));
    let found = findings(&stats, &fails);
    out.push_str("\nsuspicious (check each one before you believe it):\n");
    if found.is_empty() {
        out.push_str("  nothing stands out\n");
    }
    for (n, f) in found.iter().enumerate() {
        let _ = write!(out, "  {}. [{}] {}", n + 1, f.kind, f.text);
        if !f.proof.is_empty() {
            let _ = write!(out, " ({})", f.proof);
        }
        out.push('\n');
    }
    if !fails.is_empty() {
        out.push_str("\nrepeated errors:\n");
        for f in fails.iter().take(FAIL_SHOWN) {
            let _ = writeln!(out, "  {}x {}: {}", f.count, f.cmd, f.error);
        }
    }
    let mut eaters: Vec<&CmdStat> = stats.iter().filter(|s| s.tokens > 0).collect();
    eaters.sort_by_key(|s| -s.tokens);
    out.push_str("\ntoken eaters (all runs together):\n");
    for s in eaters.iter().take(TOKEN_SHOWN) {
        let _ = writeln!(out, "  {} {} tokens, {} runs, {} per run", s.cmd, s.tokens, s.runs, s.avg_tokens);
    }
    let lessons = with_store(d, |s| s.lessons())?;
    if !lessons.is_empty() {
        let _ = writeln!(out, "\nlessons Wisply learned from failed commands ({}):", lessons.len());
        for l in lessons.iter().take(LESSON_SHOWN) {
            let _ = writeln!(out, "  {} {} -> wisply {} (+{} -{})", l.cmd, l.kind, l.fix, l.ups, l.downs);
        }
    }
    if with_sessions {
        out.push_str(&sessions());
    }
    let rows = with_store(d, |s| s.entries(Some(PROJECT), false))?;
    let open = suggestions(&rows).len();
    if open > 0 {
        let _ = writeln!(out, "\nproposals waiting for the user: {open} (wisply insight list)");
    }
    out.push_str(
        "\nui: wisply shot app:Wisply shows the window, look at it and judge layout and clutter\n\
         next: wisply insight add \"title | why | proof\" for each real finding; the user accepts or rejects it in wisply ui under Improvements\n",
    );
    Ok(out)
}

fn sessions() -> String {
    let Ok(mut rows) = super::super::agents::board::snapshot(false) else {
        return "\nsessions: not readable right now\n".into();
    };
    rows.sort_by(|a, b| b.cost.total_cmp(&a.cost));
    let mut out = String::from("\nsessions (most expensive first):\n");
    for r in rows.iter().take(SESSIONS_SHOWN) {
        let _ = writeln!(out, "  {} {} {} tokens ~${:.0} {}", r.name, r.status.word(), r.tokens, r.cost, r.model);
    }
    out
}

fn add(d: &Daemon, c: &Call) -> Result<String, Fail> {
    let text = c.pos[1..].join(" ");
    let mut parts = text.split('|').map(str::trim);
    let title = parts.next().unwrap_or_default();
    let why = parts.next().unwrap_or_default();
    let proof = parts.next().unwrap_or_default();
    if title.is_empty() {
        return Err(Fail::hint("what is the improvement?", USAGE));
    }
    if proof.is_empty() {
        return Err(Fail::hint("an improvement needs a proof (the number or error that shows it)", USAGE));
    }
    let id = with_store(d, |s| {
        let Some(id) = s.suggest(PROJECT, title, ORIGIN)? else { return Ok(None) };
        s.todo_note(id, &format!("why: {why}"))?;
        s.todo_note(id, &format!("proof: {proof}"))?;
        Ok(Some(id))
    })?;
    match id {
        Some(id) => Ok(format!(
            "#{id} proposed: {} (the user accepts or rejects it in wisply ui, Improvements)",
            oneline(title, TASK_CHARS)
        )),
        None => Ok("not added: too short or the same proposal exists already".into()),
    }
}

fn list(d: &Daemon) -> Result<String, Fail> {
    let rows = with_store(d, |s| s.entries(Some(PROJECT), false))?;
    let open = suggestions(&rows);
    if open.is_empty() {
        return Ok("no proposals waiting".into());
    }
    Ok(open.iter().map(|t| format!("#{} {}", t.id, oneline(&t.task, TASK_CHARS))).collect::<Vec<_>>().join("\n"))
}

pub fn run(d: &Daemon, c: &Call) -> Result<String, Fail> {
    match c.pos.first().map(String::as_str) {
        None => report(d, false),
        Some("sessions") => report(d, true),
        Some("add") => add(d, c),
        Some("list") => list(d),
        Some(_) => Err(Fail::hint("what should insight do?", USAGE)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stat(cmd: &str, runs: i64, err: i64, avg_ms: i64, avg_tokens: i64) -> CmdStat {
        CmdStat { cmd: cmd.into(), runs, err, avg_ms, avg_tokens, tokens: avg_tokens * runs, snap_after_pct: None }
    }

    #[test]
    fn a_failing_command_is_suspicious_with_its_error_as_proof() {
        let stats = [stat("click", 20, 13, 100, 10)];
        let fails = [Failure { cmd: "click".into(), error: "no element e#".into(), count: 9 }];
        let found = findings(&stats, &fails);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].kind, "errors");
        assert!(found[0].text.contains("65%"));
        assert!(found[0].proof.contains("9x no element e#"));
    }

    #[test]
    fn waiting_commands_may_be_slow_and_fail() {
        let stats = [stat("wait", 20, 15, 40_000, 10)];
        assert!(findings(&stats, &[]).is_empty());
    }

    #[test]
    fn few_runs_prove_nothing() {
        let stats = [stat("open", 3, 3, 90_000, 20_000)];
        assert!(findings(&stats, &[]).is_empty());
    }

    #[test]
    fn big_outputs_and_snap_habits_are_named() {
        let mut after = stat("click", 10, 0, 50, 20);
        after.snap_after_pct = Some(80);
        let found = findings(&[stat("agents", 9, 0, 50, 15_000), after], &[]);
        let kinds: Vec<&str> = found.iter().map(|f| f.kind).collect();
        assert_eq!(kinds, ["tokens", "snap"]);
    }

    #[test]
    fn the_light_turns_red_with_many_errors_and_ignores_waits() {
        assert_eq!(light(&[stat("click", 10, 5, 100, 1)])["tone"], "bad");
        assert_eq!(light(&[stat("click", 10, 1, 100, 1)])["tone"], "warn");
        assert_eq!(light(&[stat("click", 10, 0, 100, 1), stat("wait", 10, 10, 100, 1)])["tone"], "good");
    }
}
