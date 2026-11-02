use std::fs::{self, Metadata};
use std::path::Path;
use std::time::UNIX_EPOCH;

pub const LIMIT: usize = 200_000;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Weight {
    pub bytes: u64,
    pub newest: u64,
    pub complete: bool,
}

pub fn mtime(meta: &Metadata) -> u64 {
    meta.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map_or(0, |d| d.as_secs())
}

pub fn weigh(path: &Path, skip: &[&str]) -> Weight {
    let mut w = Weight { complete: true, ..Weight::default() };
    let mut seen = 0;
    add(path, skip, &mut w, &mut seen);
    w
}

fn add(path: &Path, skip: &[&str], w: &mut Weight, seen: &mut usize) {
    let Ok(meta) = fs::symlink_metadata(path) else { return };
    w.newest = w.newest.max(mtime(&meta));
    if !meta.is_dir() {
        w.bytes += meta.len();
        return;
    }
    if meta.file_type().is_symlink() {
        return;
    }
    let Ok(entries) = fs::read_dir(path) else { return };
    for e in entries.flatten() {
        *seen += 1;
        if *seen > LIMIT {
            w.complete = false;
            return;
        }
        let name = e.file_name().to_string_lossy().to_lowercase();
        if skip.contains(&name.as_str()) {
            continue;
        }
        add(&e.path(), skip, w, seen);
        if !w.complete {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_add_up_and_skipped_names_are_left_out() {
        let dir = std::env::temp_dir().join(format!("wpc-walk-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("sub")).unwrap();
        fs::create_dir_all(dir.join("target")).unwrap();
        fs::write(dir.join("a.txt"), [0u8; 100]).unwrap();
        fs::write(dir.join("sub/b.txt"), [0u8; 50]).unwrap();
        fs::write(dir.join("target/big.bin"), [0u8; 1000]).unwrap();
        let all = weigh(&dir, &[]);
        assert_eq!((all.bytes, all.complete), (1150, true));
        assert!(all.newest > 0);
        assert_eq!(weigh(&dir, &["target"]).bytes, 150);
        assert_eq!(weigh(&dir.join("missing"), &[]).bytes, 0);
        fs::remove_dir_all(&dir).unwrap();
    }
}
