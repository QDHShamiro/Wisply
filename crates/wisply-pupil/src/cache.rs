use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

use crate::config::Cache;
use crate::video::Picture;

fn stamp(path: &Path) -> Option<(u64, u128)> {
    let meta = fs::metadata(path).ok()?;
    let modified = meta.modified().ok()?.duration_since(UNIX_EPOCH).ok()?;
    Some((meta.len(), modified.as_nanos()))
}

pub fn key(video: &Path, parts: &str) -> Option<String> {
    let (size, modified) = stamp(video)?;
    let (exe_size, exe_modified) = std::env::current_exe().ok().and_then(|exe| stamp(&exe)).unwrap_or_default();
    let path = fs::canonicalize(video).ok()?;
    let mut hash = Sha256::new();
    hash.update(path.to_string_lossy().as_bytes());
    hash.update(size.to_le_bytes());
    hash.update(modified.to_le_bytes());
    hash.update(exe_size.to_le_bytes());
    hash.update(exe_modified.to_le_bytes());
    hash.update(parts.as_bytes());
    Some(hash.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

fn file(settings: &Cache, key: &str) -> PathBuf {
    settings.dir.join(format!("{key}.bin"))
}

pub fn load(settings: &Cache, key: &str) -> Option<Vec<Picture>> {
    decode(&fs::read(file(settings, key)).ok()?)
}

pub fn save(settings: &Cache, key: &str, list: &[Picture]) {
    if fs::create_dir_all(&settings.dir).is_err() {
        return;
    }
    let target = file(settings, key);
    let temp = settings.dir.join(format!("{key}.{}.{:?}.tmp", std::process::id(), std::thread::current().id()));
    if fs::write(&temp, encode(list)).is_err() || fs::rename(&temp, &target).is_err() {
        let _ = fs::remove_file(&temp);
    }
    prune(settings);
}

fn prune(settings: &Cache) {
    let Ok(read) = fs::read_dir(&settings.dir) else {
        return;
    };
    let mut files: Vec<(SystemTime, PathBuf)> = read
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "bin"))
        .filter_map(|path| Some((fs::metadata(&path).ok()?.modified().ok()?, path)))
        .collect();
    if files.len() <= settings.max_entries {
        return;
    }
    files.sort_by_key(|a| std::cmp::Reverse(a.0));
    for (_, path) in files.into_iter().skip(settings.max_entries) {
        let _ = fs::remove_file(path);
    }
}

pub fn encode(list: &[Picture]) -> Vec<u8> {
    let size = list.iter().map(|p| p.jpeg.len() + 13).sum::<usize>() + 4;
    let mut out = Vec::with_capacity(size);
    out.extend_from_slice(&(list.len() as u32).to_le_bytes());
    for picture in list {
        out.extend_from_slice(&picture.at.to_le_bytes());
        out.push(u8::from(picture.small));
        out.extend_from_slice(&(picture.jpeg.len() as u32).to_le_bytes());
        out.extend_from_slice(&picture.jpeg);
    }
    out
}

struct Reader<'a> {
    raw: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, count: usize) -> Option<&'a [u8]> {
        let slice = self.raw.get(self.at..self.at.checked_add(count)?)?;
        self.at += count;
        Some(slice)
    }

    fn word(&mut self) -> Option<usize> {
        Some(u32::from_le_bytes(self.take(4)?.try_into().ok()?) as usize)
    }
}

pub fn decode(raw: &[u8]) -> Option<Vec<Picture>> {
    let mut reader = Reader { raw, at: 0 };
    let count = reader.word()?;
    let mut list = Vec::with_capacity(count.min(1024));
    for _ in 0..count {
        let at = f64::from_le_bytes(reader.take(8)?.try_into().ok()?);
        let small = reader.take(1)?[0] != 0;
        let length = reader.word()?;
        let jpeg = reader.take(length)?.to_vec();
        list.push(Picture { at, small, jpeg });
    }
    (reader.at == raw.len() && !list.is_empty()).then_some(list)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pictures() -> Vec<Picture> {
        vec![
            Picture { at: 1.5, small: false, jpeg: vec![0xFF, 0xD8, 1, 2, 3, 0xFF, 0xD9] },
            Picture { at: 7.25, small: true, jpeg: vec![0xFF, 0xD8, 9, 0xFF, 0xD9] },
        ]
    }

    fn settings(name: &str, max_entries: usize) -> Cache {
        let dir = std::env::temp_dir().join(format!("pupil-cache-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        Cache { enabled: true, dir, max_entries }
    }

    #[test]
    fn saved_pictures_come_back_bit_for_bit() {
        let list = pictures();
        let back = decode(&encode(&list)).expect("decodes");
        assert_eq!(back.len(), list.len());
        for (a, b) in back.iter().zip(&list) {
            assert_eq!(a.at.to_bits(), b.at.to_bits());
            assert_eq!(a.small, b.small);
            assert_eq!(a.jpeg, b.jpeg);
        }
    }

    #[test]
    fn broken_or_empty_files_are_ignored() {
        let raw = encode(&pictures());
        assert!(decode(&raw[..raw.len() - 1]).is_none());
        let mut longer = raw.clone();
        longer.push(0);
        assert!(decode(&longer).is_none());
        assert!(decode(&encode(&[])).is_none());
        assert!(decode(&[]).is_none());
    }

    #[test]
    fn the_key_follows_the_video_and_the_settings() {
        let cache = settings("key", 4);
        fs::create_dir_all(&cache.dir).expect("dir");
        let video = cache.dir.join("clip.mp4");
        fs::write(&video, b"one").expect("write");
        let first = key(&video, "a").expect("key");
        assert_eq!(key(&video, "a"), Some(first.clone()));
        assert_ne!(key(&video, "b"), Some(first.clone()));
        fs::write(&video, b"other").expect("write");
        assert_ne!(key(&video, "a"), Some(first));
        assert!(key(&cache.dir.join("missing.mp4"), "a").is_none());
        let _ = fs::remove_dir_all(&cache.dir);
    }

    #[test]
    fn save_then_load_and_keep_only_the_newest() {
        let cache = settings("prune", 2);
        for name in ["k1", "k2", "k3"] {
            save(&cache, name, &pictures());
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        assert!(load(&cache, "k1").is_none());
        assert_eq!(load(&cache, "k3").map(|l| l.len()), Some(2));
        let left = fs::read_dir(&cache.dir).expect("dir").count();
        assert_eq!(left, 2);
        let _ = fs::remove_dir_all(&cache.dir);
    }
}
