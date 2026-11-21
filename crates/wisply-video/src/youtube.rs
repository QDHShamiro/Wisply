use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::captions::{self, Segment, Track};
use crate::config;
use crate::explain;
use crate::fail::{Fail, Kind};
use crate::guard::Clock;
use crate::process::{self, Ran, Runner};
use crate::scratch::Scratch;
use crate::source::Source;
use crate::store::Store;

pub const ORIGINAL_SUFFIX: &str = "-orig";
const PROGRAM: &str = "yt-dlp";
const INSTALL_HINT: &str = if cfg!(windows) { "winget install yt-dlp.yt-dlp" } else { "install yt-dlp" };
const BROKEN_HINT: &str = "install yt-dlp again, or set command under [youtube] in video.toml to one that works";
const LIVE_NOW: &str = "is_live";
const UPCOMING: &str = "is_upcoming";
const NO_CODEC: &str = "none";
const LIVE_CHAT: &str = "live_chat";
const DEFAULT_PREFERENCE: f64 = -1.0;
const HOUR: f64 = 3600.0;
const CACHE_FOLDER: &str = "yt-dlp";
const SCRATCH_FOLDER: &str = "tmp";
const SCRATCH_AGE_FACTOR: f64 = 2.0;
const OUTPUT_NAME: &str = "%(id)s";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    pub start: f64,
    pub end: f64,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoInfo {
    pub video_id: String,
    pub title: String,
    pub channel: String,
    pub duration: Option<f64>,
    pub live_status: String,
    pub availability: String,
    pub age_limit: i64,
    pub width: Option<i64>,
    pub height: Option<i64>,
    pub has_audio: Option<bool>,
    pub language: String,
    pub chapters: Vec<Chapter>,
    pub subtitle_languages: Vec<String>,
    pub caption_languages: Vec<String>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Video {
    pub id: String,
    pub url: String,
}

pub struct YouTube<'a> {
    pub settings: &'a config::YouTube,
    pub store: &'a Store,
    pub runner: &'a dyn Runner,
    pub clock: &'a dyn Clock,
    pub tools: &'a Path,
    pub folder: &'a Path,
}

impl VideoInfo {
    pub fn is_live(&self) -> bool {
        self.live_status == LIVE_NOW
    }

    pub fn not_started(&self) -> bool {
        self.live_status == UPCOMING
    }
}

impl Video {
    pub fn of(source: &Source) -> Option<Video> {
        match source {
            Source::YouTube { id, .. } => Some(Video { id: id.clone(), url: source.url()? }),
            _ => None,
        }
    }
}

fn number(value: Option<&Value>) -> Option<f64> {
    value.and_then(Value::as_f64).filter(|number| number.is_finite())
}

fn whole(value: Option<&Value>) -> Option<i64> {
    number(value).map(|number| number as i64)
}

fn text(value: Option<&Value>) -> String {
    value.and_then(Value::as_str).unwrap_or_default().trim().to_string()
}

fn list(value: Option<&Value>) -> &[Value] {
    value.and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
}

fn chapters(raw: Option<&Value>, duration: Option<f64>) -> Vec<Chapter> {
    let read = |item: &Value| {
        let start = number(item.get("start_time"))?;
        let end = number(item.get("end_time")).or(duration)?;
        Some(Chapter { start, end, title: text(item.get("title")) })
    };
    list(raw).iter().filter_map(read).collect()
}

fn has_sound(format: &Value) -> bool {
    format.get("acodec").and_then(Value::as_str).is_some_and(|codec| codec != NO_CODEC)
}

fn has_audio(formats: &[Value]) -> Option<bool> {
    if formats.is_empty() { None } else { Some(formats.iter().any(has_sound)) }
}

fn spoken_language(data: &Value) -> String {
    let mut best: Option<(f64, String)> = None;
    for format in list(data.get("formats")).iter().filter(|format| has_sound(format)) {
        let language = text(format.get("language"));
        let preference = number(format.get("language_preference")).unwrap_or(DEFAULT_PREFERENCE);
        if !language.is_empty() && best.as_ref().is_none_or(|(top, _)| preference > *top) {
            best = Some((preference, language));
        }
    }
    best.map_or_else(|| text(data.get("language")), |(_, language)| language)
}

fn language_keys(raw: Option<&Value>) -> Vec<String> {
    let names = raw.and_then(Value::as_object).into_iter().flat_map(|tracks| tracks.keys());
    let mut keys: Vec<String> = names.filter(|key| *key != LIVE_CHAT).cloned().collect();
    keys.sort();
    keys
}

fn caption_languages(raw: Option<&Value>) -> Vec<String> {
    let keys = language_keys(raw);
    let originals: Vec<String> = keys.iter().filter(|key| key.ends_with(ORIGINAL_SUFFIX)).cloned().collect();
    if originals.is_empty() { keys } else { originals }
}

pub fn info_from_json(data: &Value, video_id: &str, description_chars: usize) -> VideoInfo {
    let duration = number(data.get("duration"));
    let title = text(data.get("title"));
    let channel = text(data.get("channel"));
    VideoInfo {
        video_id: video_id.to_string(),
        title: if title.is_empty() { video_id.to_string() } else { title },
        channel: if channel.is_empty() { text(data.get("uploader")) } else { channel },
        duration,
        live_status: text(data.get("live_status")),
        availability: text(data.get("availability")),
        age_limit: whole(data.get("age_limit")).unwrap_or(0),
        width: whole(data.get("width")),
        height: whole(data.get("height")),
        has_audio: has_audio(list(data.get("formats"))),
        language: spoken_language(data),
        chapters: chapters(data.get("chapters"), duration),
        subtitle_languages: language_keys(data.get("subtitles")),
        caption_languages: caption_languages(data.get("automatic_captions")),
        description: text(data.get("description")).chars().take(description_chars).collect(),
    }
}

fn has_suffix(file: &Path, suffix: &str) -> bool {
    file.extension().is_some_and(|found| found.eq_ignore_ascii_case(suffix))
}

fn subtitle_file(folder: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(folder).ok()?.flatten().map(|entry| entry.path());
    let mut files: Vec<PathBuf> = entries.filter(|path| path.is_file()).collect();
    files.sort();
    let wanted = [captions::JSON3_SUFFIX, captions::VTT_SUFFIX];
    wanted.iter().find_map(|suffix| files.iter().find(|file| has_suffix(file, suffix)).cloned())
}

fn read_subtitles(file: &Path) -> Result<Vec<Segment>, Fail> {
    let raw = fs::read(file).map_err(|error| {
        Fail::new(Kind::Service, format!("the subtitle file from YouTube cannot be read ({error})"))
    })?;
    if !has_suffix(file, captions::JSON3_SUFFIX) {
        return Ok(captions::parse_vtt(&String::from_utf8_lossy(&raw)));
    }
    let data: Value = serde_json::from_slice(&raw)
        .map_err(|_| Fail::new(Kind::Service, "the subtitle file from YouTube is not valid JSON"))?;
    Ok(captions::parse_json3(&data))
}

fn first_answer(runner: &dyn Runner, commands: Vec<Vec<String>>, rest: &[String], seconds: f64) -> Result<Ran, Fail> {
    let mut silent = Vec::new();
    let mut missing = None;
    for mut argv in commands {
        let program = argv.join(" ");
        argv.extend_from_slice(rest);
        match runner.run(&argv, seconds, PROGRAM) {
            Ok(ran) if ran.ok() || !ran.stdout.is_empty() || !ran.stderr.trim().is_empty() => return Ok(ran),
            Ok(_) => silent.push(program),
            Err(fail) if fail.kind == Kind::ToolMissing => missing = Some(fail.message),
            Err(fail) => return Err(fail),
        }
    }
    let message = match missing {
        Some(message) if silent.is_empty() => message,
        _ => format!("yt-dlp does not work: {} ended without any output", silent.join(" and ")),
    };
    Err(Fail::hint(Kind::ToolMissing, message, BROKEN_HINT))
}

impl YouTube<'_> {
    fn commands(&self) -> Result<Vec<Vec<String>>, Fail> {
        if !self.settings.command.is_empty() {
            return Ok(vec![self.settings.command.clone()]);
        }
        let found = process::find_programs(PROGRAM, self.tools);
        if found.is_empty() {
            return Err(Fail::hint(Kind::ToolMissing, "yt-dlp is not installed", INSTALL_HINT));
        }
        Ok(found.iter().map(|program| vec![program.to_string_lossy().into_owned()]).collect())
    }

    fn run(&self, more: &[&str]) -> Result<Ran, Fail> {
        let cache = self.folder.join(CACHE_FOLDER).to_string_lossy().into_owned();
        let always = ["--ignore-config", "--no-playlist", "--no-progress", "--cache-dir", &cache];
        let rest: Vec<String> = always.iter().chain(more).map(|part| part.to_string()).collect();
        first_answer(self.runner, self.commands()?, &rest, self.settings.timeout_seconds)
    }

    pub fn cached(&self, video_id: &str) -> Result<Option<VideoInfo>, Fail> {
        let stored = self.store.video_info(video_id, self.settings.info_hours * HOUR, self.clock.now())?;
        Ok(stored.and_then(|value| serde_json::from_value(value).ok()))
    }

    pub fn info(&self, video: &Video) -> Result<VideoInfo, Fail> {
        match self.cached(&video.id)? {
            Some(info) => Ok(info),
            None => self.fetch(video),
        }
    }

    fn fetch(&self, video: &Video) -> Result<VideoInfo, Fail> {
        let (format, sort) = (&self.settings.frame_format, &self.settings.frame_sort);
        let more = ["--skip-download", "--ignore-no-formats-error", "-f", format, "-S", sort, "-J", "--", &video.url];
        let ran = self.run(&more)?;
        if !ran.ok() {
            return Err(explain::failure(&ran.stderr));
        }
        let data: Value = serde_json::from_slice(&ran.stdout)
            .map_err(|_| Fail::new(Kind::Service, "yt-dlp gave an answer that is not JSON"))?;
        if !data.is_object() {
            return Err(Fail::new(Kind::Service, "yt-dlp gave an answer without video details"));
        }
        let info = info_from_json(&data, &video.id, self.settings.description_chars);
        if list(data.get("formats")).is_empty() && !info.not_started() {
            return Err(explain::failure(&ran.stderr));
        }
        self.store.put_video_info(&video.id, &serde_json::to_value(&info)?, self.clock.now())?;
        Ok(info)
    }

    pub fn subtitles(&self, video: &Video, track: &Track) -> Result<Vec<Segment>, Fail> {
        let max_age = self.settings.download_timeout_seconds * SCRATCH_AGE_FACTOR;
        let scratch = Scratch::new(&self.folder.join(SCRATCH_FOLDER), max_age)?;
        let target = scratch.path().join(OUTPUT_NAME).to_string_lossy().into_owned();
        let (write, languages) =
            (if track.automatic { "--write-auto-subs" } else { "--write-subs" }, regex::escape(&track.key));
        let format = &self.settings.subtitle_format;
        let more = [
            "--skip-download",
            write,
            "--sub-langs",
            &languages,
            "--sub-format",
            format,
            "-o",
            &target,
            "--",
            &video.url,
        ];
        let ran = self.run(&more)?;
        if !ran.ok() {
            return Err(explain::failure(&ran.stderr));
        }
        match subtitle_file(scratch.path()) {
            Some(file) => read_subtitles(&file),
            None => Err(Fail::new(Kind::Service, format!("YouTube gave no {} subtitles for {}", track.key, video.id))),
        }
    }
}

#[cfg(test)]
#[path = "youtube_tests.rs"]
mod tests;
