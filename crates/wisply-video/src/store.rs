use std::path::Path;
use std::time::Duration;

use rusqlite::{Connection, OptionalExtension, Transaction, TransactionBehavior, params};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::fail::{Fail, Kind};

pub const WHOLE_VIDEO: &str = "";

const MIGRATIONS: [&str; 4] = [
    "CREATE TABLE video (video_id TEXT PRIMARY KEY, fetched_at REAL NOT NULL, info TEXT NOT NULL);
     CREATE TABLE analysis (
         digest TEXT PRIMARY KEY, video_id TEXT NOT NULL, section TEXT NOT NULL, mode TEXT NOT NULL,
         focus TEXT NOT NULL, prompt_version TEXT NOT NULL, model TEXT NOT NULL, profile TEXT NOT NULL,
         created_at REAL NOT NULL, payload TEXT NOT NULL, usage TEXT NOT NULL);
     CREATE INDEX analysis_by_video ON analysis (video_id, mode);
     CREATE TABLE quota_event (
         id INTEGER PRIMARY KEY AUTOINCREMENT, scope TEXT NOT NULL, at REAL NOT NULL, requests INTEGER NOT NULL,
         tokens INTEGER NOT NULL, youtube_seconds REAL NOT NULL, audio_seconds REAL NOT NULL);
     CREATE INDEX quota_event_by_time ON quota_event (at);
     CREATE TABLE quota_block (scope TEXT PRIMARY KEY, until REAL NOT NULL, reason TEXT NOT NULL);
     CREATE TABLE quota_learned (
         scope TEXT NOT NULL, rule TEXT NOT NULL, value REAL NOT NULL, learned_at REAL NOT NULL,
         PRIMARY KEY (scope, rule));",
    "CREATE TABLE transcript (
         video_key TEXT PRIMARY KEY, kind TEXT NOT NULL, language TEXT NOT NULL, note TEXT NOT NULL,
         created_at REAL NOT NULL);
     CREATE TABLE transcript_part (
         video_key TEXT NOT NULL, part INTEGER NOT NULL, start_seconds REAL NOT NULL, end_seconds REAL NOT NULL,
         segments TEXT NOT NULL, PRIMARY KEY (video_key, part));
     CREATE VIRTUAL TABLE transcript_search USING fts5(
         text, video_key UNINDEXED, part UNINDEXED, start_seconds UNINDEXED, end_seconds UNINDEXED);",
    "CREATE TABLE spend (
         id INTEGER PRIMARY KEY AUTOINCREMENT, at REAL NOT NULL, model TEXT NOT NULL, estimate_usd REAL NOT NULL,
         cost_usd REAL);
     CREATE INDEX spend_by_time ON spend (at);",
    "CREATE TABLE answer (
         digest TEXT PRIMARY KEY, video_key TEXT NOT NULL, question TEXT NOT NULL, model TEXT NOT NULL,
         created_at REAL NOT NULL, payload TEXT NOT NULL, usage TEXT NOT NULL);
     CREATE INDEX answer_by_video ON answer (video_key);",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisKey {
    pub video_id: String,
    pub section: String,
    pub mode: String,
    pub focus: String,
    pub prompt_version: String,
    pub model: String,
    pub profile: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cached {
    pub payload: Value,
    pub usage: Value,
    pub created_at: f64,
}

pub struct Store {
    conn: Connection,
}

pub fn normalize_focus(focus: &str) -> String {
    let folded = focus.to_lowercase().replace('ß', "ss").replace('ς', "σ");
    folded.split_whitespace().collect::<Vec<&str>>().join(" ")
}

impl AnalysisKey {
    pub fn digest(&self) -> String {
        let fields =
            [&self.video_id, &self.section, &self.mode, &self.focus, &self.prompt_version, &self.model, &self.profile];
        let text = serde_json::to_string(&fields).unwrap_or_default();
        Sha256::digest(text.as_bytes()).iter().map(|byte| format!("{byte:02x}")).collect()
    }
}

impl Store {
    pub fn open(path: &Path, busy_timeout_seconds: f64) -> Result<Store, Fail> {
        if let Some(folder) = path.parent() {
            std::fs::create_dir_all(folder).map_err(|error| {
                Fail::new(Kind::Storage, format!("the folder {} cannot be created ({error})", folder.display()))
            })?;
        }
        Store::ready(Connection::open(path)?, busy_timeout_seconds)
    }

    #[cfg(test)]
    pub(crate) fn in_memory() -> Store {
        Store::ready(Connection::open_in_memory().expect("memory"), 1.0).expect("schema")
    }

    fn ready(conn: Connection, busy_timeout_seconds: f64) -> Result<Store, Fail> {
        conn.busy_timeout(Duration::from_secs_f64(busy_timeout_seconds.max(0.0)))?;
        conn.pragma_update(None, "journal_mode", "wal")?;
        conn.pragma_update(None, "synchronous", "normal")?;
        let store = Store { conn };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> Result<(), Fail> {
        let tx = self.write()?;
        let version: i64 = tx.pragma_query_value(None, "user_version", |row| row.get(0))?;
        let known = MIGRATIONS.len();
        let done = usize::try_from(version).unwrap_or(0);
        if done > known {
            let message = format!("the video cache is from a newer wisply (version {done}, this one knows {known})");
            return Err(Fail::hint(Kind::Storage, message, "update wisply, or delete video.db in the pupil folder"));
        }
        for (index, sql) in MIGRATIONS.iter().enumerate().skip(done) {
            tx.execute_batch(sql)?;
            tx.pragma_update(None, "user_version", index as i64 + 1)?;
        }
        Ok(tx.commit()?)
    }

    pub fn version(&self) -> Result<i64, Fail> {
        Ok(self.conn.pragma_query_value(None, "user_version", |row| row.get(0))?)
    }

    pub(crate) fn read(&self) -> &Connection {
        &self.conn
    }

    pub(crate) fn write(&self) -> Result<Transaction<'_>, Fail> {
        Ok(Transaction::new_unchecked(&self.conn, TransactionBehavior::Immediate)?)
    }

    pub fn analysis(&self, key: &AnalysisKey) -> Result<Option<Cached>, Fail> {
        let sql = "SELECT payload, usage, created_at FROM analysis WHERE digest = ?1";
        let row: Option<(String, String, f64)> =
            self.conn.query_row(sql, [key.digest()], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).optional()?;
        let Some((payload, usage, created_at)) = row else { return Ok(None) };
        Ok(Some(Cached { payload: serde_json::from_str(&payload)?, usage: serde_json::from_str(&usage)?, created_at }))
    }

    pub fn put_analysis(&self, key: &AnalysisKey, payload: &Value, usage: &Value, at: f64) -> Result<(), Fail> {
        let values = params![
            key.digest(),
            key.video_id,
            key.section,
            key.mode,
            key.focus,
            key.prompt_version,
            key.model,
            key.profile,
            at,
            serde_json::to_string(payload)?,
            serde_json::to_string(usage)?,
        ];
        self.conn
            .execute("INSERT OR REPLACE INTO analysis VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)", values)?;
        Ok(())
    }

    pub fn count_answers(&self) -> Result<i64, Fail> {
        let sql = "SELECT (SELECT COUNT(*) FROM analysis) + (SELECT COUNT(*) FROM answer)";
        Ok(self.conn.query_row(sql, [], |row| row.get(0))?)
    }

    pub fn video_info(&self, video_id: &str, max_age_seconds: f64, now: f64) -> Result<Option<Value>, Fail> {
        let sql = "SELECT fetched_at, info FROM video WHERE video_id = ?1";
        let row: Option<(f64, String)> =
            self.conn.query_row(sql, [video_id], |row| Ok((row.get(0)?, row.get(1)?))).optional()?;
        match row {
            Some((fetched_at, info)) if now - fetched_at <= max_age_seconds => Ok(Some(serde_json::from_str(&info)?)),
            _ => Ok(None),
        }
    }

    pub fn put_video_info(&self, video_id: &str, info: &Value, at: f64) -> Result<(), Fail> {
        let values = params![video_id, at, serde_json::to_string(info)?];
        self.conn.execute("INSERT OR REPLACE INTO video VALUES (?1, ?2, ?3)", values)?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "store_tests.rs"]
mod tests;
