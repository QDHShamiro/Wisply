use super::*;
use crate::config::Config;

fn settings() -> config::Analysis {
    Config::standard().analysis
}

fn spoken(spans: &[(f64, f64)]) -> Vec<Segment> {
    spans.iter().map(|(start, end)| Segment { start: *start, end: *end, text: "words".to_string() }).collect()
}

#[test]
fn the_fingerprints_are_the_ones_iris_stored_its_answers_under() {
    let settings = settings();
    assert_eq!(fingerprint(Mode::Summary, &settings), "2-d063a03a180a");
    assert_eq!(fingerprint(Mode::Steps, &settings), "2-42bbe9dc90d7");
    assert_eq!(fingerprint(Mode::Full, &settings), "2-b8e898597d30");
}

#[test]
fn a_changed_limit_changes_the_fingerprint() {
    let mut changed = settings();
    changed.max_steps += 1;
    assert_ne!(fingerprint(Mode::Steps, &changed), fingerprint(Mode::Steps, &settings()));
    assert_eq!(fingerprint(Mode::Summary, &changed), fingerprint(Mode::Summary, &settings()));
}

#[test]
fn the_prompt_names_video_window_task_focus_and_quiet_parts_in_that_order() {
    let chapters = [
        Chapter { start: 0.0, end: 120.0, title: "Intro".to_string() },
        Chapter { start: 120.0, end: 754.0, title: "Cutting".to_string() },
    ];
    let facts = Facts {
        title: "Build a shelf",
        channel: "Wood Shop",
        duration: Some(754.0),
        chapters: &chapters,
        description: "A short how-to.",
        window: Some(Range { start: 60.0, end: Some(300.0) }),
        quiet: &[(70.0, 95.0), (200.0, 215.5)],
        focus: Some("the saw settings"),
    };
    let prompt = build(Mode::Steps, &facts, &settings());
    let lines: Vec<&str> = prompt.split('\n').collect();
    assert_eq!(lines.len(), 8);
    assert_eq!(prompt.chars().count(), 1078);
    assert!(lines[0].starts_with("You watch a video for an assistant"), "{}", lines[0]);
    assert!(lines[0].ends_with("or said aloud, never infer it."), "{}", lines[0]);
    assert_eq!(lines[1], "Video: \"Build a shelf\" by Wood Shop, length 12:34.");
    assert_eq!(lines[2], "Chapters: 00:00 Intro; 02:00 Cutting");
    assert_eq!(lines[3], "Description from the uploader: \"\"\"A short how-to.\"\"\"");
    assert_eq!(lines[4], "You see only the part from 01:00 to 05:00.");
    assert!(lines[5].starts_with("This is a tutorial. List every step"), "{}", lines[5]);
    assert!(lines[5].ends_with("not slide changes. Also list notable sounds other than speech."), "{}", lines[5]);
    assert_eq!(lines[6], "Focus on: the saw settings. Leave out what is unrelated to it.");
    assert_eq!(
        lines[7],
        "Nobody speaks at 01:10-01:35, 03:20-03:35. Watch these parts closely, silent actions are easy to miss."
    );
}

#[test]
fn without_facts_the_prompt_is_opening_and_task() {
    let prompt = build(Mode::Summary, &Facts::default(), &settings());
    let lines: Vec<&str> = prompt.split('\n').collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[1].starts_with("Summarise the video: the gist"), "{}", lines[1]);
    assert!(lines[1].ends_with("such as music, alerts or effects."), "{}", lines[1]);
}

#[test]
fn an_open_window_ends_at_the_length_or_at_the_end() {
    let window = Some(Range { start: 90.0, end: None });
    let known = Facts { duration: Some(600.0), window, ..Facts::default() };
    assert!(build(Mode::Summary, &known, &settings()).contains("You see only the part from 01:30 to 10:00."));
    let unknown = Facts { window, ..Facts::default() };
    assert!(build(Mode::Summary, &unknown, &settings()).contains("You see only the part from 01:30 to the end."));
}

#[test]
fn only_the_chapters_inside_the_window_are_named() {
    let chapters = [
        Chapter { start: 0.0, end: 100.0, title: "One".to_string() },
        Chapter { start: 100.0, end: 200.0, title: "Two".to_string() },
        Chapter { start: 200.0, end: 300.0, title: "Three".to_string() },
    ];
    let window = Some(Range { start: 100.0, end: Some(200.0) });
    let facts = Facts { chapters: &chapters, window, ..Facts::default() };
    let prompt = build(Mode::Steps, &facts, &settings());
    assert!(prompt.contains("\nChapters: 01:40 Two\n"), "{prompt}");
}

#[test]
fn quiet_parts_are_the_longest_gaps_in_time_order() {
    let mut settings = settings();
    settings.quiet_min_seconds = 8.0;
    settings.max_quiet_spans = 2;
    let segments = spoken(&[(10.0, 20.0), (30.0, 40.0), (40.0, 47.0), (90.0, 95.0)]);
    assert_eq!(quiet_spans(&segments, 0.0, 120.0, &settings), [(47.0, 90.0), (95.0, 120.0)]);
    settings.max_quiet_spans = 12;
    assert_eq!(quiet_spans(&segments, 0.0, 120.0, &settings), [(0.0, 10.0), (20.0, 30.0), (47.0, 90.0), (95.0, 120.0)]);
    assert!(quiet_spans(&segments, 30.0, 47.0, &settings).is_empty());
    assert_eq!(quiet_spans(&[], 5.0, 20.0, &settings), [(5.0, 20.0)]);
}

#[test]
fn a_focus_is_tidied_and_a_long_one_is_refused() {
    let settings = settings();
    assert_eq!(
        clean_focus(Some("  the   saw\nsettings. "), &settings).expect("focus"),
        Some("the saw settings".to_string())
    );
    assert_eq!(clean_focus(Some(" . "), &settings).expect("empty"), None);
    assert_eq!(clean_focus(None, &settings).expect("none"), None);
    let fail = clean_focus(Some(&"word ".repeat(100)), &settings).expect_err("long");
    assert_eq!(fail.kind, Kind::Input);
    assert!(fail.message.contains("499 characters long, at most 300 fit"), "{}", fail.message);
}
