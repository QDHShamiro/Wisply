use std::fs;
use std::path::PathBuf;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde_json::{Value, json};

use super::{Log, Record, Tape, keep, on, strip};
use crate::config::Trace;
use crate::media::Source;
use crate::talk::{Mode, Out};

#[derive(Default)]
struct Sink {
    notes: Vec<String>,
    words: String,
    thoughts: String,
    ended: bool,
}

impl Out for Sink {
    fn note(&mut self, line: &str) {
        self.notes.push(line.to_string());
    }

    fn word(&mut self, piece: &str) -> bool {
        self.words.push_str(piece);
        true
    }

    fn thought(&mut self, piece: &str) -> bool {
        self.thoughts.push_str(piece);
        true
    }

    fn end(&mut self) {
        self.ended = true;
    }
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pupil-trace-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

fn spec(dir: PathBuf, frames: bool, media_max_mb: u64) -> Trace {
    Trace { dir, frames, media_max_mb, marker: "MARK".to_string() }
}

fn record(sent: Option<Value>, sources: Vec<Source>, error: Option<Value>) -> Record {
    Record {
        said: "Was passiert?".to_string(),
        sources,
        mode: Mode { think: true, ..Mode::default() },
        model: "m".to_string(),
        config: PathBuf::from("chat.toml"),
        log: Log { sent, think: Some(true), ..Log::default() },
        told: None,
        error,
    }
}

fn picture(bytes: &[u8]) -> Value {
    let url = format!("data:image/jpeg;base64,{}", STANDARD.encode(bytes));
    json!({"type": "image_url", "image_url": {"url": url}})
}

fn rows(dir: &std::path::Path) -> Vec<Value> {
    let file = fs::read_dir(dir)
        .expect("trace dir")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| path.extension().is_some_and(|ext| ext == "jsonl"))
        .expect("day file");
    fs::read_to_string(file)
        .expect("read day file")
        .lines()
        .map(|line| serde_json::from_str(line).expect("json line"))
        .collect()
}

#[test]
fn the_tape_keeps_everything_and_still_passes_it_on() {
    let mut sink = Sink::default();
    let mut tape = Tape::new(&mut sink);
    tape.note("Lade Video");
    tape.thought("erst ");
    tape.thought("denken");
    tape.word("Ant");
    tape.word("wort");
    tape.end();
    let taped = tape.done();
    assert_eq!(taped.steps.len(), 1);
    assert_eq!(taped.steps[0].1, "Lade Video");
    assert_eq!(taped.thought, "erst denken");
    assert_eq!(taped.answer, "Antwort");
    let thought = taped.first_thought.expect("first thought");
    let word = taped.first_word.expect("first word");
    assert!(thought <= word && word <= taped.total);
    assert_eq!(sink.notes, ["Lade Video"]);
    assert_eq!(sink.thoughts, "erst denken");
    assert_eq!(sink.words, "Antwort");
    assert!(sink.ended);
}

#[test]
fn pictures_leave_the_request_and_land_as_files() {
    let dir = scratch("strip");
    let mut sent = json!({"messages": [
        {"role": "system", "content": "s"},
        {"role": "user", "content": [picture(b"old")]},
        {"role": "assistant", "content": "a"},
        {"role": "user", "content": [
            {"type": "text", "text": "[00:01]"},
            picture(b"one"),
            {"type": "input_audio", "input_audio": {"data": STANDARD.encode(b"wav"), "format": "wav"}},
            picture(b"two")
        ]}
    ]});
    let saved = strip(&mut sent, Some(&dir), "MARK");
    assert_eq!(saved, ["000.jpeg", "001.wav", "002.jpeg"]);
    assert_eq!(fs::read(dir.join("000.jpeg")).expect("first"), b"one");
    assert_eq!(fs::read(dir.join("001.wav")).expect("sound"), b"wav");
    assert_eq!(fs::read(dir.join("002.jpeg")).expect("second"), b"two");
    assert_eq!(sent["messages"][1]["content"][0]["file"], "MARK");
    assert_eq!(sent["messages"][3]["content"][0]["text"], "[00:01]");
    assert_eq!(sent["messages"][3]["content"][1]["file"], "000.jpeg");
    assert_eq!(sent["messages"][3]["content"][1]["type"], "image_url");
    assert!(!sent.to_string().contains(&STANDARD.encode(b"one")));
    let mut bare = sent.clone();
    let mut other = json!({"messages": [{"role": "user", "content": [picture(b"x")]}]});
    assert!(strip(&mut other, None, "MARK").is_empty());
    assert_eq!(other["messages"][0]["content"][0]["file"], "MARK");
    assert!(strip(&mut bare, None, "MARK").is_empty());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_turn_becomes_one_line_with_frames_and_small_media() {
    let dir = scratch("keep");
    let small = dir.join("small.mp4");
    let big = dir.join("big.mp4");
    fs::write(&small, b"tiny").expect("small media");
    fs::write(&big, vec![0u8; 1024 * 1024 + 1]).expect("big media");
    let trace = spec(dir.join("trace"), true, 1);
    assert!(on(&trace));
    assert!(!on(&spec(PathBuf::new(), true, 1)));
    let sent = json!({"messages": [{"role": "user", "content": [picture(b"frame")]}]});
    let mut sink = Sink::default();
    let mut tape = Tape::new(&mut sink);
    tape.note("Schritt");
    tape.word("Ja");
    let taped = tape.done();
    let mut first =
        record(Some(sent), vec![Source::File(small.clone()), Source::Link("https://x.test/v".to_string())], None);
    first.log.media = vec![small.clone(), big.clone()];
    let file = keep(&trace, &taped, first).expect("day file");
    assert!(file.starts_with(&trace.dir));
    let failure = Some(json!({"code": "no_frames", "message": "m", "hint": "h"}));
    keep(&trace, &taped, record(None, Vec::new(), failure)).expect("second line");
    let rows = rows(&trace.dir);
    assert_eq!(rows.len(), 2);
    let row = &rows[0];
    assert_eq!(row["question"], "Was passiert?");
    assert_eq!(row["answer"], "Ja");
    assert_eq!(row["think"], true);
    assert_eq!(row["mode"]["think"], true);
    assert_eq!(row["steps"][0]["line"], "Schritt");
    assert_eq!(row["sources"][1], "https://x.test/v");
    assert_eq!(row["frames"][0], "000.jpeg");
    assert_eq!(row["media"].as_array().expect("media").len(), 2);
    assert_eq!(row["media"][0]["kept"], "media-0.mp4");
    assert_eq!(row["media"][1]["kept"], Value::Null);
    let id = row["id"].as_str().expect("id");
    let kept = trace.dir.join(id);
    assert_eq!(fs::read(kept.join("000.jpeg")).expect("frame"), b"frame");
    assert_eq!(fs::read(kept.join("media-0.mp4")).expect("copy"), b"tiny");
    assert!(!kept.join("media-1.mp4").exists());
    assert_eq!(rows[1]["error"]["code"], "no_frames");
    assert_eq!(rows[1]["told"], Value::Null);
    assert!(!trace.dir.join(rows[1]["id"].as_str().expect("id")).exists());
    let _ = fs::remove_dir_all(&dir);
}
