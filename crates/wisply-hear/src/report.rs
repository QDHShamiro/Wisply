use serde::Serialize;
use serde_json::{Value, json};

use crate::events::Event;
use crate::speech::Segment;

#[derive(Debug, Default, Serialize)]
pub struct Timing {
    pub audio_seconds: f64,
    pub decode_seconds: f64,
    pub speech_seconds: Option<f64>,
    pub speech_seconds_per_minute: Option<f64>,
    pub sound_seconds: Option<f64>,
    pub sound_seconds_per_minute: Option<f64>,
    pub speech_peak_ram_mb: Option<f64>,
    pub sound_peak_ram_mb: Option<f64>,
    pub gpu_baseline_mb: Option<f64>,
    pub gpu_peak_mb: Option<f64>,
}

pub fn per_minute(seconds: f64, audio_seconds: f64) -> Option<f64> {
    (audio_seconds > 0.0).then(|| seconds / (audio_seconds / 60.0))
}

fn round(value: f64, decimals: u32) -> f64 {
    let scale = 10f64.powi(decimals as i32);
    (value * scale).round() / scale
}

fn round_number(value: &mut Value, decimals: u32) {
    match value {
        Value::Number(n) => {
            let rounded = n.as_f64().and_then(|f| serde_json::Number::from_f64(round(f, decimals)));
            if let Some(rounded) = rounded {
                *value = Value::Number(rounded);
            }
        }
        Value::Object(map) => map.values_mut().for_each(|v| round_number(v, decimals)),
        _ => {}
    }
}

pub fn build(
    video: &str,
    start: Option<f64>,
    end: Option<f64>,
    speech: Option<&[Segment]>,
    sounds: Option<&[Event]>,
    timing: Option<&Timing>,
    decimals: u32,
) -> Value {
    let mut out = json!({
        "video": video,
        "start": start.unwrap_or(0.0),
        "end": end,
    });
    if let Some(speech) = speech {
        out["speech"] = serde_json::to_value(speech).unwrap_or(Value::Null);
    }
    if let Some(sounds) = sounds {
        out["sounds"] = serde_json::to_value(sounds).unwrap_or(Value::Null);
    }
    if let Some(timing) = timing {
        out["timing"] = serde_json::to_value(timing).unwrap_or(Value::Null);
    }
    for key in ["speech", "sounds"] {
        if let Some(Value::Array(items)) = out.get_mut(key) {
            for item in items.iter_mut() {
                for field in ["start", "end"] {
                    if let Some(v) = item.get_mut(field) {
                        round_number(v, decimals);
                    }
                }
            }
        }
    }
    if let Some(v) = out.get_mut("timing") {
        round_number(v, decimals);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn per_minute_scales() {
        assert_eq!(per_minute(30.0, 120.0), Some(15.0));
        assert_eq!(per_minute(5.0, 0.0), None);
    }

    #[test]
    fn build_rounds_times_and_skips_missing_parts() {
        let speech = vec![Segment { start: 1.23456, end: 2.0, text: "hi".into() }];
        let out = build("a.mp4", Some(0.0), None, Some(&speech), None, None, 2);
        assert_eq!(out["speech"][0]["start"], 1.23);
        assert!(out.get("sounds").is_none());
        assert!(out.get("timing").is_none());
        assert_eq!(out["video"], "a.mp4");
    }

    #[test]
    fn build_includes_sounds_and_timing() {
        let sounds = vec![Event { start: 0.0, end: 3.46789, label: "Music".into(), score: 0.9 }];
        let timing = Timing { audio_seconds: 60.0, sound_seconds: Some(1.23456), ..Timing::default() };
        let out = build("a.mp4", None, None, None, Some(&sounds), Some(&timing), 2);
        assert_eq!(out["sounds"][0]["end"], 3.47);
        assert_eq!(out["sounds"][0]["label"], "Music");
        assert_eq!(out["timing"]["sound_seconds"], 1.23);
    }
}
