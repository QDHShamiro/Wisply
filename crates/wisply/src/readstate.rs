use std::collections::{HashMap, HashSet};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const STATE_MAX_LINES: usize = 400;
const STATE_KEEP: Duration = Duration::from_secs(7 * 86_400);
const STATE_KEEP_ROWS: usize = 300;
const SHRINK_MIN_BYTES: u64 = STATE_MAX_LINES as u64 * 16;
const LOCK_TRIES: u32 = 25;
const LOCK_PAUSE: Duration = Duration::from_millis(2);

#[derive(Clone)]
pub struct Seen {
    pub size: u64,
    pub mtime: u64,
    pub hash: u64,
    pub at: u64,
    pub cut: bool,
}

#[derive(Default)]
pub struct Table {
    pub seen: HashMap<(String, u64, u64), Seen>,
    pub edited: HashSet<String>,
}

impl Table {
    fn apply(&mut self, line: &str) {
        let f: Vec<&str> = line.split('\t').collect();
        match f.as_slice() {
            ["C"] => {
                self.seen.clear();
                self.edited.clear();
            }
            ["E", path] => {
                self.seen.retain(|k, _| k.0 != *path);
                self.edited.insert((*path).to_string());
            }
            ["R", path, start, count, size, mtime, hash, at, cut] => {
                let parsed = (|| {
                    Some((
                        (path.to_string(), start.parse().ok()?, count.parse().ok()?),
                        Seen {
                            size: size.parse().ok()?,
                            mtime: mtime.parse().ok()?,
                            hash: hash.parse().ok()?,
                            at: at.parse().ok()?,
                            cut: *cut == "1",
                        },
                    ))
                })();
                if let Some((key, seen)) = parsed {
                    self.seen.insert(key, seen);
                }
            }
            _ => {}
        }
    }

    fn trim(&mut self) {
        if self.seen.len() <= STATE_KEEP_ROWS {
            return;
        }
        let mut ages: Vec<u64> = self.seen.values().map(|s| s.at).collect();
        ages.sort_unstable_by(|a, b| b.cmp(a));
        let oldest = ages[STATE_KEEP_ROWS - 1];
        let mut room = STATE_KEEP_ROWS;
        self.seen.retain(|_, s| {
            let keep = s.at >= oldest && room > 0;
            room -= usize::from(keep);
            keep
        });
    }

    fn lines(&self) -> Vec<String> {
        let mut out: Vec<String> = self.edited.iter().map(|p| format!("E\t{p}")).collect();
        for ((path, start, count), s) in &self.seen {
            out.push(record(path, *start, *count, s));
        }
        out
    }
}

pub fn record(path: &str, start: u64, count: u64, s: &Seen) -> String {
    format!("R\t{path}\t{start}\t{count}\t{}\t{}\t{}\t{}\t{}", s.size, s.mtime, s.hash, s.at, u8::from(s.cut))
}

pub fn state_file(state: &Path, sid: &str) -> PathBuf {
    state.join("reads").join(format!("{:016x}.tsv", crate::hook::fnv(&[sid])))
}

pub fn load(file: &Path) -> Table {
    let mut table = Table::default();
    fs::read_to_string(file).unwrap_or_default().lines().for_each(|line| table.apply(line));
    table
}

fn open_state(file: &Path) -> Option<File> {
    let open = || OpenOptions::new().create(true).truncate(false).read(true).write(true).open(file);
    let handle = match open() {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            if let Some(dir) = file.parent() {
                let _ = fs::create_dir_all(dir);
                sweep(dir);
            }
            open()
        }
        other => other,
    };
    let handle = handle.ok()?;
    for _ in 0..LOCK_TRIES {
        if handle.try_lock().is_ok() {
            return Some(handle);
        }
        thread::sleep(LOCK_PAUSE);
    }
    None
}

pub fn append(file: &Path, lines: &[String]) {
    let Some(mut f) = open_state(file) else { return };
    let mut text = lines.join("\n");
    text.push('\n');
    let Ok(size) = f.seek(SeekFrom::End(0)) else { return };
    if f.write_all(text.as_bytes()).is_err() || size + (text.len() as u64) < SHRINK_MIN_BYTES {
        return;
    }
    let mut all = String::new();
    if f.seek(SeekFrom::Start(0)).is_err() || f.read_to_string(&mut all).is_err() {
        return;
    }
    if all.lines().count() <= STATE_MAX_LINES {
        return;
    }
    let mut table = Table::default();
    all.lines().for_each(|line| table.apply(line));
    table.trim();
    let mut small = table.lines().join("\n");
    small.push('\n');
    if f.seek(SeekFrom::Start(0)).is_ok() && f.write_all(small.as_bytes()).is_ok() {
        let _ = f.set_len(small.len() as u64);
    }
}

pub fn sweep(dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let old = e.metadata().and_then(|m| m.modified()).ok().and_then(|t| t.elapsed().ok());
        if old.is_some_and(|age| age > STATE_KEEP) {
            let _ = fs::remove_file(e.path());
        }
    }
}

pub fn now_s() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

pub fn stat(path: &str) -> Option<(u64, u64)> {
    let meta = fs::metadata(path).ok()?;
    if !meta.is_file() {
        return None;
    }
    let mtime = meta.modified().ok()?.duration_since(UNIX_EPOCH).ok()?.as_millis() as u64;
    Some((meta.len(), mtime))
}

pub fn normal(path: &str) -> String {
    let slashed = path.replace('\\', "/");
    if cfg!(windows) { slashed.to_lowercase() } else { slashed }
}

pub fn valid(path: &str) -> bool {
    !path.is_empty() && !path.contains(['\t', '\n'])
}
