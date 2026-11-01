use std::collections::{HashMap, VecDeque};

use crate::config::Config;
use crate::model::{MB, Proc, Sample, gb, short_name, size};
use crate::score::{Facts, Fix, protected};

const SYSTEM_NAMES: [&str; 3] = ["system", "system idle process", "idle"];
const KEEP_SAMPLES: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub struct Lag {
    pub key: String,
    pub text: String,
    pub fix: Fix,
}

#[derive(Default)]
pub struct Finder {
    prev: Option<Sample>,
    cpu_since: Option<u64>,
    disk_since: Option<u64>,
    history: VecDeque<(u64, HashMap<u32, u64>)>,
}

fn cpu_pct(prev: &Sample, now: &Sample) -> Option<f32> {
    let (pb, pt) = prev.cpu?;
    let (nb, nt) = now.cpu?;
    let total = nt.checked_sub(pt).filter(|t| *t > 0)?;
    Some((nb.saturating_sub(pb) as f32 * 100.0 / total as f32).clamp(0.0, 100.0))
}

fn top_by<F: Fn(&Proc, &Proc) -> u64>(prev: &Sample, now: &Sample, delta: F) -> Option<(String, u64)> {
    let mut sums: HashMap<String, u64> = HashMap::new();
    for p in &now.procs {
        if SYSTEM_NAMES.contains(&p.name.to_lowercase().as_str()) {
            continue;
        }
        if let Some(old) = prev.procs.iter().find(|o| o.pid == p.pid && o.name == p.name) {
            *sums.entry(p.name.clone()).or_default() += delta(old, p);
        }
    }
    sums.into_iter().filter(|(_, v)| *v > 0).max_by_key(|(_, v)| *v)
}

impl Finder {
    pub fn observe(&mut self, s: &Sample, cfg: &Config) -> (Facts, Vec<Lag>) {
        let mut facts = Facts::default();
        let mut lags = Vec::new();
        let prev = self.prev.take();
        let now_s = s.at_ms / 1000;
        if let Some(prev) = &prev {
            facts.cpu_pct = cpu_pct(prev, s);
            let wall = s.at_ms.saturating_sub(prev.at_ms).max(1) as f32;
            let cores = s.cores.max(1) as f32;
            facts.top_cpu = top_by(prev, s, |o, n| n.cpu_ms.saturating_sub(o.cpu_ms))
                .map(|(name, ms)| (name, (ms as f32 * 100.0 / (wall * cores)).min(100.0)));
        }
        self.ram(s, cfg, &mut lags);
        self.growth(s, cfg, &mut lags);
        self.cpu(prev.as_ref(), &facts, cfg, now_s, &mut lags);
        self.disk(s, prev.as_ref(), cfg, now_s, &mut lags);
        self.prev = Some(s.clone());
        (facts, lags)
    }

    fn ram(&self, s: &Sample, cfg: &Config, out: &mut Vec<Lag>) {
        let pct = s.mem_pct();
        if pct <= cfg.ram_warn_pct {
            return;
        }
        let who = s
            .by_name()
            .into_iter()
            .next()
            .map_or(String::new(), |(n, b)| format!(": {} holds {}", short_name(&n), gb(b)));
        out.push(Lag { key: "ram".into(), text: format!("RAM at {pct:.0} %{who}"), fix: Fix::Free });
    }

    fn growth(&mut self, s: &Sample, cfg: &Config, out: &mut Vec<Lag>) {
        let now = s.at_ms;
        self.history.push_back((now, s.procs.iter().map(|p| (p.pid, p.rss)).collect()));
        while self.history.len() > KEEP_SAMPLES
            || self.history.front().is_some_and(|(t, _)| now - t > cfg.grow_secs * 1000)
        {
            self.history.pop_front();
        }
        let Some((_, old)) = self.history.front().filter(|_| self.history.len() > 1) else { return };
        let limit = cfg.grow_mb * MB;
        let mut sums: HashMap<String, (u64, Vec<u32>)> = HashMap::new();
        for p in &s.procs {
            if let Some(before) = old.get(&p.pid) {
                let e = sums.entry(p.name.to_lowercase()).or_default();
                e.0 += p.rss.saturating_sub(*before);
                e.1.push(p.pid);
            }
        }
        let worst = sums.into_iter().filter(|(_, (g, _))| *g >= limit).max_by_key(|(_, (g, _))| *g);
        if let Some((name, (grown, pids))) = worst {
            let fix = if protected(cfg, &name) { Fix::Free } else { Fix::Kill { name: name.clone(), pids } };
            out.push(Lag {
                key: format!("grow:{name}"),
                text: format!("{} grew by {} in {} s", short_name(&name), size(grown), cfg.grow_secs),
                fix,
            });
        }
    }

    fn cpu(&mut self, prev: Option<&Sample>, f: &Facts, cfg: &Config, now: u64, out: &mut Vec<Lag>) {
        let hot = f.cpu_pct.is_some_and(|p| p > cfg.cpu_warn_pct);
        if !hot {
            self.cpu_since = None;
            return;
        }
        let since = *self.cpu_since.get_or_insert(now);
        if now.saturating_sub(since) < cfg.busy_secs || prev.is_none() {
            return;
        }
        let pct = f.cpu_pct.unwrap_or_default();
        match &f.top_cpu {
            Some((name, share)) => {
                let fix = if protected(cfg, name) { Fix::Net } else { Fix::Lower { name: name.clone() } };
                out.push(Lag {
                    key: format!("cpu:{}", short_name(name)),
                    text: format!(
                        "CPU above {:.0} % for {} s: {} uses {share:.0} %",
                        cfg.cpu_warn_pct,
                        cfg.busy_secs,
                        short_name(name)
                    ),
                    fix,
                });
            }
            None => out.push(Lag {
                key: "cpu".into(),
                text: format!("CPU at {pct:.0} % for {} s", cfg.busy_secs),
                fix: Fix::Net,
            }),
        }
    }

    fn disk(&mut self, s: &Sample, prev: Option<&Sample>, cfg: &Config, now: u64, out: &mut Vec<Lag>) {
        if !s.disk_busy.is_some_and(|b| b >= cfg.disk_busy_pct) {
            self.disk_since = None;
            return;
        }
        let since = *self.disk_since.get_or_insert(now);
        if now.saturating_sub(since) < cfg.busy_secs {
            return;
        }
        let culprit = prev.and_then(|p| top_by(p, s, |o, n| n.io.saturating_sub(o.io)));
        match culprit {
            Some((name, bytes)) => {
                let fix = if protected(cfg, &name) { Fix::Net } else { Fix::Lower { name: name.clone() } };
                out.push(Lag {
                    key: format!("disk:{}", short_name(&name)),
                    text: format!(
                        "disk 100 % busy for {} s: {} moved {}",
                        cfg.busy_secs,
                        short_name(&name),
                        size(bytes)
                    ),
                    fix,
                });
            }
            None => out.push(Lag {
                key: "disk".into(),
                text: format!("disk 100 % busy for {} s", cfg.busy_secs),
                fix: Fix::Net,
            }),
        }
    }
}

#[derive(Default)]
pub struct Cooldown {
    last: HashMap<String, u64>,
}

impl Cooldown {
    pub fn allow(&mut self, key: &str, now: u64, gap: u64) -> bool {
        match self.last.get(key) {
            Some(at) if now.saturating_sub(*at) < gap => false,
            _ => {
                self.last.insert(key.to_string(), now);
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::proc;
    use crate::model::GB;

    fn at(ms: u64, used_pct: u64, busy: u64, procs: Vec<Proc>) -> Sample {
        Sample {
            at_ms: ms,
            mem_total: 100 * GB,
            mem_used: used_pct * GB,
            cpu: Some((busy, ms * 10)),
            cores: 4,
            procs,
            ..Sample::default()
        }
    }

    fn with_cpu(pid: u32, name: &str, ms: u64) -> Proc {
        Proc { cpu_ms: ms, ..proc(pid, 0, name, GB) }
    }

    #[test]
    fn ram_above_the_limit_names_the_biggest_program() {
        let mut f = Finder::default();
        let (_, quiet) = f.observe(&at(0, 80, 0, vec![proc(1, 0, "brave.exe", 6 * GB)]), &Config::default());
        assert!(quiet.is_empty());
        let (_, lags) = f.observe(&at(5000, 87, 0, vec![proc(1, 0, "brave.exe", 6 * GB)]), &Config::default());
        assert_eq!(lags.len(), 1);
        assert_eq!(lags[0].key, "ram");
        assert!(lags[0].text.contains("87 %") && lags[0].text.contains("brave holds 6.0 GB"), "{}", lags[0].text);
        assert_eq!(lags[0].fix, Fix::Free);
    }

    #[test]
    fn cpu_must_stay_high_for_the_whole_window_before_it_counts() {
        let cfg = Config::default();
        let mut f = Finder::default();
        let mut found = Vec::new();
        for i in 0..=8u64 {
            let ms = i * 5000;
            let s = at(ms, 40, ms * 9, vec![with_cpu(7, "cl.exe", ms * 3), with_cpu(8, "idle", 0)]);
            let (facts, lags) = f.observe(&s, &cfg);
            if i > 0 {
                assert!(facts.cpu_pct.unwrap() > 85.0);
                assert_eq!(facts.top_cpu.as_ref().map(|t| t.0.as_str()), Some("cl.exe"));
            }
            found.push(lags.iter().any(|l| l.key == "cpu:cl"));
        }
        assert_eq!(found, [false, false, false, false, false, false, false, true, true]);
    }

    #[test]
    fn a_pause_in_the_load_resets_the_cpu_window() {
        let cfg = Config { busy_secs: 10, ..Config::default() };
        let mut f = Finder::default();
        let load = |i: u64, busy: u64| at(i * 5000, 40, busy, vec![with_cpu(7, "cl.exe", i * 9000)]);
        let steps = [(0, 0), (1, 45_000), (2, 90_000), (3, 135_000), (4, 135_500), (5, 180_500), (6, 225_500)];
        let hits: Vec<bool> = steps.iter().map(|(i, b)| !f.observe(&load(*i, *b), &cfg).1.is_empty()).collect();
        assert_eq!(hits, [false, false, false, true, false, false, false]);
    }

    #[test]
    fn a_program_that_grows_fast_is_named() {
        let cfg = Config::default();
        let mut f = Finder::default();
        let mut last = Vec::new();
        for i in 0..6u64 {
            let rss = GB + i * 400 * MB;
            last =
                f.observe(&at(i * 5000, 40, 0, vec![proc(5, 0, "leaky.exe", rss), proc(6, 0, "calm.exe", GB)]), &cfg).1;
        }
        let grow = last.iter().find(|l| l.key == "grow:leaky.exe").expect("growth lag");
        assert!(grow.text.contains("leaky grew by"), "{}", grow.text);
        assert!(matches!(&grow.fix, Fix::Kill { pids, .. } if pids == &[5]));
        assert!(!last.iter().any(|l| l.key.contains("calm")));
    }

    #[test]
    fn a_protected_program_that_grows_is_never_offered_for_closing() {
        let cfg = Config::default();
        let mut f = Finder::default();
        let mut last = Vec::new();
        for i in 0..6u64 {
            last = f.observe(&at(i * 5000, 40, 0, vec![proc(5, 0, "javaw.exe", GB + i * 400 * MB)]), &cfg).1;
        }
        assert_eq!(last.iter().find(|l| l.key.starts_with("grow:")).map(|l| l.fix.clone()), Some(Fix::Free));
    }

    #[test]
    fn a_busy_disk_names_the_program_moving_the_most_data() {
        let cfg = Config { busy_secs: 10, ..Config::default() };
        let mut f = Finder::default();
        let mut last = Vec::new();
        for i in 0..4u64 {
            let mut s = at(
                i * 5000,
                40,
                0,
                vec![
                    Proc { io: i * 500 * MB, ..proc(3, 0, "copy.exe", GB) },
                    Proc { io: i * MB, ..proc(4, 0, "tiny.exe", GB) },
                ],
            );
            s.disk_busy = Some(100.0);
            last = f.observe(&s, &cfg).1;
        }
        let lag = last.iter().find(|l| l.key == "disk:copy").expect("disk lag");
        assert!(lag.text.contains("copy moved 500 MB"), "{}", lag.text);
    }

    #[test]
    fn the_same_message_comes_once_per_gap() {
        let mut c = Cooldown::default();
        assert!(c.allow("ram", 0, 1800));
        assert!(!c.allow("ram", 1799, 1800));
        assert!(c.allow("cpu", 10, 1800));
        assert!(c.allow("ram", 1800, 1800));
        assert!(!c.allow("ram", 1801, 1800));
    }
}
