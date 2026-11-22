use serde_json::json;

use super::*;

const NOW: f64 = 1_795_000_000.0;

fn quota(violations: Value, wait: &str) -> String {
    json!({"error": {"code": 429, "status": "RESOURCE_EXHAUSTED", "message": "You exceeded your current quota.",
    "details": [
        {"@type": "type.googleapis.com/google.rpc.QuotaFailure", "violations": violations},
        {"@type": "type.googleapis.com/google.rpc.RetryInfo", "retryDelay": wait}
    ]}})
    .to_string()
}

fn refused(status: u16, reason: &str, message: &str) -> Trouble {
    let body = json!({"error": {"code": status, "status": "INVALID_ARGUMENT", "message": message,
        "details": [{"reason": reason}]}});
    answered(status, None, &body.to_string(), NOW)
}

#[test]
fn a_minute_limit_names_the_wait_and_what_was_learned() {
    let violations = json!([
        {"quotaId": "GenerateContentInputTokensPerModelPerMinute-FreeTier", "quotaValue": "250000"},
        {"quotaId": "GenerateRequestsPerMinutePerProjectPerModel-FreeTier", "quotaValue": 10},
        {"quotaId": "SomethingElse", "quotaValue": "5"},
        "odd"
    ]);
    let trouble = answered(429, None, &quota(violations, "34s"), NOW);
    assert_eq!(trouble.class, Class::RateLimited);
    assert_eq!(trouble.status, Some(429));
    assert_eq!(trouble.message, "You exceeded your current quota.");
    assert_eq!(trouble.retry_seconds, Some(34.0));
    assert!(!trouble.daily);
    assert_eq!(trouble.learned, [(TOKENS_PER_MINUTE, 250_000.0), (REQUESTS_PER_MINUTE, 10.0)]);
}

#[test]
fn a_day_limit_is_marked_as_used_up_until_the_reset() {
    let violations = json!([
        {"quotaId": "GenerateRequestsPerDayPerProjectPerModel-FreeTier", "quotaValue": "20"},
        {"quotaId": "GenerateContentInputTokensPerModelPerDay-FreeTier", "quotaValue": "9000"},
        {"quotaId": "GenerateRequestsPerDayPerProject", "quotaValue": "0"},
        {"quotaId": "GenerateRequestsPerDayPerUser", "quotaValue": "many"}
    ]);
    let trouble = answered(429, Some("7"), &quota(violations, "34s"), NOW);
    assert!(trouble.daily);
    assert_eq!(trouble.learned, [(REQUESTS_PER_DAY, 20.0)]);
    assert_eq!(trouble.retry_seconds, Some(7.0));
}

#[test]
fn the_wait_falls_back_from_the_header_to_the_answer() {
    let body = quota(json!([]), "0s");
    assert_eq!(answered(429, None, &body, NOW).retry_seconds, None);
    assert_eq!(answered(429, Some("soon"), &quota(json!([]), "12.5s"), NOW).retry_seconds, Some(12.5));
    assert_eq!(answered(503, Some("3"), "", NOW).retry_seconds, Some(3.0));
}

#[test]
fn the_status_decides_what_kind_of_trouble_it_is() {
    for status in [408, 500, 502, 503, 504] {
        let trouble = answered(status, None, "<html>\n  Bad   Gateway </html>", NOW);
        assert_eq!(trouble.class, Class::Server, "{status}");
        assert_eq!(trouble.message, "<html> Bad Gateway </html>");
        assert!(trouble.learned.is_empty() && !trouble.daily);
    }
    assert_eq!(refused(404, "NOT_FOUND", "models/gemini-9 is not found").class, Class::NotFound);
    assert_eq!(refused(400, "INVALID_ARGUMENT", "The video is too long.").class, Class::Rejected);
    assert_eq!(refused(403, "PERMISSION_DENIED", "Not allowed in your country.").class, Class::Rejected);
    assert_eq!(answered(429, None, "", NOW).class, Class::RateLimited);
    assert_eq!(refused(500, "API_KEY_INVALID", "The API key service is down.").class, Class::Server);
}

#[test]
fn a_bad_key_is_told_apart_from_other_refusals() {
    assert_eq!(refused(401, "OTHER", "Sign in.").class, Class::KeyRejected);
    assert_eq!(refused(400, "API_KEY_INVALID", "Bad request.").class, Class::KeyRejected);
    assert_eq!(refused(403, "SERVICE_DISABLED", "Enable it first.").class, Class::KeyRejected);
    assert_eq!(refused(400, "OTHER", "API key not valid. Please pass a valid API Key.").class, Class::KeyRejected);
    assert_eq!(refused(404, "API_KEY_EXPIRED", "Gone.").class, Class::KeyRejected);
    let by_status = json!({"error": {"code": 403, "status": "UNAUTHENTICATED", "message": "No."}}).to_string();
    assert_eq!(answered(403, None, &by_status, NOW).class, Class::KeyRejected);
}

#[test]
fn a_request_that_got_no_answer_is_a_network_problem() {
    let trouble = no_answer("timeout:  no answer\n after 600 s");
    assert_eq!(trouble.class, Class::Network);
    assert_eq!(trouble.status, None);
    assert_eq!(trouble.message, "timeout: no answer after 600 s");
    assert_eq!(trouble.retry_seconds, None);
    assert!(trouble.learned.is_empty() && !trouble.daily);
}
