use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[path = "config_talk.rs"]
mod config_talk;

pub use config_talk::{Loops, Talk, TalkText, Trace};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub paths: Paths,
    pub server: Server,
    pub keeper: Keeper,
    pub models: Models,
    pub frames: Frames,
    pub sampling: Sampling,
    pub budget: Budget,
    pub text: Text,
    pub game: Game,
    pub bench: Bench,
    #[serde(default)]
    pub long: Long,
    #[serde(default)]
    pub audio: Audio,
    #[serde(default)]
    pub ear: Ear,
    #[serde(default)]
    pub write: Write,
    #[serde(default)]
    pub write_eval: WriteEval,
    #[serde(default)]
    pub harness: Harness,
    #[serde(default)]
    pub talk: Talk,
    #[serde(default)]
    pub cache: Cache,
    #[serde(default)]
    pub think_eval: ThinkEval,
    #[serde(default)]
    pub think_auto: ThinkAuto,
    pub messages: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Paths {
    pub run_dir: PathBuf,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Cache {
    pub enabled: bool,
    pub dir: PathBuf,
    pub max_entries: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub exe: PathBuf,
    pub host: String,
    pub port: u16,
    pub context: u32,
    pub gpu_layers: u32,
    pub kv_type: String,
    pub flash_attention: String,
    pub image_max_tokens: u32,
    pub start_timeout_seconds: u64,
    pub request_timeout_seconds: u64,
    pub health_poll_ms: u64,
    pub extra_args: Vec<String>,
    #[serde(default)]
    pub share: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Keeper {
    pub keep_loaded_seconds: u64,
    pub poll_seconds: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Models {
    pub default: String,
    pub list: Vec<Model>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Model {
    pub name: String,
    pub file: PathBuf,
    pub mmproj: PathBuf,
    #[serde(default)]
    pub extra_args: Vec<String>,
    #[serde(default)]
    pub hears: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Audio {
    pub enabled: bool,
    pub sample_rate: u32,
    pub bitrate: String,
    pub format: String,
    pub tokens_per_second: f64,
    pub header: String,
    #[serde(default)]
    pub no_transcript: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Ear {
    pub enabled: bool,
    pub exe: PathBuf,
    pub config: PathBuf,
    pub speech: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Frames {
    pub ffmpeg: String,
    pub ffprobe: String,
    pub skip_frame: String,
    pub scan_fps: f64,
    pub scan_width: u32,
    pub scan_height: u32,
    #[serde(default)]
    pub scan_parts: usize,
    #[serde(default)]
    pub scan_overlap_seconds: f64,
    #[serde(default)]
    pub scan_split_seconds: f64,
    #[serde(default)]
    pub scan_retry_skip: String,
    #[serde(default)]
    pub scan_retry_share: f64,
    pub change: f64,
    pub pixel_delta: u8,
    pub settle_steps: usize,
    pub min_gap_seconds: f64,
    pub max_gap_seconds: f64,
    pub max_frames: usize,
    #[serde(default)]
    pub max_frames_per_minute: f64,
    pub long_side: u32,
    #[serde(default)]
    pub fast_long_side: u32,
    pub patch: u32,
    pub label_tokens: u32,
    pub jpeg_quality: u32,
    pub parallel: usize,
    #[serde(default)]
    pub batch: usize,
    #[serde(default)]
    pub batch_name: String,
    #[serde(default)]
    pub grab_threads: usize,
    #[serde(default)]
    pub stamp_filter: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Sampling {
    pub temperature: f64,
    pub top_p: f64,
    pub top_k: u32,
    pub presence_penalty: f64,
    pub max_tokens: u32,
    pub enable_thinking: bool,
    #[serde(default)]
    pub dry_multiplier: f64,
    #[serde(default)]
    pub dry_base: f64,
    #[serde(default)]
    pub dry_allowed_length: u32,
    #[serde(default)]
    pub dry_penalty_last_n: i64,
    #[serde(default)]
    pub dry_sequence_breakers: Vec<String>,
    #[serde(default)]
    pub compact_grammar: bool,
    #[serde(default)]
    pub seed: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Budget {
    pub chars_per_token: f64,
    pub spare_tokens: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Text {
    pub transcript_header: String,
    pub transcript_line: String,
    pub no_transcript: String,
    pub frames_header: String,
    pub frame_label: String,
    pub answer_rule: String,
    #[serde(default)]
    pub time_key: String,
    #[serde(default)]
    pub time_pattern: String,
    #[serde(default)]
    pub max_items_phrase: String,
    #[serde(default)]
    pub fields_header: String,
    #[serde(default)]
    pub field_line: String,
    #[serde(default)]
    pub field_joiner: String,
    #[serde(default)]
    pub span_rule: String,
    #[serde(default)]
    pub spread_rule: String,
    #[serde(default)]
    pub question_turn: bool,
    #[serde(default)]
    pub seconds_decimals: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Game {
    pub enabled: bool,
    pub minecraft_title: String,
    pub minecraft_exes: Vec<String>,
    pub afk_idle_seconds: f64,
    pub exes: Vec<String>,
    pub exe_suffixes: Vec<String>,
    pub path_markers: Vec<String>,
    pub path_excludes: Vec<String>,
    pub fullscreen_states: Vec<i32>,
    #[serde(default)]
    pub fullscreen_ignore: Vec<String>,
    pub d3d_state: i32,
    pub shell_classes: Vec<String>,
    pub fullscreen_name: String,
    pub minecraft_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bench {
    pub cases: PathBuf,
    pub out_dir: PathBuf,
    pub template: PathBuf,
    pub data_marker: String,
    #[serde(default)]
    pub latest_name: String,
    pub models: Vec<String>,
    pub thumb_long_side: u32,
    pub block_seconds: f64,
    pub max_blocks: usize,
    pub time_slack_seconds: f64,
    pub ground_seconds: f64,
    pub min_word_chars: usize,
    pub stem_chars: usize,
    pub stop_words: Vec<String>,
    pub gpu_query: Vec<String>,
    pub game_error: String,
    pub game_wait_seconds: f64,
    pub game_poll_seconds: f64,
    pub weights: Weights,
    #[serde(default)]
    pub cold_start: bool,
    #[serde(default)]
    pub history_runs: usize,
    #[serde(default)]
    pub reference: Option<PathBuf>,
    #[serde(default)]
    pub measurements: Option<PathBuf>,
    #[serde(default)]
    pub style: Option<PathBuf>,
    #[serde(default)]
    pub style_marker: String,
    #[serde(default)]
    pub card: PathBuf,
    #[serde(default)]
    pub write_tries: u32,
    #[serde(default)]
    pub write_wait_ms: u64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Write {
    pub max_tokens: u32,
    #[serde(default)]
    pub system: String,
    #[serde(default)]
    pub dry_multiplier: Option<f64>,
    #[serde(default)]
    pub wrap_tokens: u32,
    #[serde(default)]
    pub wrap_from_tokens: u32,
    #[serde(default)]
    pub wrap_prompt: String,
    #[serde(default)]
    pub wrap_join: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct WriteEval {
    pub cases: PathBuf,
    pub math: PathBuf,
    pub math_count: usize,
    pub math_prompt: String,
    pub results: PathBuf,
    pub out_dir: PathBuf,
    pub retry_seconds: u64,
    pub retry_limit: u32,
    pub german_words: String,
    pub english_words: String,
    pub warm_prompt: String,
    pub warm_tokens: u32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ThinkEval {
    pub set: PathBuf,
    pub results: PathBuf,
    pub out_dir: PathBuf,
    pub letter_prompt: String,
    pub number_prompt: String,
    pub option_line: String,
    pub letters: String,
    pub letter_pattern: String,
    pub loose_letter_pattern: String,
    pub loose_tail_chars: usize,
    pub max_tokens: u32,
    pub budget_field: String,
    pub retry_seconds: u64,
    pub retry_limit: u32,
    pub game_errors: Vec<String>,
    pub warm_prompt: String,
    pub warm_tokens: u32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct ThinkAuto {
    pub quiet_patterns: Vec<String>,
    pub think_patterns: Vec<String>,
    pub option_patterns: Vec<String>,
    pub number_pattern: String,
    pub min_numbers: usize,
    pub with_options: bool,
    pub think_sees_options: bool,
    pub goal_wrong_rate: f64,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Harness {
    pub git: String,
    pub sample_seconds: f64,
    pub gpu_query: Vec<String>,
    pub processes: Vec<String>,
    pub ram_query: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Long {
    pub enabled: bool,
    pub min_seconds: f64,
    pub part_seconds: f64,
    pub least_items: u64,
    #[serde(default)]
    pub items_total: u64,
    pub ahead: usize,
    #[serde(default)]
    pub frames_total: u64,
    #[serde(default)]
    pub least_frames: u64,
    #[serde(default)]
    pub skip_failed: bool,
    pub notes_header: String,
    pub notes_line: String,
    pub merge_rule: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct Weights {
    pub complete: f64,
    pub times: f64,
    pub order: f64,
    pub spread: f64,
    pub unique: f64,
    pub coverage: f64,
    pub grounded: f64,
}

impl Config {
    pub fn load(path: &Path) -> Result<Config, String> {
        let raw = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        toml::from_str(&raw).map_err(|e| format!("{}: {e}", path.display()))
    }

    pub fn model(&self, name: Option<&str>) -> Option<&Model> {
        let wanted = name.unwrap_or(&self.models.default);
        self.models.list.iter().find(|m| m.name == wanted)
    }

    pub fn model_names(&self) -> String {
        self.models.list.iter().map(|m| m.name.as_str()).collect::<Vec<_>>().join(", ")
    }

    pub fn message(&self, key: &str, fields: &[(&str, &str)]) -> String {
        let mut text = self.messages.get(key).cloned().unwrap_or_else(|| key.to_string());
        for (name, value) in fields {
            text = text.replace(&format!("{{{name}}}"), value);
        }
        text
    }
}
