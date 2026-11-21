use std::path::PathBuf;

use serde::Deserialize;

use crate::media::Kind;

#[derive(Debug, Clone, Deserialize)]
pub struct TypeRule {
    pub starts: String,
    pub kind: Kind,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Talk {
    pub max_tokens: u32,
    pub system: String,
    pub dry_multiplier: Option<f64>,
    pub think_fields: serde_json::Map<String, serde_json::Value>,
    pub ear: bool,
    pub history_share: f64,
    pub heard_tokens_per_minute: f64,
    pub heard_share: f64,
    pub share_server: bool,
    pub picture_long_side: u32,
    pub picture_min_long_side: u32,
    pub matte: String,
    pub still_args: Vec<String>,
    pub pictures: Vec<String>,
    pub pages: Vec<String>,
    pub videos: Vec<String>,
    pub sounds: Vec<String>,
    pub texts: Vec<String>,
    pub links: Vec<String>,
    pub video_hosts: Vec<String>,
    pub types: Vec<TypeRule>,
    pub temp_name: String,
    pub curl: String,
    pub curl_args: Vec<String>,
    pub ytdlp: String,
    pub ytdlp_args: Vec<String>,
    pub ytdlp_tries: u32,
    pub about_file: String,
    pub partial_suffixes: Vec<String>,
    pub about_max_chars: usize,
    pub download_max_mb: u64,
    pub download_seconds: u64,
    pub user_agent: String,
    pub text_max_chars: usize,
    pub page_drop: Vec<String>,
    pub page_tag: String,
    pub page_entities: Vec<Vec<String>>,
    pub hear_args: Vec<String>,
    pub dictate_args: Vec<String>,
    pub status_file: PathBuf,
    pub status_run: String,
    pub status_stale_seconds: f64,
    pub calm_labels: Vec<String>,
    pub hold_poll_seconds: f64,
    pub hold_files: Vec<PathBuf>,
    pub calm_owners: Vec<String>,
    pub warm: Vec<PathBuf>,
    pub page_host: String,
    pub page_port: u16,
    pub page_open: Vec<String>,
    pub page_dir: PathBuf,
    pub page_state: String,
    pub page_idle_seconds: f64,
    pub page_poll_ms: u64,
    pub page_read_seconds: u64,
    pub page_head_bytes: u64,
    pub page_body_bytes: u64,
    pub page_sessions: usize,
    pub page_history: usize,
    pub upload_max_mb: u64,
    pub answer_key: String,
    pub answer_note: String,
    pub new_words: Vec<String>,
    pub quit_words: Vec<String>,
    pub help_words: Vec<String>,
    pub text: TalkText,
    pub loops: Loops,
    pub trace: Trace,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Trace {
    pub dir: PathBuf,
    pub frames: bool,
    pub media_max_mb: u64,
    pub marker: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Loops {
    pub repeats: usize,
    pub min_chars: usize,
    pub unit_chars: usize,
    pub finish: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct TalkText {
    pub prompt: String,
    pub hello: String,
    pub help: String,
    pub fresh: String,
    pub bye: String,
    pub hold: String,
    pub working: String,
    pub hearing: String,
    pub part: String,
    pub part_skipped: String,
    pub stats: String,
    pub error: String,
    pub question: String,
    pub picture: String,
    pub video: String,
    pub sound: String,
    pub file: String,
    pub page: String,
    pub sound_header: String,
    pub sound_line: String,
    pub no_sound: String,
    pub cut: String,
    pub looped: String,
    pub about: String,
    pub page_ready: String,
    pub page_again: String,
    pub page_locked: String,
    pub loading: String,
    pub auto_on: String,
    pub auto_off: String,
    pub auto_broken: String,
}
