use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::SystemTime;

use serde_json::Value;

use super::super::routines;

pub type Mark = Option<(SystemTime, u64)>;
pub type DirMark = Vec<(PathBuf, SystemTime, u64)>;

pub struct Slot<K, T>(Mutex<Option<(PathBuf, K, Arc<T>)>>);

impl<K: PartialEq, T> Slot<K, T> {
    pub const fn new() -> Self {
        Self(Mutex::new(None))
    }

    pub fn get(&self, home: &Path, key: K, build: impl FnOnce() -> T) -> Arc<T> {
        let mut held = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some((h, k, v)) = held.as_ref()
            && h == home
            && *k == key
        {
            return Arc::clone(v);
        }
        let fresh = Arc::new(build());
        *held = Some((home.to_path_buf(), key, Arc::clone(&fresh)));
        fresh
    }

    pub fn clear(&self) {
        *self.0.lock().unwrap_or_else(PoisonError::into_inner) = None;
    }
}

pub fn mark(path: &Path) -> Mark {
    let m = fs::metadata(path).ok()?;
    Some((m.modified().ok()?, m.len()))
}

pub fn dir_mark(dir: &Path) -> DirMark {
    let Ok(entries) = fs::read_dir(dir) else { return Vec::new() };
    let mut all: DirMark = entries
        .flatten()
        .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
        .filter_map(|e| {
            let m = e.metadata().ok()?;
            Some((e.path(), m.modified().ok()?, m.len()))
        })
        .collect();
    all.sort();
    all
}

pub struct Same(pub Arc<Vec<Value>>);

impl PartialEq for Same {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

static ROUTINES: Slot<DirMark, Vec<Value>> = Slot::new();

pub fn routines(home: &Path) -> Arc<Vec<Value>> {
    ROUTINES.get(home, dir_mark(&home.join("routines")), || routines::load_all(home))
}
