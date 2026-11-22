use rusqlite::params;

use super::store::Store;

#[derive(Debug, Clone, PartialEq)]
pub struct DayCount {
    pub day: String,
    pub key: String,
    pub n: i64,
}

const PER_DAY: &str = "
select date(created, 'unixepoch', 'localtime'), 'notes:' || coalesce(source, ''), count(*) from notes
  where created >= ?1 group by 1, 2
union all
select date(created, 'unixepoch', 'localtime'), 'lessons', count(*) from lessons where created >= ?1 group by 1
union all
select date(learned, 'unixepoch', 'localtime'), 'sessions', count(*) from sessions
  where learned > 0 and learned >= ?1 group by 1
union all
select date(at, 'unixepoch', 'localtime'), 'done', count(*) from todo_history
  where field = 'status' and new = 'done' and undone = 0 and at >= ?1 group by 1
union all
select date(created, 'unixepoch', 'localtime'), 'found:' || coalesce(kind, 'task'), count(*) from todos
  where created >= ?1 group by 1, 2
";

const TOTALS: &str = "
select 'notes', count(*) from notes
union all select 'notes:' || coalesce(source, ''), count(*) from notes group by 1
union all select 'lessons', count(*) from lessons
union all select 'ups', coalesce(sum(ups), 0) from lessons
union all select 'downs', coalesce(sum(downs), 0) from lessons
union all select 'sessions', count(*) from sessions where learned > 0
union all select 'done', count(*) from todo_history where field = 'status' and new = 'done' and undone = 0
union all select 'hits', coalesce(sum(hits), 0) from notes
union all select 'recalled', count(*) from notes where hits > 0
union all select 'found', count(*) from todos
union all select 'first', coalesce(cast(min(created) as integer), 0) from notes
";

impl Store {
    pub fn growth(&self, since: f64) -> rusqlite::Result<Vec<DayCount>> {
        let mut stmt = self.db.prepare(PER_DAY)?;
        stmt.query_map(params![since], |r| {
            Ok(DayCount { day: r.get::<_, Option<String>>(0)?.unwrap_or_default(), key: r.get(1)?, n: r.get(2)? })
        })?
        .collect()
    }

    pub fn growth_totals(&self) -> rusqlite::Result<Vec<(String, i64)>> {
        let mut stmt = self.db.prepare(TOTALS)?;
        stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?.collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{NewEntry, NewNote, Store, now};

    #[test]
    fn growth_counts_notes_lessons_and_done_entries_per_day() {
        let home = std::env::temp_dir().join(format!("lt-growth-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        let mut s = Store::open(&home).unwrap();
        s.add_note(&NewNote {
            project: "p",
            kind: "fact",
            title: "Port",
            body: "uses 8080",
            source: "user",
            session: "",
        })
        .unwrap();
        let id = s
            .entry_add(&NewEntry { task: "ship it", kind: "bug", status: "todo", who: "you", ..NewEntry::default() })
            .unwrap();
        s.entry_set(id, "status", "done", "you", "b1").unwrap();
        let rows = s.growth(now() - 86_400.0).unwrap();
        let n = |k: &str| rows.iter().filter(|c| c.key == k).map(|c| c.n).sum::<i64>();
        assert_eq!((n("notes:user"), n("done"), n("found:bug")), (1, 1, 1));
        let totals = s.growth_totals().unwrap();
        let t = |k: &str| totals.iter().find(|(key, _)| key == k).map(|(_, n)| *n).unwrap_or(-1);
        assert_eq!((t("notes"), t("notes:user"), t("done"), t("found")), (1, 1, 1, 1));
        assert!(s.growth(now() + 86_400.0).unwrap().is_empty());
        let _ = std::fs::remove_dir_all(&home);
    }
}
