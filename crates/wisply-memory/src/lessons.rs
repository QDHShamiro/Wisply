use rusqlite::{OptionalExtension, params};

use super::store::{Store, now};

const MISSES_OUT: i64 = 2;
const KEEP_S: f64 = 30.0 * 86_400.0;
const TIDIED: &str = "lessons_tidied";

#[derive(Debug, Clone, PartialEq)]
pub struct Lesson {
    pub sig: String,
    pub cmd: String,
    pub target: String,
    pub kind: String,
    pub fix: String,
    pub ups: i64,
    pub downs: i64,
}

impl Store {
    fn cutoff(&self) -> f64 {
        self.clock.unwrap_or_else(now) - KEEP_S
    }

    pub fn lesson(&self, sig: &str) -> rusqlite::Result<Option<Lesson>> {
        self.db
            .query_row(
                "select sig, cmd, target, kind, fix, ups, downs from lessons where sig = ?1 and updated >= ?2",
                params![sig, self.cutoff()],
                |r| {
                    Ok(Lesson {
                        sig: r.get(0)?,
                        cmd: r.get::<_, Option<String>>(1)?.unwrap_or_default(),
                        target: r.get::<_, Option<String>>(2)?.unwrap_or_default(),
                        kind: r.get::<_, Option<String>>(3)?.unwrap_or_default(),
                        fix: r.get::<_, Option<String>>(4)?.unwrap_or_default(),
                        ups: r.get::<_, Option<i64>>(5)?.unwrap_or(0),
                        downs: r.get::<_, Option<i64>>(6)?.unwrap_or(0),
                    })
                },
            )
            .optional()
    }

    pub fn learn(&self, sig: &str, cmd: &str, target: &str, kind: &str, fix: &str, sid: &str) -> rusqlite::Result<()> {
        self.db.execute("delete from lessons where updated < ?1", [self.cutoff()])?;
        self.db.execute(
            "insert into lessons(sig, cmd, target, kind, fix, ups, downs, created, updated, last_sid) \
             values (?1, ?2, ?3, ?4, ?5, 1, 0, ?6, ?6, ?7) on conflict(sig) do update set \
             ups = case when lessons.fix <> excluded.fix then 1 \
                   when lessons.last_sid = excluded.last_sid then lessons.ups else lessons.ups + 1 end, \
             downs = case when lessons.fix = excluded.fix then lessons.downs else 0 end, \
             fix = excluded.fix, updated = excluded.updated, last_sid = excluded.last_sid",
            params![sig, cmd, target, kind, fix, self.clock.unwrap_or_else(now), sid],
        )?;
        Ok(())
    }

    pub fn vote(&self, sig: &str, good: bool, sid: &str) -> rusqlite::Result<bool> {
        let at = self.clock.unwrap_or_else(now);
        if good {
            self.db.execute(
                "update lessons set ups = ups + (last_sid is not ?3), downs = 0, updated = ?2, last_sid = ?3 where sig = ?1",
                params![sig, at, sid],
            )?;
        } else {
            self.db.execute("update lessons set downs = downs + 1, updated = ?2 where sig = ?1", params![sig, at])?;
        }
        let gone = self.db.execute("delete from lessons where sig = ?1 and downs >= ?2", params![sig, MISSES_OUT])?;
        Ok(gone > 0)
    }

    pub fn lessons(&self) -> rusqlite::Result<Vec<Lesson>> {
        let mut stmt = self.db.prepare("select sig from lessons where updated >= ?1 order by updated desc")?;
        let sigs: Vec<String> = stmt.query_map([self.cutoff()], |r| r.get(0))?.collect::<Result<_, _>>()?;
        Ok(sigs.iter().filter_map(|s| self.lesson(s).ok().flatten()).collect())
    }

    pub fn unlearn(&self, sig: &str) -> rusqlite::Result<usize> {
        self.db.execute("delete from lessons where sig = ?1", [sig])
    }

    pub fn tidy_lessons(&self, keep: impl Fn(&Lesson) -> bool) -> rusqlite::Result<usize> {
        if self.meta(TIDIED)?.is_some() {
            return Ok(0);
        }
        self.db.execute_batch("create table if not exists lessons_old as select * from lessons")?;
        let mut gone = self.db.execute("delete from lessons where updated < ?1", [self.cutoff()])?;
        for l in self.lessons()? {
            if !keep(&l) {
                gone += self.unlearn(&l.sig)?;
            }
        }
        self.set_meta(TIDIED, &self.clock.unwrap_or_else(now).to_string())?;
        Ok(gone)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn store(tag: &str) -> (Store, std::path::PathBuf) {
        let dir = std::env::temp_dir().join(format!("ltm-lessons-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        (Store::open(&dir).unwrap(), dir)
    }

    #[test]
    fn lessons_rise_and_drop_out_after_two_misses() {
        let (s, dir) = store("rise");
        let sig = "click|example.com|element e# not found";
        assert!(s.lesson(sig).unwrap().is_none());
        s.learn(sig, "click", "example.com", "element e# not found", "snap", "a").unwrap();
        s.learn(sig, "click", "example.com", "element e# not found", "snap", "b").unwrap();
        assert_eq!(s.lesson(sig).unwrap().map(|l| (l.fix, l.ups)), Some(("snap".into(), 2)));
        s.learn(sig, "click", "example.com", "element e# not found", "scroll down", "b").unwrap();
        assert_eq!(s.lesson(sig).unwrap().map(|l| (l.fix, l.ups)), Some(("scroll down".into(), 1)));
        assert!(!s.vote(sig, false, "b").unwrap());
        assert!(s.vote(sig, false, "b").unwrap());
        assert!(s.lesson(sig).unwrap().is_none() && s.lessons().unwrap().is_empty());
        s.learn("a|b|c", "a", "b", "c", "x", "").unwrap();
        assert_eq!(s.lessons().unwrap().len(), 1);
        assert_eq!(s.unlearn("a|b|c").unwrap(), 1);
        drop(s);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn one_session_counts_once_and_old_lessons_expire() {
        let (mut s, dir) = store("once");
        let sig = "click|a.b|x";
        s.set_clock(1_000_000.0);
        s.learn(sig, "click", "a.b", "x", "snap", "s1").unwrap();
        s.learn(sig, "click", "a.b", "x", "snap", "s1").unwrap();
        assert!(!s.vote(sig, true, "s1").unwrap());
        assert_eq!(s.lesson(sig).unwrap().unwrap().ups, 1);
        s.vote(sig, true, "s2").unwrap();
        s.vote(sig, true, "s2").unwrap();
        assert_eq!(s.lesson(sig).unwrap().unwrap().ups, 2);
        s.set_clock(1_000_000.0 + KEEP_S + 1.0);
        assert!(s.lesson(sig).unwrap().is_none() && s.lessons().unwrap().is_empty());
        s.learn(sig, "click", "a.b", "x", "snap", "s3").unwrap();
        assert_eq!(s.lesson(sig).unwrap().unwrap().ups, 1);
        drop(s);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn tidy_runs_once_and_keeps_a_backup() {
        let (mut s, dir) = store("tidy");
        s.set_clock(5_000_000.0);
        s.learn("click|a|x", "click", "a", "x", "snap", "").unwrap();
        s.learn("click|a|y", "click", "a", "y", "open https://a", "").unwrap();
        s.set_clock(1_000.0);
        s.learn("type|a|z", "type", "a", "z", "type z", "").unwrap();
        s.set_clock(5_000_000.0);
        assert_eq!(s.tidy_lessons(|l| !l.fix.starts_with("open")).unwrap(), 2);
        assert_eq!(s.lessons().unwrap().iter().map(|l| l.sig.as_str()).collect::<Vec<_>>(), ["click|a|x"]);
        let kept: i64 = s.db.query_row("select count(*) from lessons_old", [], |r| r.get(0)).unwrap();
        assert_eq!(kept, 3);
        s.learn("click|a|w", "click", "a", "w", "open x", "").unwrap();
        assert_eq!(s.tidy_lessons(|_| false).unwrap(), 0);
        drop(s);
        let _ = fs::remove_dir_all(&dir);
    }
}
