use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::guard::{Guard, inside, norm};
use crate::model::{Disk, GB, gb, size};

const EXPAND: [&str; 10] = [
    "appdata",
    "local",
    "roaming",
    "locallow",
    "library",
    "application support",
    "caches",
    ".cache",
    ".local",
    "share",
];
const EXPAND_DEPTH: usize = 3;
const TOP: usize = 8;
const MOVES: usize = 3;
const ROOM: f64 = 1.25;
const MOVED_DIR: &str = "wisply-moved";
const SEPARATORS: [char; 2] = ['/', '\\'];

#[derive(Clone, Debug, PartialEq)]
pub struct Chunk {
    pub path: PathBuf,
    pub bytes: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Scan {
    pub chunks: Vec<Chunk>,
    pub complete: bool,
    pub secs: u64,
}

pub fn same_mount(a: &str, b: &str) -> bool {
    a.trim_end_matches(SEPARATORS).eq_ignore_ascii_case(b.trim_end_matches(SEPARATORS))
}

pub fn system_disk<'a>(disks: &'a [Disk], user: &Path) -> Option<&'a Disk> {
    let home = norm(user);
    disks
        .iter()
        .filter(|d| d.total > 0)
        .filter(|d| {
            let m = norm(Path::new(&d.mount));
            m.is_empty() || inside(&home, &m)
        })
        .max_by_key(|d| norm(Path::new(&d.mount)).len())
        .or_else(|| disks.iter().filter(|d| d.total > 0).min_by_key(|d| d.free))
}

fn is_dir(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok_and(|m| m.is_dir() && !m.file_type().is_symlink())
}

fn kids(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir).map(|r| r.flatten().map(|e| e.path()).filter(|p| is_dir(p)).collect()).unwrap_or_default()
}

fn expand(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    for child in kids(dir) {
        let name = child.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
        if depth < EXPAND_DEPTH && EXPAND.contains(&name.as_str()) {
            expand(&child, depth + 1, out);
        } else {
            out.push(child);
        }
    }
}

pub fn system_roots(user: &Path) -> Vec<PathBuf> {
    let home = norm(user);
    match user.ancestors().last().filter(|r| norm(r) != home) {
        Some(root) => kids(root).into_iter().filter(|c| !inside(&home, &norm(c))).collect(),
        None => Vec::new(),
    }
}

fn candidates(user: &Path, roots: &[PathBuf]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    expand(user, 0, &mut out);
    out.extend(roots.iter().cloned());
    out
}

fn measure(dir: &Path, end: Instant, whole: &mut bool) -> u64 {
    if Instant::now() >= end {
        *whole = false;
        return 0;
    }
    let Ok(entries) = fs::read_dir(dir) else { return 0 };
    let mut total = 0;
    for e in entries.flatten() {
        let Ok(meta) = fs::symlink_metadata(e.path()) else { continue };
        if meta.file_type().is_symlink() {
            continue;
        }
        total += if meta.is_dir() { measure(&e.path(), end, whole) } else { meta.len() };
    }
    total
}

pub fn scan(user: &Path, roots: &[PathBuf], budget: Duration) -> Scan {
    let started = Instant::now();
    let end = started + budget;
    let mut whole = true;
    let mut chunks: Vec<Chunk> = candidates(user, roots)
        .into_iter()
        .map(|path| Chunk { bytes: measure(&path, end, &mut whole), path })
        .collect();
    chunks.sort_by_key(|c| std::cmp::Reverse(c.bytes));
    chunks.truncate(TOP);
    Scan { chunks, complete: whole, secs: started.elapsed().as_secs() }
}

pub fn shorten(path: &Path, user: &Path) -> String {
    match path.strip_prefix(user) {
        Ok(rest) => format!("~{}{}", std::path::MAIN_SEPARATOR, rest.display()),
        Err(_) => path.display().to_string(),
    }
}

pub fn alert(disk: &Disk, scan: &Scan, user: &Path) -> String {
    let head = format!("{} only {} free", disk.mount, gb(disk.free));
    match scan.chunks.first() {
        Some(c) => format!("{head} - biggest: {} {}", shorten(&c.path, user), gb(c.bytes)),
        None => head,
    }
}

fn target(other: &Disk, path: &Path) -> String {
    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let sep = if cfg!(windows) { '\\' } else { '/' };
    let base = other.mount.trim_end_matches(SEPARATORS);
    format!("{base}{sep}{MOVED_DIR}{sep}{name}")
}

fn move_line(path: &Path, other: &Disk) -> String {
    let to = target(other, path);
    let from = path.display();
    if cfg!(windows) {
        format!("robocopy \"{from}\" \"{to}\" /E /MOVE /COPY:DAT && mklink /J \"{from}\" \"{to}\"")
    } else {
        format!("mkdir -p \"{to}\" && mv \"{from}\" \"{to}\" && ln -s \"{to}\" \"{from}\"")
    }
}

pub fn moves(scan: &Scan, disks: &[Disk], user: &Path, g: &Guard) -> Vec<String> {
    let Some(system) = system_disk(disks, user) else { return Vec::new() };
    let mut out = Vec::new();
    for c in &scan.chunks {
        let own = inside(&norm(&c.path), &norm(user)) && g.blocked(&c.path).is_none() && c.bytes >= GB;
        let other = disks
            .iter()
            .filter(|d| d.total > 0 && !same_mount(&d.mount, &system.mount) && d.free as f64 >= c.bytes as f64 * ROOM)
            .max_by_key(|d| d.free);
        if let (true, Some(other)) = (own, other) {
            out.push(format!(
                "move {} ({}) to {}: {}",
                shorten(&c.path, user),
                gb(c.bytes),
                other.mount,
                move_line(&c.path, other)
            ));
        }
        if out.len() >= MOVES {
            break;
        }
    }
    out
}

pub struct Limits {
    pub warn_gb: u64,
    pub clean_gb: u64,
    pub freeable: u64,
}

pub fn render(disks: &[Disk], scan: &Scan, user: &Path, g: &Guard, lim: &Limits) -> String {
    let mut lines = Vec::new();
    if let Some(d) = system_disk(disks, user) {
        lines.push(format!(
            "pc disk | {} {} free of {} (warning below {} GB, self-clean below {} GB)",
            d.mount,
            gb(d.free),
            gb(d.total),
            lim.warn_gb,
            lim.clean_gb
        ));
    }
    let note = if scan.complete { "" } else { " (stopped early, sizes are lower bounds)" };
    lines.push(format!("biggest folders, scanned for {} s{note}:", scan.secs));
    for (i, c) in scan.chunks.iter().enumerate() {
        lines.push(format!("{}. {} | {}", i + 1, gb(c.bytes), shorten(&c.path, user)));
    }
    if lim.freeable > 0 {
        lines.push(format!(
            "wisply pc clean would free {} safely; look first with wisply pc clean --dry",
            size(lim.freeable)
        ));
    }
    lines.extend(moves(scan, disks, user, g).into_iter().map(|m| format!("idea: {m}")));
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn lab(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("wpc-disk-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn disk(mount: &str, free: u64) -> Disk {
        Disk { mount: mount.into(), total: 1000 * GB, free }
    }

    #[test]
    fn the_system_drive_is_the_one_holding_the_home_folder() {
        let disks = [disk("C:\\", 10 * GB), disk("D:\\", 500 * GB)];
        assert_eq!(system_disk(&disks, Path::new("C:\\Users\\a")).unwrap().mount, "C:\\");
        let unix = [disk("/", 10 * GB), disk("/mnt/x", 500 * GB), disk("/home", 20 * GB)];
        assert_eq!(system_disk(&unix, Path::new("/home/a")).unwrap().mount, "/home");
        assert_eq!(system_disk(&unix, Path::new("/root")).unwrap().mount, "/");
    }

    #[test]
    fn the_scan_ranks_folders_by_size_and_looks_inside_appdata() {
        let user = lab("scan");
        let put = |rel: &str, n: usize| {
            let p = user.join(rel);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, vec![1u8; n]).unwrap();
        };
        put("AppData/Local/Big/a.bin", 5000);
        put("AppData/Local/Small/a.bin", 10);
        put("work/b.bin", 300);
        put("loose.txt", 99999);
        let s = scan(&user, &[], Duration::from_secs(30));
        let names: Vec<String> =
            s.chunks.iter().map(|c| c.path.file_name().unwrap().to_string_lossy().into_owned()).collect();
        assert_eq!(&names[..3], ["Big", "work", "Small"], "{names:?}");
        assert!(s.complete && s.chunks[0].bytes == 5000);
        fs::remove_dir_all(&user).unwrap();
    }

    #[test]
    fn an_exhausted_budget_is_reported_as_incomplete() {
        let user = lab("late");
        fs::create_dir_all(user.join("a")).unwrap();
        fs::write(user.join("a/f"), [1u8; 10]).unwrap();
        assert!(!scan(&user, &[], Duration::ZERO).complete);
        fs::remove_dir_all(&user).unwrap();
    }

    #[test]
    fn only_big_unprotected_home_folders_get_a_move_idea_and_only_to_a_roomy_drive() {
        let user = lab("move");
        let g = Guard::new(&user, &user.join(".wisply"), &Config::default());
        let scan = Scan {
            chunks: vec![
                Chunk { path: user.join("data"), bytes: 20 * GB },
                Chunk { path: user.join(".minecraft"), bytes: 30 * GB },
                Chunk { path: user.join("tiny"), bytes: 1000 },
                Chunk { path: user.join("huge"), bytes: 900 * GB },
            ],
            complete: true,
            secs: 1,
        };
        let sys = user.ancestors().last().unwrap().to_string_lossy().into_owned();
        let disks = [disk(&sys, 5 * GB), disk("Z:\\", 100 * GB)];
        let m = moves(&scan, &disks, &user, &g);
        assert_eq!(m.len(), 1, "{m:?}");
        assert!(m[0].contains("data") && m[0].contains("Z:") && !m[0].contains("minecraft"), "{m:?}");
        fs::remove_dir_all(&user).unwrap();
    }

    #[test]
    fn the_alert_names_the_biggest_chunk_relative_to_home() {
        let user = PathBuf::from("/h/u");
        let scan = Scan { chunks: vec![Chunk { path: user.join("x"), bytes: 3 * GB }], complete: true, secs: 0 };
        let text = alert(&disk("C:", 18 * GB), &scan, &user);
        assert!(text.starts_with("C: only 18.0 GB free - biggest: ~") && text.ends_with("x 3.0 GB"), "{text}");
        assert_eq!(alert(&disk("C:", GB), &Scan::default(), &user), "C: only 1.0 GB free");
    }
}
