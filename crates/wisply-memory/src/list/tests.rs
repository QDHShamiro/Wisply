use std::fs;

use super::*;

fn store(tag: &str) -> (std::path::PathBuf, Store) {
    let dir = std::env::temp_dir().join(format!("ltl-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let s = Store::open(&dir).unwrap();
    (dir, s)
}

fn hit<'a>(server: &'a str, fp: &'a str, level: &'a str, at: f64) -> LogHit<'a> {
    LogHit { server, fingerprint: fp, title: "NullPointerException in Foo", level, excerpt: "at Foo.bar", count: 2, at }
}

#[test]
fn old_todos_migrate_to_tasks_and_keep_working() {
    let dir = std::env::temp_dir().join(format!("ltl-migrate-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("memory")).unwrap();
    {
        let db = Connection::open(dir.join("memory").join("memory.db")).unwrap();
        db.execute_batch(
            "create table todos(id integer primary key, project text, list text default '', task text, \
             status text default 'todo', note text default '', created real, updated real, origin text default '');
             insert into todos(project, task, status, created, updated) values ('wisply', 'old one', 'doing', 1, 1);",
        )
        .unwrap();
    }
    let s = Store::open(&dir).unwrap();
    let t = s.todo(1).unwrap().unwrap();
    assert_eq!((t.kind.as_str(), t.priority.as_str(), t.source.as_str(), t.count), ("task", "mid", "hand", 1));
    assert_eq!(s.todos(Some("wisply"), true).unwrap().len(), 1);
    drop(s);
    let s = Store::open(&dir).unwrap();
    assert_eq!(s.todo(1).unwrap().unwrap().task, "old one");
    drop(s);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn edits_are_logged_and_undone_without_deleting() {
    let (dir, s) = store("edit");
    let id = s
        .entry_add(&NewEntry { project: "p", task: "Fix login", kind: "bug", who: "you", ..NewEntry::default() })
        .unwrap();
    s.entry_set(id, "priority", "HIGH", "you", "").unwrap();
    s.entry_set(id, "status", "doing", "sid-1", "").unwrap();
    let t = s.todo(id).unwrap().unwrap();
    assert_eq!((t.priority.as_str(), t.status.as_str(), t.assignee.as_str()), ("high", "doing", "sid-1"));
    assert_eq!(s.entry_set(id, "kind", "nonsense", "you", "").unwrap().unwrap().kind, "bug");
    let h = s.history(id).unwrap();
    assert_eq!(h.iter().map(|c| c.field.as_str()).collect::<Vec<_>>(), ["assignee", "status", "priority", "created"]);
    assert_eq!(s.undo(id).unwrap().len(), 1);
    assert_eq!(s.todo(id).unwrap().unwrap().assignee, "");
    s.undo(id).unwrap();
    s.undo(id).unwrap();
    let t = s.todo(id).unwrap().unwrap();
    assert_eq!((t.priority.as_str(), t.status.as_str()), ("mid", "todo"));
    assert!(s.undo(id).unwrap().is_empty());
    s.entry_set(id, "status", "waiting", "you", "").unwrap();
    s.entry_set(id, "wait", "restart", "you", "").unwrap();
    s.entry_set(id, "status", "done", "you", "").unwrap();
    assert_eq!(s.todo(id).unwrap().unwrap().wait, "");
    drop(s);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn log_hits_group_reopen_and_mute() {
    let (dir, s) = store("log");
    let Seen::New(id) = s.log_hit(&hit("srv", "fp1", "crash", 10.0)).unwrap() else { panic!("not new") };
    let t = s.todo(id).unwrap().unwrap();
    assert_eq!((t.kind.as_str(), t.priority.as_str(), t.count, t.project.as_str()), ("bug", "high", 2, "srv"));
    assert_eq!(s.log_hit(&hit("srv", "fp1", "crash", 20.0)).unwrap(), Seen::Again(id));
    assert_eq!(s.todo(id).unwrap().unwrap().count, 4);
    assert!(s.todos(None, true).unwrap().is_empty());
    assert_eq!(s.entries(Some("srv"), false).unwrap().len(), 1);
    s.entry_set(id, "status", "done", "you", "").unwrap();
    assert_eq!(s.log_hit(&hit("srv", "fp1", "crash", 30.0)).unwrap(), Seen::Back(id));
    let t = s.todo(id).unwrap().unwrap();
    assert!(t.status == "todo" && t.note.ends_with("kam wieder") && t.last_seen == 30.0);
    let Seen::New(w) = s.log_hit(&hit("srv", "fp2", "warn", 40.0)).unwrap() else { panic!("not new") };
    assert_eq!(s.todo(w).unwrap().unwrap().priority, "low");
    s.mute(w, "you").unwrap();
    assert_eq!(s.log_hit(&hit("srv", "fp2", "warn", 50.0)).unwrap(), Seen::Muted);
    assert_eq!(s.mutes().unwrap().len(), 1);
    s.undo(w).unwrap();
    assert!(s.mutes().unwrap().is_empty() && s.todo(w).unwrap().unwrap().status == "todo");
    drop(s);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn restart_finishes_waiting_entries_of_that_server() {
    let (dir, s) = store("restart");
    let a = s.entry_add(&NewEntry { project: "cpvpgens", task: "Coins fix", ..NewEntry::default() }).unwrap();
    let b =
        s.entry_add(&NewEntry { project: "x", server: "cpvpgens", task: "Kit fix", ..NewEntry::default() }).unwrap();
    let c = s.entry_add(&NewEntry { project: "other", task: "Not this", ..NewEntry::default() }).unwrap();
    for id in [a, b, c] {
        s.entry_set(id, "status", "waiting", "you", "").unwrap();
        s.entry_set(id, "wait", "restart", "you", "").unwrap();
    }
    assert_eq!(s.restart_done("cpvpgens", "restart:1").unwrap(), [a, b]);
    assert_eq!(s.todo(a).unwrap().unwrap().status, "done");
    assert_eq!(s.todo(c).unwrap().unwrap().status, "waiting");
    assert_eq!(s.undo_batch("restart:1").unwrap(), 4);
    let t = s.todo(a).unwrap().unwrap();
    assert_eq!((t.status.as_str(), t.wait.as_str()), ("waiting", "restart"));
    drop(s);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn suggestions_skip_known_tasks_and_stay_out_of_todos() {
    let (dir, s) = store("suggest");
    s.todo_add("p", "Deploy the new kit plugin", "", "todo").unwrap();
    assert_eq!(s.suggest("p", "- deploy the new kit plugin", "session:a").unwrap(), None);
    assert_eq!(s.suggest("p", "short", "session:a").unwrap(), None);
    let id = s.suggest("p", "Write the restart notes for Luis", "session:a").unwrap().unwrap();
    assert_eq!(s.todo(id).unwrap().unwrap().status, "suggested");
    assert_eq!(s.todos(Some("p"), false).unwrap().len(), 1);
    s.entry_set(id, "status", "dropped", "you", "").unwrap();
    assert_eq!(s.suggest("p", "Write the restart notes for Luis", "session:b").unwrap(), None);
    drop(s);
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn server_state_round_trips() {
    let (dir, s) = store("servers");
    let st = ServerState {
        name: "cpvpgens".into(),
        enabled: true,
        offset: 42,
        head: "abc".into(),
        ..ServerState::default()
    };
    s.server_save(&st).unwrap();
    assert_eq!(s.server_state("cpvpgens").unwrap().unwrap(), st);
    s.server_save(&ServerState { enabled: false, ..st }).unwrap();
    assert!(!s.servers().unwrap()[0].enabled);
    drop(s);
    fs::remove_dir_all(&dir).unwrap();
}
