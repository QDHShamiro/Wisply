use std::path::Path;
use std::time::{Duration, Instant};

use serde::Deserialize;
use serde_json::Value;

use crate::config::{Config, Model, Talk};
use crate::game::system;
use crate::job::Failure;
use crate::server::{self, Files, State};
use crate::watch;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Status {
    label: String,
    state: String,
    updated: f64,
}

pub fn timing_test(talk: &Talk, raw: &str, now: f64) -> Option<String> {
    let (from, to) = (raw.find('{')?, raw.rfind('}')?);
    let status: Status = serde_json::from_str(raw.get(from..=to)?).ok()?;
    let fresh = now - status.updated <= talk.status_stale_seconds;
    let calm = talk.calm_labels.iter().any(|label| status.label.starts_with(label.as_str()));
    (status.state == talk.status_run && fresh && !calm).then_some(status.label)
}

pub fn owner_test(talk: &Talk, line: &str) -> Option<String> {
    let calm = |name: &String| !name.is_empty() && line.contains(name.as_str());
    if talk.calm_owners.iter().any(calm) {
        return None;
    }
    let script = line
        .split_whitespace()
        .find(|word| word.contains(['/', '\\']))
        .and_then(|word| word.rsplit(['/', '\\']).next())
        .filter(|name| !name.is_empty());
    Some(script.unwrap_or(line.trim()).to_string())
}

fn owner(talk: &Talk) -> Option<String> {
    if talk.calm_owners.is_empty() {
        return None;
    }
    talk.hold_files.iter().find_map(|path| std::fs::read_to_string(path).ok())
}

pub fn held(talk: &Talk) -> Option<String> {
    if let Some(line) = owner(talk) {
        return owner_test(talk, &line);
    }
    let raw = std::fs::read_to_string(&talk.status_file).ok()?;
    timing_test(talk, &raw, server::now())
}

pub fn test(talk: &Talk) -> Option<Value> {
    let raw = std::fs::read_to_string(&talk.status_file).ok()?;
    let (from, to) = (raw.find('{')?, raw.rfind('}')?);
    let value: Value = serde_json::from_str(raw.get(from..=to)?).ok()?;
    let state = value.get("state").and_then(Value::as_str)?;
    let updated = value.get("updated").and_then(Value::as_f64)?;
    let fresh = server::now() - updated <= talk.status_stale_seconds;
    (state == talk.status_run && fresh).then_some(value)
}

fn taken(talk: &Talk) -> bool {
    talk.hold_files.iter().any(|path| path.exists())
}

fn running(files: &Files) -> Option<State> {
    server::read_state(files).filter(|state| state.keeper != 0 && system::alive(state.keeper))
}

fn busy(config: &Config) -> Failure {
    Failure::new(config, "talk_busy", &[])
}

pub fn attach(config: &Config, config_path: &Path, model: &Model, afk: bool, anyway: bool) -> Result<(), Failure> {
    let files = Files::new(config);
    if let Some(failure) = watch::game_failure(config, afk) {
        return Err(failure);
    }
    if server::healthy(config) {
        server::touch_used(&files);
        return Ok(());
    }
    if running(&files).is_some() {
        let deadline = Instant::now() + Duration::from_secs(config.server.start_timeout_seconds);
        let poll = Duration::from_millis(config.server.health_poll_ms);
        while Instant::now() < deadline && running(&files).is_some() {
            if server::healthy(config) {
                server::touch_used(&files);
                return Ok(());
            }
            std::thread::sleep(poll);
        }
        if running(&files).is_some() {
            return Err(busy(config));
        }
    }
    if taken(&config.talk) && !anyway {
        return Err(busy(config));
    }
    server::set_afk(config, afk);
    server::ensure(config, config_path, model).map_err(|e| watch::start_failure(config, afk, e))
}

pub fn own_server(config: &Config, model: &Model, anyway: bool) -> Result<(), Failure> {
    let files = Files::new(config);
    let ours =
        running(&files).is_none_or(|state| server::keeper_alive(&state) && server::same_server(&state, config, model));
    let free = anyway || !taken(&config.talk);
    if ours && free { Ok(()) } else { Err(busy(config)) }
}

#[cfg(test)]
mod tests {
    use super::{owner_test, timing_test};
    use crate::config::Config;

    fn config() -> Config {
        toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses")
    }

    fn status(label: &str, state: &str, updated: f64) -> String {
        format!(
            "window.PUPIL_STATUS = {{\"label\": \"{label}\", \"state\": \"{state}\", \"updated\": {updated}, \"case\": 3}};\n"
        )
    }

    #[test]
    fn only_a_fresh_running_timing_test_holds_the_chat() {
        let mut talk = config().talk;
        talk.status_run = "run".to_string();
        talk.status_stale_seconds = 900.0;
        talk.calm_labels = vec!["know".to_string(), "vmme".to_string()];
        let now = 10_000.0;
        let held = |label: &str, state: &str, updated: f64| timing_test(&talk, &status(label, state, updated), now);
        assert_eq!(held("write-wrap", "run", now - 5.0).as_deref(), Some("write-wrap"));
        assert_eq!(held("know-start", "run", now - 5.0), None);
        assert_eq!(held("vmme-turn", "run", now - 5.0), None);
        assert_eq!(held("write-wrap", "done", now - 5.0), None);
        assert_eq!(held("write-wrap", "run", now - 901.0), None);
        assert_eq!(timing_test(&talk, "", now), None);
        assert_eq!(timing_test(&talk, "window.PUPIL_STATUS = nope;", now), None);
    }

    #[test]
    fn the_lock_owner_decides_when_calm_owners_are_named() {
        let mut talk = config().talk;
        talk.calm_owners = vec!["pupil-know.sh".to_string(), "pupil-vmme".to_string()];
        let line = |script: &str| format!("pupil bash /c/work/{script} know-wrap /e/x.toml 00:53:47 pid 17\n");
        assert_eq!(owner_test(&talk, &line("pupil-know.sh")), None);
        assert_eq!(owner_test(&talk, &line("pupil-vmmelong.sh")), None);
        assert_eq!(owner_test(&talk, &line("pupil-write.sh")).as_deref(), Some("pupil-write.sh"));
        assert_eq!(owner_test(&talk, "wisply build\n").as_deref(), Some("wisply build"));
        talk.calm_owners = vec![String::new()];
        assert!(owner_test(&talk, &line("pupil-know.sh")).is_some());
    }
}
