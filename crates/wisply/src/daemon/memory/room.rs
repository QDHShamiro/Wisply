use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};
use wisply_core::{Fail, local_now, local_parts, stamp_secs};

pub const DEFAULT_SPAN: &str = "7";
const USAGE: &str = "wisply board headroom [today|<days>|all]";
const LOGS: [&str; 2] = ["log.1.jsonl", "log.jsonl"];
const SAVING: [&str; 3] = ["cut", "stub", "junk"];
const DAY_S: i64 = 86_400;
const DAYS_MAX: i64 = 366;
const TOP: usize = 12;
const TOKEN_CHARS: u64 = 4;
const STAMP_HEAD: &str = "{\"t\":";

#[derive(Default)]
struct Bucket {
    n: u64,
    chars_in: u64,
    chars_out: u64,
}

impl Bucket {
    fn add(&mut self, a: u64, b: u64) {
        self.n += 1;
        self.chars_in += a;
        self.chars_out += b;
    }

    fn saved(&self) -> u64 {
        self.chars_in.saturating_sub(self.chars_out)
    }

    fn json(&self, name: &str) -> Value {
        json!({"name": name, "n": self.n, "in": self.chars_in, "out": self.chars_out, "saved": self.saved(),
               "tokens": self.saved() / TOKEN_CHARS})
    }
}

#[derive(Default)]
struct Sum {
    rows: u64,
    ms: u64,
    total: Bucket,
    acts: BTreeMap<String, Bucket>,
    tools: BTreeMap<String, Bucket>,
    days: BTreeMap<i64, Bucket>,
    top: Vec<(u64, Value)>,
    last_error: String,
}

impl Sum {
    fn add(&mut self, row: &Value, t: u64, off: i64) {
        let act = row["act"].as_str().unwrap_or("");
        let a = row["in"].as_u64().unwrap_or(0);
        let b = row["out"].as_u64().unwrap_or(a);
        self.rows += 1;
        self.ms += row["ms"].as_u64().unwrap_or(0);
        self.acts.entry(act.to_string()).or_default().add(a, b);
        if act == "error" {
            self.last_error = row["why"].as_str().unwrap_or("").to_string();
        }
        if !SAVING.contains(&act) {
            return;
        }
        let tool = row["tool"].as_str().unwrap_or("");
        self.total.add(a, b);
        self.tools.entry(tool.to_string()).or_default().add(a, b);
        self.days.entry((t as i64 / 1000 + off).div_euclid(DAY_S)).or_default().add(a, b);
        let saved = a.saturating_sub(b);
        if self.top.len() < TOP || self.top.last().is_some_and(|(s, _)| saved > *s) {
            let item = json!({"ref": row["ref"], "tool": tool, "act": act, "kind": row["kind"], "in": a, "out": b,
                              "saved": saved, "t": t});
            let at = self.top.partition_point(|(s, _)| *s >= saved);
            self.top.insert(at, (saved, item));
            self.top.truncate(TOP);
        }
    }
}

fn now_s() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64)
}

fn offset(now: i64) -> i64 {
    stamp_secs(&local_now()).map_or(0, |local| local - now)
}

fn since_ms(span: &str, now: i64, off: i64) -> Result<u64, Fail> {
    let start = match span {
        "all" => 0,
        "today" => (now + off).div_euclid(DAY_S) * DAY_S - off,
        n => {
            let days: i64 = n
                .parse()
                .ok()
                .filter(|d| (1..=DAYS_MAX).contains(d))
                .ok_or_else(|| Fail::hint(format!("'{n}' is not a span of days"), USAGE))?;
            now - days * DAY_S
        }
    };
    Ok(start.max(0) as u64 * 1000)
}

fn stamp_of(line: &str) -> Option<u64> {
    let rest = line.strip_prefix(STAMP_HEAD)?;
    let end = rest.find(|c: char| !c.is_ascii_digit())?;
    rest[..end].parse().ok()
}

fn day_name(day: i64, off: i64) -> String {
    let [y, mo, d, ..] = local_parts((day * DAY_S - off + DAY_S / 2) as f64);
    format!("{y:04}-{mo:02}-{d:02}")
}

fn read(dir: &Path, since: u64, off: i64) -> Sum {
    let mut sum = Sum::default();
    for name in LOGS {
        let Ok(raw) = fs::read(dir.join(name)) else { continue };
        for line in String::from_utf8_lossy(&raw).lines() {
            let Some(t) = stamp_of(line).filter(|t| *t >= since) else { continue };
            if let Ok(row) = serde_json::from_str::<Value>(line) {
                sum.add(&row, t, off);
            }
        }
    }
    sum
}

fn view(sum: &Sum, span: &str, since: u64, off: i64) -> Value {
    let mut acts: Vec<(&String, &Bucket)> = sum.acts.iter().collect();
    acts.sort_by(|a, b| b.1.n.cmp(&a.1.n).then_with(|| a.0.cmp(b.0)));
    let mut tools: Vec<(&String, &Bucket)> = sum.tools.iter().collect();
    tools.sort_by(|a, b| b.1.saved().cmp(&a.1.saved()).then_with(|| a.0.cmp(b.0)));
    let count = |act: &str| sum.acts.get(act).map_or(0, |b| b.n);
    let cuts = count("cut") + count("stub");
    json!({
        "span": span, "since": since, "rows": sum.rows, "squeezed": sum.total.n,
        "in": sum.total.chars_in, "out": sum.total.chars_out, "saved": sum.total.saved(),
        "tokens": sum.total.saved() / TOKEN_CHARS, "avg_ms": sum.ms.checked_div(sum.rows).unwrap_or(0),
        "reloads": count("reload"), "holdouts": count("holdout"), "errors": count("error"),
        "reload_pct": (count("reload") * 100).checked_div(cuts).unwrap_or(0), "last_error": sum.last_error,
        "acts": acts.iter().map(|(n, b)| b.json(n)).collect::<Vec<_>>(),
        "tools": tools.iter().map(|(n, b)| b.json(n)).collect::<Vec<_>>(),
        "days": sum.days.iter().map(|(d, b)| b.json(&day_name(*d, off))).collect::<Vec<_>>(),
        "top": sum.top.iter().map(|(_, v)| v.clone()).collect::<Vec<_>>(),
    })
}

pub fn board(home: &Path, span: &str) -> Result<Value, Fail> {
    let now = now_s();
    let off = offset(now);
    let since = since_ms(span, now, off)?;
    Ok(view(&read(&home.join("headroom"), since, off), span, since, off))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_log_adds_up_to_saved_tokens_per_act_tool_and_day() {
        let dir = std::env::temp_dir().join(format!("lt-room-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("headroom")).unwrap();
        let rows = [
            r#"{"t":1000,"sid":"a","tool":"Bash","in":4000,"out":400,"kind":"log","ref":"hr-1","act":"cut","ms":4}"#,
            r#"{"t":2000,"sid":"a","tool":"Read","in":800,"out":40,"kind":"read","ref":"r1","act":"stub","ms":2}"#,
            r#"{"t":3000,"sid":"a","tool":"Bash","in":900,"act":"small","ms":1}"#,
            r#"{"t":4000,"sid":"a","tool":"Read","act":"reload","ref":"hr-1","ms":1}"#,
            r#"{"t":5000,"act":"error","why":"disk full"}"#,
            "garbage",
        ];
        fs::write(dir.join("headroom/log.1.jsonl"), rows[..2].join("\n")).unwrap();
        fs::write(dir.join("headroom/log.jsonl"), rows[2..].join("\n")).unwrap();
        let v = view(&read(&dir.join("headroom"), 0, 0), "all", 0, 0);
        assert_eq!((v["rows"].as_u64(), v["squeezed"].as_u64(), v["saved"].as_u64()), (Some(5), Some(2), Some(4360)));
        assert_eq!(
            (v["tokens"].as_u64(), v["reloads"].as_u64(), v["reload_pct"].as_u64()),
            (Some(1090), Some(1), Some(50))
        );
        assert_eq!((v["errors"].as_u64(), v["last_error"].as_str()), (Some(1), Some("disk full")));
        assert_eq!((v["tools"][0]["name"].as_str(), v["tools"][0]["saved"].as_u64()), (Some("Bash"), Some(3600)));
        assert_eq!((v["top"][0]["ref"].as_str(), v["top"][1]["ref"].as_str()), (Some("hr-1"), Some("r1")));
        assert_eq!((v["days"].as_array().map(Vec::len), v["days"][0]["name"].as_str()), (Some(1), Some("1970-01-01")));
        let later = view(&read(&dir.join("headroom"), 2500, 0), "x", 2500, 0);
        assert_eq!((later["rows"].as_u64(), later["squeezed"].as_u64()), (Some(3), Some(0)));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn spans_turn_into_a_start_time() {
        assert_eq!(since_ms("all", 10 * DAY_S, 0).unwrap(), 0);
        assert_eq!(since_ms("2", 10 * DAY_S, 0).unwrap(), 8 * DAY_S as u64 * 1000);
        assert_eq!(since_ms("today", 10 * DAY_S + 500, 3600).unwrap(), (10 * DAY_S - 3600) as u64 * 1000);
        assert!(since_ms("0", 10, 0).is_err() && since_ms("week", 10, 0).is_err());
        assert_eq!(stamp_of(r#"{"t":1793421188514,"sid":"x"}"#), Some(1_793_421_188_514));
        assert_eq!(stamp_of(r#"{"sid":"x"}"#), None);
    }
}
