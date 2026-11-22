use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde::Deserialize;
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};

use crate::ask::{self, Question, Said};
use crate::clock;
use crate::config::{Config, WriteEval};
use crate::gauge::{self, Gauge};
use crate::job::Failure;
use crate::merge;
use crate::rules::{self, Language};
use crate::score::{Kind, Mark, merge, scores, tempo};
use crate::server;
use crate::spawn;

#[derive(Debug, Deserialize)]
struct Case {
    name: String,
    prompt: String,
    checks: Map<String, Value>,
}

#[derive(Debug, Deserialize)]
struct Sum {
    question: String,
    answer: String,
}

#[derive(Debug, PartialEq)]
struct Options {
    label: String,
    tests: PathBuf,
    model: Option<String>,
    runs: usize,
    afk: bool,
}

struct Suite {
    spec: WriteEval,
    root: PathBuf,
    git: String,
    cases: Vec<Case>,
    maths: Vec<Sum>,
    hash: String,
    language: Language,
}

struct Asker<'a> {
    config: &'a Config,
    path: &'a Path,
    model: Option<String>,
    afk: bool,
    paused: f64,
}

fn options(args: &[String]) -> Result<Options, String> {
    let mut found =
        Options { label: String::new(), tests: PathBuf::from("pupil.toml"), model: None, runs: 1, afk: false };
    let mut rest = args.iter();
    while let Some(flag) = rest.next() {
        let mut value = || rest.next().cloned().ok_or_else(|| format!("{flag} needs a value"));
        match flag.as_str() {
            "--label" => found.label = value()?,
            "--tests" => found.tests = PathBuf::from(value()?),
            "--model" => found.model = Some(value()?).filter(|m| !m.is_empty()),
            "--runs" => found.runs = value()?.parse().map_err(|_| "--runs needs a number")?,
            "--afk" => found.afk = true,
            other => return Err(format!("unknown option {other}")),
        }
    }
    if found.label.trim().is_empty() || found.runs == 0 {
        return Err("--label is required and --runs must be at least 1".to_string());
    }
    Ok(found)
}

fn under(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        return path.to_path_buf();
    }
    root.join(path)
}

pub fn near(file: &Path, path: &Path) -> PathBuf {
    under(file.parent().unwrap_or(Path::new("")), path)
}

fn read(path: &Path) -> Result<Vec<u8>, String> {
    std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))
}

fn suite(tests: &Path) -> Result<Suite, String> {
    let config = Config::load(tests)?;
    let spec = config.write_eval;
    let cases_raw = read(&near(tests, &spec.cases))?;
    let cases: Vec<Case> = serde_json::from_slice(&cases_raw).map_err(|e| e.to_string())?;
    let math_raw = String::from_utf8_lossy(&read(&near(tests, &spec.math))?).into_owned();
    let rows: Vec<&str> = math_raw.lines().take(spec.math_count).collect();
    let maths = rows
        .iter()
        .map(|r| serde_json::from_str::<Sum>(r).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    let mut unknown: Vec<&str> =
        cases.iter().flat_map(|c| c.checks.keys()).map(String::as_str).filter(|n| !rules::NAMES.contains(n)).collect();
    unknown.sort_unstable();
    unknown.dedup();
    if !unknown.is_empty() {
        return Err(format!("unknown checks: {unknown:?}"));
    }
    let mut digest = Sha256::new();
    digest.update(&cases_raw);
    rows.iter().for_each(|r| digest.update(r.as_bytes()));
    digest.update(spec.math_prompt.as_bytes());
    let hash: String = digest.finalize().iter().map(|b| format!("{b:02x}")).collect();
    Ok(Suite {
        language: Language::new(&spec.german_words, &spec.english_words),
        root: tests.parent().unwrap_or(Path::new("")).to_path_buf(),
        git: config.harness.git,
        hash: hash[..16].to_string(),
        spec,
        cases,
        maths,
    })
}

impl Asker<'_> {
    fn ask(&mut self, spec: &WriteEval, prompt: &str, limit: Option<u32>) -> Result<Said, Failure> {
        let question =
            Question { prompt: prompt.to_string(), model: self.model.clone(), max_tokens: limit, afk: self.afk };
        let mut tries = 0;
        loop {
            let reply = ask::ask(self.config, self.path, &question);
            tries += 1;
            match reply {
                Err(failure) if failure.error == "game_running" && tries < spec.retry_limit => {
                    std::thread::sleep(Duration::from_secs(spec.retry_seconds));
                    self.paused += spec.retry_seconds as f64;
                }
                other => return other,
            }
        }
    }
}

fn reply_value(reply: &Result<Said, Failure>) -> Value {
    match reply {
        Ok(said) => serde_json::to_value(said),
        Err(failure) => serde_json::to_value(failure),
    }
    .unwrap_or(Value::Null)
}

fn flags(found: &Map<String, Value>) -> Vec<bool> {
    found.values().map(|v| v == &Value::Bool(true)).collect()
}

fn one_run(asker: &mut Asker, suite: &Suite) -> (Vec<Value>, Vec<Mark>, Vec<Said>) {
    let (mut items, mut marks, mut replies) = (Vec::new(), Vec::new(), Vec::new());
    for case in &suite.cases {
        let reply = asker.ask(&suite.spec, &case.prompt, None);
        let german = case.checks.contains_key("german");
        let mut item = json!({"kind": "rule", "name": case.name, "prompt": case.prompt});
        item["reply"] = reply_value(&reply);
        let mut mark = Mark { kind: Kind::Rule { german }, strict: None, loose: None };
        if let Ok(said) = reply {
            let found = rules::grade(&said.text, &case.checks, &suite.language);
            mark.strict = Some(flags(&found.strict));
            mark.loose = Some(flags(&found.loose));
            item["strict"] = Value::Object(found.strict);
            item["loose"] = Value::Object(found.loose);
            item["german"] = Value::Bool(german);
            replies.push(said);
        }
        items.push(item);
        marks.push(mark);
    }
    for (number, row) in suite.maths.iter().enumerate() {
        let prompt = suite.spec.math_prompt.replace("{question}", &row.question);
        let reply = asker.ask(&suite.spec, &prompt, None);
        let gold = row.answer.rsplit("####").next().and_then(rules::to_number);
        let mut item = json!({"kind": "math", "name": format!("gsm8k-{number}"), "gold": gold});
        item["reply"] = reply_value(&reply);
        let mut mark = Mark { kind: Kind::Math, strict: None, loose: None };
        if let Ok(said) = reply {
            let (strict, loose) = rules::math_pick(&said.text);
            let (strict, loose) = (rules::same(strict, gold), rules::same(loose, gold));
            mark.strict = Some(vec![strict]);
            mark.loose = Some(vec![loose]);
            item["strict"] = Value::Bool(strict);
            item["loose"] = Value::Bool(loose);
            replies.push(said);
        }
        items.push(item);
        marks.push(mark);
    }
    (items, marks, replies)
}

pub(crate) fn keep(out_dir: &Path, root: &Path, label: &str, run: usize, items: &[Value]) -> Result<String, String> {
    let dir = under(root, out_dir);
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let name = format!("{label}-{}-{run}.json", clock::local().file());
    let mut raw = Vec::new();
    let style = serde_json::ser::PrettyFormatter::with_indent(b" ");
    let mut writer = serde_json::Serializer::with_formatter(&mut raw, style);
    serde::Serialize::serialize(items, &mut writer).map_err(|e| e.to_string())?;
    std::fs::write(dir.join(&name), raw).map_err(|e| e.to_string())?;
    Ok(name)
}

pub(crate) fn commit(git: &str, root: &Path) -> String {
    let mut command = spawn::quiet(git);
    command.args(["rev-parse", "--short", "HEAD"]);
    if !root.as_os_str().is_empty() {
        command.current_dir(root);
    }
    command.output().map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).unwrap_or_default()
}

pub fn append(path: &Path, record: &Value) -> Result<(), String> {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    writeln!(file, "{record}").map_err(|e| e.to_string())
}

fn go(config: &Config, config_path: &Path, args: &[String]) -> Result<Value, String> {
    let opts = options(args)?;
    let suite = suite(&opts.tests)?;
    let exe = std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default();
    let mut record = Map::new();
    record.insert("label".into(), json!(opts.label));
    record.insert("time".into(), json!(clock::local().text()));
    record.insert("commit".into(), json!(commit(&suite.git, &suite.root)));
    record.insert("eval_hash".into(), json!(suite.hash));
    record.insert("runs".into(), json!(opts.runs));
    record.insert("config".into(), json!(config_path.display().to_string()));
    record.insert("exe".into(), json!(exe));
    let mut asker = Asker { config, path: config_path, model: opts.model.clone(), afk: opts.afk, paused: 0.0 };
    let gauge = Gauge::start(&config.harness);
    let warm = asker.ask(&suite.spec, &suite.spec.warm_prompt, Some(suite.spec.warm_tokens));
    record.insert("start_s".into(), json!(warm.ok().map(|s| s.server_seconds)));
    let (mut quality, mut speed, mut kept) = (Vec::new(), Vec::new(), Vec::new());
    let mut outcome = Ok(());
    for run in 0..opts.runs {
        let begun = Instant::now();
        let (items, marks, replies) = one_run(&mut asker, &suite);
        let wall = begun.elapsed().as_secs_f64();
        if replies.is_empty() {
            outcome = Err(items.first().map(|i| i["reply"].to_string()).unwrap_or_default());
            break;
        }
        quality.push(scores(&marks));
        speed.push(tempo(&replies, items.len(), wall));
        match keep(&suite.spec.out_dir, &suite.root, &opts.label, run, &items) {
            Ok(name) => kept.push(name),
            Err(e) => {
                outcome = Err(e);
                break;
            }
        }
    }
    server::stop(config);
    let readings = gauge.finish();
    outcome?;
    merge(&quality, &mut record);
    merge(&speed, &mut record);
    record.insert("answers".into(), json!(kept));
    record.insert("paused_s".into(), json!(asker.paused));
    record.insert("vram_peak_mib".into(), json!(gauge::peak(&readings.vram)));
    record.insert("ram_peak_mib".into(), json!(gauge::peak(&readings.ram)));
    record.insert("energy_wh".into(), json!(readings.energy_wh()));
    let record = Value::Object(record);
    append(&near(&opts.tests, &suite.spec.results), &record)?;
    Ok(record)
}

pub fn run(config: &Config, config_path: &Path, args: &[String]) -> i32 {
    let result = if args.iter().any(|a| a == "--merge") { merge::join(args) } else { go(config, config_path, args) };
    match result {
        Ok(record) => {
            println!("{}", serde_json::to_string_pretty(&record).unwrap_or_default());
            0
        }
        Err(message) => {
            println!("{}", json!({"ok": false, "error": "write_eval", "message": message}));
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{options, suite};

    #[test]
    fn options_need_a_label() {
        assert!(options(&[]).is_err());
        let args: Vec<String> = ["--label", "x", "--runs", "5", "--afk"].map(String::from).to_vec();
        let found = options(&args).expect("parses");
        assert_eq!((found.runs, found.afk, found.label.as_str()), (5, true, "x"));
    }

    #[test]
    fn the_shipped_test_set_loads_with_known_checks() {
        let root = std::env::temp_dir().join(format!("pupil-write-{}", std::process::id()));
        let math = root.join("write").join("gsm8k");
        std::fs::create_dir_all(&math).expect("math dir");
        std::fs::create_dir_all(root.join("bench")).expect("bench dir");
        std::fs::write(root.join("pupil.toml"), include_str!("../pupil.toml")).expect("config");
        std::fs::write(root.join("bench").join("write-cases.json"), include_bytes!("../bench/write-cases.json"))
            .expect("cases");
        let rows: String =
            (1..=60).map(|n| format!("{{\"question\": \"{n} + {n}?\", \"answer\": \"#### {}\"}}\n", n * 2)).collect();
        std::fs::write(math.join("test.jsonl"), rows).expect("rows");
        let found = suite(&root.join("pupil.toml"));
        let _ = std::fs::remove_dir_all(&root);
        let found = found.expect("loads");
        assert_eq!((found.cases.len(), found.maths.len(), found.hash.len()), (40, 50, 16));
    }
}
