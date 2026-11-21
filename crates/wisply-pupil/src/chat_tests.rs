use super::{
    add_probs, add_sound, body, estimate_tokens, fit_chars, frame_budget, heard_block, minute_cap, read_answer,
    span_text, transcript_block,
};
use crate::config::Config;
use crate::job::Segment;
use crate::schema::tests::text;
use crate::video::Picture;
use serde_json::json;

#[test]
fn the_model_is_told_the_span_and_the_pace() {
    let mut said = text();
    let capped = json!({"type": "object", "properties": {"pts": {"type": "array",
        "description": "Points. At most 10.",
        "items": {"type": "object", "properties": {"t": {"type": "string"}, "x": {"type": "string"}}}}}});
    let open = json!({"type": "object", "properties": {"steps": {"type": "array",
        "items": {"type": "object", "properties": {"t": {"type": "string"}}}}}});
    assert_eq!(span_text(&said, &capped, 0.0, 600.0), "From 00:00 to 10:00. One every 01:00.");
    assert_eq!(span_text(&said, &open, 900.0, 1500.0), "From 15:00 to 25:00.");
    assert_eq!(span_text(&said, &capped, 60.0, 60.0), "");
    said.spread_rule.clear();
    assert_eq!(span_text(&said, &capped, 0.0, 600.0), "From 00:00 to 10:00.");
    said.span_rule.clear();
    assert_eq!(span_text(&said, &capped, 0.0, 600.0), "");
}

#[test]
fn without_transcript_a_hearing_model_is_told_to_listen() {
    let config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
    let said = [Segment { start: 5.0, end: 7.0, text: "hello".into() }];
    assert_eq!(heard_block(&config, &[], 0.0, 60.0, true), config.audio.no_transcript);
    assert_eq!(heard_block(&config, &[], 0.0, 60.0, false), config.text.no_transcript);
    assert!(heard_block(&config, &said, 0.0, 60.0, true).contains("hello"));
}

#[test]
fn frames_come_before_the_changing_text() {
    let mut config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
    config.sampling.compact_grammar = false;
    config.text.question_turn = false;
    let pictures =
        vec![Picture { at: 4.0, small: false, jpeg: vec![1, 2, 3] }, Picture { at: 65.0, small: true, jpeg: vec![4] }];
    let schema = json!({"type": "object", "properties": {"t": {"type": "string"}}});
    let sent = body(&config, "2b", "TAIL", &pictures, &schema);
    let content = sent["messages"][0]["content"].as_array().expect("content");
    let kinds: Vec<&str> = content.iter().map(|c| c["type"].as_str().unwrap_or("")).collect();
    assert_eq!(kinds, ["text", "text", "image_url", "text", "image_url", "text"]);
    assert_eq!(content[0]["text"], config.text.frames_header.as_str());
    assert_eq!(content[3]["text"], "[01:05]");
    assert_eq!(content[5]["text"], "TAIL");
    assert_eq!(sent["cache_prompt"], true);
    let hard = &sent["response_format"]["json_schema"]["schema"];
    assert_eq!(hard["properties"]["t"]["pattern"], config.text.time_pattern.as_str());
    assert!(sent.get("grammar").is_none());
    config.sampling.compact_grammar = true;
    let tight = body(&config, "2b", "TAIL", &pictures, &schema);
    assert!(tight.get("response_format").is_none());
    let rules = tight["grammar"].as_str().expect("grammar");
    assert!(rules.contains(r#""\"t\":""#));
    assert!(rules.contains(r#"[0-5] [0-9] ":" [0-5] [0-9]"#));
}

#[test]
fn a_fixed_seed_makes_answers_repeatable() {
    let mut config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
    let schema = json!({"type": "object", "properties": {}});
    config.sampling.seed = Some(7);
    assert_eq!(body(&config, "2b", "T", &[], &schema)["seed"], 7);
    config.sampling.seed = None;
    assert!(body(&config, "2b", "T", &[], &schema).get("seed").is_none());
    assert!(config.bench.cold_start);
}

#[test]
fn question_turn_keeps_frames_apart_from_the_question() {
    let mut config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
    let schema = json!({"type": "object", "properties": {}});
    let pictures = [crate::video::Picture { at: 1.0, small: false, jpeg: vec![1, 2, 3] }];
    config.text.question_turn = false;
    let one = body(&config, "2b", "TAIL", &pictures, &schema);
    assert_eq!(one["messages"].as_array().map(Vec::len), Some(1));
    config.text.question_turn = true;
    let mut two = body(&config, "2b", "TAIL", &pictures, &schema);
    let turns = two["messages"].as_array().expect("messages");
    assert_eq!(turns.len(), 2);
    assert_eq!(turns[1]["role"], "user");
    assert_eq!(turns[1]["content"][0]["text"], "TAIL");
    let first = turns[0]["content"].as_array().expect("content");
    assert_eq!(first.len(), 3);
    assert_eq!(first[2]["type"], "image_url");
    add_sound(&config, &mut two, b"abc");
    let first = two["messages"][0]["content"].as_array().expect("content");
    assert_eq!(first[3]["text"], config.audio.header);
    assert_eq!(first[4]["type"], "input_audio");
    assert_eq!(two["messages"][1]["content"][0]["text"], "TAIL");
    let alone = body(&config, "2b", "TAIL", &[], &schema);
    assert_eq!(alone["messages"].as_array().map(Vec::len), Some(1));
}

#[test]
fn sound_goes_right_before_the_question_and_text_only_has_no_frame_header() {
    let config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
    let schema = json!({"type": "object", "properties": {}});
    let mut sent = body(&config, "lfm3", "TAIL", &[], &schema);
    assert_eq!(sent["messages"][0]["content"].as_array().map(Vec::len), Some(1));
    add_sound(&config, &mut sent, b"abc");
    let content = sent["messages"][0]["content"].as_array().expect("content");
    assert_eq!(content.len(), 3);
    assert_eq!(content[0]["text"], config.audio.header);
    assert_eq!(content[1]["type"], "input_audio");
    assert_eq!(content[1]["input_audio"]["data"], "YWJj");
    assert_eq!(content[1]["input_audio"]["format"], config.audio.format);
    assert_eq!(content[2]["text"], "TAIL");
}

fn segment(start: f64, end: f64, text: &str) -> Segment {
    Segment { start, end, text: text.into() }
}

#[test]
fn transcript_keeps_only_the_watched_part() {
    let segments = vec![segment(0.0, 4.0, "hello"), segment(65.0, 70.0, " bevel "), segment(200.0, 210.0, "late")];
    assert_eq!(transcript_block(&text(), &segments, 60.0, 120.0), "Said:\n[01:05] bevel");
    assert_eq!(transcript_block(&text(), &segments, 100.0, 120.0), "None.");
    assert_eq!(transcript_block(&text(), &[segment(1.0, 2.0, "  ")], 0.0, 9.0), "None.");
}

#[test]
fn frame_budget_leaves_room_for_the_answer() {
    assert_eq!(frame_budget(32768, 4096, 512, 2000, 260, 64), Ok(64));
    assert_eq!(frame_budget(32768, 4096, 512, 20000, 260, 64), Ok(31));
    assert_eq!(frame_budget(8192, 4096, 512, 4000, 260, 64), Err(8608));
    assert_eq!(frame_budget(32768, 4096, 512, 0, 0, 64), Err(4608));
}

#[test]
fn short_videos_get_fewer_frames_per_minute() {
    assert_eq!(minute_cap(64, 84.0, 12.0), 17);
    assert_eq!(minute_cap(64, 600.0, 12.0), 64);
    assert_eq!(minute_cap(64, 84.0, 0.0), 64);
    assert_eq!(minute_cap(64, 1.0, 12.0), 1);
    assert_eq!(minute_cap(5, 600.0, 12.0), 5);
}

#[test]
fn answers_are_read_from_the_reply() {
    let value = json!({
        "choices": [{"message": {"content": "{\"sum\":\"x\"}"}, "finish_reason": "stop"}],
        "usage": {"prompt_tokens": 900, "completion_tokens": 40}
    });
    let answer = read_answer(&value).expect("answer");
    assert_eq!(answer.text, "{\"sum\":\"x\"}");
    assert_eq!((answer.finish.as_str(), answer.prompt_tokens, answer.answer_tokens), ("stop", 900, 40));
    assert!(read_answer(&json!({"choices": []})).is_none());
}

#[test]
fn letter_odds_ride_along_only_when_asked() {
    let plain = json!({"choices": [{"message": {"content": "{}"}}]});
    assert!(read_answer(&plain).expect("plain").probs.is_none());
    let odds = json!([{"token": "A", "logprob": -0.1, "top_logprobs": [{"token": "B", "logprob": -2.3}]}]);
    let value = json!({"choices": [{"message": {"content": "{}"}, "logprobs": {"content": odds}}]});
    assert_eq!(read_answer(&value).expect("odds").probs, Some(odds));
    let mut sent = json!({"model": "m"});
    add_probs(&mut sent, 8);
    assert_eq!((sent["logprobs"].clone(), sent["top_logprobs"].clone()), (json!(true), json!(8)));
}

#[test]
fn text_cut_to_the_free_room_never_costs_more_tokens_than_the_room() {
    let config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
    let budget = &config.budget;
    for room in [0u32, 1, 7, 1000, 120_000] {
        let chars = fit_chars(room, budget);
        let text = "x".repeat(chars);
        assert!(estimate_tokens(&text, budget) <= room, "room {room}");
    }
    assert_eq!(fit_chars(0, budget), 0);
}
