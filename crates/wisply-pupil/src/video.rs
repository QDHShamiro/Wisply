use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::process::{Command, Stdio};

use serde_json::Value;

use crate::config::{Audio, Frames};
use crate::pick::Scanned;
use crate::spawn;

#[derive(Debug, Clone, PartialEq)]
pub struct Info {
    pub width: u32,
    pub height: u32,
    pub duration: f64,
}

#[derive(Debug, Clone)]
pub struct Picture {
    pub at: f64,
    pub small: bool,
    pub jpeg: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shot {
    pub at: f64,
    pub size: (u32, u32),
    pub small: bool,
}

pub fn last_lines(text: &str, count: usize) -> String {
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    lines[lines.len().saturating_sub(count)..].join(" | ")
}

pub fn parse_info(raw: &str) -> Option<Info> {
    let value: Value = serde_json::from_str(raw).ok()?;
    let stream = value.get("streams")?.as_array()?.first()?;
    let width = stream.get("width")?.as_u64()? as u32;
    let height = stream.get("height")?.as_u64()? as u32;
    let duration = value
        .get("format")
        .and_then(|f| f.get("duration"))
        .and_then(|d| d.as_str())
        .and_then(|d| d.parse::<f64>().ok())
        .or_else(|| stream.get("duration").and_then(|d| d.as_str()).and_then(|d| d.parse::<f64>().ok()))?;
    if width == 0 || height == 0 || !duration.is_finite() || duration <= 0.0 {
        return None;
    }
    Some(Info { width, height, duration })
}

pub fn parse_size(raw: &str) -> Option<(u32, u32)> {
    let value: Value = serde_json::from_str(raw).ok()?;
    let stream = value.get("streams")?.as_array()?.first()?;
    let width = stream.get("width")?.as_u64()? as u32;
    let height = stream.get("height")?.as_u64()? as u32;
    (width > 0 && height > 0).then_some((width, height))
}

pub fn size(frames: &Frames, file: &Path) -> Result<(u32, u32), String> {
    let output = spawn::quiet(&frames.ffprobe)
        .args(["-v", "error", "-select_streams", "v:0", "-show_entries", "stream=width,height", "-of", "json"])
        .arg(file)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("{}: {e}", frames.ffprobe))?;
    let text = String::from_utf8_lossy(&output.stdout);
    parse_size(&text).ok_or_else(|| last_lines(&String::from_utf8_lossy(&output.stderr), 3))
}

pub fn probe(frames: &Frames, video: &Path) -> Result<Info, String> {
    let output = spawn::quiet(&frames.ffprobe)
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-show_entries",
            "stream=width,height,duration:format=duration",
            "-of",
            "json",
        ])
        .arg(video)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("{}: {e}", frames.ffprobe))?;
    let text = String::from_utf8_lossy(&output.stdout);
    parse_info(&text).ok_or_else(|| last_lines(&String::from_utf8_lossy(&output.stderr), 3))
}

pub fn frame_size(width: u32, height: u32, long_side: u32, patch: u32) -> (u32, u32) {
    fit_size(width, height, long_side.min(width.max(height).max(1)), patch)
}

pub fn still_size(width: u32, height: u32, floor: u32, ceiling: u32, patch: u32) -> (u32, u32) {
    let long = width.max(height).max(1);
    fit_size(width, height, long.max(floor).min(ceiling), patch)
}

pub fn fit_size(width: u32, height: u32, long_side: u32, patch: u32) -> (u32, u32) {
    let patch = patch.max(1);
    let long = width.max(height).max(1);
    let short = width.min(height).max(1);
    let target = long_side.max(patch) / patch * patch;
    let scaled = f64::from(short) * f64::from(target) / f64::from(long);
    let other = ((scaled / f64::from(patch)).round() as u32).max(1) * patch;
    if width >= height { (target, other) } else { (other, target) }
}

pub fn parse_time(line: &str) -> Option<f64> {
    let rest = &line[line.find("pts_time:")? + "pts_time:".len()..];
    let number: String =
        rest.chars().take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-' || *c == 'e').collect();
    number.parse::<f64>().ok().filter(|t| t.is_finite())
}

fn read_stderr(stderr: std::process::ChildStderr) -> (Vec<f64>, String) {
    let mut times = Vec::new();
    let mut tail = String::new();
    for line in BufReader::new(stderr).lines().map_while(Result::ok) {
        if line.contains("showinfo") {
            if let Some(time) = parse_time(&line) {
                times.push(time);
            }
            continue;
        }
        tail.push_str(&line);
        tail.push('\n');
        if tail.len() > 8192 {
            let cut = tail.len() - 4096;
            let cut = (cut..tail.len()).find(|i| tail.is_char_boundary(*i)).unwrap_or(tail.len());
            tail.drain(..cut);
        }
    }
    (times, tail)
}

fn read_pictures(mut stdout: impl Read, size: usize) -> Vec<Vec<u8>> {
    let mut pictures = Vec::new();
    loop {
        let mut buffer = vec![0u8; size];
        if stdout.read_exact(&mut buffer).is_err() {
            return pictures;
        }
        pictures.push(buffer);
    }
}

fn scan_command(frames: &Frames, video: &Path, start: f64, end: f64) -> Command {
    let filter = format!(
        "fps={},scale={}:{}:flags=area,format=gray,showinfo",
        frames.scan_fps, frames.scan_width, frames.scan_height
    );
    let mut command = spawn::quiet(&frames.ffmpeg);
    command
        .args(["-hide_banner", "-nostdin", "-nostats", "-loglevel", "info", "-skip_frame", &frames.skip_frame])
        .args(["-ss", &format!("{start:.3}")])
        .arg("-i")
        .arg(video)
        .args(["-t", &format!("{:.3}", end - start), "-an", "-sn", "-dn", "-vf", &filter, "-f", "rawvideo", "-"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub lead: usize,
    pub from: usize,
    pub to: Option<usize>,
    pub stop: Option<usize>,
}

pub fn pieces(steps: usize, parts: usize, overlap: usize) -> Vec<Piece> {
    let size = steps.div_ceil(parts.clamp(1, steps.max(1))).max(1);
    (0..steps.max(1))
        .step_by(size)
        .map(|from| {
            let to = (from + size < steps).then_some(from + size);
            Piece { lead: from.saturating_sub(overlap), from, to, stop: to.map(|to| (to + overlap).min(steps)) }
        })
        .collect()
}

fn scan_piece(frames: &Frames, video: &Path, (start, end): (f64, f64), piece: Piece) -> Result<Vec<Scanned>, String> {
    let fps = frames.scan_fps;
    let time = |step: usize| start + step as f64 / fps;
    let stop = piece.stop.map_or(end, |step| time(step).min(end));
    let items = scan_once(frames, video, time(piece.lead), stop)?;
    Ok(items
        .into_iter()
        .filter(|item| {
            let step = ((item.at - start) * fps).round();
            step >= piece.from as f64 && piece.to.is_none_or(|to| step < to as f64)
        })
        .collect())
}

pub fn scan(frames: &Frames, video: &Path, start: f64, end: f64) -> Result<Vec<Scanned>, String> {
    let first = scan_with(frames, video, start, end);
    let expected = ((end - start) * frames.scan_fps).ceil();
    let found = first.as_ref().map_or(0, Vec::len);
    if !retry_scan(frames, found, expected) {
        return first;
    }
    let mut again = frames.clone();
    again.skip_frame = frames.scan_retry_skip.clone();
    match scan_with(&again, video, start, end) {
        Ok(list) if list.len() > found => Ok(list),
        Ok(_) | Err(_) => first,
    }
}

pub fn retry_scan(frames: &Frames, found: usize, expected: f64) -> bool {
    let retry = &frames.scan_retry_skip;
    !retry.is_empty()
        && *retry != frames.skip_frame
        && (found == 0 || (found as f64) < frames.scan_retry_share * expected)
}

fn scan_with(frames: &Frames, video: &Path, start: f64, end: f64) -> Result<Vec<Scanned>, String> {
    let steps = ((end - start) * frames.scan_fps).ceil();
    let split = frames.scan_parts > 1 && end - start >= frames.scan_split_seconds && steps.is_finite() && steps >= 1.0;
    if !split {
        return scan_once(frames, video, start, end);
    }
    let overlap = (frames.scan_overlap_seconds * frames.scan_fps).ceil().max(0.0) as usize;
    let list = pieces(steps as usize, frames.scan_parts, overlap);
    let made: Vec<Result<Vec<Scanned>, String>> = std::thread::scope(|scope| {
        let running: Vec<_> =
            list.iter().map(|piece| scope.spawn(move || scan_piece(frames, video, (start, end), *piece))).collect();
        running.into_iter().map(|handle| handle.join().unwrap_or_else(|_| Err(String::new()))).collect()
    });
    let mut all = Vec::new();
    for part in made {
        match part {
            Ok(items) => all.extend(items),
            Err(_) => return scan_once(frames, video, start, end),
        }
    }
    Ok(all)
}

fn scan_once(frames: &Frames, video: &Path, start: f64, end: f64) -> Result<Vec<Scanned>, String> {
    let mut child = scan_command(frames, video, start, end).spawn().map_err(|e| format!("{}: {e}", frames.ffmpeg))?;
    let stderr = child.stderr.take().ok_or("no stderr")?;
    let reader = std::thread::spawn(move || read_stderr(stderr));
    let stdout = child.stdout.take().ok_or("no stdout")?;
    let pictures = read_pictures(stdout, (frames.scan_width * frames.scan_height) as usize);
    let status = child.wait().map_err(|e| e.to_string())?;
    let (times, tail) = reader.join().map_err(|_| "ffmpeg reader crashed".to_string())?;
    if !status.success() && pictures.is_empty() {
        return Err(last_lines(&tail, 3));
    }
    let exact = times.len() == pictures.len();
    Ok(pictures
        .into_iter()
        .enumerate()
        .map(|(index, pixels)| {
            let offset = if exact { times[index] } else { index as f64 / frames.scan_fps };
            Scanned { at: start + offset.max(0.0), pixels }
        })
        .filter(|item| item.at <= end)
        .collect())
}

pub fn sound(frames: &Frames, audio: &Audio, source: &Path, start: f64, end: f64) -> Option<Vec<u8>> {
    let length = end - start;
    if !length.is_finite() || length <= 0.0 {
        return None;
    }
    let output = spawn::quiet(&frames.ffmpeg)
        .args(["-hide_banner", "-nostdin", "-nostats", "-loglevel", "error", "-ss", &format!("{start:.3}")])
        .arg("-i")
        .arg(source)
        .args(["-t", &format!("{length:.3}"), "-vn", "-sn", "-dn", "-ac", "1"])
        .args(["-ar", &audio.sample_rate.to_string(), "-b:a", &audio.bitrate])
        .args(["-f", &audio.format, "-"])
        .stdin(Stdio::null())
        .output()
        .ok()?;
    (output.status.success() && !output.stdout.is_empty()).then_some(output.stdout)
}

#[cfg(test)]
#[path = "video_tests.rs"]
mod tests;
