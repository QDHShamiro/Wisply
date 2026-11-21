use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub paths: Paths,
    pub audio: Audio,
    pub speech: Speech,
    pub sound: Sound,
    pub mel: Mel,
    pub gpu: Gpu,
    pub output: Output,
    pub messages: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Paths {
    pub ffmpeg: String,
    pub work_dir: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Audio {
    pub sample_rate: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Speech {
    pub enabled: bool,
    pub exe: PathBuf,
    pub default: String,
    pub language: String,
    pub threads: usize,
    pub extra_args: Vec<String>,
    pub noise_brackets: Vec<[String; 2]>,
    pub min_seconds: f64,
    pub models: Vec<SpeechModel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpeechModel {
    pub name: String,
    pub file: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sound {
    pub enabled: bool,
    pub runtime: PathBuf,
    pub default: String,
    pub threads: usize,
    pub window_seconds: f64,
    pub hop_seconds: f64,
    pub threshold: f32,
    pub min_seconds: f64,
    pub merge_gap_seconds: f64,
    pub max_labels_per_slot: usize,
    pub score_decimals: u32,
    pub drop_labels: Vec<String>,
    pub models: Vec<SoundModel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SoundModel {
    pub name: String,
    pub file: PathBuf,
    pub labels: PathBuf,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Mel {
    pub n_fft: usize,
    pub hop: usize,
    pub n_mels: usize,
    pub f_min: f32,
    pub f_max: f32,
    pub amin: f32,
    pub top_db: f32,
    pub frame_pad: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Gpu {
    pub query: Vec<String>,
    pub poll_ms: u64,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Output {
    pub pretty: bool,
    pub time_decimals: u32,
    pub timing: bool,
}

impl Config {
    pub fn load(path: &Path) -> Result<Config, String> {
        let raw = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        toml::from_str(&raw).map_err(|e| format!("{}: {e}", path.display()))
    }

    pub fn message(&self, key: &str, fields: &[(&str, &str)]) -> String {
        let mut text = self.messages.get(key).cloned().unwrap_or_else(|| key.to_string());
        for (name, value) in fields {
            text = text.replace(&format!("{{{name}}}"), value);
        }
        text
    }

    pub fn speech_model(&self, name: Option<&str>) -> Result<&SpeechModel, String> {
        let wanted = name.unwrap_or(&self.speech.default);
        self.speech.models.iter().find(|m| m.name == wanted).ok_or_else(|| {
            let names = self.speech.models.iter().map(|m| m.name.as_str()).collect::<Vec<_>>().join(", ");
            self.message("model_unknown", &[("name", wanted), ("names", &names)])
        })
    }

    pub fn sound_model(&self, name: Option<&str>) -> Result<&SoundModel, String> {
        let wanted = name.unwrap_or(&self.sound.default);
        self.sound.models.iter().find(|m| m.name == wanted).ok_or_else(|| {
            let names = self.sound.models.iter().map(|m| m.name.as_str()).collect::<Vec<_>>().join(", ");
            self.message("model_unknown", &[("name", wanted), ("names", &names)])
        })
    }
}
