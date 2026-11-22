use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

use serde_json::json;

use super::*;
use crate::fixture::{self, Bench};
use crate::gemini::Sent;
use crate::reply;

const LINK: &str = "https://youtu.be/abcdefghijk";
const WATCHED: &str = "https://www.youtube.com/watch?v=abcdefghijk";
const BEST: &str = "gemini-3.8-flash";
const SECOND: &str = "gemini-3.6-flash";
const CUT_OFF: &str = "incomplete";
const NO_KEY: &str = "GEMINI_API_KEY is not set";
const SUBTITLE_FILE: &str = "abcdefghijk.en.json3";
const SPOKEN: &str = r#"{"events": [
    {"tStartMs": 0, "dDurationMs": 4000, "segs": [{"utf8": "select the cube"}]},
    {"tStartMs": 4000, "dDurationMs": 4000, "segs": [{"utf8": "then bevel it"}]},
    {"tStartMs": 60000, "dDurationMs": 4000, "segs": [{"utf8": "much later"}]}]}"#;
const HEADING: &str = "# Bevel tutorial (03:07) · steps · gemini-3.8-flash · ";
const STEPS: &str = "\n\n1. [00:05] Select the cube · keys `Tab`\n\
                     2. [00:40] Bevel the edge · menu `Edge > Bevel` · vals Width `0.2` · unsure\n   \
                     snd [00:41] click\n\nunsure at 00:40";
const CUT_NOTE: &str = "Note: Gemini's answer was cut off, later parts may be missing.";
const SILENT_NOTE: &str = "Note: No transcript, so quiet parts were not pointed out to Gemini: ";

#[derive(Default)]
struct Line {
    answers: RefCell<VecDeque<Result<Sent, String>>>,
    bodies: RefCell<Vec<Value>>,
}

struct Desk {
    bench: Bench,
    line: Line,
    opened: Cell<usize>,
    keyless: Cell<bool>,
}

impl Transport for Line {
    fn post(&self, body: &str, _seconds: f64) -> Result<Sent, String> {
        self.bodies.borrow_mut().push(serde_json::from_str(body).expect("body"));
        self.answers.borrow_mut().pop_front().unwrap_or_else(|| Err("no answer is left".to_string()))
    }
}

fn steps() -> Value {
    json!({
        "steps": [
            {"t": "00:05", "do": "Select the cube", "keys": ["Tab"]},
            {"t": "00:40", "do": "Bevel the edge", "menu": ["Edge", "Bevel"], "vals": [{"n": "Width", "v": "0.2"}],
                "unsure": true},
        ],
        "snd": [{"t": "00:41", "what": "click"}],
    })
}

fn summary() -> Value {
    json!({"sum": "A cube gets a bevel.", "pts": [{"t": "00:40", "x": "The bevel tool is opened."}], "snd": []})
}

impl Desk {
    fn of(name: &str, details: Value) -> Desk {
        let bench = Bench::new(name);
        bench.runner.gives(fixture::answered(&fixture::yt_dlp_data(details)));
        Desk { bench, line: Line::default(), opened: Cell::new(0), keyless: Cell::new(false) }
    }

    fn new(name: &str) -> Desk {
        let desk = Desk::of(name, json!({}));
        desk.bench.runner.writes(SUBTITLE_FILE, SPOKEN);
        desk
    }

    fn answers(&self, status: &str, data: &Value) {
        let interaction = json!({
            "id": "interaction-1",
            "status": status,
            "steps": [{"type": "model_output", "content": [{"type": "text", "text": data.to_string()}]}],
            "usage": {"total_tokens": 5000, "total_input_tokens": 4500, "total_output_tokens": 500},
        });
        let sent = Sent { status: 200, retry_after: None, text: interaction.to_string() };
        self.line.answers.borrow_mut().push_back(Ok(sent));
    }

    fn fails(&self, code: u16, status: &str, message: &str) {
        let body = json!({"error": {"code": code, "status": status, "message": message}});
        self.line.answers.borrow_mut().push_back(Ok(Sent { status: code, retry_after: None, text: body.to_string() }));
    }

    fn refuses(&self, message: &str) {
        self.fails(400, "INVALID_ARGUMENT", message);
    }

    fn watch(&self, args: &[&str]) -> Result<String, Fail> {
        let youtube = self.bench.youtube();
        let watcher = Watcher {
            youtube: &youtube,
            config: &self.bench.config,
            connect: &|| {
                if self.keyless.get() {
                    return Err(Fail::new(Kind::MissingKey, NO_KEY));
                }
                self.opened.set(self.opened.get() + 1);
                Ok(Box::new(&self.line))
            },
            jitter: &|| 1.0,
        };
        let args: Vec<String> = [LINK].iter().chain(args).map(|arg| arg.to_string()).collect();
        run(&watcher, &args)
    }

    fn sent(&self) -> Vec<Value> {
        self.line.bodies.borrow().clone()
    }

    fn prompt(&self, index: usize) -> String {
        self.sent()[index]["input"][1]["text"].as_str().expect("prompt").to_string()
    }

    fn today(&self, model: &str) -> String {
        let guard = Guard::new(&self.bench.store, &self.bench.config.limits, &self.bench.clock);
        let usage = guard.usage_today(model).expect("usage");
        format!("Note: Today {} of {} requests on {model} are used.", usage.used, usage.allowed)
    }
}

#[test]
fn a_video_is_watched_once_and_the_answer_is_kept() {
    let desk = Desk::new("watch-once");
    desk.answers(reply::COMPLETED, &steps());
    let first = desk.watch(&[]).expect("watched");
    assert_eq!(first, format!("{HEADING}1 request\n{}{STEPS}", desk.today(BEST)));
    assert!(desk.today(BEST).starts_with("Note: Today 1 of "), "{}", desk.today(BEST));
    let body = &desk.sent()[0];
    assert_eq!(body["model"], BEST);
    let video = json!({"type": "video", "uri": WATCHED, "resolution": "high",
        "processing": {"type": "static", "fps": 1.0}});
    assert_eq!(body["input"][0], video);
    assert_eq!(body["generation_config"]["max_output_tokens"], 8000);
    assert_eq!(body["response_format"]["schema"]["required"], json!(["steps", "snd"]));
    let prompt = desk.prompt(0);
    assert!(prompt.contains("Video: \"Bevel tutorial\" by Example Channel, length 03:07."), "{prompt}");
    assert!(prompt.contains("Nobody speaks at 00:08-01:00, 01:04-03:07."), "{prompt}");
    assert!(!prompt.contains("You see only the part"), "{prompt}");
    let again = desk.watch(&[]).expect("kept");
    assert_eq!(again, format!("{HEADING}cached{STEPS}"));
    assert_eq!((desk.sent().len(), desk.opened.get()), (1, 1));
    assert_eq!(desk.bench.runner.calls().len(), 2);
}

#[test]
fn a_part_is_sent_with_its_offsets_and_named_in_the_heading() {
    let desk = Desk::new("watch-part");
    desk.answers(reply::COMPLETED, &summary());
    let args = ["--start", "0:30", "--end", "1:35", "--mode", "summary", "--focus", " the  bevel tool. "];
    let text = desk.watch(&args).expect("watched");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines[0], "# Bevel tutorial (03:07) · 00:30-01:35 · summary · gemini-3.8-flash · 1 request");
    assert_eq!(lines[1], "Note: At 0.1 frames per second the sound list is only coarse.");
    assert!(text.ends_with("\n\nA cube gets a bevel.\nKey points:\n- [00:40] The bevel tool is opened."), "{text}");
    let video = &desk.sent()[0]["input"][0];
    assert_eq!(video["resolution"], "low");
    assert_eq!(video["processing"], json!({"type": "static", "start_offset": "30s", "end_offset": "95s", "fps": 0.1}));
    let prompt = desk.prompt(0);
    assert!(prompt.contains("You see only the part from 00:30 to 01:35."), "{prompt}");
    assert!(prompt.contains("Focus on: the bevel tool. Leave out what is unrelated to it."), "{prompt}");
    assert!(prompt.contains("Nobody speaks at 00:30-01:00, 01:04-01:35."), "{prompt}");
    desk.answers(reply::COMPLETED, &summary());
    let open = desk.watch(&["--start", "0:30", "--mode", "summary"]).expect("another part");
    assert!(open.starts_with("# Bevel tutorial (03:07) · 00:30-03:07 · summary · "), "{open}");
    assert_eq!(desk.sent()[1]["input"][0]["processing"]["end_offset"], "187s");
    assert_eq!(desk.sent().len(), 2);
}

#[test]
fn a_cut_answer_is_marked_and_stays_marked_when_read_again() {
    let desk = Desk::new("watch-cut");
    desk.answers(CUT_OFF, &steps());
    let first = desk.watch(&[]).expect("watched");
    assert!(first.contains(CUT_NOTE), "{first}");
    let again = desk.watch(&[]).expect("kept");
    assert!(again.starts_with(&format!("{HEADING}cached\n{CUT_NOTE}")), "{again}");
    assert_eq!(desk.sent().len(), 1);
}

#[test]
fn a_video_that_does_not_fit_is_refused_before_anything_is_sent() {
    let desk = Desk::of("watch-long", json!({"duration": 7200}));
    desk.bench.runner.writes(SUBTITLE_FILE, SPOKEN);
    let fail = desk.watch(&[]).expect_err("too long");
    assert_eq!(fail.kind, Kind::Input);
    assert!(fail.message.starts_with("2:00:00 of video does not fit in one request"), "{}", fail.message);
    assert!(fail.hint.contains("--mode summary"), "{}", fail.hint);
    assert_eq!((desk.sent().len(), desk.opened.get()), (0, 0));
}

#[test]
fn a_long_video_is_skimmed_whole_in_summary_mode() {
    let desk = Desk::of("watch-skim", json!({"duration": 10800}));
    desk.bench.runner.writes(SUBTITLE_FILE, SPOKEN);
    desk.answers(reply::COMPLETED, &summary());
    let text = desk.watch(&["--mode", "summary"]).expect("skimmed");
    assert!(text.starts_with("# Bevel tutorial (3:00:00) · summary · gemini-3.8-flash · 1 request\n"), "{text}");
    assert!(text.contains("Note: Gemini skimmed the video on its own (agentic)"), "{text}");
    let video = json!({"type": "video", "uri": WATCHED, "resolution": "low", "processing": "agentic"});
    assert_eq!(desk.sent()[0]["input"][0], video);
    assert_eq!(desk.sent()[0]["generation_config"]["max_output_tokens"], 3000);
}

#[test]
fn an_unknown_length_goes_out_whole_with_the_leanest_profile() {
    let desk = Desk::of("watch-unknown", json!({"duration": null}));
    desk.bench.runner.writes(SUBTITLE_FILE, SPOKEN);
    desk.answers(reply::COMPLETED, &steps());
    let text = desk.watch(&[]).expect("watched");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines[0], "# Bevel tutorial · steps · gemini-3.8-flash · 1 request");
    assert!(lines[1].starts_with("Note: The video length is unknown, so the whole video went in one request."));
    assert!(lines[2].starts_with("Note: Watched with the low_1 profile (low resolution, 1 frame per second)"));
    let video = &desk.sent()[0]["input"][0];
    assert_eq!((&video["resolution"], &video["processing"]), (&json!("low"), &json!({"type": "static", "fps": 1.0})));
    assert!(desk.prompt(0).contains("Nobody speaks at 00:08-01:00."), "{}", desk.prompt(0));
}

#[test]
fn a_video_without_text_is_watched_with_a_note() {
    let cases = [
        (json!({"subtitles": {}, "automatic_captions": {}}), "this video has no subtitles or captions."),
        (
            json!({"subtitles": {}, "automatic_captions": {}, "formats": [{"acodec": "none", "vcodec": "avc1"}]}),
            "the video has no sound.",
        ),
    ];
    for (details, reason) in cases {
        let desk = Desk::of("watch-silent", details);
        desk.answers(reply::COMPLETED, &steps());
        let text = desk.watch(&[]).expect("watched");
        assert_eq!(text.lines().nth(1), Some(format!("{SILENT_NOTE}{reason}").as_str()), "{text}");
        assert!(!desk.prompt(0).contains("Nobody speaks"), "{}", desk.prompt(0));
        assert!(text.ends_with(STEPS), "{text}");
    }
}

#[test]
fn the_key_is_asked_for_only_when_a_request_is_needed() {
    let desk = Desk::new("watch-key");
    desk.answers(reply::COMPLETED, &steps());
    desk.watch(&[]).expect("watched");
    desk.keyless.set(true);
    assert_eq!(desk.watch(&[]).expect("kept"), format!("{HEADING}cached{STEPS}"));
    let fail = desk.watch(&["--mode", "full"]).expect_err("no key");
    assert_eq!((fail.kind, fail.message.as_str()), (Kind::MissingKey, NO_KEY));
    assert_eq!(desk.sent().len(), 1);
}

#[test]
fn a_refused_request_is_reported_and_nothing_is_kept() {
    let desk = Desk::new("watch-refused");
    desk.refuses("The video cannot be read.");
    let fail = desk.watch(&[]).expect_err("refused");
    assert_eq!(fail.kind, Kind::Rejected);
    assert_eq!(fail.message, "Gemini refused the request (HTTP 400): The video cannot be read.");
    desk.answers(reply::COMPLETED, &steps());
    assert!(desk.watch(&[]).expect("watched").starts_with(&format!("{HEADING}1 request\n")));
    assert_eq!(desk.sent().len(), 2);
}

#[test]
fn a_busy_model_is_named_when_the_next_one_answers() {
    let desk = Desk::new("watch-busy");
    desk.fails(503, "UNAVAILABLE", "The model is overloaded.");
    desk.fails(503, "UNAVAILABLE", "The model is overloaded.");
    desk.answers(reply::COMPLETED, &steps());
    let text = desk.watch(&[]).expect("watched");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines[0], "# Bevel tutorial (03:07) · steps · gemini-3.6-flash · 3 requests");
    assert_eq!(lines[1], "Note: gemini-3.8-flash did not answer (HTTP 503), gemini-3.6-flash took over.");
    assert_eq!(lines[2], desk.today(SECOND));
    let asked: Vec<String> =
        desk.sent().iter().map(|body| body["model"].as_str().expect("model").to_string()).collect();
    assert_eq!(asked, [BEST, BEST, SECOND]);
    let again = desk.watch(&[]).expect("kept");
    assert_eq!(again, format!("# Bevel tutorial (03:07) · steps · gemini-3.6-flash · cached{STEPS}"));
}

#[test]
fn an_answer_that_cannot_be_read_is_an_error() {
    let desk = Desk::new("watch-unreadable");
    desk.answers(reply::COMPLETED, &json!("just words"));
    let fail = desk.watch(&[]).expect_err("unreadable");
    assert_eq!(fail.kind, Kind::Service);
    desk.answers(reply::COMPLETED, &steps());
    assert_eq!(desk.watch(&[]).expect("watched").lines().next(), Some(format!("{HEADING}1 request").as_str()));
}

#[test]
fn pages_after_the_first_come_from_the_kept_answer() {
    let mut desk = Desk::new("watch-pages");
    desk.bench.config.render.page_chars = 60;
    desk.answers(reply::COMPLETED, &steps());
    let first = desk.watch(&[]).expect("watched");
    assert!(first.ends_with("\n\n1. [00:05] Select the cube · keys `Tab`\n\nPage 1/2 · next: --page 2"), "{first}");
    let second = desk.watch(&["--page", "2"]).expect("second page");
    assert!(second.starts_with(&format!("{HEADING}cached\n\n2. [00:40] Bevel the edge")), "{second}");
    assert!(second.ends_with("\n\nPage 2/2 · unsure at 00:40"), "{second}");
    let fail = desk.watch(&["--page", "3"]).expect_err("no third page");
    assert_eq!(fail.message, "page 3 does not exist, this result has 2 page(s)");
    assert_eq!(fail.hint, "use --page 1 to 2");
    assert_eq!(desk.sent().len(), 1);
}

#[test]
fn bad_wishes_are_refused_before_the_video_is_looked_up() {
    let cases = [
        (vec!["--mode", "deep"], "mode 'deep' is unknown"),
        (vec!["--page", "0"], "page has to be a whole number from 1 up"),
        (vec!["--page", "two"], "page has to be a whole number from 1 up"),
        (vec!["--speed", "2"], "watch does not know --speed"),
        (vec!["--focus", "word ".repeat(100).leak().trim()], "focus is 499 characters long, at most 300 fit"),
    ];
    for (args, message) in cases {
        let desk = Desk::new("watch-wishes");
        let fail = desk.watch(&args).expect_err(message);
        assert_eq!((fail.kind, fail.message.as_str()), (Kind::Input, message));
        assert!(desk.bench.runner.calls().is_empty() && desk.sent().is_empty());
    }
    let desk = Desk::new("watch-wishes");
    let fail = desk.watch(&["--start", "0:30", "--end", "0:10"]).expect_err("backwards");
    assert_eq!((fail.kind, fail.message.as_str()), (Kind::Time, "end 00:10 is not after start 00:30"));
    assert!(desk.bench.runner.calls().is_empty() && desk.sent().is_empty());
}

#[test]
fn times_and_streams_are_checked_against_the_video() {
    let desk = Desk::new("watch-times");
    let fail = desk.watch(&["--start", "5:00"]).expect_err("past the end");
    assert_eq!(fail.kind, Kind::Time);
    assert!(fail.message.contains("past the end of the video (03:07)"), "{}", fail.message);
    let live = Desk::of("watch-live", json!({"live_status": "is_live", "duration": null}));
    let fail = live.watch(&[]).expect_err("live");
    assert_eq!(fail.kind, Kind::Unavailable);
    assert!(fail.message.contains("running live stream"), "{}", fail.message);
    assert!(desk.sent().is_empty() && live.sent().is_empty());
}
