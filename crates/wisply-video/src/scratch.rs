use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime};

use crate::fail::{Fail, Kind};

const PREFIX: &str = "wisply-";

static NEXT: AtomicU64 = AtomicU64::new(0);

pub struct Scratch {
    path: PathBuf,
}

fn old(path: &Path, max_age: Duration) -> bool {
    let changed = fs::metadata(path).and_then(|meta| meta.modified()).ok();
    changed.and_then(|at| SystemTime::now().duration_since(at).ok()).is_some_and(|age| age > max_age)
}

fn sweep(root: &Path, max_age: Duration) {
    let Ok(entries) = fs::read_dir(root) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if entry.file_name().to_string_lossy().starts_with(PREFIX) && path.is_dir() && old(&path, max_age) {
            let _ = fs::remove_dir_all(&path);
        }
    }
}

impl Scratch {
    pub fn new(root: &Path, max_age_seconds: f64) -> Result<Scratch, Fail> {
        sweep(root, Duration::from_secs_f64(max_age_seconds.max(0.0)));
        let name = format!("{PREFIX}{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed));
        let path = root.join(name);
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).map_err(|error| {
            Fail::new(Kind::Storage, format!("the folder {} cannot be created ({error})", path.display()))
        })?;
        Ok(Scratch { path })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_scratch_folder_is_fresh_and_gone_after_use() {
        let root = crate::fixture::folder("scratch-fresh");
        let (first, second) = (Scratch::new(&root, 60.0).unwrap(), Scratch::new(&root, 60.0).unwrap());
        assert_ne!(first.path(), second.path());
        fs::write(first.path().join("left.vtt"), "x").unwrap();
        let kept = first.path().to_path_buf();
        drop(first);
        assert!(!kept.exists() && second.path().is_dir());
        drop(second);
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn old_leftovers_are_swept_and_other_folders_stay() {
        let root = crate::fixture::folder("scratch-sweep");
        let (leftover, other) = (root.join(format!("{PREFIX}1-1")), root.join("keep"));
        fs::create_dir_all(&leftover).unwrap();
        fs::create_dir_all(&other).unwrap();
        drop(Scratch::new(&root, 3600.0).unwrap());
        assert!(leftover.is_dir());
        std::thread::sleep(Duration::from_millis(30));
        drop(Scratch::new(&root, 0.0).unwrap());
        assert!(!leftover.exists() && other.is_dir());
        fs::remove_dir_all(&root).unwrap();
    }
}
