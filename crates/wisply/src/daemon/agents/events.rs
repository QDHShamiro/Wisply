use std::collections::VecDeque;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use serde_json::{Value, json};

use super::secret::scrub;

pub const KINDS: [&str; 7] = ["done", "waiting", "ci_red", "ci_green", "limit", "resumed", "disk"];
const KEEP: usize = 200;
const FILE_MAX: u64 = 256 * 1024;
const TITLE_MAX: usize = 80;
const BODY_MAX: usize = 140;

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub seq: u64,
    pub kind: String,
    pub agent_id: Option<String>,
    pub title: String,
    pub body: String,
    pub at: i64,
}

impl Event {
    pub fn json(&self) -> Value {
        let mut v = json!({"seq": self.seq, "kind": self.kind, "title": self.title, "body": self.body});
        if let Some(id) = &self.agent_id {
            v["agent_id"] = json!(id);
        }
        v
    }

    fn from(v: &Value) -> Option<Event> {
        Some(Event {
            seq: v["seq"].as_u64()?,
            kind: v["kind"].as_str()?.to_string(),
            agent_id: v["agent_id"].as_str().map(String::from),
            title: v["title"].as_str()?.to_string(),
            body: v["body"].as_str().unwrap_or("").to_string(),
            at: v["at"].as_i64().unwrap_or(0),
        })
    }
}

#[derive(Default)]
struct Store {
    next: u64,
    items: VecDeque<Event>,
    loaded: bool,
}

static STORE: Mutex<Store> = Mutex::new(Store { next: 0, items: VecDeque::new(), loaded: false });
static WAKE: Condvar = Condvar::new();

fn lock() -> MutexGuard<'static, Store> {
    STORE.lock().unwrap_or_else(PoisonError::into_inner)
}

fn path(home: &Path) -> std::path::PathBuf {
    home.join("agents").join("events.jsonl")
}

fn load(home: &Path, store: &mut Store) {
    if store.loaded {
        return;
    }
    store.loaded = true;
    let text = fs::read_to_string(path(home)).unwrap_or_default();
    for e in text.lines().filter_map(|l| serde_json::from_str::<Value>(l).ok()).filter_map(|v| Event::from(&v)) {
        store.next = store.next.max(e.seq);
        store.items.push_back(e);
        if store.items.len() > KEEP {
            store.items.pop_front();
        }
    }
}

fn append(home: &Path, e: &Event) {
    let file = path(home);
    let _ = fs::create_dir_all(home.join("agents"));
    if fs::metadata(&file).is_ok_and(|m| m.len() > FILE_MAX) {
        let mut v = e.json();
        v["at"] = json!(e.at);
        let keep: Vec<String> = lock()
            .items
            .iter()
            .map(|x| {
                let mut v = x.json();
                v["at"] = json!(x.at);
                v.to_string()
            })
            .collect();
        let _ = fs::write(&file, keep.join("\n") + "\n");
        return;
    }
    let mut v = e.json();
    v["at"] = json!(e.at);
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(&file) {
        let _ = writeln!(f, "{v}");
    }
}

pub fn emit(home: &Path, kind: &str, agent_id: Option<&str>, title: &str, body: &str, toast: bool) -> Event {
    let at = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64);
    let mut store = lock();
    load(home, &mut store);
    store.next += 1;
    let e = Event {
        seq: store.next,
        kind: kind.to_string(),
        agent_id: agent_id.map(String::from),
        title: scrub(title, TITLE_MAX),
        body: scrub(body, BODY_MAX),
        at,
    };
    store.items.push_back(e.clone());
    if store.items.len() > KEEP {
        store.items.pop_front();
    }
    drop(store);
    append(home, &e);
    WAKE.notify_all();
    if toast {
        wisply_pc::notify::send(&e.title, &e.body);
    }
    e
}

#[cfg(test)]
pub fn since(home: &Path, seq: u64) -> Vec<Event> {
    let mut store = lock();
    load(home, &mut store);
    store.items.iter().filter(|e| e.seq > seq).cloned().collect()
}

pub fn wait(home: &Path, seq: u64, timeout: Duration) -> Vec<Event> {
    let mut store = lock();
    load(home, &mut store);
    if !store.items.iter().any(|e| e.seq > seq) {
        store = WAKE.wait_timeout(store, timeout).unwrap_or_else(PoisonError::into_inner).0;
    }
    store.items.iter().filter(|e| e.seq > seq).cloned().collect()
}

#[cfg(test)]
pub fn last_seq(home: &Path) -> u64 {
    let mut store = lock();
    load(home, &mut store);
    store.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_get_ordered_numbers_a_clean_text_and_a_log_line() {
        let home = std::env::temp_dir().join(format!("wisply-agents-ev-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        let a = emit(&home, "done", Some("ab12"), "build ok", r"see C:\Users\x\p and sk-abcdefghijklmnopqrst", false);
        let b = emit(&home, "limit", None, "limit", "resets 3am", false);
        assert!(b.seq > a.seq);
        assert!(!a.body.contains("Users") && !a.body.contains("sk-abc"), "{}", a.body);
        assert!(since(&home, a.seq).iter().any(|e| e.seq == b.seq));
        assert!(since(&home, a.seq - 1).iter().any(|e| e.seq == a.seq));
        assert_eq!(a.json()["agent_id"], "ab12");
        assert!(b.json().get("agent_id").is_none());
        let text = fs::read_to_string(path(&home)).unwrap();
        assert_eq!(text.lines().count(), 2);
        assert!(wait(&home, u64::MAX - 1, Duration::from_millis(20)).is_empty());
        assert!(!wait(&home, a.seq, Duration::from_millis(20)).is_empty());
        let _ = fs::remove_dir_all(&home);
    }
}
