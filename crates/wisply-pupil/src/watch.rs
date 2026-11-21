use std::io::Read;
use std::path::Path;
use std::thread::JoinHandle;
use std::time::Instant;

use serde_json::{Map, Value};

use crate::chat;
use crate::config::{Config, Model};
use crate::ear;
use crate::game;
use crate::inputs::{Pictures, pictures};
use crate::job::{Failure, Job, Reply, Seconds};
use crate::long;
use crate::repair;
use crate::server::{self, Busy, StartError};
use crate::timecode;
use crate::video;

#[path = "watch_parts.rs"]
mod parts;

use parts::{Long, watch_long};

pub(crate) struct Plan {
    pub(crate) end: f64,
    pub(crate) size: (u32, u32),
    pub(crate) small: (u32, u32),
    pub(crate) cap: usize,
    pub(crate) tail: String,
    pub(crate) hears: bool,
}

type Starter = JoinHandle<(Result<(), StartError>, f64)>;

fn fail(config: &Config, code: &str, fields: &[(&str, &str)]) -> Failure {
    Failure::new(config, code, fields)
}

pub(crate) fn check(config: &Config, job: &Job) -> Result<Model, Failure> {
    let Some(model) = config.model(job.model.as_deref()).cloned() else {
        let name = job.model.clone().unwrap_or_default();
        return Err(fail(config, "model_unknown", &[("name", &name), ("names", &config.model_names())]));
    };
    for path in [&model.file, &model.mmproj] {
        if !path.is_file() {
            return Err(fail(config, "model_missing", &[("path", &path.display().to_string())]));
        }
    }
    if !job.video.is_file() {
        return Err(fail(config, "video_missing", &[("path", &job.video.display().to_string())]));
    }
    if let Some(name) = game::running(&config.game, job.afk) {
        return Err(fail(config, "game_running", &[("game", &name)]));
    }
    Ok(model)
}

pub(crate) fn window(config: &Config, job: &Job) -> Result<(video::Info, f64), Failure> {
    let info =
        video::probe(&config.frames, &job.video).map_err(|e| fail(config, "ffmpeg_failed", &[("detail", &e)]))?;
    let end = job.end.unwrap_or(info.duration).min(info.duration);
    if job.start >= end {
        let (start, end) = (timecode::format(job.start), timecode::format(end));
        return Err(fail(config, "no_frames", &[("start", &start), ("end", &end)]));
    }
    Ok((info, end))
}

pub(crate) fn plan(config: &Config, job: &Job, info: &video::Info, end: f64, hears: bool) -> Result<Plan, Failure> {
    let frames = &config.frames;
    let transcript = chat::heard_block(config, &job.transcript, job.start, end, hears);
    let window = (job.start, end);
    let tail = chat::tail_text(config, &job.prompt, &transcript, &job.schema, window);
    let size = video::frame_size(info.width, info.height, frames.long_side, frames.patch);
    let small = if frames.fast_long_side > 0 && frames.fast_long_side < frames.long_side {
        video::frame_size(info.width, info.height, frames.fast_long_side, frames.patch)
    } else {
        size
    };
    let patch = frames.patch.max(1);
    let per_frame = (size.0 / patch) * (size.1 / patch) + frames.label_tokens;
    let sound_tokens = if hears {
        ((end - job.start) * config.audio.tokens_per_second).ceil() as u32
            + chat::estimate_tokens(&config.audio.header, &config.budget)
    } else {
        0
    };
    let text_tokens = chat::estimate_tokens(&tail, &config.budget)
        + chat::estimate_tokens(&config.text.frames_header, &config.budget)
        + sound_tokens;
    let sampling = &config.sampling;
    let cap = chat::frame_budget(
        config.server.context,
        sampling.max_tokens,
        config.budget.spare_tokens,
        text_tokens,
        per_frame,
        job.max_frames.map_or(frames.max_frames, |cap| cap.min(frames.max_frames)),
    )
    .map_err(|used| {
        let (tokens, context) = (used.to_string(), config.server.context.to_string());
        fail(config, "too_long", &[("tokens", &tokens), ("context", &context)])
    })?;
    let cap = chat::minute_cap(cap, end - job.start, frames.max_frames_per_minute);
    Ok(Plan { end, size, small, cap, tail, hears })
}

pub fn game_failure(config: &Config, afk: bool) -> Option<Failure> {
    let name = game::running(&config.game, afk)?;
    Some(fail(config, "game_running", &[("game", &name)]))
}

fn paused_failure(config: &Config, afk: bool) -> Option<Failure> {
    game_failure(config, afk).or_else(|| {
        server::closed_reason(config).filter(|r| r == server::CLOSED_GAME).map(|_| fail(config, "game_paused", &[]))
    })
}

pub fn start_failure(config: &Config, afk: bool, error: StartError) -> Failure {
    let log = server::Files::new(config).log().display().to_string();
    if let Some(failure) = paused_failure(config, afk) {
        return failure;
    }
    match error {
        StartError::Failed(detail) => fail(config, "server_failed", &[("detail", &detail), ("log", &log)]),
        StartError::Game => fail(config, "game_paused", &[]),
        StartError::Timeout => {
            let seconds = config.server.start_timeout_seconds.to_string();
            fail(config, "server_timeout", &[("seconds", &seconds), ("log", &log)])
        }
    }
}

pub fn request_failure(config: &Config, afk: bool, detail: String) -> Failure {
    if let Some(failure) = paused_failure(config, afk) {
        return failure;
    }
    let log = server::Files::new(config).log().display().to_string();
    fail(config, "request_failed", &[("detail", &detail), ("log", &log)])
}

fn start(config: &Config, config_path: &Path, model: &Model) -> Starter {
    let (config, path, model) = (config.clone(), config_path.to_path_buf(), model.clone());
    std::thread::spawn(move || {
        let begun = Instant::now();
        (server::ensure(&config, &path, &model), begun.elapsed().as_secs_f64())
    })
}

fn started(config: &Config, job: &Job, starter: Starter) -> Result<f64, Failure> {
    let (result, seconds) = starter
        .join()
        .map_err(|_| fail(config, "server_failed", &[("detail", "start thread crashed"), ("log", "")]))?;
    result.map_err(|e| start_failure(config, job.afk, e))?;
    Ok(seconds)
}

fn ask(config: &Config, job: &Job, body: &Value) -> Result<(chat::Answer, f64), Failure> {
    let begun = Instant::now();
    let answer = {
        let _busy = Busy::mark(config);
        chat::ask(config, body)
    };
    let seconds = begun.elapsed().as_secs_f64();
    let answer = answer.map_err(|detail| request_failure(config, job.afk, detail))?;
    if answer.text.trim().is_empty() {
        return Err(fail(config, "empty_answer", &[]));
    }
    Ok((answer, seconds))
}

fn tidy(text: &str) -> String {
    repair::compact(text).or_else(|| repair::close_cut(text)).unwrap_or_else(|| text.to_string())
}

#[derive(Default)]
struct Tally {
    prompt_tokens: u64,
    answer_tokens: u64,
    prompt_seconds: f64,
    answer_seconds: f64,
    scan: f64,
    frames: f64,
    sound: f64,
    model: f64,
    finish: String,
    times: Vec<f64>,
    small: usize,
}

impl Tally {
    fn add(&mut self, answer: &chat::Answer, model_seconds: f64) {
        self.prompt_tokens += answer.prompt_tokens;
        self.answer_tokens += answer.answer_tokens;
        if answer.prompt_per_second > 0.0 {
            self.prompt_seconds += answer.prompt_tokens as f64 / answer.prompt_per_second;
        }
        if answer.answer_per_second > 0.0 {
            self.answer_seconds += answer.answer_tokens as f64 / answer.answer_per_second;
        }
        self.model += model_seconds;
        if self.finish.is_empty() || self.finish == chat::FINISH_DONE {
            self.finish = answer.finish.clone();
        }
    }

    fn pictures(&mut self, pictures: &Pictures) {
        self.scan += pictures.scan_seconds;
        self.frames += pictures.frames_seconds;
        self.sound += pictures.sound_seconds;
        self.times.extend(pictures.list.iter().map(|p| p.at));
        self.small += pictures.list.iter().filter(|p| p.small).count();
    }

    fn rate(tokens: u64, seconds: f64) -> f64 {
        if seconds > 0.0 { tokens as f64 / seconds } else { 0.0 }
    }
}

fn in_order(schema: &Value, mut value: Map<String, Value>) -> Map<String, Value> {
    let mut out = Map::new();
    for name in schema.get("properties").and_then(Value::as_object).into_iter().flat_map(|p| p.keys()) {
        if let Some(found) = value.remove(name) {
            out.insert(name.clone(), found);
        }
    }
    out.extend(value);
    out
}

pub enum Stage<'a> {
    Hearing,
    Part(usize, usize),
    Skipped(usize, usize, &'a str),
}

pub fn watch(config: &Config, config_path: &Path, job: &Job) -> Result<Reply, Failure> {
    watch_told(config, config_path, job, &mut |_| true)
}

pub fn watch_told(
    config: &Config,
    config_path: &Path,
    job: &Job,
    tell: &mut dyn FnMut(Stage) -> bool,
) -> Result<Reply, Failure> {
    let total = Instant::now();
    let model = check(config, job)?;
    server::set_afk(config, job.afk);
    let (info, end) = window(config, job)?;
    if ear::needed(&config.ear, job, end) && !tell(Stage::Hearing) {
        return Err(fail(config, "stopped", &[]));
    }
    let (heard, ear_seconds) = ear::fill(config, job, end);
    let job = heard.as_ref().unwrap_or(job);
    let hears = model.hears && config.audio.enabled;
    let ranges = long::parts(job.start, end, &config.long);
    if ranges.len() > 1 {
        let long = Long { job, model: &model, info: &info, ranges, hears };
        let mut reply = watch_long(config, config_path, long, total, tell)?;
        reply.seconds.ear = ear_seconds;
        return Ok(reply);
    }
    let plan = plan(config, job, &info, end, hears)?;
    let starter = start(config, config_path, &model);
    let pictures = pictures(config, job, &plan)?;
    let server_seconds = started(config, job, starter)?;
    let mut body = chat::body(config, &model.name, &plan.tail, &pictures.list, &job.schema);
    if let Some(sound) = &pictures.sound {
        chat::add_sound(config, &mut body, sound);
    }
    if let Some(top) = job.probs {
        chat::add_probs(&mut body, top);
    }
    let (answer, model_seconds) = ask(config, job, &body)?;
    Ok(Reply {
        ok: true,
        text: tidy(&answer.text),
        finish: answer.finish,
        model: model.name,
        frames: pictures.list.iter().map(|p| p.at).collect(),
        small_frames: pictures.list.iter().filter(|p| p.small).count(),
        parts: 1,
        prompt_tokens: answer.prompt_tokens,
        answer_tokens: answer.answer_tokens,
        prompt_per_second: answer.prompt_per_second,
        answer_per_second: answer.answer_per_second,
        seconds: Seconds {
            scan: pictures.scan_seconds,
            frames: pictures.frames_seconds,
            sound: pictures.sound_seconds,
            ear: ear_seconds,
            server: server_seconds,
            model: model_seconds,
            total: total.elapsed().as_secs_f64(),
        },
        probs: answer.probs,
    })
}

pub fn run(config: &Config, config_path: &Path) -> i32 {
    let mut raw = String::new();
    let result = match std::io::stdin().read_to_string(&mut raw) {
        Ok(_) => Job::parse(config, &raw).and_then(|job| watch(config, config_path, &job)),
        Err(e) => Err(fail(config, "bad_job", &[("detail", &e.to_string())])),
    };
    match result {
        Ok(reply) => {
            println!("{}", serde_json::to_string(&reply).unwrap_or_default());
            0
        }
        Err(failure) => {
            println!("{}", serde_json::to_string(&failure).unwrap_or_default());
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::start_failure;
    use crate::config::Config;
    use crate::server::StartError;

    #[test]
    fn a_start_refused_for_a_game_says_paused_instead_of_failed() {
        let dir = std::env::temp_dir().join(format!("pupil-game-start-{}", std::process::id()));
        let mut config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
        config.paths.run_dir = dir.clone();
        config.game.enabled = false;
        let failure = start_failure(&config, false, StartError::Game);
        assert_eq!(failure.error, "game_paused");
        assert!(!failure.message.is_empty());
        assert!(!failure.hint.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
