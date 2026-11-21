use std::ops::Range;
use std::path::Path;
use std::process::Stdio;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::config::Frames;
use crate::spawn;
use crate::timecode;
use crate::video::{Picture, Shot, last_lines};

fn grab(frames: &Frames, video: &Path, at: f64, size: (u32, u32)) -> Result<Vec<u8>, String> {
    let output = spawn::quiet(&frames.ffmpeg)
        .args(["-hide_banner", "-nostdin", "-nostats", "-loglevel", "error"])
        .args(seek_args(frames, at))
        .arg("-i")
        .arg(video)
        .args(["-frames:v", "1", "-an", "-sn", "-dn"])
        .args(["-vf", &picture_filter(frames, at, size)])
        .args(["-q:v", &frames.jpeg_quality.to_string(), "-f", "image2pipe", "-c:v", "mjpeg", "-"])
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("{}: {e}", frames.ffmpeg))?;
    if !is_jpeg(&output.stdout) {
        return Err(last_lines(&String::from_utf8_lossy(&output.stderr), 3));
    }
    Ok(output.stdout)
}

type Slot = (usize, Result<Vec<u8>, String>);

pub fn picture_filter(frames: &Frames, at: f64, size: (u32, u32)) -> String {
    let scale = format!("scale={}:{}:flags=bicubic", size.0, size.1);
    if frames.stamp_filter.is_empty() {
        return scale;
    }
    let time = timecode::format(at).replace(':', "\\:");
    let stamp = frames.stamp_filter.replace("{time}", &time).replace("{seconds}", &format!("{}", at.max(0.0).floor()));
    format!("{scale},{stamp}")
}

pub fn seek_args(frames: &Frames, at: f64) -> Vec<String> {
    let threads = (frames.grab_threads > 0).then(|| ["-threads".to_string(), frames.grab_threads.to_string()]);
    threads.into_iter().flatten().chain(["-ss".to_string(), format!("{at:.3}")]).collect()
}

fn is_jpeg(bytes: &[u8]) -> bool {
    bytes.len() >= 4 && bytes[..2] == [0xFF, 0xD8]
}

pub fn groups(count: usize, workers: usize, batch: usize) -> Vec<Range<usize>> {
    let batch = batch.max(1);
    let wanted = workers.max(1).max(count.div_ceil(batch)).min(count);
    if wanted == 0 {
        return Vec::new();
    }
    let size = count.div_ceil(wanted);
    (0..count).step_by(size).map(|from| from..(from + size).min(count)).collect()
}

fn grab_many(frames: &Frames, video: &Path, shots: &[Shot], range: Range<usize>) -> Vec<Slot> {
    let dir = std::env::temp_dir().join(format!(
        "{}-{}-{:?}-{}",
        frames.batch_name,
        std::process::id(),
        std::thread::current().id(),
        range.start
    ));
    let mut command = spawn::quiet(&frames.ffmpeg);
    command.args(["-hide_banner", "-nostdin", "-nostats", "-loglevel", "error", "-y"]);
    for shot in &shots[range.clone()] {
        command.args(seek_args(frames, shot.at)).arg("-i").arg(video);
    }
    for (input, index) in range.clone().enumerate() {
        let shot = shots[index];
        command
            .args(["-map", &format!("{input}:V:0"), "-frames:v", "1"])
            .args(["-vf", &picture_filter(frames, shot.at, shot.size)])
            .args(["-q:v", &frames.jpeg_quality.to_string()])
            .args(["-f", "image2pipe", "-c:v", "mjpeg"])
            .arg(dir.join(index.to_string()));
    }
    if std::fs::create_dir_all(&dir).is_ok() {
        let _ = command.stdin(Stdio::null()).output();
    }
    let made = range
        .map(|index| {
            let path = dir.join(index.to_string());
            let read = std::fs::read(&path).ok().filter(|jpeg| is_jpeg(jpeg));
            let _ = std::fs::remove_file(&path);
            let shot = shots[index];
            let result = read.map_or_else(|| grab(frames, video, shot.at, shot.size), Ok);
            (index, result)
        })
        .collect();
    let _ = std::fs::remove_dir(&dir);
    made
}

fn grab_part(frames: &Frames, video: &Path, shots: &[Shot], range: Range<usize>) -> Vec<Slot> {
    if range.len() > 1 {
        return grab_many(frames, video, shots, range);
    }
    range
        .map(|index| {
            let shot = shots[index];
            (index, grab(frames, video, shot.at, shot.size))
        })
        .collect()
}

pub fn extract(frames: &Frames, video: &Path, shots: &[Shot]) -> Result<Vec<Picture>, String> {
    let next = AtomicUsize::new(0);
    let results: Mutex<Vec<Slot>> = Mutex::new(Vec::with_capacity(shots.len()));
    let batched = frames.batch > 1 && !frames.batch_name.is_empty();
    let batch = if batched { frames.batch } else { 1 };
    let parts = groups(shots.len(), frames.parallel, batch);
    let workers = frames.parallel.clamp(1, parts.len().max(1));
    let work = || {
        while let Some(range) = parts.get(next.fetch_add(1, Ordering::Relaxed)).cloned() {
            let made = grab_part(frames, video, shots, range);
            if let Ok(mut list) = results.lock() {
                list.extend(made);
            }
        }
    };
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(work);
        }
    });
    let mut list = results.into_inner().map_err(|_| "frame worker crashed".to_string())?;
    list.sort_by_key(|(index, _)| *index);
    let mut pictures = Vec::new();
    let mut last_error = String::new();
    for (index, result) in list {
        match result {
            Ok(jpeg) => pictures.push(Picture { at: shots[index].at, small: shots[index].small, jpeg }),
            Err(error) => last_error = error,
        }
    }
    if pictures.is_empty() && !shots.is_empty() {
        return Err(last_error);
    }
    Ok(pictures)
}

#[cfg(test)]
mod tests {
    use super::{groups, picture_filter, seek_args};
    use crate::config::Config;

    #[test]
    fn frames_are_split_into_one_group_per_worker() {
        assert_eq!(groups(38, 4, 16), vec![0..10, 10..20, 20..30, 30..38]);
        assert_eq!(groups(64, 4, 10), vec![0..10, 10..20, 20..30, 30..40, 40..50, 50..60, 60..64]);
        assert_eq!(groups(3, 4, 16), vec![0..1, 1..2, 2..3]);
        assert_eq!(groups(0, 4, 16), Vec::<std::ops::Range<usize>>::new());
    }

    #[test]
    fn a_batch_of_one_keeps_one_frame_per_process() {
        let made = groups(5, 4, 1);
        assert_eq!(made.len(), 5);
        assert!(made.iter().all(|range| range.len() == 1));
        assert_eq!(groups(5, 4, 0), made);
    }

    #[test]
    fn a_stamp_filter_writes_the_frame_time_after_scaling() {
        let config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
        let mut frames = config.frames;
        frames.stamp_filter = String::new();
        assert_eq!(picture_filter(&frames, 83.4, (384, 216)), "scale=384:216:flags=bicubic");
        frames.stamp_filter = "drawtext=text='{time}':x=w-tw".to_string();
        assert_eq!(
            picture_filter(&frames, 83.4, (384, 216)),
            r"scale=384:216:flags=bicubic,drawtext=text='01\:23':x=w-tw"
        );
        frames.stamp_filter = "drawtext=text='{seconds}'".to_string();
        assert!(picture_filter(&frames, 83.9, (8, 8)).ends_with("text='83'"));
    }

    #[test]
    fn grab_threads_go_before_each_seek_only_when_set() {
        let config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
        let mut frames = config.frames;
        frames.grab_threads = 0;
        assert_eq!(seek_args(&frames, 1.5), vec!["-ss", "1.500"]);
        frames.grab_threads = 2;
        assert_eq!(seek_args(&frames, 12.0), vec!["-threads", "2", "-ss", "12.000"]);
    }
}
