use std::path::Path;
use std::sync::mpsc;
use std::time::Instant;

use serde_json::{Map, Value};

use crate::chat;
use crate::config::{Config, Model};
use crate::inputs::{Pictures, pictures};
use crate::job::{Failure, Job, Reply, Seconds};
use crate::long;
use crate::video;

use super::{Plan, Stage, Starter, Tally, ask, fail, in_order, plan, start, started, tidy};

fn prepare(config: &Config, part: &Job, info: &video::Info, hears: bool) -> Result<(Plan, Pictures), Failure> {
    let end = part.end.unwrap_or(info.duration);
    let plan = plan(config, part, info, end, hears)?;
    let pictures = pictures(config, part, &plan)?;
    Ok((plan, pictures))
}

pub(super) struct Long<'a> {
    pub(super) job: &'a Job,
    pub(super) model: &'a Model,
    pub(super) info: &'a video::Info,
    pub(super) ranges: Vec<(f64, f64)>,
    pub(super) hears: bool,
}

struct Done {
    answers: Vec<Value>,
    tally: Tally,
    server_seconds: f64,
}

fn part_jobs(config: &Config, job: &Job, ranges: &[(f64, f64)]) -> Vec<Job> {
    let count = ranges.len();
    ranges
        .iter()
        .enumerate()
        .map(|(i, (from, to))| Job {
            start: *from,
            end: Some(*to),
            schema: long::part_schema(&job.schema, &config.text, &config.long, i, count),
            max_frames: long::part_frames(&config.long, i, count),
            ..job.clone()
        })
        .collect()
}

fn ask_part(
    config: &Config,
    long: &Long,
    part: &Job,
    ready: &(Plan, Pictures),
    done: &mut Done,
) -> Result<Value, Failure> {
    let (plan, pictures) = ready;
    let mut body = chat::body(config, &long.model.name, &plan.tail, &pictures.list, &part.schema);
    if let Some(sound) = &pictures.sound {
        chat::add_sound(config, &mut body, sound);
    }
    let (answer, seconds) = ask(config, long.job, &body)?;
    done.tally.add(&answer, seconds);
    Ok(serde_json::from_str(&tidy(&answer.text)).unwrap_or(Value::Null))
}

fn run_parts(
    config: &Config,
    config_path: &Path,
    long: &Long,
    parts: &[Job],
    starter: Starter,
    tell: &mut dyn FnMut(Stage) -> bool,
) -> Result<Done, Failure> {
    let mut done = Done { answers: Vec::with_capacity(parts.len()), tally: Tally::default(), server_seconds: 0.0 };
    let (skip, count) = (config.long.skip_failed, parts.len());
    std::thread::scope(|scope| -> Result<(), Failure> {
        let (send, receive) = mpsc::sync_channel(config.long.ahead.max(1));
        let (info, hears) = (long.info, long.hears);
        scope.spawn(move || {
            for part in parts {
                let made = prepare(config, part, info, hears);
                let stop = made.is_err() && !skip;
                if send.send(made).is_err() || stop {
                    break;
                }
            }
        });
        let (mut starter, mut last) = (Some(starter), None);
        for (at, (part, made)) in parts.iter().zip(receive.iter()).enumerate() {
            if !tell(Stage::Part(at + 1, count)) {
                return Err(fail(config, "stopped", &[]));
            }
            let value = match made {
                Ok(ready) => {
                    if let Some(handle) = starter.take() {
                        done.server_seconds = started(config, long.job, handle)?;
                    }
                    done.tally.pictures(&ready.1);
                    match ask_part(config, long, part, &ready, &mut done) {
                        Err(_) if skip => started(config, long.job, start(config, config_path, long.model))
                            .and_then(|_| ask_part(config, long, part, &ready, &mut done)),
                        other => other,
                    }
                }
                Err(failure) => Err(failure),
            };
            match value {
                Ok(value) => done.answers.push(value),
                Err(failure) if skip => {
                    done.answers.push(Value::Null);
                    if !tell(Stage::Skipped(at + 1, count, &failure.message)) {
                        return Err(fail(config, "stopped", &[]));
                    }
                    last = Some(failure);
                }
                Err(failure) => return Err(failure),
            }
        }
        match last {
            Some(failure) if done.answers.iter().all(Value::is_null) => Err(failure),
            _ => Ok(()),
        }
    })?;
    Ok(done)
}

fn join_parts(config: &Config, long: &Long, done: &mut Done) -> Result<Map<String, Value>, Failure> {
    let job = long.job;
    let merged = long::merge(&job.schema, &config.text, &done.answers);
    let mut value = merged.value;
    if merged.open.is_empty() {
        return Ok(in_order(&job.schema, value));
    }
    let schema = long::open_schema(&job.schema, &config.text, &merged.open);
    let notes = long::notes(&config.long, &long.ranges, &done.answers, &merged.open);
    let tail = long::merge_tail(config, &notes, &job.prompt, &schema);
    let body = chat::body(config, &long.model.name, &tail, &[], &schema);
    let (answer, seconds) = ask(config, job, &body)?;
    done.tally.add(&answer, seconds);
    let joined: Value = serde_json::from_str(&tidy(&answer.text)).unwrap_or(Value::Null);
    for name in &merged.open {
        let found = joined.get(name).or_else(|| done.answers.iter().find_map(|a| a.get(name)));
        if let Some(found) = found {
            value.insert(name.clone(), found.clone());
        }
    }
    Ok(in_order(&job.schema, value))
}

pub(super) fn watch_long(
    config: &Config,
    config_path: &Path,
    long: Long,
    total: Instant,
    tell: &mut dyn FnMut(Stage) -> bool,
) -> Result<Reply, Failure> {
    let parts = part_jobs(config, long.job, &long.ranges);
    let starter = start(config, config_path, long.model);
    let mut done = run_parts(config, config_path, &long, &parts, starter, tell)?;
    if done.answers.iter().all(Value::is_null) {
        return Err(fail(config, "empty_answer", &[]));
    }
    let value = join_parts(config, &long, &mut done)?;
    let tally = done.tally;
    Ok(Reply {
        ok: true,
        text: Value::Object(value).to_string(),
        finish: tally.finish,
        model: long.model.name.clone(),
        frames: tally.times,
        small_frames: tally.small,
        parts: parts.len(),
        prompt_tokens: tally.prompt_tokens,
        answer_tokens: tally.answer_tokens,
        prompt_per_second: Tally::rate(tally.prompt_tokens, tally.prompt_seconds),
        answer_per_second: Tally::rate(tally.answer_tokens, tally.answer_seconds),
        seconds: Seconds {
            scan: tally.scan,
            frames: tally.frames,
            sound: tally.sound,
            ear: 0.0,
            server: done.server_seconds,
            model: tally.model,
            total: total.elapsed().as_secs_f64(),
        },
        probs: None,
    })
}
