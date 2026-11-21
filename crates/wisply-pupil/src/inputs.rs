use std::time::Instant;

use crate::cache;
use crate::config::Config;
use crate::grab;
use crate::job::{Failure, Job};
use crate::pick::{PickSettings, pick};
use crate::timecode;
use crate::video;
use crate::watch::Plan;

fn fail(config: &Config, code: &str, fields: &[(&str, &str)]) -> Failure {
    Failure::new(config, code, fields)
}

pub(crate) struct Pictures {
    pub(crate) list: Vec<video::Picture>,
    pub(crate) sound: Option<Vec<u8>>,
    pub(crate) scan_seconds: f64,
    pub(crate) frames_seconds: f64,
    pub(crate) sound_seconds: f64,
}

pub(crate) fn cache_key(config: &Config, job: &Job, plan: &Plan) -> Option<String> {
    if !config.cache.enabled {
        return None;
    }
    let parts = format!("{:?}|{}|{}|{:?}|{:?}|{}", config.frames, job.start, plan.end, plan.size, plan.small, plan.cap);
    cache::key(&job.video, &parts)
}

pub(crate) fn pictures(config: &Config, job: &Job, plan: &Plan) -> Result<Pictures, Failure> {
    let key = cache_key(config, job, plan);
    let load_begun = Instant::now();
    if let Some(list) = key.as_deref().and_then(|k| cache::load(&config.cache, k)) {
        let loaded = load_begun.elapsed().as_secs_f64();
        return Ok(with_sound(config, job, plan, list, 0.0, loaded));
    }
    let frames = &config.frames;
    let (start, end) = (timecode::format(job.start), timecode::format(plan.end));
    let none = || fail(config, "no_frames", &[("start", &start), ("end", &end)]);
    let scan_begun = Instant::now();
    let scanned = video::scan(frames, &job.video, job.start, plan.end)
        .map_err(|e| fail(config, "ffmpeg_failed", &[("detail", &e)]))?;
    let settings = PickSettings {
        change: frames.change,
        pixel_delta: frames.pixel_delta,
        settle_steps: frames.settle_steps,
        min_gap: frames.min_gap_seconds,
        max_gap: frames.max_gap_seconds,
        max_frames: plan.cap,
    };
    let shots: Vec<video::Shot> = pick(&scanned, &settings)
        .into_iter()
        .map(|p| video::Shot {
            at: p.at,
            size: if p.key { plan.size } else { plan.small },
            small: !p.key && plan.small != plan.size,
        })
        .collect();
    let scan_seconds = scan_begun.elapsed().as_secs_f64();
    if shots.is_empty() {
        return Err(none());
    }
    let frames_begun = Instant::now();
    let list =
        grab::extract(frames, &job.video, &shots).map_err(|e| fail(config, "ffmpeg_failed", &[("detail", &e)]))?;
    if list.is_empty() {
        return Err(none());
    }
    let frames_seconds = frames_begun.elapsed().as_secs_f64();
    if let Some(k) = key.as_deref() {
        cache::save(&config.cache, k, &list);
    }
    Ok(with_sound(config, job, plan, list, scan_seconds, frames_seconds))
}

fn with_sound(
    config: &Config,
    job: &Job,
    plan: &Plan,
    list: Vec<video::Picture>,
    scan_seconds: f64,
    frames_seconds: f64,
) -> Pictures {
    let sound_begun = Instant::now();
    let source = job.audio.as_deref().unwrap_or(&job.video);
    let sound = plan.hears.then(|| video::sound(&config.frames, &config.audio, source, job.start, plan.end)).flatten();
    Pictures { list, sound, scan_seconds, frames_seconds, sound_seconds: sound_begun.elapsed().as_secs_f64() }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::json;

    use super::cache_key;
    use crate::config::Config;
    use crate::job::Job;
    use crate::watch::Plan;

    fn plan(cap: usize) -> Plan {
        Plan { end: 30.0, size: (384, 384), small: (192, 192), cap, tail: String::new(), hears: false }
    }

    #[test]
    fn the_key_needs_the_switch_and_follows_the_picture_settings() {
        let dir = std::env::temp_dir().join(format!("pupil-inputs-{}", std::process::id()));
        fs::create_dir_all(&dir).expect("dir");
        let video = dir.join("clip.mp4");
        fs::write(&video, b"clip").expect("write");
        let job: Job = serde_json::from_value(json!({
            "video": video,
            "prompt": "q",
            "schema": {}
        }))
        .expect("job");
        let mut config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
        config.cache.enabled = false;
        assert!(cache_key(&config, &job, &plan(8)).is_none());
        config.cache.enabled = true;
        let first = cache_key(&config, &job, &plan(8)).expect("key");
        assert_eq!(cache_key(&config, &job, &plan(8)), Some(first.clone()));
        assert_ne!(cache_key(&config, &job, &plan(9)), Some(first.clone()));
        config.frames.max_gap_seconds += 1.0;
        assert_ne!(cache_key(&config, &job, &plan(8)), Some(first));
        let _ = fs::remove_dir_all(&dir);
    }
}
