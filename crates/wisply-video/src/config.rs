use std::collections::BTreeMap;
use std::io::ErrorKind;
use std::path::Path;

use serde::Deserialize;
use toml::{Table, Value};

use crate::fail::{Fail, Kind};
use crate::schema::Mode;

const DEFAULTS: &str = include_str!("default.toml");
const RESOLUTIONS: [&str; 2] = ["low", "high"];

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub store: Store,
    pub gemini: Gemini,
    pub youtube: YouTube,
    pub transcript: Transcript,
    pub analysis: Analysis,
    pub render: Render,
    pub tokens: Tokens,
    pub watch: Watch,
    pub limits: Limits,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Store {
    pub busy_timeout_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gemini {
    pub model_ladder: Vec<String>,
    pub request_timeout_seconds: f64,
    pub first_answer_seconds: f64,
    pub answer_seconds_per_media_second: f64,
    pub max_hops: usize,
    pub error_text_chars: usize,
    pub retry: Retry,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Retry {
    pub attempts: u32,
    pub tries_per_model: usize,
    pub first_delay_seconds: f64,
    pub max_delay_seconds: f64,
    pub total_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct YouTube {
    pub command: Vec<String>,
    pub timeout_seconds: f64,
    pub download_timeout_seconds: f64,
    pub info_hours: f64,
    pub frame_format: String,
    pub frame_sort: String,
    pub description_chars: usize,
    pub subtitle_format: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transcript {
    pub segment_seconds: f64,
    pub segment_chars: usize,
    pub search_results: usize,
    pub output_chars: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Analysis {
    pub max_steps: usize,
    pub max_points: usize,
    pub max_sounds: usize,
    pub max_list_items: usize,
    pub max_focus_chars: usize,
    pub max_chapters: usize,
    pub max_quiet_spans: usize,
    pub quiet_min_seconds: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Render {
    pub page_chars: usize,
    pub sound_attach_seconds: f64,
    pub unsure_times: usize,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tokens {
    pub video_frame_low: f64,
    pub video_frame_high: f64,
    pub audio_per_second: f64,
    pub text_chars_per_token: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Watch {
    pub agentic_tokens: i64,
    pub coarse_sound_fps: f64,
    pub output_tokens: BTreeMap<String, i64>,
    pub profiles: BTreeMap<String, Profile>,
    pub ladders: BTreeMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub resolution: String,
    pub fps: Option<f64>,
    #[serde(default)]
    pub agentic: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    pub usage_share: f64,
    pub max_wait_seconds: f64,
    pub youtube_seconds_per_day: f64,
    pub rejection_block_seconds: f64,
    pub history_days: f64,
    pub models: BTreeMap<String, ModelLimits>,
    pub groq: GroqLimits,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelLimits {
    pub requests_per_minute: f64,
    pub tokens_per_minute: f64,
    pub requests_per_day: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroqLimits {
    pub requests_per_minute: f64,
    pub requests_per_day: f64,
    pub audio_seconds_per_hour: f64,
    pub audio_seconds_per_day: f64,
}

fn refuse(path: &Path, reason: impl std::fmt::Display) -> Fail {
    let hint = format!("fix {} or delete the line to get the default back", path.display());
    Fail::hint(Kind::Input, format!("the video settings are not usable: {reason}"), hint)
}

fn merge(base: &mut Table, over: Table) {
    for (key, value) in over {
        if let (Some(Value::Table(under)), Value::Table(inner)) = (base.get_mut(&key), &value) {
            merge(under, inner.clone());
        } else {
            base.insert(key, value);
        }
    }
}

fn brief(error: &toml::de::Error) -> String {
    error.message().replace('\n', ", ")
}

fn read(text: &str, path: &Path) -> Result<Table, Fail> {
    text.parse::<Table>().map_err(|error| {
        let before = error.span().and_then(|span| text.get(..span.start)).unwrap_or_default();
        refuse(path, format!("line {}: {}", before.matches('\n').count() + 1, brief(&error)))
    })
}

fn build(table: Table, path: &Path) -> Result<Config, Fail> {
    let config: Config = Value::Table(table).try_into().map_err(|error| refuse(path, brief(&error)))?;
    config.check().map_err(|reason| refuse(path, reason))?;
    Ok(config)
}

fn defaults() -> Result<Table, Fail> {
    read(DEFAULTS, Path::new("default.toml"))
}

impl Config {
    pub fn load(path: &Path) -> Result<Config, Fail> {
        let mut table = defaults()?;
        match std::fs::read_to_string(path) {
            Ok(text) => merge(&mut table, read(&text, path)?),
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => return Err(refuse(path, error)),
        }
        build(table, path)
    }

    fn check_watch(&self) -> Result<(), String> {
        let watch = &self.watch;
        for (name, profile) in &watch.profiles {
            if !RESOLUTIONS.contains(&profile.resolution.as_str()) {
                return Err(format!("watch.profiles.{name} resolution has to be low or high"));
            }
            if profile.agentic == profile.fps.is_some() {
                return Err(format!("watch.profiles.{name} needs either fps or agentic = true"));
            }
        }
        let skims = |name: &String| watch.profiles.get(name).is_some_and(|profile| profile.agentic);
        for mode in Mode::ALL {
            let mode = mode.name();
            if !watch.output_tokens.contains_key(mode) {
                return Err(format!("watch.output_tokens has no entry for {mode}"));
            }
            let Some(ladder) = watch.ladders.get(mode) else {
                return Err(format!("watch.ladders has no entry for {mode}"));
            };
            if let Some(name) = ladder.iter().find(|name| !watch.profiles.contains_key(*name)) {
                return Err(format!("watch.ladders.{mode} names {name}, which has no [watch.profiles] entry"));
            }
            if ladder.iter().all(skims) {
                return Err(format!("watch.ladders.{mode} needs a profile with fps"));
            }
            if ladder.iter().rev().skip(1).any(skims) {
                return Err(format!("watch.ladders.{mode} can name an agentic profile only last"));
            }
        }
        Ok(())
    }

    fn check_gemini(&self) -> Result<Vec<(&'static str, f64)>, String> {
        let gemini = &self.gemini;
        if gemini.model_ladder.is_empty() {
            return Err("gemini.model_ladder names no model".to_string());
        }
        if let Some(model) = gemini.model_ladder.iter().find(|model| !self.limits.models.contains_key(*model)) {
            return Err(format!("gemini.model_ladder names {model}, which has no [limits.models] entry"));
        }
        let per_second = gemini.answer_seconds_per_media_second;
        if !(per_second >= 0.0 && per_second.is_finite()) {
            return Err("gemini.answer_seconds_per_media_second cannot be negative".to_string());
        }
        Ok(vec![
            ("gemini.request_timeout_seconds", gemini.request_timeout_seconds),
            ("gemini.first_answer_seconds", gemini.first_answer_seconds),
            ("gemini.error_text_chars", gemini.error_text_chars as f64),
            ("gemini.retry.attempts", f64::from(gemini.retry.attempts)),
            ("gemini.retry.tries_per_model", gemini.retry.tries_per_model as f64),
            ("gemini.retry.first_delay_seconds", gemini.retry.first_delay_seconds),
            ("gemini.retry.max_delay_seconds", gemini.retry.max_delay_seconds),
            ("gemini.retry.total_seconds", gemini.retry.total_seconds),
        ])
    }

    fn check(&self) -> Result<(), String> {
        let limits = &self.limits;
        let (analysis, render, tokens, watch) = (&self.analysis, &self.render, &self.tokens, &self.watch);
        if !(limits.usage_share > 0.0 && limits.usage_share <= 1.0) {
            return Err("limits.usage_share has to be above 0 and at most 1".to_string());
        }
        let mut positive = self.check_gemini()?;
        positive.extend([
            ("store.busy_timeout_seconds", self.store.busy_timeout_seconds),
            ("youtube.timeout_seconds", self.youtube.timeout_seconds),
            ("youtube.download_timeout_seconds", self.youtube.download_timeout_seconds),
            ("youtube.info_hours", self.youtube.info_hours),
            ("transcript.segment_seconds", self.transcript.segment_seconds),
            ("transcript.segment_chars", self.transcript.segment_chars as f64),
            ("transcript.search_results", self.transcript.search_results as f64),
            ("transcript.output_chars", self.transcript.output_chars as f64),
            ("analysis.max_steps", analysis.max_steps as f64),
            ("analysis.max_points", analysis.max_points as f64),
            ("analysis.max_sounds", analysis.max_sounds as f64),
            ("analysis.max_list_items", analysis.max_list_items as f64),
            ("analysis.max_focus_chars", analysis.max_focus_chars as f64),
            ("analysis.quiet_min_seconds", analysis.quiet_min_seconds),
            ("render.page_chars", render.page_chars as f64),
            ("render.sound_attach_seconds", render.sound_attach_seconds),
            ("tokens.video_frame_low", tokens.video_frame_low),
            ("tokens.video_frame_high", tokens.video_frame_high),
            ("tokens.audio_per_second", tokens.audio_per_second),
            ("tokens.text_chars_per_token", tokens.text_chars_per_token),
            ("watch.agentic_tokens", watch.agentic_tokens as f64),
            ("watch.coarse_sound_fps", watch.coarse_sound_fps),
            ("limits.youtube_seconds_per_day", limits.youtube_seconds_per_day),
            ("limits.rejection_block_seconds", limits.rejection_block_seconds),
            ("limits.history_days", limits.history_days),
            ("limits.groq.requests_per_minute", limits.groq.requests_per_minute),
            ("limits.groq.requests_per_day", limits.groq.requests_per_day),
            ("limits.groq.audio_seconds_per_hour", limits.groq.audio_seconds_per_hour),
            ("limits.groq.audio_seconds_per_day", limits.groq.audio_seconds_per_day),
        ]);
        for model in limits.models.values() {
            positive.push(("limits.models requests_per_minute", model.requests_per_minute));
            positive.push(("limits.models tokens_per_minute", model.tokens_per_minute));
            positive.push(("limits.models requests_per_day", model.requests_per_day));
        }
        positive.extend(watch.output_tokens.values().map(|tokens| ("watch.output_tokens", *tokens as f64)));
        positive.extend(watch.profiles.values().filter_map(|profile| Some(("watch.profiles fps", profile.fps?))));
        if let Some((name, _)) = positive.iter().find(|(_, value)| !(*value > 0.0 && value.is_finite())) {
            return Err(format!("{name} has to be above 0"));
        }
        if !(limits.max_wait_seconds >= 0.0 && limits.max_wait_seconds.is_finite()) {
            return Err("limits.max_wait_seconds cannot be negative".to_string());
        }
        self.check_watch()
    }
}

#[cfg(test)]
impl Config {
    pub(crate) fn standard() -> Config {
        build(defaults().expect("defaults"), Path::new("video.toml")).expect("standard settings")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn folder(name: &str) -> std::path::PathBuf {
        let folder = std::env::temp_dir().join(format!("wisply-video-config-{name}-{}", std::process::id()));
        std::fs::create_dir_all(&folder).expect("folder");
        folder
    }

    fn loaded(name: &str, text: &str) -> Result<Config, Fail> {
        let path = folder(name).join("video.toml");
        std::fs::write(&path, text).expect("write");
        let config = Config::load(&path);
        std::fs::remove_dir_all(folder(name)).expect("clean");
        config
    }

    #[test]
    fn without_a_file_the_built_in_settings_apply() {
        let config = Config::load(Path::new("no-such-folder").join("video.toml").as_path()).expect("defaults");
        assert_eq!(config.gemini.model_ladder.len(), 3);
        assert_eq!(config.limits.usage_share, 0.8);
        assert_eq!(config.limits.models["gemini-3.8-flash"].requests_per_day, 20.0);
        assert_eq!(config.limits.groq.audio_seconds_per_hour, 7200.0);
        assert!(config.youtube.command.is_empty());
        assert_eq!(config.youtube.subtitle_format, "json3/vtt");
        assert_eq!(config.transcript.segment_chars, 300);
        assert_eq!((config.gemini.max_hops, config.gemini.retry.attempts), (2, 4));
        assert_eq!((config.analysis.max_steps, config.render.page_chars), (80, 8000));
        assert_eq!((config.tokens.video_frame_high, config.tokens.audio_per_second), (264.0, 25.0));
        assert_eq!(config.watch.output_tokens["steps"], 8000);
        assert_eq!(config.watch.ladders["summary"], ["low_tenth", "agentic"]);
        let skim = &config.watch.profiles["agentic"];
        assert_eq!((skim.resolution.as_str(), skim.fps, skim.agentic), ("low", None, true));
        assert_eq!(config.watch.profiles["high_half"].fps, Some(0.5));
    }

    #[test]
    fn a_file_changes_only_what_it_names() {
        let text = "[limits]\nusage_share = 0.5\n[limits.models.\"gemini-3.8-flash\"]\nrequests_per_day = 100\n";
        let config = loaded("merge", text).expect("merged");
        assert_eq!(config.limits.usage_share, 0.5);
        assert_eq!(config.limits.max_wait_seconds, 65.0);
        assert_eq!(config.limits.models["gemini-3.8-flash"].requests_per_day, 100.0);
        assert_eq!(config.limits.models["gemini-3.8-flash"].requests_per_minute, 5.0);
        assert_eq!(config.limits.models["gemini-3.6-flash"].requests_per_day, 20.0);
    }

    #[test]
    fn a_file_can_add_a_watch_profile_and_use_it() {
        let text = "[watch.profiles.low_half]\nresolution = \"low\"\nfps = 0.5\n[watch.ladders]\n\
                    steps = [\"high_1\", \"low_half\"]\n[gemini.retry]\nattempts = 2\n";
        let config = loaded("merge-watch", text).expect("merged");
        assert_eq!(config.watch.ladders["steps"], ["high_1", "low_half"]);
        assert_eq!(config.watch.ladders["full"], ["high_1", "high_half", "low_1"]);
        assert_eq!((config.watch.profiles.len(), config.watch.profiles["low_half"].fps), (6, Some(0.5)));
        assert_eq!((config.gemini.retry.attempts, config.gemini.retry.tries_per_model), (2, 2));
    }

    #[test]
    fn bad_values_are_named_with_the_file() {
        let cases = [
            ("[limits]\nusage_share = 1.5\n", "usage_share"),
            ("[limits]\nhistory_days = 0\n", "history_days"),
            ("[gemini]\nmodel_ladder = []\n", "names no model"),
            ("[gemini]\nmodel_ladder = [\"gemini-0\"]\n", "gemini-0"),
            ("[limits]\nusage_shar = 0.5\n", "usage_shar"),
            ("[store]\n[limits\nusage_share = 0.5\n", "line 2"),
            ("[gemini.retry]\nattempts = 0\n", "gemini.retry.attempts has to be above 0"),
            ("[render]\npage_chars = 0\n", "render.page_chars has to be above 0"),
            ("[watch.output_tokens]\nsteps = 0\n", "watch.output_tokens has to be above 0"),
            ("[watch.profiles.high_1]\nfps = 0\n", "watch.profiles fps has to be above 0"),
            ("[watch.profiles.high_1]\nresolution = \"medium\"\n", "high_1 resolution has to be low or high"),
            ("[watch.profiles.high_1]\nagentic = true\n", "high_1 needs either fps or agentic = true"),
            ("[watch.profiles.bare]\nresolution = \"low\"\n", "bare needs either fps or agentic = true"),
            ("[watch.ladders]\nsteps = [\"high_1\", \"sharp\"]\n", "watch.ladders.steps names sharp"),
            ("[watch.ladders]\nsummary = [\"agentic\"]\n", "watch.ladders.summary needs a profile with fps"),
            ("[watch.ladders]\nfull = []\n", "watch.ladders.full needs a profile with fps"),
            ("[watch.ladders]\nsummary = [\"agentic\", \"low_tenth\"]\n", "an agentic profile only last"),
        ];
        for (text, word) in cases {
            let fail = loaded("bad", text).expect_err(text);
            assert!(fail.message.contains(word), "{text}: {}", fail.message);
            assert!(fail.hint.contains("video.toml"), "{}", fail.hint);
            assert_eq!(fail.kind, Kind::Input);
        }
    }
}
