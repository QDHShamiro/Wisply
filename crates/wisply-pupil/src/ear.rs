use std::path::Path;
use std::time::Instant;

use serde::Deserialize;

use crate::config::{Config, Ear};
use crate::job::{Job, Segment};
use crate::spawn;

#[derive(Deserialize)]
struct Heard {
    #[serde(default)]
    speech: Vec<Segment>,
}

pub fn parse(raw: &[u8]) -> Result<Vec<Segment>, String> {
    let heard: Heard = serde_json::from_slice(raw).map_err(|e| e.to_string())?;
    Ok(heard.speech.into_iter().filter(|s| !s.text.trim().is_empty()).collect())
}

fn listen(ear: &Ear, source: &Path, start: f64, end: f64) -> Result<Vec<Segment>, String> {
    let mut command = spawn::ear(&ear.exe);
    command
        .arg(source)
        .arg("--start")
        .arg(start.to_string())
        .arg("--end")
        .arg(end.to_string())
        .args(["--only", "speech", "--speech"])
        .arg(&ear.speech)
        .arg("--config")
        .arg(&ear.config);
    let out = command.output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(String::from_utf8_lossy(&out.stderr).trim().to_string());
    }
    parse(&out.stdout)
}

pub fn needed(ear: &Ear, job: &Job, end: f64) -> bool {
    ear.enabled && !job.transcript.iter().any(|s| s.end > job.start && s.start < end && !s.text.trim().is_empty())
}

pub fn fill(config: &Config, job: &Job, end: f64) -> (Option<Job>, f64) {
    if !needed(&config.ear, job, end) {
        return (None, 0.0);
    }
    let begun = Instant::now();
    let source = job.audio.as_deref().unwrap_or(&job.video);
    let heard = listen(&config.ear, source, job.start, end);
    let seconds = begun.elapsed().as_secs_f64();
    match heard {
        Ok(segments) if !segments.is_empty() => {
            let mut copy = job.clone();
            copy.transcript = segments;
            (Some(copy), seconds)
        }
        Ok(_) => (None, seconds),
        Err(detail) => {
            if let Some(line) = config.messages.get("ear_failed") {
                eprintln!("{}", line.replace("{detail}", &detail));
            }
            (None, seconds)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{needed, parse};
    use crate::config::Ear;
    use crate::job::Job;
    use serde_json::json;

    fn job(transcript: serde_json::Value) -> Job {
        serde_json::from_value(json!({
            "video": "v.mp4", "start": 60.0, "prompt": "p", "schema": {},
            "transcript": transcript
        }))
        .expect("job")
    }

    fn ear(enabled: bool) -> Ear {
        Ear { enabled, ..Ear::default() }
    }

    #[test]
    fn heard_speech_keeps_times_and_drops_empty_lines() {
        let raw = br#"{"video": "a.mp4", "start": 60, "speech": [
            {"start": 61.5, "end": 64.0, "text": " Hello there."},
            {"start": 64.0, "end": 65.0, "text": "   "}
        ], "timing": {"audio_seconds": 5}}"#;
        let got = parse(raw).expect("parses");
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].start, 61.5);
        assert_eq!(got[0].text, " Hello there.");
        assert!(parse(br#"{"video": "a.mp4"}"#).expect("parses").is_empty());
        assert!(parse(b"not json").is_err());
    }

    #[test]
    fn the_ear_listens_only_when_the_part_has_no_words() {
        let none = job(json!([]));
        let before = job(json!([{"start": 10.0, "end": 20.0, "text": "old"}]));
        let blank = job(json!([{"start": 70.0, "end": 80.0, "text": " "}]));
        let said = job(json!([{"start": 70.0, "end": 80.0, "text": "words"}]));
        assert!(needed(&ear(true), &none, 120.0));
        assert!(needed(&ear(true), &before, 120.0));
        assert!(needed(&ear(true), &blank, 120.0));
        assert!(!needed(&ear(true), &said, 120.0));
        assert!(!needed(&ear(false), &none, 120.0));
    }
}
