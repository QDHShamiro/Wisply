use std::sync::LazyLock;

use regex::Regex;
use wisply_core::{Call, Fail};
use wisply_memory::{Todo, date, now, oneline};

use super::super::Daemon;
use super::super::ctx::Ctx;
use super::list::set;
use super::{TASK_SHORT, with_store};

const DAY_S: f64 = 86_400.0;
const IDLE_DAYS: f64 = 14.0;
const DOING_DAYS: f64 = 7.0;
const CENTURY: u32 = 2000;
const CLOSE: [(&str, &str); 2] = [("done", "done"), ("drop", "dropped")];
const CLOSE_HINT: &str = "close with: wisply todo tidy --done 3,7 or --drop 4 (wisply todo undo <id> puts one back)";

static DUE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"(?i)\b(?:bis|bis zum|until|by|am|on|frist|deadline|due|spätestens|before)\s+",
        r"(?:(\d{4})-(\d{1,2})-(\d{1,2})|(\d{1,2})\.(\d{1,2})\.(\d{2,4})?)"
    ))
    .expect("due pattern")
});

type Day = (u32, u32, u32);

fn day_of(t: f64) -> Option<Day> {
    let text = date(t);
    let mut parts = text.split('-').map(|p| p.parse::<u32>().ok());
    Some((parts.next()??, parts.next()??, parts.next()??))
}

fn due(task: &str, created: Day) -> Option<Day> {
    let caps = DUE.captures(task)?;
    let num = |i: usize| caps.get(i).and_then(|m| m.as_str().parse::<u32>().ok());
    let (y, m, d) = match (num(1), num(2), num(3)) {
        (Some(y), Some(m), Some(d)) => (y, m, d),
        _ => {
            let (d, m) = (num(4)?, num(5)?);
            let y = match num(6) {
                Some(y) if y < 100 => CENTURY + y,
                Some(y) => y,
                None if (m, d) < (created.1, created.2) => created.0 + 1,
                None => created.0,
            };
            (y, m, d)
        }
    };
    ((1..=12).contains(&m) && (1..=31).contains(&d)).then_some((y, m, d))
}

pub fn stale(t: &Todo, at: f64) -> Option<String> {
    let today = day_of(at)?;
    if let Some((y, m, d)) = due(&t.task, day_of(t.created).unwrap_or(today))
        && (y, m, d) < today
    {
        return Some(format!("date {y:04}-{m:02}-{d:02} passed"));
    }
    let touched = if t.updated > 0.0 { t.updated } else { t.created };
    let idle = (at - touched) / DAY_S;
    if t.status == "doing" && t.note.trim().is_empty() && idle >= DOING_DAYS {
        return Some(format!("doing for {idle:.0} days without a note"));
    }
    (t.status != "waiting" && idle >= IDLE_DAYS).then(|| format!("untouched for {idle:.0} days"))
}

pub fn count(todos: &[Todo]) -> usize {
    let at = now();
    todos.iter().filter(|t| stale(t, at).is_some()).count()
}

pub fn tidy(d: &Daemon, ctx: &Ctx, c: &Call, here: &str) -> Result<String, Fail> {
    for (flag, status) in CLOSE {
        if c.on(flag) {
            let ids = c.val(flag).map_or_else(|| c.pos.get(1..).unwrap_or_default().join(","), str::to_string);
            if ids.trim().is_empty() {
                return Err(Fail::hint(format!("tidy --{flag} needs to-do numbers"), CLOSE_HINT));
            }
            let mut lines = Vec::new();
            for id in ids.split([',', ' ']).map(str::trim).filter(|s| !s.is_empty()) {
                let t = set(d, ctx, id, "status", status, "")?;
                lines.push(format!("#{} is {}: {}", t.id, t.status, oneline(&t.task, TASK_SHORT)));
            }
            return Ok(lines.join("\n"));
        }
    }
    let open = with_store(d, |s| s.todos(Some(here).filter(|h| !h.is_empty()), true))?;
    let at = now();
    let rows: Vec<String> = open
        .iter()
        .filter_map(|t| stale(t, at).map(|why| format!("#{} {why}: {}", t.id, oneline(&t.task, TASK_SHORT))))
        .collect();
    if rows.is_empty() {
        return Ok(format!("no stale to-dos among {} open", open.len()));
    }
    Ok(format!(
        "{} of {} open to-dos look stale (nothing closed):\n{}\n{CLOSE_HINT}",
        rows.len(),
        open.len(),
        rows.join("\n")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOON: f64 = 1_795_435_200.0;

    fn todo(task: &str, status: &str, note: &str, days_old: f64) -> Todo {
        let created = NOON - days_old * DAY_S;
        Todo {
            task: task.into(),
            status: status.into(),
            note: note.into(),
            created,
            updated: created,
            ..Todo::default()
        }
    }

    #[test]
    fn dates_in_the_text_and_idle_time_make_a_todo_stale() {
        assert_eq!(date(NOON), "2026-11-23");
        let fresh = |task: &str| stale(&todo(task, "todo", "", 2.0), NOON);
        assert_eq!(fresh("Abgabe bis 22.11.").as_deref(), Some("date 2026-11-22 passed"));
        assert_eq!(fresh("deadline 2026-01-05 report").as_deref(), Some("date 2026-01-05 passed"));
        assert_eq!(fresh("Termin am 3.12.25").as_deref(), Some("date 2025-12-03 passed"));
        assert!(fresh("seit 22.11. kaputt").is_none());
        assert!(fresh("bis 30.11. fertig").is_none());
        assert!(fresh("bis 05.01.").is_none());
        assert!(fresh("bis 45.13.").is_none());
        assert_eq!(stale(&todo("x", "todo", "", 15.0), NOON).as_deref(), Some("untouched for 15 days"));
        assert!(stale(&todo("x", "waiting", "", 20.0), NOON).is_none());
        assert_eq!(stale(&todo("x", "doing", "", 8.0), NOON).as_deref(), Some("doing for 8 days without a note"));
        assert!(stale(&todo("x", "doing", "half done", 8.0), NOON).is_none());
        let mut touched = todo("x", "todo", "", 30.0);
        touched.updated = NOON - DAY_S;
        assert!(stale(&touched, NOON).is_none());
    }
}
