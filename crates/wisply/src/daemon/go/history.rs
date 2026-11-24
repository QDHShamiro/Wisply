#![cfg_attr(not(windows), allow(dead_code))]

use std::fs;
use std::path::Path;

const FILE: &str = "go-history.json";
const KEEP: usize = 50;

#[derive(Debug, Default, PartialEq)]
pub struct History {
    items: Vec<String>,
    at: Option<usize>,
    draft: String,
}

impl History {
    pub const fn empty() -> History {
        History { items: Vec::new(), at: None, draft: String::new() }
    }

    pub fn from_json(raw: &str) -> History {
        let items: Vec<String> = serde_json::from_str(raw).unwrap_or_default();
        let mut h = History::empty();
        items.iter().for_each(|i| h.push(i));
        h
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.items).unwrap_or_else(|_| "[]".into())
    }

    pub fn load(home: &Path) -> History {
        fs::read_to_string(home.join(FILE)).map(|raw| History::from_json(&raw)).unwrap_or_default()
    }

    pub fn save(&self, home: &Path) {
        let _ = wisply_core::save(home.join(FILE), self.to_json());
    }

    pub fn push(&mut self, said: &str) {
        let said = said.trim();
        self.at = None;
        if said.is_empty() {
            return;
        }
        self.items.retain(|i| i != said);
        self.items.push(said.to_string());
        let extra = self.items.len().saturating_sub(KEEP);
        self.items.drain(..extra);
    }

    pub fn older(&mut self, current: &str) -> Option<String> {
        let last = self.items.len().checked_sub(1)?;
        let next = match self.at {
            None => {
                self.draft = current.to_string();
                last
            }
            Some(i) => i.saturating_sub(1),
        };
        self.at = Some(next);
        self.items.get(next).cloned()
    }

    pub fn newer(&mut self) -> Option<String> {
        let i = self.at?;
        if i + 1 < self.items.len() {
            self.at = Some(i + 1);
            return self.items.get(i + 1).cloned();
        }
        self.at = None;
        Some(std::mem::take(&mut self.draft))
    }

    pub fn reset(&mut self) {
        self.at = None;
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn filled(n: usize) -> History {
        let mut h = History::empty();
        (0..n).for_each(|i| h.push(&format!("cmd {i}")));
        h
    }

    #[test]
    fn up_walks_back_and_stops_at_the_oldest() {
        let mut h = filled(3);
        assert_eq!(h.older("").as_deref(), Some("cmd 2"));
        assert_eq!(h.older("").as_deref(), Some("cmd 1"));
        assert_eq!(h.older("").as_deref(), Some("cmd 0"));
        assert_eq!(h.older("").as_deref(), Some("cmd 0"));
    }

    #[test]
    fn down_returns_to_the_draft() {
        let mut h = filled(2);
        assert_eq!(h.older("half typed").as_deref(), Some("cmd 1"));
        assert_eq!(h.older("ignored").as_deref(), Some("cmd 0"));
        assert_eq!(h.newer().as_deref(), Some("cmd 1"));
        assert_eq!(h.newer().as_deref(), Some("half typed"));
        assert_eq!(h.newer(), None);
    }

    #[test]
    fn empty_history_and_down_without_up_do_nothing() {
        let mut h = History::empty();
        assert_eq!(h.older("x"), None);
        assert_eq!(h.newer(), None);
        let mut h = filled(1);
        assert_eq!(h.newer(), None);
    }

    #[test]
    fn keeps_the_last_fifty_without_duplicates() {
        let mut h = filled(60);
        assert_eq!(h.len(), KEEP);
        assert_eq!(h.older("").as_deref(), Some("cmd 59"));
        h.push("cmd 30");
        h.push("  cmd 30  ");
        h.push("   ");
        assert_eq!(h.len(), KEEP);
        assert_eq!(h.older("").as_deref(), Some("cmd 30"));
    }

    #[test]
    fn push_and_reset_leave_browsing() {
        let mut h = filled(3);
        h.older("");
        h.reset();
        assert_eq!(h.older("").as_deref(), Some("cmd 2"));
        h.push("new");
        assert_eq!(h.older("").as_deref(), Some("new"));
    }

    #[test]
    fn json_roundtrip_and_bad_files() {
        let h = filled(3);
        assert_eq!(History::from_json(&h.to_json()).to_json(), h.to_json());
        assert_eq!(History::from_json("not json").len(), 0);
        assert_eq!(History::from_json("{\"a\":1}").len(), 0);
        let long: Vec<String> = (0..80).map(|i| format!("c{i}")).collect();
        assert_eq!(History::from_json(&serde_json::to_string(&long).expect("json")).len(), KEEP);
    }

    #[test]
    fn saves_and_loads_from_home() {
        let home = std::env::temp_dir().join(format!("wisply-go-history-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(&home).expect("home");
        filled(4).save(&home);
        let mut back = History::load(&home);
        assert_eq!(back.len(), 4);
        assert_eq!(back.older("").as_deref(), Some("cmd 3"));
        assert_eq!(History::load(&home.join("missing")).len(), 0);
        let _ = fs::remove_dir_all(&home);
    }
}
