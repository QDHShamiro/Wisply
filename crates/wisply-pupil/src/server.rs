use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::chat;
use crate::config::{Config, Model};
use crate::game::{self, system};
use crate::spawn;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub keeper: u32,
    pub server: u32,
    pub model: String,
    pub port: u16,
    pub started: f64,
    #[serde(default)]
    pub args: Vec<String>,
}

pub struct Files {
    pub dir: PathBuf,
}

impl Files {
    pub fn new(config: &Config) -> Files {
        Files { dir: config.paths.run_dir.clone() }
    }
    pub fn state(&self) -> PathBuf {
        self.dir.join("state.json")
    }
    pub fn used(&self) -> PathBuf {
        self.dir.join("used")
    }
    pub fn stop(&self) -> PathBuf {
        self.dir.join("stop")
    }
    pub fn afk(&self) -> PathBuf {
        self.dir.join("afk")
    }
    pub fn lock(&self) -> PathBuf {
        self.dir.join("start.lock")
    }
    pub fn busy(&self) -> PathBuf {
        self.dir.join("busy")
    }
    pub fn log(&self) -> PathBuf {
        self.dir.join("server.log")
    }
    pub fn closed(&self) -> PathBuf {
        self.dir.join("closed")
    }
}

pub const CLOSED_GAME: &str = "game";

pub fn closed_reason(config: &Config) -> Option<String> {
    let text = fs::read_to_string(Files::new(config).closed()).ok()?;
    let reason = text.trim();
    (!reason.is_empty()).then(|| reason.to_string())
}

pub fn now() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0)
}

pub fn read_state(files: &Files) -> Option<State> {
    serde_json::from_str(&fs::read_to_string(files.state()).ok()?).ok()
}

fn write_atomic(path: &Path, text: &str) -> std::io::Result<()> {
    let temporary = path.with_extension("tmp");
    fs::write(&temporary, text)?;
    fs::rename(&temporary, path)
}

pub fn touch_used(files: &Files) {
    let _ = write_atomic(&files.used(), &now().to_string());
}

fn last_used(files: &Files) -> f64 {
    fs::read_to_string(files.used()).ok().and_then(|t| t.trim().parse().ok()).unwrap_or(0.0)
}

fn file_name(path: &Path) -> String {
    path.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default()
}

pub fn is_process(pid: u32, exe: &Path) -> bool {
    pid != 0 && system::alive(pid) && system::process_path(pid).ends_with(&file_name(exe))
}

pub fn keeper_alive(state: &State) -> bool {
    std::env::current_exe().map(|exe| is_process(state.keeper, &exe)).unwrap_or(false)
}

pub fn healthy(config: &Config) -> bool {
    let url = format!("http://{}:{}/health", config.server.host, config.server.port);
    match chat::agent(Duration::from_secs(2)).get(&url).call() {
        Ok(response) => response.status().as_u16() == 200,
        Err(_) => false,
    }
}

const CONTEXT_FLAG: &str = "-c";

pub fn args_context(args: &[String]) -> Option<u32> {
    args.iter().position(|arg| arg == CONTEXT_FLAG).and_then(|at| args.get(at + 1)).and_then(|value| value.parse().ok())
}

pub fn live_context(config: &Config) -> Option<u32> {
    read_state(&Files::new(config))
        .filter(|state| state.server != 0 && system::alive(state.server))
        .and_then(|state| args_context(&state.args))
}

pub fn fit_context(wanted: u32, live: Option<u32>) -> u32 {
    live.map_or(wanted, |live| live.min(wanted))
}

pub fn server_args(config: &Config, model: &Model) -> Vec<String> {
    let server = &config.server;
    let mut args: Vec<String> = vec![
        "-m".into(),
        model.file.to_string_lossy().into_owned(),
        "--mmproj".into(),
        model.mmproj.to_string_lossy().into_owned(),
        "-ngl".into(),
        server.gpu_layers.to_string(),
        CONTEXT_FLAG.into(),
        server.context.to_string(),
        "--host".into(),
        server.host.clone(),
        "--port".into(),
        server.port.to_string(),
        "-ctk".into(),
        server.kv_type.clone(),
        "-ctv".into(),
        server.kv_type.clone(),
        "-fa".into(),
        server.flash_attention.clone(),
        "-np".into(),
        "1".into(),
        "--image-max-tokens".into(),
        server.image_max_tokens.to_string(),
        "--no-webui".into(),
    ];
    args.extend(server.extra_args.iter().cloned());
    args.extend(model.extra_args.iter().cloned());
    args
}

static MARKS: AtomicU64 = AtomicU64::new(0);

fn marker_pid(name: &str) -> u32 {
    name.split('-').next().and_then(|pid| pid.parse::<u32>().ok()).unwrap_or(0)
}

fn count_busy(files: &Files, skip: u32) -> usize {
    let Ok(entries) = fs::read_dir(files.busy()) else {
        return 0;
    };
    let mut count = 0;
    for entry in entries.flatten() {
        let pid = marker_pid(&entry.file_name().to_string_lossy());
        if pid != 0 && system::alive(pid) {
            count += usize::from(pid != skip);
        } else {
            let _ = fs::remove_file(entry.path());
        }
    }
    count
}

pub fn live_busy(files: &Files) -> usize {
    count_busy(files, 0)
}

pub fn others_busy(files: &Files) -> usize {
    count_busy(files, std::process::id())
}

fn wait_free(config: &Config, files: &Files) {
    let poll = Duration::from_millis(config.server.health_poll_ms);
    while others_busy(files) > 0 {
        std::thread::sleep(poll);
    }
}

pub struct Busy {
    path: PathBuf,
    files: Files,
}

impl Busy {
    pub fn mark(config: &Config) -> Busy {
        let files = Files::new(config);
        let _ = fs::create_dir_all(files.busy());
        let mark = MARKS.fetch_add(1, Ordering::Relaxed);
        let path = files.busy().join(format!("{}-{mark}", std::process::id()));
        let _ = fs::write(&path, b"");
        touch_used(&files);
        Busy { path, files }
    }
}

impl Drop for Busy {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
        touch_used(&self.files);
    }
}

struct StartLock {
    path: PathBuf,
}

impl Drop for StartLock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn take_lock(files: &Files, deadline: Instant, poll: Duration) -> Option<StartLock> {
    let path = files.lock();
    loop {
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(mut file) => {
                let _ = write!(file, "{}", std::process::id());
                return Some(StartLock { path });
            }
            Err(_) => {
                let owner = fs::read_to_string(&path).ok().and_then(|t| t.trim().parse::<u32>().ok()).unwrap_or(0);
                if owner == 0 || !system::alive(owner) {
                    let _ = fs::remove_file(&path);
                    continue;
                }
                if Instant::now() >= deadline {
                    return None;
                }
                std::thread::sleep(poll);
            }
        }
    }
}

pub fn stop(config: &Config) -> bool {
    let files = Files::new(config);
    let Some(state) = read_state(&files) else {
        return false;
    };
    let _ = fs::write(files.stop(), b"");
    if others_busy(&files) > 0 {
        return false;
    }
    let until = Instant::now() + Duration::from_secs(config.keeper.poll_seconds * 3 + 5);
    while keeper_alive(&state) && Instant::now() < until {
        std::thread::sleep(Duration::from_millis(config.server.health_poll_ms));
    }
    if keeper_alive(&state) {
        system::kill(state.keeper);
    }
    if is_process(state.server, &config.server.exe) {
        system::kill(state.server);
    }
    let _ = fs::remove_file(files.state());
    let _ = fs::remove_file(files.stop());
    true
}

pub enum StartError {
    Failed(String),
    Timeout,
    Game,
}

fn log_tail(files: &Files) -> String {
    let text = fs::read_to_string(files.log()).unwrap_or_default();
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    lines[lines.len().saturating_sub(3)..].join(" | ")
}

fn spawn_keeper(config_path: &Path, model: &Model) -> Result<u32, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let child = spawn::detached(exe)
        .arg("pupil")
        .arg("keeper")
        .arg(&model.name)
        .env("PUPIL_CONFIG", config_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(child.id())
}

fn wait_ready(config: &Config, files: &Files, keeper: u32, deadline: Instant) -> Result<(), StartError> {
    let poll = Duration::from_millis(config.server.health_poll_ms);
    loop {
        let state = read_state(files);
        let ours = state.as_ref().is_some_and(|s| s.keeper == keeper);
        if ours && healthy(config) {
            return Ok(());
        }
        let alive = state.as_ref().filter(|_| ours).map(keeper_alive).unwrap_or_else(|| system::alive(keeper));
        if !alive {
            return Err(StartError::Failed(log_tail(files)));
        }
        if Instant::now() >= deadline {
            return Err(StartError::Timeout);
        }
        std::thread::sleep(poll);
    }
}

pub(crate) fn same_server(state: &State, config: &Config, model: &Model) -> bool {
    state.model == model.name && state.port == config.server.port && state.args == server_args(config, model)
}

pub fn shares(share: bool, running: Option<&str>, ours: &str, healthy: impl FnOnce() -> bool) -> bool {
    share && running == Some(ours) && healthy()
}

pub fn ensure(config: &Config, config_path: &Path, model: &Model) -> Result<(), StartError> {
    let files = Files::new(config);
    let running = read_state(&files).map(|state| state.model);
    if shares(config.server.share, running.as_deref(), &model.name, || healthy(config)) {
        let _ = fs::remove_file(files.stop());
        touch_used(&files);
        return Ok(());
    }
    if game::running(&config.game, files.afk().exists()).is_some() {
        return Err(StartError::Game);
    }
    fs::create_dir_all(&files.dir).map_err(|e| StartError::Failed(e.to_string()))?;
    let deadline = Instant::now() + Duration::from_secs(config.server.start_timeout_seconds);
    let poll = Duration::from_millis(config.server.health_poll_ms);
    let _lock = take_lock(&files, deadline, poll).ok_or(StartError::Timeout)?;
    if let Some(state) = read_state(&files) {
        if keeper_alive(&state) && same_server(&state, config, model) {
            let _ = fs::remove_file(files.stop());
            touch_used(&files);
            return wait_ready(config, &files, state.keeper, deadline);
        }
        wait_free(config, &files);
        stop(config);
    }
    let _ = fs::remove_file(files.stop());
    let _ = fs::remove_file(files.closed());
    touch_used(&files);
    let keeper = spawn_keeper(config_path, model).map_err(StartError::Failed)?;
    let deadline = deadline.max(Instant::now() + Duration::from_secs(config.server.start_timeout_seconds));
    wait_ready(config, &files, keeper, deadline)
}

pub fn set_afk(config: &Config, afk: bool) {
    let files = Files::new(config);
    let _ = fs::create_dir_all(&files.dir);
    if afk {
        let _ = fs::write(files.afk(), b"");
    } else {
        let _ = fs::remove_file(files.afk());
    }
}

pub fn should_close(config: &Config, files: &Files, idle: f64) -> Option<&'static str> {
    if files.stop().exists() && live_busy(files) == 0 {
        return Some("stop");
    }
    if game::running(&config.game, files.afk().exists()).is_some() {
        return Some(CLOSED_GAME);
    }
    if idle > config.keeper.keep_loaded_seconds as f64 && live_busy(files) == 0 {
        return Some("idle");
    }
    None
}

pub fn keeper(config: &Config, name: &str) -> i32 {
    let Some(model) = config.model(Some(name)) else {
        return 2;
    };
    let files = Files::new(config);
    let _ = fs::create_dir_all(&files.dir);
    let Ok(log) = File::create(files.log()) else {
        return 3;
    };
    let Ok(log_copy) = log.try_clone() else {
        return 3;
    };
    let args = server_args(config, model);
    let spawned = spawn::quiet(&config.server.exe)
        .args(&args)
        .stdin(Stdio::null())
        .stdout(Stdio::from(log))
        .stderr(Stdio::from(log_copy))
        .spawn();
    let mut child = match spawned {
        Ok(child) => child,
        Err(error) => {
            let _ = fs::write(files.log(), format!("{}: {error}", config.server.exe.display()));
            return 4;
        }
    };
    let me = std::process::id();
    let state = State {
        keeper: me,
        server: child.id(),
        model: model.name.clone(),
        port: config.server.port,
        started: now(),
        args,
    };
    let _ = write_atomic(&files.state(), &serde_json::to_string(&state).unwrap_or_default());
    let poll = Duration::from_secs(config.keeper.poll_seconds.max(1));
    loop {
        std::thread::sleep(poll);
        if !matches!(child.try_wait(), Ok(None)) {
            break;
        }
        if read_state(&files).is_none_or(|s| s.keeper != me) {
            break;
        }
        if let Some(reason) = should_close(config, &files, now() - last_used(&files)) {
            let _ = fs::write(files.closed(), reason);
            break;
        }
    }
    let _ = child.kill();
    let _ = child.wait();
    if read_state(&files).is_some_and(|s| s.keeper == me) {
        let _ = fs::remove_file(files.state());
    }
    let _ = fs::remove_file(files.stop());
    0
}

#[cfg(test)]
#[path = "server_tests.rs"]
mod tests;
