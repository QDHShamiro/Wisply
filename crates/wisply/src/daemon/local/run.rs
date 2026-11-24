use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use serde_json::{Map, Value};

use super::plan::{self, Budget, Live};
use super::state::{self, Slot, State};
use super::{Kind, Why, get, sys};
use crate::daemon::{config, game};

const START_WAIT: Duration = Duration::from_secs(30);
const POLL: Duration = Duration::from_millis(250);
const CHECK: Duration = Duration::from_secs(15);
const HEALTH_WAIT: Duration = Duration::from_secs(2);
const CONTEXT: &str = "1024";
const THINK_OFF: &str = "{\"enable_thinking\":false}";
const ASR_PATH: &str = "/v1/audio/transcriptions";
const LOCALHOST: &str = "127.0.0.1";
const MB: u64 = 1 << 20;

static GATE: Mutex<()> = Mutex::new(());
static STATE: Mutex<()> = Mutex::new(());

struct Settings {
    ram_mb: u64,
    idle_s: u64,
    threads: usize,
    game_mode: String,
    start_mb: [u64; 2],
}

fn num(cfg: &Map<String, Value>, key: &str, fallback: i64) -> i64 {
    cfg.get(key).and_then(Value::as_i64).unwrap_or(fallback)
}

fn settings(home: &Path) -> Settings {
    let cfg = config::load(home);
    let pos = |key: &str, fallback: i64| u64::try_from(num(&cfg, key, fallback)).unwrap_or(0);
    Settings {
        ram_mb: pos("local_ram_mb", 1024),
        idle_s: pos("local_idle_s", 120),
        threads: plan::threads(num(&cfg, "local_threads", 0), sys::cores()),
        game_mode: config::text(&cfg, "local_game"),
        start_mb: [pos("local_llm_start_mb", 650), pos("local_asr_start_mb", 450)],
    }
}

fn index(kind: Kind) -> usize {
    match kind {
        Kind::Llm => 0,
        Kind::Asr => 1,
    }
}

fn edit<R>(home: &Path, f: impl FnOnce(&mut State) -> R) -> R {
    let _lock = STATE.lock().unwrap_or_else(|e| e.into_inner());
    let mut st = state::load(home);
    let out = f(&mut st);
    state::save(home, &st);
    out
}

fn url_of(port: u16) -> String {
    format!("http://{LOCALHOST}:{port}")
}

fn healthy(port: u16) -> bool {
    let url = format!("{}/health", url_of(port));
    ureq::get(&url).config().timeout_global(Some(HEALTH_WAIT)).build().call().is_ok()
}

fn bin_root(home: &Path) -> PathBuf {
    state::dir(home).join("bin")
}

fn log_file(home: &Path, kind: Kind) -> PathBuf {
    state::dir(home).join(format!("{}.log", kind.label()))
}

pub fn args(kind: Kind, model: &Path, port: u16, threads: usize) -> Vec<String> {
    let mut list = vec![
        "-m".to_string(),
        model.display().to_string(),
        "--host".into(),
        LOCALHOST.into(),
        "--port".into(),
        port.to_string(),
        "-t".into(),
        threads.to_string(),
    ];
    let extra: &[&str] = match kind {
        Kind::Llm => &[
            "-c",
            CONTEXT,
            "-np",
            "1",
            "--reasoning",
            "off",
            "--reasoning-budget",
            "0",
            "--chat-template-kwargs",
            THINK_OFF,
            "--no-webui",
            "-cram",
            "0",
            "--cache-reuse",
            "256",
        ],
        Kind::Asr => &["--inference-path", ASR_PATH, "-nt", "-bo", "1", "-l", "auto"],
    };
    list.extend(extra.iter().map(|a| (*a).to_string()));
    list
}

fn live(st: &State) -> Vec<Live> {
    st.running().into_iter().map(|(kind, s)| Live { kind, mb: s.mb, used: s.used }).collect()
}

fn drop_dead(st: &mut State) {
    for (kind, slot) in st.running() {
        if !sys::alive(slot.pid) {
            st.put(kind, None);
        }
    }
}

fn stop_in(st: &mut State, kind: Kind, root: &Path) {
    if let Some(slot) = st.slot(kind).cloned() {
        if sys::is_ours(slot.pid, root) {
            sys::kill(slot.pid);
        }
        st.put(kind, None);
    }
}

pub fn release(home: &Path, kind: Kind) {
    let root = bin_root(home);
    edit(home, |st| stop_in(st, kind, &root));
}

pub fn stop_all(home: &Path) -> usize {
    let root = bin_root(home);
    edit(home, |st| {
        let n = st.running().len();
        stop_in(st, Kind::Llm, &root);
        stop_in(st, Kind::Asr, &root);
        n
    })
}

fn cleanup(home: &Path) {
    stop_all(home);
}

fn reuse(home: &Path, kind: Kind) -> Option<String> {
    let slot = edit(home, |st| st.slot(kind).cloned())?;
    if !sys::alive(slot.pid) || !healthy(slot.port) {
        return None;
    }
    edit(home, |st| {
        if let Some(s) = st.slot_mut(kind) {
            s.used = state::now();
        }
    });
    Some(url_of(slot.port))
}

fn make_room(home: &Path, kind: Kind, set: &Settings) -> Result<(), Why> {
    let root = bin_root(home);
    edit(home, |st| {
        drop_dead(st);
        for (k, slot) in st.running() {
            if let Some(mb) = sys::rss_mb(slot.pid)
                && let Some(s) = st.slot_mut(k)
            {
                s.mb = mb;
            }
        }
        let expect = plan::expect(st.seen[index(kind)], set.start_mb[index(kind)]);
        match plan::budget(set.ram_mb, &live(st), kind, expect, state::now()) {
            Budget::Fits => Ok(()),
            Budget::Kill(kinds) => {
                kinds.into_iter().for_each(|k| stop_in(st, k, &root));
                Ok(())
            }
            Budget::Refuse => Err(Why::Failed("the other local program is busy, try again in a moment".into())),
        }
    })
}

fn launch(home: &Path, kind: Kind, set: &Settings, playing: &str) -> Result<Slot, Why> {
    let exe = get::server_exe(home, kind).ok_or(Why::NotInstalled)?;
    let port = sys::free_port().ok_or_else(|| Why::Failed("no free port".into()))?;
    let model = get::model_path(home, kind);
    let list = args(kind, &model, port, set.threads);
    let dir = exe.parent().map_or_else(|| state::dir(home), Path::to_path_buf);
    let _ = fs::create_dir_all(state::dir(home));
    let prio = plan::priority(&set.game_mode, playing);
    let pid = sys::spawn(&exe, &list, &dir, &log_file(home, kind), prio)
        .ok_or_else(|| Why::Failed(format!("could not start {}", get::server_name(kind))))?;
    let began = state::now();
    let mut waited = Duration::ZERO;
    while waited < START_WAIT {
        if !sys::alive(pid) {
            return Err(Why::Failed(format!("{} stopped at start, see local/{}.log", kind.label(), kind.label())));
        }
        if healthy(port) {
            let mb = sys::rss_mb(pid).unwrap_or(0);
            return Ok(Slot { pid, port, used: state::now(), mb, started: began });
        }
        thread::sleep(POLL);
        waited += POLL;
    }
    sys::kill(pid);
    Err(Why::Failed(format!("{} did not get ready in {} s", kind.label(), START_WAIT.as_secs())))
}

pub fn ensure(home: &Path, kind: Kind) -> Result<String, Why> {
    if !get::installed(home, kind) {
        return Err(Why::NotInstalled);
    }
    let set = settings(home);
    let playing = game::running();
    if plan::gaming(&set.game_mode, &playing) {
        return Err(Why::Gaming);
    }
    let _gate = GATE.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(url) = reuse(home, kind) {
        return Ok(url);
    }
    release(home, kind);
    make_room(home, kind, &set)?;
    let slot = launch(home, kind, &set, &playing)?;
    let url = url_of(slot.port);
    edit(home, |st| {
        let seen = &mut st.seen[index(kind)];
        *seen = (*seen).max(slot.mb);
        st.put(kind, Some(slot));
    });
    Ok(url)
}

fn tick(home: &Path) {
    let root = bin_root(home);
    let running = edit(home, |st| {
        drop_dead(st);
        st.running()
    });
    if running.is_empty() {
        return;
    }
    let set = settings(home);
    let paused = plan::gaming(&set.game_mode, &game::running());
    let lives: Vec<Live> = running.iter().map(|(kind, s)| Live { kind: *kind, mb: s.mb, used: s.used }).collect();
    let due = plan::due(&lives, state::now(), set.idle_s, paused);
    edit(home, |st| {
        for kind in due {
            stop_in(st, kind, &root);
        }
        for (kind, slot) in st.running() {
            if let Some(mb) = sys::rss_mb(slot.pid) {
                st.seen[index(kind)] = st.seen[index(kind)].max(mb);
                if let Some(s) = st.slot_mut(kind) {
                    s.mb = mb;
                }
            }
        }
    });
}

pub fn start(home: PathBuf) {
    cleanup(&home);
    let _ = thread::Builder::new().name("local".into()).spawn(move || {
        loop {
            thread::sleep(CHECK);
            let _ = std::panic::catch_unwind(|| tick(&home));
        }
    });
}

fn ago(now: f64, then: f64) -> String {
    let secs = (now - then).max(0.0) as u64;
    if secs < 90 { format!("{secs} s ago") } else { format!("{} min ago", secs / 60) }
}

pub fn status(home: &Path) -> String {
    let st = edit(home, |st| {
        drop_dead(st);
        st.clone()
    });
    let now = state::now();
    [Kind::Llm, Kind::Asr]
        .into_iter()
        .map(|kind| {
            if !get::installed(home, kind) {
                return format!("{}: not installed (wisply local get)", kind.label());
            }
            match st.slot(kind) {
                Some(s) => format!(
                    "{}: running on port {}, {} MB, used {}",
                    kind.label(),
                    s.port,
                    sys::rss_mb(s.pid).unwrap_or(s.mb),
                    ago(now, s.used)
                ),
                None => format!("{}: installed, stopped", kind.label()),
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn size_of_dir(dir: &Path) -> u64 {
    let Ok(entries) = fs::read_dir(dir) else { return 0 };
    entries
        .flatten()
        .map(|e| match e.metadata() {
            Ok(m) if m.is_dir() => size_of_dir(&e.path()),
            Ok(m) => m.len(),
            Err(_) => 0,
        })
        .sum()
}

pub fn wipe(home: &Path, yes: bool) -> Result<String, String> {
    let dir = state::dir(home);
    if !dir.exists() {
        return Ok("local: nothing to delete".to_string());
    }
    let size = size_of_dir(&dir) / MB;
    if !yes {
        return Ok(format!("local: would delete {} ({size} MB), run again with --yes", dir.display()));
    }
    stop_all(home);
    fs::remove_dir_all(&dir).map_err(|e| format!("cannot delete {} ({e})", dir.display()))?;
    Ok(format!("local: deleted {} ({size} MB)", dir.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_llm_gets_a_small_context_no_thinking_and_one_slot() {
        let list = args(Kind::Llm, Path::new("m.gguf"), 5000, 2).join(" ");
        assert!(list.contains("-m m.gguf") && list.contains("--host 127.0.0.1") && list.contains("--port 5000"));
        assert!(list.contains("-t 2") && list.contains("-c 1024") && list.contains("-np 1"));
        assert!(list.contains("--reasoning-budget 0") && list.contains("enable_thinking"));
        assert!(list.contains("--cache-reuse 256"));
    }

    #[test]
    fn whisper_listens_on_the_openai_path() {
        let list = args(Kind::Asr, Path::new("w.bin"), 5001, 3);
        assert!(list.windows(2).any(|w| w[0] == "--inference-path" && w[1] == "/v1/audio/transcriptions"));
        assert!(!list.contains(&"-c".to_string()));
        assert!(list.contains(&"-nt".to_string()));
    }

    #[test]
    fn whisper_detects_the_language_unless_the_request_names_one() {
        let list = args(Kind::Asr, Path::new("w.bin"), 5001, 3);
        assert!(list.windows(2).any(|w| w[0] == "-l" && w[1] == "auto"));
    }

    #[test]
    fn nothing_starts_when_nothing_is_installed() {
        let home = std::env::temp_dir().join(format!("wisply-local-run-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(&home).unwrap();
        assert_eq!(ensure(&home, Kind::Llm), Err(Why::NotInstalled));
        assert_eq!(ensure(&home, Kind::Asr), Err(Why::NotInstalled));
        assert!(status(&home).contains("llm: not installed"));
        assert_eq!(stop_all(&home), 0);
        let _ = fs::remove_dir_all(&home);
    }

    #[test]
    fn rm_only_says_what_it_would_delete_without_yes() {
        let home = std::env::temp_dir().join(format!("wisply-local-rm-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(state::dir(&home)).unwrap();
        fs::write(state::dir(&home).join("x.bin"), vec![0u8; 3 << 20]).unwrap();
        let said = wipe(&home, false).unwrap();
        assert!(said.contains("would delete") && said.contains("3 MB") && state::dir(&home).exists());
        assert!(wipe(&home, true).unwrap().contains("deleted") && !state::dir(&home).exists());
        let _ = fs::remove_dir_all(&home);
    }

    #[test]
    fn a_dead_pid_in_the_state_is_dropped() {
        let home = std::env::temp_dir().join(format!("wisply-local-dead-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(&home).unwrap();
        edit(&home, |st| st.put(Kind::Asr, Some(Slot { pid: 4_000_000_000, port: 9, used: 0.0, mb: 1, started: 0.0 })));
        assert!(status(&home).contains("asr: not installed"));
        assert!(edit(&home, |st| st.slot(Kind::Asr).is_none()));
        let _ = fs::remove_dir_all(&home);
    }
}
