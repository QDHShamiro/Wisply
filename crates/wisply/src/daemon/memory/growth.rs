use std::collections::HashMap;

use serde_json::{Map, Value, json};
use wisply_core::{Call, Fail, local_parts};
use wisply_memory::{DayCount, now};

use super::super::{Daemon, routines};
use super::with_store;

const DAY_S: f64 = 86_400.0;
const NOON_S: f64 = 43_200.0;
const DAYS_DEFAULT: usize = 30;
const DAYS_MAX: usize = 3650;
const WEEK: usize = 7;
const SHELVES: [(&str, &str); 4] =
    [("notes:user", "remembered"), ("notes:session", "learned"), ("notes:vault", "vault"), ("notes:claude", "claude")];
const DAILY: [&str; 10] =
    ["remembered", "learned", "vault", "claude", "lessons", "routines", "sessions", "done", "bugs", "ideas"];
const COUNTED: [&str; 9] = ["notes", "lessons", "ups", "downs", "sessions", "done", "hits", "recalled", "found"];
const POINTS: [(&str, i64); 8] = [
    ("remembered", 3),
    ("learned", 2),
    ("vault", 1),
    ("claude", 1),
    ("lessons", 4),
    ("routines", 5),
    ("sessions", 1),
    ("done", 1),
];

const KNOWLEDGE: [(&str, i64); 4] = [("remembered", 1), ("learned", 1), ("vault", 1), ("claude", 1)];

type Series = HashMap<&'static str, Vec<i64>>;

fn day_of(epoch: f64) -> String {
    let p = local_parts(epoch);
    format!("{:04}-{:02}-{:02}", p[0], p[1], p[2])
}

fn days(count: usize, at: f64) -> Vec<String> {
    let p = local_parts(at);
    let noon = at - f64::from(p[3] * 3600 + p[4] * 60 + p[5]) + NOON_S;
    let mut out: Vec<String> = (0..count).rev().map(|i| day_of(noon - i as f64 * DAY_S)).collect();
    out.dedup();
    out
}

fn span(c: &Call, first: f64, at: f64) -> usize {
    match c.pos.get(1).map(String::as_str).unwrap_or("") {
        "all" if first > 0.0 => (((at - first) / DAY_S).ceil() as usize + 1).clamp(WEEK, DAYS_MAX),
        raw => raw.parse().unwrap_or(DAYS_DEFAULT).clamp(WEEK, DAYS_MAX),
    }
}

fn key_of(raw: &str) -> Option<&'static str> {
    if let Some((_, name)) = SHELVES.iter().find(|(k, _)| *k == raw) {
        return Some(name);
    }
    match raw {
        "lessons" => Some("lessons"),
        "sessions" => Some("sessions"),
        "done" => Some("done"),
        "found:bug" => Some("bugs"),
        "found:feature" | "found:idea" => Some("ideas"),
        _ => None,
    }
}

fn number(v: &Value) -> i64 {
    v.as_i64().or_else(|| v.as_f64().map(|f| f as i64)).unwrap_or(0)
}

fn routine_days(list: &[Value]) -> HashMap<String, i64> {
    let mut out = HashMap::new();
    for r in list {
        let created = r["created"].as_str().unwrap_or("");
        if let Some(day) = created.get(..10).filter(|d| d.as_bytes().get(4) == Some(&b'-')) {
            *out.entry(day.to_string()).or_insert(0) += 1;
        }
    }
    out
}

fn climb(daily: &[i64], total: i64) -> Vec<i64> {
    let mut out = vec![0; daily.len()];
    let mut level = total;
    for (i, n) in daily.iter().enumerate().rev() {
        out[i] = level.max(0);
        level -= n;
    }
    out
}

fn weighted(series: &Series, i: usize, keys: &[(&str, i64)]) -> i64 {
    keys.iter().map(|(k, w)| series.get(*k).and_then(|v| v.get(i)).map_or(0, |n| n * w)).sum()
}

fn week(v: &[i64]) -> [i64; 2] {
    let n = v.len();
    let this = v[n.saturating_sub(WEEK)..].iter().sum();
    let prev = v[n.saturating_sub(2 * WEEK)..n.saturating_sub(WEEK)].iter().sum();
    [this, prev]
}

fn series_of(range: &[String], rows: &[DayCount], routines: &[Value]) -> Series {
    let index: HashMap<&str, usize> = range.iter().enumerate().map(|(i, d)| (d.as_str(), i)).collect();
    let mut out: Series = DAILY.iter().map(|k| (*k, vec![0; range.len()])).collect();
    let mut put = |key: &'static str, day: &str, n: i64| {
        if let (Some(&i), Some(v)) = (index.get(day), out.get_mut(key)) {
            v[i] += n;
        }
    };
    for r in rows {
        if let Some(key) = key_of(&r.key) {
            put(key, &r.day, r.n);
        }
    }
    for (day, n) in routine_days(routines) {
        put("routines", &day, n);
    }
    out
}

fn totals_of(raw: &[(String, i64)], routines: &[Value]) -> HashMap<&'static str, i64> {
    let total = |k: &str| raw.iter().find(|(key, _)| key == k).map_or(0, |(_, n)| *n);
    let stat = |k: &str| routines.iter().map(|r| number(&r["stats"][k])).sum::<i64>();
    let mut out: HashMap<&'static str, i64> = SHELVES.iter().map(|(k, name)| (*name, total(k))).collect();
    out.extend(COUNTED.iter().map(|k| (*k, total(k))));
    out.extend([
        ("routines", routines.len() as i64),
        ("routine_runs", stat("runs")),
        ("routine_ok", stat("ok")),
        ("saved", stat("saved")),
    ]);
    out
}

pub fn intelligence(d: &Daemon, c: &Call) -> Result<Value, Fail> {
    let at = now();
    let raw = with_store(d, |s| s.growth_totals())?;
    let first = raw.iter().find(|(k, _)| k == "first").map_or(0.0, |(_, n)| *n as f64);
    let range = days(span(c, first, at), at);
    let rows = with_store(d, |s| s.growth(at - (range.len() + 1) as f64 * DAY_S))?;
    let list = routines::load_all(&d.home);
    let series = series_of(&range, &rows, &list);
    let totals = totals_of(&raw, &list);
    let points: Vec<i64> = (0..range.len()).map(|i| weighted(&series, i, &POINTS)).collect();
    let learned: Vec<i64> = (0..range.len()).map(|i| weighted(&series, i, &KNOWLEDGE)).collect();
    let score_now = POINTS.iter().map(|(k, w)| totals.get(*k).copied().unwrap_or(0) * w).sum();
    let score = climb(&points, score_now);
    let mut weeks: Map<String, Value> = series.iter().map(|(k, v)| ((*k).to_string(), json!(week(v)))).collect();
    weeks.insert("score".into(), json!(week(&points)));
    weeks.insert("knowledge".into(), json!(week(&learned)));
    let (best_at, best) = points.iter().enumerate().max_by_key(|(i, n)| (**n, *i)).map_or((0, 0), |(i, n)| (i, *n));
    Ok(json!({
        "days": range, "daily": series, "score": score, "gain": points, "knowledge": climb(&learned, totals["notes"]),
        "points": POINTS, "weeks": weeks, "totals": totals, "best": best, "best_day": range.get(best_at),
        "first": first, "now": at,
    }))
}

#[cfg(test)]
mod tests {
    use super::{climb, days, routine_days, week};
    use serde_json::json;

    #[test]
    fn climb_walks_back_from_the_total() {
        assert_eq!(climb(&[1, 0, 2], 10), [8, 8, 10]);
        assert_eq!(climb(&[5, 5], 3), [0, 3]);
        assert_eq!(week(&[1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2]), [14, 7]);
    }

    #[test]
    fn days_end_today_and_routines_count_per_day() {
        let list = days(7, 1_763_726_400.0);
        assert_eq!(list.len(), 7);
        assert!(list.windows(2).all(|w| w[0] < w[1]), "{list:?}");
        let r =
            routine_days(&[json!({"created": "2026-11-20 10:00"}), json!({"created": "2026-11-20 11:00"}), json!({})]);
        assert_eq!(r.get("2026-11-20"), Some(&2));
        assert_eq!(r.len(), 1);
    }
}
