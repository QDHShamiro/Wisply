mod apply;
mod auto;
mod release;
mod state;

use std::path::Path;
use std::process::ExitCode;

use serde_json::json;

pub use auto::{Gate, start};
pub use state::notice;

const USAGE: &str = "wisply update [--check | --rollback]   (config update auto|notify|off)";

fn say(text: impl AsRef<str>) -> ExitCode {
    println!("{}", text.as_ref());
    ExitCode::SUCCESS
}

fn err(text: impl AsRef<str>) -> ExitCode {
    println!("err: {}", text.as_ref());
    ExitCode::from(1)
}

pub fn run(home: &Path, args: &[String]) -> ExitCode {
    let flag = args.first().map(String::as_str).unwrap_or("");
    match flag {
        "--apply" => apply::helper(home, &args[1..]),
        "--rollback" => match apply::rollback(home) {
            Ok(done) => say(done),
            Err(why) => err(why),
        },
        "--check" | "" => now(home, flag.is_empty()),
        _ => err(format!("usage: {USAGE}")),
    }
}

fn now(home: &Path, install: bool) -> ExitCode {
    let found = match release::latest() {
        Ok(found) => found,
        Err(why) => return err(why),
    };
    state::set(home, "checked", json!(state::now()));
    state::set(home, "latest", json!(found.version));
    let have = state::VERSION;
    let mode = state::mode(home);
    if !release::newer(have, &found.version) {
        return say(format!("wisply {have} is current (updates: {mode})"));
    }
    if !install {
        return say(format!(
            "wisply {} is available, you run {have}: wisply update installs it (updates: {mode})",
            found.version
        ));
    }
    let root = match release::stage(home, &found) {
        Ok(root) => root,
        Err(why) => return err(format!("{why} -> try: wisply update again")),
    };
    match apply::launch(home, &root, have, &found.version) {
        Ok(()) => say(format!(
            "wisply {} downloaded and checked (SHA-256), installing in the background; the daemon restarts, `wisply update --rollback` undoes it",
            found.version
        )),
        Err(why) => err(why),
    }
}
