use std::collections::HashSet;
use std::path::Path;

use serde_json::{Value, json};
use wisply_memory::{KINDS, PRIOS, Store, Todo, now, oneline};

use super::super::{config, game};
use super::chat;

const WHO: &str = "tidy";
const EVERY_S: f64 = 1800.0;
const QUIET_S: f64 = 600.0;
const BATCH: usize = 12;
const SEEN_KEEP: usize = 5000;
const TASK_MIN: usize = 8;
const TASK_MAX: usize = 160;
const SHOWN: usize = 200;
const AT_KEY: &str = "tidy_at";
const SEEN_KEY: &str = "tidy_seen";
const OPEN: [&str; 2] = ["todo", "waiting"];
const SYSTEM: &str = "You tidy a to-do list. Input: one entry per line as `id | kind | priority | project | text`. \
Output JSON only. entries: for each entry you would change, its id and only the fields to change: task = a clearer \
title in the language of the original, same meaning, no new facts, keep names, numbers and paths; kind = bug \
(something is broken), feature (something new to build), task (work to do) or idea (maybe later); priority = high \
(crash, data loss, blocks work), mid or low; project = one name from the known projects, only when the entry has \
none. same: pairs [keep, duplicate] of entries that mean the same work. Change nothing that is fine already.";

fn entry_schema() -> Value {
    let text = json!({"type": "string"});
    let fields = json!({"id": {"type": "integer"}, "task": text, "kind": {"type": "string", "enum": KINDS},
        "priority": {"type": "string", "enum": PRIOS}, "project": text});
    json!({"type": "object", "required": ["id"], "properties": fields})
}

fn schema() -> Value {
    let pair = json!({"type": "array", "minItems": 2, "maxItems": 2, "items": {"type": "integer"}});
    let entries = json!({"type": "array", "maxItems": BATCH, "items": entry_schema()});
    let same = json!({"type": "array", "maxItems": BATCH, "items": pair});
    json!({"type": "object", "required": ["entries", "same"], "properties": {"entries": entries, "same": same}})
}

fn seen_of(store: &Store) -> Vec<i64> {
    let raw = store.meta(SEEN_KEY).ok().flatten().unwrap_or_default();
    serde_json::from_str(&raw).unwrap_or_default()
}

fn pick(rows: &[Todo], seen: &HashSet<i64>, at: f64) -> Vec<Todo> {
    rows.iter()
        .filter(|t| OPEN.contains(&t.status.as_str()) && t.source != "log" && !seen.contains(&t.id))
        .filter(|t| at - t.updated >= QUIET_S)
        .take(BATCH)
        .cloned()
        .collect()
}

fn prompt(batch: &[Todo], projects: &[String]) -> String {
    let lines: Vec<String> = batch
        .iter()
        .map(|t| {
            let project = if t.project.is_empty() { "-" } else { t.project.as_str() };
            format!("{} | {} | {} | {project} | {}", t.id, t.kind, t.priority, oneline(&t.task, SHOWN))
        })
        .collect();
    format!("Known projects: {}\nEntries:\n{}", projects.join(", "), lines.join("\n"))
}

fn good_task(old: &str, new: &str) -> bool {
    let n = new.trim().chars().count();
    (TASK_MIN..=TASK_MAX).contains(&n) && n * 3 >= old.trim().chars().count().min(TASK_MAX) && new.trim() != old.trim()
}

fn apply_entry(store: &Store, e: &Value, batch: &[Todo], projects: &[String], run: &str) -> usize {
    let Some(t) = e["id"].as_i64().and_then(|id| batch.iter().find(|t| t.id == id)) else { return 0 };
    let text = |k: &str| e[k].as_str().map(str::trim).unwrap_or("").to_string();
    let mut changes: Vec<(&str, String)> = Vec::new();
    let task = text("task");
    if good_task(&t.task, &task) {
        changes.push(("task", task));
    }
    for (field, allowed) in [("kind", &KINDS[..]), ("priority", &PRIOS[..])] {
        let v = text(field);
        if allowed.contains(&v.as_str()) {
            changes.push((field, v));
        }
    }
    let project = text("project");
    if t.project.is_empty() && projects.iter().any(|p| p == &project) {
        changes.push(("project", project));
    }
    let mut done = 0;
    for (field, value) in changes {
        if t.field(field) != value && store.entry_set(t.id, field, &value, WHO, run).ok().flatten().is_some() {
            done += 1;
        }
    }
    done
}

fn merge(store: &Store, pair: &Value, batch: &[Todo], run: &str) -> usize {
    let ids: Vec<i64> = pair.as_array().map(|a| a.iter().filter_map(Value::as_i64).collect()).unwrap_or_default();
    let [keep, dup] = ids[..] else { return 0 };
    let inside = |id: i64| batch.iter().any(|t| t.id == id);
    if keep == dup || !inside(keep) || !inside(dup) {
        return 0;
    }
    if store.entry_set(dup, "status", "dropped", WHO, run).ok().flatten().is_none() {
        return 0;
    }
    let _ = store.todo_note(dup, &format!("same as #{keep} (tidy)"));
    1
}

fn remember(store: &Store, seen: Vec<i64>, batch: &[Todo], at: f64) {
    let mut all = seen;
    all.extend(batch.iter().map(|t| t.id));
    let start = all.len().saturating_sub(SEEN_KEEP);
    if let Ok(text) = serde_json::to_string(&all[start..]) {
        let _ = store.set_meta(SEEN_KEY, &text);
    }
    let _ = store.set_meta(AT_KEY, &at.to_string());
}

fn due(store: &Store, at: f64) -> bool {
    let last: f64 = store.meta(AT_KEY).ok().flatten().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    at - last >= EVERY_S
}

pub fn tick(home: &Path, store: &Store) {
    let cfg = config::load(home);
    let at = now();
    if !config::on(&cfg, "list_tidy") || !due(store, at) || !game::running().is_empty() {
        return;
    }
    let Some(p) = chat::provider_local(home) else { return };
    let Ok(rows) = store.entries(None, false) else { return };
    let seen = seen_of(store);
    let known: HashSet<i64> = seen.iter().copied().collect();
    let batch = pick(&rows, &known, at);
    if batch.is_empty() {
        let _ = store.set_meta(AT_KEY, &at.to_string());
        return;
    }
    let mut projects: Vec<String> = rows.iter().map(|t| t.project.clone()).filter(|p| !p.is_empty()).collect();
    projects.sort();
    projects.dedup();
    let answer = match chat::chat(&p, SYSTEM, &prompt(&batch, &projects), &schema()) {
        Ok(a) => a,
        Err(chat::ChatError::NotReady(_)) => return,
        Err(chat::ChatError::Failed(_)) => {
            let _ = store.set_meta(AT_KEY, &at.to_string());
            return;
        }
    };
    if let Some(out) = super::learner::parse(&answer) {
        apply(store, &out, &batch, &projects, &format!("tidy-{}", at as i64));
    }
    remember(store, seen, &batch, at);
}

fn apply(store: &Store, out: &Value, batch: &[Todo], projects: &[String], run: &str) -> usize {
    let mut n = 0;
    for e in out["entries"].as_array().into_iter().flatten() {
        n += apply_entry(store, e, batch, projects, run);
    }
    for pair in out["same"].as_array().into_iter().flatten() {
        n += merge(store, pair, batch, run);
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    fn todo(id: i64, status: &str, source: &str, updated: f64) -> Todo {
        Todo {
            id,
            status: status.into(),
            source: source.into(),
            updated,
            task: format!("task {id}"),
            ..Todo::default()
        }
    }

    #[test]
    fn only_quiet_open_entries_by_hand_are_picked_once() {
        let rows = [
            todo(1, "todo", "hand", 0.0),
            todo(2, "doing", "hand", 0.0),
            todo(3, "todo", "log", 0.0),
            todo(4, "waiting", "ai", 0.0),
            todo(5, "todo", "hand", 1000.0),
            todo(6, "todo", "hand", 0.0),
        ];
        let seen: HashSet<i64> = [6].into();
        let ids: Vec<i64> = pick(&rows, &seen, 1000.0).iter().map(|t| t.id).collect();
        assert_eq!(ids, [1, 4]);
    }

    #[test]
    fn titles_stay_close_to_the_original() {
        assert!(good_task("fix the login page crash on submit", "Login page crashes on submit"));
        assert!(!good_task("fix the login page crash on submit when the user has no email", "fix"));
        assert!(!good_task("same text", "same text"));
        let s = schema();
        assert_eq!(s["properties"]["entries"]["maxItems"], BATCH);
    }

    #[test]
    fn tidy_renames_and_merges_with_undo_and_never_deletes() {
        let home = std::env::temp_dir().join(format!("lt-tidy-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        let s = Store::open(&home).unwrap();
        let add = |project: &str, task: &str| {
            s.entry_add(&wisply_memory::NewEntry { project, task, ..Default::default() }).unwrap()
        };
        let (a, b, c) =
            (add("Wisply", "fix login crash"), add("Wisply", "login crash again"), add("", "write the tidy docs"));
        let batch = s.entries(None, false).unwrap();
        let projects = vec!["Wisply".to_string()];
        let out = json!({
            "entries": [{"id": a, "task": "Login crashes on submit", "kind": "bug", "priority": "high"},
                {"id": c, "project": "Wisply", "kind": "task"}, {"id": 999, "task": "ghost entry here"}],
            "same": [[a, b], [a, a], [a, 999]]});
        assert_eq!(apply(&s, &out, &batch, &projects, "tidy-1"), 5);
        let one = s.todo(a).unwrap().unwrap();
        assert_eq!(
            (one.task.as_str(), one.kind.as_str(), one.priority.as_str()),
            ("Login crashes on submit", "bug", "high")
        );
        assert_eq!(s.todo(c).unwrap().unwrap().project, "Wisply");
        let dup = s.todo(b).unwrap().unwrap();
        assert!(dup.status == "dropped" && dup.note.contains(&format!("#{a}")), "{dup:?}");
        assert!(s.history(a).unwrap().iter().any(|h| h.who == WHO && h.batch == "tidy-1"));
        s.undo_batch("tidy-1").unwrap();
        let back = s.todo(a).unwrap().unwrap();
        assert_eq!((back.task.as_str(), back.kind.as_str()), ("fix login crash", "task"));
        assert_eq!(s.todo(b).unwrap().unwrap().status, "todo");
        let _ = std::fs::remove_dir_all(&home);
    }
}
