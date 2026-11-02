use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use super::{Env, Level, Row, read_json, row};
use crate::daemon::tokens::transcript::{Req, parse};

const DAYS: u64 = 14;
const DAY_S: u64 = 86_400;
const FILES: usize = 400;
const HIGH: i64 = 200_000;
const HIGHER: i64 = 400_000;
const TARGET: i64 = 250_000;
const SUMMARY: i64 = 50_000;
const W_READ: f64 = 0.1;
const W_WRITE: f64 = 2.0;
const WARN_SAVING: f64 = 5.0;
const MIN_REQUESTS: usize = 200;
const KEY: &str = "autoCompactWindow";
const ENV_KEY: &str = "CLAUDE_CODE_AUTO_COMPACT_WINDOW";

#[derive(Default, Debug, PartialEq)]
pub struct Shape {
    pub requests: usize,
    pub tokens: i64,
    pub over_high: usize,
    pub over_higher: usize,
    pub tokens_high: i64,
    pub tokens_higher: i64,
}

pub fn shape(sessions: &[Vec<Req>]) -> Shape {
    let mut s = Shape::default();
    for r in sessions.iter().flatten() {
        s.requests += 1;
        s.tokens += r.ctx;
        if r.ctx > HIGH {
            s.over_high += 1;
            s.tokens_high += r.ctx;
        }
        if r.ctx > HIGHER {
            s.over_higher += 1;
            s.tokens_higher += r.ctx;
        }
    }
    s
}

pub fn simulate(sessions: &[Vec<Req>], window: i64) -> (i64, i64) {
    let (mut total, mut compactions) = (0, 0);
    for reqs in sessions {
        let (mut offset, mut prev): (i64, Option<i64>) = (0, None);
        for r in reqs {
            if prev.is_some_and(|p| (r.ctx as f64) < p as f64 * 0.5) {
                offset = 0;
            }
            let mut sim = r.ctx - offset;
            if sim > window {
                total += sim;
                compactions += 1;
                offset += sim - SUMMARY;
                sim = SUMMARY;
            }
            total += sim;
            prev = Some(r.ctx);
        }
    }
    (total, compactions)
}

fn pct(part: f64, whole: f64) -> f64 {
    if whole <= 0.0 { 0.0 } else { 100.0 * part / whole }
}

fn files(env: &Env) -> Vec<PathBuf> {
    let from = SystemTime::now() - Duration::from_secs(DAYS * DAY_S);
    let mut found: Vec<(SystemTime, PathBuf)> = Vec::new();
    for dir in fs::read_dir(env.claude.join("projects")).into_iter().flatten().flatten() {
        for entry in fs::read_dir(dir.path()).into_iter().flatten().flatten() {
            let path = entry.path();
            let when = entry.metadata().and_then(|m| m.modified()).unwrap_or(SystemTime::UNIX_EPOCH);
            if path.extension().is_some_and(|e| e == "jsonl") && when >= from {
                found.push((when, path));
            }
        }
    }
    found.sort_by_key(|f| std::cmp::Reverse(f.0));
    found.truncate(FILES);
    found.into_iter().map(|f| f.1).collect()
}

fn set_window(env: &Env) -> Option<String> {
    let mut seen = Vec::new();
    for file in env.settings() {
        let Some(v) = read_json(&file) else { continue };
        if let Some(w) = v.get(KEY) {
            seen.push(format!("{KEY} {w}"));
        }
        if let Some(w) = v.get("env").and_then(|e| e.get(ENV_KEY)) {
            seen.push(format!("env {ENV_KEY} {w}"));
        }
    }
    (!seen.is_empty()).then(|| seen.join(", "))
}

pub fn text(s: &Shape, sessions: &[Vec<Req>], set: Option<&str>) -> (Level, String, String) {
    let (sim, compactions) = simulate(sessions, TARGET);
    let cut = pct((s.tokens - sim) as f64, s.tokens as f64);
    let cost_now = W_READ * s.tokens as f64;
    let cost_saved = W_READ * (s.tokens - sim) as f64 - W_WRITE * SUMMARY as f64 * compactions as f64;
    let saved = pct(cost_saved, cost_now);
    let line = format!(
        "{} requests in {DAYS} days: {:.0}% ran above 200k context, {:.0}% above 400k ({:.0}% / {:.0}% of the cache-read tokens). At 250k: {:.0}% fewer context tokens, {compactions} compactions, up to {:.0}% less cache-read cost after paying for the summaries (simulation, ignores re-reading after a compact){}",
        s.requests,
        pct(s.over_high as f64, s.requests as f64),
        pct(s.over_higher as f64, s.requests as f64),
        pct(s.tokens_high as f64, s.tokens as f64),
        pct(s.tokens_higher as f64, s.tokens as f64),
        cut,
        saved.max(0.0),
        set.map_or(String::new(), |w| format!(" | set: {w}")),
    );
    if set.is_none() && saved >= WARN_SAVING {
        return (Level::Warn, line, "wisply config compact 250k (wisply config compact off undoes it)".into());
    }
    (Level::Ok, line, String::new())
}

pub fn rows(env: &Env) -> Vec<Row> {
    let sessions: Vec<Vec<Req>> = files(env)
        .iter()
        .filter_map(|p| fs::read(p).ok())
        .map(|raw| parse(&String::from_utf8_lossy(&raw)))
        .filter(|reqs| !reqs.is_empty())
        .collect();
    let s = shape(&sessions);
    if s.requests < MIN_REQUESTS {
        return Vec::new();
    }
    let (level, line, fix) = text(&s, &sessions, set_window(env).as_deref());
    vec![row(level, "context length", line, fix)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req(ctx: i64) -> Req {
        Req { ctx, ..Req::default() }
    }

    #[test]
    fn shares_and_the_250k_simulation_follow_the_context_lengths() {
        let long: Vec<Req> = [100_000, 180_000, 260_000, 300_000, 420_000, 450_000].map(req).to_vec();
        let short: Vec<Req> = [50_000, 60_000].map(req).to_vec();
        let sessions = vec![long, short];
        let s = shape(&sessions);
        assert_eq!((s.requests, s.over_high, s.over_higher), (8, 4, 2));
        assert_eq!(s.tokens, 1_820_000);
        assert_eq!(s.tokens_higher, 870_000);
        let (sim, count) = simulate(&sessions, 250_000);
        assert_eq!(count, 1);
        assert_eq!(sim, 1_240_000);
    }

    #[test]
    fn nothing_over_250k_means_no_advice_and_a_set_window_is_named() {
        let calm: Vec<Vec<Req>> = vec![vec![req(80_000), req(90_000), req(100_000)]];
        let s = shape(&calm);
        let (level, line, fix) = text(&s, &calm, None);
        assert!(level == Level::Ok && fix.is_empty(), "{line}");
        assert!(line.contains("0% ran above 200k") && line.contains("0 compactions"), "{line}");
        let heavy: Vec<Vec<Req>> = vec![(0..40).map(|n| req(150_000 + n * 10_000)).collect()];
        let s = shape(&heavy);
        let (level, _, fix) = text(&s, &heavy, None);
        assert!(level == Level::Warn && fix.starts_with("wisply config compact 250k"));
        let (level, line, _) = text(&s, &heavy, Some("autoCompactWindow 250000"));
        assert!(level == Level::Ok && line.ends_with("| set: autoCompactWindow 250000"), "{line}");
    }
}
