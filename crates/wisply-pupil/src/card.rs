use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use serde_json::{Value, json};

use crate::clock;
use crate::config::Config;
use crate::goals::{self, Goals};
use crate::page;

#[derive(Deserialize)]
struct Card {
    out: PathBuf,
    variable: String,
    #[serde(default)]
    reference: Option<PathBuf>,
    runs: PathBuf,
    min_categories: usize,
    estimated_ci: f64,
    z: f64,
    sources: HashMap<String, PathBuf>,
    categories: Vec<Category>,
    models: Vec<Model>,
    #[serde(default)]
    published: Vec<String>,
    #[serde(default)]
    goals: Option<Goals>,
}

#[derive(Deserialize)]
struct Category {
    key: String,
    name: String,
    weight: f64,
    test: String,
    #[serde(default)]
    reference: Option<String>,
}

#[derive(Deserialize)]
struct Model {
    name: String,
    maker: String,
    note: String,
    values: HashMap<String, Pick>,
    #[serde(default)]
    speed: Vec<Speed>,
}

#[derive(Deserialize)]
struct Pick {
    from: String,
    label: String,
    #[serde(default)]
    key: Option<String>,
    #[serde(default)]
    n: Option<f64>,
}

#[derive(Deserialize)]
struct Speed {
    name: String,
    from: String,
    label: String,
    key: String,
    unit: String,
}

struct Found {
    value: f64,
    ci: Option<f64>,
    n: Option<f64>,
    label: String,
    time: String,
}

struct Total {
    score: Option<f64>,
    ci: Option<f64>,
    measured: usize,
    missing: Vec<String>,
}

pub fn run(config: &Config, config_path: &Path, args: &[String]) -> i32 {
    let path = match args.first() {
        Some(given) => PathBuf::from(given),
        None => page::resolved(config, config_path).bench.card,
    };
    match build(&path) {
        Ok(report) => {
            println!("{report}");
            0
        }
        Err(message) => {
            println!("{}", json!({"ok": false, "error": "card", "message": message, "hint": path}));
            1
        }
    }
}

fn build(path: &Path) -> Result<Value, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let card: Card = serde_json::from_str(&raw).map_err(|e| format!("{}: {e}", path.display()))?;
    let rows: HashMap<&str, Vec<Value>> =
        card.sources.iter().map(|(name, file)| (name.as_str(), records(file))).collect();
    let mut cards: Vec<Value> = card.models.iter().map(|model| measured(&card, &rows, model)).collect();
    let reference = card
        .reference
        .as_ref()
        .and_then(|file| fs::read_to_string(file).ok())
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok());
    if let Some(reference) = &reference {
        cards.extend(card.published.iter().filter_map(|name| published(&card, reference, name)));
    }
    let categories: Vec<Value> = card
        .categories
        .iter()
        .map(|c| json!({"key": c.key, "name": c.name, "weight": c.weight, "test": c.test}))
        .collect();
    let time = clock::local().text();
    let goals = card.goals.as_ref().map(|goals| goals::write(goals, &rows, &cards, &time)).transpose()?;
    let data = json!({
        "time": time,
        "estimated_ci": card.estimated_ci,
        "categories": categories,
        "cards": cards,
    });
    script(&card.out, &card.variable, &data)?;
    let scores: Vec<Value> = data["cards"]
        .as_array()
        .map(|all| all.iter().map(|c| json!({"name": c["name"], "score": c["score"], "ci": c["ci"]})).collect())
        .unwrap_or_default();
    Ok(json!({"ok": true, "out": card.out, "cards": scores, "goals": goals}))
}

pub fn script(out: &Path, variable: &str, data: &Value) -> Result<(), String> {
    let text = serde_json::to_string(data).map_err(|e| e.to_string())?;
    let script = format!("window.{variable} = {};\n", text.replace('<', "\\u003c"));
    if let Some(dir) = out.parent() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    fs::write(out, script).map_err(|e| format!("{}: {e}", out.display()))
}

fn records(path: &Path) -> Vec<Value> {
    fs::read_to_string(path)
        .map(|raw| {
            raw.lines()
                .filter_map(|line| serde_json::from_str::<Value>(line.trim()).ok())
                .filter(Value::is_object)
                .collect()
        })
        .unwrap_or_default()
}

pub fn last<'a>(rows: &'a [Value], label: &str) -> Option<&'a Value> {
    rows.iter().rev().find(|row| row["label"] == label)
}

pub fn number(value: &Value) -> Option<f64> {
    value.as_f64().or_else(|| value.get("median").and_then(Value::as_f64))
}

fn measure(card: &Card, rows: &HashMap<&str, Vec<Value>>, pick: &Pick) -> Option<Found> {
    let record = last(rows.get(pick.from.as_str())?, &pick.label)?;
    let key = pick.key.as_deref().unwrap_or(if record.get("accuracy").is_some() { "accuracy" } else { "quality" });
    let value = number(&record[key])?;
    let time = record["time"].as_str().unwrap_or_default().to_string();
    let label = pick.label.clone();
    if pick.key.is_none()
        && let Some(scores) = case_scores(&card.runs, record)
    {
        let n = scores.len() as f64;
        return Some(Found { value, ci: Some(card.z * deviation(&scores) / n.sqrt()), n: Some(n), label, time });
    }
    let n = pick.n.or_else(|| record["questions"].as_f64());
    Some(Found { value, ci: n.and_then(|n| binomial(value, n, card.z)), n, label, time })
}

fn case_scores(runs: &Path, record: &Value) -> Option<Vec<f64>> {
    let dir = record["run_dirs"].as_array()?.first()?.as_str()?;
    let raw = fs::read_to_string(runs.join(dir).join("results.json")).ok()?;
    let data: Value = serde_json::from_str(&raw).ok()?;
    let scores: Vec<f64> =
        data["outcomes"].as_array()?.iter().filter_map(|outcome| outcome["checks"]["score"].as_f64()).collect();
    (scores.len() > 1).then_some(scores)
}

fn deviation(values: &[f64]) -> f64 {
    let n = values.len() as f64;
    let mean = values.iter().sum::<f64>() / n;
    let square = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>();
    (square / (n - 1.0)).sqrt()
}

fn binomial(value: f64, n: f64, z: f64) -> Option<f64> {
    if n <= 0.0 {
        return None;
    }
    let p = (value / 100.0).clamp(0.0, 1.0);
    Some(z * (p * (1.0 - p) / n).sqrt() * 100.0)
}

fn total(card: &Card, found: &HashMap<&str, Found>) -> Total {
    let mut weights = 0.0;
    let mut sum = 0.0;
    let mut square = 0.0;
    let mut spread = true;
    let mut measured = 0;
    let mut missing = Vec::new();
    for category in card.categories.iter().filter(|c| c.weight > 0.0) {
        match found.get(category.key.as_str()) {
            Some(hit) => {
                weights += category.weight;
                sum += category.weight * hit.value;
                measured += 1;
                match hit.ci {
                    Some(ci) => square += (category.weight * ci).powi(2),
                    None => spread = false,
                }
            }
            None => missing.push(category.name.clone()),
        }
    }
    if measured < card.min_categories || weights <= 0.0 {
        return Total { score: None, ci: None, measured, missing };
    }
    Total { score: Some(sum / weights), ci: spread.then(|| square.sqrt() / weights), measured, missing }
}

fn shown(card: &Card, found: &HashMap<&str, Found>) -> Vec<Value> {
    card.categories
        .iter()
        .map(|category| match found.get(category.key.as_str()) {
            Some(hit) => json!({
                "key": category.key,
                "value": hit.value,
                "ci": hit.ci,
                "n": hit.n,
                "label": hit.label,
                "time": hit.time,
            }),
            None => json!({"key": category.key, "value": null}),
        })
        .collect()
}

fn summary(card: &Card, found: &HashMap<&str, Found>) -> Value {
    let sum = total(card, found);
    let estimated = !sum.missing.is_empty() || sum.ci.is_none_or(|ci| ci > card.estimated_ci);
    json!({
        "score": sum.score,
        "ci": sum.ci,
        "low": sum.score.zip(sum.ci).map(|(s, c)| (s - c).max(0.0)),
        "high": sum.score.zip(sum.ci).map(|(s, c)| (s + c).min(100.0)),
        "measured": sum.measured,
        "missing": sum.missing,
        "estimated": estimated,
        "values": shown(card, found),
    })
}

fn measured(card: &Card, rows: &HashMap<&str, Vec<Value>>, model: &Model) -> Value {
    let found: HashMap<&str, Found> = model
        .values
        .iter()
        .filter_map(|(key, pick)| measure(card, rows, pick).map(|hit| (key.as_str(), hit)))
        .collect();
    let speed: Vec<Value> = model
        .speed
        .iter()
        .filter_map(|s| {
            let record = last(rows.get(s.from.as_str())?, &s.label)?;
            let value = number(&record[s.key.as_str()])?;
            Some(json!({"name": s.name, "value": value, "unit": s.unit}))
        })
        .collect();
    let mut out = summary(card, &found);
    out["name"] = json!(model.name);
    out["maker"] = json!(model.maker);
    out["note"] = json!(model.note);
    out["kind"] = json!("measured");
    out["speed"] = json!(speed);
    out
}

fn published(card: &Card, reference: &Value, name: &str) -> Option<Value> {
    let model = reference["models"].as_array()?.iter().find(|m| m["name"] == name)?;
    let found: HashMap<&str, Found> = card
        .categories
        .iter()
        .filter_map(|category| {
            let key = category.reference.as_deref()?;
            let value = model["scores"][key].as_f64()?;
            Some((
                category.key.as_str(),
                Found {
                    value,
                    ci: None,
                    n: None,
                    label: key.to_string(),
                    time: reference["checked"].as_str().unwrap_or_default().to_string(),
                },
            ))
        })
        .collect();
    if found.is_empty() {
        return None;
    }
    let mut out = summary(card, &found);
    out["name"] = json!(name);
    out["maker"] = model["maker"].clone();
    out["note"] = model["where"].clone();
    out["kind"] = json!("published");
    out["sure"] = model["sure"].clone();
    out["source"] = model["source"].clone();
    out["speed"] = json!([]);
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card(min: usize) -> Card {
        serde_json::from_value(json!({
            "out": "x.js",
            "variable": "V",
            "runs": ".",
            "min_categories": min,
            "estimated_ci": 5.0,
            "z": 1.96,
            "sources": {},
            "categories": [
                {"key": "a", "name": "A", "weight": 2.0, "test": "t"},
                {"key": "b", "name": "B", "weight": 1.0, "test": "t"},
                {"key": "c", "name": "C", "weight": 0.0, "test": "t"}
            ],
            "models": []
        }))
        .unwrap()
    }

    fn hit(value: f64, ci: Option<f64>) -> Found {
        Found { value, ci, n: None, label: String::new(), time: String::new() }
    }

    #[test]
    fn weights_count_and_extra_rows_stay_out() {
        let found =
            HashMap::from([("a", hit(80.0, Some(4.0))), ("b", hit(50.0, Some(3.0))), ("c", hit(0.0, Some(1.0)))]);
        let sum = total(&card(2), &found);
        assert!((sum.score.unwrap() - 70.0).abs() < 1e-9);
        assert!((sum.ci.unwrap() - (64.0f64 + 9.0).sqrt() / 3.0).abs() < 1e-9);
        assert!(sum.missing.is_empty());
    }

    #[test]
    fn too_few_categories_give_no_score() {
        let found = HashMap::from([("a", hit(80.0, Some(4.0)))]);
        let sum = total(&card(2), &found);
        assert!(sum.score.is_none());
        assert_eq!(sum.missing, vec!["B".to_string()]);
    }

    #[test]
    fn binomial_matches_known_value() {
        let ci = binomial(72.7, 150.0, 1.96).unwrap();
        assert!((ci - 7.13).abs() < 0.01);
        assert!(binomial(50.0, 0.0, 1.96).is_none());
    }

    #[test]
    fn deviation_uses_sample_formula() {
        assert!((deviation(&[1.0, 3.0]) - 2.0f64.sqrt()).abs() < 1e-9);
    }
}
