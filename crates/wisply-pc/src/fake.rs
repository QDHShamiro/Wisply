use std::collections::HashMap;

use crate::model::{Proc, Sample};
use crate::plat::{Levels, Platform};

pub const LOWEST: i32 = 19;
pub const LOW: i32 = 10;
pub const BOOST: i32 = -5;
pub const NORMAL: i32 = 0;

#[derive(Default)]
pub struct Fake {
    pub sample: Sample,
    pub prio: HashMap<u32, i32>,
    pub suspended: Vec<u32>,
    pub trimmed: Vec<u32>,
    pub killed: Vec<u32>,
    pub plan: String,
    pub fast: Option<String>,
    pub calls: Vec<String>,
}

impl Fake {
    pub fn with(procs: Vec<Proc>) -> Fake {
        let prio = procs.iter().map(|p| (p.pid, NORMAL)).collect();
        Fake {
            sample: Sample { procs, mem_total: 100, mem_used: 50, ..Sample::default() },
            prio,
            plan: "balanced".into(),
            fast: Some("fast".into()),
            ..Fake::default()
        }
    }
}

pub fn proc(pid: u32, ppid: u32, name: &str, rss: u64) -> Proc {
    Proc { pid, ppid, name: name.into(), exe: format!("C:/bin/{name}"), rss, ..Proc::default() }
}

impl Platform for Fake {
    fn sample(&mut self) -> Sample {
        self.sample.procs.retain(|p| self.prio.contains_key(&p.pid));
        self.sample.clone()
    }

    fn levels(&self) -> Levels {
        Levels { lowest: LOWEST, low: LOW, boost: BOOST }
    }

    fn priority(&mut self, pid: u32) -> Option<i32> {
        self.prio.get(&pid).copied()
    }

    fn set_priority(&mut self, pid: u32, level: i32) -> bool {
        self.calls.push(format!("prio {pid} {level}"));
        self.prio.get_mut(&pid).map(|p| *p = level).is_some()
    }

    fn suspend(&mut self, pid: u32) -> bool {
        self.calls.push(format!("suspend {pid}"));
        self.suspended.push(pid);
        true
    }

    fn resume(&mut self, pid: u32) -> bool {
        self.calls.push(format!("resume {pid}"));
        self.suspended.retain(|p| *p != pid);
        true
    }

    fn trim(&mut self, pid: u32) -> bool {
        self.calls.push(format!("trim {pid}"));
        self.trimmed.push(pid);
        true
    }

    fn kill(&mut self, pid: u32) -> bool {
        self.calls.push(format!("kill {pid}"));
        self.killed.push(pid);
        self.prio.remove(&pid);
        true
    }

    fn power_plan(&mut self) -> Option<String> {
        Some(self.plan.clone())
    }

    fn fast_power_plan(&mut self) -> Option<String> {
        self.fast.clone()
    }

    fn set_power_plan(&mut self, id: &str) -> bool {
        self.calls.push(format!("plan {id}"));
        self.plan = id.into();
        true
    }
}
