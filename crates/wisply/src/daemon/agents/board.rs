use std::path::Path;
use std::sync::{Mutex, PoisonError};
use std::time::{Duration, Instant};

use serde_json::Value;
use wisply_core::{Fail, local_parts};

use super::limit::reset_utc;
use super::model::{Row, Status, last_folder, status_of};
use super::secret::scrub;
use super::transcript::stats_of;
use crate::daemon::claude::{claude_in, claude_out, log_tail};

const FRESH: Duration = Duration::from_secs(4);
const LINE_MAX: usize = 140;
const NAME_MAX: usize = 60;
const TAIL_LINE_MAX: usize = 200;
const TEXT_MAX: usize = 4000;

struct Snap {
    at: Instant,
    rows: Vec<Row>,
}

static SNAP: Mutex<Option<Snap>> = Mutex::new(None);

pub fn safe_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 64 && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
}

fn secs(ms: Option<f64>) -> i64 {
    ms.map_or(0, |m| (m / 1000.0) as i64)
}

pub fn rows_from(list: &Value, now: i64) -> Vec<Row> {
    let parts = local_parts(now as f64);
    let mut rows: Vec<Row> = list
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|r| {
            let id = r["id"].as_str().filter(|i| safe_id(i))?.to_string();
            let sid = r["sessionId"].as_str().unwrap_or("").to_string();
            let stats = stats_of(&sid);
            let state = r["state"].as_str().unwrap_or("");
            let working = state == "working" || (state.is_empty() && r["status"] == "busy");
            let limit = stats.limit.as_deref().filter(|_| !working);
            let status = status_of(state, r["status"].as_str(), limit.is_some());
            let waited = r["waitingFor"].as_str().filter(|w| !w.is_empty());
            let said = match (status, waited) {
                (Status::Waiting, Some(w)) => format!("waiting for {w}"),
                _ => stats.last_text.clone(),
            };
            let cwd = r["cwd"].as_str().unwrap_or("");
            let started = secs(r["startedAt"].as_f64());
            Some(Row {
                id,
                sid,
                name: scrub(r["name"].as_str().unwrap_or(""), NAME_MAX),
                dir: cwd.to_string(),
                dir_short: scrub(&last_folder(cwd), NAME_MAX),
                background: r["kind"] == "background",
                status,
                since: if stats.ended_ms > 0 { stats.ended_ms / 1000 } else { started },
                last_line: scrub(&said, LINE_MAX),
                tokens: stats.tokens,
                cost: stats.cost,
                model: stats.model.trim_start_matches("claude-").to_string(),
                effort: stats.effort.clone(),
                reset_at: limit.and_then(|t| reset_utc(t, parts)),
            })
        })
        .collect();
    rows.sort_by_key(|r| (rank(r.status), -r.since));
    rows
}

fn rank(s: Status) -> u8 {
    match s {
        Status::Waiting => 0,
        Status::Limit => 1,
        Status::Error => 2,
        Status::Running => 3,
        Status::Done => 4,
    }
}

pub fn now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64)
}

pub fn fetch() -> Result<Vec<Row>, Fail> {
    let raw = claude_out(&["agents", "--json", "--all"])?;
    let list: Value = serde_json::from_str(&raw).map_err(|_| Fail::new("claude agents gave no JSON"))?;
    Ok(rows_from(&list, now()))
}

pub fn snapshot(force: bool) -> Result<Vec<Row>, Fail> {
    {
        let guard = SNAP.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(s) = guard.as_ref().filter(|s| !force && s.at.elapsed() < FRESH) {
            return Ok(s.rows.clone());
        }
    }
    let rows = fetch()?;
    *SNAP.lock().unwrap_or_else(PoisonError::into_inner) = Some(Snap { at: Instant::now(), rows: rows.clone() });
    Ok(rows)
}

pub fn tail(row: &Row) -> Vec<String> {
    let raw = claude_out(&["logs", &row.id]).unwrap_or_default();
    log_tail(&raw, &row.id).lines().map(|l| scrub(l, TAIL_LINE_MAX)).filter(|l| !l.is_empty()).collect()
}

pub fn resume(row_sid: &str, dir: &str, text: &str) -> Result<(), Fail> {
    if !safe_id(row_sid) {
        return Err(Fail::new("bad session id"));
    }
    let clean: String = text.chars().filter(|c| !c.is_control() || *c == '\n').take(TEXT_MAX).collect();
    let clean = clean.trim();
    if clean.is_empty() {
        return Err(Fail::new("empty text"));
    }
    let prompt = if clean.starts_with('-') { format!(" {clean}") } else { clean.to_string() };
    claude_in(&["--bg", "--resume", row_sid, &prompt], Some(Path::new(dir))).map(|_| ())
}

pub fn stop(id: &str) -> Result<(), Fail> {
    if !safe_id(id) {
        return Err(Fail::new("bad session id"));
    }
    claude_out(&["stop", id]).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn rows_come_from_the_claude_list_and_hide_the_full_path() {
        let list = json!([
            {"id": "aa11", "sessionId": "no-such-session", "cwd": "C:\\work\\proj", "kind": "background",
             "startedAt": 1_730_000_000_000_i64, "name": "build it", "state": "working", "status": "busy"},
            {"id": "bb22", "sessionId": "", "cwd": "/x/y", "kind": "background", "startedAt": 1_730_000_100_000_i64,
             "name": "ask me", "state": "blocked", "waitingFor": "Bash"},
            {"id": "bad id!", "sessionId": "", "cwd": "", "kind": "background", "startedAt": 1, "name": "x", "state": "done"}
        ]);
        let rows = rows_from(&list, 1_730_001_000);
        assert_eq!(rows.len(), 2);
        assert_eq!((rows[0].id.as_str(), rows[0].status), ("bb22", Status::Waiting));
        assert_eq!(rows[0].last_line, "waiting for Bash");
        assert_eq!((rows[1].dir_short.as_str(), rows[1].status), ("proj", Status::Running));
        assert_eq!(rows[1].since, 1_730_000_000);
        let wire = rows[1].json().to_string();
        assert!(!wire.contains("work") && !wire.contains("sessionId") && !wire.contains("no-such"), "{wire}");
    }

    #[test]
    fn ids_are_checked_before_they_reach_a_command_line() {
        for bad in ["", "a b", "a;b", "../x", "a\"b", &"x".repeat(65)] {
            assert!(!safe_id(bad), "{bad}");
        }
        assert!(safe_id("ab12-CD"));
        assert!(resume("a b", "", "hi").is_err() && resume("ab12", "", "  ").is_err());
        assert!(stop("a;b").is_err());
    }
}
