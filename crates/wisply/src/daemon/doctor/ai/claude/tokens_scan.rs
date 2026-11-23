use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

use super::Env;
use super::tokens_cache::Cache;
use super::transcripts::mcp_server;
use super::usage::normalized;
use crate::daemon::doctor::ai::grade::{Tally, week};

pub const WINDOW_DAYS: u64 = 14;
const DAY: u64 = 86_400;
const MAX_LINE: usize = 3_000_000;
const COMMAND_OPEN: &str = "<command-name>";
const COMMAND_CLOSE: &str = "</command-name>";
const REQUEST_KEY: &str = "\"requestId\":\"";
const NAME_MAX: usize = 80;
const TOOL_USE: &str = "\"type\":\"tool_use\"";
const IS_ERROR: &str = "\"is_error\":true";
const USAGE_KEY: &str = "\"usage\":{";
const USAGE_PARTS: [&str; 4] =
    ["\"input_tokens\":", "\"cache_creation_input_tokens\":", "\"cache_read_input_tokens\":", "\"output_tokens\":"];
pub const LEGACY: [&str; 4] = ["claude-mem", "claude_mem", "context-engine", "graphify"];

#[derive(Default)]
pub struct Cost {
    pub tool_chars: usize,
    pub instruction_chars: usize,
}

#[derive(Default)]
pub struct TokenScan {
    pub sessions: usize,
    pub requests: usize,
    pub mcp_used: HashMap<String, usize>,
    pub skills_used: HashMap<String, usize>,
    pub mcp_cost: HashMap<String, Cost>,
    pub skill_cost: HashMap<String, usize>,
    pub legacy_runs: HashMap<String, (usize, usize)>,
    pub tally: Tally,
    pub weeks: [Tally; 2],
}

impl TokenScan {
    pub fn per_day(&self, tokens: usize) -> usize {
        tokens * self.sessions / WINDOW_DAYS as usize
    }

    pub fn used_skill(&self, name: &str) -> bool {
        let tail = name.rsplit(':').next().unwrap_or(name);
        self.skills_used.keys().any(|u| u == name || u.rsplit(':').next() == Some(tail))
    }
}

fn recent(env: &Env, now: u64) -> Vec<(PathBuf, u64)> {
    let mut files = Vec::new();
    for dir in fs::read_dir(env.claude.join("projects")).into_iter().flatten().flatten() {
        for entry in fs::read_dir(dir.path()).into_iter().flatten().flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|e| e != "jsonl") {
                continue;
            }
            let secs = entry
                .metadata()
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map_or(0, |d| d.as_secs());
            if now.saturating_sub(secs) <= WINDOW_DAYS * DAY {
                files.push((path, secs));
            }
        }
    }
    files
}

fn usage(line: &str) -> u64 {
    let Some(at) = line.find(USAGE_KEY) else { return 0 };
    let rest = &line[at..];
    USAGE_PARTS
        .iter()
        .filter_map(|key| {
            let from = rest.find(key)? + key.len();
            rest[from..].chars().take_while(char::is_ascii_digit).collect::<String>().parse::<u64>().ok()
        })
        .sum()
}

fn failures(line: &str, tally: &mut Tally) {
    let Ok(entry) = serde_json::from_str::<Value>(line) else { return };
    if entry.get("toolDenialKind").and_then(Value::as_str).is_some_and(|k| !k.is_empty()) {
        return;
    }
    let Some(blocks) = entry.pointer("/message/content").and_then(Value::as_array) else { return };
    for block in blocks.iter().filter(|b| b.get("is_error").and_then(Value::as_bool) == Some(true)) {
        let text = match block.get("content") {
            Some(Value::String(s)) => s.clone(),
            Some(Value::Array(parts)) => {
                parts.iter().filter_map(|p| p.get("text").and_then(Value::as_str)).collect::<Vec<_>>().join("\n")
            }
            _ => String::new(),
        };
        tally.fail(&text);
    }
}

fn tool_uses(entry: &Value, scan: &mut TokenScan) {
    let Some(blocks) = entry.get("message").and_then(|m| m.get("content")).and_then(Value::as_array) else { return };
    for block in blocks.iter().filter(|b| b.get("type").and_then(Value::as_str) == Some("tool_use")) {
        let name = block.get("name").and_then(Value::as_str).unwrap_or("");
        if let Some(server) = mcp_server(name) {
            *scan.mcp_used.entry(server.to_string()).or_default() += 1;
        }
        if name == "Skill"
            && let Some(skill) = block.get("input").and_then(|i| i.get("skill")).and_then(Value::as_str)
        {
            *scan.skills_used.entry(skill.to_string()).or_default() += 1;
        }
    }
}

fn strings<'a>(a: &'a Value, key: &str) -> impl Iterator<Item = &'a str> {
    a.get(key).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str)
}

fn keep_max(map: &mut HashMap<String, Cost>, file: HashMap<String, Cost>) {
    for (server, cost) in file {
        let slot = map.entry(server).or_default();
        slot.tool_chars = slot.tool_chars.max(cost.tool_chars);
        slot.instruction_chars = slot.instruction_chars.max(cost.instruction_chars);
    }
}

fn listing(a: &Value, scan: &mut TokenScan) {
    let Some(content) = a.get("content").and_then(Value::as_str) else { return };
    let mut found: HashMap<String, usize> = HashMap::new();
    for line in content.lines() {
        let Some(rest) = line.strip_prefix("- ") else { continue };
        let name = rest.split(": ").next().unwrap_or(rest);
        found.insert(name.to_string(), line.chars().count() + 1);
    }
    if found.len() >= scan.skill_cost.len() {
        scan.skill_cost = found;
    }
}

fn command_names(line: &str, scan: &mut TokenScan) {
    let mut rest = line;
    while let Some(start) = rest.find(COMMAND_OPEN) {
        rest = &rest[start + COMMAND_OPEN.len()..];
        let Some(end) = rest.find(COMMAND_CLOSE) else { return };
        let name = rest[..end].trim().trim_start_matches('/');
        if !name.is_empty() && name.len() < NAME_MAX && !name.contains(char::is_whitespace) {
            *scan.skills_used.entry(name.to_string()).or_default() += 1;
        }
        rest = &rest[end..];
    }
}

fn legacy_hook(a: &Value, scan: &mut TokenScan) {
    let command = a.get("command").and_then(Value::as_str).unwrap_or("");
    let Some(tag) = LEGACY.iter().find(|t| command.contains(*t)) else { return };
    let out = a.get("stdout").and_then(Value::as_str).unwrap_or("").chars().count();
    let slot = scan.legacy_runs.entry((*tag).to_string()).or_default();
    slot.0 += 1;
    slot.1 += out;
}

#[derive(Default)]
struct Live {
    tools: HashMap<String, HashMap<String, usize>>,
    notes: HashMap<String, usize>,
    peak: HashMap<String, Cost>,
}

impl Live {
    fn tools(&mut self, a: &Value) {
        for name in strings(a, "removedNames") {
            if let Some(server) = mcp_server(name)
                && let Some(set) = self.tools.get_mut(server)
            {
                set.remove(name);
            }
        }
        for name in strings(a, "addedNames") {
            if let Some(server) = mcp_server(name) {
                self.tools.entry(server.to_string()).or_default().insert(name.to_string(), name.chars().count() + 1);
            }
        }
        for (server, set) in &self.tools {
            let slot = self.peak.entry(server.clone()).or_default();
            slot.tool_chars = slot.tool_chars.max(set.values().sum());
        }
    }

    fn notes(&mut self, a: &Value) {
        for name in strings(a, "removedNames") {
            self.notes.remove(&normalized(name));
        }
        for (name, block) in strings(a, "addedNames").zip(strings(a, "addedBlocks")) {
            self.notes.insert(normalized(name), block.chars().count());
        }
        for (server, chars) in &self.notes {
            let slot = self.peak.entry(server.clone()).or_default();
            slot.instruction_chars = slot.instruction_chars.max(*chars);
        }
    }
}

fn attachment(a: &Value, scan: &mut TokenScan, live: &mut Live) {
    match a.get("type").and_then(Value::as_str) {
        Some("deferred_tools_delta") => live.tools(a),
        Some("mcp_instructions_delta") => live.notes(a),
        Some("skill_listing") => listing(a, scan),
        Some("hook_success") => legacy_hook(a, scan),
        _ => {}
    }
}

fn wanted(line: &str) -> bool {
    (line.contains("tool_use") && (line.contains("mcp__") || line.contains("\"Skill\"")))
        || line.contains("deferred_tools_delta")
        || line.contains("mcp_instructions_delta")
        || line.contains("skill_listing")
        || (line.contains("hook_success") && LEGACY.iter().any(|t| line.contains(t)))
}

fn read_file(path: &PathBuf, scan: &mut TokenScan) {
    let Ok(file) = File::open(path) else { return };
    let mut requests: HashMap<String, u64> = HashMap::new();
    let mut live = Live::default();
    for line in BufReader::new(file).lines().map_while(Result::ok) {
        if line.len() > MAX_LINE {
            continue;
        }
        if line.contains("\"type\":\"assistant\"") {
            scan.tally.calls += line.matches(TOOL_USE).count();
            if let Some(at) = line.find(REQUEST_KEY) {
                let id = &line[at + REQUEST_KEY.len()..];
                let slot = requests.entry(id.split('"').next().unwrap_or("").to_string()).or_default();
                *slot = (*slot).max(usage(&line));
            }
        }
        if line.contains(IS_ERROR) && line.contains("\"type\":\"user\"") {
            failures(&line, &mut scan.tally);
        }
        if line.contains(COMMAND_OPEN) {
            command_names(&line, scan);
        }
        if !wanted(&line) {
            continue;
        }
        let Ok(entry) = serde_json::from_str::<Value>(&line) else { continue };
        match entry.get("type").and_then(Value::as_str) {
            Some("assistant") => tool_uses(&entry, scan),
            Some("attachment") => {
                if let Some(a) = entry.get("attachment") {
                    attachment(a, scan, &mut live);
                }
            }
            _ => {}
        }
    }
    scan.requests += requests.len();
    scan.tally.tokens = requests.values().sum();
    scan.tally.sessions = usize::from(!requests.is_empty());
    keep_max(&mut scan.mcp_cost, live.peak);
}

impl TokenScan {
    fn absorb(&mut self, one: TokenScan) {
        self.requests += one.requests;
        for (k, n) in one.mcp_used {
            *self.mcp_used.entry(k).or_default() += n;
        }
        for (k, n) in one.skills_used {
            *self.skills_used.entry(k).or_default() += n;
        }
        keep_max(&mut self.mcp_cost, one.mcp_cost);
        if one.skill_cost.len() >= self.skill_cost.len() {
            self.skill_cost = one.skill_cost;
        }
        for (k, (runs, chars)) in one.legacy_runs {
            let slot = self.legacy_runs.entry(k).or_default();
            slot.0 += runs;
            slot.1 += chars;
        }
    }
}

pub fn scan(env: &Env) -> TokenScan {
    scan_with(env, Some(&wisply_proto::home()))
}

pub fn scan_with(env: &Env, cache_dir: Option<&Path>) -> TokenScan {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let dated = recent(env, now);
    let files: Vec<PathBuf> = dated.iter().map(|(p, _)| p.clone()).collect();
    let mut cache = Cache::open(cache_dir);
    let mut scan = TokenScan { sessions: files.len(), ..TokenScan::default() };
    for (path, secs) in &dated {
        let one = cache.get(path).unwrap_or_else(|| {
            let mut fresh = TokenScan::default();
            read_file(path, &mut fresh);
            cache.put(path, &fresh);
            fresh
        });
        if let Some(w) = week(now, *secs) {
            scan.weeks[w].add(&one.tally);
        }
        scan.absorb(one);
    }
    cache.keep_only(&files);
    cache.save();
    scan
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connector_cost_is_the_peak_not_the_sum_of_repeats() {
        let path = std::env::temp_dir().join(format!("wisply-peak-{}.jsonl", std::process::id()));
        let tools = |added: &str, removed: &str| {
            format!(
                r#"{{"type":"attachment","attachment":{{"type":"deferred_tools_delta","addedNames":[{added}],"removedNames":[{removed}]}}}}"#
            )
        };
        let notes = r#"{"type":"attachment","attachment":{"type":"mcp_instructions_delta","addedNames":["srv"],"addedBlocks":["0123456789"]}}"#;
        let both = r#""mcp__srv__a","mcp__srv__bb""#;
        let lines = [
            tools(both, ""),
            notes.to_string(),
            tools(both, ""),
            notes.to_string(),
            tools("", r#""mcp__srv__bb""#),
            tools(r#""mcp__srv__a""#, ""),
        ];
        fs::write(&path, lines.join("\n")).unwrap();
        let mut scan = TokenScan::default();
        read_file(&path, &mut scan);
        let cost = &scan.mcp_cost["srv"];
        assert_eq!((cost.tool_chars, cost.instruction_chars), (25, 10));
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn a_session_counts_calls_errors_and_tokens_once_per_request() {
        let path = std::env::temp_dir().join(format!("wisply-tally-{}.jsonl", std::process::id()));
        let usage = r#""usage":{"input_tokens":10,"cache_creation_input_tokens":100,"cache_read_input_tokens":1000,"cache_creation":{"ephemeral_5m_input_tokens":100},"output_tokens":5}"#;
        let lines = [
            format!(r#"{{"type":"assistant","requestId":"r1","message":{{"content":[{{"type":"tool_use","id":"a","name":"Bash"}}],{usage}}}}}"#),
            format!(r#"{{"type":"assistant","requestId":"r1","message":{{"content":[{{"type":"tool_use","id":"b","name":"Read"}}],{usage}}}}}"#),
            r#"{"type":"user","message":{"content":[{"type":"tool_result","tool_use_id":"a","is_error":true,"content":"Exit code 1\nfatal: not a git repository"}]}}"#.to_string(),
            r#"{"type":"user","toolDenialKind":"automode-blocked","message":{"content":[{"type":"tool_result","tool_use_id":"b","is_error":true,"content":"no"}]}}"#.to_string(),
            format!(r#"{{"type":"assistant","requestId":"r2","message":{{"content":[{{"type":"text","text":"done"}}],{usage}}}}}"#),
        ];
        fs::write(&path, lines.join("\n")).unwrap();
        let mut scan = TokenScan::default();
        read_file(&path, &mut scan);
        let t = &scan.tally;
        assert_eq!((t.sessions, t.calls, t.errors, t.tokens), (1, 2, 1, 2230));
        assert_eq!(t.fails["fatal: not a git repository"], 1);
        let _ = fs::remove_file(&path);
    }
}
