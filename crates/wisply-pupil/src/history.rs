use std::fs;
use std::path::Path;

use serde_json::{Value, json};

pub const RUN_PREFIX: &str = "run-";

fn row(outcome: &Value) -> Value {
    let window = outcome.get("window").and_then(Value::as_array);
    let at = |i: usize| window.and_then(|w| w.get(i)).and_then(Value::as_f64).unwrap_or(0.0);
    let reply = outcome.get("reply").filter(|r| !r.is_null());
    json!({
        "case": outcome.pointer("/case/name"),
        "model": outcome.get("model"),
        "score": outcome.pointer("/checks/score"),
        "seconds": reply.and_then(|r| r.pointer("/seconds/total")),
        "minutes": (at(1) - at(0)).max(1.0) / 60.0,
        "tokens": reply.and_then(|r| r.get("answer_tokens")),
        "finish": reply.and_then(|r| r.get("finish")),
    })
}

pub fn summary(run: &str, data: &Value) -> Option<Value> {
    let outcomes = data.get("outcomes")?.as_array()?;
    if outcomes.is_empty() {
        return None;
    }
    Some(json!({
        "run": run,
        "started": data.get("started"),
        "config": data.get("config"),
        "seed": data.get("seed"),
        "planned": data.get("planned"),
        "rows": outcomes.iter().map(row).collect::<Vec<_>>(),
    }))
}

pub fn collect(dir: &Path, limit: usize) -> Vec<Value> {
    if limit == 0 {
        return Vec::new();
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut runs: Vec<(f64, Value)> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            if !name.starts_with(RUN_PREFIX) {
                return None;
            }
            let raw = fs::read_to_string(entry.path().join("results.json")).ok()?;
            let data: Value = serde_json::from_str(&raw).ok()?;
            let started = data.get("started").and_then(Value::as_f64)?;
            Some((started, summary(&name, &data)?))
        })
        .collect();
    runs.sort_by(|a, b| a.0.total_cmp(&b.0));
    let skip = runs.len().saturating_sub(limit);
    runs.into_iter().skip(skip).map(|(_, run)| run).collect()
}

#[cfg(test)]
mod tests {
    use super::{RUN_PREFIX, collect};
    use serde_json::json;
    use std::fs;

    #[test]
    fn runs_come_back_oldest_first_and_capped() {
        let dir = std::env::temp_dir().join(format!("pupil-history-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let outcome = json!({"case": {"name": "a"}, "model": "2b", "window": [0.0, 120.0],
            "checks": {"score": 90.0},
            "reply": {"seconds": {"total": 12.0}, "answer_tokens": 300, "finish": "stop"}});
        let runs = [(30.0, vec![outcome.clone()]), (10.0, vec![outcome]), (20.0, vec![])];
        for (started, outcomes) in runs {
            let run = dir.join(format!("{RUN_PREFIX}{started}"));
            fs::create_dir_all(&run).expect("dir");
            let data = json!({"started": started, "planned": 7, "outcomes": outcomes});
            fs::write(run.join("results.json"), data.to_string()).expect("write");
        }
        fs::create_dir_all(dir.join("jobs")).expect("dir");
        let all = collect(&dir, 10);
        assert_eq!(all.len(), 2);
        assert_eq!(all[0]["started"], 10.0);
        assert_eq!(all[1]["rows"][0]["minutes"], 2.0);
        assert_eq!(all[1]["rows"][0]["tokens"], 300);
        assert_eq!(collect(&dir, 1)[0]["started"], 30.0);
        assert!(collect(&dir, 0).is_empty());
        let _ = fs::remove_dir_all(&dir);
    }
}
