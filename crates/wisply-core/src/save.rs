use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::Duration;

const SWAP_TRIES: u32 = if cfg!(windows) { 20 } else { 1 };
const SWAP_PAUSE: Duration = Duration::from_millis(5);

static NEXT: AtomicU32 = AtomicU32::new(0);

fn linked(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok_and(|m| m.file_type().is_symlink())
}

fn spare(path: &Path) -> PathBuf {
    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    path.with_file_name(format!(".{name}.{}.{n}.tmp", std::process::id()))
}

fn fill(tmp: &Path, path: &Path, data: &[u8]) -> io::Result<()> {
    let mut f = File::create(tmp)?;
    if cfg!(unix)
        && let Ok(meta) = fs::metadata(path)
    {
        f.set_permissions(meta.permissions())?;
    }
    f.write_all(data)?;
    f.sync_data()
}

fn swap(tmp: &Path, path: &Path) -> io::Result<()> {
    let mut last = fs::rename(tmp, path);
    for _ in 1..SWAP_TRIES {
        if last.is_ok() {
            break;
        }
        thread::sleep(SWAP_PAUSE);
        last = fs::rename(tmp, path);
    }
    last
}

pub fn save(path: impl AsRef<Path>, data: impl AsRef<[u8]>) -> io::Result<()> {
    let (path, data) = (path.as_ref(), data.as_ref());
    let real = if linked(path) { fs::canonicalize(path).ok() } else { Some(path.to_path_buf()) };
    let Some(real) = real else { return fs::write(path, data) };
    if let Some(dir) = real.parent().filter(|d| !d.as_os_str().is_empty()) {
        fs::create_dir_all(dir)?;
    }
    let tmp = spare(&real);
    if fill(&tmp, &real, data).and_then(|()| swap(&tmp, &real)).is_ok() {
        return Ok(());
    }
    let _ = fs::remove_file(&tmp);
    fs::write(&real, data)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;

    use super::*;

    fn room(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("wisply-save-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    fn names(dir: &Path) -> Vec<String> {
        let mut all: Vec<String> =
            fs::read_dir(dir).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
        all.sort();
        all
    }

    #[test]
    fn a_save_makes_the_folder_replaces_the_file_and_leaves_nothing_behind() {
        let dir = room("plain");
        let file = dir.join("deep").join("state.json");
        save(&file, "one").unwrap();
        save(&file, b"two").unwrap();
        assert_eq!(fs::read_to_string(&file).unwrap(), "two");
        assert_eq!(names(&dir.join("deep")), ["state.json"]);
        assert!(save(dir.join("deep"), "x").is_err());
        assert_eq!(names(&dir), ["deep"]);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_reader_never_sees_half_a_file_while_others_save() {
        const SIZE: usize = 256 * 1024;
        const ROUNDS: usize = 40;
        let dir = room("race");
        let file = dir.join("state.json");
        save(&file, vec![b'a'; SIZE]).unwrap();
        let stop = Arc::new(AtomicBool::new(false));
        let reader = {
            let (file, stop) = (file.clone(), Arc::clone(&stop));
            thread::spawn(move || {
                let mut seen = 0usize;
                while !stop.load(Ordering::Relaxed) {
                    if let Ok(body) = fs::read(&file) {
                        assert_eq!(body.len(), SIZE, "a reader saw a short file");
                        assert!(body.iter().all(|b| *b == body[0]), "a reader saw a mixed file");
                        seen += 1;
                    }
                }
                seen
            })
        };
        let writers: Vec<_> = (*b"bcd")
            .into_iter()
            .map(|letter| {
                let file = file.clone();
                thread::spawn(move || {
                    for _ in 0..ROUNDS {
                        save(&file, vec![letter; SIZE]).unwrap();
                    }
                })
            })
            .collect();
        for w in writers {
            w.join().unwrap();
        }
        stop.store(true, Ordering::Relaxed);
        assert!(reader.join().unwrap() > 0);
        assert_eq!(names(&dir), ["state.json"]);
        let _ = fs::remove_dir_all(&dir);
    }

    #[cfg(unix)]
    #[test]
    fn a_linked_file_stays_a_link_and_keeps_its_rights() {
        use std::os::unix::fs::PermissionsExt;
        let dir = room("link");
        fs::create_dir_all(dir.join("real")).unwrap();
        let real = dir.join("real").join("settings.json");
        fs::write(&real, "old").unwrap();
        fs::set_permissions(&real, fs::Permissions::from_mode(0o600)).unwrap();
        let link = dir.join("settings.json");
        std::os::unix::fs::symlink(&real, &link).unwrap();
        save(&link, "new").unwrap();
        assert!(linked(&link));
        assert_eq!(fs::read_to_string(&real).unwrap(), "new");
        assert_eq!(fs::metadata(&real).unwrap().permissions().mode() & 0o777, 0o600);
        let _ = fs::remove_dir_all(&dir);
    }
}
