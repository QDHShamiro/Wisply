use serde_json::{Value, json};

use super::*;
use crate::fixture;

const AUTO_VTT: &str = "WEBVTT\nKind: captions\nLanguage: en\n\n\
    00:00:00.000 --> 00:00:02.000 align:start position:0%\nIn<00:00:00.120><c> the</c><00:00:00.200><c> next</c>\n\n\
    00:00:02.000 --> 00:00:02.010 align:start position:0%\nIn the next\n\n\
    00:00:02.010 --> 00:00:05.000 align:start position:0%\nIn the next\nfew&nbsp;videos\n\n\
    01:02:03.5 --> 01:02:04.250\nlate line\n";

fn close(found: f64, wanted: f64) -> bool {
    (found - wanted).abs() < 1e-6
}

fn texts(cues: &[Segment]) -> Vec<&str> {
    cues.iter().map(|cue| cue.text.as_str()).collect()
}

fn track(changes: Value) -> Option<Track> {
    choose_track(&fixture::info(changes))
}

fn found(key: &str, automatic: bool) -> Option<Track> {
    Some(Track { key: key.to_string(), automatic })
}

fn sound(language: &str, preference: i64, picture: &str) -> Value {
    json!({"acodec": "opus", "vcodec": picture, "language": language, "language_preference": preference})
}

fn manual_json3() -> Value {
    json!({"wireMagic": "pb3", "events": [
        {"tStartMs": 1480, "dDurationMs": 3861, "segs": [{"utf8": "hello this is a\u{a0}quick"}]},
        {"tStartMs": 5341, "dDurationMs": 7278, "segs": [{"utf8": "default scene,\nthe cube"}]},
        {"tStartMs": 12619, "dDurationMs": 2584, "segs": [{"utf8": "Shift + Ctrl + B"}]},
    ]})
}

fn auto_json3() -> Value {
    json!({"events": [
        {"tStartMs": 0, "dDurationMs": 85720, "id": 1, "wpWinPosId": 1},
        {"tStartMs": 0, "dDurationMs": 3120, "wWinId": 1,
         "segs": [{"utf8": "In"}, {"utf8": " the", "tOffsetMs": 120}, {"utf8": " next"}]},
        {"tStartMs": 2000, "dDurationMs": 1120, "wWinId": 1, "aAppend": 1, "segs": [{"utf8": "\n"}]},
        {"tStartMs": 2010, "dDurationMs": 3000, "wWinId": 1, "segs": [{"utf8": "few videos"}]},
    ]})
}

fn words() -> Vec<Segment> {
    (0..10).map(|number| Segment::new(f64::from(number), f64::from(number) + 1.0, format!("w{number}"))).collect()
}

fn thirds() -> Vec<Segment> {
    vec![Segment::new(0.0, 10.0, "a"), Segment::new(10.0, 20.0, "b"), Segment::new(20.0, 30.0, "c")]
}

#[test]
fn manual_subtitles_in_the_video_language_come_first() {
    let chosen = track(json!({})).unwrap();
    assert_eq!(chosen, Track { key: "en".to_string(), automatic: false });
    assert_eq!(chosen.language(), "en");
}

#[test]
fn original_captions_win_over_foreign_manual_subtitles() {
    let chosen = track(json!({"subtitles": {"fr": []}})).unwrap();
    assert_eq!(chosen, Track { key: "en-orig".to_string(), automatic: true });
    assert_eq!(chosen.language(), "en");
    assert_eq!(track(json!({"language": null, "subtitles": {}})), found("en-orig", true));
}

#[test]
fn a_dubbed_video_reads_the_captions_of_its_original_sound() {
    let formats = json!([
        sound("ar", -1, "none"),
        sound("en-US", 10, "none"),
        sound("ar", -1, "avc1"),
        {"format_id": "614", "acodec": "none", "vcodec": "vp09.00.40.08"},
    ]);
    let captions = json!({"ar-orig": [], "bn-orig": [], "de-DE-orig": [], "en-orig": [], "ko-orig": []});
    let changes = json!({"language": "ar", "formats": formats, "subtitles": {}, "automatic_captions": captions});
    assert_eq!(track(changes), found("en-orig", true));
}

#[test]
fn other_tracks_are_taken_only_when_nothing_better_exists() {
    let foreign = json!({"language": "ja", "subtitles": {"fr": []}, "automatic_captions": {}});
    assert_eq!(track(foreign), found("fr", false));
    let translated = json!({"subtitles": {}, "automatic_captions": {"de": [], "en": []}});
    assert_eq!(track(translated), found("en", true));
    let single = json!({"language": null, "subtitles": {}, "automatic_captions": {"de": []}});
    assert_eq!(track(single), found("de", true));
}

#[test]
fn no_track_is_chosen_when_nothing_fits() {
    assert_eq!(track(json!({"subtitles": {}, "automatic_captions": {}})), None);
    let unknown = json!({"language": "ja", "subtitles": {}, "automatic_captions": {"de": [], "en": []}});
    assert_eq!(track(unknown), None);
}

#[test]
fn language_codes_and_text_are_tidied() {
    assert_eq!(base_language("en-US"), "en");
    assert_eq!(base_language("DE"), "de");
    assert_eq!(tidy_text(" a&nbsp;b\u{a0} c\n d "), "a b c d");
    assert_eq!(tidy_text("Tom &amp; Jerry &#39;s &#x41; &unknown;"), "Tom & Jerry 's A &unknown;");
}

#[test]
fn manual_json3_becomes_timed_lines() {
    let cues = parse_json3(&manual_json3());
    assert_eq!(texts(&cues), ["hello this is a quick", "default scene, the cube", "Shift + Ctrl + B"]);
    let times: Vec<(f64, f64)> = cues.iter().map(|cue| (cue.start, cue.end)).collect();
    for ((start, end), (wanted_start, wanted_end)) in
        times.iter().zip([(1.48, 5.341), (5.341, 12.619), (12.619, 15.203)])
    {
        assert!(close(*start, wanted_start) && close(*end, wanted_end), "{start} {end}");
    }
}

#[test]
fn automatic_json3_drops_window_rows_and_overlaps() {
    let cues = parse_json3(&auto_json3());
    assert_eq!(texts(&cues), ["In the next", "few videos"]);
    assert!(close(cues[0].end, 2.01) && close(cues[1].start, 2.01));
}

#[test]
fn broken_json3_gives_no_lines() {
    for broken in [json!(null), json!([]), json!({"events": "x"}), json!({"events": [1, {"segs": []}]})] {
        assert!(parse_json3(&broken).is_empty(), "{broken}");
    }
}

#[test]
fn vtt_times_are_read_in_every_usual_form() {
    assert!(close(vtt_seconds("00:00:01.480").unwrap(), 1.48));
    assert!(close(vtt_seconds("01:02:03.5").unwrap(), 3723.5));
    assert!(close(vtt_seconds("02:03,250").unwrap(), 123.25));
    assert!(close(vtt_seconds(" 00:00:05.341 align:start").unwrap(), 5.341));
    assert_eq!(vtt_seconds("nonsense"), None);
    assert_eq!(vtt_seconds(""), None);
}

#[test]
fn rolling_vtt_captions_lose_their_repeated_lines() {
    let cues = parse_vtt(&format!("\u{feff}{AUTO_VTT}"));
    assert_eq!(texts(&cues), ["In the next", "few videos", "late line"]);
    assert!(close(cues[1].start, 2.01) && close(cues[2].start, 3723.5));
    assert_eq!(parse_vtt(&AUTO_VTT.replace('\n', "\r\n")).len(), 3);
    assert!(parse_vtt("WEBVTT\n\nNOTE nothing here\n").is_empty());
    assert!(parse_vtt("").is_empty());
}

#[test]
fn overlapping_lines_are_clipped_at_the_next_start() {
    let cues =
        clip_overlaps(vec![Segment::new(5.0, 9.0, "b"), Segment::new(0.0, 7.0, "a"), Segment::new(5.0, 6.0, "c")]);
    assert_eq!(cues, [Segment::new(0.0, 5.0, "a"), Segment::new(5.0, 9.0, "b"), Segment::new(5.0, 6.0, "c")]);
}

#[test]
fn short_lines_are_grouped_by_time_and_length() {
    let by_time = group_segments(&words(), 4.0, 100);
    assert_eq!(texts(&by_time), ["w0 w1 w2 w3", "w4 w5 w6 w7", "w8 w9"]);
    assert_eq!((by_time[0].start, by_time[0].end), (0.0, 4.0));
    let by_length = group_segments(&words(), 60.0, 5);
    assert_eq!(texts(&by_length), ["w0 w1", "w2 w3", "w4 w5", "w6 w7", "w8 w9"]);
    assert!(group_segments(&[], 4.0, 100).is_empty());
}

#[test]
fn a_time_window_keeps_the_lines_that_touch_it() {
    assert_eq!(texts(&within(&thirds(), Some(12.0), Some(19.0))), ["b"]);
    assert_eq!(texts(&within(&thirds(), None, Some(10.0))), ["a", "b"]);
    assert_eq!(texts(&within(&thirds(), Some(20.0), None)), ["b", "c"]);
    assert_eq!(within(&thirds(), None, None).len(), 3);
}
