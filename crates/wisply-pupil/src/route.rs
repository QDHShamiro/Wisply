use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use regex::Regex;
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

use crate::config::{Config, ThinkAuto};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Why {
    Quiet,
    Pattern,
    Numbers,
    Plain,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Choice {
    Fixed(bool),
    Auto(bool),
    Broken(String),
}

pub fn choose(think: bool, auto: bool, spec: &ThinkAuto, question: &str) -> Choice {
    if think || !auto {
        return Choice::Fixed(think);
    }
    match Router::new(spec) {
        Ok(router) => Choice::Auto(router.decide::<&str>(question, &[]).0),
        Err(detail) => Choice::Broken(detail),
    }
}

pub struct Router {
    quiet: Vec<Regex>,
    think: Vec<Regex>,
    wide: Vec<Regex>,
    number: Regex,
    min_numbers: usize,
    with_options: bool,
    think_sees_options: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct Case {
    group: String,
    think: bool,
    question: String,
    #[serde(default)]
    options: Vec<String>,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Tally {
    total: usize,
    on: usize,
    wrong_on: usize,
    wrong_off: usize,
    groups: BTreeMap<String, (usize, usize)>,
}

fn compile(list: &[String]) -> Result<Vec<Regex>, String> {
    list.iter().map(|raw| Regex::new(raw).map_err(|e| format!("{raw}: {e}"))).collect()
}

impl Router {
    pub fn new(spec: &ThinkAuto) -> Result<Router, String> {
        if spec.think_patterns.is_empty() && spec.option_patterns.is_empty() && spec.min_numbers == 0 {
            return Err("think_auto needs think_patterns or min_numbers".to_string());
        }
        Ok(Router {
            quiet: compile(&spec.quiet_patterns)?,
            think: compile(&spec.think_patterns)?,
            wide: compile(&spec.option_patterns)?,
            number: Regex::new(&spec.number_pattern).map_err(|e| format!("{}: {e}", spec.number_pattern))?,
            min_numbers: spec.min_numbers,
            with_options: spec.with_options,
            think_sees_options: spec.think_sees_options,
        })
    }

    pub fn decide<S: AsRef<str>>(&self, question: &str, options: &[S]) -> (bool, Why) {
        let text = if self.with_options && !options.is_empty() {
            let lines: Vec<&str> = options.iter().map(AsRef::as_ref).collect();
            format!("{question}\n{}", lines.join("\n"))
        } else {
            question.to_string()
        };
        if self.quiet.iter().any(|p| p.is_match(&text)) {
            return (false, Why::Quiet);
        }
        let narrow = if self.think_sees_options { text.as_str() } else { question };
        if self.think.iter().any(|p| p.is_match(narrow)) || self.wide.iter().any(|p| p.is_match(&text)) {
            return (true, Why::Pattern);
        }
        if self.min_numbers > 0 && self.number.find_iter(question).count() >= self.min_numbers {
            return (true, Why::Numbers);
        }
        (false, Why::Plain)
    }
}

fn tally(router: &Router, cases: &[Case]) -> Tally {
    let mut found = Tally::default();
    for case in cases {
        let (on, _) = router.decide(&case.question, &case.options);
        let entry = found.groups.entry(case.group.clone()).or_default();
        entry.0 += 1;
        found.total += 1;
        if on {
            found.on += 1;
        }
        if on != case.think {
            entry.1 += 1;
            if on {
                found.wrong_on += 1;
            } else {
                found.wrong_off += 1;
            }
        }
    }
    found
}

fn rules_hash(spec: &ThinkAuto) -> String {
    let mut digest = Sha256::new();
    for part in spec
        .quiet_patterns
        .iter()
        .chain(spec.think_patterns.iter())
        .chain(spec.option_patterns.iter())
        .chain(std::iter::once(&spec.number_pattern))
    {
        digest.update(part.as_bytes());
        digest.update([0u8]);
    }
    digest.update(spec.min_numbers.to_le_bytes());
    digest.update([u8::from(spec.with_options)]);
    digest.update([u8::from(spec.think_sees_options)]);
    digest.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

fn report(spec: &ThinkAuto, set: &Path, raw: &[u8], found: &Tally) -> Value {
    let wrong = found.wrong_on + found.wrong_off;
    let rate = |count: usize, total: usize| {
        if total == 0 { 0.0 } else { 100.0 * count as f64 / total as f64 }
    };
    let set_hash: String = Sha256::digest(raw).iter().map(|b| format!("{b:02x}")).collect();
    let groups: BTreeMap<&String, Value> =
        found.groups.iter().map(|(name, (total, bad))| (name, json!({"n": total, "wrong": bad}))).collect();
    let wrong_rate = rate(wrong, found.total);
    json!({
        "ok": true,
        "set": set.display().to_string(),
        "set_sha256": set_hash,
        "rules_sha256": rules_hash(spec),
        "n": found.total,
        "on": found.on,
        "wrong_on": found.wrong_on,
        "wrong_off": found.wrong_off,
        "wrong_rate": (wrong_rate * 10.0).round() / 10.0,
        "goal_wrong_rate": spec.goal_wrong_rate,
        "goal_met": wrong_rate <= spec.goal_wrong_rate,
        "groups": groups,
    })
}

fn set_path(args: &[String]) -> Result<PathBuf, String> {
    let mut rest = args.iter();
    let mut found = None;
    while let Some(flag) = rest.next() {
        match flag.as_str() {
            "--set" => found = Some(PathBuf::from(rest.next().ok_or_else(|| "--set needs a value".to_string())?)),
            other => return Err(format!("unknown option {other}")),
        }
    }
    found.ok_or_else(|| "--set is required".to_string())
}

fn go(config: &Config, args: &[String]) -> Result<Value, String> {
    let set = set_path(args)?;
    let spec = &config.think_auto;
    let router = Router::new(spec)?;
    let raw = std::fs::read(&set).map_err(|e| format!("{}: {e}", set.display()))?;
    let cases = String::from_utf8_lossy(&raw)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<Case>(line).map_err(|e| e.to_string()))
        .collect::<Result<Vec<Case>, String>>()?;
    Ok(report(spec, &set, &raw, &tally(&router, &cases)))
}

pub fn run(config: &Config, args: &[String]) -> i32 {
    match go(config, args) {
        Ok(record) => {
            println!("{}", serde_json::to_string_pretty(&record).unwrap_or_default());
            0
        }
        Err(message) => {
            println!("{}", json!({"ok": false, "error": "think_auto", "message": message}));
            1
        }
    }
}

#[cfg(test)]
#[path = "route_tests.rs"]
mod tests;
