use std::collections::BTreeMap;

use serde_json::{Map, Value, json};

use super::{Reply, Row};
use crate::score::median;

fn share(hits: usize, total: usize) -> Value {
    if total == 0 {
        return Value::Null;
    }
    json!(hits as f64 * 100.0 / total as f64)
}

fn mean(values: &[f64]) -> Option<f64> {
    (!values.is_empty()).then(|| values.iter().sum::<f64>() / values.len() as f64)
}

pub(super) fn summary(
    rows: &[&Row],
    marks: &[Option<(bool, bool)>],
    replies: &[Reply],
    wall: f64,
) -> Map<String, Value> {
    let mut out = Map::new();
    let loose = marks.iter().filter(|m| matches!(m, Some((_, true)))).count();
    let strict = marks.iter().filter(|m| matches!(m, Some((true, _)))).count();
    out.insert("score".into(), share(loose, rows.len()));
    out.insert("score_strict".into(), share(strict, rows.len()));
    let mut groups: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for (row, mark) in rows.iter().zip(marks) {
        let entry = groups.entry(row.group.as_str()).or_default();
        entry.1 += 1;
        if matches!(mark, Some((_, true))) {
            entry.0 += 1;
        }
    }
    for (group, (hits, total)) in groups {
        out.insert(format!("score_{group}"), share(hits, total));
    }
    let over = |get: fn(&Reply) -> f64| replies.iter().map(get).collect::<Vec<f64>>();
    out.insert("seconds_per_q".into(), json!(median(&over(|r| r.seconds))));
    out.insert("seconds_mean".into(), json!(mean(&over(|r| r.seconds))));
    out.insert("thought_per_q".into(), json!(mean(&over(|r| r.thought_tokens as f64))));
    out.insert("answer_per_q".into(), json!(mean(&over(|r| r.answer_tokens as f64))));
    out.insert("decode_tps".into(), json!(median(&over(|r| r.answer_per_second))));
    let cut = replies.iter().filter(|r| r.finish == "length").count();
    out.insert("cut_answers".into(), json!(cut));
    out.insert("wall_s".into(), json!(wall));
    let errors = (!rows.is_empty()).then(|| 1.0 - replies.len() as f64 / rows.len() as f64);
    out.insert("error_rate".into(), json!(errors));
    out
}
