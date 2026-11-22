use std::collections::BTreeMap;
use std::io::ErrorKind;
use std::path::Path;

use serde::Deserialize;
use toml::{Table, Value};

use crate::fail::{Fail, Kind};

const DEFAULTS: &str = include_str!("default.toml");

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub store: Store,
    pub gemini: Gemini,
    pub youtube: YouTube,
    pub transcript: Transcript,
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

    fn check(&self) -> Result<(), String> {
        let limits = &self.limits;
        if !(limits.usage_share > 0.0 && limits.usage_share <= 1.0) {
            return Err("limits.usage_share has to be above 0 and at most 1".to_string());
        }
        if self.gemini.model_ladder.is_empty() {
            return Err("gemini.model_ladder names no model".to_string());
        }
        if let Some(model) = self.gemini.model_ladder.iter().find(|model| !limits.models.contains_key(*model)) {
            return Err(format!("gemini.model_ladder names {model}, which has no [limits.models] entry"));
        }
        let mut positive = vec![
            ("store.busy_timeout_seconds", self.store.busy_timeout_seconds),
            ("youtube.timeout_seconds", self.youtube.timeout_seconds),
            ("youtube.download_timeout_seconds", self.youtube.download_timeout_seconds),
            ("youtube.info_hours", self.youtube.info_hours),
            ("transcript.segment_seconds", self.transcript.segment_seconds),
            ("transcript.segment_chars", self.transcript.segment_chars as f64),
            ("transcript.search_results", self.transcript.search_results as f64),
            ("transcript.output_chars", self.transcript.output_chars as f64),
            ("limits.youtube_seconds_per_day", limits.youtube_seconds_per_day),
            ("limits.rejection_block_seconds", limits.rejection_block_seconds),
            ("limits.history_days", limits.history_days),
            ("limits.groq.requests_per_minute", limits.groq.requests_per_minute),
            ("limits.groq.requests_per_day", limits.groq.requests_per_day),
            ("limits.groq.audio_seconds_per_hour", limits.groq.audio_seconds_per_hour),
            ("limits.groq.audio_seconds_per_day", limits.groq.audio_seconds_per_day),
        ];
        for model in limits.models.values() {
            positive.push(("limits.models requests_per_minute", model.requests_per_minute));
            positive.push(("limits.models tokens_per_minute", model.tokens_per_minute));
            positive.push(("limits.models requests_per_day", model.requests_per_day));
        }
        if let Some((name, _)) = positive.iter().find(|(_, value)| !(*value > 0.0 && value.is_finite())) {
            return Err(format!("{name} has to be above 0"));
        }
        if !(limits.max_wait_seconds >= 0.0 && limits.max_wait_seconds.is_finite()) {
            return Err("limits.max_wait_seconds cannot be negative".to_string());
        }
        Ok(())
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
    fn bad_values_are_named_with_the_file() {
        let cases = [
            ("[limits]\nusage_share = 1.5\n", "usage_share"),
            ("[limits]\nhistory_days = 0\n", "history_days"),
            ("[gemini]\nmodel_ladder = []\n", "names no model"),
            ("[gemini]\nmodel_ladder = [\"gemini-0\"]\n", "gemini-0"),
            ("[limits]\nusage_shar = 0.5\n", "usage_shar"),
            ("[store]\n[limits\nusage_share = 0.5\n", "line 2"),
        ];
        for (text, word) in cases {
            let fail = loaded("bad", text).expect_err(text);
            assert!(fail.message.contains(word), "{text}: {}", fail.message);
            assert!(fail.hint.contains("video.toml"), "{}", fail.hint);
            assert_eq!(fail.kind, Kind::Input);
        }
    }
}
