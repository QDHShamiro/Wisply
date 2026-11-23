use rusqlite::{OptionalExtension, params};

use super::store::{Store, now, real, text};

const KEEP_S: f64 = 90.0 * 86_400.0;
const PRUNE_EVERY_S: f64 = 86_400.0;
const PRUNED: &str = "uses_pruned";
const AGENTS: &[(&str, &str)] = &[
    ("/.claude/", "Claude Code"),
    ("/.codex/", "Codex"),
    ("/.gemini/", "Gemini CLI"),
    ("/.cursor/", "Cursor"),
    ("/.qwen/", "Qwen Code"),
    ("/.opencode/", "opencode"),
];

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Use {
    pub session: String,
    pub how: String,
    pub n: i64,
    pub t: f64,
    pub title: String,
    pub project: String,
    pub cwd: String,
    pub agent: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Origin {
    pub id: String,
    pub title: String,
    pub project: String,
    pub cwd: String,
    pub agent: String,
    pub started: f64,
    pub last: f64,
}

pub fn agent_of(transcript: &str) -> String {
    let path = transcript.replace('\\', "/").to_lowercase();
    AGENTS.iter().find(|(part, _)| path.contains(part)).map(|(_, name)| name.to_string()).unwrap_or_default()
}

impl Store {
    pub fn used(&self, ids: &[i64], session: &str, how: &str) -> rusqlite::Result<()> {
        if ids.is_empty() || session.is_empty() {
            return Ok(());
        }
        let t = self.clock.unwrap_or_else(now);
        let mut stmt = self.db.prepare_cached(
            "insert into uses(note, session, how, n, t) values (?1, ?2, ?3, 1, ?4) \
             on conflict(note, session, how) do update set n = n + 1, t = excluded.t",
        )?;
        for id in ids {
            stmt.execute(params![id, session, how, t])?;
        }
        let last = self.meta(PRUNED)?.and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
        if t - last >= PRUNE_EVERY_S {
            self.db.execute("delete from uses where t < ?1", [t - KEEP_S])?;
            self.set_meta(PRUNED, &format!("{t:.0}"))?;
        }
        Ok(())
    }

    pub fn uses_of(&self, note: i64, limit: usize) -> rusqlite::Result<Vec<Use>> {
        let mut stmt = self.db.prepare(
            "select u.session, u.how, u.n, u.t, s.title, s.project, s.cwd, s.transcript from uses u \
             left join sessions s on s.id = u.session where u.note = ?1 order by u.t desc limit ?2",
        )?;
        stmt.query_map(params![note, limit as i64], |r| {
            Ok(Use {
                session: text(r, "session"),
                how: text(r, "how"),
                n: r.get::<_, Option<i64>>("n")?.unwrap_or(1),
                t: real(r, "t"),
                title: text(r, "title"),
                project: text(r, "project"),
                cwd: text(r, "cwd"),
                agent: agent_of(&text(r, "transcript")),
            })
        })?
        .collect()
    }

    pub fn use_counts(&self, note: i64) -> rusqlite::Result<Vec<(String, i64, i64)>> {
        let mut stmt = self.db.prepare(
            "select how, count(distinct session), coalesce(sum(n), 0) from uses where note = ?1 group by how order by how",
        )?;
        stmt.query_map([note], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?.collect()
    }

    pub fn origin(&self, session: &str) -> rusqlite::Result<Option<Origin>> {
        if session.is_empty() {
            return Ok(None);
        }
        let mut stmt =
            self.db.prepare("select id, title, project, cwd, transcript, started, last from sessions where id = ?1")?;
        let mut rows = stmt.query_map([session], |r| {
            Ok(Origin {
                id: text(r, "id"),
                title: text(r, "title"),
                project: text(r, "project"),
                cwd: text(r, "cwd"),
                agent: agent_of(&text(r, "transcript")),
                started: real(r, "started"),
                last: real(r, "last"),
            })
        })?;
        rows.next().transpose()
    }

    pub fn set_via(&self, note: i64, via: &str) -> rusqlite::Result<()> {
        self.db.execute("update notes set via = ?2 where id = ?1", params![note, via])?;
        Ok(())
    }

    pub fn via(&self, note: i64) -> rusqlite::Result<String> {
        let via = self.db.query_row("select coalesce(via, '') from notes where id = ?1", [note], |r| r.get(0));
        Ok(via.optional()?.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NewNote;

    #[test]
    fn a_note_keeps_who_got_it_and_where_it_came_from() {
        let home = std::env::temp_dir().join(format!("lt-uses-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        let mut s = Store::open(&home).unwrap();
        s.session_seen("s1", "web", "/w", "C:/Users/x/.claude/projects/w/s1.jsonl", false).unwrap();
        s.session_seen("s2", "web", "/w", "/home/x/.codex/sessions/s2.jsonl", false).unwrap();
        let id = s
            .add_note(&NewNote {
                project: "web",
                kind: "fact",
                title: "Port",
                body: "8080",
                source: "session",
                session: "s1",
            })
            .unwrap()
            .unwrap();
        s.set_clock(1_000.0);
        s.used(&[id], "s1", "start").unwrap();
        s.used(&[id], "s1", "start").unwrap();
        s.set_clock(2_000.0);
        s.used(&[id], "s2", "search").unwrap();
        s.used(&[id], "", "hint").unwrap();
        let uses = s.uses_of(id, 10).unwrap();
        assert_eq!(uses.len(), 2);
        assert_eq!((uses[0].session.as_str(), uses[0].how.as_str(), uses[0].agent.as_str()), ("s2", "search", "Codex"));
        assert_eq!((uses[1].n, uses[1].agent.as_str()), (2, "Claude Code"));
        assert_eq!(s.use_counts(id).unwrap(), vec![("search".into(), 1, 1), ("start".into(), 1, 2)]);
        let o = s.origin("s1").unwrap().unwrap();
        assert_eq!((o.project.as_str(), o.agent.as_str()), ("web", "Claude Code"));
        assert!(s.origin("").unwrap().is_none() && s.origin("zz").unwrap().is_none());
        s.set_via(id, "gemini").unwrap();
        assert_eq!(s.via(id).unwrap(), "gemini");
        s.set_clock(2_000.0 + KEEP_S + PRUNE_EVERY_S);
        s.used(&[id], "s3", "hint").unwrap();
        assert_eq!(s.uses_of(id, 10).unwrap().iter().map(|u| u.session.as_str()).collect::<Vec<_>>(), ["s3"]);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn the_ai_is_read_from_the_transcript_path() {
        assert_eq!(agent_of("C:\\Users\\x\\.claude\\projects\\a\\b.jsonl"), "Claude Code");
        assert_eq!(agent_of("/home/x/.gemini/tmp/c.json"), "Gemini CLI");
        assert_eq!(agent_of(""), "");
    }
}
