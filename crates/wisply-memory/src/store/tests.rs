use super::*;

fn home(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("ltm-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

fn unit(seed: usize) -> Vec<f32> {
    let mut v = vec![0.0; VEC_DIM];
    v[seed] = 1.0;
    v
}

#[test]
fn notes_are_found_by_words_and_vectors() {
    let dir = home("notes");
    let mut s = Store::open(&dir).unwrap();
    let new = |title: &'static str, project: &'static str| NewNote {
        project,
        kind: "fact",
        title,
        body: "",
        source: "user",
        session: "s1",
    };
    let a = s.add_note(&new("Brave is the only browser", "wisply")).unwrap().unwrap();
    let b = s.add_note(&new("Deploy with ship.sh and token: abcdefgh1", "other")).unwrap().unwrap();
    assert_eq!(s.add_note(&new("   ", "x")).unwrap(), None);
    assert_eq!(s.note(b).unwrap().unwrap().title, "Deploy with ship.sh and token: [secret]");
    let hits = s.search("which browser", "wisply", 8, None, 0.0).unwrap();
    assert_eq!(hits.iter().map(|h| h.note.id).collect::<Vec<_>>(), [a]);
    s.save_vecs(&[(a, unit(1)), (b, unit(2))]).unwrap();
    let hits = s.search("zzz", "", 8, Some(&unit(2)), 0.0).unwrap();
    assert_eq!(hits[0].note.id, b);
    assert!(hits[0].sim.is_some_and(|v| (v - 1.0).abs() < 1e-3));
    s.touch(&[a]).unwrap();
    assert_eq!(s.note(a).unwrap().unwrap().hits, 1);
    assert_eq!(s.forget(a).unwrap(), Ok(1));
    assert!(s.note(a).unwrap().is_none() && s.search("browser", "", 8, None, 0.0).unwrap().is_empty());
    drop(s);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn todos_keep_status_and_notes() {
    let dir = home("todos");
    let s = Store::open(&dir).unwrap();
    let one = s.todo_add("p", "write the spec", "", "todo").unwrap();
    let two = s.todo_add("p", "ship it", "release", "doing").unwrap();
    s.todo_add("q", "elsewhere", "", "todo").unwrap();
    let open: Vec<i64> = s.todos(Some("p"), true).unwrap().iter().map(|t| t.id).collect();
    assert_eq!(open, [one, two]);
    let done = s.todo_set(one, Some("done"), Some("  all   good ")).unwrap().unwrap();
    assert!(done.status == "done" && done.note.ends_with(" all good"), "{done:?}");
    assert_eq!(s.todos(Some("p"), true).unwrap().len(), 1);
    assert_eq!(s.todos(None, false).unwrap().len(), 3);
    assert!(s.todo_set(99, Some("done"), None).unwrap().is_none());
    drop(s);
    let _ = fs::remove_dir_all(&dir);
}
fn mix(parts: &[(usize, f32)]) -> Vec<f32> {
    let mut v = vec![0.0f32; VEC_DIM];
    for (i, w) in parts {
        v[*i] = *w;
    }
    let norm = v.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 0.0 {
        for x in &mut v {
            *x /= norm;
        }
    }
    v
}

#[test]
fn semantic_recall_ranks_by_vector_closeness() {
    let dir = home("semantic");
    let mut s = Store::open(&dir).unwrap();
    let new =
        |title: &'static str| NewNote { project: "", kind: "fact", title, body: "", source: "user", session: "s1" };
    let a = s.add_note(&new("alpha nimbus")).unwrap().unwrap();
    let b = s.add_note(&new("beta quartz")).unwrap().unwrap();
    let c = s.add_note(&new("gamma cobalt")).unwrap().unwrap();
    let e = s.add_note(&new("delta ember")).unwrap().unwrap();
    s.save_vecs(&[(a, mix(&[(10, 1.0)])), (b, mix(&[(11, 1.0)])), (c, mix(&[(12, 1.0)])), (e, mix(&[(13, 1.0)]))])
        .unwrap();
    let cases: &[(Vec<f32>, i64)] = &[
        (mix(&[(10, 0.95), (11, 0.31)]), a),
        (mix(&[(11, 1.0)]), b),
        (mix(&[(12, 0.9), (13, 0.44)]), c),
        (mix(&[(13, 0.97), (12, 0.24)]), e),
    ];
    for (q, want) in cases {
        let hits = s.search("qqqq zzzz", "", 8, Some(q), 0.0).unwrap();
        assert_eq!(hits[0].note.id, *want, "query toward {want} ranked {:?} first", hits[0].note.id);
        let top = hits[0].sim.expect("top carries a similarity");
        assert!(top >= SEMANTIC_SURE, "top sim {top} below the sure threshold");
        for w in hits.windows(2) {
            let (x, y) = (w[0].sim.unwrap_or(0.0), w[1].sim.unwrap_or(0.0));
            assert!(x >= y - 1e-6, "similarity not sorted: {x} then {y}");
        }
    }
    drop(s);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn many_first_opens_at_once_all_get_the_store() {
    for round in 0..20 {
        let dir = home(&format!("race{round}"));
        let gate = std::sync::Arc::new(std::sync::Barrier::new(8));
        let opens: Vec<_> = (0..8)
            .map(|_| {
                let (dir, gate) = (dir.clone(), gate.clone());
                std::thread::spawn(move || {
                    gate.wait();
                    Store::open(&dir).map(|_| ()).map_err(|e| e.to_string())
                })
            })
            .collect();
        for open in opens {
            assert_eq!(open.join().unwrap(), Ok(()), "round {round}");
        }
        let _ = fs::remove_dir_all(&dir);
    }
}
