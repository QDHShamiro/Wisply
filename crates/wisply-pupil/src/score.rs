use serde_json::{Map, Value, json};

use crate::ask::Said;

pub enum Kind {
    Rule { german: bool },
    Math,
}

pub struct Mark {
    pub kind: Kind,
    pub strict: Option<Vec<bool>>,
    pub loose: Option<Vec<bool>>,
}

fn share(flags: impl Iterator<Item = bool>) -> Value {
    let (mut yes, mut all) = (0usize, 0usize);
    for flag in flags {
        yes += usize::from(flag);
        all += 1;
    }
    if all == 0 {
        return Value::Null;
    }
    json!(100.0 * yes as f64 / all as f64)
}

fn all_true(flags: &Option<Vec<bool>>) -> bool {
    flags.as_ref().is_some_and(|f| f.iter().all(|x| *x))
}

pub fn scores(marks: &[Mark]) -> Map<String, Value> {
    let rules: Vec<&Mark> = marks.iter().filter(|m| matches!(m.kind, Kind::Rule { .. })).collect();
    let maths: Vec<&Mark> = marks.iter().filter(|m| matches!(m.kind, Kind::Math)).collect();
    let done: Vec<&&Mark> = rules.iter().filter(|m| m.strict.is_some()).collect();
    let flat = |pick: fn(&Mark) -> &Option<Vec<bool>>| {
        done.iter().flat_map(move |m| pick(m).clone().unwrap_or_default()).collect::<Vec<bool>>()
    };
    let mut out = Map::new();
    out.insert("rule_strict".into(), share(rules.iter().map(|m| all_true(&m.strict))));
    out.insert("rule_loose".into(), share(rules.iter().map(|m| all_true(&m.loose))));
    out.insert("inst_strict".into(), share(flat(|m| &m.strict).into_iter()));
    out.insert("inst_loose".into(), share(flat(|m| &m.loose).into_iter()));
    let german = done.iter().filter(|m| matches!(m.kind, Kind::Rule { german: true })).map(|m| all_true(&m.strict));
    out.insert("rule_de_strict".into(), share(german));
    out.insert("math_strict".into(), share(maths.iter().map(|m| all_true(&m.strict))));
    out.insert("math_loose".into(), share(maths.iter().map(|m| all_true(&m.loose))));
    out
}

pub fn median(values: &[f64]) -> Option<f64> {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len();
    match n {
        0 => None,
        _ if n % 2 == 1 => Some(sorted[n / 2]),
        _ => Some((sorted[n / 2 - 1] + sorted[n / 2]) / 2.0),
    }
}

fn first_token(said: &Said) -> f64 {
    let writing = if said.answer_per_second > 0.0 { said.answer_tokens as f64 / said.answer_per_second } else { 0.0 };
    said.model_seconds - writing
}

pub fn tempo(replies: &[Said], total: usize, wall: f64) -> Map<String, Value> {
    let over = |get: fn(&Said) -> f64| median(&replies.iter().map(get).collect::<Vec<_>>());
    let mut out = Map::new();
    out.insert("decode_tps".into(), json!(over(|s| s.answer_per_second)));
    out.insert("prefill_tps".into(), json!(over(|s| s.prompt_per_second)));
    out.insert("latency_s".into(), json!(over(|s| s.model_seconds)));
    out.insert("ttft_s".into(), json!(over(first_token)));
    let tokens: u64 = replies.iter().map(|s| s.answer_tokens).sum();
    out.insert("answer_tokens".into(), json!(tokens));
    let cut = replies.iter().filter(|s| s.finish == "length").count();
    out.insert("cut_answers".into(), json!(cut));
    out.insert("wall_s".into(), json!(wall));
    let errors = (total > 0).then(|| 1.0 - replies.len() as f64 / total as f64);
    out.insert("error_rate".into(), json!(errors));
    out
}

pub fn spread(values: &[Value]) -> Value {
    let xs: Vec<f64> = values.iter().filter_map(Value::as_f64).collect();
    let Some(mid) = median(&xs) else {
        return json!({"median": null, "min": null, "max": null, "rel": null});
    };
    let low = xs.iter().copied().fold(f64::INFINITY, f64::min);
    let high = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let rel = (mid != 0.0).then(|| (high - low) / mid);
    json!({"median": mid, "min": low, "max": high, "rel": rel})
}

pub fn merge(per_run: &[Map<String, Value>], into: &mut Map<String, Value>) {
    let Some(first) = per_run.first() else {
        return;
    };
    for key in first.keys() {
        let values: Vec<Value> = per_run.iter().map(|r| r.get(key).cloned().unwrap_or(Value::Null)).collect();
        into.insert(key.clone(), spread(&values));
    }
}

#[cfg(test)]
mod tests {
    use super::{Kind, Mark, median, scores, spread};
    use serde_json::json;

    fn mark(kind: Kind, strict: &[bool], loose: &[bool]) -> Mark {
        Mark { kind, strict: Some(strict.to_vec()), loose: Some(loose.to_vec()) }
    }

    #[test]
    fn scores_count_cases_rules_german_and_math() {
        let marks = vec![
            mark(Kind::Rule { german: false }, &[true, true], &[true, true]),
            mark(Kind::Rule { german: true }, &[true, false], &[true, true]),
            Mark { kind: Kind::Rule { german: true }, strict: None, loose: None },
            mark(Kind::Math, &[true], &[true]),
            mark(Kind::Math, &[false], &[true]),
        ];
        let found = scores(&marks);
        let third = 100.0 / 3.0;
        assert_eq!(found["rule_strict"], json!(third));
        assert_eq!(found["rule_loose"], json!(2.0 * third));
        assert_eq!(found["inst_strict"], json!(75.0));
        assert_eq!(found["rule_de_strict"], json!(0.0));
        assert_eq!(found["math_strict"], json!(50.0));
        assert_eq!(found["math_loose"], json!(100.0));
    }

    #[test]
    fn spread_matches_the_python_harness() {
        assert_eq!(median(&[3.0, 1.0, 2.0, 10.0]), Some(2.5));
        let found = spread(&[json!(2.0), json!(4.0), json!(null), json!(3.0)]);
        assert_eq!(found, json!({"median": 3.0, "min": 2.0, "max": 4.0, "rel": 2.0 / 3.0}));
        assert_eq!(spread(&[json!(null)])["median"], json!(null));
    }
}
