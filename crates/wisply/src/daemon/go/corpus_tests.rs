use std::time::{Duration, Instant};

use super::*;

const NOW_RAW: f64 = 1_795_000_000.0;
const PEOPLE: &[&str] = &["franz", "lukas", "anna", "mama", "papa", "luis", "ben", "oma", "pixelfox", "max mustermann"];
const SITES: &[&str] = &["youtube", "google", "amazon", "github", "reddit", "wikipedia", "twitch"];
const FILES: &[&str] = &[
    include_str!("corpus/msg.txt"),
    include_str!("corpus/remind.txt"),
    include_str!("corpus/media.txt"),
    include_str!("corpus/misc.txt"),
];
const MIN_RATE: f64 = 95.0;
const MAX_MS: f64 = 20.0;
const DAY_SECS: f64 = 86_400.0;

struct Case {
    said: &'static str,
    body: &'static str,
    when: Option<&'static str>,
}

fn cases() -> Vec<Case> {
    FILES
        .iter()
        .flat_map(|f| f.lines())
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let (said, spec) = line.split_once(" => ").unwrap_or_else(|| panic!("bad line: {line}"));
            let (body, when) = match spec.split_once(" @ ") {
                Some((b, w)) => (b, Some(w)),
                None => (spec, None),
            };
            Case { said, body, when }
        })
        .collect()
}

fn noon() -> f64 {
    let [_, _, _, h, mi, s] = local_parts(NOW_RAW);
    NOW_RAW - f64::from(h * 3600 + mi * 60 + s) + 12.0 * 3600.0
}

fn weekday(at: f64) -> u32 {
    const T: [u32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let [y, m, d, ..] = local_parts(at);
    let y = if m < 3 { y - 1 } else { y };
    let v = (y + y / 4 - y / 100 + y / 400 + T[(m - 1) as usize] + d) % 7;
    if v == 0 { 7 } else { v }
}

fn clock_of(text: &str) -> f64 {
    let (h, m) = text.split_once(':').expect("hh:mm");
    h.parse::<f64>().expect("h") * 3600.0 + m.parse::<f64>().expect("m") * 60.0
}

fn expected_when(now: f64, spec: &str) -> f64 {
    let day0 = now - 12.0 * 3600.0;
    if let Some(secs) = spec.strip_prefix('+') {
        return now + secs.parse::<f64>().expect("secs");
    }
    let (head, clock) = spec.split_once(' ').expect("when");
    let days: f64 = match head.split_at(1) {
        ("d", n) => n.parse().expect("days"),
        ("w", n) => {
            let want: u32 = n.parse().expect("weekday");
            f64::from((want + 7 - weekday(now) - 1) % 7 + 1)
        }
        _ => panic!("bad when {spec}"),
    };
    day0 + days * DAY_SECS + clock_of(clock)
}

fn person(w: &str) -> bool {
    PEOPLE.contains(&w)
}

fn site(w: &str) -> bool {
    SITES.contains(&w)
}

fn canon(act: &Act) -> String {
    let dash = |o: &Option<String>| o.clone().unwrap_or_else(|| "-".into());
    match act {
        Act::Msg { who, app, text } => format!("msg {} | {} | {text}", who.join(","), dash(app)),
        Act::Play { app, q } => format!("play {} | {q}", dash(app)),
        Act::Media { key, .. } => format!("media {key}"),
        Act::Open { what } => format!("open {what}"),
        Act::Search { site, q } => format!("search {} | {q}", dash(site)),
        Act::Remind { text } => format!("remind {text}"),
        Act::Shot { of } => format!("shot {}", dash(of)),
        Act::Remember { text } => format!("remember {text}"),
        Act::Recall { q } => format!("recall {q}"),
    }
}

fn run(said: &str, now: f64) -> (String, Option<f64>) {
    match parse(said, now, &Know { person: &person, site: &site }) {
        Ok(Some(p)) => (canon(&p.act), p.when),
        Ok(None) => ("none".into(), None),
        Err(_) => ("err".into(), None),
    }
}

fn judge(c: &Case, now: f64) -> Result<(), String> {
    let (body, when) = run(c.said, now);
    let want = c.when.map(|w| expected_when(now, w));
    let same_when = match (when, want) {
        (None, None) => true,
        (Some(a), Some(b)) => (a - b).abs() < 0.5,
        _ => false,
    };
    if body == c.body && same_when {
        return Ok(());
    }
    let show = |w: Option<f64>| w.map_or("-".to_string(), |w| format!("{:+.0}s", w - now));
    Err(format!("{}\n    want {} @ {}\n    got  {body} @ {}", c.said, c.body, show(want), show(when)))
}

#[test]
fn corpus_hit_rate() {
    let now = noon();
    let all = cases();
    assert!(all.len() >= 300, "corpus has only {} sentences", all.len());
    let fails: Vec<String> = all.iter().filter_map(|c| judge(c, now).err()).collect();
    let rate = 100.0 * (all.len() - fails.len()) as f64 / all.len() as f64;
    println!("corpus: {}/{} ok = {rate:.1} %", all.len() - fails.len(), all.len());
    for f in &fails {
        println!("  FAIL {f}");
    }
    assert!(rate >= MIN_RATE, "corpus rate {rate:.1} % below {MIN_RATE} %");
}

#[test]
fn corpus_parse_time() {
    let now = noon();
    let all = cases();
    let first = Instant::now();
    run("schreib franz hi", now);
    let cold = first.elapsed();
    let (mut total, mut worst, mut slow) = (Duration::ZERO, Duration::ZERO, "");
    for c in &all {
        let t0 = Instant::now();
        run(c.said, now);
        let took = t0.elapsed();
        total += took;
        if took > worst {
            (worst, slow) = (took, c.said);
        }
    }
    let avg_ms = total.as_secs_f64() * 1000.0 / all.len() as f64;
    let max_ms = worst.as_secs_f64() * 1000.0;
    println!(
        "parse time over {} sentences: avg {avg_ms:.4} ms, max {max_ms:.4} ms (\"{slow}\"), first call {:.3} ms",
        all.len(),
        cold.as_secs_f64() * 1000.0
    );
    assert!(max_ms < MAX_MS, "slowest parse {max_ms:.3} ms");
}
