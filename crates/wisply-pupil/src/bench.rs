use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::config::Config;
use crate::game;
use crate::grab;
use crate::history;
use crate::job::{Failure, Job, Reply, Segment};
use crate::page;
use crate::quality::{self, Checks, Item, Window};
use crate::server;
use crate::spawn;
use crate::video;
use crate::watch;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Case {
    pub name: String,
    pub video: String,
    #[serde(default)]
    pub kind: String,
    pub mode: String,
    #[serde(default)]
    pub start: f64,
    #[serde(default)]
    pub end: Option<f64>,
    pub job: PathBuf,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub channel: String,
    #[serde(default)]
    pub audio: Option<PathBuf>,
    #[serde(default)]
    pub hear_only: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Outcome {
    pub case: Case,
    pub model: String,
    pub game: Option<String>,
    pub window: (f64, f64),
    pub reply: Option<Reply>,
    pub failure: Option<Failure>,
    pub answer: Option<Value>,
    pub items: Vec<Item>,
    pub checks: Option<Checks>,
    pub thumbs: Vec<String>,
    pub realtime: Option<f64>,
    pub paused: f64,
}

struct Options {
    cases: PathBuf,
    models: Vec<String>,
    only: Vec<String>,
    baseline: Option<PathBuf>,
}

fn options(config: &Config, args: &[String]) -> Options {
    let mut found = Options {
        cases: config.bench.cases.clone(),
        models: config.bench.models.clone(),
        only: Vec::new(),
        baseline: None,
    };
    let list = |text: &str| text.split(',').map(str::trim).filter(|s| !s.is_empty()).map(String::from).collect();
    let mut index = 0;
    while index < args.len() {
        let next = args.get(index + 1).map(String::as_str).unwrap_or_default();
        match args[index].as_str() {
            "--model" => found.models = list(next),
            "--only" => found.only = list(next),
            "--baseline" => found.baseline = Some(PathBuf::from(next)),
            other => {
                found.cases = PathBuf::from(other);
                index += 1;
                continue;
            }
        }
        index += 2;
    }
    found
}

fn gpu(config: &Config) -> Option<String> {
    let (program, rest) = config.bench.gpu_query.split_first()?;
    let output = spawn::quiet(program).args(rest).output().ok()?;
    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (output.status.success() && !text.is_empty()).then_some(text)
}

fn thumbs(config: &Config, job: &Job, times: &[f64], dir: &Path, prefix: &str) -> Vec<String> {
    let Ok(info) = video::probe(&config.frames, &job.video) else {
        return Vec::new();
    };
    let size = video::frame_size(info.width, info.height, config.bench.thumb_long_side, 2);
    let shots: Vec<video::Shot> = times.iter().map(|&at| video::Shot { at, size, small: false }).collect();
    let Ok(pictures) = grab::extract(&config.frames, &job.video, &shots) else {
        return Vec::new();
    };
    let folder = dir.join("frames").join(prefix);
    if fs::create_dir_all(&folder).is_err() {
        return Vec::new();
    }
    pictures
        .iter()
        .enumerate()
        .filter_map(|(index, picture)| {
            let name = format!("{index:03}.jpg");
            fs::write(folder.join(&name), &picture.jpeg).ok()?;
            Some(format!("frames/{prefix}/{name}"))
        })
        .collect()
}

fn run_case(config: &Config, config_path: &Path, case: &Case, model: &str, dir: &Path) -> Outcome {
    let mut outcome = Outcome {
        case: case.clone(),
        model: model.to_string(),
        game: game::running(&config.game, false),
        window: (case.start, case.end.unwrap_or(case.start)),
        reply: None,
        failure: None,
        answer: None,
        items: Vec::new(),
        checks: None,
        thumbs: Vec::new(),
        realtime: None,
        paused: 0.0,
    };
    let mut job = match load_job(config, case, model) {
        Ok(job) => job,
        Err(failure) => {
            outcome.failure = Some(failure);
            return outcome;
        }
    };
    let truth = std::mem::take(&mut job.transcript);
    if !case.hear_only {
        job.transcript.clone_from(&truth);
    }
    if case.audio.is_some() {
        job.audio.clone_from(&case.audio);
    }
    if let Ok(info) = video::probe(&config.frames, &job.video) {
        let end = job.end.unwrap_or(info.duration).min(info.duration);
        outcome.window = (job.start, end);
    }
    let (result, paused) = attempt(config, config_path, &job, case);
    outcome.paused = paused;
    match result {
        Ok(reply) => judge(config, &job, &truth, reply, dir, &mut outcome),
        Err(failure) => outcome.failure = Some(failure),
    }
    outcome
}

fn load_job(config: &Config, case: &Case, model: &str) -> Result<Job, Failure> {
    let raw =
        fs::read_to_string(&case.job).map_err(|e| Failure::new(config, "bad_job", &[("detail", &e.to_string())]))?;
    let mut job = Job::parse(config, &raw)?;
    job.model = Some(model.to_string());
    Ok(job)
}

fn attempt(config: &Config, config_path: &Path, job: &Job, case: &Case) -> (Result<Reply, Failure>, f64) {
    let begun = Instant::now();
    let limit = config.bench.game_wait_seconds;
    let poll = Duration::from_secs_f64(config.bench.game_poll_seconds.max(1.0));
    let mut paused = 0.0;
    loop {
        let result = watch::watch(config, config_path, job);
        let blocked = matches!(&result, Err(f) if f.error == config.bench.game_error);
        if !blocked || begun.elapsed().as_secs_f64() >= limit {
            return (result, paused);
        }
        let wait = Instant::now();
        while let Some(name) = game::running(&config.game, job.afk) {
            print(&json!({"case": case.name, "paused": name}));
            if begun.elapsed().as_secs_f64() >= limit {
                break;
            }
            std::thread::sleep(poll);
        }
        paused += wait.elapsed().as_secs_f64();
    }
}

fn judge(config: &Config, job: &Job, truth: &[Segment], reply: Reply, dir: &Path, outcome: &mut Outcome) {
    let window = Window { start: outcome.window.0, end: outcome.window.1 };
    let (answer, items, checks) = quality::check(&reply.text, &reply.finish, &window, truth, &config.bench);
    let length = outcome.window.1 - outcome.window.0;
    outcome.realtime = (reply.seconds.total > 0.0).then(|| length / reply.seconds.total);
    let prefix = format!("{}-{}", outcome.model, outcome.case.name);
    outcome.thumbs = thumbs(config, job, &reply.frames, dir, &prefix);
    outcome.answer = answer;
    outcome.items = items;
    outcome.checks = Some(checks);
    outcome.reply = Some(reply);
}

fn load_baseline(path: &Path) -> Result<Value, String> {
    let file = if path.is_dir() { path.join("results.json") } else { path.to_path_buf() };
    let raw = fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?;
    let mut value: Value = serde_json::from_str(&raw).map_err(|e| format!("{}: {e}", file.display()))?;
    if let Some(map) = value.as_object_mut() {
        map.remove("baseline");
    }
    Ok(value)
}

fn read_cases(config: &Config, chosen: &Options) -> Result<Vec<Case>, Failure> {
    let bad = |detail: String| Failure::new(config, "bench_cases", &[("detail", &detail)]);
    let raw = fs::read_to_string(&chosen.cases).map_err(|e| bad(format!("{}: {e}", chosen.cases.display())))?;
    let cases: Vec<Case> = serde_json::from_str(&raw).map_err(|e| bad(e.to_string()))?;
    let picked: Vec<Case> =
        cases.into_iter().filter(|c| chosen.only.is_empty() || chosen.only.contains(&c.name)).collect();
    if picked.is_empty() {
        return Err(bad(chosen.cases.display().to_string()));
    }
    Ok(picked)
}

fn print(value: &Value) {
    println!("{value}");
}

fn line(outcome: &Outcome) -> Value {
    json!({
        "case": outcome.case.name,
        "model": outcome.model,
        "score": outcome.checks.as_ref().map(|c| c.score),
        "seconds": outcome.reply.as_ref().map(|r| (r.seconds.total * 10.0).round() / 10.0),
        "error": outcome.failure.as_ref().map(|f| f.error.clone()),
    })
}

fn snapshot(
    config: &Config,
    config_path: &Path,
    started: f64,
    gpu: &Option<String>,
    chosen: &Options,
    planned: usize,
    outcomes: &[Outcome],
) -> Value {
    let weights = &config.bench.weights;
    json!({
        "started": started,
        "finished": server::now(),
        "done": outcomes.len(),
        "planned": planned,
        "gpu": gpu,
        "models": chosen.models,
        "config": config_path.file_stem().map(|s| s.to_string_lossy()),
        "seed": config.sampling.seed,
        "cold_start": config.bench.cold_start,
        "kv_type": config.server.kv_type,
        "frames": {
            "change": config.frames.change,
            "long_side": config.frames.long_side,
            "max_frames": config.frames.max_frames,
            "context": config.server.context,
        },
        "weights": {
            "complete": weights.complete,
            "times": weights.times,
            "order": weights.order,
            "spread": weights.spread,
            "unique": weights.unique,
            "coverage": weights.coverage,
            "grounded": weights.grounded,
        },
        "outcomes": outcomes,
    })
}

pub fn run(config: &Config, config_path: &Path, args: &[String]) -> i32 {
    let config = page::resolved(config, config_path);
    let chosen = options(&config, args);
    let cases = match read_cases(&config, &chosen) {
        Ok(cases) => cases,
        Err(failure) => {
            print(&json!(failure));
            return 1;
        }
    };
    let started = server::now();
    let dir = config.bench.out_dir.join(format!("{}{}", history::RUN_PREFIX, started as u64));
    let out_failure = |detail: String| {
        let path = dir.display().to_string();
        json!(Failure::new(&config, "bench_out", &[("detail", &detail), ("path", &path)]))
    };
    if let Err(e) = fs::create_dir_all(&dir) {
        print(&out_failure(e.to_string()));
        return 1;
    }
    let session = Session {
        config: &config,
        config_path,
        chosen: &chosen,
        dir: &dir,
        started,
        gpu: gpu(&config),
        planned: chosen.models.len() * cases.len(),
        baseline: None,
    };
    match session.with_baseline().and_then(|s| s.measure(&cases)) {
        Ok(report) => {
            print(&json!({"ok": true, "report": report}));
            0
        }
        Err(e) => {
            print(&out_failure(e));
            1
        }
    }
}

struct Session<'a> {
    config: &'a Config,
    config_path: &'a Path,
    chosen: &'a Options,
    dir: &'a Path,
    started: f64,
    gpu: Option<String>,
    planned: usize,
    baseline: Option<Value>,
}

impl Session<'_> {
    fn with_baseline(mut self) -> Result<Self, String> {
        self.baseline = self.chosen.baseline.as_deref().map(load_baseline).transpose()?;
        Ok(self)
    }

    fn publish(&self, outcomes: &[Outcome]) -> Result<PathBuf, String> {
        let data =
            snapshot(self.config, self.config_path, self.started, &self.gpu, self.chosen, self.planned, outcomes);
        page::write(self.config, self.dir, &data, self.baseline.as_ref())
    }

    fn measure(&self, cases: &[Case]) -> Result<PathBuf, String> {
        let mut outcomes: Vec<Outcome> = Vec::new();
        let mut report = self.publish(&outcomes)?;
        print(&json!({"ok": true, "running": true, "report": report}));
        if self.config.bench.cold_start {
            server::stop(self.config);
        }
        for model in &self.chosen.models {
            for case in cases {
                let outcome = run_case(self.config, self.config_path, case, model, self.dir);
                print(&line(&outcome));
                outcomes.push(outcome);
                report = self.publish(&outcomes)?;
            }
        }
        Ok(report)
    }
}

#[cfg(test)]
mod tests {
    use super::Case;

    #[test]
    fn cases_from_the_job_builder_are_read() {
        let raw = r#"[{"name":"a","video":"x","kind":"steps","mode":"full","start":0,"end":null,"job":"j.json","title":"T","channel":"C"},
                      {"name":"b","video":"y","mode":"summary","start":900,"end":1500,"job":"k.json"}]"#;
        let cases: Vec<Case> = serde_json::from_str(raw).expect("cases");
        assert_eq!(cases.len(), 2);
        assert_eq!(cases[0].end, None);
        assert_eq!(cases[1].end, Some(1500.0));
        assert_eq!(cases[1].title, "");
        assert!(!cases[0].hear_only);
        assert!(cases[0].audio.is_none());
    }

    #[test]
    fn hearing_cases_carry_their_sound_and_hide_the_transcript() {
        let raw = r#"[{"name":"a","video":"x","mode":"steps","job":"j.json","audio":"s.m4a","hear_only":true}]"#;
        let cases: Vec<Case> = serde_json::from_str(raw).expect("cases");
        assert!(cases[0].hear_only);
        assert_eq!(cases[0].audio.as_deref(), Some(std::path::Path::new("s.m4a")));
    }
}
