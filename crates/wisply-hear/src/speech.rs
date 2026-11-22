use std::path::{Path, PathBuf};
use std::process::Stdio;

use serde::{Deserialize, Serialize};

use crate::audio;
use crate::config::Config;
use crate::spawn;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[derive(Debug, Default)]
pub struct Run {
    pub segments: Vec<Segment>,
    pub peak_ram_mb: Option<f64>,
}

#[derive(Deserialize)]
struct Raw {
    transcription: Vec<RawSegment>,
}

#[derive(Deserialize)]
struct RawSegment {
    offsets: Offsets,
    text: String,
}

#[derive(Deserialize)]
struct Offsets {
    from: i64,
    to: i64,
}

fn is_noise(text: &str, brackets: &[[String; 2]]) -> bool {
    brackets.iter().any(|pair| {
        text.starts_with(&pair[0]) && text.ends_with(&pair[1]) && text.len() >= pair[0].len() + pair[1].len()
    })
}

pub fn read_text(path: &std::path::Path) -> std::io::Result<String> {
    let bytes = std::fs::read(path)?;
    Ok(String::from_utf8_lossy(&bytes).replace(char::REPLACEMENT_CHARACTER, ""))
}

pub fn parse_segments(
    raw: &str,
    brackets: &[[String; 2]],
    min_seconds: f64,
    offset: f64,
) -> Result<Vec<Segment>, String> {
    let parsed: Raw = serde_json::from_str(raw).map_err(|e| e.to_string())?;
    Ok(parsed
        .transcription
        .into_iter()
        .filter_map(|s| {
            let text = s.text.trim().to_string();
            let start = s.offsets.from as f64 / 1000.0 + offset;
            let end = s.offsets.to as f64 / 1000.0 + offset;
            (text.chars().any(char::is_alphanumeric) && !is_noise(&text, brackets) && end - start >= min_seconds)
                .then_some(Segment { start, end, text })
        })
        .collect())
}

pub fn build_args(
    model: &Path,
    wav: &Path,
    base: &Path,
    language: &str,
    threads: usize,
    extra: &[String],
) -> Vec<String> {
    let mut args = vec![
        "-m".to_string(),
        model.display().to_string(),
        "-f".to_string(),
        wav.display().to_string(),
        "-l".to_string(),
        language.to_string(),
        "-t".to_string(),
        threads.to_string(),
        "-oj".to_string(),
        "-of".to_string(),
        base.display().to_string(),
    ];
    args.extend(extra.iter().cloned());
    args
}

struct Files {
    wav: PathBuf,
    base: PathBuf,
    json: PathBuf,
    log: PathBuf,
}

impl Files {
    fn new(work: &Path, tag: &str) -> Files {
        Files {
            wav: work.join(format!("{tag}.wav")),
            base: work.join(tag),
            json: work.join(format!("{tag}.json")),
            log: work.join(format!("{tag}.log")),
        }
    }

    fn clean(&self) {
        for path in [&self.wav, &self.json, &self.log] {
            let _ = std::fs::remove_file(path);
        }
    }
}

fn failure_tail(log: &Path) -> String {
    let text = std::fs::read_to_string(log).unwrap_or_default();
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    lines[lines.len().saturating_sub(3)..].join(" | ")
}

fn transcribe(config: &Config, model: &Path, files: &Files) -> Result<Option<f64>, String> {
    let fail = |detail: String| config.message("speech_failed", &[("detail", &detail)]);
    let log_file = std::fs::File::create(&files.log).map_err(|e| fail(e.to_string()))?;
    let args = build_args(
        model,
        &files.wav,
        &files.base,
        &config.speech.language,
        config.speech.threads,
        &config.speech.extra_args,
    );
    let mut child = spawn::quiet(&config.speech.exe)
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::from(log_file))
        .spawn()
        .map_err(|e| fail(e.to_string()))?;
    let status = child.wait().map_err(|e| fail(e.to_string()))?;
    let peak = spawn::peak_ram_mb(&child);
    if !status.success() {
        let detail = format!("exit {:?}: {}", status.code(), failure_tail(&files.log));
        return Err(fail(detail));
    }
    Ok(peak)
}

pub fn hear(config: &Config, model_name: Option<&str>, samples: &[f32], offset: f64, tag: &str) -> Result<Run, String> {
    let model = config.speech_model(model_name)?;
    if !model.file.exists() {
        let path = model.file.display().to_string();
        return Err(config.message("model_missing", &[("path", &path)]));
    }
    let work = &config.paths.work_dir;
    let fail_dir = |e: std::io::Error| {
        let path = work.display().to_string();
        config.message("work_dir", &[("path", &path), ("detail", &e.to_string())])
    };
    std::fs::create_dir_all(work).map_err(fail_dir)?;
    let files = Files::new(work, tag);
    std::fs::write(&files.wav, audio::wav_bytes(samples, config.audio.sample_rate)).map_err(fail_dir)?;
    let outcome = transcribe(config, &model.file, &files);
    let parsed = outcome.and_then(|peak_ram_mb| {
        let bad = |detail: String| config.message("speech_output", &[("detail", &detail)]);
        let raw = read_text(&files.json).map_err(|e| bad(e.to_string()))?;
        let segments =
            parse_segments(&raw, &config.speech.noise_brackets, config.speech.min_seconds, offset).map_err(bad)?;
        Ok(Run { segments, peak_ram_mb })
    });
    files.clean();
    parsed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_multibyte_text_still_parses_without_replacement_marks() {
        let dir = std::env::temp_dir().join(format!("hear-utf8-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("dir");
        let file = dir.join("split.json");
        let mut raw = br#"{"transcription":[{"offsets":{"from":0,"to":2500},"text":" "#.to_vec();
        raw.extend_from_slice(&[0xE4, 0xBD, 0xA0, 0xE5, 0xA5]);
        raw.extend_from_slice(br#""}]}"#);
        std::fs::write(&file, &raw).expect("write");
        assert!(std::fs::read_to_string(&file).is_err());
        let text = read_text(&file).expect("reads");
        let got = parse_segments(&text, &brackets(), 0.0, 0.0).expect("parses");
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].text, char::from_u32(0x4F60).map(String::from).unwrap_or_default());
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn brackets() -> Vec<[String; 2]> {
        vec![["[".to_string(), "]".to_string()], ["(".to_string(), ")".to_string()]]
    }

    const SAMPLE: &str = r#"{"systeminfo":"x","transcription":[
        {"timestamps":{"from":"00:00:00,000","to":"00:00:02,500"},"offsets":{"from":0,"to":2500},"text":" Hello there."},
        {"timestamps":{"from":"00:00:02,500","to":"00:00:04,000"},"offsets":{"from":2500,"to":4000},"text":" [MUSIC]"},
        {"timestamps":{"from":"00:00:04,000","to":"00:00:04,100"},"offsets":{"from":4000,"to":4100},"text":"   "},
        {"timestamps":{"from":"00:00:04,100","to":"00:00:06,000"},"offsets":{"from":4100,"to":6000},"text":" (applause) thanks"}
    ]}"#;

    #[test]
    fn parses_trims_and_drops_noise() {
        let segments = parse_segments(SAMPLE, &brackets(), 0.0, 0.0).unwrap();
        assert_eq!(segments.len(), 2);
        assert_eq!(segments[0].text, "Hello there.");
        assert_eq!((segments[0].start, segments[0].end), (0.0, 2.5));
        assert_eq!(segments[1].text, "(applause) thanks");
    }

    #[test]
    fn punctuation_only_is_dropped() {
        let raw = r#"{"transcription":[{"offsets":{"from":0,"to":900},"text":" ."}]}"#;
        assert!(parse_segments(raw, &brackets(), 0.0, 0.0).unwrap().is_empty());
    }

    #[test]
    fn applies_offset() {
        let segments = parse_segments(SAMPLE, &brackets(), 0.0, 10.0).unwrap();
        assert_eq!(segments[0].start, 10.0);
        assert_eq!(segments[1].end, 16.0);
    }

    #[test]
    fn min_seconds_filters_short_segments() {
        let segments = parse_segments(SAMPLE, &brackets(), 2.0, 0.0).unwrap();
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text, "Hello there.");
    }

    #[test]
    fn bad_json_is_an_error() {
        assert!(parse_segments("{}", &brackets(), 0.0, 0.0).is_err());
        assert!(parse_segments("nope", &brackets(), 0.0, 0.0).is_err());
    }

    #[test]
    fn args_end_with_extras() {
        let args =
            build_args(Path::new("m.bin"), Path::new("a.wav"), Path::new("out"), "auto", 4, &["-np".to_string()]);
        assert_eq!(args[0], "-m");
        assert_eq!(args.last().unwrap(), "-np");
        let of = args.iter().position(|a| a == "-of").unwrap();
        assert_eq!(args[of + 1], "out");
    }
}
