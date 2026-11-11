use wisply_memory::{NewNote, VEC_DIM};

use super::*;

const NOTES: [(&str, &str, &str); 4] = [
    ("garden", "fact", "The greenhouse door code is 4812"),
    ("garden", "fact", "The beehive stands behind the old shed"),
    ("garden", "rule", "Never water the cactus shelf on weekdays"),
    ("kitchen", "fact", "The oven needs ten minutes to preheat"),
];
const ASKED: &str = r#"{"id": "door", "class": "fact", "project": "garden", "query": "greenhouse door code", "good": [1]}
{"id": "bees", "class": "fact", "cross": true, "project": "garden", "query": "Wo steht der Bienenstock genau", "good": [2]}
{"id": "cactus", "class": "rule", "project": "garden", "query": "water the cactus shelf"}
{"id": "tower", "class": "none", "project": "garden", "query": "how tall is the lighthouse"}
{"id": "oven", "class": "fact", "split": "hold", "project": "kitchen", "query": "oven preheat minutes", "good": [4]}
"#;

fn home(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("wisply-mbench-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn unit(seed: usize) -> Vec<f32> {
    let mut v = vec![0.0; VEC_DIM];
    v[seed] = 1.0;
    v
}

fn say(home: &Path, args: &[&str]) -> Result<String, String> {
    command(home, &args.iter().map(|a| (*a).to_string()).collect::<Vec<_>>()).map_err(|f| f.text())
}

fn names(dir: &Path, part: &str) -> Vec<String> {
    results(dir)
        .iter()
        .filter_map(|p| p.file_name().map(|n| n.to_string_lossy().into_owned()))
        .filter(|n| n.contains(part))
        .collect()
}

fn seeded(home: &Path, dir: &Path) {
    let mut s = Store::open(home).expect("live store");
    for (project, kind, title) in NOTES {
        s.add_note(&NewNote { project, kind, title, body: "", source: "user", session: "s1" }).expect("note");
    }
    s.save_vecs(&[(2, unit(3))]).expect("vectors");
    drop(s);
    assert!(say(home, &["freeze"]).is_ok_and(|t| t.starts_with("froze 4 notes (1 with vectors)")));
    assert!(say(home, &["freeze"]).is_err_and(|e| e.contains("--again")));
    assert!(say(home, &["run"]).is_err_and(|e| e.contains("no questions")));
    fs::write(dir.join(QUESTIONS), ASKED).expect("questions");
}

fn compares(home: &Path, dir: &Path) {
    let (base, vec, held) = (names(dir, "-baseone-"), names(dir, "-vec-"), names(dir, "-hold"));
    assert_eq!((base.len(), vec.len(), held.len()), (1, 1, 1));
    let diff = say(home, &["compare", &base[0], vec[0].trim_end_matches(".json")]).expect("compare");
    assert!(diff.contains("baseone words (") && diff.contains(" -> vec vectors ("), "{diff}");
    assert!(diff.contains("tune set, 2 pairs") && diff.contains("won 1, lost 0 (top 3): no proven gain"), "{diff}");
    assert!(diff.contains("\nleft out: 1 asked in only one of the two runs, 0 without a label today"), "{diff}");
    assert!(diff.contains("hit@1 50.0 % -> 100.0 %") && diff.contains("  cross     won 1, lost 0 of 1"), "{diff}");
    assert!(say(home, &["compare", &base[0], &held[0]]).is_err_and(|e| e.contains("different question sets")));
    assert!(say(home, &["compare", &base[0]]).is_err());
    assert_eq!(say(home, &["compare", "baseone", "vec"]).as_deref(), Ok(diff.as_str()));
    let printed = (format!("results/{}", base[0]), format!("results\\{}", vec[0]));
    assert_eq!(say(home, &["compare", &printed.0, &printed.1]).as_deref(), Ok(diff.as_str()));
    assert!(say(home, &["compare", "base", "vec"]).is_err_and(|e| e.contains("no result 'base'")));
    let again = say(home, &["run", "--tag", "vec"]).expect("second run with the same tag");
    assert!(again.contains("-vec-words-tune.json"), "{again}");
    let newest = say(home, &["compare", "baseone", "vec"]).expect("compare by tag");
    assert!(newest.contains(" -> vec words (") && newest.contains("won 0, lost 0"), "{newest}");
}

#[test]
fn an_invented_memory_is_frozen_measured_judged_and_compared() {
    let home = home("flow");
    let dir = home.join("bench").join("memory");
    assert!(say(&home, &[]).is_err_and(|e| e.contains("bench freeze")));
    seeded(&home, &dir);
    let state = say(&home, &[]).expect("status");
    assert!(state.contains("questions: 5 (4 tuning, 1 hold-out), 1 without a label, 1 asked in German"), "{state}");
    assert!(state.contains("classes: fact 3, rule 1, none 1") && state.contains("vectors: 0 of 5"), "{state}");

    let words = say(&home, &["run", "--tag", "base one", "--misses"]).expect("words run");
    assert!(words.contains("words only, tune set: 3 questions (1 without a label skipped), copy of"), "{words}");
    assert!(words.contains("hit@1 50.0 % (1/2)") && words.contains("hit@3 50.0 % (1/2)"), "{words}");
    assert!(words.contains("  bees fact rank - \"Wo steht der Bienenstock genau\""), "{words}");
    assert!(words.contains("-baseone-words-tune.json") && !words.contains("door fact rank"), "{words}");
    assert!(words.contains("missed, good note not in the top 3: 1 (question"), "{words}");

    let bare = say(&home, &["open"]).expect("open with an unlabelled question");
    assert!(bare.contains("without a good note yet: 1 (question"), "{bare}");
    assert!(bare.contains("  cactus \"water the cactus shelf\" got #3"), "{bare}");
    let listed = fs::read_to_string(dir.join(OPEN)).expect("list of open hits");
    assert!(listed.contains("cactus\twater the cactus shelf\t3\trule\tgarden\tNever water"), "{listed}");

    assert!(say(&home, &["judge", "cactus", "maybe", "3"]).is_err());
    assert!(say(&home, &["judge", "nobody", "good", "3"]).is_err_and(|e| e.contains("no question 'nobody'")));
    assert_eq!(say(&home, &["judge", "cactus", "bad", "#3", "4"]).as_deref(), Ok("cactus: good [], bad [3,4], old []"));
    assert_eq!(say(&home, &["judge", "cactus", "good", "3"]).as_deref(), Ok("cactus: good [3], bad [4], old []"));

    assert!(say(&home, &["run", "--vectors"]).is_err_and(|e| e.contains("4 questions have no cached vector")));
    let cached: Vec<String> = load(&dir)
        .expect("questions")
        .iter()
        .map(|q| json!({"query": q.query, "vec": unit(if q.id == "bees" { 3 } else { 9 })}).to_string())
        .collect();
    fs::write(dir.join(VECTORS), cached.join("\n")).expect("vector cache");
    let vecs = say(&home, &["run", "--vectors", "--tag", "vec"]).expect("vector run");
    assert!(vecs.contains("words and cached vectors, tune set: 4 questions, copy of"), "{vecs}");
    assert!(vecs.contains("hit@1 100.0 % (3/3)") && vecs.contains("  cross     n 1 "), "{vecs}");

    let hold = say(&home, &["run", "--hold"]).expect("hold run");
    assert!(hold.contains("hold set: 1 questions") && hold.contains("hit@1 100.0 % (1/1)"), "{hold}");
    assert!(say(&home, &["run", "--hold", "--misses"]).is_err_and(|e| e.contains("sums only")));

    compares(&home, &dir);

    let open = say(&home, &["open"]).expect("open");
    assert!(open.contains("hits wait for a verdict") && dir.join(OPEN).is_file(), "{open}");
    assert!(!open.contains("without a good note yet"), "{open}");
    assert!(say(&home, &["nonsense"]).is_err_and(|e| e.contains("unknown bench command")));
    let _ = fs::remove_dir_all(&home);
}
