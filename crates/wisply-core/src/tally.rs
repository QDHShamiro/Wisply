use std::path::Path;

use serde_json::Value;

use crate::log::{rows, tail};
use crate::stamp_secs;

const NOT_COUNTED: &[&str] = &["selftest", "bench", "demo", "done", "ext-reload"];
const ACTION_VERBS: &[&str] =
    &["click", "type", "press", "fill", "select", "check", "scroll", "hover", "open", "launch", "do", "run"];
const SNAP_GAP_S: i64 = 15;
const ERROR_CHARS: usize = 90;
const LOOP_MIN: i64 = 20;
const APP_PREFIX: &str = "wisply-";

#[derive(Default)]
struct Tally {
    runs: i64,
    err: i64,
    chars: i64,
    sized: i64,
    ms: i64,
    snap_after: i64,
}

pub fn own_app(sid: &str) -> bool {
    sid.starts_with(APP_PREFIX)
}

fn sid_of(r: &Value) -> &str {
    r.get("sid").and_then(Value::as_str).unwrap_or_default()
}

fn failed(r: &Value) -> bool {
    !r.get("ok").is_some_and(truthy)
}

pub struct Loop {
    pub sid: String,
    pub cmd: String,
    pub error: String,
    pub count: i64,
    pub last: String,
}

impl Loop {
    fn holds(&self, r: &Value) -> bool {
        failed(r)
            && sid_of(r) == self.sid
            && r["cmd"].as_str() == Some(self.cmd.as_str())
            && shape(r["e"].as_str().unwrap_or_default()) == self.error
    }
}

pub struct Caller {
    pub sid: String,
    pub runs: i64,
    pub err: i64,
    pub top: Option<Failure>,
}

pub struct LogView {
    ai: Vec<Value>,
    apps: Vec<Value>,
    pub loops: Vec<Loop>,
}

pub fn view(home: &Path, last: usize) -> LogView {
    let mut rows = rows(&home.join("log.jsonl")).unwrap_or_default();
    rows.retain(|r| !r.get("x").is_some_and(truthy) && !NOT_COUNTED.contains(&r["cmd"].as_str().unwrap_or_default()));
    let (mut apps, mut ai): (Vec<Value>, Vec<Value>) = rows.into_iter().partition(|r| own_app(sid_of(r)));
    ai.drain(..tail(ai.len(), last));
    apps.drain(..tail(apps.len(), last));
    let loops = loops(&ai);
    ai.retain(|r| !loops.iter().any(|l| l.holds(r)));
    LogView { ai, apps, loops }
}

impl LogView {
    pub fn stats(&self) -> Vec<CmdStat> {
        to_stats(tallies(&self.ai))
    }

    pub fn failures(&self) -> Vec<Failure> {
        failure_list(&self.ai)
    }

    pub fn looped_runs(&self) -> i64 {
        self.loops.iter().map(|l| l.count).sum()
    }

    pub fn apps(&self) -> Vec<Caller> {
        let mut out: Vec<Caller> = Vec::new();
        for r in &self.apps {
            let sid = sid_of(r);
            let at = match out.iter().position(|c| c.sid == sid) {
                Some(at) => at,
                None => {
                    out.push(Caller { sid: sid.to_string(), runs: 0, err: 0, top: None });
                    out.len() - 1
                }
            };
            out[at].runs += 1;
            out[at].err += i64::from(failed(r));
        }
        for c in &mut out {
            let mine: Vec<Value> = self.apps.iter().filter(|r| sid_of(r) == c.sid).cloned().collect();
            c.top = failure_list(&mine).into_iter().next();
        }
        out.sort_by_key(|c| -c.runs);
        out
    }
}

fn loops(rows: &[Value]) -> Vec<Loop> {
    let mut seen: Vec<Loop> = Vec::new();
    for e in rows.iter().filter(|e| failed(e) && !sid_of(e).is_empty()) {
        let (sid, cmd) = (sid_of(e), e["cmd"].as_str().unwrap_or_default());
        let error = shape(e["e"].as_str().unwrap_or_default());
        let last = e["t"].as_str().unwrap_or_default().to_string();
        match seen.iter_mut().find(|l| l.sid == sid && l.cmd == cmd && l.error == error) {
            Some(l) => {
                l.count += 1;
                l.last = last;
            }
            None => seen.push(Loop { sid: sid.to_string(), cmd: cmd.to_string(), error, count: 1, last }),
        }
    }
    seen.retain(|l| l.count >= LOOP_MIN);
    seen.sort_by_key(|l| -l.count);
    seen
}

fn tallies(rows: &[Value]) -> Vec<(String, Tally)> {
    let mut seen: Vec<(String, Tally)> = Vec::new();
    let mut prev: Option<&Value> = None;
    for e in rows {
        let cmd = e["cmd"].as_str().unwrap_or_default();
        let at = match seen.iter().position(|(c, _)| c == cmd) {
            Some(at) => at,
            None => {
                seen.push((cmd.to_string(), Tally::default()));
                seen.len() - 1
            }
        };
        let s = &mut seen[at].1;
        s.runs += 1;
        s.err += i64::from(!e.get("ok").is_some_and(truthy));
        s.ms += e.get("ms").and_then(Value::as_f64).unwrap_or(0.0) as i64;
        if let Some(n) = e.get("n") {
            s.chars += n.as_f64().unwrap_or(0.0) as i64;
            s.sized += 1;
        }
        if let Some(p) = prev {
            let pcmd = p["cmd"].as_str().unwrap_or_default();
            if cmd == "snap" && ACTION_VERBS.contains(&pcmd) && !p.get("f").is_some_and(truthy) {
                let gap = stamp_secs(e["t"].as_str().unwrap_or_default())
                    .zip(stamp_secs(p["t"].as_str().unwrap_or_default()))
                    .map(|(a, b)| a - b);
                if gap.is_some_and(|g| g <= SNAP_GAP_S)
                    && let Some((_, t)) = seen.iter_mut().find(|(c, _)| c == pcmd)
                {
                    t.snap_after += 1;
                }
            }
        }
        prev = Some(e);
    }
    seen.sort_by_key(|(_, s)| -s.runs);
    seen
}

pub fn stats(home: &Path, last: usize) -> Option<String> {
    let seen = tallies(&view(home, last).ai);
    if seen.is_empty() {
        return None;
    }
    let mut out = vec!["cmd          runs  err%  avg ms  avg tok  snap after".to_string()];
    for (cmd, s) in &seen {
        let tok = if s.sized > 0 { format!("{:7}", s.chars / s.sized / 4 + 1) } else { "      -".into() };
        let after = if ACTION_VERBS.contains(&cmd.as_str()) {
            format!("{}%", 100 * s.snap_after / s.runs)
        } else {
            String::new()
        };
        out.push(format!("{cmd:<12} {:5} {:4}% {:7}  {tok}  {after}", s.runs, 100 * s.err / s.runs, s.ms / s.runs));
    }
    Some(out.join("\n"))
}

pub struct CmdStat {
    pub cmd: String,
    pub runs: i64,
    pub err: i64,
    pub avg_ms: i64,
    pub avg_tokens: i64,
    pub tokens: i64,
    pub snap_after_pct: Option<i64>,
}

pub fn command_stats(home: &Path, last: usize) -> Vec<CmdStat> {
    view(home, last).stats()
}

fn to_stats(seen: Vec<(String, Tally)>) -> Vec<CmdStat> {
    seen.into_iter()
        .map(|(cmd, s)| CmdStat {
            snap_after_pct: ACTION_VERBS.contains(&cmd.as_str()).then(|| 100 * s.snap_after / s.runs),
            avg_ms: s.ms / s.runs,
            avg_tokens: if s.sized > 0 { s.chars / s.sized / 4 + 1 } else { 0 },
            tokens: s.chars / 4,
            err: s.err,
            runs: s.runs,
            cmd,
        })
        .collect()
}

pub struct Failure {
    pub cmd: String,
    pub error: String,
    pub count: i64,
}

pub fn failures(home: &Path, last: usize) -> Vec<Failure> {
    view(home, last).failures()
}

fn failure_list(rows: &[Value]) -> Vec<Failure> {
    let mut seen: Vec<Failure> = Vec::new();
    for e in rows.iter().filter(|e| failed(e)) {
        let cmd = e["cmd"].as_str().unwrap_or_default();
        let error = shape(e["e"].as_str().unwrap_or_default());
        match seen.iter_mut().find(|f| f.cmd == cmd && f.error == error) {
            Some(f) => f.count += 1,
            None => seen.push(Failure { cmd: cmd.to_string(), error, count: 1 }),
        }
    }
    seen.sort_by_key(|f| -f.count);
    seen
}

fn shape(error: &str) -> String {
    let first = error.lines().next().unwrap_or_default();
    let mut out = String::new();
    let mut in_digits = false;
    for ch in first.chars().take(ERROR_CHARS) {
        if ch.is_ascii_digit() {
            if !in_digits {
                out.push('#');
            }
            in_digits = true;
        } else {
            in_digits = false;
            out.push(ch);
        }
    }
    out
}

fn truthy(v: &Value) -> bool {
    match v {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().is_some_and(|f| f != 0.0),
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn commands_and_repeated_errors_come_out_structured() {
        let home = std::env::temp_dir().join(format!("wisply-core-insight-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(&home).unwrap();
        let lines = [
            r#"{"t": "2026-10-30 12:00:00", "cmd": "click", "ok": false, "ms": 100, "n": 400, "e": "no element e12 on tab 3"}"#,
            r#"{"t": "2026-10-30 12:00:10", "cmd": "click", "ok": false, "ms": 300, "n": 400, "e": "no element e48 on tab 9"}"#,
            r#"{"t": "2026-10-30 12:00:20", "cmd": "click", "ok": true, "ms": 200, "n": 400}"#,
            r#"{"t": "2026-10-30 12:00:30", "cmd": "open", "ok": false, "ms": 50, "n": 8, "e": "timeout"}"#,
        ];
        fs::write(home.join("log.jsonl"), lines.join("\n") + "\n").unwrap();
        let rows = command_stats(&home, 2000);
        let click = rows.iter().find(|r| r.cmd == "click").unwrap();
        assert_eq!((click.runs, click.err, click.avg_ms, click.tokens, click.avg_tokens), (3, 2, 200, 300, 101));
        assert_eq!(click.snap_after_pct, Some(0));
        assert_eq!(rows.iter().find(|r| r.cmd == "open").unwrap().snap_after_pct, Some(0));
        let bad = failures(&home, 2000);
        assert_eq!((bad[0].cmd.as_str(), bad[0].error.as_str(), bad[0].count), ("click", "no element e# on tab #", 2));
        assert_eq!(bad[1].cmd, "open");
        assert!(command_stats(&home.join("none"), 10).is_empty() && failures(&home.join("none"), 10).is_empty());
        fs::remove_dir_all(&home).unwrap();
    }

    #[test]
    fn own_apps_and_one_session_loops_are_kept_apart() {
        let home = std::env::temp_dir().join(format!("wisply-core-apart-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(&home).unwrap();
        let mut lines = Vec::new();
        for n in 0..25 {
            lines.push(format!(r#"{{"t": "2026-11-26 10:{n:02}:00", "cmd": "click", "ok": false, "ms": 16000, "e": "nothing matches \"Mum\"", "sid": "dead-loop"}}"#));
        }
        lines.push(r#"{"t": "2026-11-26 11:00:00", "cmd": "click", "ok": false, "ms": 90, "e": "no element e3", "sid": "s-b"}"#.into());
        for _ in 0..3 {
            lines.push(r#"{"t": "2026-11-26 11:00:01", "cmd": "click", "ok": true, "ms": 30, "sid": "s-b"}"#.into());
        }
        for _ in 0..10 {
            lines.push(
                r#"{"t": "2026-11-26 11:00:02", "cmd": "agents", "ok": true, "ms": 5, "n": 60000, "sid": "wisply-ui"}"#
                    .into(),
            );
            lines.push(r#"{"t": "2026-11-26 11:00:03", "cmd": "go", "ok": false, "ms": 4, "e": "unknown order \"spielat\"", "sid": "wisply-ai"}"#.into());
        }
        fs::write(
            home.join("log.jsonl"),
            lines.join(
                "
",
            ) + "
",
        )
        .unwrap();
        let v = view(&home, 2000);
        let stats = v.stats();
        let click = stats.iter().find(|s| s.cmd == "click").unwrap();
        assert_eq!((click.runs, click.err, click.avg_ms), (4, 1, 45));
        assert!(stats.iter().all(|s| s.cmd != "agents" && s.cmd != "go"));
        assert_eq!(v.loops.len(), 1);
        let l = &v.loops[0];
        assert_eq!(
            (l.sid.as_str(), l.cmd.as_str(), l.count, l.last.as_str()),
            ("dead-loop", "click", 25, "2026-11-26 10:24:00")
        );
        assert_eq!(v.looped_runs(), 25);
        let apps = v.apps();
        let ai = apps.iter().find(|a| a.sid == "wisply-ai").unwrap();
        assert_eq!((ai.runs, ai.err, ai.top.as_ref().map(|f| f.count)), (10, 10, Some(10)));
        assert!(apps.iter().any(|a| a.sid == "wisply-ui" && a.err == 0 && a.top.is_none()));
        assert_eq!(v.failures().len(), 1);
        assert!(own_app("wisply-ui") && !own_app("7f3a-uuid"));
        fs::remove_dir_all(&home).unwrap();
    }
}
