use super::*;

const FULL: &str = concat!(
    r#"{"sum": "  Builds a  shelf.\n", "steps": ["#,
    r#"{"t": "01:01", "do": " Open   the file ", "keys": ["Ctrl+O", "Ctrl+O", " "], "menu": ["File", "Open"], "#,
    r#""vals": [{"n": "Name", "v": "shelf.txt"}, {"n": "Empty", "v": " "}, "x"], "#,
    r#""code": "fn main() {  \r\n    run();\r\n}\n\n", "see": "A dialog  opens", "unsure": true}, "#,
    r#"{"t": "1:02:03", "do": "Save"}, {"t": "soon", "do": "Close"}, {"t": "00:05"}, "junk"], "#,
    r#""snd": [{"t": "00:10", "what": "click"}, {"what": ""}], "pts": [{"t": 12, "x": "Point"}]}"#,
);
const FULL_STORED: &str = concat!(
    r#"{"sum":"Builds a shelf.","pts":[{"t":72.0,"x":"Point"}],"steps":[{"t":121.0,"do":"Open the file","#,
    r#""keys":["Ctrl+O"],"menu":["File","Open"],"vals":[{"n":"Name","v":"shelf.txt"}],"#,
    r#""code":"fn main() {\n    run();\n}","see":"A dialog opens","unsure":true},"#,
    r#"{"t":3783.0,"do":"Save"},{"do":"Close"}],"snd":[{"t":70.0,"what":"click"}],"dropped":3}"#,
);

fn stored(text: &str, mode: Mode, shift: f64, cut: bool) -> String {
    let analysis = parse(text, mode, shift, cut).expect("readable");
    serde_json::to_string(&data(&analysis)).expect("json")
}

fn refused(text: &str) -> Fail {
    parse(text, Mode::Steps, 0.0, false).expect_err("refused")
}

#[test]
fn a_whole_answer_is_tidied_shifted_and_counted_the_way_iris_stored_it() {
    assert_eq!(stored(FULL, Mode::Full, 60.0, false), FULL_STORED);
    let analysis = parse(FULL, Mode::Full, 60.0, false).expect("readable");
    assert_eq!(analysis.unsure_times(), [121.0]);
    assert_eq!(analysis.steps[0].code, "fn main() {\n    run();\n}");
    assert_eq!(analysis.steps[2].at, None);
    assert!(!analysis.cut);
    assert!(!analysis.is_empty());
}

#[test]
fn a_stored_answer_reads_back_as_the_same_answer() {
    let first = parse(FULL, Mode::Full, 60.0, false).expect("readable");
    let again = read(&data(&first), Mode::Full, 0.0, false).expect("stored");
    assert_eq!(again, first);
    let kept = r#"{"steps":[{"t":61.5,"do":"a"}],"dropped":2,"cut":true}"#;
    assert_eq!(stored(kept, Mode::Steps, 0.0, false), kept);
    assert_eq!(stored("  {}  ", Mode::Steps, 0.0, false), "{}");
    assert!(parse("{}", Mode::Steps, 0.0, false).expect("empty").is_empty());
}

#[test]
fn a_cut_off_answer_keeps_every_whole_entry_and_is_marked_cut() {
    let between = r#"{"steps":[{"t":"00:01","do":"a"},{"t":"00:02","do":"b","keys":["x","y"]},{"t":"00:0"#;
    assert_eq!(
        stored(between, Mode::Steps, 0.0, false),
        r#"{"steps":[{"t":1.0,"do":"a"},{"t":2.0,"do":"b","keys":["x","y"]}],"cut":true}"#
    );
    let in_text = r#"{"sum":"He said \"[hi]\" {ok}","pts":[{"t":"00:03","x":"one"},{"t":"00:04","x":"tw"#;
    assert_eq!(
        stored(in_text, Mode::Summary, 0.0, false),
        r#"{"sum":"He said \"[hi]\" {ok}","pts":[{"t":3.0,"x":"one"}],"cut":true}"#
    );
    let wide = r#"{"sum":"Größe ≥ 3 🙂","pts":[{"t":"00:03","x":"eins"},{"t":"00:0"#;
    assert_eq!(
        stored(wide, Mode::Summary, 0.0, false),
        r#"{"sum":"Größe ≥ 3 🙂","pts":[{"t":3.0,"x":"eins"}],"cut":true}"#
    );
}

#[test]
fn an_answer_that_was_told_cut_stays_cut() {
    assert_eq!(stored(r#"{"sum":"x"}"#, Mode::Summary, 0.0, true), r#"{"sum":"x","cut":true}"#);
}

#[test]
fn only_a_whole_count_and_a_true_flag_are_taken_over() {
    assert_eq!(stored(r#"{"sum":"x","dropped":true,"cut":"yes"}"#, Mode::Summary, 0.0, false), r#"{"sum":"x"}"#);
    assert_eq!(stored(r#"{"sum":"x","dropped":2.0}"#, Mode::Summary, 0.0, false), r#"{"sum":"x"}"#);
    assert_eq!(stored(r#"{"sum":"x","dropped":-4}"#, Mode::Summary, 0.0, false), r#"{"sum":"x"}"#);
}

#[test]
fn what_cannot_be_read_is_refused_with_its_reason() {
    for text in [r#"{"steps":[{"t":"00:01","do":"a","keys":["Ctrl+A","Ctrl"#, "Sorry, no.", ""] {
        let fail = refused(text);
        assert_eq!(fail.kind, Kind::Service);
        assert_eq!(fail.message, "Gemini sent an answer that is not readable JSON");
    }
    let fail = refused("[1,2]");
    assert_eq!(fail.kind, Kind::Service);
    assert_eq!(fail.message, "Gemini sent JSON that is not an object");
}

#[test]
fn only_the_last_tries_are_searched_for_a_whole_end() {
    let entries = r#"{"t":"00:01","do":"a"},"#.repeat(MAX_SALVAGE_TRIES);
    let text = format!(r#"{{"steps":[{entries}{{"t":"00:02""#);
    let analysis = parse(&text, Mode::Steps, 0.0, false).expect("rescued");
    assert_eq!(analysis.steps.len(), MAX_SALVAGE_TRIES);
    assert!(analysis.cut);
}
