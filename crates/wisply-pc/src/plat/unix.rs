use std::io;

pub fn priority(pid: u32) -> Option<i32> {
    clear_errno();
    let value = unsafe { libc::getpriority(libc::PRIO_PROCESS, pid as libc::id_t) };
    (value != -1 || io::Error::last_os_error().raw_os_error() == Some(0)).then_some(value)
}

pub fn set_priority(pid: u32, level: i32) -> bool {
    pid != std::process::id() && unsafe { libc::setpriority(libc::PRIO_PROCESS, pid as libc::id_t, level) } == 0
}

pub fn signal(pid: u32, sig: i32) -> bool {
    pid != std::process::id() && pid > 1 && unsafe { libc::kill(pid as libc::pid_t, sig) } == 0
}

pub fn suspend(pid: u32) -> bool {
    signal(pid, libc::SIGSTOP)
}

pub fn resume(pid: u32) -> bool {
    signal(pid, libc::SIGCONT)
}

pub fn kill(pid: u32) -> bool {
    signal(pid, libc::SIGTERM)
}

#[cfg(target_os = "linux")]
fn clear_errno() {
    unsafe { *libc::__errno_location() = 0 };
}

#[cfg(not(target_os = "linux"))]
fn clear_errno() {
    unsafe { *libc::__error() = 0 };
}
