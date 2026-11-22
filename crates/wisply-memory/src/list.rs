use rusqlite::{Connection, OptionalExtension, Row, params};
use wisply_core::local_parts;

use super::store::{Store, TASK_CHARS, now, real, text};
use super::text::oneline;

pub const KINDS: [&str; 4] = ["bug", "feature", "task", "idea"];
pub const STATUSES: [&str; 6] = ["suggested", "todo", "doing", "waiting", "done", "dropped"];
pub const OPEN: [&str; 3] = ["todo", "doing", "waiting"];
pub const PRIOS: [&str; 3] = ["high", "mid", "low"];
pub const WAITS: [&str; 2] = ["restart", "you"];
pub const LEVELS: [&str; 3] = ["warn", "error", "crash"];
pub const FIELDS: [&str; 9] = ["task", "kind", "priority", "status", "wait", "project", "body", "assignee", "list"];
pub const BODY_CHARS: usize = 20_000;
const HISTORY_ROWS: i64 = 100;
const SCHEMA: &str = "
create table if not exists todo_history(id integer primary key, todo integer, at real, who text default '',
  field text, old text default '', new text default '', batch text default '', undone integer default 0);
create index if not exists todo_history_todo on todo_history(todo, id);
create index if not exists todo_history_batch on todo_history(batch);
create table if not exists todo_mutes(fingerprint text primary key, server text default '', title text default '',
  at real);
create table if not exists servers(name text primary key, enabled integer default 1, log text default '',
  offset integer default 0, head text default '', checked real default 0, ok real default 0,
  error text default '', started real default 0, crashes text default '', hostkey text default '');
";
const COLUMNS: &[(&str, &str)] = &[
    ("kind", "text default 'task'"),
    ("priority", "text default 'mid'"),
    ("wait", "text default ''"),
    ("assignee", "text default ''"),
    ("source", "text default 'hand'"),
    ("body", "text default ''"),
    ("fingerprint", "text default ''"),
    ("count", "integer default 1"),
    ("first_seen", "real default 0"),
    ("last_seen", "real default 0"),
    ("server", "text default ''"),
    ("level", "text default ''"),
    ("excerpt", "text default ''"),
];

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Todo {
    pub id: i64,
    pub project: String,
    pub list: String,
    pub task: String,
    pub status: String,
    pub note: String,
    pub created: f64,
    pub updated: f64,
    pub kind: String,
    pub priority: String,
    pub wait: String,
    pub assignee: String,
    pub source: String,
    pub body: String,
    pub fingerprint: String,
    pub count: i64,
    pub first_seen: f64,
    pub last_seen: f64,
    pub server: String,
    pub level: String,
    pub excerpt: String,
    pub origin: String,
}

impl Todo {
    pub fn field(&self, name: &str) -> String {
        match name {
            "task" => self.task.clone(),
            "kind" => self.kind.clone(),
            "priority" => self.priority.clone(),
            "status" => self.status.clone(),
            "wait" => self.wait.clone(),
            "project" => self.project.clone(),
            "body" => self.body.clone(),
            "assignee" => self.assignee.clone(),
            "list" => self.list.clone(),
            _ => String::new(),
        }
    }

    pub fn open(&self) -> bool {
        OPEN.contains(&self.status.as_str())
    }
}

#[derive(Debug, Clone, Default)]
pub struct NewEntry<'a> {
    pub project: &'a str,
    pub task: &'a str,
    pub list: &'a str,
    pub status: &'a str,
    pub kind: &'a str,
    pub priority: &'a str,
    pub source: &'a str,
    pub body: &'a str,
    pub server: &'a str,
    pub assignee: &'a str,
    pub origin: &'a str,
    pub who: &'a str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Change {
    pub id: i64,
    pub todo: i64,
    pub at: f64,
    pub who: String,
    pub field: String,
    pub old: String,
    pub new: String,
    pub batch: String,
    pub undone: bool,
}

pub(crate) fn migrate(db: &Connection) {
    let _ = db.execute_batch(SCHEMA);
    let have: Vec<String> = db
        .prepare("select name from pragma_table_info('todos')")
        .and_then(|mut s| s.query_map([], |r| r.get::<_, String>(0))?.collect())
        .unwrap_or_default();
    for (col, ty) in COLUMNS {
        if !have.iter().any(|h| h == col) {
            let _ = db.execute(&format!("alter table todos add column {col} {ty}"), []);
        }
    }
    let _ = db.execute("create index if not exists todos_fingerprint on todos(server, fingerprint)", []);
}

fn int(row: &Row, key: &str) -> i64 {
    row.get::<_, Option<i64>>(key).ok().flatten().unwrap_or(0)
}

fn or(value: String, fallback: &str) -> String {
    if value.is_empty() { fallback.to_string() } else { value }
}

fn or_str<'a>(value: &'a str, fallback: &'a str) -> &'a str {
    if value.trim().is_empty() { fallback } else { value.trim() }
}

pub(crate) fn todo_of(row: &Row) -> rusqlite::Result<Todo> {
    Ok(Todo {
        id: row.get("id")?,
        project: text(row, "project"),
        list: text(row, "list"),
        task: text(row, "task"),
        status: text(row, "status"),
        note: text(row, "note"),
        created: real(row, "created"),
        updated: real(row, "updated"),
        kind: or(text(row, "kind"), "task"),
        priority: or(text(row, "priority"), "mid"),
        wait: text(row, "wait"),
        assignee: text(row, "assignee"),
        source: or(text(row, "source"), "hand"),
        body: text(row, "body"),
        fingerprint: text(row, "fingerprint"),
        count: int(row, "count").max(1),
        first_seen: real(row, "first_seen"),
        last_seen: real(row, "last_seen"),
        server: text(row, "server"),
        level: text(row, "level"),
        excerpt: text(row, "excerpt"),
        origin: text(row, "origin"),
    })
}

fn change_of(row: &Row) -> rusqlite::Result<Change> {
    Ok(Change {
        id: row.get("id")?,
        todo: int(row, "todo"),
        at: real(row, "at"),
        who: text(row, "who"),
        field: text(row, "field"),
        old: text(row, "old"),
        new: text(row, "new"),
        batch: text(row, "batch"),
        undone: int(row, "undone") != 0,
    })
}

pub fn valid(field: &str, value: &str) -> Option<String> {
    let v = value.trim();
    let lower = v.to_lowercase();
    match field {
        "task" => Some(oneline(v, TASK_CHARS)).filter(|t| !t.is_empty()),
        "kind" => KINDS.contains(&lower.as_str()).then_some(lower),
        "priority" => PRIOS.contains(&lower.as_str()).then_some(lower),
        "status" => STATUSES.contains(&lower.as_str()).then_some(lower),
        "wait" => (lower.is_empty() || WAITS.contains(&lower.as_str())).then_some(lower),
        "project" | "assignee" | "list" => Some(oneline(v, TASK_CHARS)),
        "body" => Some(value.chars().take(BODY_CHARS).collect::<String>().trim_end().to_string()),
        _ => None,
    }
}

pub fn prio_of(level: &str) -> &'static str {
    match level {
        "crash" => "high",
        "error" => "mid",
        _ => "low",
    }
}

fn stamp(words: &str) -> String {
    let [_, m, d, h, mi, _] = local_parts(now());
    format!("{m:02}-{d:02} {h:02}:{mi:02} {}", words.split_whitespace().collect::<Vec<_>>().join(" "))
}

impl Store {
    fn log_change(&self, id: i64, who: &str, field: &str, old: &str, new: &str, batch: &str) -> rusqlite::Result<()> {
        self.db.execute(
            "insert into todo_history(todo, at, who, field, old, new, batch) values (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, now(), who, field, old, new, batch],
        )?;
        Ok(())
    }

    pub fn todo(&self, id: i64) -> rusqlite::Result<Option<Todo>> {
        self.db.query_row("select * from todos where id = ?1", [id], todo_of).optional()
    }

    pub fn todo_add(&self, project: &str, task: &str, list: &str, status: &str) -> rusqlite::Result<i64> {
        self.entry_add(&NewEntry { project, task, list, status, ..NewEntry::default() })
    }

    pub fn entry_add(&self, e: &NewEntry) -> rusqlite::Result<i64> {
        let t = now();
        let status = or_str(e.status, "todo");
        let source = or_str(e.source, "hand");
        self.db.execute(
            "insert into todos(project, list, task, status, note, created, updated, origin, kind, priority, source, \
             body, server, assignee, first_seen, last_seen, count) \
             values (?1, ?2, ?3, ?4, '', ?5, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?5, ?5, 1)",
            params![
                e.project.trim(),
                e.list.trim(),
                oneline(e.task, TASK_CHARS),
                status,
                t,
                e.origin,
                or_str(e.kind, "task"),
                or_str(e.priority, "mid"),
                source,
                e.body.chars().take(BODY_CHARS).collect::<String>(),
                e.server.trim(),
                e.assignee.trim(),
            ],
        )?;
        let id = self.db.last_insert_rowid();
        self.log_change(id, or_str(e.who, source), "created", "", status, "")?;
        Ok(id)
    }

    pub fn entry_set(
        &self,
        id: i64,
        field: &str,
        value: &str,
        who: &str,
        batch: &str,
    ) -> rusqlite::Result<Option<Todo>> {
        let Some(old) = self.todo(id)? else { return Ok(None) };
        let Some(value) = valid(field, value) else { return Ok(Some(old)) };
        let before = old.field(field);
        if before == value {
            return Ok(Some(old));
        }
        self.db.execute(
            &format!("update todos set {field} = ?1, updated = ?2 where id = ?3"),
            params![value, now(), id],
        )?;
        self.log_change(id, who, field, &before, &value, batch)?;
        if field == "status" && value != "waiting" && !old.wait.is_empty() {
            self.entry_set(id, "wait", "", who, batch)?;
        }
        if field == "status" && value == "doing" && !who.is_empty() && old.assignee != who && who != "log" {
            self.entry_set(id, "assignee", who, who, batch)?;
        }
        self.todo(id)
    }

    pub fn todo_note(&self, id: i64, note: &str) -> rusqlite::Result<Option<Todo>> {
        let Some(old) = self.todo(id)? else { return Ok(None) };
        let add = note.trim();
        if add.is_empty() {
            return Ok(Some(old));
        }
        let mut text = old.note;
        if !text.is_empty() {
            text.push('\n');
        }
        text.push_str(&stamp(add));
        self.db.execute("update todos set note = ?1, updated = ?2 where id = ?3", params![text, now(), id])?;
        self.todo(id)
    }

    pub fn todo_set(&self, id: i64, status: Option<&str>, note: Option<&str>) -> rusqlite::Result<Option<Todo>> {
        self.todo_set_by(id, status, note, "")
    }

    pub fn todo_set_by(
        &self,
        id: i64,
        status: Option<&str>,
        note: Option<&str>,
        who: &str,
    ) -> rusqlite::Result<Option<Todo>> {
        if self.todo(id)?.is_none() {
            return Ok(None);
        }
        if let Some(s) = status {
            self.entry_set(id, "status", s, who, "")?;
        }
        if let Some(n) = note {
            self.todo_note(id, n)?;
        }
        self.todo(id)
    }

    pub fn todos(&self, project: Option<&str>, open_only: bool) -> rusqlite::Result<Vec<Todo>> {
        let mut sql = String::from(
            "select * from todos where status != 'suggested' and not (source = 'log' and status in ('todo', 'dropped'))",
        );
        if project.is_some() {
            sql.push_str(" and project = ?1");
        }
        if open_only {
            sql.push_str(" and status in ('todo', 'doing', 'waiting')");
        }
        sql.push_str(" order by list, status = 'doing' desc, id");
        let mut stmt = self.db.prepare(&sql)?;
        let rows = match project {
            Some(p) => stmt.query_map([p], todo_of)?.collect::<Result<Vec<_>, _>>()?,
            None => stmt.query_map([], todo_of)?.collect::<Result<Vec<_>, _>>()?,
        };
        Ok(rows)
    }

    pub fn entries(&self, project: Option<&str>, with_dropped: bool) -> rusqlite::Result<Vec<Todo>> {
        let mut sql = String::from("select * from todos where 1 = 1");
        if !with_dropped {
            sql.push_str(" and status != 'dropped'");
        }
        if project.is_some() {
            sql.push_str(" and (project = ?1 or server = ?1)");
        }
        sql.push_str(" order by id");
        let mut stmt = self.db.prepare(&sql)?;
        let rows = match project {
            Some(p) => stmt.query_map([p], todo_of)?.collect::<Result<Vec<_>, _>>()?,
            None => stmt.query_map([], todo_of)?.collect::<Result<Vec<_>, _>>()?,
        };
        Ok(rows)
    }

    pub fn history(&self, id: i64) -> rusqlite::Result<Vec<Change>> {
        let mut stmt = self.db.prepare("select * from todo_history where todo = ?1 order by id desc limit ?2")?;
        stmt.query_map(params![id, HISTORY_ROWS], change_of)?.collect()
    }

    fn revert(&self, c: &Change) -> rusqlite::Result<()> {
        if FIELDS.contains(&c.field.as_str()) {
            self.db.execute(
                &format!("update todos set {} = ?1, updated = ?2 where id = ?3", c.field),
                params![c.old, now(), c.todo],
            )?;
        }
        if c.field == "status" && c.new == "dropped" {
            self.db.execute(
                "delete from todo_mutes where fingerprint = (select fingerprint from todos where id = ?1)",
                [c.todo],
            )?;
        }
        self.db.execute("update todo_history set undone = 1 where id = ?1", [c.id])?;
        Ok(())
    }

    pub fn undo(&self, id: i64) -> rusqlite::Result<Vec<Change>> {
        let last: Option<Change> = self
            .db
            .query_row(
                "select * from todo_history where todo = ?1 and undone = 0 and field != 'created' order by id desc limit 1",
                [id],
                change_of,
            )
            .optional()?;
        let Some(last) = last else { return Ok(Vec::new()) };
        if !last.batch.is_empty() {
            let rows: Vec<Change> = self
                .history(id)?
                .into_iter()
                .filter(|c| !c.undone && c.batch == last.batch && c.field != "created")
                .collect();
            for c in &rows {
                self.revert(c)?;
            }
            return Ok(rows);
        }
        self.revert(&last)?;
        Ok(vec![last])
    }

    pub fn undo_batch(&self, batch: &str) -> rusqlite::Result<usize> {
        if batch.is_empty() {
            return Ok(0);
        }
        let mut stmt = self.db.prepare(
            "select * from todo_history where batch = ?1 and undone = 0 and field != 'created' order by id desc",
        )?;
        let rows: Vec<Change> = stmt.query_map([batch], change_of)?.collect::<Result<_, _>>()?;
        for c in &rows {
            self.revert(c)?;
        }
        Ok(rows.len())
    }
}

mod log;

pub use log::{LogHit, Mute, Seen, ServerState};

#[cfg(test)]
mod tests;
