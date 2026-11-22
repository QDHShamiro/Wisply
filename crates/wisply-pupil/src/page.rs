use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use serde_json::Value;

use crate::config::Config;
use crate::history;

pub fn write(config: &Config, dir: &Path, run: &Value, baseline: Option<&Value>) -> Result<PathBuf, String> {
    let data = serde_json::to_string(run).map_err(|e| e.to_string())?;
    put(config, &dir.join("results.json"), &data)?;
    let mut shown = run.clone();
    if let Some(base) = baseline {
        shown["baseline"] = base.clone();
    }
    shown["history"] = Value::from(history::collect(&config.bench.out_dir, config.bench.history_runs));
    if let Some(reference) = config
        .bench
        .reference
        .as_ref()
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|raw| serde_json::from_str::<Value>(&raw).ok())
    {
        shown["reference"] = reference;
    }
    if let Some(raw) = config.bench.measurements.as_ref().and_then(|path| fs::read_to_string(path).ok()) {
        shown["measurements"] = Value::from(measurements(&raw));
    }
    let shown = serde_json::to_string(&shown).map_err(|e| e.to_string())?;
    let template =
        fs::read_to_string(&config.bench.template).map_err(|e| format!("{}: {e}", config.bench.template.display()))?;
    let template = styled(config, template)?;
    let html = template.replace(&config.bench.data_marker, &shown.replace('<', "\\u003c"));
    let report = dir.join("report.html");
    put(config, &report, &html)?;
    if !config.bench.latest_name.is_empty() {
        let latest = config.bench.out_dir.join(&config.bench.latest_name);
        put(config, &latest, &html)?;
    }
    Ok(report)
}

fn put(config: &Config, path: &Path, text: &str) -> Result<(), String> {
    let wait = Duration::from_millis(config.bench.write_wait_ms);
    patient(config.bench.write_tries, wait, || fs::write(path, text)).map_err(|e| format!("{}: {e}", path.display()))
}

fn patient<T>(tries: u32, wait: Duration, mut run: impl FnMut() -> io::Result<T>) -> io::Result<T> {
    let mut last = run();
    for _ in 1..tries {
        if last.is_ok() {
            break;
        }
        thread::sleep(wait);
        last = run();
    }
    last
}

fn styled(config: &Config, template: String) -> Result<String, String> {
    let marker = &config.bench.style_marker;
    match config.bench.style.as_ref() {
        Some(path) if !marker.is_empty() => {
            let style = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
            Ok(template.replace(marker, style.trim_end()))
        }
        _ => Ok(template),
    }
}

fn measurements(raw: &str) -> Vec<Value> {
    raw.lines().filter_map(|line| serde_json::from_str::<Value>(line.trim()).ok()).filter(Value::is_object).collect()
}

pub fn resolved(config: &Config, config_path: &Path) -> Config {
    let mut config = config.clone();
    if let Some(base) = config_path.parent() {
        if config.bench.template.is_relative() {
            config.bench.template = base.join(&config.bench.template);
        }
        if let Some(reference) = config.bench.reference.as_mut()
            && reference.is_relative()
        {
            *reference = base.join(&*reference);
        }
        if let Some(measurements) = config.bench.measurements.as_mut()
            && measurements.is_relative()
        {
            *measurements = base.join(&*measurements);
        }
        if let Some(style) = config.bench.style.as_mut()
            && style.is_relative()
        {
            *style = base.join(&*style);
        }
        if config.bench.card.is_relative() {
            config.bench.card = base.join(&config.bench.card);
        }
    }
    config
}

#[cfg(test)]
mod tests {
    use super::{patient, resolved, write};
    use crate::config::Config;
    use serde_json::{Value, json};
    use std::fs;
    use std::io;
    use std::path::Path;
    use std::time::Duration;

    #[test]
    fn the_report_gets_history_and_published_scores() {
        let dir = std::env::temp_dir().join(format!("pupil-write-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let run = dir.join("run-5");
        fs::create_dir_all(&run).expect("dir");
        fs::write(dir.join("page.html"), "/*S*/const DATA = /*PUPIL_DATA*/null;").expect("page");
        fs::write(dir.join("page.css"), "b { color:red; }\n").expect("style");
        fs::write(dir.join("ref.json"), r#"{"models":[{"name":"<x>"}]}"#).expect("ref");
        fs::write(dir.join("m.jsonl"), "{\"label\":\"baseline\"}\n\nnot json\n{\"label\":\"r1\"}\n")
            .expect("measurements");
        let mut config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
        config.bench.out_dir = dir.clone();
        config.bench.template = "page.html".into();
        config.bench.style = Some("page.css".into());
        config.bench.style_marker = "/*S*/".into();
        config.bench.reference = Some("ref.json".into());
        config.bench.measurements = Some("m.jsonl".into());
        config.bench.latest_name = String::new();
        config.bench.history_runs = 5;
        let config = resolved(&config, &dir.join("pupil.toml"));
        let outcome = json!({"case": {"name": "a"}, "window": [0.0, 60.0], "checks": {"score": 1}});
        let data = json!({"started": 5.0, "outcomes": [outcome]});
        let report = write(&config, &run, &data, None).expect("write");
        let html = fs::read_to_string(report).expect("html");
        let shown =
            html.strip_prefix("b { color:red; }const DATA = ").and_then(|rest| rest.strip_suffix(';')).expect("data");
        assert!(!shown.contains('<'));
        let shown: Value = serde_json::from_str(shown).expect("json");
        assert_eq!(shown["history"][0]["run"], "run-5");
        assert_eq!(shown["reference"]["models"][0]["name"], "<x>");
        assert_eq!(shown["measurements"].as_array().map(Vec::len), Some(2));
        assert_eq!(shown["measurements"][1]["label"], "r1");
        assert!(Path::new(&run.join("results.json")).exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_briefly_locked_file_is_written_on_a_later_try() {
        let mut calls = 0;
        let done = patient(4, Duration::ZERO, || {
            calls += 1;
            if calls < 3 { Err(io::Error::other("locked")) } else { Ok(calls) }
        });
        assert_eq!((done.ok(), calls), (Some(3), 3));
    }

    #[test]
    fn a_file_that_stays_locked_fails_after_the_last_try() {
        for (tries, runs) in [(0, 1), (1, 1), (3, 3)] {
            let mut calls = 0;
            let failed: io::Result<()> = patient(tries, Duration::ZERO, || {
                calls += 1;
                Err(io::Error::other("locked"))
            });
            assert!(failed.is_err());
            assert_eq!(calls, runs, "{tries}");
        }
    }

    #[test]
    fn a_failed_write_names_the_file() {
        let mut config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
        config.bench.write_wait_ms = 0;
        let missing = std::env::temp_dir().join(format!("pupil-missing-{}", std::process::id())).join("run");
        let failed = write(&config, &missing, &json!({}), None).expect_err("no folder");
        assert!(failed.contains("results.json"), "{failed}");
    }
}
