use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::{Value, json};

use crate::config::Config;
use crate::fail::Fail;
use crate::guard::Clock;
use crate::process::{Ran, Runner};
use crate::source;
use crate::store::Store;
use crate::youtube::{self, Video, VideoInfo, YouTube};

pub const VIDEO_ID: &str = "abcdefghijk";
pub const START_TIME: f64 = 1_768_507_200.0;
pub const COMMAND: &str = "yt-dlp-for-tests";
const DESCRIPTION_CHARS: usize = 10;
const OUTPUT_FLAG: &str = "-o";
const DETAILS_FLAG: &str = "-J";

static NEXT: AtomicU64 = AtomicU64::new(0);

type Step = Box<dyn Fn(&[String]) -> Result<Ran, Fail>>;

pub struct Moving(Cell<f64>);

#[derive(Default)]
pub struct FakeRunner {
    steps: RefCell<VecDeque<Step>>,
    calls: RefCell<Vec<Vec<String>>>,
}

pub struct Bench {
    pub config: Config,
    pub store: Store,
    pub runner: FakeRunner,
    pub clock: Moving,
    pub folder: PathBuf,
}

pub fn folder(name: &str) -> PathBuf {
    let unique = format!("wisply-video-{name}-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed));
    let path = std::env::temp_dir().join(unique);
    std::fs::create_dir_all(&path).expect("folder");
    path
}

pub fn yt_dlp_data(changes: Value) -> Value {
    let mut data = json!({
        "id": VIDEO_ID,
        "title": "Bevel tutorial",
        "channel": "Example Channel",
        "duration": 187,
        "live_status": "not_live",
        "availability": "public",
        "width": 1920,
        "height": 1080,
        "language": "en",
        "chapters": [
            {"start_time": 0, "end_time": 30, "title": "Intro"},
            {"start_time": 30, "end_time": null, "title": " Bevel "},
        ],
        "subtitles": {"en": [], "live_chat": []},
        "automatic_captions": {"en-orig": [], "en": [], "de": []},
        "description": "d".repeat(5000),
        "formats": [
            {"format_id": "140", "acodec": "mp4a.40.2", "vcodec": "none"},
            {"format_id": "614", "acodec": "none", "vcodec": "vp09.00.40.08"},
        ],
    });
    if let (Some(base), Value::Object(extra)) = (data.as_object_mut(), changes) {
        base.extend(extra);
    }
    data
}

pub fn info(changes: Value) -> VideoInfo {
    youtube::info_from_json(&yt_dlp_data(changes), VIDEO_ID, DESCRIPTION_CHARS)
}

pub fn video() -> Video {
    let source = source::parse(&format!("https://youtu.be/{VIDEO_ID}")).expect("link");
    Video::of(&source).expect("video")
}

pub fn done() -> Ran {
    Ran { code: 0, stdout: Vec::new(), stderr: String::new() }
}

pub fn answered(data: &Value) -> Ran {
    Ran { stdout: serde_json::to_vec(data).expect("json"), ..done() }
}

pub fn failed(stderr: &str) -> Ran {
    Ran { code: 1, stdout: Vec::new(), stderr: stderr.to_string() }
}

pub fn value_after(argv: &[String], flag: &str) -> Option<String> {
    argv.iter().position(|arg| arg == flag).and_then(|at| argv.get(at + 1)).cloned()
}

impl Moving {
    pub fn advance(&self, seconds: f64) {
        self.0.set(self.0.get() + seconds);
    }
}

impl Clock for Moving {
    fn now(&self) -> f64 {
        self.0.get()
    }

    fn sleep(&self, seconds: f64) {
        self.advance(seconds);
    }
}

impl FakeRunner {
    pub fn then(&self, step: impl Fn(&[String]) -> Result<Ran, Fail> + 'static) {
        self.steps.borrow_mut().push_back(Box::new(step));
    }

    pub fn gives(&self, ran: Ran) {
        self.then(move |_| Ok(ran.clone()));
    }

    pub fn writes(&self, name: &'static str, content: &str) {
        let content = content.to_string();
        self.then(move |argv| {
            let target = value_after(argv, OUTPUT_FLAG).expect("output");
            let folder = Path::new(&target).parent().expect("folder").to_path_buf();
            std::fs::write(folder.join(name), &content).expect("write");
            Ok(done())
        });
    }

    pub fn calls(&self) -> Vec<Vec<String>> {
        self.calls.borrow().clone()
    }
}

impl Runner for FakeRunner {
    fn run(&self, argv: &[String], _limit_seconds: f64, _label: &str) -> Result<Ran, Fail> {
        self.calls.borrow_mut().push(argv.to_vec());
        let step = self.steps.borrow_mut().pop_front();
        match step {
            Some(step) => step(argv),
            None if argv.iter().any(|arg| arg == DETAILS_FLAG) => Ok(answered(&yt_dlp_data(json!({})))),
            None => Ok(done()),
        }
    }
}

impl Bench {
    pub fn new(name: &str) -> Bench {
        let mut config = Config::standard();
        config.youtube.command = vec![COMMAND.to_string()];
        Bench {
            config,
            store: Store::in_memory(),
            runner: FakeRunner::default(),
            clock: Moving(Cell::new(START_TIME)),
            folder: folder(name),
        }
    }

    pub fn youtube(&self) -> YouTube<'_> {
        YouTube {
            settings: &self.config.youtube,
            store: &self.store,
            runner: &self.runner,
            clock: &self.clock,
            tools: &self.folder,
            folder: &self.folder,
        }
    }
}

impl Drop for Bench {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.folder);
    }
}
