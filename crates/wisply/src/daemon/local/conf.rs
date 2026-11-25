use std::fs;
use std::path::Path;

use serde_json::{Map, Value};

use super::plan;
use super::{Kind, get, sys};
use crate::daemon::config;

pub struct Settings {
    pub ram_mb: u64,
    pub ram_set: bool,
    pub idle_s: u64,
    pub threads: usize,
    pub game_mode: String,
    pub start_mb: [u64; 2],
}

fn num(cfg: &Map<String, Value>, key: &str, fallback: i64) -> i64 {
    cfg.get(key).and_then(Value::as_i64).unwrap_or(fallback)
}

fn saved(home: &Path) -> Map<String, Value> {
    match fs::read_to_string(home.join("config.json")).map(|t| serde_json::from_str::<Value>(&t)) {
        Ok(Ok(Value::Object(map))) => map,
        _ => Map::new(),
    }
}

fn user_mb(saved: &Map<String, Value>, key: &str) -> Option<u64> {
    saved.get(key).and_then(Value::as_u64).filter(|mb| *mb > 0)
}

pub fn index(kind: Kind) -> usize {
    match kind {
        Kind::Llm => 0,
        Kind::Asr => 1,
    }
}

fn model_bytes(kind: Kind) -> u64 {
    get::table().iter().find(|i| i.part == kind && i.form == get::Form::File).map_or(0, |i| i.size)
}

fn start_of(saved: &Map<String, Value>, key: &str, kind: Kind) -> u64 {
    user_mb(saved, key).unwrap_or_else(|| plan::start_mb(kind, model_bytes(kind)))
}

pub fn settings(home: &Path) -> Settings {
    let cfg = config::load(home);
    let own = saved(home);
    let set = user_mb(&own, "local_ram_mb");
    Settings {
        ram_mb: plan::limit(set, sys::total_mb()),
        ram_set: set.is_some(),
        idle_s: u64::try_from(num(&cfg, "local_idle_s", 120)).unwrap_or(0),
        threads: plan::threads(num(&cfg, "local_threads", 0), sys::cores()),
        game_mode: config::text(&cfg, "local_game"),
        start_mb: [start_of(&own, "local_llm_start_mb", Kind::Llm), start_of(&own, "local_asr_start_mb", Kind::Asr)],
    }
}

pub fn limit_line(set: &Settings) -> String {
    let how = if set.ram_set { "set by local_ram_mb" } else { "automatic from the installed RAM" };
    format!(
        "limit: {} MB for both programs ({how}; change: wisply config local_ram_mb <MB>, 0 = automatic)",
        set.ram_mb
    )
}
