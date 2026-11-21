pub mod config;
pub mod fail;
pub mod guard;
mod limits;
pub mod pacific;
pub mod source;
pub mod store;
pub mod timecode;

use std::path::Path;

pub const CONFIG_FILE: &str = "video.toml";
pub const STORE_FILE: &str = "video.db";
const USAGE: &str = "wisply pupil limits";

pub fn run(folder: &Path, args: &[String]) -> i32 {
    let done = match args.first().map(String::as_str) {
        Some("limits") => limits::run(folder),
        _ => {
            eprintln!("{USAGE}");
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
