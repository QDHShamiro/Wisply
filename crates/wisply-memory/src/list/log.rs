use rusqlite::{OptionalExtension, Row, params};

use super::{NewEntry, Todo, int, prio_of, todo_of};
use crate::store::{Store, TASK_CHARS, now, real, text};
use crate::text::{oneline, similar};

const EXCERPT_CHARS: usize = 6_000;
const SAME_TASK: f64 = 0.7;
const MIN_SUGGESTION: usize = 8;

#[derive(Debug, Clone, Default)]
pub struct LogHit<'a> {
    pub server: &'a str,
    pub fingerprint: &'a str,
    pub title: &'a str,
    pub level: &'a str,
    pub excerpt: &'a str,
    pub count: i64,
    pub at: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seen {
    New(i64),
    Again(i64),
    Back(i64),
    Quiet(i64),
    Muted,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ServerState {
    pub name: String,
    pub enabled: bool,
    pub log: String,
    pub offset: i64,
    pub head: String,
    pub checked: f64,
    pub ok: f64,
    pub error: String,
    pub started: f64,
    pub crashes: String,
    pub hostkey: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mute {
    pub fingerprint: String,
    pub server: String,
    pub title: String,
    pub at: f64,
}

fn server_of(row: &Row) -> rusqlite::Result<ServerState> {
    Ok(ServerState {
        name: text(row, "name"),
        enabled: int(row, "enabled") != 0,
        log: text(row, "log"),
        offset: int(row, "offset"),
        head: text(row, "head"),
        checked: real(row, "checked"),
        ok: real(row, "ok"),
        error: text(row, "error"),
        started: real(row, "started"),
        crashes: text(row, "crashes"),
        hostkey: text(row, "hostkey"),
    })
}

impl Store {
    pub fn muted(&self, fingerprint: &str) -> rusqlite::Result<bool> {
        self.db
            .query_row("select 1 from todo_mutes where fingerprint = ?1", [fingerprint], |_| Ok(()))
            .optional()
            .map(|r| r.is_some())
    }

    pub fn mute(&self, id: i64, who: &str) -> rusqlite::Result<Option<Todo>> {
        let Some(t) = self.todo(id)? else { return Ok(None) };
        if !t.fingerprint.is_empty() {
            self.db.execute(
                "insert or replace into todo_mutes(fingerprint, server, title, at) values (?1, ?2, ?3, ?4)",
                params![t.fingerprint, t.server, t.task, now()],
            )?;
        }
        self.entry_set(id, "status", "dropped", who, "")
    }

    pub fn unmute(&self, fingerprint: &str) -> rusqlite::Result<usize> {
        self.db.execute("delete from todo_mutes where fingerprint = ?1", [fingerprint])
    }

    pub fn mutes(&self) -> rusqlite::Result<Vec<Mute>> {
        let mut stmt = self.db.prepare("select * from todo_mutes order by at desc")?;
        stmt.query_map([], |r| {
            Ok(Mute {
                fingerprint: text(r, "fingerprint"),
                server: text(r, "server"),
                title: text(r, "title"),
                at: real(r, "at"),
            })
        })?
        .collect()
    }

    pub fn log_hit(&self, h: &LogHit) -> rusqlite::Result<Seen> {
        if self.muted(h.fingerprint)? {
            return Ok(Seen::Muted);
        }
        let excerpt: String = h.excerpt.chars().take(EXCERPT_CHARS).collect();
        let n = h.count.max(1);
        let known: Option<Todo> = self
            .db
            .query_row(
                "select * from todos where server = ?1 and fingerprint = ?2 and source = 'log' order by id desc limit 1",
                params![h.server, h.fingerprint],
                todo_of,
            )
            .optional()?;
        if let Some(t) = known {
            self.db.execute(
                "update todos set count = count + ?1, last_seen = ?2, excerpt = ?3 where id = ?4",
                params![n, h.at, excerpt, t.id],
            )?;
            return Ok(match t.status.as_str() {
                "done" => {
                    self.entry_set(t.id, "status", "todo", "log", "")?;
                    self.todo_note(t.id, "kam wieder")?;
                    Seen::Back(t.id)
                }
                "dropped" => Seen::Quiet(t.id),
                _ => Seen::Again(t.id),
            });
        }
        let id = self.entry_add(&NewEntry {
            project: h.server,
            task: h.title,
            kind: "bug",
            priority: prio_of(h.level),
            source: "log",
            server: h.server,
            who: "log",
            ..NewEntry::default()
        })?;
        self.db.execute(
            "update todos set fingerprint = ?1, level = ?2, excerpt = ?3, count = ?4, first_seen = ?5, last_seen = ?5 \
             where id = ?6",
            params![h.fingerprint, h.level, excerpt, n, h.at, id],
        )?;
        Ok(Seen::New(id))
    }

    pub fn restart_done(&self, server: &str, batch: &str) -> rusqlite::Result<Vec<i64>> {
        let mut stmt = self.db.prepare(
            "select id from todos where status = 'waiting' and wait = 'restart' \
             and (server = ?1 or (server = '' and project = ?1)) order by id",
        )?;
        let ids: Vec<i64> = stmt.query_map([server], |r| r.get(0))?.collect::<Result<_, _>>()?;
        for id in &ids {
            self.entry_set(*id, "status", "done", "restart", batch)?;
            self.todo_note(*id, "Server neu gestartet")?;
        }
        Ok(ids)
    }

    pub fn suggest(&self, project: &str, task: &str, origin: &str) -> rusqlite::Result<Option<i64>> {
        let task = oneline(task.trim_start_matches(['-', '*', ' ']), TASK_CHARS);
        if task.chars().count() < MIN_SUGGESTION {
            return Ok(None);
        }
        let mut stmt = self.db.prepare("select task from todos where project = ?1")?;
        let seen: Vec<String> = stmt.query_map([project], |r| r.get(0))?.collect::<Result<_, _>>()?;
        if seen.iter().any(|t| t.eq_ignore_ascii_case(&task) || similar(t, &task) >= SAME_TASK) {
            return Ok(None);
        }
        self.entry_add(&NewEntry {
            project,
            task: &task,
            status: "suggested",
            source: "suggestion",
            origin,
            who: "suggestion",
            ..NewEntry::default()
        })
        .map(Some)
    }

    pub fn server_state(&self, name: &str) -> rusqlite::Result<Option<ServerState>> {
        self.db.query_row("select * from servers where name = ?1", [name], server_of).optional()
    }

    pub fn server_save(&self, s: &ServerState) -> rusqlite::Result<()> {
        self.db.execute(
            "insert or replace into servers(name, enabled, log, offset, head, checked, ok, error, started, crashes, hostkey) \
             values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![s.name, s.enabled, s.log, s.offset, s.head, s.checked, s.ok, s.error, s.started, s.crashes, s.hostkey],
        )?;
        Ok(())
    }

    pub fn servers(&self) -> rusqlite::Result<Vec<ServerState>> {
        let mut stmt = self.db.prepare("select * from servers order by name")?;
        stmt.query_map([], server_of)?.collect()
    }
}
