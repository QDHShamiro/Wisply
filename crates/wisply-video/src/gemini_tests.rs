use std::cell::RefCell;
use std::collections::VecDeque;

use rusqlite::{OptionalExtension, params};
use serde_json::json;

use super::*;
use crate::bodies;
use crate::fixture::{Bench, START_TIME};
use crate::guard::REQUESTS_PER_MINUTE;

const BEST: &str = "gemini-3.8-flash";
const SECOND: &str = "gemini-3.6-flash";
const ESTIMATE: i64 = 1000;
const HALF_DAY: f64 = 43_200.0;
const NEAR: f64 = 1e-6;
const MINUTE_QUOTA: &str = "GenerateRequestsPerMinutePerProjectPerModel-FreeTier";
const DAY_QUOTA: &str = "GenerateRequestsPerDayPerProjectPerModel-FreeTier";
const OTHER_QUOTA: &str = "SomethingElse";
const REFUSED_START: &str = "Gemini refused the request (HTTP 400): ";

type Answer = Result<Sent, String>;

#[derive(Default)]
struct Line {
    answers: RefCell<VecDeque<Answer>>,
    bodies: RefCell<Vec<Value>>,
    waits: RefCell<Vec<f64>>,
}

struct Desk {
    bench: Bench,
    line: Line,
}

impl Transport for Line {
    fn post(&self, body: &str, seconds: f64) -> Answer {
        self.bodies.borrow_mut().push(serde_json::from_str(body).expect("body"));
        self.waits.borrow_mut().push(seconds);
        self.answers.borrow_mut().pop_front().expect("an answer")
    }
}

fn answer(status: &str, text: &str, token: Option<&str>) -> Answer {
    let interaction = json!({
        "id": "interaction-1",
        "status": status,
        "continuation_token": token,
        "steps": [{"type": "model_output", "content": [{"type": "text", "text": text}]}],
        "usage": {"total_tokens": 1200, "total_input_tokens": 1000, "total_output_tokens": 200,
            "initial_input_tokens": 900},
    });
    Ok(Sent { status: 200, retry_after: None, text: interaction.to_string() })
}

fn refused(status: u16, reason: &str, message: &str) -> Answer {
    let body =
        json!({"error": {"code": status, "status": reason, "message": message, "details": [{"reason": reason}]}});
    Ok(Sent { status, retry_after: None, text: body.to_string() })
}

fn busy() -> Answer {
    refused(503, "UNAVAILABLE", "The model is overloaded.")
}

fn lost() -> Answer {
    Err("connection  reset\n by peer".to_string())
}

fn limited(quota: &str, wait: &str) -> Answer {
    let details = json!([{"violations": [{"quotaId": quota, "quotaValue": "3"}]}, {"retryDelay": wait}]);
    let error = json!({"code": 429, "status": "RESOURCE_EXHAUSTED", "message": "Quota exceeded.", "details": details});
    Ok(Sent { status: 429, retry_after: None, text: json!({"error": error}).to_string() })
}

impl Desk {
    fn new(answers: Vec<Answer>) -> Desk {
        let line = Line::default();
        line.answers.borrow_mut().extend(answers);
        Desk { bench: Bench::new("gemini"), line }
    }

    fn ask(&self, models: &[&str]) -> Result<Reply, Fail> {
        self.ask_for(models, 0.0)
    }

    fn ask_for(&self, models: &[&str], youtube_seconds: f64) -> Result<Reply, Fail> {
        let config = &self.bench.config;
        let clock = &self.bench.clock;
        let guard = Guard::new(&self.bench.store, &config.limits, clock);
        let gemini = Gemini { settings: &config.gemini, guard: &guard, clock, transport: &self.line, jitter: &|| 1.0 };
        let models: Vec<String> = models.iter().map(|model| model.to_string()).collect();
        let body = bodies::request(None, "What happens?", 100, None);
        let estimate = Ask { tokens: ESTIMATE, youtube_seconds, ..Ask::default() };
        gemini.send(&Request { body: &body, estimate, models: &models })
    }

    fn sent(&self) -> Vec<Value> {
        self.line.bodies.borrow().clone()
    }

    fn asked(&self) -> Vec<String> {
        self.sent().iter().map(|body| reply::word(body, MODEL).to_string()).collect()
    }

    fn passed(&self) -> f64 {
        self.bench.clock.now() - START_TIME
    }

    fn tokens_counted(&self) -> i64 {
        let sql = "SELECT COALESCE(SUM(tokens), 0) FROM quota_event";
        self.bench.store.read().query_row(sql, [], |row| row.get(0)).unwrap()
    }

    fn learned(&self, scope: &str, rule: &str) -> Option<f64> {
        let sql = "SELECT value FROM quota_learned WHERE scope = ?1 AND rule = ?2";
        self.bench.store.read().query_row(sql, params![scope, rule], |row| row.get(0)).optional().unwrap()
    }

    fn blocked_until(&self, scope: &str) -> Option<f64> {
        Guard::new(&self.bench.store, &self.bench.config.limits, &self.bench.clock).blocked_until(scope).unwrap()
    }
}

#[test]
fn an_answer_comes_back_with_its_model_and_tokens() {
    let desk = Desk::new(vec![answer("completed", "A cat jumps.", None)]);
    let reply = desk.ask(&[BEST, SECOND]).unwrap();
    assert_eq!((reply.text.as_str(), reply.model.as_str(), reply.status.as_str()), ("A cat jumps.", BEST, "completed"));
    assert_eq!((reply.requests, reply.hops, reply.complete), (1, 0, true));
    assert_eq!(reply::total_tokens(&reply.usage), 1200);
    let sent = desk.sent();
    assert_eq!(sent.len(), 1);
    assert_eq!(sent[0].as_object().unwrap().keys().next().map(String::as_str), Some(MODEL));
    assert_eq!(sent[0][MODEL], BEST);
    assert_eq!(sent[0]["store"], false);
    assert_eq!(sent[0]["input"][0]["text"], "What happens?");
    assert!(sent[0].get(CONTINUATION).is_none());
    assert_eq!(*desk.line.waits.borrow(), [desk.bench.config.gemini.first_answer_seconds]);
    assert_eq!(desk.tokens_counted(), 1200);
}

#[test]
fn a_short_video_waits_less_for_a_silent_model_than_a_long_one() {
    let short = Desk::new(vec![answer("completed", "A cat jumps.", None)]);
    short.ask_for(&[BEST], 20.0).unwrap();
    let long = Desk::new(vec![answer("completed", "A cat jumps.", None)]);
    long.ask_for(&[BEST], 3600.0).unwrap();
    let gemini = &short.bench.config.gemini;
    let expected = gemini.first_answer_seconds + gemini.answer_seconds_per_media_second * 20.0;
    assert_eq!(*short.line.waits.borrow(), [expected]);
    assert!(expected < gemini.request_timeout_seconds / 2.0, "{expected}");
    assert_eq!(*long.line.waits.borrow(), [gemini.request_timeout_seconds]);
}

#[test]
fn a_long_answer_is_fetched_in_several_hops() {
    let first = answer("requires_continuation", "A cat ", Some("token-1"));
    let desk = Desk::new(vec![first, answer("completed", "jumps.", None)]);
    let reply = desk.ask(&[BEST, SECOND]).unwrap();
    assert_eq!(reply.text, "A cat jumps.");
    assert_eq!((reply.requests, reply.hops, reply.complete), (2, 1, true));
    assert_eq!(reply.usage["total_tokens"], 2400);
    assert_eq!(reply.usage["initial_input_tokens"], 900);
    let sent = desk.sent();
    assert_eq!(sent[1][CONTINUATION], "token-1");
    assert_eq!(sent[1]["input"], sent[0]["input"]);
    assert_eq!(desk.asked(), [BEST, BEST]);
}

#[test]
fn the_hops_end_at_the_allowed_number() {
    let more = || answer("requires_continuation", "more ", Some("token"));
    let desk = Desk::new(vec![more(), more(), more(), more()]);
    let reply = desk.ask(&[BEST]).unwrap();
    assert_eq!(reply.text, "more more more ");
    assert_eq!((reply.requests, reply.hops, reply.complete), (3, 2, false));
    assert_eq!(reply.status, "requires_continuation");
    assert_eq!(desk.line.answers.borrow().len(), 1);
}

#[test]
fn a_failed_hop_keeps_what_was_already_answered() {
    let first = answer("requires_continuation", "A cat ", Some("token-1"));
    let desk = Desk::new(vec![first, refused(400, "INVALID_ARGUMENT", "The token is not valid.")]);
    let reply = desk.ask(&[BEST]).unwrap();
    assert_eq!(reply.text, "A cat ");
    assert_eq!((reply.requests, reply.hops, reply.complete), (1, 0, false));
    assert_eq!(desk.sent().len(), 2);
}

#[test]
fn a_model_that_keeps_failing_is_left_for_the_next_one() {
    let desk = Desk::new(vec![busy(), busy(), answer("completed", "Done.", None)]);
    let reply = desk.ask(&[BEST, SECOND]).unwrap();
    assert_eq!((reply.model.as_str(), reply.requests), (SECOND, 3));
    assert_eq!(desk.asked(), [BEST, BEST, SECOND]);
    let busy = Miss { model: BEST.to_string(), problem: "HTTP 503".to_string() };
    assert_eq!(reply.misses, [busy.clone(), busy]);
    assert!((desk.passed() - 2.0).abs() < NEAR, "{}", desk.passed());
}

#[test]
fn a_server_that_stays_down_ends_with_a_clear_error() {
    let desk = Desk::new(vec![busy(), busy(), busy(), busy(), busy()]);
    let fail = desk.ask(&[BEST]).unwrap_err();
    assert_eq!(fail.kind, Kind::Service);
    let wanted = "Gemini did not answer (HTTP 503: The model is overloaded.) after 4 tries, nothing more was sent";
    assert_eq!((fail.message.as_str(), fail.hint.as_str()), (wanted, "try again in a few minutes"));
    assert_eq!(desk.sent().len(), 4);
    assert!((desk.passed() - 14.0).abs() < NEAR, "{}", desk.passed());
}

#[test]
fn a_network_problem_is_tried_again_on_the_same_model() {
    let desk = Desk::new(vec![lost(), lost(), lost(), answer("completed", "Done.", None)]);
    let reply = desk.ask(&[BEST, SECOND]).unwrap();
    assert_eq!((reply.model.as_str(), reply.requests), (BEST, 4));
    assert_eq!(desk.asked(), [BEST, BEST, BEST, BEST]);
}

#[test]
fn the_whole_wait_never_passes_the_total_time() {
    let mut desk = Desk::new(vec![lost(), lost(), lost()]);
    desk.bench.config.gemini.retry.total_seconds = 5.0;
    let fail = desk.ask(&[BEST]).unwrap_err();
    let wanted =
        "Gemini did not answer (a network problem: connection reset by peer) after 2 tries, nothing more was sent";
    assert_eq!(fail.message, wanted);
    assert_eq!(*desk.line.waits.borrow(), [5.0, 3.0]);
}

#[test]
fn the_wait_a_server_names_is_kept() {
    let later = Sent { status: 503, retry_after: Some("7".to_string()), text: String::new() };
    let desk = Desk::new(vec![Ok(later), answer("completed", "Done.", None)]);
    let reply = desk.ask(&[BEST]).unwrap();
    assert_eq!(reply.requests, 2);
    assert!((desk.passed() - 7.0).abs() < NEAR, "{}", desk.passed());
    assert!((reply.seconds - 7.0).abs() < NEAR, "{}", reply.seconds);
}

#[test]
fn a_short_limit_is_waited_out_and_remembered() {
    let desk = Desk::new(vec![limited(MINUTE_QUOTA, "34s"), answer("completed", "Done.", None)]);
    let reply = desk.ask(&[BEST, SECOND]).unwrap();
    assert_eq!((reply.model.as_str(), reply.requests), (BEST, 2));
    assert!(desk.passed() >= 34.0 && desk.passed() < 35.0, "{}", desk.passed());
    assert_eq!(desk.learned(BEST, REQUESTS_PER_MINUTE), Some(3.0));
}

#[test]
fn a_used_up_day_moves_on_to_the_next_model() {
    let desk = Desk::new(vec![limited(DAY_QUOTA, "34s"), answer("completed", "Done.", None)]);
    let reply = desk.ask(&[BEST, SECOND]).unwrap();
    assert_eq!((reply.model.as_str(), reply.requests), (SECOND, 2));
    assert!(desk.passed() < 1.0, "{}", desk.passed());
    assert_eq!(desk.blocked_until(BEST), Some(START_TIME + HALF_DAY));
    assert_eq!(desk.blocked_until(SECOND), None);
}

#[test]
fn a_limit_on_every_try_ends_without_more_requests() {
    let hit = || limited(OTHER_QUOTA, "1s");
    let desk = Desk::new(vec![hit(), hit(), hit(), hit(), hit()]);
    let fail = desk.ask(&[BEST]).unwrap_err();
    assert_eq!(fail.kind, Kind::Limit);
    assert!(fail.message.contains("blocked after a rejected request of gemini-3.8-flash"), "{}", fail.message);
    assert!(fail.hint.starts_with("wait until "), "{}", fail.hint);
    assert_eq!(desk.sent().len(), 4);
}

#[test]
fn a_rejected_key_names_the_variable_and_is_not_tried_again() {
    let bad = refused(400, "API_KEY_INVALID", "API key not valid. Please pass a valid API key.");
    let desk = Desk::new(vec![bad, answer("completed", "Done.", None)]);
    let fail = desk.ask(&[BEST, SECOND]).unwrap_err();
    assert_eq!(fail.kind, Kind::KeyRejected);
    assert_eq!(fail.message, "Gemini rejected the key in GEMINI_API_KEY");
    assert_eq!(desk.sent().len(), 1);
}

#[test]
fn an_unknown_model_points_to_the_settings() {
    let desk = Desk::new(vec![refused(404, "NOT_FOUND", "models/gemini-3.8-flash is not found")]);
    let fail = desk.ask(&[BEST, SECOND]).unwrap_err();
    assert_eq!(fail.kind, Kind::Service);
    assert_eq!(fail.message, "Gemini answered 404 for gemini-3.8-flash: models/gemini-3.8-flash is not found");
    assert_eq!(fail.hint, "check the model names under [gemini] in video.toml");
}

#[test]
fn a_refused_request_is_told_short_and_without_links() {
    let long = format!("The video at https://www.youtube.com/watch?v=abcdefghijk cannot be used. {}", "x".repeat(2000));
    let desk = Desk::new(vec![refused(400, "INVALID_ARGUMENT", &long), answer("completed", "Done.", None)]);
    let fail = desk.ask(&[BEST, SECOND]).unwrap_err();
    assert_eq!((fail.kind, fail.status, fail.hint.as_str()), (Kind::Rejected, Some(400), ""));
    assert!(
        fail.message.starts_with(&format!("{REFUSED_START}The video at <link> cannot be used. x")),
        "{}",
        fail.message
    );
    assert!(!fail.message.contains("youtube.com"));
    assert_eq!(fail.message.chars().count(), REFUSED_START.len() + desk.bench.config.gemini.error_text_chars);
    assert_eq!(desk.sent().len(), 1);
}

#[test]
fn a_stop_without_an_answer_is_an_error() {
    let fail = Desk::new(vec![answer("failed", "", None)]).ask(&[BEST]).unwrap_err();
    assert_eq!(fail.kind, Kind::Service);
    assert_eq!(fail.message, "Gemini stopped with status failed and gave no answer");
    assert_eq!(fail.hint, "try again later or with a shorter part");
    let bare = Ok(Sent { status: 200, retry_after: None, text: "{}".to_string() });
    let fail = Desk::new(vec![bare]).ask(&[BEST]).unwrap_err();
    assert_eq!(fail.message, "Gemini stopped with status unknown and gave no answer");
    let cut = Desk::new(vec![answer("incomplete", "Half an ans", None)]).ask(&[BEST]).unwrap();
    assert_eq!((cut.text.as_str(), cut.complete), ("Half an ans", false));
}

#[test]
fn an_answer_that_is_not_json_is_an_error_at_once() {
    let page = Ok(Sent { status: 200, retry_after: None, text: "<html>ok</html>".to_string() });
    let desk = Desk::new(vec![page, answer("completed", "Done.", None)]);
    let fail = desk.ask(&[BEST, SECOND]).unwrap_err();
    assert_eq!(fail.kind, Kind::Service);
    assert_eq!(fail.message, "Gemini sent an answer that cannot be read");
    assert_eq!(desk.sent().len(), 1);
}

#[test]
fn the_pause_between_tries_grows_up_to_its_top() {
    let desk = Desk::new(Vec::new());
    let config = &desk.bench.config;
    let guard = Guard::new(&desk.bench.store, &config.limits, &desk.bench.clock);
    let half = Gemini {
        settings: &config.gemini,
        guard: &guard,
        clock: &desk.bench.clock,
        transport: &desk.line,
        jitter: &|| 0.0,
    };
    let retry = &config.gemini.retry;
    assert_eq!(half.backoff(1), retry.first_delay_seconds / 2.0);
    assert_eq!(half.backoff(2), retry.first_delay_seconds);
    assert_eq!(half.backoff(30), retry.max_delay_seconds / 2.0);
    assert_eq!(times(1), "1 try");
    let now = jitter();
    assert!((0.0..1.0).contains(&now), "{now}");
}
