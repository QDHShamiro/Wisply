use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use wisply_core::Fail;

const LIMIT: u32 = 3;
const WINDOW: Duration = Duration::from_secs(600);
const KEEP: usize = 512;
const MISS: [&str; 3] = ["nothing matches", "nothing called", "no field called"];
pub const AGAIN: &str = "again";

struct Seen {
    misses: u32,
    at: Instant,
    text: String,
}

static SEEN: LazyLock<Mutex<HashMap<String, Seen>>> = LazyLock::new(Default::default);

pub fn key(sid: &str, argv: &[String]) -> String {
    let own: Vec<&str> = argv.iter().map(String::as_str).filter(|a| *a != "--again").collect();
    format!("{sid}\u{1}{}", own.join("\u{1}"))
}

pub fn stuck(key: &str, name: &str, target: Option<&str>) -> Option<Fail> {
    let seen = SEEN.lock().ok()?;
    let last = seen.get(key).filter(|s| s.misses >= LIMIT && s.at.elapsed() < WINDOW)?;
    let what = last.text.split(" -> try:").next().unwrap_or(&last.text).trim_start_matches("err: ");
    let word = target.and_then(|t| t.split_whitespace().next()).unwrap_or("word");
    Some(Fail::hint(
        format!("{what} (this {name} missed {} times in a row in this session, not run again)", last.misses),
        format!(
            "wisply snap -f \"{word}\" (what the page has), or stop the loop and ask the user; --again runs it anyway"
        ),
    ))
}

pub fn ran(key: String, fail: Option<&Fail>) {
    let Ok(mut seen) = SEEN.lock() else { return };
    let Some(fail) = fail else {
        seen.remove(&key);
        return;
    };
    let text = fail.text();
    if !MISS.iter().any(|m| text.contains(m)) {
        return;
    }
    if seen.len() >= KEEP {
        seen.retain(|_, s| s.at.elapsed() < WINDOW);
    }
    let last = seen.entry(key).or_insert(Seen { misses: 0, at: Instant::now(), text: String::new() });
    if last.at.elapsed() >= WINDOW {
        last.misses = 0;
    }
    last.misses += 1;
    last.at = Instant::now();
    last.text = text;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn argv(line: &str) -> Vec<String> {
        line.split(' ').map(String::from).collect()
    }

    #[test]
    fn a_click_that_keeps_missing_stops_waiting_until_it_works() {
        let k = key("loop-a", &argv("click Mum"));
        let miss = Fail::hint("nothing matches \"Mum\"", "wisply snap -f \"Mum\"");
        for _ in 0..LIMIT {
            assert!(stuck(&k, "click", Some("Mum")).is_none());
            ran(k.clone(), Some(&miss));
        }
        let fail = stuck(&k, "click", Some("Mum")).expect("stuck").text();
        assert!(fail.starts_with("err: nothing matches \"Mum\" (this click missed 3 times"), "{fail}");
        assert!(fail.contains("--again runs it anyway"), "{fail}");
        assert_eq!(key("loop-a", &argv("click Mum --again")), k);
        assert!(stuck(&key("loop-b", &argv("click Mum")), "click", Some("Mum")).is_none());
        ran(k.clone(), None);
        assert!(stuck(&k, "click", Some("Mum")).is_none());
    }

    #[test]
    fn other_errors_never_count_as_a_miss() {
        let k = key("loop-c", &argv("click Save"));
        for _ in 0..LIMIT + 2 {
            ran(k.clone(), Some(&Fail::hint("no tab yet", "wisply open <url>")));
        }
        assert!(stuck(&k, "click", Some("Save")).is_none());
    }
}
