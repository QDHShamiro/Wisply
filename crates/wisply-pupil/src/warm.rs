use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::door;
use crate::watch;

fn listed(root: &Path) -> Vec<PathBuf> {
    if root.is_file() {
        return vec![root.to_path_buf()];
    }
    let Ok(entries) = std::fs::read_dir(root) else {
        return Vec::new();
    };
    let mut files: Vec<PathBuf> =
        entries.filter_map(Result::ok).map(|entry| entry.path()).filter(|path| path.is_file()).collect();
    files.sort();
    files
}

pub fn files(roots: &[PathBuf]) -> Vec<PathBuf> {
    roots.iter().flat_map(|root| listed(root.as_path())).collect()
}

pub fn read(files: &[PathBuf]) -> u64 {
    files
        .iter()
        .filter_map(|path| File::open(path).ok())
        .filter_map(|mut file| io::copy(&mut file, &mut io::sink()).ok())
        .sum()
}

pub fn start(config: &Config, afk: bool) -> bool {
    let talk = &config.talk;
    let quiet = watch::game_failure(config, afk).is_none() && door::held(talk).is_none();
    if talk.warm.is_empty() || !quiet {
        return false;
    }
    let roots = talk.warm.clone();
    std::thread::spawn(move || read(&files(&roots)));
    true
}

#[cfg(test)]
mod tests {
    use super::{files, read};

    #[test]
    fn folders_files_and_missing_paths_are_read_once_through() {
        let root = std::env::temp_dir().join(format!("pupil-warm-{}", std::process::id()));
        let inner = root.join("inner");
        std::fs::create_dir_all(&inner).expect("folder");
        std::fs::write(root.join("b.bin"), vec![1u8; 3000]).expect("file");
        std::fs::write(root.join("a.bin"), vec![2u8; 500]).expect("file");
        std::fs::write(inner.join("deep.bin"), vec![3u8; 70]).expect("file");
        let single = inner.join("deep.bin");
        let roots = vec![root.clone(), root.join("gone"), single.clone()];
        let found = files(&roots);
        assert_eq!(found, vec![root.join("a.bin"), root.join("b.bin"), single]);
        assert_eq!(read(&found), 3570);
        assert_eq!(read(&[root.join("gone")]), 0);
        assert!(files(&[]).is_empty());
        std::fs::remove_dir_all(&root).expect("clean up");
    }
}
