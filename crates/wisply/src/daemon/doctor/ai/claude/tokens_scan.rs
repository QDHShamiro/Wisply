use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

use super::Env;
use super::tokens_cache::Cache;
use super::transcripts::mcp_server;
use super::usage::normalized;

pub const WINDOW_DAYS: u64 = 14;
const DAY: u64 = 86_400;
const MAX_LINE: usize = 3_000_000;
const COMMAND_OPEN: &str = "<command-name>";
const COMMAND_CLOSE: &str = "</command-name>";
const REQUEST_KEY: &str = "\"requestId\":\"";
const NAME_MAX: usize = 80;
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

fn recent(env: &Env, now: u64) -> Vec<PathBuf> {
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
                files.push(path);
            }
        }
    }
    files
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

fn attachment(a: &Value, scan: &mut TokenScan, file: &mut HashMap<String, Cost>) {
    match a.get("type").and_then(Value::as_str) {
        Some("deferred_tools_delta") => {
            for name in strings(a, "addedNames").filter(|n| n.starts_with("mcp__")) {
                if let Some(server) = mcp_server(name) {
                    file.entry(server.to_string()).or_default().tool_chars += name.chars().count() + 1;
                }
            }
        }
        Some("mcp_instructions_delta") => {
            for (name, block) in strings(a, "addedNames").zip(strings(a, "addedBlocks")) {
                file.entry(normalized(name)).or_default().instruction_chars += block.chars().count();
            }
        }
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
    let mut requests: HashSet<String> = HashSet::new();
    let mut costs: HashMap<String, Cost> = HashMap::new();
    for line in BufReader::new(file).lines().map_while(Result::ok) {
        if line.len() > MAX_LINE {
            continue;
        }
        if line.contains("\"type\":\"assistant\"")
            && let Some(at) = line.find(REQUEST_KEY)
        {
            let id = &line[at + REQUEST_KEY.len()..];
            requests.insert(id.split('"').next().unwrap_or("").to_string());
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
                    attachment(a, scan, &mut costs);
                }
            }
            _ => {}
        }
    }
    scan.requests += requests.len();
    keep_max(&mut scan.mcp_cost, costs);
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
    let files = recent(env, now);
    let mut cache = Cache::open(cache_dir);
    let mut scan = TokenScan { sessions: files.len(), ..TokenScan::default() };
    for path in &files {
        let one = cache.get(path).unwrap_or_else(|| {
            let mut fresh = TokenScan::default();
            read_file(path, &mut fresh);
            cache.put(path, &fresh);
            fresh
        });
        scan.absorb(one);
    }
    cache.keep_only(&files);
    cache.save();
    scan
}
