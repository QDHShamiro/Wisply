use regex::Regex;
use serde_json::{Map, Value, json};
use std::cmp::Reverse;
use std::fs::{self, OpenOptions};
use std::io::{self, IsTerminal, Read, Write};
use std::panic;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use wisply_proto::home;
use wisply_squeeze::{SHORT_CHARS, Squeezed, apply, commas, extract, squeeze, squeeze_tool};

const MIN_CHARS: usize = SHORT_CHARS;
const TEXT_TOOLS: [&str; 3] = ["Grep", "WebSearch", "WebFetch"];
const INLINE_MAX: usize = 400_000;
const EDIT_TOOLS: [&str; 4] = ["Edit", "Write", "MultiEdit", "NotebookEdit"];
const MIX: u64 = 0x9e37_79b9_7f4a_7c15;
const BUDGET: Duration = Duration::from_millis(50);
const HOLDOUT_EVERY: u64 = 10;
const ERRORS_MAX: usize = 3;
const ERRORS_WINDOW_S: u64 = 600;
const KEEP_FOR: Duration = Duration::from_secs(7 * 86_400);
const KEEP_BYTES: u64 = 200_000_000;
const TIDY_EVERY: Duration = Duration::from_secs(3600);
const LOG_MAX: u64 = 16_000_000;
const INPUT_MAX: u64 = 64 * 1024 * 1024;
const INPUT_WAIT: Duration = Duration::from_millis(2500);
const FNV_START: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0100_0000_01b3;
const SEGMENT: &str = r"(?:^|[;\n(]|&&|\|\|)\s*(?:(?:then|do|else)\s+)?(?:rtk\s+)?";
const GIT: &str = r"git(?:\s+(?:-C\s+\S+|-c\s+\S+|--no-pager))*\s+";

fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("hook pattern")
}

static FILE_READ: LazyLock<Regex> = LazyLock::new(|| re(&format!(r"{SEGMENT}(?:cat|head|tail|sed\s+-n)\b([^|;&\n]*)")));
static DIFF: LazyLock<Regex> = LazyLock::new(|| re(&format!(r"{SEGMENT}(?:{GIT}(?:diff|show)|diff)\b")));
static LISTING: LazyLock<Regex> = LazyLock::new(|| {
    re(&format!(
        r"{SEGMENT}(?:ls|dir|tree|find|fd|{GIT}(?:log|ls-files|status|blame|rev-list|show-ref|branch|tag)|rg\s+(?:--files|-l)|grep\s+-[A-Za-z]*l)\b"
    ))
});
static OWN: LazyLock<Regex> = LazyLock::new(|| re(&format!(r"{SEGMENT}(?:\S*[/\\])?(?:wisply|lighting)(?:\.exe)?\b")));
static LOGISH: LazyLock<Regex> =
    LazyLock::new(|| re(r"(?i)\.(?:log|jsonl|out|output|txt|csv)\b|/logs?/|results\.md|tasks/"));
static KEPT: LazyLock<Regex> = LazyLock::new(|| re(r"(?:headroom[/\\]+out[/\\]+|\bmore\s+)(hr-[0-9a-f]{10})"));

struct Paths {
    state: PathBuf,
    out: PathBuf,
}

impl Paths {
    fn new(root: &Path) -> Self {
        let state = root.join("headroom");
        Paths { out: state.join("out"), state }
    }
}

pub fn post() {
    let started = Instant::now();
    panic::set_hook(Box::new(|_| {}));
    let root = home();
    if io::stdin().is_terminal() {
        return;
    }
    let Some(input) = take_input(io::stdin(), INPUT_MAX, INPUT_WAIT) else { return };
    let parsed = serde_json::from_slice::<Value>(&input);
    let paths = Paths::new(&root);
    if let Ok(event) = &parsed
        && event["hook_event_name"].as_str().is_some_and(|n| n != "PostToolUse")
    {
        if let Some(text) = crate::reads::other_event(&paths.state, event) {
            let mut out = io::stdout().lock();
            let _ = out.write_all(text.as_bytes());
            let _ = out.flush();
        }
        return;
    }
    let editing = parsed.as_ref().is_ok_and(|e| e["tool_name"].as_str().is_some_and(|t| EDIT_TOOLS.contains(&t)));
    if editing && let Ok(event) = &parsed {
        crate::reads::forget(&paths.state, event);
    }
    if editing && let Some(text) = wisply_think::after_edit(&root, &input) {
        let mut out = io::stdout().lock();
        let _ = out.write_all(text.as_bytes());
        let _ = out.flush();
        return;
    }
    if paths.state.join("off").exists() {
        return;
    }
    let event = parsed.map_err(|e| format!("hook input is not JSON ({e})"));
    let (reply, kept, entry) = run_event(event, &paths, started, BUDGET);
    if let Some(text) = reply {
        let mut out = io::stdout().lock();
        let _ = out.write_all(text.as_bytes());
        let _ = out.flush();
    }
    if let Some(entry) = entry {
        note(&paths, &entry);
    }
    if kept {
        tidy(&paths);
    }
}

fn take_input(source: impl Read + Send + 'static, max: u64, wait: Duration) -> Option<Vec<u8>> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut raw = Vec::new();
        let read = source.take(max.saturating_add(1)).read_to_end(&mut raw);
        let _ = tx.send(read.ok().map(|_| raw));
    });
    rx.recv_timeout(wait).ok().flatten().filter(|raw| raw.len() as u64 <= max)
}

#[cfg(test)]
fn run(input: &[u8], paths: &Paths, started: Instant, budget: Duration) -> (Option<String>, bool) {
    let event = serde_json::from_slice::<Value>(input).map_err(|e| format!("hook input is not JSON ({e})"));
    let (reply, kept, entry) = run_event(event, paths, started, budget);
    if let Some(entry) = entry {
        note(paths, &entry);
    }
    (reply, kept)
}

type Done = (Option<String>, bool, Option<Value>);

fn run_event(event: Result<Value, String>, paths: &Paths, started: Instant, budget: Duration) -> Done {
    let event = match event {
        Ok(event) => event,
        Err(why) => return (failed(paths, &why), false, None),
    };
    match panic::catch_unwind(panic::AssertUnwindSafe(|| handle(event, paths, started, budget))) {
        Ok(Ok((reply, entry))) => {
            let kept = reply.is_some();
            (reply, kept, entry)
        }
        Ok(Err(why)) => (failed(paths, &why), false, None),
        Err(_) => (failed(paths, "the hook crashed"), false, None),
    }
}

type Handled = (Option<String>, Option<Value>);

fn handle(mut event: Value, paths: &Paths, started: Instant, budget: Duration) -> Result<Handled, String> {
    let tool = event["tool_name"].as_str().unwrap_or_default().to_owned();
    let tool = tool.as_str();
    let mut entry = Map::new();
    entry.insert("t".into(), json!(unix_ms()));
    entry.insert("sid".into(), event["session_id"].clone());
    entry.insert("tool".into(), json!(tool));
    let reply = if let Some(done) = crate::reads::post_tool(&paths.state, &mut event, &mut entry) {
        Some(done)
    } else if tool == "Bash" {
        bash(&mut event, paths, started + budget, &mut entry)?
    } else if TEXT_TOOLS.contains(&tool) {
        text_tool(&mut event, tool, paths, started + budget, &mut entry)?
    } else {
        if let Some(name) = kept_ref(&event["tool_input"].to_string()) {
            entry.insert("act".into(), json!("reload"));
            entry.insert("ref".into(), json!(name));
        }
        None
    };
    let entry = entry.contains_key("act").then(|| {
        entry.insert("ms".into(), json!(started.elapsed().as_millis() as u64));
        Value::Object(entry)
    });
    Ok((reply, entry))
}

fn act(entry: &mut Map<String, Value>, why: &str) -> Result<Option<String>, String> {
    entry.insert("act".into(), json!(why));
    Ok(None)
}

fn bash(
    event: &mut Value,
    paths: &Paths,
    deadline: Instant,
    entry: &mut Map<String, Value>,
) -> Result<Option<String>, String> {
    let cmd = event["tool_input"]["command"].as_str().unwrap_or_default();
    let response = &event["tool_response"];
    let Some(stdout) = response.get("stdout").and_then(Value::as_str) else { return act(entry, "shape") };
    let chars = stdout.chars().count();
    entry.insert("in".into(), json!(chars));
    if response.get("isImage").and_then(Value::as_bool) == Some(true) {
        return act(entry, "image");
    }
    if let Some(name) = kept_ref(cmd) {
        entry.insert("ref".into(), json!(name));
        return act(entry, "reload");
    }
    if let Some(why) = untouchable(cmd) {
        return act(entry, why);
    }
    if chars <= MIN_CHARS {
        return act(entry, "small");
    }
    let squeezed = if chars <= INLINE_MAX {
        let done = squeeze(stdout);
        if Instant::now() > deadline {
            return act(entry, "slow");
        }
        done
    } else {
        let text = stdout.to_owned();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(squeeze(&text));
        });
        match rx.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
            Ok(s) => s,
            Err(RecvTimeoutError::Timeout) => return act(entry, "slow"),
            Err(RecvTimeoutError::Disconnected) => return Err("squeeze crashed".into()),
        }
    };
    let Some(s) = squeezed else { return act(entry, "same") };
    cut(event, "Bash", paths, entry, s, chars)
}

fn text_tool(
    event: &mut Value,
    tool: &str,
    paths: &Paths,
    deadline: Instant,
    entry: &mut Map<String, Value>,
) -> Result<Option<String>, String> {
    let Some(text) = extract(tool, &event["tool_response"]) else { return act(entry, "shape") };
    let chars = text.chars().count();
    entry.insert("in".into(), json!(chars));
    if kept_ref(&event["tool_input"].to_string()).is_some() {
        return act(entry, "reload");
    }
    if chars <= MIN_CHARS {
        return act(entry, "small");
    }
    let done = squeeze_tool(tool, &text);
    if Instant::now() > deadline {
        return act(entry, "slow");
    }
    let Some(s) = done else { return act(entry, "same") };
    cut(event, tool, paths, entry, s, chars)
}

fn cut(
    event: &mut Value,
    tool: &str,
    paths: &Paths,
    entry: &mut Map<String, Value>,
    s: Squeezed,
    chars: usize,
) -> Result<Option<String>, String> {
    let name =
        format!("hr-{:010x}", quick(&[event["tool_use_id"].as_str().unwrap_or_default(), s.plain.as_str()]) >> 24);
    let file = paths.out.join(format!("{name}.txt"));
    let top = format!(
        "wisply squeezed {} lines (~{} tokens) to {} lines (~{} tokens), all of it: {}\n",
        commas(s.lines_in),
        commas(s.chars_in / 4),
        s.lines_out,
        commas(s.chars_out / 4),
        slashed(&file),
    );
    let small = top.chars().count() + s.chars_out;
    if small >= chars {
        return act(entry, "same");
    }
    entry.insert("out".into(), json!(small));
    entry.insert("kind".into(), json!(if s.json { "json" } else { "log" }));
    entry.insert("ref".into(), json!(name));
    if fnv(&[event["session_id"].as_str().unwrap_or_default()]).is_multiple_of(HOLDOUT_EVERY) {
        return act(entry, "holdout");
    }
    keep(&paths.out, &file, &s.plain).map_err(|e| format!("cannot keep the full output ({e})"))?;
    entry.insert("act".into(), json!("cut"));
    let mut updated = event["tool_response"].take();
    if tool == "Bash" {
        updated["stdout"] = Value::String(top + &s.body);
    } else {
        apply(tool, &mut updated, &(top + &s.body));
    }
    Ok(Some(json!({"hookSpecificOutput": {"hookEventName": "PostToolUse", "updatedToolOutput": updated}}).to_string()))
}

fn keep(dir: &Path, file: &Path, text: &str) -> io::Result<()> {
    match fs::write(file, text) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            fs::create_dir_all(dir)?;
            fs::write(file, text)
        }
        other => other,
    }
}

fn untouchable(cmd: &str) -> Option<&'static str> {
    if OWN.is_match(cmd) {
        Some("own")
    } else if DIFF.is_match(cmd)
        || LISTING.is_match(cmd)
        || FILE_READ.captures_iter(cmd).any(|c| !LOGISH.is_match(&c[1]))
    {
        Some("exact")
    } else {
        None
    }
}

fn kept_ref(text: &str) -> Option<String> {
    KEPT.captures(text).map(|c| c[1].to_string())
}

fn failed(paths: &Paths, why: &str) -> Option<String> {
    let now = unix_ms() / 1000;
    let file = paths.state.join("errors.txt");
    let mut times: Vec<u64> = fs::read_to_string(&file)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .filter(|t| now.saturating_sub(*t) < ERRORS_WINDOW_S)
        .collect();
    times.push(now);
    let _ = fs::create_dir_all(&paths.state);
    let _ = fs::write(&file, times.iter().map(u64::to_string).collect::<Vec<_>>().join("\n") + "\n");
    note(paths, &json!({"t": unix_ms(), "act": "error", "why": why}));
    if times.len() < ERRORS_MAX {
        return None;
    }
    let off = paths.state.join("off");
    let _ = fs::write(&off, format!("{why}\n"));
    let message = format!(
        "Wisply stopped squeezing tool outputs after {ERRORS_MAX} errors in {} minutes (last: {why}). Outputs come through in full now; delete {} to turn it back on.",
        ERRORS_WINDOW_S / 60,
        slashed(&off)
    );
    Some(json!({ "systemMessage": message }).to_string())
}

fn open_log(paths: &Paths, file: &Path) -> Option<fs::File> {
    let open = || OpenOptions::new().create(true).append(true).open(file);
    match open() {
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            fs::create_dir_all(&paths.state).ok()?;
            open().ok()
        }
        other => other.ok(),
    }
}

fn note(paths: &Paths, entry: &Value) {
    let file = paths.state.join("log.jsonl");
    let Some(mut f) = open_log(paths, &file) else { return };
    let line = format!("{entry}\n");
    if f.metadata().is_ok_and(|m| m.len() > LOG_MAX) {
        drop(f);
        let _ = fs::rename(&file, paths.state.join("log.1.jsonl"));
        let Some(fresh) = open_log(paths, &file) else { return };
        f = fresh;
    }
    let _ = f.write_all(line.as_bytes());
}

fn tidy(paths: &Paths) {
    let mark = paths.state.join("tidied");
    let recent = fs::metadata(&mark).and_then(|m| m.modified()).ok().and_then(|t| t.elapsed().ok());
    if recent.is_some_and(|age| age < TIDY_EVERY) {
        return;
    }
    let _ = fs::create_dir_all(&paths.state);
    let _ = fs::write(&mark, b"");
    let Ok(dir) = fs::read_dir(&paths.out) else { return };
    let mut files: Vec<(SystemTime, u64, PathBuf)> = dir
        .flatten()
        .filter(|e| e.file_name().to_string_lossy().starts_with("hr-"))
        .filter_map(|e| {
            let meta = e.metadata().ok()?;
            Some((meta.modified().ok()?, meta.len(), e.path()))
        })
        .collect();
    files.sort_by_key(|f| Reverse(f.0));
    let mut total = 0;
    for (when, size, path) in files {
        total += size;
        if total > KEEP_BYTES || when.elapsed().is_ok_and(|age| age > KEEP_FOR) {
            let _ = fs::remove_file(path);
        }
    }
}

pub fn fnv(parts: &[&str]) -> u64 {
    let mut h = FNV_START;
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            h = (h ^ 0xff).wrapping_mul(FNV_PRIME);
        }
        for b in part.bytes() {
            h = (h ^ u64::from(b)).wrapping_mul(FNV_PRIME);
        }
    }
    h
}

pub fn quick(parts: &[&str]) -> u64 {
    let mut h = FNV_START;
    for part in parts {
        let (words, tail) = part.as_bytes().as_chunks::<8>();
        for w in words {
            h = (h ^ u64::from_le_bytes(*w)).wrapping_mul(MIX);
            h ^= h >> 29;
        }
        let rest = tail.iter().enumerate().fold(part.len() as u64, |a, (i, &b)| a ^ (u64::from(b) << (8 * i)));
        h = (h ^ rest).wrapping_mul(MIX);
        h ^= h >> 32;
    }
    h.wrapping_mul(MIX) ^ (h >> 29)
}

fn unix_ms() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64)
}

fn slashed(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
#[path = "hook/tools.rs"]
mod tools;

#[cfg(test)]
#[path = "hook/tests.rs"]
mod tests;
