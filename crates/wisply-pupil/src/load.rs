use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::config::{Config, Talk};
use crate::game::system;
use crate::job::Failure;
use crate::media::{
    Kind, Loaded, Source, clip, fill, heard_text, is_tube, kind_of, kind_of_type, last_line, page_text, safe_name,
    said_plain,
};
use crate::spawn;
use crate::video;

pub struct Temp {
    dir: PathBuf,
    count: AtomicU64,
}

impl Temp {
    pub fn new(talk: &Talk) -> Result<Temp, String> {
        let name = Path::new(&talk.temp_name);
        let mut parts = name.components();
        let single = matches!(parts.next(), Some(Component::Normal(_))) && parts.next().is_none();
        if !single {
            return Err(talk.temp_name.clone());
        }
        let root = std::env::temp_dir().join(name);
        sweep(&root);
        Ok(Temp { dir: root.join(std::process::id().to_string()), count: AtomicU64::new(0) })
    }

    pub fn root(&self) -> Option<&Path> {
        self.dir.parent()
    }

    pub fn fresh(&self) -> Result<PathBuf, String> {
        let number = self.count.fetch_add(1, Ordering::Relaxed);
        let dir = self.dir.join(number.to_string());
        fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        Ok(dir)
    }
}

impl Drop for Temp {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

fn sweep(root: &Path) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let owner = entry.file_name().to_string_lossy().parse::<u32>().ok();
        if owner.is_some_and(|pid| pid != 0 && !system::alive(pid)) {
            let _ = fs::remove_dir_all(entry.path());
        }
    }
}

fn fetch(talk: &Talk, url: &str, dir: &Path) -> Result<(PathBuf, String), String> {
    let out = dir.join(safe_name(url, &talk.temp_name));
    let limit = talk.download_max_mb.saturating_mul(1024 * 1024);
    let (bytes, seconds) = (limit.to_string(), talk.download_seconds.to_string());
    let target = out.to_string_lossy().into_owned();
    let args = fill(
        &talk.curl_args,
        &[("bytes", &bytes), ("seconds", &seconds), ("agent", &talk.user_agent), ("out", &target), ("url", url)],
    );
    let output =
        spawn::quiet(&talk.curl).args(args).stdin(Stdio::null()).output().map_err(|e| format!("{}: {e}", talk.curl))?;
    let size = fs::metadata(&out).map(|m| m.len()).unwrap_or(0);
    if !output.status.success() || size == 0 || size > limit {
        let _ = fs::remove_file(&out);
        return Err(last_line(&output.stderr));
    }
    Ok((out, String::from_utf8_lossy(&output.stdout).trim().to_string()))
}

fn unfinished(path: &Path, partial: &[String]) -> bool {
    let name = path.file_name().map(|name| name.to_string_lossy().to_lowercase()).unwrap_or_default();
    partial.iter().any(|end| name.ends_with(&end.to_lowercase()))
}

fn largest(dir: &Path, skip: &Path, partial: &[String]) -> Option<PathBuf> {
    fs::read_dir(dir)
        .ok()?
        .flatten()
        .filter_map(|entry| {
            let meta = entry.metadata().ok()?;
            let path = entry.path();
            (meta.is_file() && path != skip && !unfinished(&path, partial)).then_some((meta.len(), path))
        })
        .max_by_key(|(size, _)| *size)
        .map(|(_, path)| path)
}

fn about_text(talk: &Talk, file: &Path) -> String {
    let raw = fs::read(file).unwrap_or_default();
    let text = String::from_utf8_lossy(&raw);
    let lines: Vec<&str> = text.lines().map(str::trim).filter(|line| !line.is_empty()).collect();
    clip(&lines.join("\n"), talk.about_max_chars, &talk.text.cut)
}

fn tries<T>(count: u32, mut run: impl FnMut() -> Result<T, String>) -> Result<T, String> {
    let mut last = run();
    for _ in 1..count {
        if last.is_ok() {
            break;
        }
        last = run();
    }
    last
}

fn tube(talk: &Talk, url: &str, dir: &Path) -> Result<(PathBuf, String), String> {
    let notes = dir.join(&talk.about_file);
    let (mb, target, about) =
        (talk.download_max_mb.to_string(), dir.to_string_lossy().into_owned(), notes.to_string_lossy().into_owned());
    let args = fill(
        &talk.ytdlp_args,
        &[("mb", &mb), ("agent", &talk.user_agent), ("dir", &target), ("about", &about), ("url", url)],
    );
    tries(talk.ytdlp_tries, || {
        let _ = fs::remove_file(&notes);
        let output = spawn::quiet(&talk.ytdlp)
            .args(&args)
            .stdin(Stdio::null())
            .output()
            .map_err(|e| format!("{}: {e}", talk.ytdlp))?;
        match largest(dir, &notes, &talk.partial_suffixes) {
            Some(path) if output.status.success() => Ok((path, about_text(talk, &notes))),
            _ => Err(last_line(&output.stderr)),
        }
    })
}

fn still(config: &Config, file: &Path) -> Result<(Vec<u8>, (u32, u32)), String> {
    let (frames, talk) = (&config.frames, &config.talk);
    let (width, height) = video::size(frames, file)?;
    let size = video::still_size(width, height, talk.picture_min_long_side, talk.picture_long_side, frames.patch);
    let (wide, high) = (size.0.to_string(), size.1.to_string());
    let (quality, source) = (frames.jpeg_quality.to_string(), file.to_string_lossy().into_owned());
    let args = fill(
        &talk.still_args,
        &[("w", &wide), ("h", &high), ("matte", &talk.matte), ("quality", &quality), ("file", &source)],
    );
    let output = spawn::quiet(&frames.ffmpeg)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("{}: {e}", frames.ffmpeg))?;
    if output.stdout.len() < 4 || output.stdout[..2] != [0xFF, 0xD8] {
        return Err(last_line(&output.stderr));
    }
    Ok((output.stdout, size))
}

fn listen(config: &Config, file: &Path, wanted: &[String]) -> Result<Vec<u8>, String> {
    let ear = &config.ear;
    let (source, settings) = (file.to_string_lossy().into_owned(), ear.config.to_string_lossy().into_owned());
    let args = fill(wanted, &[("speech", &ear.speech), ("config", &settings), ("file", &source)]);
    let output = spawn::ear(&ear.exe)
        .args(args)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("{}: {e}", ear.exe.display()))?;
    if !output.status.success() {
        return Err(last_line(&output.stderr));
    }
    Ok(output.stdout)
}

fn hear(config: &Config, file: &Path) -> Result<String, String> {
    heard_text(config, &listen(config, file, &config.talk.hear_args)?)
}

pub fn dictate(config: &Config, file: &Path) -> Result<String, String> {
    said_plain(&listen(config, file, &config.talk.dictate_args)?)
}

fn read_text(talk: &Talk, file: &Path, page: bool) -> Result<String, String> {
    let raw = fs::read(file).map_err(|e| e.to_string())?;
    let text = String::from_utf8_lossy(&raw);
    let text = if page { page_text(talk, &text)? } else { text.trim().to_string() };
    Ok(clip(&text, talk.text_max_chars, &talk.text.cut))
}

fn located(config: &Config, temp: &Temp, source: &Source) -> Result<(PathBuf, Kind, String, String), Failure> {
    let talk = &config.talk;
    match source {
        Source::File(path) => {
            let shown = path.display().to_string();
            if !path.is_file() {
                return Err(Failure::new(config, "talk_missing", &[("path", &shown)]));
            }
            let name = path.file_name().map_or(shown.clone(), |n| n.to_string_lossy().into_owned());
            let kind = kind_of(talk, &shown).ok_or_else(|| Failure::new(config, "talk_kind", &[("name", &name)]))?;
            Ok((path.clone(), kind, name, String::new()))
        }
        Source::Link(url) => {
            let failed = |detail: String| Failure::new(config, "talk_download", &[("name", url), ("detail", &detail)]);
            let dir = temp.fresh().map_err(failed)?;
            if is_tube(talk, url) {
                let (path, about) = tube(talk, url, &dir).map_err(failed)?;
                return Ok((path, Kind::Video, url.clone(), about));
            }
            let (path, content_type) = fetch(talk, url, &dir).map_err(failed)?;
            let kind = kind_of_type(talk, &content_type)
                .or_else(|| kind_of(talk, url))
                .ok_or_else(|| Failure::new(config, "talk_kind", &[("name", url)]))?;
            Ok((path, kind, url.clone(), String::new()))
        }
    }
}

pub fn load(config: &Config, temp: &Temp, source: &Source) -> Result<Loaded, Failure> {
    let (path, kind, name, about) = located(config, temp, source)?;
    let failed = |detail: String| Failure::new(config, "talk_read", &[("name", &name), ("detail", &detail)]);
    let moving = kind == Kind::Video && video::probe(&config.frames, &path).is_ok();
    match kind {
        Kind::Picture => {
            let (jpeg, size) = still(config, &path).map_err(failed)?;
            Ok(Loaded::Picture { name, jpeg, size })
        }
        Kind::Video if moving => Ok(Loaded::Video { name, path, about }),
        Kind::Video | Kind::Sound => {
            Ok(Loaded::Words { text: hear(config, &path).map_err(failed)?, kind: Kind::Sound, name })
        }
        Kind::Text | Kind::Page => {
            Ok(Loaded::Words { text: read_text(&config.talk, &path, kind == Kind::Page).map_err(failed)?, kind, name })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Temp, about_text, largest, tries, unfinished};
    use crate::config::Config;
    use std::path::Path;

    fn config() -> Config {
        toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses")
    }

    #[test]
    fn the_temp_folder_needs_one_plain_name() {
        let mut talk = config().talk;
        for bad in ["", "..", "a/b", "C:\\x"] {
            talk.temp_name = bad.to_string();
            assert!(Temp::new(&talk).is_err(), "{bad}");
        }
    }

    #[test]
    fn a_failed_download_is_tried_again_up_to_the_limit() {
        let mut calls = 0;
        let second = tries(3, || {
            calls += 1;
            if calls < 2 { Err("403".to_string()) } else { Ok(calls) }
        });
        assert_eq!((second, calls), (Ok(2), 2));
        for (limit, runs) in [(0, 1), (1, 1), (2, 2), (4, 4)] {
            let mut calls = 0;
            let failed: Result<(), String> = tries(limit, || {
                calls += 1;
                Err(format!("try {calls}"))
            });
            assert_eq!((failed, calls), (Err(format!("try {runs}")), runs));
        }
    }

    #[test]
    fn the_link_info_is_read_trimmed_and_never_taken_for_the_video() {
        let mut talk = config().talk;
        assert!(talk.ytdlp_tries >= 1 && talk.about_max_chars > 0);
        assert!(talk.ytdlp_args.iter().any(|arg| arg == "{about}"));
        assert!(talk.text.about.contains("{about}") && talk.text.about.contains("{name}"));
        let dir = std::env::temp_dir().join(format!("pupil-about-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("folder");
        let notes = dir.join(&talk.about_file);
        assert_eq!(about_text(&talk, &notes), "");
        assert_eq!(largest(&dir, &notes, &talk.partial_suffixes), None);
        std::fs::write(&notes, "  Llamigos \r\n\r\nKoro, größer als Oti.\n\n").expect("file");
        assert_eq!(about_text(&talk, &notes), "Llamigos\nKoro, größer als Oti.");
        assert_eq!(largest(&dir, &notes, &talk.partial_suffixes), None);
        std::fs::write(dir.join("video.mp4"), [0u8; 4]).expect("file");
        assert_eq!(largest(&dir, &notes, &talk.partial_suffixes), Some(dir.join("video.mp4")));
        std::fs::write(dir.join("video.f397.mp4.PART"), [0u8; 64]).expect("file");
        std::fs::write(dir.join("video.f251.webm.ytdl"), [0u8; 32]).expect("file");
        assert_eq!(largest(&dir, &notes, &talk.partial_suffixes), Some(dir.join("video.mp4")));
        assert!(unfinished(Path::new("a/b.mp4.part"), &talk.partial_suffixes));
        assert!(!unfinished(Path::new("a/b.mp4"), &talk.partial_suffixes));
        assert!(!unfinished(Path::new("a/b.part"), &[]));
        talk.about_max_chars = 8;
        assert_eq!(about_text(&talk, &notes), format!("Llamigos{}", talk.text.cut));
        std::fs::remove_dir_all(&dir).expect("clean up");
    }
}
