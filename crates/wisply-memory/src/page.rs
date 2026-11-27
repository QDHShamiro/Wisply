use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use rusqlite::params;

use super::pages::safe_name;
use super::store::{Store, date};
use super::text::oneline;

pub const DIR: &str = "Projects";
const MARK: &str = "wisply: project-page";
const LEARNED: &str = "source not in ('vault', 'claude', 'files', 'log') and origin not like 's:%'";
const MIN_ROWS: i64 = 3;
const FACTS: i64 = 12;
const SESSIONS: i64 = 6;
const TASKS: i64 = 10;
const DOCS: usize = 8;
const FACT_CHARS: usize = 170;
const TITLE_CHARS: usize = 110;
const NAME_CHARS: usize = 80;
const DOC_SCAN: i64 = 400;
const OPEN_COUNT: &str = "select count(*) from todos where project = ?1 and status in ('todo', 'doing', 'waiting')";
const OPEN_TASKS: &str = "select status, task, note from todos where project = ?1     and status in ('todo', 'doing', 'waiting') order by updated desc limit ?2";
const RECENT: &str = "select cast(last as text), title, '' from sessions where project = ?1 and title != ''     order by last desc limit ?2";
const DOC_ORIGINS: &str =
    "select origin, title, '' from notes where project = ?1 and source = 'files'     order by origin limit ?2";

fn line(text: &str, n: usize) -> String {
    oneline(text, n)
}

impl Store {
    fn count(&self, sql: &str, project: &str) -> rusqlite::Result<i64> {
        self.db.query_row(sql, [project], |r| r.get(0))
    }

    fn rows(&self, sql: &str, project: &str, limit: i64) -> rusqlite::Result<Vec<(String, String, String)>> {
        let mut stmt = self.db.prepare(sql)?;
        let rows = stmt.query_map(params![project, limit], |r| {
            Ok((
                r.get::<_, Option<String>>(0)?.unwrap_or_default(),
                r.get::<_, Option<String>>(1)?.unwrap_or_default(),
                r.get::<_, Option<String>>(2)?.unwrap_or_default(),
            ))
        })?;
        rows.collect()
    }

    fn stats(&self, name: &str, notes: i64, sessions: i64) -> rusqlite::Result<String> {
        let open = self.count(OPEN_COUNT, name)?;
        let last: f64 = self.db.query_row(
            &format!(
                "select max(t) from (select max(updated) t from notes where project = ?1 and {LEARNED}                  union all select max(last) from sessions where project = ?1)"
            ),
            [name],
            |r| r.get::<_, Option<f64>>(0).map(Option::unwrap_or_default),
        )?;
        let when = if last > 0.0 { date(last) } else { "-".into() };
        Ok(format!("Notes {notes} · Sessions {sessions} · Open tasks {open} · Last work {when}"))
    }

    fn facts(&self, name: &str) -> rusqlite::Result<Vec<String>> {
        let sql = format!(
            "select kind, title, body from notes where project = ?1 and {LEARNED} order by hits desc, updated desc limit ?2"
        );
        Ok(self.rows(&sql, name, FACTS)?.iter().map(|(k, t, b)| fact(k, t, b)).collect())
    }

    fn tasks(&self, name: &str) -> rusqlite::Result<Vec<String>> {
        let rows = self.rows(OPEN_TASKS, name, TASKS)?;
        Ok(rows.iter().map(|(s, t, n)| task(s, t, n)).collect())
    }

    fn recent(&self, name: &str) -> rusqlite::Result<Vec<String>> {
        let rows = self.rows(RECENT, name, SESSIONS)?;
        Ok(rows
            .iter()
            .map(|(t, title, _)| format!("- {} {}", date(t.parse().unwrap_or(0.0)), line(title, TITLE_CHARS)))
            .collect())
    }

    fn documents(&self, name: &str) -> rusqlite::Result<Vec<String>> {
        Ok(documents(&self.rows(DOC_ORIGINS, name, DOC_SCAN)?))
    }

    pub fn project_page(&self, name: &str) -> rusqlite::Result<Option<String>> {
        let notes = self.count(&format!("select count(*) from notes where project = ?1 and {LEARNED}"), name)?;
        let sessions = self.count("select count(*) from sessions where project = ?1", name)?;
        if notes + sessions < MIN_ROWS {
            return Ok(None);
        }
        let mut out: Vec<String> = ["---", "type: project"].iter().map(|s| s.to_string()).collect();
        out.extend([format!("project: {name}"), MARK.into(), "---".into(), format!("# {name}"), String::new()]);
        out.extend([self.stats(name, notes, sessions)?, String::new()]);
        section(&mut out, "Key facts", self.facts(name)?);
        section(&mut out, "Open tasks", self.tasks(name)?);
        section(&mut out, "Recent sessions", self.recent(name)?);
        section(&mut out, "Documents", self.documents(name)?);
        Ok(Some(out.join("\n") + "\n"))
    }

    pub fn write_project_pages(&self, vault: &Path) -> rusqlite::Result<usize> {
        let dir = vault.join(DIR);
        let mut written = 0;
        for name in self.known_projects()? {
            let Some(page) = self.project_page(&name)? else { continue };
            let path = dir.join(format!("{}.md", safe_name(&name, NAME_CHARS)));
            let old = fs::read_to_string(&path).ok();
            if old.as_deref() == Some(page.as_str()) || old.as_deref().is_some_and(|o| !o.contains(MARK)) {
                continue;
            }
            if fs::create_dir_all(&dir).is_ok() && wisply_core::save(&path, &page).is_ok() {
                written += 1;
            }
        }
        Ok(written)
    }
}

fn section(out: &mut Vec<String>, head: &str, items: Vec<String>) {
    if items.is_empty() {
        return;
    }
    out.push(format!("## {head}"));
    out.extend(items);
    out.push(String::new());
}

fn fact(kind: &str, title: &str, body: &str) -> String {
    let first = body.lines().map(str::trim).find(|l| !l.is_empty() && *l != title).unwrap_or("");
    let tail = if first.is_empty() { String::new() } else { format!(" — {}", line(first, FACT_CHARS)) };
    let tag = if kind.is_empty() { String::new() } else { format!("[{kind}] ") };
    format!("- {tag}{}{tail}", line(title, TITLE_CHARS))
}

fn task(status: &str, task: &str, note: &str) -> String {
    let tail = if note.trim().is_empty() { String::new() } else { format!(" — {}", line(note, FACT_CHARS)) };
    format!("- ({status}) {}{tail}", line(task, TITLE_CHARS))
}

fn documents(rows: &[(String, String, String)]) -> Vec<String> {
    let mut files: BTreeMap<String, usize> = BTreeMap::new();
    for (origin, _, _) in rows {
        let path = origin.split_once(':').map_or(origin.as_str(), |(_, p)| p);
        let file = path.rsplit_once('#').map_or(path, |(f, _)| f);
        *files.entry(file.to_string()).or_default() += 1;
    }
    files.into_iter().take(DOCS).map(|(file, n)| format!("- {file} ({n} sections)")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::NewNote;

    #[test]
    fn a_project_gets_one_page_with_its_facts_and_never_overwrites_a_hand_written_one() {
        let base = std::env::temp_dir().join(format!("ltm-page-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        let mut s = Store::open(&base).unwrap();
        for (project, kind, title, body) in [
            ("Boxes", "decision", "Anchors may break blocks", "pvp area allows block damage\nmore"),
            ("Boxes", "rule", "Never restart live", "players are online"),
            ("Boxes", "note", "Third", "text"),
            ("Tiny", "note", "Only one", "text"),
        ] {
            s.add_note(&NewNote { project, kind, title, body, source: "user", session: "" }).unwrap();
        }
        let page = s.project_page("Boxes").unwrap().unwrap();
        assert!(page.starts_with("---\ntype: project\nproject: Boxes\nwisply: project-page\n---\n# Boxes\n"));
        assert!(page.contains("## Key facts"));
        assert!(page.contains("- [decision] Anchors may break blocks — pvp area allows block damage"));
        assert!(s.project_page("Tiny").unwrap().is_none());
        let vault = base.join("Vault");
        assert_eq!(s.write_project_pages(&vault).unwrap(), 1);
        assert_eq!(s.write_project_pages(&vault).unwrap(), 0);
        let file = vault.join(DIR).join("Boxes.md");
        assert_eq!(fs::read_to_string(&file).unwrap(), page);
        fs::write(&file, "# mine\nhand written").unwrap();
        assert_eq!(s.write_project_pages(&vault).unwrap(), 0);
        assert_eq!(fs::read_to_string(&file).unwrap(), "# mine\nhand written");
        drop(s);
        let _ = fs::remove_dir_all(&base);
    }
}
