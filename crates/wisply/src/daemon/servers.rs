use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};
use wisply_core::Fail;
use wisply_memory::{LogHit, Seen, ServerState, Store, day, oneline};
use wisply_servers::{DEFAULT_LOG, Got, Hit, Problem, Profile, Want, crash, crash_dir, fetch, profiles, scan};

use super::{agents, config, game};

const EVERY: Duration = Duration::from_secs(300);
const NAP: Duration = Duration::from_secs(30);
const LIMIT: Duration = Duration::from_secs(40);
const ONE_LIMIT: Duration = Duration::from_secs(25);
const ALERT_TITLE: usize = 90;
const SET_FILE: &str = "wisply config servers_file <path to profiles.json>";

static BUSY: Mutex<()> = Mutex::new(());

struct Job {
    profile: Profile,
    state: ServerState,
    log: String,
    dir: Option<String>,
}

impl Job {
    fn want(&self) -> Want<'_> {
        Want {
            log: &self.log,
            offset: u64::try_from(self.state.offset).unwrap_or(0),
            head: &self.state.head,
            crash_dir: self.dir.as_deref(),
            crashes_after: &self.state.crashes,
            host_key: &self.state.hostkey,
        }
    }

    fn first(&self) -> bool {
        self.state.head.is_empty() && self.state.offset == 0
    }
}

fn now() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
}

fn file_of(home: &Path) -> Option<PathBuf> {
    let raw = config::text(&config::load(home), "servers_file");
    let raw = raw.trim();
    if raw.is_empty() { None } else { Some(PathBuf::from(raw)) }
}

fn known(home: &Path) -> Result<Vec<Profile>, String> {
    let Some(path) = file_of(home) else { return Err(format!("no profiles file yet: {SET_FILE}")) };
    profiles(&path)
}

pub fn state_of(store: &Store, name: &str) -> ServerState {
    store.server_state(name).ok().flatten().unwrap_or_else(|| ServerState {
        name: name.to_string(),
        enabled: true,
        ..ServerState::default()
    })
}

pub fn log_of(state: &ServerState) -> String {
    if state.log.trim().is_empty() { DEFAULT_LOG.to_string() } else { state.log.trim().to_string() }
}

pub fn start(home: PathBuf) {
    let _ = thread::Builder::new().name("servers".into()).spawn(move || {
        let mut last: Option<Instant> = None;
        loop {
            let cfg = config::load(&home);
            let due = last.is_none_or(|t| t.elapsed() >= EVERY);
            if due && config::on(&cfg, "servers") && file_of(&home).is_some() && game::running().is_empty() {
                last = Some(Instant::now());
                if panic::catch_unwind(AssertUnwindSafe(|| check_all(&home, None))).is_err() {
                    eprintln!("wisply servers: a check failed, see the panic above");
                }
            }
            thread::sleep(NAP);
        }
    });
}

fn check_all(home: &Path, only: Option<&str>) -> Result<Vec<ServerState>, String> {
    let Ok(_busy) = BUSY.try_lock() else { return Err("a check is already running".into()) };
    let list = known(home)?;
    let store = Store::open(home).map_err(|e| format!("cannot open memory.db ({e})"))?;
    let jobs: Vec<Job> = list
        .into_iter()
        .filter(|p| only.is_none_or(|o| o.eq_ignore_ascii_case(&p.name)))
        .map(|profile| {
            let state = state_of(&store, &profile.name);
            let log = log_of(&state);
            let dir = crash_dir(&log);
            Job { profile, state, log, dir }
        })
        .filter(|j| j.state.enabled)
        .collect();
    let limit = if only.is_some() { ONE_LIMIT } else { LIMIT };
    let results: Vec<Result<Got, Problem>> = thread::scope(|sc| {
        let handles: Vec<_> = jobs
            .iter()
            .map(|j| {
                sc.spawn(move || {
                    if j.profile.usable() {
                        fetch(&j.profile, &j.want(), limit)
                    } else {
                        Err(Problem::Failed("no login in the profiles file".into()))
                    }
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|h| h.join().unwrap_or_else(|_| Err(Problem::Failed("the check crashed".into()))))
            .collect()
    });
    Ok(jobs.into_iter().zip(results).map(|(j, r)| apply(home, &store, j, r)).collect())
}

fn record(store: &Store, server: &str, h: &Hit, at: f64) -> Option<Seen> {
    let hit = LogHit {
        server,
        fingerprint: &h.fingerprint,
        title: &h.title,
        level: h.level,
        excerpt: &h.excerpt,
        count: h.count,
        at,
    };
    store.log_hit(&hit).ok()
}

fn apply(home: &Path, store: &Store, job: Job, result: Result<Got, Problem>) -> ServerState {
    let first = job.first();
    let Job { profile, mut state, .. } = job;
    state.checked = now();
    match result {
        Ok(got) => {
            let found = scan(&got.text);
            let mut alerts = Vec::new();
            let crashes = got.crashes.iter().map(|c| crash(&c.name, &c.text));
            for h in found.hits.iter().cloned().chain(crashes) {
                let seen = record(store, &profile.name, &h, state.checked);
                if h.level == "crash" && matches!(seen, Some(Seen::New(_) | Seen::Back(_))) {
                    alerts.push(h.title.clone());
                }
            }
            if !first && (got.rotated || found.started) {
                state.started = state.checked;
                let _ =
                    store.restart_done(&profile.name, &format!("restart:{}:{}", profile.name, state.checked as i64));
            }
            if !first && !alerts.is_empty() {
                let title = match alerts.as_slice() {
                    [one] => format!("{}: {}", profile.name, oneline(one, ALERT_TITLE)),
                    many => format!("{}: {} crashes", profile.name, many.len()),
                };
                agents::alert(home, &title, &format!("wisply list --server {}", profile.name));
            }
            state.offset = i64::try_from(got.offset).unwrap_or(i64::MAX);
            state.head = got.head;
            state.crashes = got.newest_crash;
            if state.hostkey.is_empty() {
                state.hostkey = got.host_key;
            }
            state.ok = state.checked;
            state.error.clear();
        }
        Err(e) => state.error = e.to_string(),
    }
    let _ = store.server_save(&state);
    state
}

fn line(store: &Store, name: &str, st: &ServerState) -> String {
    let rows = store.entries(Some(name), false).unwrap_or_default();
    let open: Vec<_> = rows.iter().filter(|t| t.open() && t.kind == "bug").collect();
    let warns = open.iter().filter(|t| t.level == "warn").count();
    let health = if !st.enabled {
        "off".to_string()
    } else if st.checked == 0.0 {
        "not checked yet".to_string()
    } else if st.error.is_empty() {
        format!("ok {}", day(st.ok))
    } else {
        format!("{} ({})", st.error, day(st.checked))
    };
    let started = if st.started > 0.0 { format!(", restarted {}", day(st.started)) } else { String::new() };
    format!("{name}: {health}{started} | {} bugs, {warns} warnings | {}", open.len() - warns, log_of(st))
}

pub fn status_text(home: &Path, store: &Store) -> String {
    let list = match known(home) {
        Ok(list) => list,
        Err(why) => return why,
    };
    if list.is_empty() {
        return "the profiles file has no servers".into();
    }
    let watch = if config::on(&config::load(home), "servers") {
        "every 5 min, paused while a game runs"
    } else {
        "off (wisply config servers true)"
    };
    let mut out = vec![format!("{} servers, {watch}", list.len())];
    out.extend(list.iter().map(|p| line(store, &p.name, &state_of(store, &p.name))));
    out.join("\n")
}

pub fn rows(home: &Path, store: &Store) -> Value {
    let on = config::on(&config::load(home), "servers");
    let list = match known(home) {
        Ok(list) => list,
        Err(why) => return json!({"rows": [], "problem": why, "on": on}),
    };
    let rows: Vec<Value> = list
        .iter()
        .map(|p| {
            let st = state_of(store, &p.name);
            let open: Vec<_> =
                store.entries(Some(&p.name), false).unwrap_or_default().into_iter().filter(|t| t.open()).collect();
            let warns = open.iter().filter(|t| t.level == "warn").count();
            let bugs = open.iter().filter(|t| t.kind == "bug" && t.level != "warn").count();
            json!({"name": p.name, "enabled": st.enabled, "log": log_of(&st), "checked": st.checked, "ok": st.ok,
                   "error": st.error, "started": st.started, "bugs": bugs, "warns": warns, "usable": p.usable()})
        })
        .collect();
    json!({"rows": rows, "problem": "", "on": on})
}

pub fn check_now(home: &Path, store: &Store, only: Option<&str>) -> Result<String, Fail> {
    let Some(name) = only else {
        let home = home.to_path_buf();
        let _ = thread::Builder::new().name("servers-now".into()).spawn(move || {
            let _ = check_all(&home, None);
        });
        return Ok("checking all servers in the background (wisply list servers)".into());
    };
    let states = check_all(home, Some(name)).map_err(|why| Fail::hint(why, SET_FILE))?;
    let Some(st) = states.first() else {
        return Err(Fail::hint(format!("'{name}' is not on (or not in the profiles file)"), "wisply list servers"));
    };
    Ok(line(store, &st.name, st))
}

pub fn set(home: &Path, store: &Store, name: &str, field: &str, value: &str) -> Result<String, Fail> {
    let list = known(home).map_err(|why| Fail::hint(why, SET_FILE))?;
    let Some(p) = list.iter().find(|p| p.name.eq_ignore_ascii_case(name.trim())) else {
        let names: Vec<&str> = list.iter().map(|p| p.name.as_str()).collect();
        return Err(Fail::hint(format!("no server '{name}'"), format!("one of: {}", names.join(", "))));
    };
    let mut st = state_of(store, &p.name);
    let said = match field {
        "enabled" => {
            st.enabled = value == "on";
            format!("{} is {value}", p.name)
        }
        _ => {
            let log = value.trim().replace('\\', "/");
            st.log = if log == DEFAULT_LOG { String::new() } else { log };
            (st.offset, st.head, st.crashes) = (0, String::new(), String::new());
            format!("{} reads {} from the next check", p.name, log_of(&st))
        }
    };
    store.server_save(&st).map_err(|e| Fail::new(format!("memory.db: {e}")))?;
    Ok(said)
}
