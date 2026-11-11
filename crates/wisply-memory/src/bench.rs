use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;
use std::time::Instant;

use serde_json::{Value, json};

use super::store::{Hit, Store};

pub const BENCH_LIMIT: usize = 6;
pub const BENCH_TOP: usize = 3;
pub const SMALL_CLASS: usize = 30;
pub const NO_ANSWER: &str = "none";
pub const CROSS: &str = "cross";
pub const CLASSES: &[&str] = &["fact", "decision", "rule", "stale", "time", NO_ANSWER];
const GAIN_RATIO: usize = 2;
const GAIN_WINS: usize = 10;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Question {
    pub id: String,
    pub class: String,
    pub cross: bool,
    pub hold: bool,
    pub origin: String,
    pub project: String,
    pub query: String,
    pub good: Vec<i64>,
    pub bad: Vec<i64>,
    pub old: Vec<i64>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Row {
    pub id: String,
    pub class: String,
    pub cross: bool,
    pub rank: Option<usize>,
    pub stale: bool,
    pub top: Vec<i64>,
    pub open: Vec<i64>,
    pub chars: usize,
    pub hints: usize,
    pub wrong_hints: usize,
    pub ms: f64,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Sums {
    pub asked: usize,
    pub n: usize,
    pub hit1: usize,
    pub hit3: usize,
    pub mrr: f64,
    pub stale: usize,
    pub open: usize,
    pub hints: usize,
    pub wrong_hints: usize,
    pub hinted_without_answer: usize,
    pub without_answer: usize,
    pub chars: usize,
    pub p50: f64,
    pub p95: f64,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Gain {
    pub pairs: usize,
    pub wins: usize,
    pub losses: usize,
    pub counts: bool,
}

fn word(v: &Value, key: &str) -> String {
    v.get(key).and_then(Value::as_str).unwrap_or("").trim().to_string()
}

fn flag(v: &Value, key: &str) -> bool {
    v.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn ids(v: &Value, key: &str) -> Vec<i64> {
    v.get(key).and_then(Value::as_array).map(|a| a.iter().filter_map(Value::as_i64).collect()).unwrap_or_default()
}

fn count(v: &Value, key: &str) -> usize {
    v.get(key).and_then(Value::as_u64).and_then(|n| usize::try_from(n).ok()).unwrap_or(0)
}

pub fn questions(text: &str) -> Result<Vec<Question>, String> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
        let at = n + 1;
        let v: Value = serde_json::from_str(line).map_err(|e| format!("line {at}: {e}"))?;
        let q = Question {
            id: word(&v, "id"),
            class: word(&v, "class"),
            cross: flag(&v, CROSS),
            hold: word(&v, "split") == "hold",
            origin: word(&v, "origin"),
            project: word(&v, "project"),
            query: word(&v, "query"),
            good: ids(&v, "good"),
            bad: ids(&v, "bad"),
            old: ids(&v, "old"),
        };
        if q.id.is_empty() || q.query.is_empty() || !CLASSES.contains(&q.class.as_str()) {
            return Err(format!("line {at}: needs id, query and class ({})", CLASSES.join("|")));
        }
        if !seen.insert(q.id.clone()) {
            return Err(format!("line {at}: id {} is used twice", q.id));
        }
        out.push(q);
    }
    Ok(out)
}

impl Question {
    pub fn labelled(&self) -> bool {
        self.class == NO_ANSWER || !self.good.is_empty()
    }

    fn judged(&self, id: i64) -> bool {
        self.good.contains(&id) || self.bad.contains(&id) || self.old.contains(&id)
    }
}

pub fn judge(q: &Question, top: &[i64]) -> Row {
    let near = || top.iter().take(BENCH_TOP);
    Row {
        id: q.id.clone(),
        class: q.class.clone(),
        cross: q.cross,
        rank: top.iter().position(|id| q.good.contains(id)).map(|p| p + 1),
        stale: near().any(|id| q.old.contains(id)),
        top: top.to_vec(),
        open: near().filter(|id| !q.judged(**id)).copied().collect(),
        ..Row::default()
    }
}

impl Row {
    fn judged_again(&self, q: &Question) -> Row {
        let (chars, hints, wrong_hints, ms) = (self.chars, self.hints, self.wrong_hints, self.ms);
        Row { chars, hints, wrong_hints, ms, ..judge(q, &self.top) }
    }

    pub fn hint(&mut self, q: &Question, hinted: &[i64]) {
        self.hints = hinted.len();
        self.wrong_hints = hinted.iter().filter(|id| !q.good.contains(id)).count();
        for id in hinted {
            if !q.judged(*id) && !self.open.contains(id) {
                self.open.push(*id);
            }
        }
    }
}

impl Sums {
    pub fn json(&self) -> Value {
        json!({
            "asked": self.asked, "n": self.n, "hit1": self.hit1, "hit3": self.hit3, "mrr": self.mrr,
            "stale": self.stale, "open": self.open, "hints": self.hints, "wrong_hints": self.wrong_hints,
            "hinted_without_answer": self.hinted_without_answer, "without_answer": self.without_answer,
            "chars": self.chars, "p50": self.p50, "p95": self.p95,
        })
    }
}

fn percentile(sorted: &[f64], share: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let at = (share * sorted.len() as f64).ceil() as usize;
    sorted[at.clamp(1, sorted.len()) - 1]
}

pub fn sums<'a>(rows: impl IntoIterator<Item = &'a Row>) -> Sums {
    let mut s = Sums::default();
    let mut ms = Vec::new();
    let mut chars = 0;
    for r in rows {
        s.asked += 1;
        s.hints += r.hints;
        s.wrong_hints += r.wrong_hints;
        chars += r.chars;
        ms.push(r.ms);
        if r.class == NO_ANSWER {
            s.without_answer += 1;
            s.hinted_without_answer += usize::from(r.hints > 0);
            continue;
        }
        s.n += 1;
        s.hit1 += usize::from(r.rank == Some(1));
        s.hit3 += usize::from(r.rank.is_some_and(|k| k <= BENCH_TOP));
        s.mrr += r.rank.map_or(0.0, |k| 1.0 / k as f64);
        s.stale += usize::from(r.stale);
        s.open += r.open.len();
    }
    if s.n > 0 {
        s.mrr /= s.n as f64;
    }
    s.chars = chars.checked_div(s.asked).unwrap_or(0);
    ms.sort_by(f64::total_cmp);
    s.p50 = percentile(&ms, 0.5);
    s.p95 = percentile(&ms, 0.95);
    s
}

pub fn by_class(rows: &[Row]) -> BTreeMap<String, Sums> {
    let mut out = BTreeMap::new();
    for class in CLASSES.iter().filter(|c| **c != NO_ANSWER) {
        let part: Vec<&Row> = rows.iter().filter(|r| r.class == *class).collect();
        if !part.is_empty() {
            out.insert((*class).to_string(), sums(part));
        }
    }
    let cross: Vec<&Row> = rows.iter().filter(|r| r.cross && r.class != NO_ANSWER).collect();
    if !cross.is_empty() {
        out.insert(CROSS.to_string(), sums(cross));
    }
    out
}

pub fn gain(old: &[Row], new: &[Row]) -> Gain {
    let near = |r: &Row| r.rank.is_some_and(|k| k <= BENCH_TOP);
    let before: HashMap<&str, bool> =
        old.iter().filter(|r| r.class != NO_ANSWER).map(|r| (r.id.as_str(), near(r))).collect();
    let mut g = Gain::default();
    for r in new.iter().filter(|r| r.class != NO_ANSWER) {
        let Some(was) = before.get(r.id.as_str()) else { continue };
        g.pairs += 1;
        g.wins += usize::from(near(r) && !was);
        g.losses += usize::from(!near(r) && *was);
    }
    g.counts = g.wins >= GAIN_WINS && g.wins >= GAIN_RATIO * g.losses;
    g
}

pub fn paired(old: &[Row], new: &[Row], asked: &[Question]) -> (Vec<Row>, Vec<Row>, [usize; 2]) {
    let today: HashMap<&str, &Question> = asked.iter().filter(|q| q.labelled()).map(|q| (q.id.as_str(), q)).collect();
    let ids = |rows: &[Row]| -> HashSet<String> { rows.iter().map(|r| r.id.clone()).collect() };
    let (before, after) = (ids(old), ids(new));
    let both = |rows: &[Row], there: &HashSet<String>| -> Vec<Row> {
        rows.iter()
            .filter(|r| there.contains(&r.id))
            .filter_map(|r| today.get(r.id.as_str()).map(|q| r.judged_again(q)))
            .collect()
    };
    let (a, b) = (both(old, &after), both(new, &before));
    let shared = before.intersection(&after).count();
    let left = [before.len() + after.len() - 2 * shared, shared.saturating_sub(b.len())];
    (a, b, left)
}

pub fn rows_json(rows: &[Row]) -> Value {
    Value::Array(
        rows.iter()
            .map(|r| {
                json!({
                    "id": r.id, "class": r.class, CROSS: r.cross, "rank": r.rank, "stale": r.stale,
                    "top": r.top, "open": r.open, "chars": r.chars, "hints": r.hints,
                    "wrong_hints": r.wrong_hints, "ms": r.ms,
                })
            })
            .collect(),
    )
}

pub fn rows_of(v: &Value) -> Vec<Row> {
    let Some(all) = v.as_array() else { return Vec::new() };
    all.iter()
        .map(|r| Row {
            id: word(r, "id"),
            class: word(r, "class"),
            cross: flag(r, CROSS),
            rank: Some(count(r, "rank")).filter(|k| *k > 0),
            stale: flag(r, "stale"),
            top: ids(r, "top"),
            open: ids(r, "open"),
            chars: count(r, "chars"),
            hints: count(r, "hints"),
            wrong_hints: count(r, "wrong_hints"),
            ms: r.get("ms").and_then(Value::as_f64).unwrap_or(0.0),
        })
        .collect()
}

impl Store {
    pub fn copy_to(&self, to: &Path) -> rusqlite::Result<()> {
        self.db.execute("vacuum into ?1", [to.to_string_lossy().as_ref()]).map(|_| ())
    }

    pub fn set_clock(&mut self, at: f64) {
        self.clock = Some(at);
    }

    pub fn ask(&mut self, q: &Question, vec: Option<&[f32]>) -> rusqlite::Result<(Vec<Hit>, f64)> {
        let t = Instant::now();
        let hits = self.search(&q.query, &q.project, BENCH_LIMIT, vec, 0.0)?;
        Ok((hits, t.elapsed().as_secs_f64() * 1000.0))
    }
}

#[cfg(test)]
mod tests;
