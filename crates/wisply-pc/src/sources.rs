use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, UNIX_EPOCH};

use crate::guard::path_of;
use crate::plat::shell;
use crate::walk::mtime;

const DAY: u64 = 86_400;
const GIT_WAIT: Duration = Duration::from_secs(8);
const BASES: [&str; 4] = ["origin/main", "origin/master", "main", "master"];
const PROFILE_CACHES: [&str; 3] = ["Cache", "Code Cache", "GPUCache"];
const MAVEN_DEPTH: usize = 12;

fn kids(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir).map(|r| r.flatten().map(|e| e.path()).collect()).unwrap_or_default()
}

fn profiles(data: &Path, tool: &'static str, out: &mut Vec<(PathBuf, &'static str)>) {
    for profile in kids(data) {
        let name = profile.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        if name == "Default" || name.starts_with("Profile ") || profile.join("Preferences").is_file() {
            for cache in PROFILE_CACHES {
                let dir = profile.join(cache);
                if dir.is_dir() {
                    out.push((dir, tool));
                }
            }
        }
    }
}

pub fn browsers(user: &Path, local: Option<&Path>) -> Vec<(PathBuf, &'static str)> {
    let mut out = Vec::new();
    if cfg!(windows) {
        let base = local.map_or_else(|| path_of(user, &["AppData", "Local"]), Path::to_path_buf);
        let chromium = [
            (path_of(&base, &["BraveSoftware", "Brave-Browser", "User Data"]), "brave"),
            (path_of(&base, &["Google", "Chrome", "User Data"]), "chrome"),
            (path_of(&base, &["Microsoft", "Edge", "User Data"]), "edge"),
        ];
        for (data, tool) in chromium {
            profiles(&data, tool, &mut out);
        }
        for p in kids(&path_of(&base, &["Mozilla", "Firefox", "Profiles"])) {
            out.push((p.join("cache2"), "firefox"));
        }
    } else if cfg!(target_os = "macos") {
        let caches = path_of(user, &["Library", "Caches"]);
        out.push((path_of(&caches, &["BraveSoftware", "Brave-Browser"]), "brave"));
        out.push((path_of(&caches, &["Google", "Chrome"]), "chrome"));
        out.push((path_of(&caches, &["Microsoft Edge"]), "edge"));
        out.push((path_of(&caches, &["Firefox", "Profiles"]), "firefox"));
    } else {
        let caches = path_of(user, &[".cache"]);
        out.push((path_of(&caches, &["BraveSoftware", "Brave-Browser"]), "brave"));
        out.push((path_of(&caches, &["google-chrome"]), "chrome"));
        out.push((path_of(&caches, &["chromium"]), "chrome"));
        out.push((path_of(&caches, &["microsoft-edge"]), "edge"));
        out.push((path_of(&caches, &["mozilla", "firefox"]), "firefox"));
    }
    out.retain(|(p, _)| p.is_dir());
    out
}

fn used_at(version: &Path) -> u64 {
    let mut newest = 0;
    for f in kids(version) {
        let Ok(meta) = fs::symlink_metadata(&f) else { continue };
        newest = newest.max(mtime(&meta));
        if let Some(at) = meta.accessed().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()) {
            newest = newest.max(at.as_secs());
        }
        if meta.is_dir() {
            newest = newest.max(crate::walk::weigh(&f, &[]).newest);
        }
    }
    newest
}

fn is_version(dir: &Path) -> bool {
    kids(dir).iter().any(|f| {
        let name = f.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
        f.is_file() && (name.ends_with(".pom") || name.ends_with(".jar"))
    })
}

fn maven_walk(dir: &Path, depth: usize, now: u64, days: u64, out: &mut Vec<PathBuf>) {
    if depth > MAVEN_DEPTH {
        return;
    }
    if is_version(dir) {
        if now.saturating_sub(used_at(dir)) >= days * DAY {
            out.push(dir.to_path_buf());
        }
        return;
    }
    for child in kids(dir) {
        if fs::symlink_metadata(&child).is_ok_and(|m| m.is_dir() && !m.file_type().is_symlink()) {
            maven_walk(&child, depth + 1, now, days, out);
        }
    }
}

pub fn maven_old(repo: &Path, now: u64, days: u64) -> Vec<PathBuf> {
    let mut out = Vec::new();
    maven_walk(repo, 0, now, days, &mut out);
    out
}

fn git(dir: &Path, args: &[&str]) -> Option<String> {
    let path = dir.to_string_lossy();
    let mut all = vec!["-C", path.as_ref()];
    all.extend_from_slice(args);
    shell::run("git", &all, GIT_WAIT)
}

pub fn tracked(project: &Path, rel: &str) -> bool {
    match git(project, &["ls-files", "--", rel]) {
        Some(out) => !out.trim().is_empty(),
        None => project.join(".git").exists(),
    }
}

pub fn unmerged_worktree(project: &Path) -> bool {
    if !project.join(".git").is_file() {
        return false;
    }
    for base in BASES {
        if git(project, &["rev-parse", "--verify", "-q", base]).is_some() {
            let range = format!("{base}..HEAD");
            return git(project, &["rev-list", "--count", &range])
                .and_then(|n| n.trim().parse::<u64>().ok())
                .is_none_or(|n| n > 0);
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;

    fn lab(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("wpc-src-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    fn old(path: &Path, days: u64) {
        let when = SystemTime::now() - Duration::from_secs(days * DAY);
        let mut opts = fs::File::options();
        opts.read(true);
        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            opts.access_mode(0x100).custom_flags(0x0200_0000);
        }
        if let Ok(h) = opts.open(path) {
            let _ = h.set_modified(when);
        }
    }

    #[test]
    fn only_maven_versions_unused_for_long_are_listed() {
        let root = lab("mvn");
        for v in ["1.0", "2.0"] {
            let dir = root.join("org/acme/lib").join(v);
            fs::create_dir_all(&dir).unwrap();
            fs::write(dir.join(format!("lib-{v}.jar")), b"jar").unwrap();
            fs::write(dir.join(format!("lib-{v}.pom")), b"pom").unwrap();
        }
        let ancient = root.join("org/acme/lib/1.0");
        old(&ancient.join("lib-1.0.jar"), 90);
        old(&ancient.join("lib-1.0.pom"), 90);
        let now = crate::model::secs();
        let got = maven_old(&root, now + 100 * DAY, 60);
        assert_eq!(got.len(), 2);
        let got = maven_old(&root, now, 60);
        assert!(got.is_empty() || got == [ancient.clone()], "{got:?}");
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn browser_caches_are_found_per_profile_and_empty_without_a_browser() {
        let root = lab("br");
        assert!(browsers(&root, Some(&root)).is_empty());
        if cfg!(windows) {
            let cache = root.join("BraveSoftware/Brave-Browser/User Data/Default/Cache");
            fs::create_dir_all(&cache).unwrap();
            fs::create_dir_all(root.join("BraveSoftware/Brave-Browser/User Data/Default/Local Storage")).unwrap();
            let got = browsers(&root, Some(&root));
            assert_eq!(got, [(cache, "brave")]);
        }
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_folder_without_git_is_never_tracked_or_an_unmerged_worktree() {
        let root = lab("nogit");
        assert!(!unmerged_worktree(&root));
        assert!(!tracked(&root, "target") || root.ancestors().any(|a| a.join(".git").exists()));
        fs::remove_dir_all(&root).unwrap();
    }
}
