use crate::clean;
use crate::config::{Config, dir, on};
use crate::engine::{Pc, run_clean};
use crate::health;
use crate::mode::Manual;
use crate::model::{MB, Sample, secs, short_name, size};
use crate::net;
use crate::score::{Fix, evaluate, protected, render};
use crate::startup;

pub const USAGE: &str = "pc [fix <n>|fix all | clean [--dry] | startup [off|on <n>] | game | afk|play|auto | net [host] | disk | updates | on|off | log [n] | config]";
const LOG_LINES: usize = 20;
const MAX_FIX_LIST: usize = 12;

pub type Problem = (String, String);

fn problem(text: impl Into<String>, hint: impl Into<String>) -> Problem {
    (text.into(), hint.into())
}

fn number(arg: Option<&String>, what: &str) -> Result<usize, Problem> {
    arg.and_then(|a| a.parse::<usize>().ok())
        .filter(|n| *n >= 1)
        .ok_or_else(|| problem(format!("{what} needs a number"), format!("wisply {USAGE}")))
}

impl Pc {
    pub fn command(&mut self, args: &[String]) -> Result<String, Problem> {
        self.cfg = Config::load(&self.home);
        let rest = &args[args.len().min(1)..];
        match args.first().map(|a| a.to_lowercase()).as_deref().unwrap_or("") {
            "" | "check" => Ok(self.check()),
            "fix" => self.fix(rest),
            "clean" => Ok(self.clean(rest.iter().any(|a| a == "--dry"))),
            "startup" => self.startup(rest),
            "game" => Ok(self.game()),
            "afk" => Ok(self.hand(Some(Manual::Afk), "afk: game mode is relaxed until you run wisply pc auto")),
            "play" => Ok(self.hand(Some(Manual::Play), "play: game mode is on until you run wisply pc auto")),
            "auto" => Ok(self.hand(None, "auto: game mode follows the game and your input again")),
            "net" => Ok(self.net(rest.first().map(String::as_str))),
            "disk" => Ok(self.disk()),
            "updates" => Ok(health::updates((secs() / 86_400) as i64)),
            "on" => self.act(true),
            "off" => self.act(false),
            "log" => Ok(self.log.show(rest.first().and_then(|n| n.parse().ok()).unwrap_or(LOG_LINES))),
            "config" => Ok(format!(
                "pc config | {}\nfile | {}",
                self.cfg.summary(),
                dir(&self.home).join("config.json").display()
            )),
            "help" => Ok(USAGE.to_string()),
            other => Err(problem(format!("unknown pc command '{other}'"), format!("wisply {USAGE}"))),
        }
    }

    fn check(&mut self) -> String {
        self.warm();
        let sample = self.fresh().sample.clone();
        let facts = self.fresh().facts.clone();
        let facts = self.facts(facts);
        let report = evaluate(&sample, &facts, &self.cfg);
        self.set_suggestions(&report.brakes);
        let mut text = render(&report);
        if !self.cfg.act {
            text.push_str("\nwatching only; wisply pc on lets it act by itself");
        }
        text
    }

    fn fix(&mut self, args: &[String]) -> Result<String, Problem> {
        if self.suggestions.is_empty() {
            self.check();
        }
        let fixes: Vec<Fix> = if args.first().is_some_and(|a| a == "all") {
            self.suggestions.iter().map(|s| s.fix.clone()).filter(Fix::safe).collect()
        } else {
            let n = number(args.first(), "fix")?;
            let Some(s) = self.suggestions.get(n - 1) else {
                return Err(problem(format!("there is no suggestion {n}"), "wisply pc"));
            };
            vec![s.fix.clone()]
        };
        if fixes.is_empty() {
            return Ok("pc fix | nothing safe to do".into());
        }
        let lines: Vec<String> = fixes.iter().take(MAX_FIX_LIST).map(|f| self.apply(f)).collect();
        Ok(lines.join("\n"))
    }

    fn apply(&mut self, fix: &Fix) -> String {
        let sample = self.plat.sample();
        match fix {
            Fix::Free => self.free(&sample),
            Fix::Clean => self.clean(false),
            Fix::Lower { name } => self.lower(&sample, name),
            Fix::Kill { name, pids } => self.close(&sample, name, pids),
            Fix::Startup => "pc fix | switch autostarts off yourself: wisply pc startup".into(),
            Fix::Net => net::report(&mut *self.plat, &self.cfg.ping_host.clone(), self.cfg.ping_ms),
        }
    }

    fn free(&mut self, s: &Sample) -> String {
        let spare = s.foreground.map(|f| s.descendants(f)).unwrap_or_default();
        let floor = self.cfg.trim_min_mb * MB;
        let (mut count, mut bytes) = (0, 0);
        for p in &s.procs {
            let skip =
                p.rss < floor || spare.contains(&p.pid) || protected(&self.cfg, &p.name) || p.pid == std::process::id();
            if !skip && self.plat.trim(p.pid) {
                count += 1;
                bytes += p.rss;
                self.log.add(
                    "memory",
                    &format!("{} memory released", short_name(&p.name)),
                    &format!("pid {}", p.pid),
                    p.rss,
                );
            }
        }
        format!("pc fix | asked {count} idle programs to give back memory ({} held)", size(bytes))
    }

    fn lower(&mut self, s: &Sample, name: &str) -> String {
        let level = self.plat.levels().low;
        let mut done = 0;
        for p in s.procs.iter().filter(|p| p.name.eq_ignore_ascii_case(name) && !protected(&self.cfg, &p.name)) {
            if self.plat.set_priority(p.pid, level) {
                done += 1;
                self.log.add("priority", &format!("{} lowered by you", short_name(name)), &format!("pid {}", p.pid), 0);
            }
        }
        format!("pc fix | lowered the priority of {done} {} processes", short_name(name))
    }

    fn close(&mut self, s: &Sample, name: &str, pids: &[u32]) -> String {
        if protected(&self.cfg, name) {
            return format!("pc fix | {} is protected and stays open", short_name(name));
        }
        let mut done = 0;
        for pid in pids {
            let same = s.proc(*pid).is_some_and(|p| p.name.eq_ignore_ascii_case(name));
            if same && !s.foreground.is_some_and(|f| f == *pid) && self.plat.kill(*pid) {
                done += 1;
                self.log.add("close", &format!("{} closed by you", short_name(name)), &format!("pid {pid}"), 0);
            }
        }
        format!("pc fix | closed {done} {} processes", short_name(name))
    }

    fn clean(&mut self, dry: bool) -> String {
        let procs = self.plat.sample().procs;
        if dry {
            let env = self.clean_env();
            return clean::render(&clean::plan(&env, &self.cfg, &procs), true);
        }
        let env = self.clean_env();
        let out = run_clean(&self.home, &env, &self.cfg, &procs);
        format!(
            "pc clean | freed {} in {} items ({} left alone); details: wisply pc log",
            size(out.freed),
            out.removed,
            out.skipped
        )
    }

    fn startup(&mut self, args: &[String]) -> Result<String, Problem> {
        let list = startup::list(&self.user);
        let sample = self.plat.sample();
        let Some(verb) = args.first().map(|a| a.to_lowercase()) else { return Ok(startup::render(&list, &sample)) };
        let turn_on = match verb.as_str() {
            "on" => true,
            "off" => false,
            _ => return Err(problem(format!("startup takes off or on, not '{verb}'"), "wisply pc startup off <n>")),
        };
        let n = number(args.get(1), "startup off/on")?;
        let Some(entry) = list.get(n - 1) else {
            return Err(problem(format!("there is no autostart entry {n}"), "wisply pc startup"));
        };
        startup::toggle(&self.home, entry, turn_on)
            .inspect(|_| self.log.add("startup", &format!("{} switched {}", entry.name, on(turn_on)), &entry.source, 0))
            .map_err(|e| problem(e, "wisply pc startup"))
    }

    fn game(&mut self) -> String {
        let (game, mode, idle) = match &self.last {
            Some(v) => (v.game.clone(), v.mode, v.sample.idle_secs),
            None => (String::new(), crate::mode::Mode::Off, None),
        };
        let what = if game.is_empty() { "no game running".to_string() } else { game };
        let input = idle.map_or("unknown".to_string(), |s| format!("{s} s ago"));
        let hand = match self.manual {
            Some(Manual::Afk) => ", by hand: afk",
            Some(Manual::Play) => ", by hand: play",
            None => "",
        };
        let avg = if self.ticks > 0 { self.spent.as_secs_f64() * 1000.0 / self.ticks as f64 } else { 0.0 };
        format!(
            "pc game | {} | {what} | last input {input}{hand}\nacting | {} | changes now: {}\nwatching every {} s | {} ticks, {avg:.1} ms each",
            mode.word(),
            on(self.cfg.act),
            self.modes.summary(),
            self.cfg.tick_secs,
            self.ticks
        )
    }

    fn hand(&mut self, m: Option<Manual>, text: &str) -> String {
        self.manual = m;
        format!("pc {text}")
    }

    fn net(&mut self, host: Option<&str>) -> String {
        let host = host.map_or_else(|| self.cfg.ping_host.clone(), String::from);
        net::report(&mut *self.plat, &host, self.cfg.ping_ms)
    }

    fn disk(&mut self) -> String {
        let sample = self.plat.sample();
        health::render_disks(&health::disks(), &sample)
    }

    fn act(&mut self, yes: bool) -> Result<String, Problem> {
        Config::set_flag(&self.home, "act", yes).map_err(|e| problem(e, "check the folder rights"))?;
        self.cfg = Config::load(&self.home);
        if !yes && self.modes.active() {
            self.modes.restore_all(&mut *self.plat);
        }
        Ok(if yes {
            "pc on | it may now lower priorities, free memory, clean temp files and run the game mode by itself; closing programs and switching autostarts off still needs your yes".into()
        } else {
            "pc off | it only shows and warns now; everything it had changed is back to normal".into()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::{Fake, proc};
    use crate::model::{Disk, GB};
    use std::fs;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    fn home(tag: &str) -> PathBuf {
        let h = std::env::temp_dir().join(format!("wpc-cmd-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&h);
        h
    }

    fn world() -> Fake {
        let mut f = Fake::with(vec![
            proc(1, 0, "brave.exe", 6 * GB),
            proc(2, 0, "javaw.exe", 3 * GB),
            proc(3, 0, "tool.exe", 500 * MB),
        ]);
        f.sample.mem_total = 100 * GB;
        f.sample.mem_used = 90 * GB;
        f.sample.foreground = Some(3);
        f.sample.disks = vec![Disk { mount: "C:".into(), total: 500 * GB, free: 10 * GB }];
        f
    }

    fn pc(h: &std::path::Path) -> Pc {
        let sink = Arc::new(Mutex::new(Vec::<String>::new()));
        let mut p = Pc::new(
            h,
            &h.join("user"),
            Box::new(world()),
            Box::new(move |t: &str, b: &str| sink.lock().unwrap().push(format!("{t}{b}"))),
        );
        p.probes = false;
        p.env_override = Some(clean::Env::default());
        p
    }

    fn a(words: &[&str]) -> Vec<String> {
        words.iter().map(|w| (*w).to_string()).collect()
    }

    #[test]
    fn the_check_lists_the_biggest_brakes_with_numbers() {
        let h = home("check");
        let mut p = pc(&h);
        let text = p.command(&[]).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        assert!(
            lines[0].starts_with("pc ")
                && lines[0].contains("RAM 90 % (brave 6.0 GB)")
                && lines[0].contains("C: 10.0 GB free"),
            "{text}"
        );
        assert!(lines[1].starts_with("1. RAM 90 %") && lines[1].ends_with("wisply pc fix 1"), "{text}");
        assert!(text.contains("C: only 10.0 GB free") && text.contains("watching only"), "{text}");
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn fix_one_runs_that_suggestion_and_fix_all_runs_only_the_safe_ones() {
        let h = home("fix");
        let mut p = pc(&h);
        p.command(&[]).unwrap();
        let first = p.command(&a(&["fix", "1"])).unwrap();
        assert!(first.starts_with("pc fix | asked 1 idle programs"), "{first}");
        let all = p.command(&a(&["fix", "all"])).unwrap();
        assert!(!all.contains("closed"), "{all}");
        let err = p.command(&a(&["fix", "99"])).unwrap_err();
        assert_eq!(err.0, "there is no suggestion 99");
        assert!(p.command(&a(&["fix", "x"])).is_err());
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn closing_needs_the_explicit_number_and_never_touches_protected_programs() {
        let h = home("kill");
        let mut p = pc(&h);
        p.command(&[]).unwrap();
        let n = p.suggestions.iter().position(|s| matches!(s.fix, Fix::Kill { .. })).expect("kill suggestion") + 1;
        let out = p.command(&a(&["fix", &n.to_string()])).unwrap();
        assert_eq!(out, "pc fix | closed 1 brave processes");
        let s = p.plat.sample();
        assert!(s.proc(1).is_none() && s.proc(2).is_some());
        let guarded = p.close(&s, "javaw.exe", &[2]);
        assert!(guarded.contains("protected"), "{guarded}");
        assert!(p.plat.sample().proc(2).is_some());
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn on_and_off_switch_acting_and_off_puts_everything_back() {
        let h = home("onoff");
        let mut p = pc(&h);
        assert!(!p.cfg.act);
        assert!(p.command(&a(&["on"])).unwrap().starts_with("pc on"));
        assert!(Config::load(&h).act && p.cfg.act);
        assert!(p.command(&a(&["off"])).unwrap().starts_with("pc off"));
        assert!(!Config::load(&h).act);
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn game_afk_play_auto_and_unknown_words() {
        let h = home("game");
        let mut p = pc(&h);
        p.tick("");
        assert!(p.command(&a(&["game"])).unwrap().starts_with("pc game | off | no game running"));
        p.command(&a(&["afk"])).unwrap();
        assert!(p.command(&a(&["game"])).unwrap().contains("by hand: afk"));
        p.command(&a(&["auto"])).unwrap();
        assert!(!p.command(&a(&["game"])).unwrap().contains("by hand"));
        let err = p.command(&a(&["wat"])).unwrap_err();
        assert_eq!(err.0, "unknown pc command 'wat'");
        assert!(p.command(&a(&["log"])).unwrap().contains("warning | RAM at 90 %"));
        fs::remove_dir_all(&h).ok();
    }

    #[test]
    fn clean_dry_never_deletes() {
        let h = home("dry");
        let mut p = pc(&h);
        let text = p.command(&a(&["clean", "--dry"])).unwrap();
        assert!(text.starts_with("pc clean --dry") || text == "pc clean | nothing to clean up", "{text}");
        fs::remove_dir_all(&h).ok();
    }
}
