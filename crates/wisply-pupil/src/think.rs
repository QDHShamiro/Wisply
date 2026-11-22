use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use regex::Regex;
use serde::Deserialize;
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};

use crate::chat;
use crate::clock;
use crate::config::{Config, Model, ThinkEval};
use crate::game;
use crate::gauge::{self, Gauge};
use crate::job::Failure;
use crate::route::Router;
use crate::rules;
use crate::score::merge;
use crate::server::{self, Busy};
use crate::stream::{self, Piece};
use crate::talk;
use crate::watch::{request_failure, start_failure};
use crate::write::{append, commit, keep, near};

#[path = "think_sum.rs"]
mod think_sum;

use think_sum::summary;

#[derive(Debug, Clone, Deserialize)]
struct Row {
    id: String,
    group: String,
    kind: String,
    question: String,
    #[serde(default)]
    options: Vec<String>,
    answer: String,
}

#[derive(Debug, PartialEq)]
struct Options {
    label: String,
    tests: PathBuf,
    model: Option<String>,
    runs: usize,
    think: bool,
    auto: bool,
    budget: Option<u64>,
    limit: Option<usize>,
    afk: bool,
    probs: Option<u32>,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Reply {
    text: String,
    thought: String,
    thought_tokens: u64,
    answer_tokens: u64,
    finish: String,
    seconds: f64,
    first_seconds: f64,
    answer_per_second: f64,
    probs: Vec<Value>,
}

struct Picker {
    strict: Regex,
    loose: Regex,
    tail: usize,
}

struct Suite {
    spec: ThinkEval,
    root: PathBuf,
    git: String,
    rows: Vec<Row>,
    hash: String,
    picker: Picker,
}

struct Asker<'a> {
    config: &'a Config,
    path: &'a Path,
    model: Model,
    afk: bool,
    think: bool,
    router: Option<Router>,
    budget: Option<u64>,
    probs: Option<u32>,
    paused: f64,
}

fn options(args: &[String]) -> Result<Options, String> {
    let mut found = Options {
        label: String::new(),
        tests: PathBuf::from("pupil.toml"),
        model: None,
        runs: 1,
        think: false,
        auto: false,
        budget: None,
        limit: None,
        afk: false,
        probs: None,
    };
    let mut rest = args.iter();
    while let Some(flag) = rest.next() {
        let mut value = || rest.next().cloned().ok_or_else(|| format!("{flag} needs a value"));
        match flag.as_str() {
            "--label" => found.label = value()?,
            "--tests" => found.tests = PathBuf::from(value()?),
            "--model" => found.model = Some(value()?).filter(|m| !m.is_empty()),
            "--runs" => found.runs = value()?.parse().map_err(|_| "--runs needs a number")?,
            "--budget" => found.budget = Some(value()?.parse().map_err(|_| "--budget needs a number")?),
            "--limit" => found.limit = Some(value()?.parse().map_err(|_| "--limit needs a number")?),
            "--probs" => found.probs = Some(value()?.parse().map_err(|_| "--probs needs a number")?),
            "--think" => found.think = true,
            "--auto" => found.auto = true,
            "--afk" => found.afk = true,
            other => return Err(format!("unknown option {other}")),
        }
    }
    if found.label.trim().is_empty() || found.runs == 0 {
        return Err("--label is required and --runs must be at least 1".to_string());
    }
    if found.think && found.auto {
        return Err("--think and --auto exclude each other".to_string());
    }
    if found.budget.is_some() && !found.think && !found.auto {
        return Err("--budget only works with --think or --auto".to_string());
    }
    Ok(found)
}

impl Picker {
    fn new(spec: &ThinkEval) -> Result<Picker, String> {
        let build = |raw: &str| Regex::new(raw).map_err(|e| format!("{raw}: {e}"));
        Ok(Picker {
            strict: build(&spec.letter_pattern)?,
            loose: build(&spec.loose_letter_pattern)?,
            tail: spec.loose_tail_chars,
        })
    }

    fn letter(&self, text: &str) -> (Option<String>, Option<String>) {
        let last = |pattern: &Regex, text: &str| {
            pattern.captures_iter(text).last().and_then(|c| c.get(1)).map(|m| m.as_str().to_uppercase())
        };
        let strict = last(&self.strict, text);
        let loose = strict.clone().or_else(|| last(&self.loose, tail(text, self.tail)));
        (strict, loose)
    }
}

fn tail(text: &str, chars: usize) -> &str {
    let count = text.chars().count();
    let skip = count.saturating_sub(chars);
    text.char_indices().nth(skip).map(|(at, _)| &text[at..]).unwrap_or("")
}

fn prompt(spec: &ThinkEval, row: &Row) -> String {
    if row.options.is_empty() {
        return spec.number_prompt.replace("{question}", &row.question);
    }
    let lines: Vec<String> = row
        .options
        .iter()
        .zip(spec.letters.chars())
        .map(|(option, letter)| spec.option_line.replace("{letter}", &letter.to_string()).replace("{option}", option))
        .collect();
    spec.letter_prompt.replace("{question}", &row.question).replace("{options}", &lines.join("\n"))
}

fn grade(row: &Row, text: &str, picker: &Picker) -> (Value, bool, bool) {
    if row.options.is_empty() {
        let gold = rules::to_number(&row.answer);
        let (strict, loose) = rules::math_pick(text);
        return (json!(loose), rules::same(strict, gold), rules::same(loose, gold));
    }
    let gold = row.answer.trim().to_uppercase();
    let (strict, loose) = picker.letter(text);
    let hit = |found: &Option<String>| found.as_deref() == Some(gold.as_str());
    (json!(loose), hit(&strict), hit(&loose))
}

fn suite(tests: &Path) -> Result<Suite, String> {
    let config = Config::load(tests)?;
    let spec = config.think_eval;
    let path = near(tests, &spec.set);
    let raw = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let rows = String::from_utf8_lossy(&raw)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| serde_json::from_str::<Row>(line).map_err(|e| format!("{line}: {e}")))
        .collect::<Result<Vec<Row>, String>>()?;
    let mut digest = Sha256::new();
    digest.update(&raw);
    for part in [
        &spec.letter_prompt,
        &spec.number_prompt,
        &spec.option_line,
        &spec.letters,
        &spec.letter_pattern,
        &spec.loose_letter_pattern,
    ] {
        digest.update(part.as_bytes());
    }
    digest.update(spec.max_tokens.to_le_bytes());
    digest.update(spec.loose_tail_chars.to_le_bytes());
    let hash: String = digest.finalize().iter().map(|b| format!("{b:02x}")).collect();
    Ok(Suite {
        picker: Picker::new(&spec)?,
        root: tests.parent().unwrap_or(Path::new("")).to_path_buf(),
        git: config.harness.git,
        hash: hash[..16].to_string(),
        spec,
        rows,
    })
}

impl Asker<'_> {
    fn once(&self, spec: &ThinkEval, prompt: &str, limit: u32, think: bool) -> Result<Reply, Failure> {
        let config = self.config;
        if let Some(name) = game::running(&config.game, self.afk) {
            return Err(Failure::new(config, "game_running", &[("game", &name)]));
        }
        server::set_afk(config, self.afk);
        server::ensure(config, self.path, &self.model).map_err(|e| start_failure(config, self.afk, e))?;
        let user = json!({"role": "user", "content": prompt});
        let mut sent = talk::body(config, &self.model.name, &[], &user, limit, think);
        if let (Some(budget), true) = (self.budget, think) {
            sent[spec.budget_field.as_str()] = json!(budget);
        }
        if let Some(top) = self.probs {
            chat::add_probs(&mut sent, top);
        }
        let mut pieces = 0u64;
        let begun = Instant::now();
        let acc = {
            let _busy = Busy::mark(config);
            stream::send(config, &sent, &mut |piece| {
                if let Piece::Thought(_) = piece {
                    pieces += 1;
                }
                true
            })
        };
        let seconds = begun.elapsed().as_secs_f64();
        let acc = acc.map_err(|detail| request_failure(config, self.afk, detail))?;
        Ok(Reply {
            text: acc.text.trim().to_string(),
            thought_tokens: pieces,
            answer_tokens: acc.answer_tokens.saturating_sub(pieces),
            thought: acc.thought,
            finish: acc.finish,
            seconds,
            first_seconds: acc.first_seconds,
            answer_per_second: acc.answer_per_second,
            probs: acc.probs,
        })
    }

    fn ask(&mut self, spec: &ThinkEval, prompt: &str, limit: u32, think: bool) -> Result<Reply, Failure> {
        let mut tries = 0;
        loop {
            let reply = self.once(spec, prompt, limit, think);
            tries += 1;
            match reply {
                Err(failure) if spec.game_errors.contains(&failure.error) && tries < spec.retry_limit => {
                    std::thread::sleep(Duration::from_secs(spec.retry_seconds));
                    self.paused += spec.retry_seconds as f64;
                }
                other => return other,
            }
        }
    }
}

fn one_run(asker: &mut Asker, suite: &Suite, rows: &[&Row]) -> (Vec<Value>, Map<String, Value>) {
    let spec = &suite.spec;
    let begun = Instant::now();
    let (mut items, mut marks, mut replies) = (Vec::new(), Vec::new(), Vec::new());
    for row in rows {
        let think = asker.router.as_ref().map_or(asker.think, |router| router.decide(&row.question, &row.options).0);
        let reply = asker.ask(spec, &prompt(spec, row), spec.max_tokens, think);
        let mut item = json!({"id": row.id, "group": row.group, "kind": row.kind, "gold": row.answer, "think": think});
        match reply {
            Ok(reply) => {
                let (picked, strict, loose) = grade(row, &reply.text, &suite.picker);
                item["picked"] = picked;
                item["strict"] = json!(strict);
                item["loose"] = json!(loose);
                item["seconds"] = json!(reply.seconds);
                item["first_seconds"] = json!(reply.first_seconds);
                item["thought_tokens"] = json!(reply.thought_tokens);
                item["answer_tokens"] = json!(reply.answer_tokens);
                item["finish"] = json!(reply.finish);
                item["thought"] = json!(reply.thought);
                item["text"] = json!(reply.text);
                if !reply.probs.is_empty() {
                    item["probs"] = json!(reply.probs);
                }
                marks.push(Some((strict, loose)));
                replies.push(reply);
            }
            Err(failure) => {
                item["error"] = serde_json::to_value(&failure).unwrap_or(Value::Null);
                marks.push(None);
            }
        }
        items.push(item);
    }
    let wall = begun.elapsed().as_secs_f64();
    (items, summary(rows, &marks, &replies, wall))
}

fn go(config: &Config, config_path: &Path, args: &[String]) -> Result<Value, String> {
    let opts = options(args)?;
    let suite = suite(&opts.tests)?;
    let model = config
        .model(opts.model.as_deref())
        .cloned()
        .ok_or_else(|| format!("unknown model, known: {}", config.model_names()))?;
    let rows: Vec<&Row> = suite.rows.iter().take(opts.limit.unwrap_or(usize::MAX)).collect();
    let mut record = header(&opts, &suite, rows.len(), config_path);
    let mut asker = Asker {
        config,
        path: config_path,
        model,
        afk: opts.afk,
        think: opts.think,
        router: opts.auto.then(|| Router::new(&config.think_auto)).transpose()?,
        budget: opts.budget,
        probs: opts.probs,
        paused: 0.0,
    };
    let spec = &suite.spec;
    let gauge = Gauge::start(&config.harness);
    let started = Instant::now();
    let warm = asker.ask(spec, &spec.warm_prompt, spec.warm_tokens, false);
    record.insert("start_s".into(), json!(warm.ok().map(|_| started.elapsed().as_secs_f64())));
    let (per_run, kept, outcome) = runs(&mut asker, &suite, &rows, &opts);
    server::stop(config);
    let readings = gauge.finish();
    outcome?;
    merge(&per_run, &mut record);
    record.insert("answers".into(), json!(kept));
    record.insert("paused_s".into(), json!(asker.paused));
    record.insert("vram_peak_mib".into(), json!(gauge::peak(&readings.vram)));
    record.insert("ram_peak_mib".into(), json!(gauge::peak(&readings.ram)));
    record.insert("energy_wh".into(), json!(readings.energy_wh()));
    let record = Value::Object(record);
    append(&near(&opts.tests, &spec.results), &record)?;
    Ok(record)
}

fn header(opts: &Options, suite: &Suite, rows: usize, config_path: &Path) -> Map<String, Value> {
    let exe = std::env::current_exe().map(|p| p.display().to_string()).unwrap_or_default();
    let mut record = Map::new();
    record.insert("label".into(), json!(opts.label));
    record.insert("time".into(), json!(clock::local().text()));
    record.insert("commit".into(), json!(commit(&suite.git, &suite.root)));
    record.insert("eval_hash".into(), json!(suite.hash));
    record.insert("rows".into(), json!(rows));
    record.insert("think".into(), json!(opts.think));
    record.insert("auto".into(), json!(opts.auto));
    record.insert("budget".into(), json!(opts.budget));
    record.insert("runs".into(), json!(opts.runs));
    record.insert("config".into(), json!(config_path.display().to_string()));
    record.insert("exe".into(), json!(exe));
    record
}

type Runs = (Vec<Map<String, Value>>, Vec<String>, Result<(), String>);

fn runs(asker: &mut Asker, suite: &Suite, rows: &[&Row], opts: &Options) -> Runs {
    let (mut per_run, mut kept) = (Vec::new(), Vec::new());
    for run in 0..opts.runs {
        let (items, sums) = one_run(asker, suite, rows);
        if sums.get("error_rate").and_then(Value::as_f64) == Some(1.0) {
            let first = items.first().map(|i| i["error"].to_string());
            return (per_run, kept, Err(first.unwrap_or_default()));
        }
        per_run.push(sums);
        match keep(&suite.spec.out_dir, &suite.root, &opts.label, run, &items) {
            Ok(name) => kept.push(name),
            Err(e) => return (per_run, kept, Err(e)),
        }
    }
    (per_run, kept, Ok(()))
}

pub fn run(config: &Config, config_path: &Path, args: &[String]) -> i32 {
    match go(config, config_path, args) {
        Ok(record) => {
            println!("{}", serde_json::to_string_pretty(&record).unwrap_or_default());
            0
        }
        Err(message) => {
            println!("{}", json!({"ok": false, "error": "think_eval", "message": message}));
            1
        }
    }
}

#[cfg(test)]
#[path = "think_tests.rs"]
mod tests;
