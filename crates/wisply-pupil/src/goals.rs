use std::collections::HashMap;
use std::path::PathBuf;

use serde::Deserialize;
use serde_json::{Value, json};

use crate::card::{last, number, script};

#[derive(Deserialize)]
pub struct Goals {
    out: PathBuf,
    variable: String,
    list: Vec<Goal>,
}

#[derive(Deserialize)]
struct Goal {
    key: String,
    name: String,
    unit: String,
    start: f64,
    target: f64,
    due: String,
    test: String,
    #[serde(default)]
    note: String,
    #[serde(default)]
    now: Option<f64>,
    #[serde(default)]
    score: Option<String>,
    #[serde(default)]
    from: Option<String>,
    #[serde(default)]
    label: Option<String>,
    #[serde(default)]
    value: Vec<String>,
    #[serde(default)]
    count: Option<String>,
    #[serde(default)]
    pending: bool,
}

struct Seen {
    value: f64,
    label: String,
    time: String,
}

fn fixed(value: f64, label: &str) -> Seen {
    Seen { value, label: label.to_string(), time: String::new() }
}

fn seen(goal: &Goal, rows: &HashMap<&str, Vec<Value>>, cards: &[Value]) -> Option<Seen> {
    if let Some(value) = goal.now {
        return Some(fixed(value, ""));
    }
    if let Some(name) = &goal.score {
        let card = cards.iter().find(|card| card["name"] == name.as_str())?;
        return Some(fixed(card["score"].as_f64()?, name));
    }
    let all = rows.get(goal.from.as_deref()?)?;
    if let Some(flag) = &goal.count {
        let hits = all.iter().filter(|row| row[flag.as_str()] == true).count();
        return Some(fixed(hits as f64, ""));
    }
    let label = goal.label.as_deref()?;
    let record = last(all, label)?;
    let found = goal.value.iter().fold(record, |at, key| &at[key.as_str()]);
    Some(Seen {
        value: number(found)?,
        label: label.to_string(),
        time: record["time"].as_str().unwrap_or_default().to_string(),
    })
}

fn progress(start: f64, target: f64, now: f64) -> f64 {
    let span = target - start;
    if span.abs() < f64::EPSILON {
        return 1.0;
    }
    ((now - start) / span).clamp(0.0, 1.0)
}

fn shown(goal: &Goal, seen: Option<Seen>) -> Value {
    let done = seen.as_ref().map(|seen| progress(goal.start, goal.target, seen.value));
    let hit = done.is_some_and(|done| done >= 1.0);
    json!({
        "key": goal.key,
        "name": goal.name,
        "unit": goal.unit,
        "start": goal.start,
        "target": goal.target,
        "due": goal.due,
        "test": goal.test,
        "note": goal.note,
        "now": seen.as_ref().map(|seen| seen.value),
        "label": seen.as_ref().map(|seen| seen.label.as_str()),
        "time": seen.as_ref().map(|seen| seen.time.as_str()),
        "progress": done,
        "hit": hit,
        "pending": goal.pending,
        "reached": hit && !goal.pending,
    })
}

pub fn write(goals: &Goals, rows: &HashMap<&str, Vec<Value>>, cards: &[Value], time: &str) -> Result<Value, String> {
    let list: Vec<Value> = goals.list.iter().map(|goal| shown(goal, seen(goal, rows, cards))).collect();
    let reached = list.iter().filter(|goal| goal["reached"] == true).count();
    let all = list.len();
    script(&goals.out, &goals.variable, &json!({"time": time, "goals": list}))?;
    Ok(json!({"out": goals.out, "goals": all, "reached": reached}))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::{Value, json};

    use super::{Goal, progress, seen, shown};

    fn goal(extra: Value) -> Goal {
        let mut base = json!({
            "key": "Z", "name": "n", "unit": "%", "start": 40.0, "target": 50.0,
            "due": "2026-11-20", "test": "t"
        });
        for (key, value) in extra.as_object().expect("object") {
            base[key] = value.clone();
        }
        serde_json::from_value(base).expect("goal")
    }

    #[test]
    fn progress_runs_from_start_to_target_in_both_directions() {
        assert_eq!(progress(40.0, 50.0, 45.0), 0.5);
        assert_eq!(progress(40.0, 50.0, 55.0), 1.0);
        assert_eq!(progress(40.0, 50.0, 30.0), 0.0);
        assert_eq!(progress(7.2, 2.5, 0.6), 1.0);
        assert_eq!(progress(8.0, 4.0, 6.0), 0.5);
        assert_eq!(progress(8.0, 4.0, 9.0), 0.0);
        assert_eq!(progress(1.0, 1.0, 0.0), 1.0);
    }

    #[test]
    fn the_current_value_comes_from_the_named_place() {
        let rows = HashMap::from([
            (
                "know",
                vec![
                    json!({"label": "a", "accuracy": 41.0, "time": "old"}),
                    json!({"label": "a", "accuracy": 47.5, "time": "new", "parts": {"x": {"median": 3.0}}}),
                ],
            ),
            ("notes", vec![json!({"ok": true}), json!({"ok": false}), json!({"ok": true})]),
        ]);
        let cards = vec![json!({"name": "Pupil", "score": 74.25})];
        let fixed = seen(&goal(json!({"now": 1.0})), &rows, &cards).expect("fixed");
        assert_eq!(fixed.value, 1.0);
        let scored = seen(&goal(json!({"score": "Pupil"})), &rows, &cards).expect("score");
        assert_eq!((scored.value, scored.label.as_str()), (74.25, "Pupil"));
        let counted = goal(json!({"from": "notes", "count": "ok"}));
        assert_eq!(seen(&counted, &rows, &cards).expect("count").value, 2.0);
        let picked = goal(json!({"from": "know", "label": "a", "value": ["accuracy"]}));
        let read = seen(&picked, &rows, &cards).expect("label");
        assert_eq!((read.value, read.time.as_str()), (47.5, "new"));
        let deep = goal(json!({"from": "know", "label": "a", "value": ["parts", "x"]}));
        assert_eq!(seen(&deep, &rows, &cards).expect("deep").value, 3.0);
        let row = shown(&picked, Some(read));
        assert_eq!(row["progress"], json!(0.75));
        assert_eq!(row["reached"], json!(false));
        let done = shown(&goal(json!({"now": 50.0})), Some(super::fixed(50.0, "")));
        assert_eq!((done["hit"].clone(), done["reached"].clone()), (json!(true), json!(true)));
        let open = goal(json!({"now": 50.0, "pending": true}));
        let open = shown(&open, Some(super::fixed(50.0, "")));
        assert_eq!((open["hit"].clone(), open["reached"].clone()), (json!(true), json!(false)));
        for lost in [
            json!({"score": "Other"}),
            json!({"from": "gone", "label": "a"}),
            json!({"from": "know", "label": "b", "value": ["accuracy"]}),
            json!({"from": "know", "label": "a", "value": ["nothing"]}),
            json!({}),
        ] {
            let lost = goal(lost);
            assert!(seen(&lost, &rows, &cards).is_none());
            let row = shown(&lost, None);
            assert_eq!(row["now"], Value::Null);
            assert_eq!(row["reached"], json!(false));
        }
    }
}
