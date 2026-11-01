use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};

use super::config::dir;
use super::git::Sync;
use super::github::Pr;

pub const GIT_TTL: u64 = 5;
pub const PR_TTL: u64 = 60;
pub const SPAWN_GAP: u64 = 4;
const DROP_AFTER: u64 = 600;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Entry {
    pub sync: Option<Sync>,
    pub pr: Option<Pr>,
    pub gh: String,
    pub repo: String,
    pub git_at: u64,
    pub pr_at: u64,
    pub started: u64,
}

pub type Map2 = Map<String, Value>;

fn file(home: &Path) -> PathBuf {
    dir(home).join("cache.json")
}

pub fn key(root: &Path, branch: &str) -> String {
    format!("{}|{branch}", root.to_string_lossy().replace('\\', "/"))
}

impl Entry {
    pub fn to_value(&self) -> Value {
        let sync = self.sync.map(|s| json!({"dirty": s.dirty, "ahead": s.ahead, "behind": s.behind}));
        json!({"sync": sync, "pr": self.pr.map(Pr::to_value), "gh": self.gh, "repo": self.repo,
               "git_at": self.git_at, "pr_at": self.pr_at, "started": self.started})
    }

    pub fn from_value(v: &Value) -> Self {
        let num = |k: &str| v.get(k).and_then(Value::as_u64).unwrap_or(0);
        let text = |k: &str| v.get(k).and_then(Value::as_str).unwrap_or("").to_string();
        let sync = v.get("sync").filter(|s| s.is_object()).map(|s| Sync {
            dirty: s.get("dirty").and_then(Value::as_bool).unwrap_or(false),
            ahead: s.get("ahead").and_then(Value::as_u64).unwrap_or(0) as u32,
            behind: s.get("behind").and_then(Value::as_u64).unwrap_or(0) as u32,
        });
        Entry {
            sync,
            pr: v.get("pr").and_then(Pr::from_value),
            gh: text("gh"),
            repo: text("repo"),
            git_at: num("git_at"),
            pr_at: num("pr_at"),
            started: num("started"),
        }
    }

    pub fn newest(&self) -> u64 {
        self.git_at.max(self.pr_at)
    }

    pub fn wants_git(&self, now: u64) -> bool {
        self.git_at == 0 || now.saturating_sub(self.git_at) >= GIT_TTL
    }

    pub fn wants_pr(&self, now: u64) -> bool {
        self.pr_at == 0 || now.saturating_sub(self.pr_at) >= PR_TTL
    }

    pub fn busy(&self, now: u64) -> bool {
        self.started != 0 && now.saturating_sub(self.started) < SPAWN_GAP
    }
}

pub fn load(home: &Path) -> Map2 {
    let Ok(text) = fs::read_to_string(file(home)) else { return Map::new() };
    match serde_json::from_str::<Value>(&text) {
        Ok(Value::Object(map)) => map,
        _ => Map::new(),
    }
}

pub fn get(map: &Map2, key: &str) -> Option<Entry> {
    map.get(key).map(Entry::from_value)
}

pub fn fresh(entry: &Entry, now: u64) -> bool {
    entry.newest() != 0 && now.saturating_sub(entry.newest()) <= DROP_AFTER
}

pub fn prune(map: &mut Map2, now: u64) {
    map.retain(|_, v| {
        let e = Entry::from_value(v);
        fresh(&e, now) || (e.started != 0 && now.saturating_sub(e.started) < DROP_AFTER)
    });
}

pub fn save(home: &Path, map: &Map2) -> Result<(), String> {
    let path = file(home);
    let tmp = path.with_extension(format!("tmp{}", std::process::id()));
    let text = serde_json::to_string(&Value::Object(map.clone())).map_err(|e| e.to_string())?;
    fs::create_dir_all(dir(home))
        .and_then(|()| fs::write(&tmp, text))
        .and_then(|()| fs::rename(&tmp, &path))
        .map_err(|e| format!("cannot save {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::statusline::github::{Checks, State};

    #[test]
    fn an_entry_round_trips_and_knows_when_it_is_stale() {
        let e = Entry {
            sync: Some(Sync { dirty: true, ahead: 2, behind: 1 }),
            pr: Some(Pr { number: 3, state: State::Open, checks: Checks::Pass }),
            gh: "ok".into(),
            repo: "o/r".into(),
            git_at: 100,
            pr_at: 100,
            started: 0,
        };
        assert_eq!(Entry::from_value(&e.to_value()), e);
        assert!(!e.wants_git(104) && e.wants_git(105));
        assert!(!e.wants_pr(159) && e.wants_pr(160));
        assert!(Entry::default().wants_pr(1) && Entry::default().wants_git(1));
        assert!(Entry { started: 100, ..Entry::default() }.busy(103));
        assert!(!Entry { started: 100, ..Entry::default() }.busy(104));
        assert!(!Entry::default().busy(5));
    }

    #[test]
    fn old_entries_are_dropped_and_the_file_round_trips() {
        let home = std::env::temp_dir().join(format!("lt-sl-cache-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        let mut map = load(&home);
        assert!(map.is_empty());
        let old = Entry { git_at: 10, pr_at: 10, ..Entry::default() };
        let new = Entry { git_at: 5000, pr_at: 5000, ..Entry::default() };
        map.insert(key(Path::new("C:\\a"), "old"), old.to_value());
        map.insert(key(Path::new("C:\\a"), "new"), new.to_value());
        prune(&mut map, 5001);
        assert_eq!(map.len(), 1);
        save(&home, &map).unwrap();
        let back = load(&home);
        assert_eq!(get(&back, "C:/a|new"), Some(new.clone()));
        assert!(fresh(&new, 5001) && !fresh(&old, 5001) && !fresh(&Entry::default(), 5001));
        fs::remove_dir_all(&home).unwrap();
    }
}
