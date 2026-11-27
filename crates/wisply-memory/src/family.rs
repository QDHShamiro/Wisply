use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::store::Store;

const BACKUP: &str = "memory.db.bak-before-families";
const TABLES: &[&str] = &["notes", "sessions", "todos"];
const KNOWN: &str = "select project, sum(n) c from (\
    select project, count(*) n from notes group by project \
    union all select project, count(*) from sessions group by project \
    union all select project, count(*) from todos group by project) \
    where project is not null and project != '' group by project order by c desc, project";

fn splits(c: char) -> bool {
    c.is_uppercase() || c == '-' || c == '_' || c.is_ascii_digit()
}

pub fn root(name: &str, known: &[String]) -> String {
    let lower = name.to_lowercase();
    let cut = name.chars().count();
    let parent = known
        .iter()
        .filter(|x| {
            let n = x.chars().count();
            n > 0 && n < cut && lower.starts_with(&x.to_lowercase()) && name.chars().nth(n).is_some_and(splits)
        })
        .min_by_key(|x| x.chars().count());
    if let Some(parent) = parent {
        return parent.clone();
    }
    known.iter().find(|x| x.to_lowercase() == lower).cloned().unwrap_or_else(|| name.to_string())
}

impl Store {
    pub fn known_projects(&self) -> rusqlite::Result<Vec<String>> {
        let mut stmt = self.db.prepare(KNOWN)?;
        let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
        rows.collect()
    }

    pub fn fold_projects(&mut self, home: &Path) -> rusqlite::Result<usize> {
        let known = self.known_projects()?;
        let moves: HashMap<&String, String> =
            known.iter().map(|p| (p, root(p, &known))).filter(|(p, r)| *p != r).collect();
        if moves.is_empty() {
            return Ok(0);
        }
        let backup = home.join("memory").join(BACKUP);
        if !backup.exists() {
            let target = backup.to_string_lossy().into_owned();
            self.db.execute("vacuum into ?1", [target.as_str()])?;
            if !backup.exists() {
                let _ = fs::write(&backup, b"");
            }
        }
        let tx = self.db.transaction()?;
        let mut changed = 0;
        for (from, to) in &moves {
            for table in TABLES {
                changed += tx.execute(
                    &format!("update {table} set project = ?2 where project = ?1"),
                    [from.as_str(), to.as_str()],
                )?;
            }
        }
        tx.commit()?;
        Ok(changed)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::store::NewNote;

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn names_fold_into_the_project_they_extend() {
        let known =
            names(&["CpvpGens", "CpvpGensStore", "cpvpgens-font", "CpvpGens-Promo", "AsyncShop", "AsyncFakePlayers"]);
        assert_eq!(root("CpvpGensStore", &known), "CpvpGens");
        assert_eq!(root("cpvpgens-font", &known), "CpvpGens");
        assert_eq!(root("CpvpGens-Promo", &known), "CpvpGens");
        assert_eq!(root("cpvpgens", &known), "CpvpGens");
        assert_eq!(root("CpvpGens", &known), "CpvpGens");
        assert_eq!(root("AsyncFakePlayers", &known), "AsyncFakePlayers");
        assert_eq!(root("AsyncShop", &known), "AsyncShop");
        assert_eq!(root("Cpvpgensx", &known), "Cpvpgensx");
        assert_eq!(root("Wisply", &known), "Wisply");
        assert_eq!(root("AsyncShop2", &names(&["AsyncShop", "AsyncShop2"])), "AsyncShop");
        assert_eq!(root("Shopper", &names(&["Shop", "Shopper"])), "Shopper");
    }

    #[test]
    fn folding_renames_rows_once_and_keeps_a_backup() {
        let base = std::env::temp_dir().join(format!("ltm-family-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).unwrap();
        let mut s = Store::open(&base).unwrap();
        for (project, title) in
            [("CpvpGens", "main"), ("CpvpGens", "second"), ("CpvpGensStore", "shop"), ("Tool", "other")]
        {
            let n = NewNote { project, kind: "note", title, body: "text", source: "user", session: "" };
            s.add_note(&n).unwrap();
        }
        let backup = base.join("memory").join(BACKUP);
        assert!(!backup.exists());
        assert_eq!(s.fold_projects(&base).unwrap(), 1);
        assert!(backup.exists());
        assert!(fs::metadata(&backup).unwrap().len() > 0);
        assert_eq!(s.known_projects().unwrap(), vec!["CpvpGens".to_string(), "Tool".to_string()]);
        assert_eq!(s.fold_projects(&base).unwrap(), 0);
        let _ = fs::remove_dir_all(&base);
    }
}
