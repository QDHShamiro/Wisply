use std::fs;

use super::super::store::{NewNote, VEC_DIM};
use super::*;

const NOTES: &str = include_str!("notes.tsv");
const QUESTIONS: &str = include_str!("questions.jsonl");
const BEES: i64 = 7;

fn home(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("ltm-bench-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn unit(seed: usize) -> Vec<f32> {
    let mut v = vec![0.0; VEC_DIM];
    v[seed] = 1.0;
    v
}

fn row(id: &str, class: &str, rank: Option<usize>) -> Row {
    Row { id: id.into(), class: class.into(), rank, ..Row::default() }
}

#[test]
fn questions_parse_and_bad_lines_are_refused() {
    let all = questions(QUESTIONS).unwrap();
    assert_eq!(all.len(), 7);
    assert!(all[3].class == "stale" && all[3].good == [5] && all[3].old == [4] && !all[3].hold);
    assert!(all[6].cross && all[6].hold && all[6].project == "garden");
    assert!(all[5].labelled() && all[5].good.is_empty());
    assert!(!Question { class: "fact".into(), ..Question::default() }.labelled());
    assert!(questions("{\"id\": \"a\", \"class\": \"fact\"}").unwrap_err().starts_with("line 1: needs"));
    assert!(questions("{\"id\": \"a\", \"class\": \"other\", \"query\": \"x\"}").is_err());
    let twice = "{\"id\": \"a\", \"class\": \"fact\", \"query\": \"x\"}\n\n{\"id\": \"a\", \"class\": \"fact\", \"query\": \"y\"}";
    assert_eq!(questions(twice).unwrap_err(), "line 3: id a is used twice");
    assert!(questions("not json").unwrap_err().starts_with("line 1:"));
}

#[test]
fn a_row_knows_rank_old_notes_and_what_nobody_judged() {
    let q = Question {
        id: "q".into(),
        class: "stale".into(),
        good: vec![5],
        bad: vec![9],
        old: vec![4],
        ..Question::default()
    };
    let r = judge(&q, &[4, 9, 7, 5, 8]);
    assert_eq!((r.rank, r.stale, r.open.as_slice()), (Some(4), true, [7].as_slice()));
    let mut r = judge(&q, &[5, 9]);
    assert_eq!((r.rank, r.stale, r.open.is_empty()), (Some(1), false, true));
    r.hint(&q, &[5, 4, 11]);
    assert_eq!((r.hints, r.wrong_hints, r.open.as_slice()), (3, 2, [11].as_slice()));
    assert_eq!(judge(&q, &[]).rank, None);
}

#[test]
fn sums_leave_questions_without_an_answer_out_of_the_hit_rate() {
    let mut rows =
        vec![row("a", "fact", Some(1)), row("b", "fact", Some(3)), row("c", "rule", None), row("d", "fact", Some(4))];
    let mut none = row("e", NO_ANSWER, None);
    none.hints = 1;
    none.wrong_hints = 1;
    none.chars = 50;
    rows.push(none);
    rows[2].cross = true;
    for (i, r) in rows.iter_mut().enumerate() {
        r.ms = (i + 1) as f64;
    }
    let s = sums(&rows);
    assert_eq!((s.asked, s.n, s.hit1, s.hit3, s.without_answer, s.hinted_without_answer), (5, 4, 1, 2, 1, 1));
    assert!((s.mrr - (1.0 + 1.0 / 3.0 + 0.25) / 4.0).abs() < 1e-9, "{}", s.mrr);
    assert_eq!((s.hints, s.wrong_hints, s.chars, s.p50, s.p95), (1, 1, 10, 3.0, 5.0));
    let parts = by_class(&rows);
    assert_eq!(parts.keys().map(String::as_str).collect::<Vec<_>>(), [CROSS, "fact", "rule"]);
    assert_eq!((parts["fact"].n, parts["fact"].hit3, parts[CROSS].n, parts[CROSS].hit3), (3, 2, 1, 0));
    assert_eq!(sums(&[]), Sums::default());
    assert_eq!(s.json()["hit3"], 2);
}

#[test]
fn a_gain_needs_ten_wins_and_twice_as_many_as_losses() {
    let set = |hits: usize, n: usize| -> Vec<Row> {
        (0..n).map(|i| row(&format!("q{i}"), "fact", if i < hits { Some(1) } else { None })).collect()
    };
    let old = set(10, 40);
    let g = gain(&old, &set(20, 40));
    assert_eq!((g.pairs, g.wins, g.losses, g.counts), (40, 10, 0, true));
    assert!(!gain(&old, &set(19, 40)).counts);
    let mut mixed = set(22, 40);
    for r in mixed.iter_mut().take(6) {
        r.rank = Some(5);
    }
    let g = gain(&old, &mixed);
    assert_eq!((g.wins, g.losses, g.counts), (12, 6, true));
    mixed[6].rank = None;
    assert!(!gain(&old, &mixed).counts);
    let mut other = set(40, 41);
    other.push(row("none", NO_ANSWER, None));
    assert_eq!(gain(&old, &other).pairs, 40);
}

#[test]
fn two_results_are_compared_on_shared_questions_with_the_labels_of_today() {
    let ask = |id: &str, good: &[i64]| Question {
        id: id.into(),
        class: "fact".into(),
        good: good.to_vec(),
        ..Question::default()
    };
    let asked = [ask("a", &[1, 2]), ask("b", &[3]), ask("c", &[4]), ask("d", &[])];
    let seen = |id: &str, top: &[i64]| Row { top: top.to_vec(), chars: 7, ms: 2.0, ..row(id, "fact", None) };
    let old = [seen("a", &[2, 9]), seen("b", &[8, 3]), seen("d", &[5]), seen("gone", &[1])];
    let new = [seen("a", &[1]), seen("b", &[3]), seen("c", &[4]), seen("d", &[5])];
    let (before, after, left) = paired(&old, &new, &asked);
    assert_eq!(before.iter().map(|r| (r.id.as_str(), r.rank)).collect::<Vec<_>>(), [("a", Some(1)), ("b", Some(2))]);
    assert_eq!(after.iter().map(|r| (r.id.as_str(), r.rank)).collect::<Vec<_>>(), [("a", Some(1)), ("b", Some(1))]);
    assert_eq!((left, before[0].chars, before[0].ms), ([2, 1], 7, 2.0));
    let g = gain(&before, &after);
    assert_eq!((g.pairs, g.wins, g.losses), (2, 0, 0));
}

#[test]
fn rows_survive_the_result_file() {
    let q = Question { id: "q".into(), class: "time".into(), cross: true, good: vec![2], ..Question::default() };
    let mut r = judge(&q, &[3, 2]);
    r.hint(&q, &[3]);
    r.chars = 120;
    r.ms = 1.5;
    let none = row("n", NO_ANSWER, None);
    let text = rows_json(&[r.clone(), none.clone()]).to_string();
    assert_eq!(rows_of(&serde_json::from_str(&text).unwrap()), [r, none]);
    assert!(rows_of(&Value::Null).is_empty());
}

#[test]
fn the_invented_set_runs_on_a_frozen_copy() {
    let live = home("live");
    let frozen = home("frozen");
    let mut s = Store::open(&live).unwrap();
    for (n, line) in NOTES.lines().enumerate() {
        let cell: Vec<&str> = line.split('\t').collect();
        let new = NewNote {
            project: cell[0],
            kind: cell[1],
            title: cell[2],
            body: cell.get(3).copied().unwrap_or(""),
            source: "user",
            session: "s1",
        };
        assert_eq!(s.add_note(&new).unwrap(), Some(n as i64 + 1));
    }
    s.save_vecs(&[(BEES, unit(3)), (1, unit(4))]).unwrap();
    let to = frozen.join("memory").join("memory.db");
    fs::create_dir_all(to.parent().unwrap()).unwrap();
    s.copy_to(&to).unwrap();
    assert!(s.copy_to(&to).is_err());
    assert_eq!(s.forget(BEES).unwrap(), Ok(1));
    drop(s);
    let mut s = Store::open(&frozen).unwrap();
    s.set_clock(1_800_000_000.0);
    let all = questions(QUESTIONS).unwrap();
    let ask = |s: &mut Store, cached: bool| -> Vec<Row> {
        let vec = unit(3);
        all.iter()
            .map(|q| {
                let (hits, ms) = s.ask(q, (cached && q.cross).then_some(vec.as_slice())).unwrap();
                let mut r = judge(q, &hits.iter().map(|h| h.note.id).collect::<Vec<_>>());
                r.ms = ms;
                r
            })
            .collect()
    };
    let words = ask(&mut s, false);
    let by_id = |rows: &[Row], id: &str| rows.iter().find(|r| r.id == id).cloned().unwrap();
    for id in ["door", "tomato", "cactus", "seeds"] {
        assert_eq!(by_id(&words, id).rank, Some(1), "{id}: {:?}", by_id(&words, id));
    }
    assert!(by_id(&words, "pump").stale, "{:?}", by_id(&words, "pump"));
    assert_eq!(by_id(&words, "bees").rank, None);
    let tops = |rows: &[Row]| rows.iter().map(|r| r.top.clone()).collect::<Vec<_>>();
    assert_eq!(tops(&ask(&mut s, false)), tops(&words));
    let cached = ask(&mut s, true);
    assert_eq!(by_id(&cached, "bees").rank, Some(1));
    let g = gain(&words, &cached);
    assert_eq!((g.pairs, g.wins, g.losses, g.counts), (6, 1, 0, false));
    let total = sums(&words);
    assert_eq!((total.asked, total.n, total.stale, total.without_answer), (7, 6, 1, 1));
    assert!((4..=5).contains(&total.hit3), "{total:?}");
    drop(s);
    let _ = fs::remove_dir_all(&live);
    let _ = fs::remove_dir_all(&frozen);
}
