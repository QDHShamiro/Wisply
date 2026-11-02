use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use serde_json::{Map, Value, json};

use super::tokens_scan::{Cost, TokenScan};

const FILE: &str = "doctor-tokens-cache.json";
const VERSION: u64 = 1;

pub struct Cache {
    path: Option<PathBuf>,
    entries: Map<String, Value>,
    dirty: bool,
}

fn counts(map: &HashMap<String, usize>) -> Value {
    Value::Object(map.iter().map(|(k, v)| (k.clone(), json!(v))).collect())
}

fn read_counts(v: Option<&Value>) -> HashMap<String, usize> {
    v.and_then(Value::as_object)
        .into_iter()
        .flatten()
        .filter_map(|(k, n)| Some((k.clone(), usize::try_from(n.as_u64()?).ok()?)))
        .collect()
}

fn encode(one: &TokenScan) -> Value {
    let cost: Map<String, Value> =
        one.mcp_cost.iter().map(|(k, c)| (k.clone(), json!([c.tool_chars, c.instruction_chars]))).collect();
    let legacy: Map<String, Value> =
        one.legacy_runs.iter().map(|(k, (runs, chars))| (k.clone(), json!([runs, chars]))).collect();
    json!({
        "requests": one.requests,
        "mcp_used": counts(&one.mcp_used),
        "skills_used": counts(&one.skills_used),
        "skill_cost": counts(&one.skill_cost),
        "mcp_cost": cost,
        "legacy": legacy,
    })
}

fn pair(v: &Value) -> Option<(usize, usize)> {
    let list = v.as_array()?;
    Some((usize::try_from(list.first()?.as_u64()?).ok()?, usize::try_from(list.get(1)?.as_u64()?).ok()?))
}

fn decode(v: &Value) -> Option<TokenScan> {
    let mut one = TokenScan { requests: usize::try_from(v.get("requests")?.as_u64()?).ok()?, ..TokenScan::default() };
    one.mcp_used = read_counts(v.get("mcp_used"));
    one.skills_used = read_counts(v.get("skills_used"));
    one.skill_cost = read_counts(v.get("skill_cost"));
    for (k, c) in v.get("mcp_cost")?.as_object()? {
        let (tool_chars, instruction_chars) = pair(c)?;
        one.mcp_cost.insert(k.clone(), Cost { tool_chars, instruction_chars });
    }
    for (k, c) in v.get("legacy")?.as_object()? {
        one.legacy_runs.insert(k.clone(), pair(c)?);
    }
    Some(one)
}

fn stamp(path: &Path) -> Option<(u64, u64)> {
    let meta = fs::metadata(path).ok()?;
    let secs = meta.modified().ok()?.duration_since(UNIX_EPOCH).ok()?.as_secs();
    Some((secs, meta.len()))
}

impl Cache {
    pub fn open(dir: Option<&Path>) -> Self {
        let path = dir.map(|d| d.join(FILE));
        let stored = path
            .as_ref()
            .and_then(|p| fs::read_to_string(p).ok())
            .and_then(|t| serde_json::from_str::<Value>(&t).ok())
            .filter(|v| v.get("version").and_then(Value::as_u64) == Some(VERSION));
        let entries = stored.and_then(|v| v.get("files").and_then(Value::as_object).cloned()).unwrap_or_default();
        Cache { path, entries, dirty: false }
    }

    pub fn get(&self, file: &Path) -> Option<TokenScan> {
        let (secs, len) = stamp(file)?;
        let entry = self.entries.get(file.to_string_lossy().as_ref())?;
        if entry.get("secs")?.as_u64()? != secs || entry.get("len")?.as_u64()? != len {
            return None;
        }
        decode(entry.get("data")?)
    }

    pub fn put(&mut self, file: &Path, one: &TokenScan) {
        let Some((secs, len)) = stamp(file) else { return };
        let entry = json!({"secs": secs, "len": len, "data": encode(one)});
        self.entries.insert(file.to_string_lossy().into_owned(), entry);
        self.dirty = true;
    }

    pub fn keep_only(&mut self, files: &[PathBuf]) {
        let keep: Vec<String> = files.iter().map(|f| f.to_string_lossy().into_owned()).collect();
        let before = self.entries.len();
        self.entries.retain(|k, _| keep.contains(k));
        self.dirty |= self.entries.len() != before;
    }

    pub fn save(&self) {
        let Some(path) = self.path.as_ref().filter(|_| self.dirty) else { return };
        let body = json!({"version": VERSION, "files": self.entries});
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(path, body.to_string());
    }
}
