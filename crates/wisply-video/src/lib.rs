pub mod analysis;
mod asked;
pub mod bodies;
pub mod captions;
pub mod config;
mod explain;
pub mod fail;
#[cfg(test)]
mod fixture;
pub mod gemini;
pub mod guard;
mod info;
mod key;
mod limits;
pub mod pacific;
pub mod planner;
pub mod process;
pub mod prompt;
mod redact;
pub mod render;
pub mod reply;
pub mod schema;
mod scratch;
pub mod source;
pub mod store;
pub mod store_transcript;
mod text;
pub mod timecode;
pub mod transcript;
pub mod trouble;
mod watch;
pub mod youtube;

use std::path::Path;

use config::Config;
use fail::Fail;
use gemini::Http;
use store::Store;
use watch::Watcher;
use youtube::YouTube;

pub const FOLDER: &str = "pupil";
pub const CONFIG_FILE: &str = "video.toml";
pub const STORE_FILE: &str = "video.db";
const TOOLS: &str = "bin";
const LIMITS: &str = "limits";
const LIMITS_USAGE: &str = "wisply pupil limits";
const VERBS: [&str; 3] = [info::VERB, text::VERB, watch::VERB];

fn video(home: &Path, verb: &str, args: &[String]) -> Result<String, Fail> {
    let (folder, tools) = (home.join(FOLDER), home.join(TOOLS));
    let config = Config::load(&folder.join(CONFIG_FILE))?;
    let store = Store::open(&folder.join(STORE_FILE), config.store.busy_timeout_seconds)?;
    let youtube = YouTube {
        settings: &config.youtube,
        store: &store,
        runner: &process::System,
        clock: &guard::System,
        tools: &tools,
        folder: &folder,
    };
    if verb == info::VERB {
        return info::run(&youtube, args);
    }
    if verb == text::VERB {
        return text::run(&youtube, &config.transcript, args);
    }
    let watcher = Watcher {
        youtube: &youtube,
        config: &config,
        connect: &|| Ok(Box::new(Http::new(key::read()?))),
        jitter: &gemini::jitter,
    };
    watch::run(&watcher, args)
}

pub fn run(home: &Path, args: &[String]) -> i32 {
    let rest = args.get(1..).unwrap_or_default();
    let done = match args.first().map(String::as_str) {
        Some(LIMITS) => limits::run(&home.join(FOLDER)),
        Some(verb) if VERBS.contains(&verb) => video(home, verb, rest),
        _ => {
            for usage in [info::USAGE, text::USAGE, watch::USAGE, LIMITS_USAGE] {
                eprintln!("{usage}");
            }
            return 2;
        }
    };
    match done {
        Ok(text) => {
            println!("{text}");
            0
        }
        Err(fail) => {
            println!("{}", fail.text());
            1
        }
    }
}
