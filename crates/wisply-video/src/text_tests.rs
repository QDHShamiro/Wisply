use serde_json::json;

use super::*;
use crate::fail::Kind;
use crate::fixture::{self, Bench};

const LINK: &str = "https://www.youtube.com/watch?v=abcdefghijk";
const MANUAL: &str = r#"{"events": [
    {"tStartMs": 0, "dDurationMs": 4000, "segs": [{"utf8": "select the cube"}]},
    {"tStartMs": 30000, "dDurationMs": 4000, "segs": [{"utf8": "then bevel the edges"}]},
    {"tStartMs": 90000, "dDurationMs": 4000, "segs": [{"utf8": "bevelling twice looks smooth"}]},
    {"tStartMs": 150000, "dDurationMs": 4000, "segs": [{"utf8": "save the file"}]}]}"#;

fn bench(name: &str) -> Bench {
    let bench = Bench::new(name);
    bench.runner.gives(fixture::answered(&fixture::yt_dlp_data(json!({}))));
    bench.runner.writes("abcdefghijk.en.json3", MANUAL);
    bench
}

fn shown(bench: &Bench, words: &[&str]) -> Result<String, Fail> {
    let args: Vec<String> = std::iter::once(LINK).chain(words.iter().copied()).map(String::from).collect();
    run(&bench.youtube(), &bench.config.transcript, &args)
}

#[test]
fn every_line_is_shown_with_its_time() {
    let bench = bench("text-all");
    let lines = [
        "subtitles  en  4 lines",
        "00:00  select the cube",
        "00:30  then bevel the edges",
        "01:30  bevelling twice looks smooth",
        "02:30  save the file",
    ];
    assert_eq!(shown(&bench, &[]).unwrap(), lines.join("\n"));
    assert_eq!(shown(&bench, &[]).unwrap(), lines.join("\n"));
    assert_eq!(bench.runner.calls().len(), 2);
}

#[test]
fn a_part_of_the_video_can_be_read_alone() {
    let bench = bench("text-part");
    let part = shown(&bench, &["--start", "0:20", "--end", "1:40"]).unwrap();
    assert_eq!(part, "subtitles  en  2 of 4 lines\n00:30  then bevel the edges\n01:30  bevelling twice looks smooth");
    assert_eq!(shown(&bench, &["--start", "2:50"]).unwrap(), "no line in this part of the video");
    let late = shown(&bench, &["--start", "9:00"]).unwrap_err();
    assert_eq!(late.kind, Kind::Time);
    assert!(late.message.contains("past the end of the video"), "{}", late.message);
}

#[test]
fn a_wrong_time_is_refused_before_anything_is_fetched() {
    let bench = Bench::new("text-time");
    assert_eq!(shown(&bench, &["--start", "2:00", "--end", "1:00"]).unwrap_err().kind, Kind::Time);
    assert_eq!(shown(&bench, &["--start", "soon"]).unwrap_err().kind, Kind::Time);
    assert!(bench.runner.calls().is_empty());
}

#[test]
fn words_are_found_in_the_lines() {
    let bench = bench("text-find");
    let found = shown(&bench, &["--find", "Bevel"]).unwrap();
    assert_eq!(found, "subtitles  en  2 of 4 lines\n00:30  then bevel the edges\n01:30  bevelling twice looks smooth");
    let late = shown(&bench, &["--find", "bevel", "--start", "1:00"]).unwrap();
    assert_eq!(late, "subtitles  en  1 of 4 lines\n01:30  bevelling twice looks smooth");
    assert_eq!(shown(&bench, &["--find", "render"]).unwrap(), "no line matches \"render\"");
    assert_eq!(shown(&bench, &["--find", "cube", "--start", "1:00"]).unwrap(), "no line in this part matches \"cube\"");
}

#[test]
fn a_search_with_many_hits_says_that_it_shows_the_best_ones() {
    let mut bench = bench("text-best");
    bench.config.transcript.search_results = 2;
    let found = shown(&bench, &["--find", "the"]).unwrap();
    let lines: Vec<&str> = found.lines().collect();
    assert_eq!(lines[0], "subtitles  en  2 of 4 lines, the 2 best matches (--start and --end narrow it)");
    assert_eq!(lines.len(), 3);
    assert!(lines[1] < lines[2], "{found}");
    bench.config.transcript.search_results = 3;
    let exact = shown(&bench, &["--find", "the"]).unwrap();
    assert_eq!(exact.lines().next(), Some("subtitles  en  3 of 4 lines"));
}

#[test]
fn a_long_text_stops_and_names_the_next_call() {
    let mut bench = bench("text-long");
    bench.config.transcript.output_chars = 60;
    let lines = [
        "subtitles  en  4 lines",
        "00:00  select the cube",
        "00:30  then bevel the edges",
        "2 more lines from 01:30: wisply pupil captions https://www.youtube.com/watch?v=abcdefghijk --start 01:30",
    ];
    assert_eq!(shown(&bench, &[]).unwrap(), lines.join("\n"));
    let rest = shown(&bench, &["--start", "01:30", "--end", "2:40"]).unwrap();
    assert!(rest.starts_with("subtitles  en  2 of 4 lines\n01:30  bevelling twice looks smooth\n"), "{rest}");
    bench.config.transcript.output_chars = 1;
    let tiny = shown(&bench, &["--end", "2:40", "--find", "the"]).unwrap();
    let next = format!("2 more lines from 00:30: wisply pupil captions {LINK} --start 00:30 --end 2:40 --find \"the\"");
    assert_eq!(tiny, format!("subtitles  en  3 of 4 lines\n00:00  select the cube\n{next}"));
}

#[test]
fn a_video_without_sound_says_so() {
    let bench = Bench::new("text-silent");
    let silent = json!({"subtitles": {}, "automatic_captions": {}, "formats": [{"acodec": "none", "vcodec": "avc1"}]});
    bench.runner.gives(fixture::answered(&fixture::yt_dlp_data(silent)));
    assert_eq!(shown(&bench, &[]).unwrap(), "no text  the video has no sound");
}
