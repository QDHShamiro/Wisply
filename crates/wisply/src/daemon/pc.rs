use std::env;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::Duration;

use wisply_core::{Call, Fail};
use wisply_pc::config::Config;
use wisply_pc::{Pc, notify, plat};

use super::{Daemon, Out, game};

const OFF_CHECK: Duration = Duration::from_secs(10);

static PC: OnceLock<Mutex<Pc>> = OnceLock::new();

fn user_home() -> PathBuf {
    env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(PathBuf::from).unwrap_or_default()
}

fn make(home: &Path) -> Mutex<Pc> {
    Mutex::new(Pc::new(home, &user_home(), plat::system(), Box::new(notify::send)))
}

fn lock(home: &Path) -> MutexGuard<'static, Pc> {
    PC.get_or_init(|| make(home)).lock().unwrap_or_else(|e| e.into_inner())
}

pub fn start(home: PathBuf) {
    let _ = thread::Builder::new().name("pc".into()).spawn(move || {
        drop(lock(&home));
        loop {
            let cfg = Config::load(&home);
            if !cfg.enabled {
                thread::sleep(OFF_CHECK);
                continue;
            }
            let playing = game::running();
            let tick = panic::catch_unwind(AssertUnwindSafe(|| lock(&home).tick(&playing)));
            if tick.is_err() {
                eprintln!("wisply pc: a check failed, see the panic above");
            }
            thread::sleep(Duration::from_secs(cfg.tick_secs));
        }
    });
}

pub fn run(d: &Daemon, c: &Call) -> Result<Out, Fail> {
    let mut args = c.pos.clone();
    if c.on("dry") {
        args.push("--dry".into());
    }
    if args.first().is_some_and(|a| a.eq_ignore_ascii_case("updates")) {
        return Ok(Out::Text(wisply_pc::health::updates((wisply_pc::model::secs() / 86_400) as i64)));
    }
    lock(&d.home).command(&args).map(Out::Text).map_err(|(text, hint)| Fail::hint(text, hint))
}

pub fn status_line(d: &Daemon) -> Option<String> {
    let pc = lock(&d.home);
    let view = pc.last.as_ref()?;
    Some(format!("pc | {}/100 | acting {}", view.report.score, if pc.cfg.act { "on" } else { "off" }))
}
