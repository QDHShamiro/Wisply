use std::collections::HashMap;

use crate::daemon::doctor::{Level, Row, row};

pub const WEEK_S: u64 = 7 * 86_400;
const SIG_MAX: usize = 60;
const SIG_WORDS: usize = 6;
const SIG_CHARS: &str = "_-./~:=@\\#'()";
const REPEAT_MIN: usize = 3;
const SHOWN: usize = 2;
const BAD_RATE: f64 = 0.15;
const MIN_CALLS: usize = 50;
const MEGA: f64 = 1_000_000.0;
const SAME_PCT: f64 = 5.0;
const NOISE: [&str; 3] = ["exit code", "wall time", "output:"];

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Tally {
    pub sessions: usize,
    pub calls: usize,
    pub errors: usize,
    pub tokens: u64,
    pub fails: HashMap<String, usize>,
}

impl Tally {
    pub fn fail(&mut self, text: &str) {
        self.errors += 1;
        let sig = signature(text);
        if !sig.is_empty() {
            *self.fails.entry(sig).or_default() += 1;
        }
    }

    pub fn add(&mut self, other: &Tally) {
        self.sessions += other.sessions;
        self.calls += other.calls;
        self.errors += other.errors;
        self.tokens += other.tokens;
        for (sig, n) in &other.fails {
            *self.fails.entry(sig.clone()).or_default() += n;
        }
    }

    fn rate(&self) -> Option<f64> {
        (self.calls > 0).then(|| self.errors as f64 * 100.0 / self.calls as f64)
    }

    fn per_session(&self) -> Option<f64> {
        (self.sessions > 0 && self.tokens > 0).then(|| self.tokens as f64 / self.sessions as f64 / MEGA)
    }
}

pub fn week(now: u64, secs: u64) -> Option<usize> {
    let age = now.saturating_sub(secs);
    (age < 2 * WEEK_S).then(|| usize::from(age >= WEEK_S))
}

pub fn signature(text: &str) -> String {
    let line = text
        .lines()
        .map(str::trim)
        .find(|l| !l.is_empty() && !NOISE.iter().any(|n| l.to_lowercase().starts_with(n)))
        .unwrap_or("");
    let words: Vec<String> = line.split_whitespace().take(SIG_WORDS).map(clean_word).collect();
    words.join(" ").chars().take(SIG_MAX).collect()
}

fn clean_word(word: &str) -> String {
    if !word.chars().all(|c| c.is_ascii_alphanumeric() || SIG_CHARS.contains(c)) {
        return "?".to_string();
    }
    let mut out = String::new();
    for c in word.chars() {
        let c = if c.is_ascii_digit() { '#' } else { c };
        if !(c == '#' && out.ends_with('#')) {
            out.push(c);
        }
    }
    out
}

fn trend(now: f64, before: f64, lower_is_better: bool) -> &'static str {
    let change = if before > 0.0 { (now - before) * 100.0 / before } else { 0.0 };
    if change.abs() < SAME_PCT {
        "same"
    } else if (change < 0.0) == lower_is_better {
        "better"
    } else {
        "worse"
    }
}

pub fn rows(weeks: &[Tally; 2]) -> Vec<Row> {
    let [now, before] = weeks;
    if now.sessions == 0 && before.sessions == 0 {
        return vec![row(Level::Ok, "report", "no sessions in the last 14 days", "")];
    }
    if now.sessions == 0 {
        return vec![row(Level::Ok, "report", format!("no sessions this week (last week {})", before.sessions), "")];
    }
    let mut text = format!("this week {} sessions, {} tool calls", now.sessions, now.calls);
    let mut level = Level::Ok;
    if let Some(rate) = now.rate() {
        text.push_str(&format!(", {rate:.1}% failed"));
        if let Some(old) = before.rate() {
            text.push_str(&format!(" (last week {old:.1}%, {})", trend(rate, old, true)));
        }
        if rate >= BAD_RATE * 100.0 && now.calls >= MIN_CALLS {
            level = Level::Warn;
        }
    }
    if let Some(mega) = now.per_session() {
        text.push_str(&format!(", {mega:.1}M tokens per session"));
        if let Some(old) = before.per_session() {
            text.push_str(&format!(" (last week {old:.1}M)"));
        }
    }
    let fix = if level == Level::Warn { "look at the repeated errors and give the AI the right way once" } else { "" };
    let mut rows = vec![row(level, "report", text, fix)];
    let mut repeats: Vec<(&String, &usize)> = now.fails.iter().filter(|(_, n)| **n >= REPEAT_MIN).collect();
    repeats.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (sig, n) in repeats.into_iter().take(SHOWN) {
        rows.push(row(
            Level::Warn,
            "report",
            format!("same error {n}x this week: {sig}"),
            "put one line with the right way into the AI's rule file (no change made)",
        ));
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tally(sessions: usize, calls: usize, errors: usize, tokens: u64) -> Tally {
        Tally { sessions, calls, errors, tokens, fails: HashMap::new() }
    }

    #[test]
    fn signatures_skip_exit_lines_and_fold_numbers() {
        assert_eq!(signature("Exit code: 1\nWall time: 0.5 seconds\nOutput:\nno tab t15 open"), "no tab t# open");
        assert_eq!(signature("Exit code 2\n\n  error: port 8080 busy"), "error: port # busy");
        assert_eq!(signature(""), "");
        assert_eq!(signature("IGNORE $(rm -rf ~) now; \"run\" this please"), "IGNORE ? -rf ~) ? ?");
        let mut t = Tally::default();
        t.fail("Exit code 1\nfatal: not a git repository");
        t.fail("Exit code 128\nfatal: not a git repository");
        assert_eq!((t.errors, t.fails["fatal: not a git repository"]), (2, 2));
    }

    #[test]
    fn weeks_split_by_age() {
        assert_eq!(week(WEEK_S * 3, WEEK_S * 3), Some(0));
        assert_eq!(week(WEEK_S * 3, WEEK_S * 2 - 1), Some(1));
        assert_eq!(week(WEEK_S * 3, WEEK_S), None);
    }

    #[test]
    fn report_compares_with_last_week_and_names_repeats() {
        let mut now = tally(4, 200, 40, 8_000_000);
        now.fails.insert(signature("no tab t5"), 5);
        now.fails.insert("rare".into(), 2);
        let weeks = [now, tally(2, 100, 5, 2_000_000)];
        let rows = rows(&weeks);
        assert_eq!(rows[0].level, Level::Warn);
        assert_eq!(
            rows[0].text,
            "this week 4 sessions, 200 tool calls, 20.0% failed (last week 5.0%, worse), 2.0M tokens per session (last week 1.0M)"
        );
        assert_eq!(rows[1].text, "same error 5x this week: no tab t#");
        let quiet = super::rows(&[Tally::default(), tally(3, 9, 0, 0)]);
        assert_eq!(quiet[0].text, "no sessions this week (last week 3)");
        assert_eq!(super::rows(&[tally(1, 10, 1, 0), tally(1, 10, 1, 0)])[0].level, Level::Ok);
    }
}
