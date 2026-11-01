use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use super::proc::capture;

const SHORT_SHA: usize = 7;
const LIMIT: Duration = Duration::from_secs(15);

#[derive(Clone, Debug, PartialEq)]
pub struct Repo {
    pub root: PathBuf,
    pub head: PathBuf,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Head {
    Branch(String),
    Detached(String),
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Sync {
    pub dirty: bool,
    pub ahead: u32,
    pub behind: u32,
}

impl Head {
    pub fn label(&self) -> &str {
        match self {
            Head::Branch(name) | Head::Detached(name) => name,
        }
    }

    pub fn branch(&self) -> Option<&str> {
        match self {
            Head::Branch(name) => Some(name),
            Head::Detached(_) => None,
        }
    }
}

pub fn find(start: &Path) -> Option<Repo> {
    start.ancestors().find_map(|dir| {
        let dot = dir.join(".git");
        if dot.is_dir() {
            return Some(Repo { root: dir.to_path_buf(), head: dot.join("HEAD") });
        }
        let text = fs::read_to_string(&dot).ok()?;
        let target = text.lines().find_map(|l| l.strip_prefix("gitdir:"))?.trim();
        let git_dir = if Path::new(target).is_absolute() { PathBuf::from(target) } else { dir.join(target) };
        Some(Repo { root: dir.to_path_buf(), head: git_dir.join("HEAD") })
    })
}

pub fn read_head(head: &Path) -> Option<Head> {
    let text = fs::read_to_string(head).ok()?;
    let line = text.lines().next()?.trim();
    if let Some(reference) = line.strip_prefix("ref:") {
        let reference = reference.trim();
        let name = reference.strip_prefix("refs/heads/").unwrap_or(reference);
        return (!name.is_empty()).then(|| Head::Branch(name.to_string()));
    }
    let hex = line.len() >= SHORT_SHA && line.bytes().all(|b| b.is_ascii_hexdigit());
    hex.then(|| Head::Detached(line[..SHORT_SHA].to_string()))
}

pub fn parse_status(text: &str) -> Sync {
    let mut sync = Sync::default();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("# branch.ab ") {
            let mut words = rest.split_whitespace();
            sync.ahead = words.next().and_then(|w| w.strip_prefix('+')).and_then(|n| n.parse().ok()).unwrap_or(0);
            sync.behind = words.next().and_then(|w| w.strip_prefix('-')).and_then(|n| n.parse().ok()).unwrap_or(0);
        } else if !line.starts_with('#') && !line.trim().is_empty() {
            sync.dirty = true;
        }
    }
    sync
}

fn git(root: &Path) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(root).arg("--no-optional-locks").env("GIT_OPTIONAL_LOCKS", "0");
    cmd
}

pub fn run_status(root: &Path) -> Option<Sync> {
    let done = capture(git(root).args(["status", "--porcelain=v2", "--branch"]), LIMIT)?;
    done.ok.then(|| parse_status(&done.out))
}

pub fn remote_repo(root: &Path) -> Option<String> {
    let done = capture(git(root).args(["config", "--get", "remote.origin.url"]), LIMIT)?;
    done.ok.then(|| parse_remote(done.out.trim())).flatten()
}

pub fn parse_remote(url: &str) -> Option<String> {
    let tail = url.trim().trim_end_matches('/').trim_end_matches(".git");
    let path = match tail.split_once("://") {
        Some((_, rest)) => rest.split_once('/')?.1,
        None => tail.split_once(':')?.1,
    };
    let mut parts = path.rsplit('/');
    let name = parts.next().filter(|p| !p.is_empty())?;
    let owner = parts.next().filter(|p| !p.is_empty())?;
    Some(format!("{owner}/{name}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("lt-sl-git-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn head_reads_branches_detached_and_worktrees() {
        let d = dir("head");
        let repo = d.join("repo");
        fs::create_dir_all(repo.join(".git")).unwrap();
        fs::create_dir_all(repo.join("src/deep")).unwrap();
        fs::write(repo.join(".git/HEAD"), "ref: refs/heads/feat/x\n").unwrap();
        let found = find(&repo.join("src/deep")).unwrap();
        assert_eq!(found.root, repo);
        assert_eq!(read_head(&found.head), Some(Head::Branch("feat/x".into())));
        fs::write(repo.join(".git/HEAD"), "0123456789abcdef0123456789abcdef01234567\n").unwrap();
        assert_eq!(read_head(&found.head), Some(Head::Detached("0123456".into())));
        fs::write(repo.join(".git/HEAD"), "junk\n").unwrap();
        assert_eq!(read_head(&found.head), None);
        let tree = d.join("tree");
        let store = d.join("store/worktrees/tree");
        fs::create_dir_all(&tree).unwrap();
        fs::create_dir_all(&store).unwrap();
        fs::write(store.join("HEAD"), "ref: refs/heads/side\n").unwrap();
        fs::write(tree.join(".git"), format!("gitdir: {}\n", store.display())).unwrap();
        let wt = find(&tree).unwrap();
        assert_eq!(read_head(&wt.head), Some(Head::Branch("side".into())));
        fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn status_text_gives_dirty_and_sync() {
        let clean = "# branch.oid abc\n# branch.head main\n# branch.upstream origin/main\n# branch.ab +0 -0\n";
        assert_eq!(parse_status(clean), Sync { dirty: false, ahead: 0, behind: 0 });
        let busy = "# branch.ab +2 -1\n1 .M N... 100644 100644 100644 a b src/x.rs\n? new.txt\n";
        assert_eq!(parse_status(busy), Sync { dirty: true, ahead: 2, behind: 1 });
        assert_eq!(parse_status("# branch.head main\n"), Sync::default());
    }

    #[test]
    fn remotes_become_owner_slash_name() {
        assert_eq!(parse_remote("https://github.com/QDHShamiro/Wisply.git"), Some("QDHShamiro/Wisply".into()));
        assert_eq!(parse_remote("git@github.com:QDHShamiro/Wisply.git\n"), Some("QDHShamiro/Wisply".into()));
        assert_eq!(parse_remote("ssh://git@github.com/o/r/"), Some("o/r".into()));
        assert_eq!(parse_remote("nonsense"), None);
    }
}
