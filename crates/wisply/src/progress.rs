use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

#[path = "progress/label.rs"]
mod label;
#[path = "progress/paint.rs"]
pub mod paint;

use label::activity;
use paint::{draw, plain};

const FOLDER: &str = "progress";
const USAGE: &str =
    "usage: wisply progress [<percent> | <done>/<total>] [\"what you are doing\"] | show [--color] [--ascii] | clear";
const BAD_AMOUNT: &str = "the amount is a percent from 0 to 100 or done/total, like 40 or 3/8";
const NOTHING: &str = "no progress noted -> try: wisply progress 40 \"what you are doing\"";
const NO_KEY: &str = "no session and no folder to note progress for";
const NOT_SAVED: &str = "cannot save the progress";
const CLEARED: &str = "progress cleared";
const LOCK_TRIES: u32 = 40;
const LOCK_PAUSE: Duration = Duration::from_millis(5);
const READ_TRIES: u32 = 3;
const READ_PAUSE: Duration = Duration::from_millis(2);
const STEPS_MAX: usize = 200;
const TEXT_MAX: usize = 120;
const LAST_FRESH: u64 = 180;
const PLAN_FRESH: u64 = 12 * 3600;
const DONE_FRESH: u64 = 600;
const KEEP_FOR: Duration = Duration::from_secs(3 * 86_400);

#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
    Open,
    Doing,
    Done,
}

impl State {
    fn parse(word: &str) -> Option<State> {
        match word {
            "pending" | "open" | "todo" => Some(State::Open),
            "in_progress" | "doing" => Some(State::Doing),
            "completed" | "done" => Some(State::Done),
            _ => None,
        }
    }

    fn word(self) -> &'static str {
        match self {
            State::Open => "open",
            State::Doing => "doing",
            State::Done => "done",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Step {
    id: String,
    text: String,
    doing: String,
    state: State,
}

impl Step {
    fn parse(v: &Value, place: usize) -> Option<Step> {
        let text = cut(word(v, &["text", "content", "step", "subject"])?, TEXT_MAX);
        let doing = word(v, &["doing", "activeForm"]).map(|d| cut(d, TEXT_MAX)).unwrap_or_default();
        let state = word(v, &["state", "status"]).and_then(State::parse).unwrap_or(State::Open);
        let id = id_of(v.get("id")).unwrap_or_else(|| place.to_string());
        (!text.is_empty()).then_some(Step { id, text, doing, state })
    }

    fn value(&self) -> Value {
        json!({"id": self.id, "text": self.text, "doing": self.doing, "state": self.state.word()})
    }

    fn shown(&self) -> &str {
        let running = self.state == State::Doing && !self.doing.is_empty();
        if running { self.doing.as_str() } else { self.text.as_str() }
    }
}

struct View {
    percent: Option<i64>,
    count: String,
    text: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Plan {
    at: u64,
    steps: Vec<Step>,
    percent: Option<i64>,
    count: String,
    label: String,
}

impl Plan {
    fn parse(text: &str) -> Plan {
        let v: Value = serde_json::from_str(text).unwrap_or(Value::Null);
        Plan {
            at: v["t"].as_u64().unwrap_or(0),
            steps: steps_of(&v["steps"]),
            percent: v["percent"].as_i64().map(|p| p.clamp(0, 100)),
            count: cut(v["count"].as_str().unwrap_or_default(), TEXT_MAX),
            label: cut(v["label"].as_str().unwrap_or_default(), TEXT_MAX),
        }
    }

    fn dump(&self) -> String {
        let steps: Vec<Value> = self.steps.iter().map(Step::value).collect();
        let (percent, count, label) = (self.percent, &self.count, &self.label);
        json!({"t": self.at, "steps": steps, "percent": percent, "count": count, "label": label}).to_string()
    }

    fn by_steps(&mut self) {
        self.percent = None;
        self.count.clear();
        self.label.clear();
    }

    fn replace(&mut self, steps: Vec<Step>) {
        self.steps = steps;
        self.by_steps();
    }

    fn add(&mut self, input: &Value, made: &Value, now: u64) {
        if self.finished() && now.saturating_sub(self.at) > DONE_FRESH {
            self.steps.clear();
        }
        let place = self.steps.len() + 1;
        let Some(mut step) = Step::parse(input, place).filter(|_| place <= STEPS_MAX) else { return };
        step.id = id_of(made.get("id")).unwrap_or(step.id);
        step.state = State::Open;
        self.steps.retain(|s| s.id != step.id);
        self.steps.push(step);
        self.by_steps();
    }

    fn update(&mut self, input: &Value) {
        let Some(id) = id_of(input.get("taskId").or_else(|| input.get("id"))) else { return };
        let status = word(input, &["status"]);
        if status == Some("deleted") {
            self.steps.retain(|s| s.id != id);
            return;
        }
        let Some(step) = self.steps.iter_mut().find(|s| s.id == id) else { return };
        if let Some(state) = status.and_then(State::parse) {
            step.state = state;
        }
        if let Some(text) = word(input, &["subject"]) {
            step.text = cut(text, TEXT_MAX);
        }
        if let Some(doing) = word(input, &["activeForm"]) {
            step.doing = cut(doing, TEXT_MAX);
        }
        self.by_steps();
    }

    fn report(&mut self, amount: Option<(i64, String)>, label: String) {
        let numbered = amount.is_some();
        if let Some((percent, count)) = amount {
            self.steps.clear();
            self.percent = Some(percent);
            self.count = count;
        }
        if !numbered || !label.is_empty() {
            self.label = label;
        }
    }

    fn finished(&self) -> bool {
        match self.percent {
            Some(percent) => percent >= 100,
            None => !self.steps.is_empty() && self.steps.iter().all(|s| s.state == State::Done),
        }
    }

    fn view(&self) -> Option<View> {
        let total = self.steps.len();
        let done = self.steps.iter().filter(|s| s.state == State::Done).count();
        let by_steps = (total > 0).then(|| i64::try_from(done * 100 / total).unwrap_or(100));
        let percent = self.percent.or(by_steps);
        let count = if self.count.is_empty() && total > 0 { format!("{done}/{total}") } else { self.count.clone() };
        let by_state = |state: State| self.steps.iter().find(|s| s.state == state);
        let current = by_state(State::Doing).or_else(|| by_state(State::Open));
        let step = current.map(Step::shown).unwrap_or_default();
        let text = if self.label.is_empty() { step } else { self.label.as_str() }.to_string();
        (percent.is_some() || !text.is_empty()).then_some(View { percent, count, text })
    }
}

fn word<'a>(v: &'a Value, keys: &[&str]) -> Option<&'a str> {
    keys.iter().find_map(|key| v.get(*key).and_then(Value::as_str)).map(str::trim).filter(|s| !s.is_empty())
}

fn id_of(v: Option<&Value>) -> Option<String> {
    match v? {
        Value::String(s) if !s.trim().is_empty() => Some(s.trim().to_string()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn steps_of(list: &Value) -> Vec<Step> {
    let Some(items) = list.as_array() else { return Vec::new() };
    items.iter().take(STEPS_MAX).enumerate().filter_map(|(i, v)| Step::parse(v, i + 1)).collect()
}

fn cut(text: &str, max: usize) -> String {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    flat.chars().filter(|c| !c.is_control()).take(max).collect()
}

fn dir(home: &Path) -> PathBuf {
    home.join(FOLDER)
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

fn keys(sid: Option<&str>, cwd: Option<&str>) -> Vec<String> {
    let filled = |s: Option<&str>| s.map(str::trim).filter(|s| !s.is_empty()).map(str::to_string);
    let session = filled(sid).map(|sid| format!("s-{:016x}", crate::hook::fnv(&[sid.as_str()])));
    let folder = filled(cwd).map(|cwd| {
        let flat = cwd.replace('\\', "/").trim_end_matches('/').to_lowercase();
        format!("d-{:016x}", crate::hook::fnv(&[flat.as_str()]))
    });
    session.into_iter().chain(folder).collect()
}

fn locked(file: &Path) -> Option<File> {
    let open = || OpenOptions::new().create(true).truncate(false).read(true).write(true).open(file);
    let handle = match open() {
        Err(e) if e.kind() == ErrorKind::NotFound => {
            if let Some(dir) = file.parent() {
                let _ = fs::create_dir_all(dir);
            }
            open()
        }
        other => other,
    };
    let handle = handle.ok()?;
    for _ in 0..LOCK_TRIES {
        if handle.try_lock().is_ok() {
            return Some(handle);
        }
        thread::sleep(LOCK_PAUSE);
    }
    None
}

fn sweep(dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let age = entry.metadata().and_then(|m| m.modified()).ok().and_then(|at| at.elapsed().ok());
        if age.is_some_and(|age| age > KEEP_FOR) {
            let _ = fs::remove_file(entry.path());
        }
    }
}

fn change(file: &Path, edit: impl FnOnce(&mut Plan)) -> Option<Plan> {
    let mut handle = locked(file)?;
    let mut text = String::new();
    if handle.read_to_string(&mut text).is_err() {
        text.clear();
    }
    let fresh = text.is_empty();
    if fresh && let Some(dir) = file.parent() {
        sweep(dir);
    }
    let mut plan = Plan::parse(&text);
    edit(&mut plan);
    plan.at = now();
    let out = plan.dump();
    handle.seek(SeekFrom::Start(0)).ok()?;
    handle.write_all(out.as_bytes()).ok()?;
    handle.set_len(out.len() as u64).ok()?;
    Some(plan)
}

fn read(file: &Path) -> Option<String> {
    for _ in 0..READ_TRIES {
        match fs::read_to_string(file) {
            Ok(text) if !text.trim().is_empty() => return Some(text),
            Err(e) if e.kind() == ErrorKind::NotFound => return None,
            _ => thread::sleep(READ_PAUSE),
        }
    }
    None
}

fn fresh_plan(file: &Path, now: u64) -> Option<Plan> {
    let plan = Plan::parse(&read(file)?);
    let limit = if plan.finished() { DONE_FRESH } else { PLAN_FRESH };
    (plan.at > 0 && now.saturating_sub(plan.at) <= limit).then_some(plan)
}

fn fresh_last(file: &Path, now: u64) -> Option<String> {
    let text = fs::read_to_string(file).ok()?;
    let (at, label) = text.split_once('\t')?;
    let at: u64 = at.trim().parse().ok()?;
    let label = cut(label, TEXT_MAX);
    (now.saturating_sub(at) <= LAST_FRESH && !label.is_empty()).then_some(label)
}

pub fn field(home: &Path, sid: Option<&str>, cwd: Option<&str>, now: u64, ascii: bool) -> Option<String> {
    let folder = dir(home);
    let names = keys(sid, cwd);
    let plan = names.iter().find_map(|k| fresh_plan(&folder.join(format!("{k}.json")), now));
    let view = plan.and_then(|p| p.view());
    let last = names.iter().find_map(|k| fresh_last(&folder.join(format!("{k}.now")), now));
    let (percent, count, text) = match &view {
        Some(v) => (v.percent, v.count.as_str(), v.text.as_str()),
        None => (None, "", ""),
    };
    draw(percent, count, text, last.as_deref(), ascii)
}

fn mark(file: &Path, label: &str) {
    let line = format!("{}\t{label}", now());
    let failed = fs::write(file, line.as_bytes()).is_err();
    if failed && let Some(dir) = file.parent() {
        let _ = fs::create_dir_all(dir);
        let _ = fs::write(file, line.as_bytes());
    }
}

pub fn note(home: &Path, event: &Value) -> bool {
    let Some(key) = keys(event["session_id"].as_str(), event["cwd"].as_str()).into_iter().next() else {
        return false;
    };
    let tool = event["tool_name"].as_str().unwrap_or_default();
    let (input, time) = (&event["tool_input"], now());
    let plan = dir(home).join(format!("{key}.json"));
    match tool {
        "" => return false,
        "TodoWrite" => change(&plan, |p| p.replace(steps_of(&input["todos"]))),
        "update_plan" => change(&plan, |p| p.replace(steps_of(&input["plan"]))),
        "TaskCreate" => change(&plan, |p| p.add(input, &event["tool_response"]["task"], time)),
        "TaskUpdate" => change(&plan, |p| p.update(input)),
        _ => {
            mark(&dir(home).join(format!("{key}.now")), &activity(tool, input));
            return false;
        }
    };
    true
}

fn parse_amount(word: &str) -> Option<(i64, String)> {
    let word = word.trim_end_matches('%');
    let Some((done, total)) = word.split_once('/') else {
        let percent: i64 = word.parse().ok()?;
        return (0..=100).contains(&percent).then_some((percent, String::new()));
    };
    let (done, total): (u32, u32) = (done.parse().ok()?, total.parse().ok()?);
    let percent = (i64::from(done) * 100).checked_div(i64::from(total))?.min(100);
    Some((percent, format!("{}/{total}", done.min(total))))
}

fn report(home: &Path, key: &str, words: &[&str]) -> Result<String, String> {
    let first = words.first().copied().unwrap_or_default();
    let numbered = first.starts_with(|c: char| c.is_ascii_digit());
    let amount = if numbered { Some(parse_amount(first).ok_or(BAD_AMOUNT)?) } else { None };
    let label = cut(&words[usize::from(numbered).min(words.len())..].join(" "), TEXT_MAX);
    let plan = change(&dir(home).join(format!("{key}.json")), |p| p.report(amount, label)).ok_or(NOT_SAVED)?;
    let view = plan.view().ok_or(NOT_SAVED)?;
    Ok(plain(&draw(view.percent, &view.count, &view.text, None, true).unwrap_or_default()))
}

fn command(home: &Path, args: &[String]) -> Result<String, String> {
    let mut session = env::var("CLAUDE_CODE_SESSION_ID").ok();
    let (mut color, mut ascii, mut rest) = (false, false, Vec::new());
    let mut list = args.iter();
    while let Some(arg) = list.next() {
        match arg.as_str() {
            "--session" => session = Some(list.next().ok_or(USAGE)?.clone()),
            "--color" => color = true,
            "--ascii" => ascii = true,
            _ => rest.push(arg.as_str()),
        }
    }
    let folder = env::current_dir().ok().map(|dir| dir.to_string_lossy().into_owned());
    let (sid, cwd) = (session.as_deref(), folder.as_deref());
    let Some(key) = keys(sid, cwd).into_iter().next() else { return Err(NO_KEY.into()) };
    match rest.as_slice() {
        [] | ["show"] => {
            let line = field(home, sid, cwd, now(), ascii || !color);
            let line = line.map(|l| if color { l } else { plain(&l) });
            Ok(line.unwrap_or_else(|| NOTHING.to_string()))
        }
        ["clear"] => {
            for ext in ["json", "now"] {
                let _ = fs::remove_file(dir(home).join(format!("{key}.{ext}")));
            }
            Ok(CLEARED.to_string())
        }
        words => report(home, &key, words),
    }
}

pub fn run(home: &Path, args: &[String]) -> ExitCode {
    match command(home, args) {
        Ok(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("err: {e}");
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
#[path = "progress/tests.rs"]
mod tests;
