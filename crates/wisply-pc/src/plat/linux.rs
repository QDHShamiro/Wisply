use std::collections::HashSet;
use std::ffi::CString;
use std::fs;
use std::time::{Duration, Instant};

use super::parse;
use super::shell;
use super::unix;
use super::{Levels, Platform};
use crate::model::{Disk, GB, Proc, Sample};

const LOWEST: i32 = 19;
const LOW: i32 = 10;
const BOOST: i32 = -5;
const MIN_DISK: u64 = 4 * GB;
const TOOL_WAIT: Duration = Duration::from_secs(3);
const FAST_PLAN: &str = "performance";

pub struct Linux {
    born: Instant,
    page: u64,
    hz: u64,
    cores: u32,
    prev_ticks: Option<(u64, u64)>,
    idle_tool: bool,
    focus_tool: bool,
}

impl Linux {
    pub fn new() -> Linux {
        let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) }.max(4096) as u64;
        let hz = unsafe { libc::sysconf(libc::_SC_CLK_TCK) }.max(1) as u64;
        let cores = std::thread::available_parallelism().map_or(1, |n| n.get() as u32);
        Linux { born: Instant::now(), page, hz, cores, prev_ticks: None, idle_tool: true, focus_tool: true }
    }

    fn processes(&self) -> Vec<Proc> {
        let Ok(dir) = fs::read_dir("/proc") else { return Vec::new() };
        dir.flatten()
            .filter_map(|e| {
                let pid: u32 = e.file_name().to_str()?.parse().ok()?;
                let base = e.path();
                let (name, ppid, ticks) = parse::proc_stat(&fs::read_to_string(base.join("stat")).ok()?)?;
                let pages: u64 =
                    fs::read_to_string(base.join("statm")).ok()?.split_whitespace().nth(1)?.parse().ok()?;
                let exe = fs::read_link(base.join("exe")).map(|p| p.to_string_lossy().into_owned()).unwrap_or_default();
                let io = fs::read_to_string(base.join("io")).map_or(0, |t| {
                    t.lines()
                        .filter(|l| l.starts_with("read_bytes") || l.starts_with("write_bytes"))
                        .filter_map(|l| l.split(':').nth(1)?.trim().parse::<u64>().ok())
                        .sum()
                });
                Some(Proc { pid, ppid, name, exe, rss: pages * self.page, cpu_ms: ticks * 1000 / self.hz, io })
            })
            .collect()
    }

    fn disks() -> Vec<Disk> {
        let mounts = fs::read_to_string("/proc/mounts").unwrap_or_default();
        let mut seen = HashSet::new();
        mounts
            .lines()
            .filter_map(|l| {
                let mut f = l.split_whitespace();
                let (dev, mount) = (f.next()?, f.next()?);
                if !dev.starts_with("/dev/") || dev.starts_with("/dev/loop") || !seen.insert(dev.to_string()) {
                    return None;
                }
                let mount = mount.replace("\\040", " ");
                let path = CString::new(mount.clone()).ok()?;
                let mut st: libc::statvfs = unsafe { std::mem::zeroed() };
                if unsafe { libc::statvfs(path.as_ptr(), &mut st) } != 0 {
                    return None;
                }
                let total = st.f_blocks as u64 * st.f_frsize as u64;
                let free = st.f_bavail as u64 * st.f_frsize as u64;
                (total >= MIN_DISK).then_some(Disk { mount, total, free })
            })
            .collect()
    }

    fn idle(&mut self) -> Option<u64> {
        if !self.idle_tool {
            return None;
        }
        let out = shell::run("xprintidle", &[], TOOL_WAIT);
        if out.is_none() && !shell::exists("xprintidle") {
            self.idle_tool = false;
        }
        Some(out?.trim().parse::<u64>().ok()? / 1000)
    }

    fn foreground(&mut self) -> Option<u32> {
        if !self.focus_tool {
            return None;
        }
        let out = shell::run("xdotool", &["getwindowfocus", "getwindowpid"], TOOL_WAIT);
        if out.is_none() && !shell::exists("xdotool") {
            self.focus_tool = false;
        }
        out?.trim().parse().ok()
    }

    fn busy(&mut self, at_ms: u64) -> Option<f32> {
        let ticks = parse::io_ticks(&fs::read_to_string("/proc/diskstats").ok()?);
        let prev = self.prev_ticks.replace((at_ms, ticks))?;
        let wall = at_ms.checked_sub(prev.0).filter(|w| *w > 0)?;
        Some((ticks.saturating_sub(prev.1) as f32 * 100.0 / wall as f32).clamp(0.0, 100.0))
    }
}

impl Platform for Linux {
    fn sample(&mut self) -> Sample {
        let at_ms = self.born.elapsed().as_millis() as u64;
        let mem = fs::read_to_string("/proc/meminfo").ok().and_then(|t| parse::meminfo(&t));
        let (total, avail) = mem.unwrap_or((0, 0));
        Sample {
            at_ms,
            mem_total: total,
            mem_used: total.saturating_sub(avail),
            cpu: fs::read_to_string("/proc/stat").ok().and_then(|t| parse::cpu_line(&t)),
            cores: self.cores,
            disk_busy: self.busy(at_ms),
            idle_secs: self.idle(),
            foreground: self.foreground(),
            procs: self.processes(),
            disks: Self::disks(),
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
        Some(shell::run("powerprofilesctl", &["get"], TOOL_WAIT)?.trim().to_string()).filter(|p| !p.is_empty())
    }

    fn fast_power_plan(&mut self) -> Option<String> {
        let list = shell::run("powerprofilesctl", &["list"], TOOL_WAIT)?;
        list.contains(FAST_PLAN).then(|| FAST_PLAN.to_string())
    }

    fn set_power_plan(&mut self, id: &str) -> bool {
        shell::succeeds("powerprofilesctl", &["set", id], TOOL_WAIT)
    }
}
