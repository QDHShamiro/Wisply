use serde_json::{Value, json};

use crate::config::Config;
use crate::model::{GB, Sample, gb, short_name};

const RAM_FREE_PCT: f32 = 70.0;
const RAM_MAX: f32 = 35.0;
const RAM_SLOPE: f32 = 1.5;
const BIG_PROC: u64 = 2 * GB;
const DISK_MAX: f32 = 25.0;
const DISK_SLOPE: f32 = 1.5;
const CPU_FREE_PCT: f32 = 60.0;
const CPU_MAX: f32 = 20.0;
const AUTOSTART_FREE: usize = 8;
const AUTOSTART_MAX: f32 = 10.0;
const PING_FREE_MS: u64 = 100;
const PING_MAX: f32 = 10.0;
const PING_LOST: f32 = 5.0;
const TOP: usize = 3;

#[derive(Clone, Debug, PartialEq)]
pub enum Fix {
    Free,
    Clean,
    Lower { name: String },
    Kill { name: String, pids: Vec<u32> },
    Startup,
    Net,
}

impl Fix {
    pub fn safe(&self) -> bool {
        matches!(self, Fix::Free | Fix::Clean | Fix::Lower { .. })
    }

    pub fn to_json(&self) -> Value {
        match self {
            Fix::Free => json!({"kind": "free"}),
            Fix::Clean => json!({"kind": "clean"}),
            Fix::Lower { name } => json!({"kind": "lower", "name": name}),
            Fix::Kill { name, pids } => json!({"kind": "kill", "name": name, "pids": pids}),
            Fix::Startup => json!({"kind": "startup"}),
            Fix::Net => json!({"kind": "net"}),
        }
    }

    pub fn from_json(v: &Value) -> Option<Fix> {
        let name = || v.get("name").and_then(Value::as_str).map(String::from);
        Some(match v.get("kind")?.as_str()? {
            "free" => Fix::Free,
            "clean" => Fix::Clean,
            "startup" => Fix::Startup,
            "net" => Fix::Net,
            "lower" => Fix::Lower { name: name()? },
            "kill" => {
                let pids = v.get("pids")?.as_array()?.iter().filter_map(|p| p.as_u64().map(|p| p as u32)).collect();
                Fix::Kill { name: name()?, pids }
            }
            _ => return None,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Brake {
    pub cost: f32,
    pub text: String,
    pub todo: String,
    pub fix: Fix,
}

#[derive(Clone, Debug, Default)]
pub struct Facts {
    pub cpu_pct: Option<f32>,
    pub top_cpu: Option<(String, f32)>,
    pub autostarts: Option<usize>,
    pub ping: Option<(String, Option<u64>)>,
}

#[derive(Clone, Debug)]
pub struct Report {
    pub score: u32,
    pub headline: String,
    pub brakes: Vec<Brake>,
}

pub fn protected(cfg: &Config, name: &str) -> bool {
    let short = short_name(name);
    cfg.protect.iter().any(|p| short_name(p) == short)
}

fn ram(s: &Sample, cfg: &Config) -> Vec<Brake> {
    let pct = s.mem_pct();
    if pct <= RAM_FREE_PCT {
        return Vec::new();
    }
    let top = s.by_name().into_iter().next();
    let cost = ((pct - RAM_FREE_PCT) * RAM_SLOPE).min(RAM_MAX);
    let who = top.as_ref().map_or(String::new(), |(n, b)| format!(" ({} holds {})", short_name(n), gb(*b)));
    let mut out = vec![Brake {
        cost,
        text: format!("RAM {pct:.0} %{who}"),
        todo: "free the memory of idle programs".into(),
        fix: Fix::Free,
    }];
    if let Some((name, bytes)) = top.filter(|(n, b)| *b >= BIG_PROC && !protected(cfg, n)) {
        let pids = s.procs.iter().filter(|p| p.name.to_lowercase() == name).map(|p| p.pid).collect();
        out.push(Brake {
            cost: cost / 2.0,
            text: format!("{} holds {}", short_name(&name), gb(bytes)),
            todo: format!("close {} if you do not need it right now", short_name(&name)),
            fix: Fix::Kill { name, pids },
        });
    }
    out
}

fn disks(s: &Sample, cfg: &Config) -> Vec<Brake> {
    let min = cfg.clean_min_free_gb as f32;
    s.disks
        .iter()
        .filter(|d| d.total > 0 && (d.free as f32 / GB as f32) < min)
        .map(|d| {
            let free = d.free as f32 / GB as f32;
            Brake {
                cost: ((min - free) * DISK_SLOPE).min(DISK_MAX),
                text: format!("{} only {} free", d.mount, gb(d.free)),
                todo: "clean temp files, caches and old build folders".into(),
                fix: Fix::Clean,
            }
        })
        .collect()
}

fn cpu(f: &Facts, cfg: &Config) -> Vec<Brake> {
    let Some(pct) = f.cpu_pct.filter(|p| *p > CPU_FREE_PCT) else { return Vec::new() };
    let cost = ((pct - CPU_FREE_PCT) / 2.0).min(CPU_MAX);
    let Some((name, share)) = &f.top_cpu else {
        return vec![Brake {
            cost,
            text: format!("CPU {pct:.0} %"),
            todo: "see wisply pc net or close heavy programs".into(),
            fix: Fix::Net,
        }];
    };
    let fix = if protected(cfg, name) { Fix::Net } else { Fix::Lower { name: name.clone() } };
    vec![Brake {
        cost,
        text: format!("CPU {pct:.0} % ({} uses {share:.0} %)", short_name(name)),
        todo: format!("lower the priority of {}", short_name(name)),
        fix,
    }]
}

fn startup(f: &Facts) -> Vec<Brake> {
    let Some(n) = f.autostarts.filter(|n| *n > AUTOSTART_FREE) else { return Vec::new() };
    vec![Brake {
        cost: ((n - AUTOSTART_FREE) as f32 * 2.0).min(AUTOSTART_MAX),
        text: format!("{n} autostarts"),
        todo: "look at wisply pc startup and switch off what you do not need".into(),
        fix: Fix::Startup,
    }]
}

fn line(f: &Facts) -> Vec<Brake> {
    match &f.ping {
        Some((host, Some(ms))) if *ms > PING_FREE_MS => vec![Brake {
            cost: ((*ms - PING_FREE_MS) as f32 / 20.0).min(PING_MAX),
            text: format!("ping {host} {ms} ms"),
            todo: "see who uses the line with wisply pc net".into(),
            fix: Fix::Net,
        }],
        Some((host, None)) => vec![Brake {
            cost: PING_LOST,
            text: format!("no answer from {host}"),
            todo: "check the connection with wisply pc net".into(),
            fix: Fix::Net,
        }],
        _ => Vec::new(),
    }
}

pub fn headline(score: u32, s: &Sample, f: &Facts) -> String {
    let mut parts = vec![format!("pc {score}/100")];
    let top = s.by_name().into_iter().next();
    if s.mem_total > 0 {
        let who = top.map_or(String::new(), |(n, b)| format!(" ({} {})", short_name(&n), gb(b)));
        parts.push(format!("RAM {:.0} %{who}", s.mem_pct()));
    }
    if let Some(d) = s.disks.iter().filter(|d| d.total > 0).min_by_key(|d| d.free) {
        parts.push(format!("{} {} free", d.mount, gb(d.free)));
    }
    if let Some(n) = f.autostarts {
        parts.push(format!("{n} autostarts"));
    }
    match &f.ping {
        Some((host, Some(ms))) => parts.push(format!("ping {host} {ms} ms")),
        Some((host, None)) => parts.push(format!("ping {host} no answer")),
        None => {}
    }
    parts.join(" \u{b7} ")
}

pub fn evaluate(s: &Sample, f: &Facts, cfg: &Config) -> Report {
    let mut brakes: Vec<Brake> = [ram(s, cfg), disks(s, cfg), cpu(f, cfg), startup(f), line(f)].concat();
    brakes.sort_by(|a, b| b.cost.total_cmp(&a.cost));
    let lost: f32 = brakes.iter().filter(|b| !matches!(b.fix, Fix::Kill { .. })).map(|b| b.cost).sum();
    let score = (100.0 - lost).round().clamp(0.0, 100.0) as u32;
    brakes.truncate(TOP);
    Report { score, headline: headline(score, s, f), brakes }
}

pub fn render(r: &Report) -> String {
    let mut lines = vec![r.headline.clone()];
    if r.brakes.is_empty() {
        lines.push("nothing slows this PC down right now".into());
    }
    for (i, b) in r.brakes.iter().enumerate() {
        lines.push(format!("{}. {} - {} -> wisply pc fix {}", i + 1, b.text, b.todo, i + 1));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::proc;
    use crate::model::Disk;

    fn sample(used_pct: u64, free_gb: u64) -> Sample {
        Sample {
            mem_total: 100 * GB,
            mem_used: used_pct * GB,
            procs: vec![proc(1, 0, "brave.exe", 6 * GB), proc(2, 0, "javaw.exe", 3 * GB)],
            disks: vec![Disk { mount: "C:".into(), total: 500 * GB, free: free_gb * GB }],
            ..Sample::default()
        }
    }

    #[test]
    fn a_healthy_pc_scores_100_and_names_no_brake() {
        let r = evaluate(&sample(40, 200), &Facts::default(), &Config::default());
        assert_eq!(r.score, 100);
        assert!(r.brakes.is_empty());
        assert!(render(&r).contains("nothing slows this PC down"));
    }

    #[test]
    fn the_example_line_has_every_field() {
        let f = Facts { autostarts: Some(6), ping: Some(("hugosmp".into(), Some(34))), ..Facts::default() };
        let r = evaluate(&sample(87, 18), &f, &Config::default());
        assert_eq!(
            r.headline,
            format!(
                "pc {}/100 \u{b7} RAM 87 % (brave 6.0 GB) \u{b7} C: 18.0 GB free \u{b7} 6 autostarts \u{b7} ping hugosmp 34 ms",
                r.score
            )
        );
        assert!(r.score < 100 && r.score > 40, "{}", r.score);
    }

    #[test]
    fn the_three_biggest_brakes_come_first_and_protected_programs_are_never_kill_targets() {
        let f = Facts {
            cpu_pct: Some(95.0),
            top_cpu: Some(("cargo.exe".into(), 70.0)),
            autostarts: Some(20),
            ping: Some(("h".into(), None)),
        };
        let r = evaluate(&sample(97, 5), &f, &Config::default());
        assert_eq!(r.brakes.len(), 3);
        assert!(r.brakes.windows(2).all(|w| w[0].cost >= w[1].cost));
        let all: Vec<Fix> =
            evaluate(&sample(97, 5), &f, &Config::default()).brakes.into_iter().map(|b| b.fix).collect();
        assert!(!all.iter().any(|x| matches!(x, Fix::Kill { name, .. } if name.contains("javaw"))));
        let heavy_java = Sample { procs: vec![proc(9, 0, "javaw.exe", 9 * GB)], ..sample(97, 200) };
        let r = evaluate(&heavy_java, &Facts::default(), &Config::default());
        assert!(r.brakes.iter().all(|b| !matches!(b.fix, Fix::Kill { .. })));
    }

    #[test]
    fn a_big_ordinary_program_is_offered_for_closing_only_after_the_safe_fix() {
        let r = evaluate(&sample(90, 200), &Facts::default(), &Config::default());
        assert_eq!(r.brakes[0].fix, Fix::Free);
        assert!(matches!(&r.brakes[1].fix, Fix::Kill { name, pids } if name == "brave.exe" && pids == &[1]));
        assert!(Fix::Free.safe() && !r.brakes[1].fix.safe());
    }

    #[test]
    fn fixes_survive_the_suggestion_file() {
        let fixes = [
            Fix::Free,
            Fix::Clean,
            Fix::Startup,
            Fix::Net,
            Fix::Lower { name: "cargo.exe".into() },
            Fix::Kill { name: "x".into(), pids: vec![3, 4] },
        ];
        for fix in fixes {
            assert_eq!(Fix::from_json(&fix.to_json()), Some(fix));
        }
        assert_eq!(Fix::from_json(&json!({"kind": "nope"})), None);
    }
}
