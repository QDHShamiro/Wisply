use std::collections::HashMap;

use rusqlite::Connection;

use super::ask::{MEANT, Sym};
use super::words::{near, query_terms, split_ident, stem, synonyms, words_of};

const W_NAME: f32 = 3.0;
const W_PARENT: f32 = 1.5;
const W_SIG: f32 = 1.2;
const W_PATH: f32 = 0.8;
const W_BAG: f32 = 0.6;
const W_CAP: f32 = 4.0;
const K1: f32 = 1.2;
const B: f32 = 0.4;
const SYN_WEIGHT: f32 = 0.7;
const FUZZY_WEIGHT: f32 = 0.7;
const COVER_FLOOR: f32 = 0.1;
const NAME_FLOOR: f32 = 0.4;
const KIND_OTHER: f32 = 0.7;
const TEST_PENALTY: f32 = 0.08;
const BUILT_PENALTY: f32 = 0.2;
const SIG_WORDS: usize = 16;

type Postings = HashMap<String, Vec<(u32, f32, bool)>>;

struct Doc {
    sym: Sym,
    len: f32,
    scale: f32,
}

pub struct Lex {
    docs: Vec<Doc>,
    post: Postings,
    vocab: Vec<String>,
    avg: f32,
}

fn is_test(sym: &Sym) -> bool {
    let low = sym.path.to_lowercase();
    low.contains("/test")
        || low.starts_with("test")
        || low.contains("_test.")
        || low.contains(".test.")
        || sym.parent.to_lowercase().contains("test")
        || sym.name.starts_with("test_")
}

fn is_built(path: &str) -> bool {
    let low = path.to_lowercase();
    low.contains("/dist/") || low.contains(".min.") || low.contains("/vendor/") || low.contains("/generated/")
}

fn add(terms: &mut HashMap<String, (f32, bool)>, words: impl IntoIterator<Item = String>, weight: f32, named: bool) {
    for w in words {
        let slot = terms.entry(stem(&w)).or_insert((0.0, false));
        slot.0 = (slot.0.max(weight) + if slot.0 > 0.0 { weight * 0.25 } else { 0.0 }).min(W_CAP);
        slot.1 |= named;
    }
}

fn split(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for run in text.split(|c: char| !(c.is_alphanumeric() || c == '_')).filter(|r| !r.is_empty()) {
        split_ident(run, &mut out);
    }
    out.retain(|w| w.chars().count() >= 2);
    out
}

impl Lex {
    pub fn build(db: &Connection) -> rusqlite::Result<Lex> {
        let mut q =
            db.prepare("select id, path, name, low, kind, line, end_line, parent, sig, bag from symbols order by id")?;
        let mut rows = q.query([])?;
        let (mut docs, mut post): (Vec<Doc>, Postings) = (Vec::new(), HashMap::new());
        let mut total = 0.0;
        while let Some(r) = rows.next()? {
            let opt = |i: usize| r.get::<_, Option<String>>(i).map(Option::unwrap_or_default);
            let sym = Sym {
                id: r.get(0)?,
                path: r.get(1)?,
                name: r.get(2)?,
                low: r.get(3)?,
                kind: r.get(4)?,
                line: r.get(5)?,
                end: r.get(6)?,
                parent: opt(7)?,
                sig: opt(8)?,
            };
            let bag = opt(9)?;
            let mut terms: HashMap<String, (f32, bool)> = HashMap::new();
            add(&mut terms, split(&sym.name), W_NAME, true);
            add(&mut terms, split(&sym.parent), W_PARENT, false);
            add(&mut terms, words_of(&sym.sig, SIG_WORDS), W_SIG, false);
            add(&mut terms, split(&sym.path), W_PATH, false);
            add(&mut terms, bag.split_whitespace().map(String::from), W_BAG, false);
            let scale = if MEANT.contains(&sym.kind.as_str()) { 1.0 } else { KIND_OTHER }
                * if is_test(&sym) { TEST_PENALTY } else { 1.0 }
                * if is_built(&sym.path) { BUILT_PENALTY } else { 1.0 };
            let n = docs.len() as u32;
            let len = terms.len() as f32;
            total += len;
            for (t, (w, named)) in terms {
                post.entry(t).or_default().push((n, w, named));
            }
            docs.push(Doc { sym, len, scale });
        }
        let avg = (total / docs.len().max(1) as f32).max(1.0);
        let vocab = post.keys().cloned().collect();
        Ok(Lex { docs, post, vocab, avg })
    }

    fn idf(&self, df: usize) -> f32 {
        let n = self.docs.len() as f32;
        (1.0 + (n - df as f32 + 0.5) / (df as f32 + 0.5)).ln()
    }

    pub fn search(&self, text: &str, limit: usize) -> Vec<&Sym> {
        let terms = query_terms(text);
        let n = self.docs.len();
        if terms.is_empty() || n == 0 {
            return Vec::new();
        }
        let mut score = vec![0.0f32; n];
        let (mut cover, mut named) = (vec![0.0f32; n], vec![0.0f32; n]);
        let (mut loose, mut loose_named) = (vec![0.0f32; n], vec![0.0f32; n]);
        let (mut best, mut mass, mut tag) = (vec![0.0f32; n], vec![0.0f32; n], vec![0.0f32; n]);
        let (mut touched, mut seen): (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
        let mut weight_sum = 0.0f32;
        for (t, tw) in &terms {
            let mut group_idf = 0.0f32;
            let mut cands: Vec<(&str, f32)> = vec![(t.as_str(), 1.0)];
            cands.extend(synonyms(t).into_iter().map(|s| (s, SYN_WEIGHT)));
            if !self.post.contains_key(t.as_str()) {
                cands.extend(self.vocab.iter().filter(|v| near(t, v)).map(|v| (v.as_str(), FUZZY_WEIGHT)));
            }
            for (cand, mult) in cands {
                let Some(list) = self.post.get(cand) else { continue };
                let idf = self.idf(list.len());
                group_idf = group_idf.max(idf * tw);
                for (d, w, name) in list {
                    let d = *d as usize;
                    let tf = w * (K1 + 1.0) / (w + K1 * (1.0 - B + B * self.docs[d].len / self.avg));
                    if best[d] == 0.0 {
                        touched.push(d);
                    }
                    best[d] = best[d].max(idf * tf * mult * tw);
                    mass[d] = mass[d].max(mult);
                    if *name {
                        tag[d] = tag[d].max(mult);
                    }
                }
            }
            weight_sum += group_idf;
            for d in touched.drain(..) {
                if score[d] == 0.0 {
                    seen.push(d);
                }
                score[d] += best[d];
                let (own, own_named) = (mass[d] * group_idf, tag[d] * group_idf);
                if mass[d] >= 1.0 {
                    cover[d] += own;
                } else {
                    loose[d] += own;
                }
                if tag[d] >= 1.0 {
                    named[d] += own_named;
                } else {
                    loose_named[d] += own_named;
                }
                (best[d], mass[d], tag[d]) = (0.0, 0.0, 0.0);
            }
        }
        let count = weight_sum.max(f32::MIN_POSITIVE);
        let cap = count / terms.len() as f32;
        let mut ranked: Vec<(f32, &Doc)> = seen
            .into_iter()
            .map(|d| {
                let (c, nm) = ((cover[d] + loose[d].min(cap)) / count, (named[d] + loose_named[d].min(cap)) / count);
                let fit = (COVER_FLOOR + (1.0 - COVER_FLOOR) * c * c) * (NAME_FLOOR + (1.0 - NAME_FLOOR) * nm);
                (score[d] * fit * self.docs[d].scale, &self.docs[d])
            })
            .collect();
        ranked.sort_by(|a, b| {
            b.0.total_cmp(&a.0)
                .then_with(|| a.1.sym.name.len().cmp(&b.1.sym.name.len()))
                .then_with(|| a.1.sym.path.cmp(&b.1.sym.path))
                .then_with(|| a.1.sym.line.cmp(&b.1.sym.line))
        });
        ranked.into_iter().take(limit).map(|(_, d)| &d.sym).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::params;

    fn lex_of(defs: &[(&str, &str, &str, &str)]) -> Lex {
        let db = Connection::open_in_memory().unwrap();
        db.execute_batch(
            "create table symbols(id integer primary key, path text, name text, low text, kind text, line integer,              end_line integer, parent text, sig text, bag text);",
        )
        .unwrap();
        for (path, name, kind, bag) in defs {
            db.execute(
                "insert into symbols(path, name, low, kind, line, end_line, parent, sig, bag) values (?1, ?2, ?3, ?4, 1, 2, '', '', ?5)",
                params![path, name, name.to_lowercase(), kind, bag],
            )
            .unwrap();
        }
        Lex::build(&db).unwrap()
    }

    #[test]
    fn meaning_questions_reach_names_bodies_and_skip_tests() {
        let lex = lex_of(&[
            ("src/text.rs", "scrub", "function", "secrets replace format"),
            ("src/args.rs", "parse_args", "function", "flags values"),
            ("src/store.rs", "add_note", "function", "insert notes title"),
            ("src/store.rs", "note_count", "function", "count notes"),
            ("tests/a.rs", "scrub_secrets_before_saving_notes", "function", "secrets"),
        ]);
        let first = |q: &str| lex.search(q, 3).first().map(|s| s.name.clone());
        assert_eq!(first("strip secrets before saving").as_deref(), Some("scrub"));
        assert_eq!(first("where do we save notes").as_deref(), Some("add_note"));
        assert_eq!(first("scrubb secrets").as_deref(), Some("scrub"));
        assert!(lex.search("zzzz", 3).is_empty());
    }

    #[test]
    #[ignore = "debug helper: WISPLY_BENCH_HOME and WISPLY_PEEK_CODE"]
    fn explain() {
        let (Some(home), Ok(query)) = (std::env::var_os("WISPLY_BENCH_HOME"), std::env::var("WISPLY_PEEK_CODE")) else {
            return;
        };
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..").canonicalize().unwrap();
        let ix = crate::index::Index::open(std::path::Path::new(&home), &root).unwrap();
        let lex = Lex::build(&ix.db).unwrap();
        println!("terms {:?}", query_terms(&query));
        for (t, _) in query_terms(&query) {
            println!("  {t}: df={} syn={:?}", lex.post.get(&t).map_or(0, Vec::len), synonyms(&t));
        }
        for s in lex.search(&query, 12) {
            println!("{}:{} {} {}", s.path, s.line, s.kind, s.name);
        }
    }
}
