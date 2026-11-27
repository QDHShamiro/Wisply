use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use rusqlite::{Transaction, params};

use super::project::project_of;
use super::store::Store;
use super::text::{oneline, scrub};
use super::transcript::chunks_of;
use super::vault::{HEAD, front, mtime, posix, walk};

pub const SOURCE: &str = "files";
const KIND: &str = "reference";
const BODY_CHARS: usize = 12_000;
const TITLE_CHARS: usize = 200;
const SECTION: &str = "## ";
const FENCE: &str = "```";
const MIN_CHARS: usize = 40;
const DOCS: &[&str] = &["md", "markdown", "txt"];

struct Part {
    title: String,
    body: String,
}

fn is_doc(p: &Path) -> bool {
    p.extension().is_some_and(|e| DOCS.contains(&e.to_string_lossy().to_lowercase().as_str()))
}

fn parts(text: &str, name: &str) -> Vec<Part> {
    let mut raw: Vec<(String, Vec<&str>)> = vec![(name.to_string(), Vec::new())];
    let mut fenced = false;
    for line in text.lines() {
        if line.trim_start().starts_with(FENCE) {
            fenced = !fenced;
        }
        if !fenced && let Some(head) = line.strip_prefix(SECTION) {
            raw.push((format!("{name}: {}", head.trim()), Vec::new()));
        } else if let Some(last) = raw.last_mut() {
            last.1.push(line);
        }
    }
    let mut out = Vec::new();
    for (title, lines) in raw {
        let body = lines.join("\n");
        if body.trim().chars().count() < MIN_CHARS {
            continue;
        }
        let pieces = chunks_of(body.trim(), BODY_CHARS);
        let total = pieces.len();
        for (i, piece) in pieces.into_iter().enumerate() {
            let shown = if total > 1 { format!("{title} ({}/{total})", i + 1) } else { title.clone() };
            out.push(Part { title: shown, body: piece });
        }
    }
    out
}

fn gather(paths: &[PathBuf]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for p in paths {
        if p.is_dir() {
            let mut found = Vec::new();
            walk(p, "", &mut found);
            out.extend(found.into_iter().map(|(f, _)| f));
        } else if p.is_file() && is_doc(p) {
            out.push(p.clone());
        }
    }
    out
}

fn origin_file(origin: &str) -> &str {
    let path = origin.split_once(':').map_or(origin, |(_, p)| p);
    path.rsplit_once('#').map_or(path, |(file, _)| file)
}

fn origin_of(place: &str, n: usize) -> String {
    format!("{SOURCE}:{place}#{n}")
}

struct Doc {
    project: String,
    parts: Vec<Part>,
}

type Have = HashMap<String, (i64, f64)>;

fn read_doc(file: &Path) -> Option<Doc> {
    let raw = fs::read(file).ok()?;
    let text = String::from_utf8_lossy(&raw).into_owned();
    let (meta, body) = front(&text);
    let stem = file.file_stem().map_or(String::new(), |s| s.to_string_lossy().into_owned());
    let name = meta
        .get("title")
        .filter(|t| !t.is_empty())
        .cloned()
        .or_else(|| HEAD.captures(body).map(|h| h[1].trim().to_string()))
        .unwrap_or(stem);
    let project = match meta.get("project").filter(|p| !p.is_empty()) {
        Some(p) => p.trim_matches(['[', ']']).rsplit('/').next().unwrap_or("").to_string(),
        None => file.parent().map(|d| project_of(&d.to_string_lossy())).unwrap_or_default(),
    };
    Some(Doc { project, parts: parts(body, &name) })
}

fn unchanged(have: &Have, place: &str, t: f64, seen: &mut HashSet<String>) -> bool {
    let mut kept = 0;
    while have.get(&origin_of(place, kept)).is_some_and(|(_, u)| (u - t).abs() < 1.0) {
        seen.insert(origin_of(place, kept));
        kept += 1;
    }
    kept > 0 && !have.contains_key(&origin_of(place, kept))
}

fn save(tx: &Transaction, have: &Have, origin: &str, project: &str, part: &Part, t: f64) -> rusqlite::Result<()> {
    let title = oneline(&scrub(&part.title), TITLE_CHARS);
    let text: String = scrub(part.body.trim()).chars().take(BODY_CHARS).collect();
    match have.get(origin) {
        Some((id, _)) => tx.execute(
            "update notes set project = ?1, kind = ?2, title = ?3, body = ?4, updated = ?5 where id = ?6",
            params![project, KIND, title, text, t, id],
        )?,
        None => tx.execute(
            "insert into notes(project, kind, title, body, files, source, created, updated, origin)              values (?1, ?2, ?3, ?4, '[]', ?5, ?6, ?6, ?7)",
            params![project, KIND, title, text, SOURCE, t, origin],
        )?,
    };
    Ok(())
}

fn prune(tx: &Transaction, have: &Have, seen: &HashSet<String>) -> rusqlite::Result<usize> {
    let mut gone = 0;
    for (origin, (id, _)) in have {
        let file = Path::new(origin_file(origin));
        if !seen.contains(origin) && (file.exists() || file.parent().is_some_and(Path::is_dir)) {
            tx.execute("delete from notes where id = ?1", [id])?;
            gone += 1;
        }
    }
    Ok(gone)
}

impl Store {
    fn extra_have(&self) -> rusqlite::Result<Have> {
        let mut stmt = self.db.prepare("select id, origin, updated from notes where source = ?1")?;
        let rows = stmt.query_map([SOURCE], |r| {
            Ok((
                r.get::<_, Option<String>>(1)?.unwrap_or_default(),
                (r.get(0)?, r.get::<_, Option<f64>>(2)?.unwrap_or(0.0)),
            ))
        })?;
        rows.collect()
    }

    pub fn index_extra(&mut self, paths: &[PathBuf]) -> rusqlite::Result<usize> {
        let have = self.extra_have()?;
        let tx = self.db.transaction()?;
        let mut seen = HashSet::new();
        let mut changed = 0;
        for file in gather(paths) {
            let Some(t) = mtime(&file) else { continue };
            let place = posix(&file);
            if unchanged(&have, &place, t, &mut seen) {
                continue;
            }
            let Some(doc) = read_doc(&file) else { continue };
            for (n, part) in doc.parts.iter().enumerate() {
                let origin = origin_of(&place, n);
                save(&tx, &have, &origin, &doc.project, part, t)?;
                seen.insert(origin);
                changed += 1;
            }
        }
        changed += prune(&tx, &have, &seen)?;
        tx.commit()?;
        if changed > 0 {
            self.vecs = None;
        }
        Ok(changed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_documents_are_split_by_heading_and_found_again() {
        let base = std::env::temp_dir().join(format!("ltm-extra-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let mirror = base.join("sftp").join("server").join("boxsrv");
        fs::create_dir_all(&mirror).unwrap();
        let doc = mirror.join("server-spec.md");
        let long = "line about the nether portal rules\n".repeat(30);
        fs::write(
            &doc,
            format!(
                "# Box server\nIntro text that is long enough to count as a section of its own.\n\n## Anchors\n\
                 Respawn anchors blow up blocks in the pvp box area when the flag allows it.\n\n## Portals\n{long}\n\
                 ```\n## not a heading inside a fence\n```\n"
            ),
        )
        .unwrap();
        let mut s = Store::open(&base).unwrap();
        assert_eq!(s.index_extra(std::slice::from_ref(&mirror)).unwrap(), 3);
        assert_eq!(s.index_extra(std::slice::from_ref(&mirror)).unwrap(), 0);
        let hit = &s.search("respawn anchors pvp box", "", 5, None, 0.0).unwrap()[0].note;
        assert_eq!(
            (hit.source.as_str(), hit.kind.as_str(), hit.title.as_str()),
            ("files", "reference", "Box server: Anchors")
        );
        assert_eq!(hit.project, "boxsrv");
        assert_eq!(hit.path(), Some(posix(&doc)).as_deref());
        let again = s.search("nether portal rules", "", 5, None, 0.0).unwrap();
        assert_eq!(again[0].note.title, "Box server: Portals");
        assert!(again[0].note.body.contains("## not a heading inside a fence"));
        fs::write(&doc, "# Box server\nOnly one section is left and it has enough words to be kept around.\n").unwrap();
        let later = std::time::SystemTime::now() + std::time::Duration::from_secs(5);
        fs::File::options().write(true).open(&doc).unwrap().set_modified(later).unwrap();
        assert_eq!(s.index_extra(std::slice::from_ref(&mirror)).unwrap(), 3);
        assert!(s.search("respawn anchors", "", 5, None, 0.0).unwrap().is_empty());
        fs::remove_dir_all(&mirror).unwrap();
        assert_eq!(s.index_extra(std::slice::from_ref(&mirror)).unwrap(), 0);
        assert!(!s.search("only one section", "", 5, None, 0.0).unwrap().is_empty());
        drop(s);
        let _ = fs::remove_dir_all(&base);
    }
}
