use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde_json::{Value, json};

use super::model::{Row, Status};

pub const GRACE_S: i64 = 120;
const STALE_S: i64 = 3600;
const DAY_S: i64 = 86_400;
const FILE: &str = "guard.json";
const MONTHS: [&str; 12] = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"];

static RESET: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)resets?\s+(?:on\s+)?(?:([a-z]{3,9})\.?\s+(\d{1,2}),?\s+)?(?:at\s+)?(\d{1,2})(?::(\d{2}))?\s*(am|pm)?",
    )
    .expect("reset")
});
static EPOCH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)limit reached\s*\|\s*(\d{10})\b").expect("epoch"));

#[derive(Debug, PartialEq)]
pub enum Reset {
    Local(i64),
    Utc(i64),
}

pub fn parse_reset(text: &str, now: [u32; 6]) -> Option<Reset> {
    if let Some(m) = EPOCH.captures(text) {
        return m[1].parse().ok().map(Reset::Utc);
    }
    let m = RESET.captures(text)?;
    let mut hour: u32 = m[3].parse().ok()?;
    let minute: u32 = m.get(4).map_or(Some(0), |x| x.as_str().parse().ok())?;
    match m.get(5).map(|x| x.as_str().to_lowercase()).as_deref() {
        Some("am") if hour == 12 => hour = 0,
        Some("pm") if hour < 12 => hour += 12,
        _ => {}
    }
    if hour > 23 || minute > 59 {
        return None;
    }
    let stamp = |y: u32, mo: u32, d: u32| format!("{y:04}-{mo:02}-{d:02} {hour:02}:{minute:02}:00");
    let [year, month, day, h, mi, s] = now;
    let now_local = wisply_core::stamp_secs(&format!("{year:04}-{month:02}-{day:02} {h:02}:{mi:02}:{s:02}"))?;
    if let (Some(name), Some(d)) = (m.get(1), m.get(2)) {
        let key = name.as_str().to_lowercase();
        let mo = MONTHS.iter().position(|x| key.starts_with(x))? as u32 + 1;
        let d: u32 = d.as_str().parse().ok()?;
        let at = wisply_core::stamp_secs(&stamp(year, mo, d))?;
        let at = if at < now_local - DAY_S { wisply_core::stamp_secs(&stamp(year + 1, mo, d))? } else { at };
        return Some(Reset::Local(at));
    }
    let at = wisply_core::stamp_secs(&stamp(year, month, day))?;
    Some(Reset::Local(if at <= now_local { at + DAY_S } else { at }))
}

pub fn reset_utc(text: &str, now: [u32; 6]) -> Option<i64> {
    match parse_reset(text, now)? {
        Reset::Utc(at) => Some(at),
        Reset::Local(at) => Some(wisply_core::local_to_utc(at)),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Waiting {
    pub id: String,
    pub sid: String,
    pub dir: String,
    pub resumed: bool,
}

#[derive(Debug, PartialEq)]
pub struct Change {
    pub id: String,
    pub old: Option<Status>,
    pub new: Status,
}

#[derive(Default, Debug)]
pub struct Guard {
    pub reset_at: i64,
    pub waiting: Vec<Waiting>,
    pub last: BTreeMap<String, Status>,
}

impl Guard {
    pub fn load(home: &Path) -> Guard {
        let v: Value = fs::read_to_string(home.join("agents").join(FILE))
            .ok()
            .and_then(|t| serde_json::from_str(&t).ok())
            .unwrap_or(Value::Null);
        let text = |x: &Value, k: &str| x[k].as_str().unwrap_or("").to_string();
        Guard {
            reset_at: v["reset_at"].as_i64().unwrap_or(0),
            waiting: v["waiting"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|w| Waiting {
                    id: text(w, "id"),
                    sid: text(w, "sid"),
                    dir: text(w, "dir"),
                    resumed: w["resumed"].as_bool().unwrap_or(false),
                })
                .collect(),
            last: v["last"]
                .as_object()
                .into_iter()
                .flatten()
                .filter_map(|(k, s)| Some((k.clone(), Status::parse(s.as_str()?)?)))
                .collect(),
        }
    }

    pub fn save(&self, home: &Path) {
        let dir = home.join("agents");
        let waiting: Vec<Value> = self
            .waiting
            .iter()
            .map(|w| json!({"id": w.id, "sid": w.sid, "dir": w.dir, "resumed": w.resumed}))
            .collect();
        let last: serde_json::Map<String, Value> =
            self.last.iter().map(|(k, s)| (k.clone(), json!(s.word()))).collect();
        let body = json!({"reset_at": self.reset_at, "waiting": waiting, "last": last});
        let _ = fs::create_dir_all(&dir);
        let _ = fs::write(dir.join(FILE), body.to_string());
    }

    pub fn observe(&mut self, rows: &[Row], now: i64) -> Vec<Change> {
        let first = self.last.is_empty();
        let mut changes = Vec::new();
        for row in rows {
            let old = self.last.get(&row.id).copied();
            if old != Some(row.status) {
                if old.is_some() || !first {
                    changes.push(Change { id: row.id.clone(), old, new: row.status });
                }
                self.last.insert(row.id.clone(), row.status);
            }
            if row.status != Status::Limit || !row.background || self.waiting.iter().any(|w| w.id == row.id) {
                continue;
            }
            let at = row.reset_at.unwrap_or(0);
            let ran = old == Some(Status::Running) || (old.is_none() && at > now - STALE_S);
            if ran {
                self.waiting.push(Waiting {
                    id: row.id.clone(),
                    sid: row.sid.clone(),
                    dir: row.dir.clone(),
                    resumed: false,
                });
                self.reset_at = self.reset_at.max(at);
            }
        }
        let live: Vec<&str> = rows.iter().filter(|r| r.status == Status::Limit).map(|r| r.id.as_str()).collect();
        self.waiting.retain(|w| w.resumed || live.contains(&w.id.as_str()));
        self.last.retain(|id, _| rows.iter().any(|r| &r.id == id));
        if self.waiting.iter().all(|w| w.resumed) {
            self.reset_at = 0;
        }
        changes
    }

    pub fn due(&mut self, now: i64, playing: bool) -> Vec<Waiting> {
        if self.reset_at == 0 || now < self.reset_at + GRACE_S || playing {
            return Vec::new();
        }
        let mut out = Vec::new();
        for w in self.waiting.iter_mut().filter(|w| !w.resumed) {
            w.resumed = true;
            out.push(w.clone());
        }
        out
    }

    pub fn hit(&self) -> bool {
        self.waiting.iter().any(|w| !w.resumed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: [u32; 6] = [2026, 11, 2, 1, 0, 0];

    fn local(stamp: &str) -> Option<Reset> {
        wisply_core::stamp_secs(stamp).map(Reset::Local)
    }

    #[test]
    fn reset_times_in_all_known_shapes() {
        let cases = [
            ("You've hit your session limit · resets 3:40am (Europe/Berlin)", "2026-11-02 03:40:00"),
            ("hit your limit · resets 12:05am", "2026-11-03 00:05:00"),
            ("usage limit · resets 3pm", "2026-11-02 15:00:00"),
            ("weekly limit · resets Nov 5, 3pm (Europe/Berlin)", "2026-11-05 15:00:00"),
            ("weekly limit · resets Jan 2 at 9am", "2027-01-02 09:00:00"),
            ("limit · resets 12pm", "2026-11-02 12:00:00"),
        ];
        for (text, want) in cases {
            assert_eq!(parse_reset(text, NOW), local(want), "{text}");
        }
        assert_eq!(parse_reset("Claude AI usage limit reached|1730000000", NOW), Some(Reset::Utc(1_730_000_000)));
        assert_eq!(parse_reset("resets 25:00", NOW), None);
        assert_eq!(parse_reset("nothing here", NOW), None);
    }

    fn row(id: &str, status: Status, reset: Option<i64>) -> Row {
        Row {
            id: id.into(),
            sid: format!("sid-{id}"),
            name: "n".into(),
            dir: "d".into(),
            dir_short: "d".into(),
            background: true,
            status,
            since: 0,
            last_line: String::new(),
            tokens: 0,
            cost: 0.0,
            model: String::new(),
            effort: String::new(),
            reset_at: reset,
        }
    }

    #[test]
    fn a_running_session_that_hits_the_limit_is_resumed_exactly_once_after_reset() {
        let mut g = Guard::default();
        g.observe(&[row("a", Status::Running, None), row("b", Status::Done, None)], 1000);
        let ch = g.observe(&[row("a", Status::Limit, Some(5000)), row("b", Status::Done, None)], 2000);
        assert_eq!(ch, vec![Change { id: "a".into(), old: Some(Status::Running), new: Status::Limit }]);
        assert!(g.hit());
        assert!(g.due(5000 + GRACE_S - 1, false).is_empty());
        assert!(g.due(5000 + GRACE_S, true).is_empty());
        let go = g.due(5000 + GRACE_S, false);
        assert_eq!(go.len(), 1);
        assert_eq!(go[0].sid, "sid-a");
        assert!(g.due(9000, false).is_empty());
        g.observe(&[row("a", Status::Limit, Some(5000))], 9000);
        assert!(g.due(9000, false).is_empty());
        assert!(!g.hit());
    }

    #[test]
    fn sessions_that_never_ran_and_stale_limits_are_left_alone() {
        let mut g = Guard::default();
        g.observe(&[row("a", Status::Done, None)], 1000);
        g.observe(&[row("a", Status::Done, None), row("new", Status::Done, None)], 2000);
        g.observe(&[row("old", Status::Limit, Some(100)), row("a", Status::Done, None)], 100_000);
        assert!(g.waiting.is_empty());
        assert!(g.due(1_000_000, false).is_empty());
    }

    #[test]
    fn a_limit_seen_after_a_daemon_restart_still_counts_when_the_reset_is_near() {
        let mut g = Guard::default();
        g.observe(&[row("x", Status::Done, None)], 1000);
        g.observe(&[row("x", Status::Done, None), row("a", Status::Limit, Some(9000))], 2000);
        assert_eq!(g.waiting.len(), 1);
    }

    #[test]
    fn manual_resume_drops_the_wait_and_state_survives_a_restart() {
        let mut g = Guard::default();
        g.observe(&[row("a", Status::Running, None)], 1000);
        g.observe(&[row("a", Status::Limit, Some(5000))], 2000);
        let dir = std::env::temp_dir().join(format!("wisply-agents-guard-{}", std::process::id()));
        g.save(&dir);
        let mut back = Guard::load(&dir);
        assert_eq!(back.waiting, g.waiting);
        assert_eq!(back.reset_at, 5000);
        back.observe(&[row("a", Status::Running, None)], 3000);
        assert!(back.waiting.is_empty() && back.reset_at == 0);
        let _ = fs::remove_dir_all(&dir);
    }
}
