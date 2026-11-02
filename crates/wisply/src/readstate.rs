use std::collections::{HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const STATE_MAX_LINES: usize = 400;
const STATE_KEEP: Duration = Duration::from_secs(7 * 86_400);

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

pub fn load(file: &Path) -> (Table, usize) {
    let mut table = Table::default();
    let text = fs::read_to_string(file).unwrap_or_default();
    let mut n = 0;
    for line in text.lines() {
        table.apply(line);
        n += 1;
    }
    (table, n)
}

pub fn append(file: &Path, lines: &[String]) {
    let open = || OpenOptions::new().create(true).append(true).open(file);
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
    if let Ok(mut f) = handle {
        let mut text = lines.join("\n");
        text.push('\n');
        let _ = f.write_all(text.as_bytes());
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

pub fn shrink(file: &Path, table: &Table, lines: usize) {
    if lines > STATE_MAX_LINES {
        let mut text = table.lines().join("\n");
        text.push('\n');
        let _ = fs::write(file, text);
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
