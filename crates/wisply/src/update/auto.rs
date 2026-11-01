use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use serde_json::json;

use super::{apply, release, state};
use crate::setup::place::exe_name;

const FIRST: Duration = Duration::from_secs(180);
const EVERY: Duration = Duration::from_secs(600);
const CHECK_EVERY: u64 = 86_400;

pub struct Gate {
    pub game: Box<dyn Fn() -> bool + Send>,
    pub idle: Box<dyn Fn() -> bool + Send>,
}

fn runs_from_bin(home: &Path) -> bool {
    let me = std::env::current_exe().and_then(fs::canonicalize).ok();
    let bin = fs::canonicalize(home.join("bin")).ok();
    matches!((me.as_deref().and_then(Path::parent), bin), (Some(a), Some(b)) if a == b)
}

pub fn start(home: PathBuf, gate: Gate) {
    let managed =
        wisply_proto::is_default_home(&home) && std::env::var_os("WISPLY_NO_UPDATE").is_none() && runs_from_bin(&home);
    if !managed {
        return;
    }
    let work = move || {
        thread::sleep(FIRST);
        loop {
            tick(&home, &gate);
            thread::sleep(EVERY);
        }
    };
    let _ = thread::Builder::new().name("update".into()).spawn(work);
}

fn staged(home: &Path) -> Option<(String, PathBuf)> {
    let s = state::load(home);
    let version = s["staged"]["version"].as_str()?.to_string();
    let root = PathBuf::from(s["staged"]["root"].as_str()?);
    (release::newer(state::VERSION, &version) && root.join(exe_name("wisply")).is_file()).then_some((version, root))
}

fn go(home: &Path, version: &str, root: &Path) {
    if let Err(e) = apply::launch(home, root, state::VERSION, version) {
        state::finished(home, state::VERSION, version, false, &e);
    }
}

fn tick(home: &Path, gate: &Gate) {
    let mode = state::mode(home);
    if mode == "off" || (gate.game)() {
        return;
    }
    if mode == "auto"
        && (gate.idle)()
        && let Some((version, root)) = staged(home)
    {
        return go(home, &version, &root);
    }
    let seen = state::load(home);
    if state::now().saturating_sub(seen["checked"].as_u64().unwrap_or(0)) < CHECK_EVERY {
        return;
    }
    state::set(home, "checked", json!(state::now()));
    let Ok(found) = release::latest() else { return };
    state::set(home, "latest", json!(found.version));
    if mode != "auto"
        || !release::newer(state::VERSION, &found.version)
        || state::gave_up(&state::load(home), &found.version)
    {
        return;
    }
    match release::stage(home, &found) {
        Ok(root) => {
            state::set(home, "staged", json!({"version": found.version, "root": root.to_string_lossy()}));
            if (gate.idle)() && !(gate.game)() {
                go(home, &found.version, &root);
            }
        }
        Err(e) => state::finished(home, state::VERSION, &found.version, false, &e),
    }
}
