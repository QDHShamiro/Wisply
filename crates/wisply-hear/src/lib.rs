mod args;
mod audio;
mod config;
mod events;
mod gpu;
mod labels;
mod mel;
mod report;
mod sound;
mod spawn;
mod speech;
mod tagger;

use std::path::{Path, PathBuf};
use std::time::Instant;

use args::Part;
use config::Config;

fn config_path(given: Option<PathBuf>, fallback: &Path) -> PathBuf {
    given.unwrap_or_else(|| fallback.to_path_buf())
}

fn listen(
    config: &Config,
    args: &args::Args,
    samples: &[f32],
    timing: &mut report::Timing,
) -> Result<Vec<speech::Segment>, String> {
    let watch = gpu::start(&config.gpu);
    let started = Instant::now();
    let run = speech::hear(config, args.speech_model.as_deref(), samples, args.start.unwrap_or(0.0), "hear");
    let seconds = started.elapsed().as_secs_f64();
    let (baseline, peak) = watch.finish();
    let run = run?;
    timing.speech_seconds = Some(seconds);
    timing.speech_seconds_per_minute = report::per_minute(seconds, timing.audio_seconds);
    timing.speech_peak_ram_mb = run.peak_ram_mb;
    timing.gpu_baseline_mb = baseline;
    timing.gpu_peak_mb = peak;
    Ok(run.segments)
}

fn tag_sounds(
    config: &Config,
    args: &args::Args,
    samples: &[f32],
    timing: &mut report::Timing,
) -> Result<Vec<events::Event>, String> {
    let started = Instant::now();
    let found = sound::hear(config, args.sound_model.as_deref(), samples, args.start.unwrap_or(0.0))?;
    let seconds = started.elapsed().as_secs_f64();
    timing.sound_seconds = Some(seconds);
    timing.sound_seconds_per_minute = report::per_minute(seconds, timing.audio_seconds);
    timing.sound_peak_ram_mb = spawn::own_peak_ram_mb();
    Ok(found)
}

fn hear(args: &args::Args, config: &Config) -> Result<(), String> {
    if !Path::new(&args.video).exists() {
        let path = args.video.display().to_string();
        return Err(config.message("video_missing", &[("path", &path)]));
    }
    let want_speech = config.speech.enabled && args.only != Some(Part::Sound);
    let want_sound = config.sound.enabled && args.only != Some(Part::Speech);
    let decode_started = Instant::now();
    let samples = audio::decode(config, &args.video, args.start, args.end)?;
    let mut timing = report::Timing {
        decode_seconds: decode_started.elapsed().as_secs_f64(),
        audio_seconds: samples.len() as f64 / config.audio.sample_rate as f64,
        ..report::Timing::default()
    };
    let segments = if want_speech { Some(listen(config, args, &samples, &mut timing)?) } else { None };
    let sounds = if want_sound { Some(tag_sounds(config, args, &samples, &mut timing)?) } else { None };
    let video = args.video.display().to_string();
    let out = report::build(
        &video,
        args.start,
        args.end,
        segments.as_deref(),
        sounds.as_deref(),
        config.output.timing.then_some(&timing),
        config.output.time_decimals,
    );
    let text = if config.output.pretty { serde_json::to_string_pretty(&out) } else { serde_json::to_string(&out) }
        .map_err(|e| e.to_string())?;
    println!("{text}");
    Ok(())
}

fn start(fallback: &Path, raw: &[String]) -> Result<(), String> {
    let args = args::parse(raw.iter().cloned()).map_err(|detail| {
        let config = Config::load(&config_path(None, fallback)).ok();
        match config {
            Some(config) => format!(
                "{}
{}",
                config.message("bad_args", &[("detail", &detail)]),
                config.message("usage", &[])
            ),
            None => detail,
        }
    })?;
    let config =
        Config::load(&config_path(args.config.clone(), fallback)).map_err(|detail| format!("config: {detail}"))?;
    hear(&args, &config)
}

pub fn run(fallback: &Path, raw: &[String]) -> i32 {
    match start(fallback, raw) {
        Ok(()) => 0,
        Err(detail) => {
            eprintln!("{detail}");
            1
        }
    }
}
