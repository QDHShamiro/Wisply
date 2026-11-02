mod corpus;

use corpus::{Case, Kind, all};
use std::time::Instant;
use wisply_squeeze::{Squeezed, squeeze, squeeze_tool};

fn run(case: &Case) -> Option<Squeezed> {
    match case.kind {
        Kind::Web => squeeze_tool("WebFetch", &case.text),
        Kind::Grep => squeeze_tool("Grep", &case.text),
        _ => squeeze(&case.text),
    }
}

#[test]
fn the_corpus_is_big_enough_and_short() {
    let cases = all();
    assert!(cases.iter().filter(|c| matches!(c.kind, Kind::Dense | Kind::Bash)).count() >= 40);
    assert!(cases.iter().filter(|c| c.name.starts_with("grep-")).count() >= 10);
    assert!(cases.iter().filter(|c| c.kind == Kind::Web).count() >= 5);
    for c in cases.iter().filter(|c| matches!(c.kind, Kind::Dense | Kind::Bash)) {
        assert!(c.text.lines().count() <= 60, "{}", c.name);
    }
}

#[test]
fn dense_outputs_stay_untouched() {
    for c in all().iter().filter(|c| c.kind == Kind::Dense) {
        assert!(run(c).is_none(), "{} was touched", c.name);
    }
}

#[test]
fn nothing_important_is_lost() {
    let mut cut = 0;
    for c in all() {
        let Some(s) = run(&c) else { continue };
        cut += 1;
        assert!(s.chars_out < s.chars_in, "{} grew", c.name);
        for m in &c.must {
            assert!(s.body.lines().any(|l| l.contains(m.as_str())), "{} lost {m:?}\n{}", c.name, s.body);
        }
    }
    assert!(cut >= 30, "only {cut} cases were cut");
}

#[test]
fn measure_old_against_new() {
    let (mut before, mut after, mut times) = (0usize, 0usize, Vec::new());
    for c in all() {
        let chars = c.text.chars().count();
        let t = Instant::now();
        let s = run(&c);
        times.push(t.elapsed().as_secs_f64() * 1000.0);
        let out = s.map_or(chars, |s| s.chars_out);
        before += chars;
        after += out;
        println!("{:<16} {:>6} -> {:>6}", c.name, chars, out);
    }
    times.sort_by(|a, b| a.total_cmp(b));
    println!(
        "total {before} -> {after}, median {:.3} ms, max {:.3} ms",
        times[times.len() / 2],
        times[times.len() - 1]
    );
    if !cfg!(debug_assertions) {
        assert!(times[times.len() / 2] < 1.0);
    }
}
