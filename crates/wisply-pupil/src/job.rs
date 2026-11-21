use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::Config;

#[derive(Debug, Clone, Deserialize)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Job {
    pub video: PathBuf,
    #[serde(default)]
    pub start: f64,
    #[serde(default)]
    pub end: Option<f64>,
    pub prompt: String,
    pub schema: Value,
    #[serde(default)]
    pub transcript: Vec<Segment>,
    #[serde(default)]
    pub audio: Option<PathBuf>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub afk: bool,
    #[serde(default)]
    pub max_frames: Option<usize>,
    #[serde(default)]
    pub probs: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Seconds {
    pub scan: f64,
    pub frames: f64,
    pub sound: f64,
    pub ear: f64,
    pub server: f64,
    pub model: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Reply {
    pub ok: bool,
    pub text: String,
    pub finish: String,
    pub model: String,
    pub frames: Vec<f64>,
    pub small_frames: usize,
    pub parts: usize,
    pub prompt_tokens: u64,
    pub answer_tokens: u64,
    pub prompt_per_second: f64,
    pub answer_per_second: f64,
    pub seconds: Seconds,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub probs: Option<Value>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Failure {
    pub ok: bool,
    pub error: String,
    pub message: String,
    pub hint: String,
}

impl Failure {
    pub fn new(config: &Config, code: &str, fields: &[(&str, &str)]) -> Failure {
        Failure {
            ok: false,
            error: code.to_string(),
            message: config.message(code, fields),
            hint: config.message(&format!("{code}_hint"), fields),
        }
    }
}

impl Job {
    pub fn parse(config: &Config, raw: &str) -> Result<Job, Failure> {
        let job: Job =
            serde_json::from_str(raw).map_err(|e| Failure::new(config, "bad_job", &[("detail", &e.to_string())]))?;
        let bad = |detail: &str| Failure::new(config, "bad_job", &[("detail", detail)]);
        if !job.start.is_finite() || job.start < 0.0 {
            return Err(bad("start has to be a time of zero or more seconds"));
        }
        if let Some(end) = job.end
            && (!end.is_finite() || end <= job.start)
        {
            return Err(bad("end has to come after start"));
        }
        if job.prompt.trim().is_empty() {
            return Err(bad("prompt is empty"));
        }
        if !job.schema.is_object() {
            return Err(bad("schema has to be a JSON object"));
        }
        Ok(job)
    }
}
