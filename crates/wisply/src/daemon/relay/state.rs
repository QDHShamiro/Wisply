use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

pub const LIMIT_GUESS_S: u64 = 300;
pub const OVERLOAD_S: u64 = 30;

static LIMIT_UNTIL: AtomicU64 = AtomicU64::new(0);
static ENABLED: AtomicBool = AtomicBool::new(false);
static LOCAL_OK_UNTIL: AtomicU64 = AtomicU64::new(0);
static ASKED_AT: AtomicU64 = AtomicU64::new(0);
static COOL: LazyLock<Mutex<HashMap<String, u64>>> = LazyLock::new(Default::default);
static COUNTS: LazyLock<Mutex<HashMap<String, (u64, u64)>>> = LazyLock::new(Default::default);

pub fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}

pub fn set_enabled(on: bool) {
    ENABLED.store(on, Ordering::Relaxed);
}

pub fn limit_until() -> u64 {
    LIMIT_UNTIL.load(Ordering::Relaxed)
}

pub fn limited() -> bool {
    limit_until() > now()
}

pub fn set_limit(wait_s: u64) {
    LIMIT_UNTIL.store(now() + wait_s, Ordering::Relaxed);
}

pub fn clear_limit() {
    LIMIT_UNTIL.store(0, Ordering::Relaxed);
}

pub fn local_ok_until() -> u64 {
    LOCAL_OK_UNTIL.load(Ordering::Relaxed)
}

pub fn local_ok() -> bool {
    local_ok_until() > now()
}

pub fn allow_local(secs: u64) {
    LOCAL_OK_UNTIL.store(now() + secs, Ordering::Relaxed);
}

pub fn revoke_local() {
    LOCAL_OK_UNTIL.store(0, Ordering::Relaxed);
}

pub fn should_ask(gap_s: u64) -> bool {
    let at = now();
    let last = ASKED_AT.load(Ordering::Relaxed);
    if last != 0 && at < last + gap_s {
        return false;
    }
    ASKED_AT.store(at, Ordering::Relaxed);
    true
}

pub fn cool(name: &str, wait_s: u64) {
    if let Ok(mut m) = COOL.lock() {
        m.insert(name.to_string(), now() + wait_s);
    }
}

pub fn cooling(name: &str) -> u64 {
    COOL.lock().ok().and_then(|m| m.get(name).copied()).map_or(0, |t| t.saturating_sub(now()))
}

pub fn count(name: &str, ok: bool) {
    if let Ok(mut m) = COUNTS.lock() {
        let e = m.entry(name.to_string()).or_default();
        if ok {
            e.0 += 1;
        } else {
            e.1 += 1;
        }
    }
}

pub fn counts(name: &str) -> (u64, u64) {
    COUNTS.lock().ok().and_then(|m| m.get(name).copied()).unwrap_or_default()
}

pub fn retry_wait(header: Option<&str>) -> u64 {
    let Some(v) = header.map(str::trim).and_then(|h| h.parse::<f64>().ok()) else { return LIMIT_GUESS_S };
    let secs = if v > 1.0e9 { v - now() as f64 } else { v };
    (secs.ceil().max(1.0) as u64).min(6 * 3600)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_headers_count_seconds_or_epochs_and_default_when_missing() {
        assert_eq!(retry_wait(Some("17")), 17);
        assert_eq!(retry_wait(Some("0")), 1);
        assert_eq!(retry_wait(Some(&(now() + 90).to_string())), 90);
        assert_eq!(retry_wait(Some("soon")), LIMIT_GUESS_S);
        assert_eq!(retry_wait(None), LIMIT_GUESS_S);
        assert_eq!(retry_wait(Some("999999")), 6 * 3600);
    }

    #[test]
    fn the_local_yes_runs_out_and_the_question_is_not_repeated_too_soon() {
        revoke_local();
        assert!(!local_ok());
        allow_local(30);
        assert!(local_ok() && (28..=30).contains(&(local_ok_until() - now())));
        revoke_local();
        assert!(!local_ok());
        ASKED_AT.store(0, Ordering::Relaxed);
        assert!(should_ask(600));
        assert!(!should_ask(600));
    }

    #[test]
    fn a_cooled_provider_comes_back_and_counts_split_by_outcome() {
        cool("t-prov", 40);
        assert!((38..=40).contains(&cooling("t-prov")));
        assert_eq!(cooling("t-none"), 0);
        count("t-count", true);
        count("t-count", false);
        count("t-count", false);
        assert_eq!(counts("t-count"), (1, 2));
    }
}
