use std::time::{Duration, Instant};

use super::parse;
use super::shell;
use super::unix;
use super::{Levels, Platform};
use crate::model::Sample;

const LOWEST: i32 = 19;
const LOW: i32 = 10;
const BOOST: i32 = -5;
const TOOL_WAIT: Duration = Duration::from_secs(5);

pub struct Mac {
    born: Instant,
    memory: u64,
    cores: u32,
}

impl Mac {
    pub fn new() -> Mac {
        let memory =
            shell::run("sysctl", &["-n", "hw.memsize"], TOOL_WAIT).and_then(|t| t.trim().parse().ok()).unwrap_or(0);
        let cores = std::thread::available_parallelism().map_or(1, |n| n.get() as u32);
        Mac { born: Instant::now(), memory, cores }
    }
}

impl Platform for Mac {
    fn sample(&mut self) -> Sample {
        let at_ms = self.born.elapsed().as_millis() as u64;
        let ps = shell::run("ps", &["-axo", "pid=,ppid=,rss=,cputime=,comm="], TOOL_WAIT);
        let procs = ps.map(|t| parse::ps_rows(&t)).unwrap_or_default();
        let used = shell::run("vm_stat", &[], TOOL_WAIT).and_then(|t| parse::vm_stat(&t, self.memory)).unwrap_or(0);
        let busy: u64 = procs.iter().map(|p| p.cpu_ms).sum();
        let idle = shell::run("ioreg", &["-c", "IOHIDSystem", "-d", "4"], TOOL_WAIT).and_then(|t| parse::hid_idle(&t));
        Sample {
            at_ms,
            mem_total: self.memory,
            mem_used: used,
            cpu: Some((busy, at_ms * u64::from(self.cores))),
            cores: self.cores,
            disk_busy: None,
            idle_secs: idle,
            foreground: None,
            procs,
            disks: shell::run("df", &["-k", "-P"], TOOL_WAIT).map(|t| parse::df_rows(&t)).unwrap_or_default(),
        }
    }

    fn levels(&self) -> Levels {
        Levels { lowest: LOWEST, low: LOW, boost: BOOST }
    }

    fn priority(&mut self, pid: u32) -> Option<i32> {
        unix::priority(pid)
    }

    fn set_priority(&mut self, pid: u32, level: i32) -> bool {
        unix::set_priority(pid, level)
    }

    fn suspend(&mut self, pid: u32) -> bool {
        unix::suspend(pid)
    }

    fn resume(&mut self, pid: u32) -> bool {
        unix::resume(pid)
    }

    fn trim(&mut self, _pid: u32) -> bool {
        false
    }

    fn kill(&mut self, pid: u32) -> bool {
        unix::kill(pid)
    }

    fn power_plan(&mut self) -> Option<String> {
        None
    }

    fn fast_power_plan(&mut self) -> Option<String> {
        None
    }

    fn set_power_plan(&mut self, _id: &str) -> bool {
        false
    }
}
