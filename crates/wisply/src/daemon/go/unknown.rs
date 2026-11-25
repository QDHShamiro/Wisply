use std::fs;
use std::path::Path;

use serde_json::{Value, json};

const FILE: &str = "go/unknown.json";
const KEEP: usize = 50;
const SHOW: usize = 20;

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub said: String,
    pub count: u32,
    pub last: u64,
}

pub fn key(said: &str) -> String {
    said.split_whitespace().collect::<Vec<_>>().join(" ").trim_end_matches(['.', '!', '?']).to_lowercase()
}

fn entry(v: &Value) -> Option<Entry> {
    Some(Entry {
        said: v.get("said")?.as_str()?.to_string(),
        count: u32::try_from(v.get("count")?.as_u64()?).ok()?,
        last: v.get("last")?.as_u64()?,
    })
}

pub fn load(home: &Path) -> Vec<Entry> {
    let raw = fs::read_to_string(home.join(FILE)).unwrap_or_default();
    let all: Vec<Value> = serde_json::from_str(&raw).unwrap_or_default();
    let mut rows: Vec<Entry> = all.iter().filter_map(entry).collect();
    let extra = rows.len().saturating_sub(KEEP);
    rows.drain(..extra);
    rows
}

fn store(home: &Path, rows: &[Entry]) {
    let list: Vec<Value> = rows.iter().map(|e| json!({"said": e.said, "count": e.count, "last": e.last})).collect();
    let _ = wisply_core::save(home.join(FILE), Value::Array(list).to_string());
}

pub fn note_at(home: &Path, said: &str, at: u64) -> u32 {
    let want = key(said);
    if want.is_empty() {
        return 0;
    }
    let mut rows = load(home);
    let old = rows.iter().position(|e| key(&e.said) == want).map(|i| rows.remove(i));
    let count = old.map_or(0, |e| e.count).saturating_add(1);
    let shown = said.split_whitespace().collect::<Vec<_>>().join(" ");
    rows.push(Entry { said: shown.trim_end_matches(['.', '!', '?']).to_string(), count, last: at });
    let extra = rows.len().saturating_sub(KEEP);
    rows.drain(..extra);
    store(home, &rows);
    count
}

pub fn note(home: &Path, said: &str) -> u32 {
    note_at(home, said, super::super::ctx::now() as u64)
}

pub fn forget(home: &Path, said: &str) {
    let want = key(said);
    let mut rows = load(home);
    let before = rows.len();
    rows.retain(|e| key(&e.said) != want);
    if rows.len() != before {
        store(home, &rows);
    }
}

pub fn clear(home: &Path) -> String {
    let n = load(home).len();
    if n == 0 {
        return "no unknown orders".into();
    }
    store(home, &[]);
    format!("cleared {n} unknown orders")
}

pub fn list(home: &Path) -> String {
    let mut rows = load(home);
    if rows.is_empty() {
        return "no unknown orders".into();
    }
    rows.sort_by(|a, b| b.count.cmp(&a.count).then(b.last.cmp(&a.last)));
    let mut lines = vec![format!("{} unknown orders, most asked first:", rows.len())];
    lines.extend(rows.iter().take(SHOW).map(|e| format!("  {}x \"{}\"", e.count, e.said)));
    if rows.len() > SHOW {
        lines.push(format!("  ... {} more", rows.len() - SHOW));
    }
    lines.push("do one once, then: wisply go --teach \"<order>\"   clear: wisply go --unknown clear".into());
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn home(tag: &str) -> std::path::PathBuf {
        let home = std::env::temp_dir().join(format!("wisply-go-unknown-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(&home).expect("home");
        home
    }

    #[test]
    fn counts_the_same_sentence_up() {
        let h = home("count");
        assert_eq!(note_at(&h, "mach das Licht an", 10), 1);
        assert_eq!(note_at(&h, "  Mach das  LICHT an! ", 20), 2);
        assert_eq!(note_at(&h, "mach das licht an", 30), 3);
        let rows = load(&h);
        assert_eq!(rows.len(), 1);
        assert_eq!((rows[0].count, rows[0].last), (3, 30));
        assert_eq!(rows[0].said, "mach das licht an");
        let _ = fs::remove_dir_all(&h);
    }

    #[test]
    fn different_sentences_stay_apart_and_blanks_are_ignored() {
        let h = home("apart");
        assert_eq!(note_at(&h, "eins", 1), 1);
        assert_eq!(note_at(&h, "zwei", 2), 1);
        assert_eq!(note_at(&h, "  ", 3), 0);
        assert_eq!(load(&h).len(), 2);
        let _ = fs::remove_dir_all(&h);
    }

    #[test]
    fn keeps_fifty_and_drops_the_oldest() {
        let h = home("max");
        for i in 0..60 {
            note_at(&h, &format!("order {i}"), i);
        }
        note_at(&h, "order 20", 100);
        let rows = load(&h);
        assert_eq!(rows.len(), KEEP);
        assert!(rows.iter().all(|e| e.said != "order 0" && e.said != "order 9"));
        assert!(rows.iter().any(|e| e.said == "order 59"));
        assert_eq!(rows.last().map(|e| e.said.as_str()), Some("order 20"));
        let _ = fs::remove_dir_all(&h);
    }

    #[test]
    fn clear_empties_the_list() {
        let h = home("clear");
        note_at(&h, "a b", 1);
        note_at(&h, "c d", 2);
        assert_eq!(clear(&h), "cleared 2 unknown orders");
        assert!(load(&h).is_empty());
        assert_eq!(clear(&h), "no unknown orders");
        assert_eq!(list(&h), "no unknown orders");
        let _ = fs::remove_dir_all(&h);
    }

    #[test]
    fn forget_removes_only_that_sentence() {
        let h = home("forget");
        note_at(&h, "Spiel was Neues", 1);
        note_at(&h, "other", 2);
        forget(&h, "spiel was neues.");
        let rows = load(&h);
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].said, "other");
        forget(&h, "never seen");
        assert_eq!(load(&h).len(), 1);
        let _ = fs::remove_dir_all(&h);
    }

    #[test]
    fn list_puts_the_most_asked_first() {
        let h = home("list");
        note_at(&h, "rare", 5);
        note_at(&h, "often", 6);
        note_at(&h, "often", 7);
        let out = list(&h);
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines[1], "  2x \"often\"");
        assert_eq!(lines[2], "  1x \"rare\"");
        assert!(out.contains("go --unknown clear"));
        let _ = fs::remove_dir_all(&h);
    }

    #[test]
    fn bad_files_read_as_empty() {
        let h = home("bad");
        fs::create_dir_all(h.join("go")).expect("dir");
        fs::write(h.join(FILE), "not json").expect("write");
        assert!(load(&h).is_empty());
        assert_eq!(note_at(&h, "fresh", 1), 1);
        let _ = fs::remove_dir_all(&h);
    }

    #[test]
    fn dry_does_not_save_but_a_real_run_does() {
        let h = home("dry");
        let t = crate::daemon::ctx::now();
        let said = "qwertz asdf yxcv";
        let seen = super::super::plan(&h, said, t, false).expect_err("unknown").text();
        assert!(seen.contains("unknown order \"qwertz asdf yxcv\""), "{seen}");
        assert!(!seen.contains("asked"), "{seen}");
        assert!(load(&h).is_empty());
        super::super::plan(&h, said, t, true).expect_err("unknown");
        assert_eq!(load(&h).len(), 1);
        let again = super::super::plan(&h, said, t, true).expect_err("unknown").text();
        assert!(again.contains("(asked 2x)") && again.contains("do it once, then: wisply go --teach"), "{again}");
        let _ = fs::remove_dir_all(&h);
    }

    #[test]
    fn known_orders_are_never_saved() {
        let h = home("known");
        let t = crate::daemon::ctx::now();
        assert!(super::super::plan(&h, "lauter", t, true).is_ok());
        assert!(load(&h).is_empty());
        let _ = fs::remove_dir_all(&h);
    }
}
