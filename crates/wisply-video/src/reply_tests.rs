use serde_json::json;

use super::*;

fn output(texts: &[&str]) -> Value {
    let content: Vec<Value> = texts.iter().map(|text| json!({"type": "text", "text": text})).collect();
    json!({"type": "model_output", "content": content})
}

fn answer(steps: Vec<Value>) -> Value {
    json!({"status": "completed", "id": "v1_abc", "steps": steps})
}

fn at(day: i64, hour: i64, second: i64) -> f64 {
    (pacific::days_from_civil(2026, 11, day) * DAY + hour * HOUR + second) as f64
}

#[test]
fn the_answer_is_the_text_after_the_last_question() {
    let steps = vec![
        output(&["old"]),
        json!({"type": "user_input", "content": [{"type": "text", "text": "ask"}]}),
        output(&["a", "b"]),
        json!({"type": "thought", "summary": "hm"}),
        output(&["c"]),
    ];
    assert_eq!(output_text(&answer(steps)), "abc");
    assert_eq!(output_text(&answer(Vec::new())), "");
    assert_eq!(output_text(&json!({"status": "failed"})), "");
    assert_eq!(output_text(&json!({"steps": "none"})), "");
}

#[test]
fn text_before_a_tool_call_does_not_count_as_the_answer() {
    let steps = vec![output(&["plan"]), json!({"type": "function_call", "name": "look"}), output(&["done"])];
    assert_eq!(output_text(&answer(steps)), "done");
    let waiting = vec![output(&["so far"]), json!({"type": "function_call", "name": "look"})];
    assert_eq!(output_text(&answer(waiting)), "so far");
}

#[test]
fn only_the_closing_text_of_a_mixed_output_is_read() {
    let picture = json!({"type": "image", "data": "x"});
    let mixed = json!({"type": "model_output", "content": [
        {"type": "text", "text": "x"}, picture, {"type": "text", "text": "y"}, {"type": "text", "text": 7}
    ]});
    assert_eq!(output_text(&answer(vec![output(&["before"]), mixed])), "y");
    let trailing = json!({"type": "model_output", "content": [{"type": "text", "text": "x"}, picture]});
    assert_eq!(output_text(&answer(vec![trailing])), "x");
    assert_eq!(output_text(&answer(vec![json!({"type": "model_output", "content": "odd"}), output(&["z"])])), "z");
}

#[test]
fn the_state_and_names_of_an_answer_are_read_as_words() {
    let reply = json!({"status": "requires_continuation", "id": "v1_abc", "continuation_token": 5});
    assert_eq!(word(&reply, "status"), REQUIRES_CONTINUATION);
    assert_eq!(word(&reply, "id"), "v1_abc");
    assert_eq!(word(&reply, "continuation_token"), "");
    assert_eq!(word(&reply, "model"), "");
}

#[test]
fn the_tokens_of_an_answer_are_counted() {
    let reply = json!({"usage": {"total_tokens": 12, "total_input_tokens": 9, "cached": null}});
    let read = usage(&reply);
    assert_eq!(read.keys().collect::<Vec<_>>(), ["total_tokens", "total_input_tokens"]);
    assert_eq!(total_tokens(&read), 12);
    assert!(usage(&json!({"usage": [1]})).is_empty());
    assert!(usage(&json!({})).is_empty());
    let parts =
        json!({"total_input_tokens": 5, "total_output_tokens": 3, "total_thought_tokens": 2, "total_tokens": true});
    assert_eq!(total_tokens(parts.as_object().expect("object")), 10);
    assert_eq!(total_tokens(&Usage::new()), 0);
}

#[test]
fn the_tokens_of_several_hops_are_added_up() {
    let first = json!({
        "total_tokens": 10, "initial_input_tokens": 7, "note": "a",
        "input_tokens_by_modality": [{"modality": "video", "tokens": 5}]
    });
    let second = json!({
        "total_tokens": 4, "initial_input_tokens": 9, "note": "b", "total_output_tokens": 2,
        "input_tokens_by_modality": [{"modality": "text", "tokens": 1}]
    });
    let hops = [first, second].map(|hop| hop.as_object().expect("object").clone());
    let wanted = json!({
        "total_tokens": 14, "initial_input_tokens": 7, "note": "a",
        "input_tokens_by_modality": [{"modality": "video", "tokens": 5}, {"modality": "text", "tokens": 1}],
        "total_output_tokens": 2
    });
    assert_eq!(Value::Object(merge_usage(&hops)), wanted);
    assert!(merge_usage(&[]).is_empty());
}

#[test]
fn the_parts_of_an_error_answer_are_read() {
    let body = json!({"error": {"code": 429, "status": "RESOURCE_EXHAUSTED", "message": " Quota\n exceeded.  Wait. ",
        "details": [{"reason": "RATE_LIMIT_EXCEEDED"}, "odd", {"retryDelay": "34s"}, {"reason": 5}]}});
    assert_eq!(error_message(&body), "Quota exceeded. Wait.");
    assert_eq!(error_reasons(&body), ["RESOURCE_EXHAUSTED", "RATE_LIMIT_EXCEEDED"]);
    assert_eq!(error_details(&body).count(), 3);
    for odd in [json!(null), json!("<html>"), json!({"error": "no"}), json!({"error": {"details": {}}})] {
        assert_eq!(error_message(&odd), "");
        assert!(error_reasons(&odd).is_empty());
        assert_eq!(error_details(&odd).count(), 0);
    }
}

#[test]
fn a_wait_is_read_from_a_duration() {
    assert_eq!(seconds_from_duration(Some(&json!("34s"))), Some(34.0));
    assert_eq!(seconds_from_duration(Some(&json!(" 1.5s "))), Some(1.5));
    for odd in [json!("34"), json!("s"), json!("-3s"), json!("1.s"), json!(34), json!(null)] {
        assert_eq!(seconds_from_duration(Some(&odd)), None, "{odd}");
    }
    assert_eq!(seconds_from_duration(None), None);
}

#[test]
fn a_wait_is_read_from_the_retry_header() {
    let now = at(21, 20, 0);
    assert_eq!(seconds_from_header(Some("12"), now), Some(12.0));
    assert_eq!(seconds_from_header(Some(" 2.5 "), now), Some(2.5));
    assert_eq!(seconds_from_header(Some("Sat, 21 Nov 2026 20:00:30 GMT"), now), Some(30.0));
    assert_eq!(seconds_from_header(Some("Sun, 22 nov 2026 20:00:00 utc"), now), Some(DAY as f64));
    let nothing =
        ["0", "-3", "inf", "NaN", "", "soon", "Sat, 21 Nov 2026 19:59:00 GMT", "Sat, 21 Nov 2026 21:00:00 CET"];
    for odd in nothing.into_iter().chain([
        "Sat, 21 Foo 2026 21:00:00 GMT",
        "21 Nov 2026 21:00:00 GMT",
        "Sat, 21 Nov 2026 21:00 GMT",
    ]) {
        assert_eq!(seconds_from_header(Some(odd), now), None, "{odd}");
    }
    assert_eq!(seconds_from_header(None, now), None);
}
