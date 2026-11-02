use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::json;

use crate::bg::Bg;
use crate::clean;
use crate::config::{Config, dir};
use crate::lag::{Cooldown, Finder, Lag};
use crate::log::Log;
use crate::mode::{Engine, Manual, Mode, decide};
use crate::model::{GB, Sample, secs};
use crate::net;
use crate::plat::Platform;
use crate::score::{Brake, Facts, Fix, Report, evaluate};
use crate::startup;

const RELOAD_EVERY: u64 = 12;
const AUTOSTART_EVERY: u64 = 600;
const PING_EVERY: u64 = 60;
const STATUS_EVERY: u64 = 30;
const CLEAN_RETRY: u64 = 3600;
const MAX_SUGGESTIONS: usize = 8;
const STATUS_FILE: &str = "status.json";
const CLEAN_FILE: &str = "last-clean";

pub type Notifier = Box<dyn FnMut(&str, &str) + Send>;

pub struct View {
    pub sample: Sample,
    pub facts: Facts,
    pub report: Report,
    pub game: String,
    pub mode: Mode,
}

pub struct Suggestion {
    pub fix: Fix,
    pub text: String,
}

pub struct Pc {
    pub home: PathBuf,
    pub user: PathBuf,
    pub cfg: Config,
    pub plat: Box<dyn Platform>,
    pub modes: Engine,
    pub log: Log,
    pub last: Option<View>,
    pub manual: Option<Manual>,
    pub suggestions: Vec<Suggestion>,
    pub probes: bool,
    pub env_override: Option<clean::Env>,
    pub ticks: u64,
    pub spent: Duration,
    finder: Finder,
    cool: Cooldown,
    autostarts: Bg<usize>,
    ping: Bg<(String, Option<u64>)>,
    cleaning: Arc<AtomicBool>,
    clean_try: u64,
    status: (String, u64),
    notifier: Notifier,
}

impl Pc {
    pub fn new(home: &Path, user: &Path, plat: Box<dyn Platform>, notifier: Notifier) -> Pc {
        let mut pc = Pc {
            home: home.into(),
            user: user.into(),
            cfg: Config::load(home),
            plat,
            modes: Engine::load(home),
            log: Log::new(&dir(home)),
            last: None,
            manual: None,
            suggestions: Vec::new(),
            probes: true,
            env_override: None,
            ticks: 0,
            spent: Duration::ZERO,
            finder: Finder::default(),
            cool: Cooldown::default(),
            autostarts: Bg::new(),
            ping: Bg::new(),
            cleaning: Arc::new(AtomicBool::new(false)),
            clean_try: 0,
            status: (String::new(), 0),
            notifier,
        };
        if pc.modes.active() {
            pc.modes.restore_all(&mut *pc.plat);
        }
        pc
    }

    pub fn facts(&mut self, mut facts: Facts) -> Facts {
        let now = secs();
        if !self.probes {
            return facts;
        }
        let (user, host, wait) = (self.user.clone(), self.cfg.ping_host.clone(), self.cfg.ping_ms);
        self.autostarts.refresh(now, AUTOSTART_EVERY, move || startup::count(&user));
        self.ping.refresh(now, PING_EVERY, move || {
            let ms = net::ping(&host, wait);
            (host, ms)
        });
        facts.autostarts = self.autostarts.get();
        facts.ping = self.ping.get();
        facts
    }

    pub fn warm(&mut self) {
        if !self.probes {
            return;
        }
        let now = secs();
        if self.ping.get().is_none() {
            let ms = net::ping(&self.cfg.ping_host, self.cfg.ping_ms);
            self.ping.put((self.cfg.ping_host.clone(), ms), now);
        }
        if self.autostarts.get().is_none() {
            self.autostarts.put(startup::count(&self.user), now);
        }
    }

    pub fn tick(&mut self, game: &str) {
        let started = Instant::now();
        if self.ticks.is_multiple_of(RELOAD_EVERY) {
            self.cfg = Config::load(&self.home);
        }
        let sample = self.plat.sample();
        let (facts, lags) = self.finder.observe(&sample, &self.cfg);
        let facts = self.facts(facts);
        let mode = decide(!game.is_empty(), sample.idle_secs, self.cfg.afk_secs, self.manual);
        let name = (!game.is_empty()).then_some(game);
        self.modes.tick(&mut *self.plat, &sample, &self.cfg, name, mode);
        let report = evaluate(&sample, &facts, &self.cfg);
        self.write_status(&report);
        self.warn(&lags, &report);
        self.auto_clean(&sample, mode);
        self.last = Some(View { sample, facts, report, game: game.into(), mode });
        self.ticks += 1;
        self.spent += started.elapsed();
    }

    fn write_status(&mut self, r: &Report) {
        if !self.cfg.statusline {
            return;
        }
        let now = secs();
        let brake = r.brakes.first().map_or(String::new(), |b| b.text.clone());
        let key = format!("{}|{brake}", r.score);
        if key == self.status.0 && now.saturating_sub(self.status.1) < STATUS_EVERY {
            return;
        }
        self.status = (key, now);
        let row = json!({"t": now, "score": r.score, "brake": brake});
        let folder = dir(&self.home);
        let _ = fs::create_dir_all(&folder);
        let _ = fs::write(folder.join(STATUS_FILE), row.to_string());
    }

    pub fn remember(&mut self, fix: Fix, text: &str) -> usize {
        if let Some(i) = self.suggestions.iter().position(|s| s.fix == fix) {
            return i + 1;
        }
        self.suggestions.push(Suggestion { fix, text: text.into() });
        if self.suggestions.len() > MAX_SUGGESTIONS {
            self.suggestions.remove(0);
        }
        self.suggestions.len()
    }

    pub fn set_suggestions(&mut self, brakes: &[Brake]) {
        let keep: Vec<Suggestion> = std::mem::take(&mut self.suggestions)
            .into_iter()
            .filter(|s| !brakes.iter().any(|b| b.fix == s.fix))
            .collect();
        self.suggestions = brakes.iter().map(|b| Suggestion { fix: b.fix.clone(), text: b.text.clone() }).collect();
        self.suggestions.extend(keep);
        self.suggestions.truncate(MAX_SUGGESTIONS);
    }

    fn warn(&mut self, lags: &[Lag], r: &Report) {
        if !self.cfg.warn {
            return;
        }
        let now = secs();
        for lag in lags {
            if !self.cool.allow(&lag.key, now, self.cfg.notify_gap_secs) {
                continue;
            }
            self.set_suggestions(&r.brakes);
            let n = self.remember(lag.fix.clone(), &lag.text);
            self.log.add("warning", &lag.text, &format!("fix {n}"), 0);
            if self.cfg.notify {
                (self.notifier)("Wisply PC", &format!("{} - wisply pc fix {n}", lag.text));
            }
        }
    }

    fn last_clean(&self) -> u64 {
        fs::read_to_string(dir(&self.home).join(CLEAN_FILE)).ok().and_then(|t| t.trim().parse().ok()).unwrap_or(0)
    }

    fn auto_clean(&mut self, s: &Sample, mode: Mode) {
        if !self.cfg.act || mode == Mode::Play || self.cleaning.load(Ordering::Acquire) {
            return;
        }
        let now = secs();
        let low = s.disks.iter().any(|d| d.total > 0 && d.free < self.cfg.clean_min_free_gb * GB);
        let idle = s.idle_secs.is_some_and(|i| i >= self.cfg.afk_secs);
        let weekly = idle && now.saturating_sub(self.last_clean()) >= self.cfg.clean_weekly_secs;
        if !(low || weekly) || now.saturating_sub(self.clean_try) < CLEAN_RETRY {
            return;
        }
        self.clean_try = now;
        self.cleaning.store(true, Ordering::Release);
        let (home, env, cfg, procs) = (self.home.clone(), self.clean_env(), self.cfg.clone(), s.procs.clone());
        let flag = Arc::clone(&self.cleaning);
        let spawned = thread::Builder::new().name("pc-clean".into()).spawn(move || {
            run_clean(&home, &env, &cfg, &procs);
            flag.store(false, Ordering::Release);
        });
        if spawned.is_err() {
            self.cleaning.store(false, Ordering::Release);
        }
    }

    pub fn clean_env(&self) -> clean::Env {
        self.env_override.clone().unwrap_or_else(|| clean::env_here(&self.user, &self.home, &self.cfg, secs()))
    }

    pub fn fresh(&mut self) -> &View {
        if self.last.is_none() {
            let sample = self.plat.sample();
            let facts = self.facts(Facts::default());
            let report = evaluate(&sample, &facts, &self.cfg);
            self.last = Some(View { sample, facts, report, game: String::new(), mode: Mode::Off });
        }
        self.last.as_ref().expect("view")
    }
}

pub fn run_clean(home: &Path, env: &clean::Env, cfg: &Config, procs: &[crate::model::Proc]) -> clean::Outcome {
    let items = clean::plan(env, cfg, procs);
    let log = Log::new(&dir(home));
    let out = clean::execute(&items, env, cfg, procs, &log);
    let folder = dir(home);
    let _ = fs::create_dir_all(&folder);
    let _ = fs::write(folder.join(CLEAN_FILE), secs().to_string());
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::{Fake, proc};
    use crate::model::Disk;
    use std::sync::Mutex;

    fn home(tag: &str) -> PathBuf {
        let h = std::env::temp_dir().join(format!("wpc-eng-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&h);
        h
    }

    fn pc(h: &Path, fake: Fake, sink: Arc<Mutex<Vec<String>>>) -> Pc {
        let n = Box::new(move |t: &str, b: &str| sink.lock().unwrap().push(format!("{t}|{b}")));
        let mut p = Pc::new(h, &h.join("user"), Box::new(fake), n);
        p.probes = false;
        p
    }

    fn crowded() -> Fake {
        let mut f = Fake::with(vec![proc(1, 0, "brave.exe", 6 * GB), proc(2, 0, "other.exe", GB)]);
        f.sample.mem_total = 100 * GB;
        f.sample.mem_used = 90 * GB;
        f.sample.disks = vec![Disk { mount: "C:".into(), total: 500 * GB, free: 200 * GB }];
        f
    }

    #[test]
    fn a_lag_warns_once_with_a_fix_number_and_writes_the_status_file() {
        let h = home("warn");
        let sink = Arc::new(Mutex::new(Vec::new()));
        let mut p = pc(&h, crowded(), Arc::clone(&sink));
        p.tick("");
        p.tick("");
        let got = sink.lock().unwrap().clone();
        assert_eq!(got.len(), 1, "{got:?}");
        assert!(got[0].starts_with("Wisply PC|RAM at 90 %") && got[0].ends_with("wisply pc fix 1"), "{got:?}");
        let status = fs::read_to_string(dir(&h).join(STATUS_FILE)).unwrap();
        assert!(status.contains("\"score\"") && status.contains("RAM 90 %"), "{status}");
        assert!(p.log.show(5).contains("warning"));
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn warnings_can_be_switched_off_and_acting_stays_off_by_default() {
        let h = home("quiet");
        fs::create_dir_all(dir(&h)).unwrap();
        fs::write(dir(&h).join("config.json"), r#"{"warn":false}"#).unwrap();
        let sink = Arc::new(Mutex::new(Vec::new()));
        let mut p = pc(&h, crowded(), Arc::clone(&sink));
        p.tick("game.exe");
        assert!(sink.lock().unwrap().is_empty());
        assert!(!p.modes.active());
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn the_status_file_is_not_rewritten_every_tick() {
        let h = home("status");
        let sink = Arc::new(Mutex::new(Vec::new()));
        let mut p = pc(&h, crowded(), sink);
        p.tick("");
        let file = dir(&h).join(STATUS_FILE);
        let first = fs::read_to_string(&file).unwrap();
        fs::write(&file, "marker").unwrap();
        p.tick("");
        assert_eq!(fs::read_to_string(&file).unwrap(), "marker");
        assert!(first.contains("score"));
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn remembered_fixes_get_stable_numbers() {
        let h = home("num");
        let mut p = pc(&h, crowded(), Arc::new(Mutex::new(Vec::new())));
        assert_eq!(p.remember(Fix::Free, "a"), 1);
        assert_eq!(p.remember(Fix::Clean, "b"), 2);
        assert_eq!(p.remember(Fix::Free, "a"), 1);
        fs::remove_dir_all(&h).ok();
    }
}
