use std::cell::Cell;

use super::*;
use crate::config::Config;

const NOON: f64 = 1_768_507_200.0;
const NEXT_DAY: f64 = 1_768_550_400.0;
const BEST: &str = "gemini-3.8-flash";
const SECOND: &str = "gemini-3.6-flash";
const NEAR: f64 = 1e-6;

struct Fake {
    now: Cell<f64>,
    slept: Cell<f64>,
}

struct Bench {
    store: Store,
    limits: Limits,
    clock: Fake,
}

impl Clock for Fake {
    fn now(&self) -> f64 {
        self.now.get()
    }

    fn sleep(&self, seconds: f64) {
        self.now.set(self.now.get() + seconds);
        self.slept.set(self.slept.get() + seconds);
    }
}

impl Bench {
    fn new() -> Bench {
        let clock = Fake { now: Cell::new(NOON), slept: Cell::new(0.0) };
        Bench { store: Store::in_memory(), limits: Config::standard().limits, clock }
    }

    fn guard(&self) -> Guard<'_> {
        Guard::new(&self.store, &self.limits, &self.clock)
    }

    fn pass(&self, seconds: f64) {
        self.clock.now.set(self.clock.now.get() + seconds);
    }

    fn slept(&self) -> f64 {
        self.clock.slept.get()
    }

    fn fill_day(&self, scope: &str, count: usize) {
        for _ in 0..count {
            self.guard().reserve(&[scope], &Ask::default()).unwrap();
            self.pass(20.0);
        }
    }

    fn count(&self, table: &str) -> i64 {
        self.store.read().query_row(&format!("SELECT COUNT(*) FROM {table}"), [], |row| row.get(0)).unwrap()
    }
}

fn used_up(what: &str) -> String {
    format!("the free limit is used up ({what}), nothing was sent")
}

fn too_large(what: &str) -> String {
    format!("the request is larger than the limit allows ({what}), nothing was sent")
}

fn video(seconds: f64) -> Ask {
    Ask { youtube_seconds: seconds, ..Ask::default() }
}

fn tokens(tokens: i64) -> Ask {
    Ask { tokens, ..Ask::default() }
}

#[test]
fn the_test_noon_lies_in_the_middle_of_a_pacific_day() {
    assert_eq!(pacific::next_day_start(NOON), NEXT_DAY);
    assert_eq!(pacific::day_start(NOON), NEXT_DAY - DAY);
}

#[test]
fn every_model_shares_the_video_time_rule() {
    let bench = Bench::new();
    let rules = bench.guard().rules_for(BEST).unwrap();
    let names: Vec<&str> = rules.iter().map(|rule| rule.name).collect();
    assert_eq!(names, [REQUESTS_PER_MINUTE, TOKENS_PER_MINUTE, REQUESTS_PER_DAY, YOUTUBE_SECONDS_PER_DAY]);
    assert!(rules[3].all_scopes && rules[3].scope == YOUTUBE && rules[2].window.is_none());
    assert!(bench.guard().rules_for(GROQ).unwrap().iter().all(|rule| rule.window.is_some() && !rule.all_scopes));
}

#[test]
fn the_longest_wait_wins_and_too_large_beats_every_wait() {
    let refusal = |reason: &str, free_at| Refusal { scope: BEST.to_string(), reason: reason.to_string(), free_at };
    let (short, long, never) = (refusal("short", Some(10.0)), refusal("long", Some(20.0)), refusal("never", None));
    assert_eq!(later(None, short.clone()), short);
    assert_eq!(later(Some(short.clone()), long.clone()), long);
    assert_eq!(later(Some(long.clone()), short), long);
    assert_eq!(later(Some(long.clone()), never.clone()), never);
    assert_eq!(later(Some(never.clone()), long), never);
}

#[test]
fn a_request_inside_the_limits_is_reserved_at_once() {
    let bench = Bench::new();
    let reservation = bench.guard().reserve(&[BEST, SECOND], &Ask::default()).unwrap();
    assert_eq!(reservation.scope, BEST);
    assert_eq!(bench.guard().usage_today(BEST).unwrap(), DayUsage { used: 1, allowed: 16 });
    assert_eq!(bench.guard().usage_today(SECOND).unwrap(), DayUsage { used: 0, allowed: 16 });
    assert_eq!(bench.slept(), 0.0);
}

#[test]
fn a_full_minute_is_waited_out() {
    let bench = Bench::new();
    for _ in 0..4 {
        bench.guard().reserve(&[BEST], &Ask::default()).unwrap();
        bench.pass(1.0);
    }
    let reservation = bench.guard().reserve(&[BEST, SECOND], &Ask::default()).unwrap();
    assert_eq!(reservation.scope, BEST);
    assert!((bench.slept() - 56.05).abs() < NEAR, "{}", bench.slept());
    assert_eq!(bench.guard().usage_today(BEST).unwrap().used, 5);
}

#[test]
fn a_full_day_moves_on_to_the_next_model() {
    let bench = Bench::new();
    bench.fill_day(BEST, 16);
    let reservation = bench.guard().reserve(&[BEST, SECOND], &Ask::default()).unwrap();
    assert_eq!(reservation.scope, SECOND);
    assert_eq!(bench.slept(), 0.0);
    assert_eq!(bench.guard().usage_today(BEST).unwrap(), DayUsage { used: 16, allowed: 16 });
}

#[test]
fn a_full_day_on_the_last_model_names_the_limit_and_the_reset() {
    let bench = Bench::new();
    bench.fill_day(BEST, 16);
    let fail = bench.guard().reserve(&[BEST], &Ask::default()).unwrap_err();
    assert_eq!(fail.kind, Kind::Limit);
    assert_eq!(fail.message, used_up("requests per day of gemini-3.8-flash"));
    assert_eq!(fail.hint, format!("wait until {}", local_minute(NEXT_DAY)));
    assert_eq!(bench.slept(), 0.0);
}

#[test]
fn the_day_ends_at_pacific_midnight() {
    let bench = Bench::new();
    bench.fill_day(BEST, 16);
    bench.clock.now.set(NEXT_DAY + 1.0);
    assert_eq!(bench.guard().usage_today(BEST).unwrap().used, 0);
    assert_eq!(bench.guard().reserve(&[BEST], &Ask::default()).unwrap().scope, BEST);
}

#[test]
fn a_request_larger_than_the_limit_is_refused_without_waiting() {
    let bench = Bench::new();
    let fail = bench.guard().reserve(&[BEST, SECOND], &video(30_000.0)).unwrap_err();
    assert_eq!(fail.kind, Kind::Limit);
    assert_eq!(fail.message, too_large("YouTube video time per day"));
    assert_eq!(fail.hint, "use a shorter section or a lower quality");
    assert_eq!((bench.slept(), bench.count("quota_event")), (0.0, 0));
}

#[test]
fn video_time_counts_across_all_models() {
    let bench = Bench::new();
    bench.guard().reserve(&[BEST], &video(20_000.0)).unwrap();
    let fail = bench.guard().reserve(&[SECOND], &video(5_000.0)).unwrap_err();
    assert_eq!(fail.message, used_up("YouTube video time per day"));
    assert_eq!(fail.hint, format!("wait until {}", local_minute(NEXT_DAY)));
    bench.guard().reserve(&[SECOND], &video(3_000.0)).unwrap();
    let (used, allowed) = bench.guard().youtube_today().unwrap();
    assert!((used - 23_000.0).abs() < NEAR && (allowed - 23_040.0).abs() < NEAR, "{used} of {allowed}");
}

#[test]
fn a_rejected_request_blocks_the_model_for_a_while() {
    let bench = Bench::new();
    assert_eq!(bench.guard().note_rejection(BEST, None, false).unwrap(), NOON + 60.0);
    assert_eq!(bench.guard().blocked_until(BEST).unwrap(), Some(NOON + 60.0));
    assert_eq!(bench.guard().blocked_until(SECOND).unwrap(), None);
    bench.guard().reserve(&[BEST], &Ask::default()).unwrap();
    assert!((bench.slept() - 60.05).abs() < NEAR, "{}", bench.slept());
    assert_eq!(bench.guard().blocked_until(BEST).unwrap(), None);
}

#[test]
fn a_used_up_day_blocks_until_the_reset_and_a_block_never_gets_shorter() {
    let bench = Bench::new();
    assert_eq!(bench.guard().note_rejection(BEST, Some(30.0), false).unwrap(), NOON + 30.0);
    assert_eq!(bench.guard().note_rejection(BEST, None, true).unwrap(), NEXT_DAY);
    assert_eq!(bench.guard().note_rejection(BEST, Some(5.0), false).unwrap(), NEXT_DAY);
    let fail = bench.guard().reserve(&[BEST], &Ask::default()).unwrap_err();
    assert_eq!(fail.message, used_up("blocked after a rejected request of gemini-3.8-flash"));
    assert_eq!(fail.hint, format!("wait until {}", local_minute(NEXT_DAY)));
    assert_eq!(bench.guard().reserve(&[BEST, SECOND], &Ask::default()).unwrap().scope, SECOND);
    assert_eq!(bench.slept(), 0.0);
}

#[test]
fn a_learned_limit_only_ever_tightens() {
    let bench = Bench::new();
    let guard = bench.guard();
    guard.learn_limit(BEST, REQUESTS_PER_DAY, 10.0).unwrap();
    assert_eq!(guard.usage_today(BEST).unwrap().allowed, 8);
    guard.learn_limit(BEST, REQUESTS_PER_DAY, 15.0).unwrap();
    guard.learn_limit(BEST, REQUESTS_PER_DAY, 0.0).unwrap();
    assert_eq!(guard.usage_today(BEST).unwrap().allowed, 8);
    guard.learn_limit(SECOND, REQUESTS_PER_DAY, 100.0).unwrap();
    assert_eq!(guard.usage_today(SECOND).unwrap().allowed, 16);
    guard.learn_limit(SECOND, REQUESTS_PER_DAY, 0.5).unwrap();
    assert_eq!(guard.usage_today(SECOND).unwrap().allowed, 1);
}

#[test]
fn settling_writes_the_real_tokens_and_cancelling_gives_the_request_back() {
    let bench = Bench::new();
    let guard = bench.guard();
    let first = guard.reserve(&[BEST], &tokens(1_000)).unwrap();
    guard.settle(&first, 180_000).unwrap();
    assert_eq!(
        guard.reserve(&[BEST], &tokens(250_000)).unwrap_err().message,
        too_large("tokens per minute of gemini-3.8-flash")
    );
    let second = guard.reserve(&[BEST], &tokens(20_000)).unwrap();
    assert_eq!(bench.slept(), 0.0);
    guard.reserve(&[BEST], &tokens(20_000)).unwrap();
    assert!((bench.slept() - 60.05).abs() < NEAR, "{}", bench.slept());
    assert_eq!(guard.usage_today(BEST).unwrap().used, 3);
    guard.cancel(&second).unwrap();
    assert_eq!(guard.usage_today(BEST).unwrap().used, 2);
}

#[test]
fn pruning_drops_old_events_and_past_blocks() {
    let bench = Bench::new();
    let guard = bench.guard();
    guard.reserve(&[BEST], &Ask::default()).unwrap();
    guard.note_rejection(SECOND, None, false).unwrap();
    bench.pass(2.0 * DAY);
    guard.reserve(&[BEST], &Ask::default()).unwrap();
    guard.note_rejection(BEST, None, true).unwrap();
    bench.pass(DAY + 1.0);
    guard.note_rejection(GROQ, Some(600.0), false).unwrap();
    guard.prune().unwrap();
    assert_eq!((bench.count("quota_event"), bench.count("quota_block")), (1, 1));
    assert!(guard.blocked_until(GROQ).unwrap().is_some());
}

#[test]
fn a_model_without_limits_is_refused() {
    let bench = Bench::new();
    let fail = bench.guard().reserve(&["gemini-0"], &Ask::default()).unwrap_err();
    assert_eq!(fail.kind, Kind::Input);
    assert!(fail.message.contains("gemini-0") && fail.hint.contains("[limits.models]"), "{}", fail.text());
    let none = bench.guard().reserve(&[], &Ask::default()).unwrap_err();
    assert_eq!(none.message, "no model was given to send the request to");
}

#[test]
fn the_speech_service_counts_a_rolling_day() {
    let mut bench = Bench::new();
    bench.limits.groq.requests_per_day = 5.0;
    let speech = Ask { audio_seconds: 600.0, ..Ask::default() };
    for _ in 0..4 {
        bench.guard().reserve(&[GROQ], &speech).unwrap();
        bench.pass(HOUR);
    }
    let fail = bench.guard().reserve(&[GROQ], &speech).unwrap_err();
    assert_eq!(fail.message, used_up("requests per day of groq"));
    assert_eq!(fail.hint, format!("wait until {}", local_minute(NOON + DAY)));
    bench.clock.now.set(NOON + DAY + 1.0);
    bench.guard().reserve(&[GROQ], &Ask::default()).unwrap();
    assert_eq!(bench.guard().usage_today(GROQ).unwrap(), DayUsage { used: 4, allowed: 4 });
    let long = Ask { audio_seconds: 6_000.0, ..Ask::default() };
    assert_eq!(bench.guard().reserve(&[GROQ], &long).unwrap_err().message, too_large("audio seconds per hour of groq"));
}
