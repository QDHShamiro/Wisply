use super::*;
use crate::analysis;
use crate::config::Config;

const FULL: &str = concat!(
    r#"{"sum": "  Builds a  shelf.\n", "steps": ["#,
    r#"{"t": "01:01", "do": " Open   the file ", "keys": ["Ctrl+O", "Ctrl+O", " "], "menu": ["File", "Open"], "#,
    r#""vals": [{"n": "Name", "v": "shelf.txt"}, {"n": "Empty", "v": " "}, "x"], "#,
    r#""code": "fn main() {  \r\n    run();\r\n}\n\n", "see": "A dialog  opens", "unsure": true}, "#,
    r#"{"t": "1:02:03", "do": "Save"}, {"t": "soon", "do": "Close"}, {"t": "00:05"}, "junk"], "#,
    r#""snd": [{"t": "00:10", "what": "click"}, {"what": ""}], "pts": [{"t": 12, "x": "Point"}]}"#,
);
const STEPS: &str = concat!(
    r#"{"steps": ["#,
    r#"{"t": "00:10", "do": "Press the key", "keys": ["`", "Ctrl+`"], "vals": [{"n": "", "v": "a`b"}], "#,
    r#""code": "ls -la"},"#,
    r#"{"t": "00:30", "do": "Type the loop", "code": "for x in y:\n\n    print(```x```)", "unsure": true},"#,
    r#"{"do": "Wait"},"#,
    r#"{"t": "01:40", "do": "Run it", "unsure": true}],"#,
    r#""snd": [{"t": "00:12", "what": "beep"}, {"t": "00:29", "what": "click"}, {"t": "00:55", "what": "music"},"#,
    r#"{"what": "hum"}, {"t": "00:05", "what": "early"}, {"t": "01:45", "what": "ding"}]}"#,
);
const SUMMARY: &str = concat!(
    r#"{"sum": "A talk.", "pts": [{"t": "00:03", "x": "First"}, {"x": "Second"}], "#,
    r#""snd": [{"t": "00:01", "what": "applause"}]}"#,
);
const STEPS_HEAD: &str = "# Video · 01:00-05:00 · steps · gemini-3.6-flash · 2 requests";
const CACHED_HEAD: &str = "# Talk · 01:30-end · summary · gemini-3.8-flash · cached";
const FIRST_STEP: [&str; 3] = [
    "1. [00:10] Press the key · keys `` ` ``, `` Ctrl+` `` · vals ``a`b`` · code `ls -la`",
    "   snd [00:12] beep",
    "   snd [00:29] click",
];
const MIDDLE_STEPS: [&str; 7] = [
    "2. [00:30] Type the loop · unsure",
    "   ````",
    "   for x in y:",
    "",
    "       print(```x```)",
    "   ````",
    "3. Wait",
];
const LAST_STEP: [&str; 6] =
    ["4. [01:40] Run it · unsure", "   snd [01:45] ding", "Sounds:", "- [00:55] music", "- hum", "- [00:05] early"];

fn settings(page_chars: usize, unsure_times: usize) -> config::Render {
    let mut settings = Config::standard().render;
    settings.page_chars = page_chars;
    settings.unsure_times = unsure_times;
    settings
}

fn windowed() -> Heading<'static> {
    let window = Some(Range { start: 60.0, end: Some(300.0) });
    Heading { title: "", mode: Mode::Steps, model: "gemini-3.6-flash", requests: 2, duration: None, window }
}

fn cached() -> Heading<'static> {
    let window = Some(Range { start: 90.0, end: None });
    Heading { title: "Talk", mode: Mode::Summary, model: "gemini-3.8-flash", requests: 0, duration: None, window }
}

fn steps() -> Analysis {
    analysis::parse(STEPS, Mode::Steps, 0.0, false).expect("steps")
}

fn joined(parts: &[&[&str]]) -> String {
    parts.concat().join("\n")
}

#[test]
fn a_full_answer_is_one_page_with_notes_points_steps_and_sounds() {
    let answer = analysis::parse(FULL, Mode::Full, 60.0, true).expect("full");
    let heading = Heading {
        title: " Build a\n shelf ",
        mode: Mode::Full,
        model: "gemini-3.8-flash",
        requests: 1,
        duration: Some(754.0),
        window: None,
    };
    let text = page(&answer, &heading, 1, &settings(8000, 6), &["A note.".to_string()]).expect("page");
    let wanted = [
        "# Build a shelf (12:34) · full · gemini-3.8-flash · 1 request",
        "Note: A note.",
        "Note: 3 unreadable item(s) from Gemini were left out.",
        "Note: Gemini's answer was cut off, later parts may be missing. Watch the rest with --start and --end.",
        "",
        "Builds a shelf.",
        "Key points:",
        "- [01:12] Point",
        "Steps:",
        "1. [02:01] Open the file · keys `Ctrl+O` · menu `File > Open` · vals Name `shelf.txt` · see: A dialog opens · unsure",
        "   ```",
        "   fn main() {",
        "       run();",
        "   }",
        "   ```",
        "2. [1:03:03] Save",
        "3. Close",
        "Sounds:",
        "- [01:10] click",
        "",
        "unsure at 02:01",
    ];
    assert_eq!(text, wanted.join("\n"));
}

#[test]
fn sounds_sit_under_the_step_they_follow_and_code_keeps_its_backticks() {
    let text = page(&steps(), &windowed(), 1, &settings(8000, 6), &[]).expect("page");
    let wanted = joined(&[&[STEPS_HEAD, ""], &FIRST_STEP, &MIDDLE_STEPS, &LAST_STEP, &["", "unsure at 00:30, 01:40"]]);
    assert_eq!(text, wanted);
}

#[test]
fn a_long_answer_is_cut_into_pages_between_entries() {
    let small = settings(120, 1);
    let read = |number: usize| page(&steps(), &windowed(), number, &small, &[]).expect("page");
    assert_eq!(read(1), joined(&[&[STEPS_HEAD, ""], &FIRST_STEP, &["", "Page 1/3 · next: --page 2"]]));
    let second = joined(&[&[STEPS_HEAD, ""], &MIDDLE_STEPS, &["", "Page 2/3 · next: --page 3 · unsure at 00:30"]]);
    assert_eq!(read(2), second);
    assert_eq!(read(3), joined(&[&[STEPS_HEAD, ""], &LAST_STEP, &["", "Page 3/3 · unsure at 01:40"]]));
}

#[test]
fn a_page_that_does_not_exist_is_refused_with_the_range() {
    for number in [0, 9] {
        let fail = page(&steps(), &windowed(), number, &settings(120, 1), &[]).expect_err("refused");
        assert_eq!(fail.kind, Kind::Input);
        assert_eq!(fail.message, format!("page {number} does not exist, this result has 3 page(s)"));
        assert_eq!(fail.hint, "use --page 1 to 3");
    }
}

#[test]
fn a_cached_summary_names_its_window_and_has_no_footer() {
    let answer = analysis::parse(SUMMARY, Mode::Summary, 0.0, false).expect("summary");
    let text = page(&answer, &cached(), 1, &settings(8000, 6), &[]).expect("page");
    let body = ["A talk.", "Key points:", "- [00:03] First", "- Second", "Sounds:", "- [00:01] applause"];
    assert_eq!(text, joined(&[&[CACHED_HEAD, ""], &body]));
}

#[test]
fn an_empty_answer_says_that_nothing_was_found() {
    let answer = analysis::parse("{}", Mode::Summary, 0.0, false).expect("empty");
    let text = page(&answer, &cached(), 1, &settings(8000, 6), &[]).expect("page");
    assert_eq!(text, joined(&[&[CACHED_HEAD, "", "Gemini found nothing to report here."]]));
}
