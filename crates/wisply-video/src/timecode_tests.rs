use serde_json::json;

use super::*;

fn span(start: f64, end: Option<f64>) -> Option<Range> {
    Some(Range { start, end })
}

#[test]
fn time_forms_become_seconds() {
    let cases = [
        ("83", 83.0),
        ("83.5", 83.5),
        ("83s", 83.0),
        ("83 sec", 83.0),
        (" 1:23 ", 83.0),
        ("01:23", 83.0),
        ("1:02:03", 3723.0),
        ("90:00", 5400.0),
        ("0:07.5", 7.5),
        ("1m23s", 83.0),
        ("1M23S", 83.0),
        ("1h2m3s", 3723.0),
        ("1h 2m", 3720.0),
        ("2min", 120.0),
        ("1h", 3600.0),
        ("\"0:30\"", 30.0),
        ("'83'", 83.0),
        ("`1:23`", 83.0),
        ("[90]", 90.0),
        ("\t1:23\n", 83.0),
        ("[ '0:30' ]", 30.0),
    ];
    for (text, seconds) in cases {
        assert_eq!(parse(text).unwrap(), seconds, "{text}");
    }
    assert_eq!(parse_value(&json!(83)).unwrap(), 83.0);
    assert_eq!(parse_value(&json!(83.5)).unwrap(), 83.5);
    assert_eq!(parse_value(&json!(0)).unwrap(), 0.0);
}

#[test]
fn bad_times_are_refused_with_the_accepted_forms() {
    let texts = ["", "   ", "\"\"", "[]", "soon", "-5", "1:60", "1:75:00", "1h30", "1:2:3:4", "12abc", "nan", "inf"];
    for text in texts {
        let fail = parse(text).unwrap_err();
        assert_eq!(fail.kind, Kind::Time, "{text}");
        assert!(fail.text().contains("1:02:03"), "{text}");
    }
    for value in [json!(-5), json!(true), json!(null), json!(["1:23"])] {
        assert_eq!(parse_value(&value).unwrap_err().kind, Kind::Time, "{value}");
    }
}

#[test]
fn times_inside_model_text_are_found() {
    let cases = [
        (json!("01:15"), 75.0),
        (json!("1:02:03"), 3723.0),
        (json!("00:07.5"), 7.5),
        (json!("at 02:10 - 02:20"), 130.0),
        (json!("75"), 75.0),
        (json!("75s"), 75.0),
        (json!("1m15s"), 75.0),
        (json!(42), 42.0),
    ];
    for (value, seconds) in cases {
        assert_eq!(find(&value), Some(seconds), "{value}");
    }
    for value in [json!("soon"), json!(""), json!(null), json!(true), json!(["01:15"]), json!(-3)] {
        assert_eq!(find(&value), None, "{value}");
    }
}

#[test]
fn seconds_are_written_as_clock_text_and_read_back() {
    let cases =
        [(0.0, "00:00"), (83.0, "01:23"), (83.9, "01:23"), (3600.0, "1:00:00"), (3723.0, "1:02:03"), (-4.0, "00:00")];
    for (seconds, text) in cases {
        assert_eq!(format(seconds), text);
    }
    for seconds in [0.0, 59.0, 60.0, 3599.0, 3600.0, 43199.0] {
        assert_eq!(parse(&format(seconds)).unwrap(), seconds);
    }
}

#[test]
fn no_start_and_no_end_means_the_whole_video() {
    assert_eq!(range(None, None, None).unwrap(), None);
    assert_eq!(range(Some("0"), None, None).unwrap(), None);
    assert_eq!(range(Some("0:00"), None, Some(600.0)).unwrap(), None);
}

#[test]
fn a_range_keeps_start_and_end() {
    assert_eq!(range(Some("1:00"), Some("2:00"), None).unwrap(), span(60.0, Some(120.0)));
    assert_eq!(range(None, Some("90"), None).unwrap(), span(0.0, Some(90.0)));
    assert_eq!(range(Some("5m"), None, None).unwrap(), span(300.0, None));
}

#[test]
fn range_offsets_are_whole_seconds_that_cover_the_range() {
    let wide = Range { start: 60.7, end: Some(119.2) };
    assert_eq!(wide.start_offset(), "60s");
    assert_eq!(wide.end_offset().as_deref(), Some("120s"));
    assert_eq!(Range { start: 60.0, end: None }.end_offset(), None);
    assert_eq!(Range { start: 60.0, end: Some(3723.0) }.label(), "01:00-1:02:03");
    assert_eq!(Range { start: 60.0, end: None }.label(), "01:00-end");
}

#[test]
fn the_end_has_to_come_after_the_start() {
    for (start, end) in [(Some("2:00"), "1:00"), (Some("60"), "60"), (None, "0")] {
        let fail = range(start, Some(end), None).unwrap_err();
        assert!(fail.message.contains("is not after start"), "{}", fail.message);
    }
}

#[test]
fn the_video_length_bounds_a_range() {
    let fail = range(Some("11:00"), None, Some(600.0)).unwrap_err();
    assert!(fail.message.contains("10:00"), "{}", fail.message);
    assert_eq!(range(Some("1:00"), Some("20:00"), Some(600.0)).unwrap(), span(60.0, None));
    assert_eq!(range(None, Some("20:00"), Some(600.0)).unwrap(), None);
}

#[test]
fn timestamps_are_sorted_and_deduplicated() {
    assert_eq!(stamps("2:00, 30, 30, 1m", 10, None).unwrap(), [30.0, 60.0, 120.0]);
    assert_eq!(stamps("1:23, 0:10; 45", 10, None).unwrap(), [10.0, 45.0, 83.0]);
    assert_eq!(stamps("12", 10, None).unwrap(), [12.0]);
    assert_eq!(stamps("[\"0:30\", \"1:00\"]", 10, None).unwrap(), [30.0, 60.0]);
}

#[test]
fn missing_and_bad_timestamps_are_refused() {
    for values in ["", " , ", " ; ,"] {
        assert_eq!(stamps(values, 10, None).unwrap_err().kind, Kind::Time, "{values}");
    }
    let fail = stamps("1:00, later", 10, None).unwrap_err();
    assert!(fail.message.contains("'later'"), "{}", fail.message);
}

#[test]
fn too_many_timestamps_are_refused_with_the_limit() {
    let fail = stamps("1, 2, 3, 4", 3, None).unwrap_err();
    assert!(fail.message.contains("at most 3"), "{}", fail.message);
    assert_eq!(stamps("1, 1, 2, 3", 3, None).unwrap(), [1.0, 2.0, 3.0]);
}

#[test]
fn timestamps_past_the_video_end_are_named() {
    let fail = stamps("1:00, 10:00, 12:30", 10, Some(600.0)).unwrap_err();
    assert!(fail.message.contains("10:00, 12:30"), "{}", fail.message);
}
