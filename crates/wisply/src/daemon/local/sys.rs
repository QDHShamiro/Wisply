use std::fs::OpenOptions;
use std::path::Path;
use std::process::{Command, Stdio};

use super::plan::Prio;

pub fn spawn(exe: &Path, args: &[String], dir: &Path, log: &Path, prio: Prio) -> Option<u32> {
    let out = OpenOptions::new().create(true).append(true).open(log).ok()?;
    let err = out.try_clone().ok()?;
    let mut cmd = launcher(exe, prio);
    cmd.args(args).current_dir(dir).stdin(Stdio::null()).stdout(out).stderr(err);
    flags(&mut cmd, prio);
    cmd.spawn().ok().map(|child| child.id())
}

#[cfg(windows)]
fn launcher(exe: &Path, _prio: Prio) -> Command {
    Command::new(exe)
}

#[cfg(not(windows))]
fn launcher(exe: &Path, _prio: Prio) -> Command {
    let mut cmd = Command::new("nice");
    cmd.arg("-n").arg("10").arg(exe);
    cmd
}

#[cfg(windows)]
fn flags(cmd: &mut Command, prio: Prio) {
    use std::os::windows::process::CommandExt;
    const NO_WINDOW: u32 = 0x0800_0000;
    const BELOW_NORMAL: u32 = 0x0000_4000;
    const IDLE: u32 = 0x0000_0040;
    cmd.creation_flags(NO_WINDOW | if prio == Prio::Idle { IDLE } else { BELOW_NORMAL });
}

#[cfg(not(windows))]
fn flags(_cmd: &mut Command, _prio: Prio) {}

#[cfg(windows)]
mod imp {
    use std::os::windows::io::RawHandle;

    const QUERY_LIMITED: u32 = 0x1000;
    const TERMINATE: u32 = 0x0001;
    const STILL_ACTIVE: u32 = 259;
    const PATH_UNITS: usize = 1024;

    #[repr(C)]
    #[derive(Default)]
    struct Counters {
        size: u32,
        faults: u32,
        peak: usize,
        working: usize,
        rest: [usize; 6],
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn OpenProcess(access: u32, inherit: i32, pid: u32) -> RawHandle;
        fn CloseHandle(handle: RawHandle) -> i32;
        fn TerminateProcess(process: RawHandle, code: u32) -> i32;
        fn GetExitCodeProcess(process: RawHandle, code: *mut u32) -> i32;
        fn K32GetProcessMemoryInfo(process: RawHandle, counters: *mut Counters, size: u32) -> i32;
        fn QueryFullProcessImageNameW(process: RawHandle, flags: u32, buf: *mut u16, size: *mut u32) -> i32;
    }

    pub fn alive(pid: u32) -> bool {
        let process = unsafe { OpenProcess(QUERY_LIMITED, 0, pid) };
        if process.is_null() {
            return false;
        }
        let mut code = 0u32;
        let ok = unsafe { GetExitCodeProcess(process, &raw mut code) } != 0 && code == STILL_ACTIVE;
        unsafe { CloseHandle(process) };
        ok
    }

    pub fn kill(pid: u32) {
        let process = unsafe { OpenProcess(TERMINATE, 0, pid) };
        if !process.is_null() {
            unsafe {
                TerminateProcess(process, 1);
                CloseHandle(process);
            }
        }
    }

    pub fn rss_mb(pid: u32) -> Option<u64> {
        let process = unsafe { OpenProcess(QUERY_LIMITED, 0, pid) };
        if process.is_null() {
            return None;
        }
        let mut counters = Counters { size: size_of::<Counters>() as u32, ..Counters::default() };
        let ok = unsafe { K32GetProcessMemoryInfo(process, &raw mut counters, counters.size) } != 0;
        unsafe { CloseHandle(process) };
        ok.then_some((counters.working as u64) >> 20)
    }

    #[repr(C)]
    struct MemoryStatus {
        length: u32,
        load: u32,
        total: u64,
        rest: [u64; 6],
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GlobalMemoryStatusEx(status: *mut MemoryStatus) -> i32;
    }

    pub fn total_mb() -> u64 {
        let mut status = MemoryStatus { length: size_of::<MemoryStatus>() as u32, load: 0, total: 0, rest: [0; 6] };
        let ok = unsafe { GlobalMemoryStatusEx(&raw mut status) } != 0;
        if ok { status.total >> 20 } else { 0 }
    }

    pub fn image(pid: u32) -> Option<String> {
        let process = unsafe { OpenProcess(QUERY_LIMITED, 0, pid) };
        if process.is_null() {
            return None;
        }
        let mut buf = [0u16; PATH_UNITS];
        let mut size = PATH_UNITS as u32;
        let ok = unsafe { QueryFullProcessImageNameW(process, 0, buf.as_mut_ptr(), &raw mut size) } != 0;
        unsafe { CloseHandle(process) };
        ok.then(|| String::from_utf16_lossy(&buf[..(size as usize).min(PATH_UNITS)]))
    }
}

#[cfg(not(windows))]
mod imp {
    use std::process::{Command, Stdio};

    fn ps(field: &str, pid: u32) -> Option<String> {
        let out = Command::new("ps").args(["-o", field, "-p", &pid.to_string()]).stderr(Stdio::null()).output().ok()?;
        let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
        (out.status.success() && !text.is_empty()).then_some(text)
    }

    pub fn alive(pid: u32) -> bool {
        ps("pid=", pid).is_some()
    }

    pub fn kill(pid: u32) {
        let _ = Command::new("kill").arg(pid.to_string()).stderr(Stdio::null()).status();
    }

    pub fn rss_mb(pid: u32) -> Option<u64> {
        ps("rss=", pid)?.parse::<u64>().ok().map(|kb| kb >> 10)
    }

    pub fn image(pid: u32) -> Option<String> {
        ps("args=", pid)
    }

    pub fn total_mb() -> u64 {
        let proc = std::fs::read_to_string("/proc/meminfo").ok().and_then(|t| {
            let line = t.lines().find(|l| l.starts_with("MemTotal:"))?;
            line.split_whitespace().nth(1)?.parse::<u64>().ok().map(|kb| kb >> 10)
        });
        proc.or_else(|| {
            let out = Command::new("sysctl").args(["-n", "hw.memsize"]).stderr(Stdio::null()).output().ok()?;
            String::from_utf8_lossy(&out.stdout).trim().parse::<u64>().ok().map(|b| b >> 20)
        })
        .unwrap_or(0)
    }
}

pub fn alive(pid: u32) -> bool {
    pid > 0 && imp::alive(pid)
}

pub fn kill(pid: u32) {
    if pid > 0 {
        imp::kill(pid);
    }
}

pub fn rss_mb(pid: u32) -> Option<u64> {
    imp::rss_mb(pid)
}

fn norm(text: &str) -> String {
    text.replace('\\', "/").to_lowercase()
}

pub fn belongs(image: &str, root: &Path) -> bool {
    let root = norm(&root.display().to_string());
    !root.is_empty() && norm(image).contains(&root)
}

pub fn is_ours(pid: u32, root: &Path) -> bool {
    pid > 0 && imp::image(pid).is_some_and(|image| belongs(&image, root))
}

pub fn free_port() -> Option<u16> {
    let listener = std::net::TcpListener::bind(("127.0.0.1", 0)).ok()?;
    listener.local_addr().ok().map(|a| a.port())
}

pub fn total_mb() -> u64 {
    imp::total_mb()
}

pub fn cores() -> usize {
    std::thread::available_parallelism().map_or(2, usize::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_programs_from_our_folder_count_as_ours() {
        let root = Path::new("C:/Users/x/.wisply/local/bin");
        assert!(belongs("C:\\Users\\X\\.wisply\\local\\bin\\llama\\llama-server.exe", root));
        assert!(!belongs("C:\\Program Files\\other\\llama-server.exe", root));
        assert!(!belongs("anything", Path::new("")));
        assert!(!is_ours(0, root));
    }

    #[test]
    fn our_own_process_is_alive_and_has_a_size() {
        let me = std::process::id();
        assert!(alive(me));
        assert!(rss_mb(me).is_some());
        assert!(!alive(0));
    }

    #[test]
    fn the_installed_ram_is_read() {
        assert!(total_mb() >= 256);
    }

    #[test]
    fn a_free_port_is_found() {
        assert!(free_port().is_some_and(|p| p > 0));
    }
}
