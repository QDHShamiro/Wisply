use std::path::{Path, PathBuf};
use std::time::Instant;

use wisply_code::Index;
use wisply_memory::{Store, mentions, words};

const CODE: &str = include_str!("fixtures/find_code.tsv");
const RUNS: usize = 9;
const TOP: usize = 3;
const LIMIT: usize = 6;
const MAX_MS: f64 = 1.0;

fn bench_home() -> Option<PathBuf> {
    std::env::var_os("WISPLY_BENCH_HOME").map(PathBuf::from).filter(|p| Path::new(p).is_dir())
}

fn median(mut ms: Vec<f64>) -> f64 {
    ms.sort_by(f64::total_cmp);
    ms[ms.len() / 2]
}

fn timed<T>(mut run: impl FnMut() -> T) -> (T, f64) {
    let mut last = run();
    let mut all = Vec::new();
    for _ in 0..RUNS {
        let t = Instant::now();
        last = run();
        all.push(t.elapsed().as_secs_f64() * 1000.0);
    }
    (last, median(all))
}

fn place(rank: Option<usize>) -> String {
    rank.map_or("-".into(), |r| r.to_string())
}

#[test]
#[ignore = "needs WISPLY_BENCH_HOME with a copy of memory/memory.db and your own find_recall.tsv"]
fn recall_questions_land_in_the_top_three() {
    let Some(home) = bench_home() else { return };
    let Ok(recall) = std::fs::read_to_string(home.join("find_recall.tsv")) else { return };
    let mut store = Store::open(&home).unwrap();
    let cold = Instant::now();
    store.search("warm up", "", LIMIT, None, 0.0).unwrap();
    println!("recall cold ms={:.0}", cold.elapsed().as_secs_f64() * 1000.0);
    let todos = store.todos(None, true).unwrap();
    let mut slow = Vec::new();
    let mut misses = Vec::new();
    let mut all = Vec::new();
    for row in recall.lines().filter(|l| !l.trim().is_empty()) {
        let [set, query, want] = row.split('\t').collect::<Vec<_>>()[..] else { panic!("{row}") };
        let want: Vec<&str> = want.split(',').collect();
        let (hits, ms) = timed(|| store.search(query, "", LIMIT, None, 0.0).unwrap());
        let ids: Vec<String> = hits.iter().map(|h| h.note.id.to_string()).collect();
        let raw = words(query);
        let mut order: Vec<String> = todos
            .iter()
            .filter(|t| mentions(&format!("{} {}", t.task, t.note), &raw))
            .take(2)
            .map(|t| format!("todo:{}", t.id))
            .collect();
        order.extend(ids);
        let rank = order.iter().position(|id| want.contains(&id.as_str())).map(|p| p + 1);
        println!("recall {set} rank={} ms={ms:.2} {query}", place(rank));
        if rank.is_none_or(|r| r > TOP) {
            misses.push(query.to_string());
        }
        all.push(ms);
        slow.push(ms);
    }
    let mid = median(slow);
    println!("recall median ms={mid:.2}");
    assert!(misses.is_empty(), "outside the top {TOP}: {misses:?}");
    assert!(mid <= MAX_MS, "median {mid:.2} ms");
}

fn names_of(text: &str) -> Vec<(String, String)> {
    text.lines()
        .skip(1)
        .filter_map(|l| {
            let mut parts = l.trim().splitn(3, ' ');
            let at = parts.next()?;
            let _kind = parts.next()?;
            let name = parts.next()?.split(' ').next()?;
            Some((at.split(':').next()?.to_string(), name.to_string()))
        })
        .collect()
}

#[test]
#[ignore = "needs WISPLY_BENCH_HOME (parsers cached under code/parsers)"]
fn code_questions_land_in_the_top_three() {
    let Some(home) = bench_home() else { return };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..").canonicalize().unwrap();
    let mut index = Index::open(&home, &root).unwrap();
    index.refresh(None).unwrap();
    let mut misses = Vec::new();
    let mut times = Vec::new();
    for row in CODE.lines().filter(|l| !l.trim().is_empty()) {
        let [query, want] = row.split('\t').collect::<Vec<_>>()[..] else { panic!("{row}") };
        let (out, ms) = timed(|| index.by_name(query, "closest").unwrap());
        let found = names_of(&out);
        let rank = found.iter().take(TOP).position(|(path, name)| {
            want.split('|').any(|w| match w.split_once('@') {
                Some((n, p)) => name == n && path.contains(p),
                None => name == w,
            })
        });
        let chars = out.chars().count();
        println!("code rank={} chars={chars} ms={ms:.3} {query}", place(rank.map(|r| r + 1)));
        if rank.is_none() {
            misses.push(query.to_string());
        }
        times.push(ms);
    }
    let mid = median(times);
    println!("code median ms={mid:.3}");
    assert!(misses.is_empty(), "outside the top {TOP}: {misses:?}");
    assert!(mid <= MAX_MS, "median {mid:.3} ms");
}

#[test]
#[ignore = "prints the hits of WISPLY_PEEK"]
fn peek() {
    let (Some(home), Ok(query)) = (bench_home(), std::env::var("WISPLY_PEEK")) else { return };
    let mut store = Store::open(&home).unwrap();
    for h in store.search(&query, "", 8, None, 0.0).unwrap() {
        println!(
            "#{} {:.5} [{}] {}",
            h.note.id,
            h.score,
            h.note.project,
            h.note.title.chars().take(90).collect::<String>()
        );
    }
}

#[test]
#[ignore = "prints the code hits of WISPLY_PEEK_CODE"]
fn peek_code() {
    let (Some(home), Ok(query)) = (bench_home(), std::env::var("WISPLY_PEEK_CODE")) else { return };
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..").canonicalize().unwrap();
    let mut index = Index::open(&home, &root).unwrap();
    index.refresh(None).unwrap();
    println!("{}", index.by_name(&query, "closest").unwrap());
}
