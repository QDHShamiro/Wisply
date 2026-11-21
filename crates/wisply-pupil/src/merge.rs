use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

use crate::clock;
use crate::config::Config;
use crate::gauge;
use crate::score::{median, spread};
use crate::write::{append, near};

fn latest(path: &Path, label: &str) -> Result<Map<String, Value>, String> {
    let raw = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    raw.lines()
        .filter_map(|l| serde_json::from_str::<Map<String, Value>>(l).ok())
        .rfind(|r| r.get("label").and_then(Value::as_str) == Some(label))
        .ok_or_else(|| format!("no record for {label} in {}", path.display()))
}

fn numbers<'a>(parts: &'a [Map<String, Value>], key: &'a str) -> impl Iterator<Item = f64> + 'a {
    parts.iter().filter_map(move |p| p.get(key).and_then(Value::as_f64))
}

fn total(parts: &[Map<String, Value>], key: &str) -> Value {
    let found: Vec<f64> = numbers(parts, key).collect();
    if found.is_empty() { Value::Null } else { json!(found.iter().sum::<f64>()) }
}

fn combine(label: &str, parts: &[Map<String, Value>]) -> Result<Map<String, Value>, String> {
    let first = parts.first().ok_or("--parts needs at least one label")?;
    for key in ["eval_hash", "commit", "config"] {
        if parts.iter().any(|p| p.get(key) != first.get(key)) {
            return Err(format!("parts differ in {key}"));
        }
    }
    if parts.iter().any(|p| p.get("runs") != Some(&json!(1))) {
        return Err("every part must be a single run (--runs 1)".to_string());
    }
    let mut record = Map::new();
    record.insert("label".into(), json!(label));
    record.insert("time".into(), json!(clock::local().text()));
    for key in ["commit", "eval_hash"] {
        record.insert(key.into(), first.get(key).cloned().unwrap_or(Value::Null));
    }
    record.insert("runs".into(), json!(parts.len()));
    let names: Vec<Value> = parts.iter().map(|p| p["label"].clone()).collect();
    record.insert("parts".into(), Value::Array(names));
    for key in ["config", "exe"] {
        record.insert(key.into(), first.get(key).cloned().unwrap_or(Value::Null));
    }
    let starts: Vec<f64> = numbers(parts, "start_s").collect();
    record.insert("start_s".into(), json!(median(&starts)));
    for (key, value) in first {
        if value.get("median").is_some() {
            let values: Vec<Value> = parts
                .iter()
                .map(|p| p.get(key).and_then(|v| v.get("median")).cloned())
                .map(Option::unwrap_or_default)
                .collect();
            record.insert(key.clone(), spread(&values));
        }
    }
    let answers: Vec<Value> =
        parts.iter().filter_map(|p| p.get("answers").and_then(Value::as_array)).flatten().cloned().collect();
    record.insert("answers".into(), Value::Array(answers));
    record.insert("paused_s".into(), total(parts, "paused_s"));
    for key in ["vram_peak_mib", "ram_peak_mib"] {
        let found: Vec<f64> = numbers(parts, key).collect();
        record.insert(key.into(), json!(gauge::peak(&found)));
    }
    record.insert("energy_wh".into(), total(parts, "energy_wh"));
    Ok(record)
}

pub fn join(args: &[String]) -> Result<Value, String> {
    let (mut label, mut parts, mut tests) = (String::new(), Vec::new(), PathBuf::from("pupil.toml"));
    let mut rest = args.iter();
    while let Some(flag) = rest.next() {
        let mut value = || rest.next().cloned().ok_or_else(|| format!("{flag} needs a value"));
        match flag.as_str() {
            "--merge" => label = value()?,
            "--parts" => {
                parts = value()?.split(',').map(str::trim).filter(|p| !p.is_empty()).map(String::from).collect()
            }
            "--tests" => tests = PathBuf::from(value()?),
            other => return Err(format!("unknown option {other}")),
        }
    }
    if label.trim().is_empty() || parts.is_empty() {
        return Err("--merge needs a label and --parts a,b,...".to_string());
    }
    let spec = Config::load(&tests)?.write_eval;
    let path = near(&tests, &spec.results);
    let found = parts.iter().map(|p| latest(&path, p)).collect::<Result<Vec<_>, _>>()?;
    let record = Value::Object(combine(&label, &found)?);
    append(&path, &record)?;
    Ok(record)
}

#[cfg(test)]
mod tests {
    use super::combine;
    use serde_json::{Map, Value, json};

    fn part(label: &str, quality: f64, energy: f64, vram: f64) -> Map<String, Value> {
        let one = |v: f64| json!({"median": v, "min": v, "max": v, "rel": 0.0});
        let found = json!({
            "label": label, "commit": "abc", "eval_hash": "h", "runs": 1, "config": "c",
            "exe": "e", "start_s": 4.0, "rule_strict": one(quality), "decode_tps": one(50.0),
            "answers": [format!("{label}.json")], "paused_s": 0.0, "vram_peak_mib": vram,
            "ram_peak_mib": 2900.0, "energy_wh": energy
        });
        found.as_object().cloned().expect("object")
    }

    #[test]
    fn single_runs_merge_into_one_record() {
        let parts = [part("a", 90.0, 2.0, 4500.0), part("b", 94.0, 3.0, 4600.0), part("c", 92.0, 1.0, 4400.0)];
        let found = combine("base", &parts).expect("merges");
        assert_eq!(found["runs"], json!(3));
        assert_eq!(found["rule_strict"], json!({"median": 92.0, "min": 90.0, "max": 94.0, "rel": 4.0 / 92.0}));
        assert_eq!(found["energy_wh"], json!(6.0));
        assert_eq!(found["vram_peak_mib"], json!(4600.0));
        assert_eq!(found["answers"], json!(["a.json", "b.json", "c.json"]));
        assert_eq!(found["parts"], json!(["a", "b", "c"]));
    }

    #[test]
    fn parts_from_other_tests_or_many_runs_are_refused() {
        let mut other = part("b", 94.0, 3.0, 4600.0);
        other.insert("eval_hash".into(), json!("x"));
        assert!(combine("base", &[part("a", 90.0, 2.0, 4500.0), other]).is_err());
        let mut many = part("b", 94.0, 3.0, 4600.0);
        many.insert("runs".into(), json!(5));
        assert!(combine("base", &[many]).is_err());
        assert!(combine("base", &[]).is_err());
    }
}
