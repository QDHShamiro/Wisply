use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use serde_json::json;

use super::{apply, plugins, release, state};
use crate::setup::place::exe_name;

const FIRST: Duration = Duration::from_secs(180);
const EVERY: Duration = Duration::from_secs(60);
pub const CHECK_EVERY: u64 = 3_600;
pub const FORCE_AFTER: u64 = 21_600;

pub struct Gate {
    pub game: Box<dyn Fn() -> bool + Send>,
    pub idle: Box<dyn Fn() -> bool + Send>,
    pub free: Box<dyn Fn() -> bool + Send>,
}

fn runs_from_bin(home: &Path) -> bool {
    let me = std::env::current_exe().and_then(fs::canonicalize).ok();
    let bin = fs::canonicalize(home.join("bin")).ok();
    matches!((me.as_deref().and_then(Path::parent), bin), (Some(a), Some(b)) if a == b)
}

pub fn managed(home: &Path) -> bool {
    wisply_proto::is_default_home(home) && std::env::var_os("WISPLY_NO_UPDATE").is_none() && runs_from_bin(home)
}

pub fn start(home: PathBuf, gate: Gate) {
    if !managed(&home) {
        return;
    }
    let work = move || {
        thread::sleep(FIRST);
        let mut first = true;
        loop {
            tick(&home, &gate, first);
            first = false;
            thread::sleep(EVERY);
        }
    };
    let _ = thread::Builder::new().name("update".into()).spawn(work);
}

pub fn may_apply(waited: u64, idle: bool, free: bool) -> bool {
    idle || (free && waited >= FORCE_AFTER)
}

fn staged(home: &Path) -> Option<(String, PathBuf, u64)> {
    let s = state::load(home);
    let version = s["staged"]["version"].as_str()?.to_string();
    let root = PathBuf::from(s["staged"]["root"].as_str()?);
    let at = s["staged"]["at"].as_u64().unwrap_or(0);
    (release::newer(state::VERSION, &version) && root.join(exe_name("wisply")).is_file()).then_some((version, root, at))
}

fn go(home: &Path, version: &str, root: &Path) {
    if let Err(e) = apply::launch(home, root, state::VERSION, version) {
        state::finished(home, state::VERSION, version, false, &e);
    }
}

fn tick(home: &Path, gate: &Gate, first: bool) {
    let mode = state::mode(home);
    if mode == "off" || (gate.game)() {
        return;
    }
    let waiting = if mode == "auto" { staged(home) } else { None };
    if let Some((version, root, at)) = &waiting
        && may_apply(state::now().saturating_sub(*at), (gate.idle)(), (gate.free)())
    {
        return go(home, version, root);
    }
    let seen = state::load(home);
    let due = state::now().saturating_sub(seen["checked"].as_u64().unwrap_or(0)) >= CHECK_EVERY;
    if mode == "auto" && waiting.is_none() && (first || due) {
        plugins::catch_up(home, false);
    }
    if due {
        check(home, gate, mode == "auto", waiting.map(|(version, _, _)| version));
    }
}

fn check(home: &Path, gate: &Gate, auto: bool, waiting: Option<String>) {
    state::set(home, "checked", json!(state::now()));
    let Ok(found) = release::latest() else { return };
    state::set(home, "latest", json!(found.version));
    if !auto
        || !release::newer(state::VERSION, &found.version)
        || state::gave_up(&state::load(home), &found.version)
        || waiting.as_deref() == Some(found.version.as_str())
    {
        return;
    }
    match release::stage(home, &found) {
        Ok(root) => {
            state::staged(home, &found.version, &root);
            if (gate.idle)() && !(gate.game)() {
                go(home, &found.version, &root);
            }
        }
        Err(e) => state::finished(home, state::VERSION, &found.version, false, &e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_update_waits_for_quiet_and_after_six_hours_only_for_a_free_moment() {
        assert!(may_apply(0, true, true));
        assert!(may_apply(60, true, false));
        assert!(!may_apply(60, false, true));
        assert!(!may_apply(FORCE_AFTER - 1, false, true));
        assert!(may_apply(FORCE_AFTER, false, true));
        assert!(!may_apply(FORCE_AFTER * 4, false, false));
    }
}
