use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::config::{Config, dir};
use crate::log::Log;
use crate::model::{MB, Proc, Sample, short_name};
use crate::plat::Platform;
use crate::score::protected;

const FILE: &str = "mode-state.json";
const RESUME_MARGIN_PCT: f32 = 8.0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Mode {
    Off,
    Play,
    Afk,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Manual {
    Play,
    Afk,
}

pub fn decide(game: bool, idle: Option<u64>, afk_secs: u64, manual: Option<Manual>) -> Mode {
    match manual {
        Some(Manual::Afk) => Mode::Afk,
        Some(Manual::Play) => Mode::Play,
        None if !game => Mode::Off,
        None => match idle {
            Some(s) if s >= afk_secs => Mode::Afk,
            _ => Mode::Play,
        },
    }
}

impl Mode {
    pub fn word(self) -> &'static str {
        match self {
            Mode::Off => "off",
            Mode::Play => "playing",
            Mode::Afk => "afk",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct Held {
    pid: u32,
    name: String,
    value: i32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    prio: Vec<Held>,
    suspended: Vec<Held>,
    plan: Option<String>,
    trimmed_at: u64,
}

impl State {
    fn empty(&self) -> bool {
        self.prio.is_empty() && self.suspended.is_empty() && self.plan.is_none()
    }

    fn held(list: &[Held]) -> Value {
        Value::Array(list.iter().map(|h| json!({"pid": h.pid, "name": h.name, "value": h.value})).collect())
    }

    fn read(v: &Value, key: &str) -> Vec<Held> {
        let rows = v.get(key).and_then(Value::as_array).cloned().unwrap_or_default();
        rows.iter()
            .filter_map(|r| {
                Some(Held {
                    pid: r.get("pid")?.as_u64()? as u32,
                    name: r.get("name")?.as_str()?.to_string(),
                    value: r.get("value").and_then(Value::as_i64).unwrap_or(0) as i32,
                })
            })
            .collect()
    }

    fn to_json(&self) -> Value {
        json!({"prio": Self::held(&self.prio), "suspended": Self::held(&self.suspended), "plan": self.plan})
    }

    fn from_json(v: &Value) -> State {
        State {
            prio: Self::read(v, "prio"),
            suspended: Self::read(v, "suspended"),
            plan: v.get("plan").and_then(Value::as_str).map(String::from),
            trimmed_at: 0,
        }
    }
}

pub struct Engine {
    file: PathBuf,
    state: State,
    log: Log,
}

fn same(sample: &Sample, h: &Held) -> bool {
    sample.proc(h.pid).is_some_and(|p| p.name.to_lowercase() == h.name)
}

impl Engine {
    pub fn load(home: &Path) -> Engine {
        let folder = dir(home);
        let file = folder.join(FILE);
        let state = fs::read_to_string(&file)
            .ok()
            .and_then(|t| serde_json::from_str::<Value>(&t).ok())
            .map(|v| State::from_json(&v))
            .unwrap_or_default();
        Engine { file, state, log: Log::new(&folder) }
    }

    pub fn active(&self) -> bool {
        !self.state.empty()
    }

    pub fn summary(&self) -> String {
        let s = &self.state;
        format!(
            "{} lowered or raised, {} paused, power plan {}",
            s.prio.len(),
            s.suspended.len(),
            if s.plan.is_some() { "changed" } else { "untouched" }
        )
    }

    fn save(&self) {
        if self.state.empty() {
            let _ = fs::remove_file(&self.file);
            return;
        }
        if let Some(parent) = self.file.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::write(&self.file, self.state.to_json().to_string());
    }

    pub fn restore_all(&mut self, p: &mut dyn Platform) {
        let sample = p.sample();
        for h in std::mem::take(&mut self.state.suspended) {
            if same(&sample, &h) && p.resume(h.pid) {
                self.log.add("resume", &format!("{} continued", short_name(&h.name)), &format!("pid {}", h.pid), 0);
            }
        }
        for h in std::mem::take(&mut self.state.prio) {
            if same(&sample, &h) && p.set_priority(h.pid, h.value) {
                self.log.add(
                    "priority",
                    &format!("{} back to normal", short_name(&h.name)),
                    &format!("pid {}", h.pid),
                    0,
                );
            }
        }
        if let Some(plan) = self.state.plan.take()
            && p.set_power_plan(&plan)
        {
            self.log.add("power", "power plan restored", &plan, 0);
        }
        self.save();
    }

    fn drop_gone(&mut self, sample: &Sample) {
        self.state.prio.retain(|h| same(sample, h));
        self.state.suspended.retain(|h| same(sample, h));
    }

    fn lower(&mut self, p: &mut dyn Platform, proc: &Proc, level: i32, why: &str) {
        if self.state.prio.iter().any(|h| h.pid == proc.pid) {
            return;
        }
        let Some(orig) = p.priority(proc.pid) else { return };
        if orig == level {
            return;
        }
        let held = Held { pid: proc.pid, name: proc.name.to_lowercase(), value: orig };
        self.state.prio.push(held);
        self.save();
        if p.set_priority(proc.pid, level) {
            self.log.add("priority", &format!("{} {why}", short_name(&proc.name)), &format!("pid {}", proc.pid), 0);
        } else {
            self.state.prio.pop();
            self.save();
        }
    }

    fn power(&mut self, p: &mut dyn Platform) {
        if self.state.plan.is_some() {
            return;
        }
        let (Some(now), Some(fast)) = (p.power_plan(), p.fast_power_plan()) else { return };
        if now == fast {
            return;
        }
        self.state.plan = Some(now);
        self.save();
        if p.set_power_plan(&fast) {
            self.log.add("power", "high performance plan on", &fast, 0);
        } else {
            self.state.plan = None;
            self.save();
        }
    }

    fn suspend_for_ram(&mut self, p: &mut dyn Platform, sample: &Sample, cfg: &Config, spare: &[u32]) {
        let pct = sample.mem_pct();
        if pct > cfg.ram_suspend_pct {
            let victim = sample
                .procs
                .iter()
                .filter(|x| {
                    builder(cfg, x) && !spare.contains(&x.pid) && !self.state.suspended.iter().any(|h| h.pid == x.pid)
                })
                .max_by_key(|x| x.rss);
            if let Some(v) = victim {
                self.state.suspended.push(Held { pid: v.pid, name: v.name.to_lowercase(), value: 0 });
                self.save();
                if p.suspend(v.pid) {
                    self.log.add(
                        "pause",
                        &format!("{} paused, RAM at {pct:.0} %", short_name(&v.name)),
                        &format!("pid {}", v.pid),
                        0,
                    );
                } else {
                    self.state.suspended.pop();
                    self.save();
                }
            }
        } else if pct < cfg.ram_suspend_pct - RESUME_MARGIN_PCT {
            for h in std::mem::take(&mut self.state.suspended) {
                if same(sample, &h) && p.resume(h.pid) {
                    self.log.add("resume", &format!("{} continued", short_name(&h.name)), &format!("pid {}", h.pid), 0);
                }
            }
            self.save();
        }
    }

    fn trim_idle(&mut self, p: &mut dyn Platform, sample: &Sample, cfg: &Config, spare: &[u32], game: &[String]) {
        let now = sample.at_ms;
        if self.state.trimmed_at != 0 && now.saturating_sub(self.state.trimmed_at) < cfg.trim_every_secs * 1000 {
            return;
        }
        self.state.trimmed_at = now.max(1);
        let floor = cfg.trim_min_mb * MB;
        for x in &sample.procs {
            let skip = x.rss < floor
                || spare.contains(&x.pid)
                || protected(cfg, &x.name)
                || game.contains(&short_name(&x.name))
                || self.state.suspended.iter().any(|h| h.pid == x.pid);
            if !skip && p.trim(x.pid) {
                self.log.add(
                    "memory",
                    &format!("{} memory released", short_name(&x.name)),
                    &format!("pid {}", x.pid),
                    x.rss,
                );
            }
        }
    }

    pub fn tick(&mut self, p: &mut dyn Platform, sample: &Sample, cfg: &Config, game: Option<&str>, mode: Mode) {
        if !cfg.act || mode != Mode::Play {
            if self.active() {
                self.restore_all(p);
            }
            self.state.trimmed_at = 0;
            return;
        }
        self.drop_gone(sample);
        let mut spare = sample.foreground.map(|f| sample.descendants(f)).unwrap_or_default();
        spare.push(std::process::id());
        let game_names: Vec<String> = game.map(|g| cfg.game_procs(g)).unwrap_or_default();
        let levels = p.levels();
        self.power(p);
        for x in &sample.procs {
            let short = short_name(&x.name);
            if game_names.contains(&short) {
                self.lower(p, x, levels.boost, "raised for the game");
            } else if spare.contains(&x.pid) || protected(cfg, &x.name) {
                continue;
            } else if builder(cfg, x) {
                self.lower(p, x, levels.lowest, "lowered while you play");
            } else if cfg.background.contains(&short) {
                self.lower(p, x, levels.low, "quieter while you play");
            }
        }
        self.suspend_for_ram(p, sample, cfg, &spare);
        self.trim_idle(p, sample, cfg, &spare, &game_names);
    }
}

pub fn builder(cfg: &Config, p: &Proc) -> bool {
    !protected(cfg, &p.name) && cfg.builders.contains(&short_name(&p.name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::{BOOST, Fake, LOW, LOWEST, NORMAL, proc};
    use crate::model::GB;

    fn home(tag: &str) -> PathBuf {
        let h = std::env::temp_dir().join(format!("wpc-mode-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&h);
        h
    }

    fn acting() -> Config {
        Config { act: true, trim_min_mb: 100, ..Config::default() }
    }

    fn world() -> Fake {
        let mut f = Fake::with(vec![
            proc(1, 0, "wt.exe", 100 * MB),
            proc(2, 1, "claude.exe", 200 * MB),
            proc(3, 0, "claude.exe", 200 * MB),
            proc(4, 0, "cargo.exe", 300 * MB),
            proc(5, 0, "game.exe", GB),
            proc(6, 0, "OneDrive.exe", 400 * MB),
            proc(7, 0, "javaw.exe", 2 * GB),
            proc(8, 0, "notes.exe", 50 * MB),
        ]);
        f.sample.foreground = Some(1);
        f.sample.cores = 4;
        f
    }

    fn snap(f: &mut Fake, ms: u64) -> Sample {
        f.sample.at_ms = ms;
        f.sample()
    }

    #[test]
    fn the_mode_follows_game_input_and_hand_switches() {
        assert_eq!(decide(false, Some(0), 300, None), Mode::Off);
        assert_eq!(decide(true, Some(10), 300, None), Mode::Play);
        assert_eq!(decide(true, Some(299), 300, None), Mode::Play);
        assert_eq!(decide(true, Some(300), 300, None), Mode::Afk);
        assert_eq!(decide(true, Some(4000), 300, None), Mode::Afk);
        assert_eq!(decide(true, Some(0), 300, None), Mode::Play);
        assert_eq!(decide(true, None, 300, None), Mode::Play);
        assert_eq!(decide(false, None, 300, Some(Manual::Play)), Mode::Play);
        assert_eq!(decide(true, Some(0), 300, Some(Manual::Afk)), Mode::Afk);
    }

    #[test]
    fn playing_lowers_builders_but_not_the_foreground_session_or_protected_programs() {
        let h = home("lower");
        let mut e = Engine::load(&h);
        let mut f = world();
        let s = snap(&mut f, 1000);
        e.tick(&mut f, &s, &acting(), Some("game.exe"), Mode::Play);
        assert_eq!(f.prio[&3], LOWEST);
        assert_eq!(f.prio[&4], LOWEST);
        assert_eq!(f.prio[&2], NORMAL);
        assert_eq!(f.prio[&5], BOOST);
        assert_eq!(f.prio[&6], LOW);
        assert_eq!(f.prio[&7], NORMAL);
        assert_eq!(f.prio[&8], NORMAL);
        assert_eq!(f.plan, "fast");
        assert!(
            f.trimmed.contains(&3) && !f.trimmed.contains(&2) && !f.trimmed.contains(&5) && !f.trimmed.contains(&7)
        );
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn nothing_changes_while_acting_is_off() {
        let h = home("off");
        let mut e = Engine::load(&h);
        let mut f = world();
        let s = snap(&mut f, 1000);
        e.tick(&mut f, &s, &Config::default(), Some("game.exe"), Mode::Play);
        assert!(f.calls.is_empty() && !e.active());
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn going_afk_or_leaving_the_game_restores_everything_exactly() {
        for end in [Mode::Afk, Mode::Off] {
            let h = home("restore");
            let mut e = Engine::load(&h);
            let mut f = world();
            f.prio.insert(4, 7);
            let before = f.prio.clone();
            let s = snap(&mut f, 1000);
            e.tick(&mut f, &s, &acting(), Some("game.exe"), Mode::Play);
            assert!(e.active() && h.join("pc").join(FILE).is_file());
            let s = snap(&mut f, 6000);
            e.tick(&mut f, &s, &acting(), Some("game.exe"), end);
            assert_eq!(f.prio, before);
            assert_eq!(f.plan, "balanced");
            assert!(!e.active() && !h.join("pc").join(FILE).exists());
            fs::remove_dir_all(&h).ok();
        }
    }

    #[test]
    fn a_crashed_daemon_restores_from_the_file_at_the_next_start() {
        let h = home("crash");
        let mut f = world();
        {
            let mut e = Engine::load(&h);
            let s = snap(&mut f, 1000);
            f.sample.mem_used = 95;
            let s2 = Sample { mem_used: 95, ..s };
            e.tick(&mut f, &s2, &acting(), Some("game.exe"), Mode::Play);
            assert!(!f.suspended.is_empty() && f.prio[&4] == LOWEST);
        }
        let mut again = Engine::load(&h);
        assert!(again.active());
        again.restore_all(&mut f);
        assert!(f.suspended.is_empty());
        assert_eq!(f.prio[&4], NORMAL);
        assert_eq!(f.plan, "balanced");
        assert!(!again.active());
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn a_reused_process_id_is_never_touched_on_restore() {
        let h = home("reuse");
        let mut f = world();
        let mut e = Engine::load(&h);
        let s = snap(&mut f, 1000);
        e.tick(&mut f, &s, &acting(), Some("game.exe"), Mode::Play);
        f.sample.procs.iter_mut().find(|p| p.pid == 4).unwrap().name = "other.exe".into();
        f.calls.clear();
        e.restore_all(&mut f);
        assert!(!f.calls.iter().any(|c| c.starts_with("prio 4 ")));
        assert_eq!(f.prio[&3], NORMAL);
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn scarce_memory_pauses_the_biggest_builder_and_lets_it_go_again() {
        let h = home("ram");
        let mut e = Engine::load(&h);
        let mut f = world();
        f.sample.mem_used = 95;
        let s = snap(&mut f, 1000);
        e.tick(&mut f, &s, &acting(), Some("game.exe"), Mode::Play);
        assert_eq!(f.suspended, [4]);
        let s = snap(&mut f, 6000);
        e.tick(&mut f, &s, &acting(), Some("game.exe"), Mode::Play);
        assert_eq!(f.suspended.len(), 2);
        f.sample.mem_used = 70;
        let s = snap(&mut f, 11_000);
        e.tick(&mut f, &s, &acting(), Some("game.exe"), Mode::Play);
        assert!(f.suspended.is_empty());
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn the_game_mode_never_pauses_the_foreground_session_or_the_game() {
        let h = home("spare");
        let mut e = Engine::load(&h);
        let mut f = world();
        f.sample.mem_used = 99;
        for i in 0..6 {
            let s = snap(&mut f, 1000 + i * 5000);
            e.tick(&mut f, &s, &acting(), Some("game.exe"), Mode::Play);
        }
        assert!(f.suspended.iter().all(|p| [3, 4].contains(p)), "{:?}", f.suspended);
        assert!(!f.suspended.contains(&2) && !f.suspended.contains(&5) && !f.suspended.contains(&7));
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn minecraft_is_raised_by_its_java_name_and_restored() {
        let h = home("mc");
        let mut e = Engine::load(&h);
        let mut f = world();
        let s = snap(&mut f, 1000);
        e.tick(&mut f, &s, &acting(), Some("minecraft"), Mode::Play);
        assert_eq!(f.prio[&7], BOOST);
        let s = snap(&mut f, 2000);
        e.tick(&mut f, &s, &acting(), None, Mode::Off);
        assert_eq!(f.prio[&7], NORMAL);
        assert!(!f.trimmed.contains(&7));
        fs::remove_dir_all(&h).ok();
    }
}
