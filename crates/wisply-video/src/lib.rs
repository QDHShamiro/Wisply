mod asked;
pub mod captions;
pub mod config;
mod explain;
pub mod fail;
#[cfg(test)]
mod fixture;
pub mod guard;
mod info;
mod limits;
pub mod pacific;
pub mod process;
mod redact;
mod scratch;
pub mod source;
pub mod store;
pub mod store_transcript;
mod text;
pub mod timecode;
pub mod transcript;
pub mod youtube;

use std::path::Path;

use config::Config;
use fail::Fail;
use store::Store;
use youtube::YouTube;

pub const FOLDER: &str = "pupil";
pub const CONFIG_FILE: &str = "video.toml";
pub const STORE_FILE: &str = "video.db";
const TOOLS: &str = "bin";
const LIMITS: &str = "limits";
const LIMITS_USAGE: &str = "wisply pupil limits";

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
    if verb == info::VERB { info::run(&youtube, args) } else { text::run(&youtube, &config.transcript, args) }
}

pub fn run(home: &Path, args: &[String]) -> i32 {
    let rest = args.get(1..).unwrap_or_default();
    let done = match args.first().map(String::as_str) {
        Some(LIMITS) => limits::run(&home.join(FOLDER)),
        Some(verb) if verb == info::VERB || verb == text::VERB => video(home, verb, rest),
        _ => {
            for usage in [info::USAGE, text::USAGE, LIMITS_USAGE] {
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
