use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::shell;
use super::{Levels, Platform};
use crate::model::{Disk, Proc, Sample};

type Handle = isize;

const SNAP_PROCESS: u32 = 0x2;
const QUERY_LIMITED: u32 = 0x1000;
const SET_INFORMATION: u32 = 0x200;
const SET_QUOTA: u32 = 0x100;
const QUERY: u32 = 0x400;
const SUSPEND_RESUME: u32 = 0x800;
const TERMINATE: u32 = 0x1;
const DRIVE_FIXED: u32 = 3;
const PATH_UNITS: usize = 1024;
const MAX_PATH: usize = 260;
const PDH_DOUBLE: u32 = 0x200;
const PDH_OK: i32 = 0;
const DISK_COUNTER: &str = "\\PhysicalDisk(_Total)\\% Disk Time";
const IDLE_CLASS: i32 = 0x40;
const BELOW_NORMAL: i32 = 0x4000;
const ABOVE_NORMAL: i32 = 0x8000;
const FILETIME_PER_MS: u64 = 10_000;
const POWER_WAIT: Duration = Duration::from_secs(8);
const HIGH_PLANS: [&str; 2] = ["8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c", "e9a42b02-d5df-448d-aa00-03f14749eb61"];

#[repr(C)]
struct ProcessEntry {
    size: u32,
    usage: u32,
    pid: u32,
    heap: usize,
    module: u32,
    threads: u32,
    ppid: u32,
    base: i32,
    flags: u32,
    exe: [u16; MAX_PATH],
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
struct FileTime {
    low: u32,
    high: u32,
}

impl FileTime {
    fn get(self) -> u64 {
        (u64::from(self.high) << 32) | u64::from(self.low)
    }
}

#[repr(C)]
#[derive(Default)]
struct MemoryCounters {
    size: u32,
    faults: u32,
    peak: usize,
    working: usize,
    rest: [usize; 6],
}

#[repr(C)]
#[derive(Default)]
struct IoCounters {
    ops: [u64; 6],
}

#[repr(C)]
struct MemoryStatus {
    size: u32,
    load: u32,
    total: u64,
    avail: u64,
    rest: [u64; 5],
}

#[repr(C)]
#[derive(Default)]
struct InputInfo {
    size: u32,
    time: u32,
}

#[repr(C)]
#[derive(Default)]
struct PdhValue {
    status: u32,
    value: f64,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> Handle;
    fn Process32FirstW(snap: Handle, entry: *mut ProcessEntry) -> i32;
    fn Process32NextW(snap: Handle, entry: *mut ProcessEntry) -> i32;
    fn OpenProcess(access: u32, inherit: i32, pid: u32) -> Handle;
    fn CloseHandle(handle: Handle) -> i32;
    fn GetProcessTimes(
        p: Handle,
        created: *mut FileTime,
        exited: *mut FileTime,
        kernel: *mut FileTime,
        user: *mut FileTime,
    ) -> i32;
    fn K32GetProcessMemoryInfo(p: Handle, counters: *mut MemoryCounters, size: u32) -> i32;
    fn K32EmptyWorkingSet(p: Handle) -> i32;
    fn GetProcessIoCounters(p: Handle, counters: *mut IoCounters) -> i32;
    fn QueryFullProcessImageNameW(p: Handle, flags: u32, buf: *mut u16, size: *mut u32) -> i32;
    fn GlobalMemoryStatusEx(status: *mut MemoryStatus) -> i32;
    fn GetSystemTimes(idle: *mut FileTime, kernel: *mut FileTime, user: *mut FileTime) -> i32;
    fn GetLogicalDrives() -> u32;
    fn GetDriveTypeW(root: *const u16) -> u32;
    fn GetDiskFreeSpaceExW(root: *const u16, avail: *mut u64, total: *mut u64, free: *mut u64) -> i32;
    fn GetPriorityClass(p: Handle) -> u32;
    fn SetPriorityClass(p: Handle, class: u32) -> i32;
    fn TerminateProcess(p: Handle, code: u32) -> i32;
    fn GetTickCount() -> u32;
    fn GetCurrentProcessId() -> u32;
}

#[link(name = "user32")]
unsafe extern "system" {
    fn GetLastInputInfo(info: *mut InputInfo) -> i32;
    fn GetForegroundWindow() -> Handle;
    fn GetWindowThreadProcessId(hwnd: Handle, pid: *mut u32) -> u32;
}

#[link(name = "ntdll")]
unsafe extern "system" {
    fn NtSuspendProcess(p: Handle) -> i32;
    fn NtResumeProcess(p: Handle) -> i32;
}

#[link(name = "pdh")]
unsafe extern "system" {
    fn PdhOpenQueryW(source: *const u16, user: usize, query: *mut Handle) -> i32;
    fn PdhAddEnglishCounterW(query: Handle, path: *const u16, user: usize, counter: *mut Handle) -> i32;
    fn PdhCollectQueryData(query: Handle) -> i32;
    fn PdhGetFormattedCounterValue(counter: Handle, format: u32, kind: *mut u32, value: *mut PdhValue) -> i32;
    fn PdhCloseQuery(query: Handle) -> i32;
}

struct Owned(Handle);

impl Owned {
    fn open(access: u32, pid: u32) -> Option<Owned> {
        let h = unsafe { OpenProcess(access, 0, pid) };
        (h != 0).then_some(Owned(h))
    }
}

impl Drop for Owned {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(std::iter::once(0)).collect()
}

fn utf16(buf: &[u16]) -> String {
    let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..end])
}

fn image_path(p: &Owned) -> String {
    let mut buf = [0u16; PATH_UNITS];
    let mut size = PATH_UNITS as u32;
    let ok = unsafe { QueryFullProcessImageNameW(p.0, 0, buf.as_mut_ptr(), &mut size) };
    if ok == 0 { String::new() } else { String::from_utf16_lossy(&buf[..(size as usize).min(PATH_UNITS)]) }
}

struct Counter {
    query: Handle,
    counter: Handle,
}

impl Counter {
    fn open() -> Option<Counter> {
        let mut query = 0;
        if unsafe { PdhOpenQueryW(std::ptr::null(), 0, &mut query) } != PDH_OK {
            return None;
        }
        let mut counter = 0;
        let path = wide(DISK_COUNTER);
        if unsafe { PdhAddEnglishCounterW(query, path.as_ptr(), 0, &mut counter) } != PDH_OK {
            unsafe { PdhCloseQuery(query) };
            return None;
        }
        unsafe { PdhCollectQueryData(query) };
        Some(Counter { query, counter })
    }

    fn read(&self) -> Option<f32> {
        if unsafe { PdhCollectQueryData(self.query) } != PDH_OK {
            return None;
        }
        let mut value = PdhValue::default();
        let got = unsafe { PdhGetFormattedCounterValue(self.counter, PDH_DOUBLE, std::ptr::null_mut(), &mut value) };
        (got == PDH_OK && value.status == 0).then(|| value.value.clamp(0.0, 100.0) as f32)
    }
}

impl Drop for Counter {
    fn drop(&mut self) {
        unsafe { PdhCloseQuery(self.query) };
    }
}

pub struct Windows {
    born: Instant,
    paths: HashMap<(u32, u64), String>,
    disk: Option<Counter>,
    disk_tried: bool,
    cores: u32,
}

impl Windows {
    pub fn new() -> Windows {
        let cores = std::thread::available_parallelism().map_or(1, |n| n.get() as u32);
        Windows { born: Instant::now(), paths: HashMap::new(), disk: None, disk_tried: false, cores }
    }

    fn processes(&mut self) -> Vec<Proc> {
        let snap = unsafe { CreateToolhelp32Snapshot(SNAP_PROCESS, 0) };
        if snap == -1 || snap == 0 {
            return Vec::new();
        }
        let mut entry: ProcessEntry = unsafe { std::mem::zeroed() };
        entry.size = size_of::<ProcessEntry>() as u32;
        let mut procs = Vec::new();
        let mut more = unsafe { Process32FirstW(snap, &mut entry) } != 0;
        let mut alive: HashMap<(u32, u64), String> = HashMap::new();
        while more {
            let pid = entry.pid;
            let mut p = Proc { pid, ppid: entry.ppid, name: utf16(&entry.exe), ..Proc::default() };
            if pid != 0
                && let Some(h) = Owned::open(QUERY_LIMITED, pid)
            {
                {
                    let (mut c, mut e, mut k, mut u) = Default::default();
                    if unsafe { GetProcessTimes(h.0, &mut c, &mut e, &mut k, &mut u) } != 0 {
                        p.cpu_ms = (k.get() + u.get()) / FILETIME_PER_MS;
                    }
                    let mut mem = MemoryCounters { size: size_of::<MemoryCounters>() as u32, ..Default::default() };
                    if unsafe { K32GetProcessMemoryInfo(h.0, &mut mem, mem.size) } != 0 {
                        p.rss = mem.working as u64;
                    }
                    let mut io = IoCounters::default();
                    if unsafe { GetProcessIoCounters(h.0, &mut io) } != 0 {
                        p.io = io.ops[3] + io.ops[4];
                    }
                    let key = (pid, c.get());
                    let path = self.paths.remove(&key).unwrap_or_else(|| image_path(&h));
                    p.exe = path.clone();
                    alive.insert(key, path);
                }
            }
            procs.push(p);
            more = unsafe { Process32NextW(snap, &mut entry) } != 0;
        }
        unsafe { CloseHandle(snap) };
        self.paths = alive;
        procs
    }

    fn disks() -> Vec<Disk> {
        let mask = unsafe { GetLogicalDrives() };
        (0..26u8)
            .filter(|i| mask & (1 << i) != 0)
            .filter_map(|i| {
                let root = format!("{}:\\", (b'A' + i) as char);
                let w = wide(&root);
                if unsafe { GetDriveTypeW(w.as_ptr()) } != DRIVE_FIXED {
                    return None;
                }
                let (mut avail, mut total, mut free) = (0u64, 0u64, 0u64);
                let ok = unsafe { GetDiskFreeSpaceExW(w.as_ptr(), &mut avail, &mut total, &mut free) };
                (ok != 0).then(|| Disk { mount: format!("{}:", (b'A' + i) as char), total, free: avail })
            })
            .collect()
    }

    fn cpu() -> Option<(u64, u64)> {
        let (mut idle, mut kernel, mut user) = Default::default();
        if unsafe { GetSystemTimes(&mut idle, &mut kernel, &mut user) } == 0 {
            return None;
        }
        let total = kernel.get() + user.get();
        Some((total.saturating_sub(idle.get()), total))
    }

    fn idle_secs() -> Option<u64> {
        let mut info = InputInfo { size: size_of::<InputInfo>() as u32, time: 0 };
        if unsafe { GetLastInputInfo(&mut info) } == 0 {
            return None;
        }
        Some(u64::from(unsafe { GetTickCount() }.wrapping_sub(info.time)) / 1000)
    }

    fn foreground() -> Option<u32> {
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd == 0 {
            return None;
        }
        let mut pid = 0;
        unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
        (pid != 0).then_some(pid)
    }
}

impl Platform for Windows {
    fn sample(&mut self) -> Sample {
        let mut status: MemoryStatus = unsafe { std::mem::zeroed() };
        status.size = size_of::<MemoryStatus>() as u32;
        let (total, used) = if unsafe { GlobalMemoryStatusEx(&mut status) } != 0 {
            (status.total, status.total.saturating_sub(status.avail))
        } else {
            (0, 0)
        };
        if !self.disk_tried {
            self.disk_tried = true;
            self.disk = Counter::open();
        }
        Sample {
            at_ms: self.born.elapsed().as_millis() as u64,
            mem_total: total,
            mem_used: used,
            cpu: Self::cpu(),
            cores: self.cores,
            disk_busy: self.disk.as_ref().and_then(Counter::read),
            idle_secs: Self::idle_secs(),
            foreground: Self::foreground(),
            procs: self.processes(),
            disks: Self::disks(),
        }
    }

    fn levels(&self) -> Levels {
        Levels { lowest: IDLE_CLASS, low: BELOW_NORMAL, boost: ABOVE_NORMAL }
    }

    fn priority(&mut self, pid: u32) -> Option<i32> {
        let h = Owned::open(QUERY_LIMITED, pid)?;
        let class = unsafe { GetPriorityClass(h.0) };
        (class != 0).then_some(class as i32)
    }

    fn set_priority(&mut self, pid: u32, level: i32) -> bool {
        if pid == unsafe { GetCurrentProcessId() } {
            return false;
        }
        Owned::open(SET_INFORMATION, pid).is_some_and(|h| unsafe { SetPriorityClass(h.0, level as u32) } != 0)
    }

    fn suspend(&mut self, pid: u32) -> bool {
        Owned::open(SUSPEND_RESUME, pid).is_some_and(|h| unsafe { NtSuspendProcess(h.0) } >= 0)
    }

    fn resume(&mut self, pid: u32) -> bool {
        Owned::open(SUSPEND_RESUME, pid).is_some_and(|h| unsafe { NtResumeProcess(h.0) } >= 0)
    }

    fn trim(&mut self, pid: u32) -> bool {
        Owned::open(SET_QUOTA | QUERY, pid).is_some_and(|h| unsafe { K32EmptyWorkingSet(h.0) } != 0)
    }

    fn kill(&mut self, pid: u32) -> bool {
        pid != unsafe { GetCurrentProcessId() }
            && Owned::open(TERMINATE, pid).is_some_and(|h| unsafe { TerminateProcess(h.0, 1) } != 0)
    }

    fn power_plan(&mut self) -> Option<String> {
        guid_in(&shell::run("powercfg", &["/getactivescheme"], POWER_WAIT)?)
    }

    fn fast_power_plan(&mut self) -> Option<String> {
        let list = shell::run("powercfg", &["/list"], POWER_WAIT)?.to_lowercase();
        HIGH_PLANS.iter().find(|g| list.contains(**g)).map(|g| (*g).to_string())
    }

    fn set_power_plan(&mut self, id: &str) -> bool {
        guid_in(id).is_some_and(|g| shell::succeeds("powercfg", &["/setactive", &g], POWER_WAIT))
    }
}

pub fn guid_in(text: &str) -> Option<String> {
    text.split(|c: char| !(c.is_ascii_hexdigit() || c == '-'))
        .find(|t| {
            t.len() == 36
                && t.split('-').map(str::len).eq([8, 4, 4, 4, 12])
                && t.chars().all(|c| c == '-' || c.is_ascii_hexdigit())
        })
        .map(str::to_lowercase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_plan_guids_are_found_in_any_language() {
        let out = "Power Scheme GUID: 381B4222-F694-41F0-9685-FF5BB260DF2E  (Ausbalanciert)";
        assert_eq!(guid_in(out).as_deref(), Some("381b4222-f694-41f0-9685-ff5bb260df2e"));
        assert_eq!(guid_in("nothing here"), None);
        assert_eq!(guid_in("381b4222-f694-41f0-9685-ff5bb260df2"), None);
    }

    #[test]
    fn a_real_sample_has_memory_disks_and_this_process() {
        let mut w = Windows::new();
        let s = w.sample();
        assert!(s.mem_total > 0 && s.mem_used > 0 && s.mem_used <= s.mem_total);
        assert!(!s.disks.is_empty() && s.disks.iter().all(|d| d.total >= d.free));
        let me = std::process::id();
        let mine = s.proc(me).expect("own process");
        assert!(mine.rss > 0 && mine.name.to_lowercase().contains(".exe"));
        assert!(s.cpu.is_some() && s.procs.len() > 5);
        let lv = w.levels();
        assert_eq!(w.priority(me).map(|p| p > 0), Some(true));
        assert!(!w.set_priority(me, lv.lowest));
        assert!(!w.kill(me));
    }
}
