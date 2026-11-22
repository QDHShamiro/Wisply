use std::collections::HashMap;
use std::path::Path;

use wisply_memory::{Record, Store, Unsummarized, now, project_of_files, titled};

use super::handoff;
use super::learner::{MIN_CHARS, MIN_USERS};

const SINCE_S: f64 = 172_800.0;
const AGAIN_S: f64 = 600.0;
const BATCH: usize = 8;
const UNTITLED: &str = "session, not summarized yet";

pub type Seen = HashMap<String, f64>;

fn write(store: &mut Store, row: &Unsummarized) -> Option<i64> {
    let found = handoff::early(Path::new(&row.transcript), MIN_CHARS, MIN_USERS)?;
    let project = Some(project_of_files(&found.paths)).filter(|p| !p.is_empty()).unwrap_or_else(|| row.project.clone());
    if project != row.project {
        store.session_project(&row.id, &project).ok()?;
    }
    let head = if found.title.is_empty() { UNTITLED } else { found.title.as_str() };
    let record = Record { title: titled(&project, head), summary: found.body, ..Record::default() };
    store.session_note(&row.id, &project, &record).ok().flatten()
}

pub fn notes(store: &mut Store, seen: &mut Seen) {
    let Ok(rows) = store.unsummarized(now() - SINCE_S, AGAIN_S) else { return };
    let mut next = Seen::new();
    let mut read = 0;
    for row in &rows {
        let known = seen.get(&row.id).is_some_and(|last| *last == row.last);
        if !known && read == BATCH {
            continue;
        }
        if !known {
            read += 1;
        }
        if known || write(store, row).is_none() {
            next.insert(row.id.clone(), row.last);
        }
    }
    *seen = next;
}

#[cfg(test)]
mod tests {
    use std::fs;

    use serde_json::json;

    use super::*;

    fn user(text: &str) -> String {
        json!({"type": "user", "message": {"content": text}}).to_string()
    }

    fn said(text: &str) -> String {
        json!({"type": "assistant", "message": {"content": [{"type": "text", "text": text}]}}).to_string()
    }

    #[test]
    fn a_session_nobody_summarized_yet_is_found_and_listed() {
        let dir = std::env::temp_dir().join(format!("lt-early-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let mut store = Store::open(&dir).unwrap();
        let long = dir.join("long.jsonl");
        let mut rows = vec![user("Measure the recall ranking with the benchmark and report hit@1")];
        rows.extend((0..8).map(|n| {
            said(&format!("Found: run {n} of the ranking benchmark moved hit@1 because the snippet fusion changed."))
        }));
        rows.push(user("now compare the embedding fusion against words only"));
        rows.push(json!({"type": "ai-title", "aiTitle": "Recall ranking benchmark"}).to_string());
        fs::write(&long, rows.join("\n") + "\n").unwrap();
        let short = dir.join("short.jsonl");
        fs::write(&short, [user("hi"), said("Hello.")].join("\n") + "\n").unwrap();
        store.session_seen("s1", "bench", "/work/bench", &long.to_string_lossy(), false).unwrap();
        store.session_seen("s2", "bench", "/work/bench", &short.to_string_lossy(), false).unwrap();
        let mut seen = Seen::new();
        notes(&mut store, &mut seen);
        let hits = store.search("recall ranking benchmark hit@1 embedding fusion", "", 3, None, 0.0).unwrap();
        assert_eq!(hits.first().map(|h| h.note.origin.as_str()), Some("s:s1"));
        let listed = store.sessions("bench", 3).unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].title, "bench: Recall ranking benchmark");
        assert_eq!(listed[0].summary, "Latest request: now compare the embedding fusion against words only");
        assert!(seen.contains_key("s2") && !seen.contains_key("s1"));
        let before = store.note_count().unwrap();
        notes(&mut store, &mut seen);
        assert_eq!(store.note_count().unwrap(), before);
        drop(store);
        let _ = fs::remove_dir_all(&dir);
    }
}
