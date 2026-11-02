use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};

use serde_json::Value;

use super::price;

const TEXT_KEEP: usize = 600;

pub fn claude_home() -> PathBuf {
    std::env::var_os("CLAUDE_CONFIG_DIR")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .or_else(|| std::env::home_dir().map(|h| h.join(".claude")))
        .unwrap_or_default()
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Stats {
    pub tokens: i64,
    pub cost: f64,
    pub last_text: String,
    pub limit: Option<String>,
    pub ended_ms: i64,
    pub model: String,
    pub effort: String,
}

#[derive(Clone, Default)]
struct Usage {
    input: i64,
    write: i64,
    read: i64,
    out: i64,
    model: String,
}

#[derive(Default)]
pub struct Parser {
    ids: HashMap<String, Usage>,
    last_text: String,
    limit: Option<String>,
    ended_ms: i64,
    model: String,
    effort: String,
    offset: u64,
    len: u64,
}

fn text_of(content: &Value) -> String {
    match content {
        Value::String(s) => s.clone(),
        Value::Array(blocks) => blocks
            .iter()
            .filter(|b| b["type"] == "text")
            .filter_map(|b| b["text"].as_str())
            .collect::<Vec<_>>()
            .join(" "),
        _ => String::new(),
    }
}

fn is_limit(text: &str) -> bool {
    let low = text.to_lowercase();
    low.contains("limit reached") || (low.contains("hit your") && low.contains("limit"))
}

fn millis(stamp: &str) -> i64 {
    let flat: String = stamp.chars().take(19).collect::<String>().replace('T', " ");
    wisply_core::stamp_secs(&flat).map_or(0, |s| s * 1000)
}

impl Parser {
    pub fn feed(&mut self, chunk: &str) {
        for line in chunk.lines() {
            if line.contains(r#""effort":""#) && !line.contains(r#""type":"assistant""#) {
                if let Some(effort) =
                    serde_json::from_str::<Value>(line).ok().and_then(|e| e["effort"].as_str().map(String::from))
                {
                    self.effort = effort;
                }
                continue;
            }
            if !line.contains(r#""type":"assistant""#) && !line.contains(r#""type":"user""#) {
                continue;
            }
            let Ok(e) = serde_json::from_str::<Value>(line) else { continue };
            if e["isSidechain"].as_bool().unwrap_or(false) {
                continue;
            }
            if let Some(ts) = e["timestamp"].as_str() {
                self.ended_ms = self.ended_ms.max(millis(ts));
            }
            let m = &e["message"];
            let text = text_of(&m["content"]);
            match e["type"].as_str() {
                Some("assistant") if e["isApiErrorMessage"].as_bool().unwrap_or(false) => {
                    if is_limit(&text) {
                        self.limit = Some(text);
                    }
                }
                Some("assistant") => {
                    self.limit = None;
                    self.take_usage(&e, m);
                    if !text.trim().is_empty() {
                        self.last_text = text.chars().take(TEXT_KEEP).collect();
                    }
                }
                Some("user") => self.limit = None,
                _ => {}
            }
        }
    }

    fn take_usage(&mut self, e: &Value, m: &Value) {
        let u = &m["usage"];
        if !u.is_object() {
            return;
        }
        let n = |k: &str| u[k].as_i64().unwrap_or(0);
        let key = m["id"].as_str().or_else(|| e["uuid"].as_str()).unwrap_or("").to_string();
        let slot = self.ids.entry(key).or_default();
        slot.input = slot.input.max(n("input_tokens"));
        slot.write = slot.write.max(n("cache_creation_input_tokens"));
        slot.read = slot.read.max(n("cache_read_input_tokens"));
        slot.out = slot.out.max(n("output_tokens"));
        if let Some(model) = m["model"].as_str().filter(|s| !s.is_empty()) {
            slot.model = model.to_string();
            self.model = model.to_string();
        }
    }

    pub fn stats(&self) -> Stats {
        let (mut tokens, mut cost) = (0, 0.0);
        for u in self.ids.values() {
            tokens += u.input + u.write + u.out;
            cost += price::cost(&u.model, u.input, u.write, u.read, u.out);
        }
        Stats {
            tokens,
            cost,
            last_text: self.last_text.clone(),
            limit: self.limit.clone(),
            ended_ms: self.ended_ms,
            model: self.model.clone(),
            effort: self.effort.clone(),
        }
    }

    pub fn refresh(&mut self, path: &Path) -> bool {
        let Ok(mut file) = File::open(path) else { return false };
        let len = file.metadata().map_or(0, |m| m.len());
        if len < self.len {
            *self = Parser::default();
        }
        self.len = len;
        if len == self.offset || file.seek(SeekFrom::Start(self.offset)).is_err() {
            return false;
        }
        let mut raw = Vec::new();
        if file.read_to_end(&mut raw).is_err() {
            return false;
        }
        let whole = raw.iter().rposition(|b| *b == b'\n').map_or(0, |i| i + 1);
        self.feed(&String::from_utf8_lossy(&raw[..whole]));
        self.offset += whole as u64;
        true
    }
}

#[derive(Default)]
pub struct Cache {
    paths: HashMap<String, PathBuf>,
    parsers: HashMap<String, Parser>,
}

impl Cache {
    fn locate(&mut self, root: &Path, sid: &str) -> Option<PathBuf> {
        if let Some(p) = self.paths.get(sid).filter(|p| p.is_file()) {
            return Some(p.clone());
        }
        let name = format!("{sid}.jsonl");
        let found =
            std::fs::read_dir(root).ok()?.filter_map(Result::ok).map(|d| d.path().join(&name)).find(|p| p.is_file())?;
        self.paths.insert(sid.to_string(), found.clone());
        Some(found)
    }

    pub fn stats(&mut self, root: &Path, sid: &str) -> Stats {
        if sid.is_empty() || !sid.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-') {
            return Stats::default();
        }
        let Some(path) = self.locate(root, sid) else { return Stats::default() };
        let parser = self.parsers.entry(sid.to_string()).or_default();
        parser.refresh(&path);
        parser.stats()
    }
}

static CACHE: Mutex<Option<Cache>> = Mutex::new(None);

pub fn stats_of(sid: &str) -> Stats {
    let root = claude_home().join("projects");
    let mut guard = CACHE.lock().unwrap_or_else(PoisonError::into_inner);
    guard.get_or_insert_with(Cache::default).stats(&root, sid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMIT: &str = r#"{"type":"assistant","timestamp":"2026-11-02T01:00:00.000Z","isApiErrorMessage":true,"message":{"content":[{"type":"text","text":"You've hit your session limit · resets 3:40am (Europe/Berlin)"}]}}"#;
    const ANSWER: &str = r#"{"type":"assistant","timestamp":"2026-11-02T00:59:00.000Z","message":{"id":"m1","model":"claude-sonnet-5-5","usage":{"input_tokens":10,"cache_creation_input_tokens":100,"cache_read_input_tokens":1000,"output_tokens":50},"content":[{"type":"text","text":"building now"}]}}"#;
    const USER: &str = r#"{"type":"user","timestamp":"2026-11-02T03:00:00.000Z","message":{"content":"weiter"}}"#;
    const SIDE: &str = r#"{"type":"user","isSidechain":true,"message":{}}"#;

    #[test]
    fn numbers_text_and_an_open_limit_are_read() {
        let mut p = Parser::default();
        p.feed(&format!("{ANSWER}\n{ANSWER}\n{LIMIT}\n"));
        let s = p.stats();
        assert_eq!(s.tokens, 160);
        assert!(s.cost > 0.0);
        assert_eq!(s.last_text, "building now");
        assert!(s.limit.as_deref().is_some_and(|l| l.contains("resets 3:40am")));
        assert!(s.ended_ms > 0);
    }

    #[test]
    fn a_later_message_closes_the_limit() {
        let mut p = Parser::default();
        p.feed(&format!("{ANSWER}\n{LIMIT}\n{USER}\n"));
        assert_eq!(p.stats().limit, None);
    }

    #[test]
    fn sidechains_and_noise_are_skipped_and_files_grow_in_steps() {
        let dir = std::env::temp_dir().join(format!("wisply-agents-tr-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("s.jsonl");
        std::fs::write(&file, format!("{ANSWER}\n{SIDE}\nnot json\n{LIMIT}")).unwrap();
        let mut p = Parser::default();
        assert!(p.refresh(&file));
        assert_eq!(p.stats().limit, None);
        std::fs::write(&file, format!("{ANSWER}\n{SIDE}\nnot json\n{LIMIT}\n")).unwrap();
        assert!(p.refresh(&file));
        assert!(p.stats().limit.is_some());
        assert!(!p.refresh(&file));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
