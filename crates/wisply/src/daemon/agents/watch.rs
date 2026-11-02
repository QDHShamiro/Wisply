use std::fs::OpenOptions;
use std::io::Write;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use serde_json::{Map, Value};
use wisply_core::local_parts;

use super::board::{self, now, resume};
use super::events::emit;
use super::limit::{Change, Guard};
use super::model::{Row, Status};
use crate::daemon::config;
use crate::daemon::pc;

const POLL: Duration = Duration::from_secs(15);
const OFF_POLL: Duration = Duration::from_secs(30);
const BOARD_STALE_S: i64 = 120;
const RESUME_TEXT: &str = "weiter";

#[derive(Debug, PartialEq)]
pub struct Note {
    pub kind: &'static str,
    pub id: Option<String>,
    pub title: String,
    pub body: String,
}

fn clock(at: i64) -> String {
    let [_, _, _, h, m, _] = local_parts(at as f64);
    format!("{h:02}:{m:02}")
}

pub fn notes_for(changes: &[Change], rows: &[Row]) -> Vec<Note> {
    let mut notes = Vec::new();
    let mut limited: Vec<&Row> = Vec::new();
    for c in changes {
        let Some(row) = rows.iter().find(|r| r.id == c.id) else { continue };
        let name = if row.name.is_empty() { row.dir_short.as_str() } else { row.name.as_str() };
        match (c.old, c.new) {
            (Some(Status::Running), Status::Done) => {
                notes.push(Note {
                    kind: "done",
                    id: Some(row.id.clone()),
                    title: format!("{name} is done"),
                    body: row.last_line.clone(),
                });
            }
            (Some(Status::Running), Status::Error) | (None, Status::Error) => {
                notes.push(Note {
                    kind: "done",
                    id: Some(row.id.clone()),
                    title: format!("{name} failed"),
                    body: row.last_line.clone(),
                });
            }
            (_, Status::Waiting) => {
                notes.push(Note {
                    kind: "waiting",
                    id: Some(row.id.clone()),
                    title: format!("{name} is waiting"),
                    body: row.last_line.clone(),
                });
            }
            (_, Status::Limit) => limited.push(row),
            _ => {}
        }
    }
    match limited.as_slice() {
        [] => {}
        [one] => notes.push(Note {
            kind: "limit",
            id: Some(one.id.clone()),
            title: "usage limit reached".into(),
            body: one
                .reset_at
                .map_or_else(|| one.name.clone(), |at| format!("{} resumes after {}", one.name, clock(at))),
        }),
        many => {
            let at = many.iter().filter_map(|r| r.reset_at).max();
            notes.push(Note {
                kind: "limit",
                id: None,
                title: "usage limit reached".into(),
                body: format!(
                    "{} sessions stopped{}",
                    many.len(),
                    at.map_or_else(String::new, |a| format!(", back after {}", clock(a)))
                ),
            });
        }
    }
    notes
}

fn log_line(home: &Path, text: &str) {
    let dir = home.join("agents");
    let _ = std::fs::create_dir_all(&dir);
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(dir.join("guard.log")) {
        let _ = writeln!(f, "{} {text}", wisply_core::local_now());
    }
}

fn write_board(home: &Path, rows: &[Row], guard: &Guard, at: i64) {
    let count = |s: Status| rows.iter().filter(|r| r.status == s).count();
    let reset = if guard.hit() && guard.reset_at > 0 { clock(guard.reset_at) } else { String::new() };
    let body = serde_json::json!({
        "t": at,
        "running": count(Status::Running),
        "waiting": count(Status::Waiting),
        "limit": count(Status::Limit),
        "reset": reset,
    });
    let dir = home.join("agents");
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join("board.json"), body.to_string());
}

pub fn status_line(home: &Path) -> Option<String> {
    let v: Value = serde_json::from_str(&std::fs::read_to_string(home.join("agents").join("board.json")).ok()?).ok()?;
    if now() - v["t"].as_i64()? > BOARD_STALE_S {
        return None;
    }
    let n = |k: &str| v[k].as_i64().unwrap_or(0);
    Some(format!("agents | {} running | {} waiting | {} limit", n("running"), n("waiting"), n("limit")))
}

fn flag(cfg: &Map<String, Value>, key: &str) -> bool {
    config::on(cfg, key)
}

pub fn tick(home: &Path, guard: &mut Guard) {
    let cfg = config::load(home);
    let Ok(rows) = board::snapshot(true) else { return };
    let at = now();
    write_board(home, &rows, guard, at);
    let before = (guard.reset_at, guard.waiting.len(), guard.last.clone());
    let changes = guard.observe(&rows, at);
    let toast = flag(&cfg, "agents_notify");
    for n in notes_for(&changes, &rows) {
        emit(home, n.kind, n.id.as_deref(), &n.title, &n.body, toast);
    }
    let mut due = Vec::new();
    if flag(&cfg, "agents_resume") {
        due = guard.due(at, pc::facts(home).playing);
    }
    if (guard.reset_at, guard.waiting.len(), guard.last.clone()) != before || !due.is_empty() {
        guard.save(home);
    }
    for w in due {
        match resume(&w.sid, &w.dir, RESUME_TEXT) {
            Ok(()) => {
                log_line(home, &format!("resumed {}", w.id));
                let name = rows.iter().find(|r| r.id == w.id).map_or("session", |r| r.name.as_str());
                emit(
                    home,
                    "resumed",
                    Some(&w.id),
                    "session resumed",
                    &format!("{name} continues after the limit"),
                    toast,
                );
            }
            Err(e) => log_line(home, &format!("resume {} failed: {}", w.id, e.msg)),
        }
    }
}

pub fn start(home: PathBuf) {
    let _ = thread::Builder::new().name("agents".into()).spawn(move || {
        let mut guard = Guard::load(&home);
        loop {
            if !flag(&config::load(&home), "agents") {
                thread::sleep(OFF_POLL);
                continue;
            }
            if panic::catch_unwind(AssertUnwindSafe(|| tick(&home, &mut guard))).is_err() {
                eprintln!("wisply agents: a check failed, see the panic above");
            }
            thread::sleep(POLL);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daemon::agents::api::fake::row;

    fn change(id: &str, old: Option<Status>, new: Status) -> Change {
        Change { id: id.into(), old, new }
    }

    #[test]
    fn finishing_waiting_and_limits_become_short_notes() {
        let mut a = row("a", Status::Done);
        a.last_line = "all green".into();
        let b = row("b", Status::Waiting);
        let c = row("c", Status::Limit);
        let d = row("d", Status::Limit);
        let rows = vec![a, b, c, d];
        let changes = vec![
            change("a", Some(Status::Running), Status::Done),
            change("b", Some(Status::Running), Status::Waiting),
            change("c", Some(Status::Running), Status::Limit),
            change("d", None, Status::Limit),
        ];
        let notes = notes_for(&changes, &rows);
        let kinds: Vec<&str> = notes.iter().map(|n| n.kind).collect();
        assert_eq!(kinds, ["done", "waiting", "limit"]);
        assert_eq!(notes[0].body, "all green");
        assert!(notes[2].id.is_none() && notes[2].body.starts_with("2 sessions stopped"), "{:?}", notes[2]);
    }

    #[test]
    fn quiet_changes_make_no_note() {
        let rows = vec![row("a", Status::Done), row("b", Status::Running), row("c", Status::Done)];
        let changes = vec![
            change("a", Some(Status::Waiting), Status::Done),
            change("b", None, Status::Running),
            change("c", None, Status::Done),
            change("gone", Some(Status::Running), Status::Done),
        ];
        assert!(notes_for(&changes, &rows).is_empty());
        let single = notes_for(&[change("a", Some(Status::Running), Status::Limit)], &[row("a", Status::Limit)]);
        assert_eq!(single.len(), 1);
        assert_eq!(single[0].id.as_deref(), Some("a"));
    }
}
