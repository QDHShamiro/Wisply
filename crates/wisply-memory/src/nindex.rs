use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, LazyLock, Mutex, PoisonError, RwLock};

use rusqlite::Connection;

use super::expand::{near, stem_word, variants};
use super::text::stop_word;

const BODY_TERMS: usize = 220;
const MAX_WORD: usize = 32;
const MIN_WORD: usize = 2;
const TITLE_W: f32 = 3.0;
const BODY_W: f32 = 0.5;
const BODY_CAP: u8 = 4;
const K1: f32 = 1.2;
const B: f32 = 0.5;
const VARIANT_W: f32 = 0.85;
const PREFIX_W: f32 = 0.6;
const FUZZY_W: f32 = 0.7;
const PREFIX_FROM: usize = 5;
const PREFIX_MAX: usize = 6;
const FUZZY_MAX: usize = 3;
const REBUILD_DEAD: f32 = 0.25;
const BODY_COVER: f32 = 0.6;

#[derive(Clone, Copy)]
struct Post {
    doc: u32,
    title: u8,
    body: u8,
}

struct Meta {
    id: i64,
    project: Box<str>,
    user: bool,
    updated: f64,
    len: f32,
    alive: bool,
}

#[derive(Default)]
pub(crate) struct NoteIndex {
    meta: Vec<Meta>,
    slot: HashMap<i64, u32>,
    post: HashMap<Box<str>, Vec<Post>>,
    sorted: Vec<Box<str>>,
    sorted_for: usize,
    alive: usize,
    total_len: f64,
    synced: f64,
    fuzzy: Mutex<HashMap<String, Vec<String>>>,
}

pub(crate) struct Lexical {
    pub id: i64,
    pub score: f32,
    pub cover: f64,
    pub project: String,
    pub user: bool,
    pub updated: f64,
}

static SHARED: LazyLock<Mutex<HashMap<PathBuf, Arc<RwLock<NoteIndex>>>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

pub(crate) fn shared(key: &Path) -> Arc<RwLock<NoteIndex>> {
    let mut all = SHARED.lock().unwrap_or_else(PoisonError::into_inner);
    all.entry(key.to_path_buf()).or_default().clone()
}

fn words(text: &str, mut each: impl FnMut(String)) {
    let mut word = String::new();
    let flush = |word: &mut String, each: &mut dyn FnMut(String)| {
        if word.chars().count() >= MIN_WORD && word.len() <= MAX_WORD {
            let low = word.to_lowercase();
            if !stop_word(&low) {
                let mut parts = Vec::new();
                let mut start = 0;
                let chars: Vec<(usize, char)> = word.char_indices().collect();
                for w in chars.windows(2) {
                    if w[0].1.is_lowercase() && w[1].1.is_uppercase() {
                        parts.push(&word[start..w[1].0]);
                        start = w[1].0;
                    }
                }
                if !parts.is_empty() {
                    parts.push(&word[start..]);
                    for p in parts.iter().filter(|p| p.chars().count() >= 3) {
                        each(stem_word(&p.to_lowercase()));
                    }
                }
                each(stem_word(&low));
            }
        }
        word.clear();
    };
    for c in text.chars() {
        if c.is_alphanumeric() {
            word.push(c);
        } else {
            flush(&mut word, &mut each);
        }
    }
    flush(&mut word, &mut each);
}

impl NoteIndex {
    pub(crate) fn ready(&self) -> bool {
        self.alive > 0
    }

    fn put(&mut self, id: i64, project: &str, user: bool, updated: f64, title: &str, body: &str) {
        if let Some(old) = self.slot.remove(&id)
            && let Some(m) = self.meta.get_mut(old as usize)
            && m.alive
        {
            m.alive = false;
            self.alive -= 1;
            self.total_len -= f64::from(m.len);
        }
        let mut counts: HashMap<String, (u8, u8)> = HashMap::new();
        words(title, |w| counts.entry(w).or_default().0 = 1);
        words(project, |w| counts.entry(w).or_default().0 = 1);
        let mut seen_body = 0;
        words(body, |w| {
            if let Some(c) = counts.get_mut(&w) {
                c.1 = c.1.saturating_add(1).min(BODY_CAP);
            } else if seen_body < BODY_TERMS {
                seen_body += 1;
                counts.insert(w, (0, 1));
            }
        });
        let doc = self.meta.len() as u32;
        let len = counts.len().max(1) as f32;
        for (w, (title, body)) in counts {
            self.post.entry(w.into_boxed_str()).or_default().push(Post { doc, title, body });
        }
        self.meta.push(Meta { id, project: project.into(), user, updated, len, alive: true });
        self.slot.insert(id, doc);
        self.alive += 1;
        self.total_len += f64::from(len);
        if updated > self.synced {
            self.synced = updated;
        }
    }

    pub(crate) fn sync(&mut self, db: &Connection) -> rusqlite::Result<()> {
        let rebuild = self.meta.len() as f32 > 64.0
            && (self.meta.len() - self.alive) as f32 > REBUILD_DEAD * self.meta.len() as f32;
        if rebuild {
            *self = NoteIndex::default();
        }
        let mut changed = false;
        {
            let mut stmt = db.prepare_cached(
                "select id, project, source, title, body, updated from notes where updated >= ?1 order by updated",
            )?;
            let mut rows = stmt.query([self.synced])?;
            while let Some(r) = rows.next()? {
                let id: i64 = r.get(0)?;
                let updated: f64 = r.get::<_, Option<f64>>(5)?.unwrap_or(0.0);
                if self.slot.get(&id).is_some_and(|s| self.meta[*s as usize].updated == updated) {
                    continue;
                }
                let text = |i: usize| r.get::<_, Option<String>>(i).map(Option::unwrap_or_default);
                let source = text(2)?;
                self.put(id, &text(1)?, source == "user", updated, &text(3)?, &text(4)?);
                changed = true;
            }
        }
        let have: i64 = db.query_row("select count(*) from notes", [], |r| r.get(0))?;
        if have as usize != self.alive {
            let mut stmt = db.prepare_cached("select id from notes")?;
            let live: HashSet<i64> = stmt.query_map([], |r| r.get(0))?.collect::<Result<_, _>>()?;
            let gone: Vec<i64> = self.slot.keys().filter(|id| !live.contains(id)).copied().collect();
            for id in gone {
                if let Some(s) = self.slot.remove(&id)
                    && let Some(m) = self.meta.get_mut(s as usize)
                {
                    m.alive = false;
                    self.alive -= 1;
                    self.total_len -= f64::from(m.len);
                }
            }
            let missing: Vec<i64> = live.into_iter().filter(|id| !self.slot.contains_key(id)).collect();
            for id in missing {
                let row =
                    db.query_row("select project, source, title, body, updated from notes where id = ?1", [id], |r| {
                        Ok((
                            r.get::<_, Option<String>>(0)?.unwrap_or_default(),
                            r.get::<_, Option<String>>(1)?.unwrap_or_default(),
                            r.get::<_, Option<String>>(2)?.unwrap_or_default(),
                            r.get::<_, Option<String>>(3)?.unwrap_or_default(),
                            r.get::<_, Option<f64>>(4)?.unwrap_or(0.0),
                        ))
                    });
                if let Ok((project, source, title, body, updated)) = row {
                    self.put(id, &project, source == "user", updated, &title, &body);
                    changed = true;
                }
            }
        }
        if changed {
            self.sorted_for = 0;
            self.fuzzy = Mutex::default();
        }
        Ok(())
    }

    fn vocab(&mut self) -> &[Box<str>] {
        if self.sorted_for != self.post.len() || self.sorted.is_empty() {
            let mut all: Vec<Box<str>> = self.post.keys().cloned().collect();
            all.sort_unstable();
            self.sorted_for = self.post.len();
            self.sorted = all;
        }
        &self.sorted
    }

    pub(crate) fn prepare(&mut self) {
        self.vocab();
    }

    pub(crate) fn members(&self, terms: &[String]) -> Vec<Vec<(String, f32)>> {
        terms
            .iter()
            .map(|t| {
                let key = stem_word(t);
                let mut group: Vec<(String, f32)> = vec![(key.clone(), 1.0)];
                for v in variants(t) {
                    let s = stem_word(v);
                    if !group.iter().any(|(g, _)| *g == s) {
                        group.push((s, VARIANT_W));
                    }
                }
                if key.chars().count() >= PREFIX_FROM {
                    let at = self.sorted.partition_point(|v| &**v < key.as_str());
                    group.extend(
                        self.sorted[at..]
                            .iter()
                            .take_while(|v| v.starts_with(&key))
                            .filter(|v| v.as_ref() != key.as_str())
                            .take(PREFIX_MAX)
                            .map(|v| (v.to_string(), PREFIX_W)),
                    );
                }
                if !self.post.contains_key(key.as_str()) {
                    group.extend(self.neighbours(&key).into_iter().map(|v| (v, FUZZY_W)));
                }
                group
            })
            .collect()
    }

    fn neighbours(&self, key: &str) -> Vec<String> {
        let mut cache = self.fuzzy.lock().unwrap_or_else(PoisonError::into_inner);
        cache
            .entry(key.to_string())
            .or_insert_with(|| {
                self.sorted.iter().filter(|v| near(key, v)).take(FUZZY_MAX).map(|v| v.to_string()).collect()
            })
            .clone()
    }

    pub(crate) fn search(&self, groups: &[Vec<(String, f32)>], limit: usize) -> Vec<Lexical> {
        let n = self.meta.len();
        if n == 0 || groups.is_empty() {
            return Vec::new();
        }
        let avg = (self.total_len / self.alive.max(1) as f64) as f32;
        let alive = self.alive as f32;
        let (mut total, mut cover) = (vec![0.0f32; n], vec![0.0f32; n]);
        let (mut best, mut cov) = (vec![0.0f32; n], vec![0.0f32; n]);
        let mut touched: Vec<u32> = Vec::new();
        let mut any: Vec<u32> = Vec::new();
        for g in groups {
            for (term, mult) in g {
                let Some(list) = self.post.get(term.as_str()) else { continue };
                let idf = (1.0 + (alive - list.len() as f32 + 0.5) / (list.len() as f32 + 0.5)).ln();
                for p in list {
                    let d = p.doc as usize;
                    let m = &self.meta[d];
                    if !m.alive {
                        continue;
                    }
                    let w = f32::from(p.title.min(1)) * TITLE_W + f32::from(p.body.min(BODY_CAP)) * BODY_W;
                    let tf = w * (K1 + 1.0) / (w + K1 * (1.0 - B + B * m.len / avg));
                    let val = idf * tf * mult;
                    let c = mult * if p.title > 0 { 1.0 } else { BODY_COVER };
                    if best[d] == 0.0 && cov[d] == 0.0 {
                        touched.push(p.doc);
                    }
                    if val > best[d] {
                        best[d] = val;
                    }
                    if c > cov[d] {
                        cov[d] = c;
                    }
                }
            }
            for d in touched.drain(..) {
                let d = d as usize;
                if total[d] == 0.0 && cover[d] == 0.0 {
                    any.push(d as u32);
                }
                total[d] += best[d];
                cover[d] += cov[d];
                best[d] = 0.0;
                cov[d] = 0.0;
            }
        }
        let count = groups.len() as f32;
        let mut ranked: Vec<(f32, u32)> = any.iter().map(|d| (total[*d as usize], *d)).collect();
        ranked.sort_unstable_by(|a, b| b.0.total_cmp(&a.0));
        ranked.truncate(limit);
        let top = ranked.first().map_or(1.0, |r| r.0).max(f32::MIN_POSITIVE);
        ranked
            .into_iter()
            .map(|(s, d)| {
                let m = &self.meta[d as usize];
                Lexical {
                    id: m.id,
                    score: s / top,
                    cover: f64::from((cover[d as usize] / count).min(1.0)),
                    project: m.project.to_string(),
                    user: m.user,
                    updated: m.updated,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::super::store::{NewNote, Store};

    fn put(s: &mut Store, project: &str, title: &str, body: &str) -> i64 {
        let new = NewNote { project, kind: "fact", title, body, source: "user", session: "s" };
        s.add_note(&new).unwrap().unwrap()
    }

    #[test]
    fn words_synonyms_typos_and_changes_are_found_in_memory() {
        let dir = std::env::temp_dir().join(format!("nidx-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let mut s = Store::open(&dir).unwrap();
        let a = put(&mut s, "Echora", "Voice prewarming on startup", "The model warms up before the first request");
        let b = put(&mut s, "Architect", "Slime farm water fix", "");
        put(&mut s, "Other", "Deploy with ship.sh", "unrelated body about hoppers");
        let top = |s: &mut Store, q: &str| s.search(q, "", 3, None, 0.0).unwrap().first().map(|h| h.note.id);
        assert_eq!(top(&mut s, "echora voise prewarm"), Some(a));
        assert_eq!(top(&mut s, "slime farm wasser"), Some(b));
        assert_eq!(top(&mut s, "architect"), Some(b));
        let c = put(&mut s, "Echora", "Fresh note about hoppers", "");
        assert_eq!(top(&mut s, "fresh hoppers"), Some(c));
        assert_eq!(s.forget(c).unwrap(), Ok(1));
        assert_ne!(top(&mut s, "fresh hoppers"), Some(c));
        drop(s);
        let _ = fs::remove_dir_all(&dir);
    }
}
