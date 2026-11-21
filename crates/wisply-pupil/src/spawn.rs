use std::ffi::OsStr;
use std::process::Command;

#[cfg(windows)]
const NO_WINDOW: u32 = windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;
#[cfg(windows)]
const DETACHED: u32 = windows_sys::Win32::System::Threading::DETACHED_PROCESS
    | windows_sys::Win32::System::Threading::CREATE_NEW_PROCESS_GROUP;

pub fn quiet(program: impl AsRef<OsStr>) -> Command {
    let mut command = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(NO_WINDOW);
    }
    command
}

pub fn ear(exe: &std::path::Path) -> Command {
    if !exe.as_os_str().is_empty() {
        return quiet(exe);
    }
    let me = std::env::current_exe().unwrap_or_default();
    let mut command = quiet(me);
    command.args(["pupil", "hear"]);
    command
}

pub fn detached(program: impl AsRef<OsStr>) -> Command {
    let mut command = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(DETACHED | NO_WINDOW);
    }
    command
}

#[cfg(windows)]
pub fn seal_std_handles() {
    use windows_sys::Win32::Foundation::{HANDLE_FLAG_INHERIT, INVALID_HANDLE_VALUE, SetHandleInformation};
    use windows_sys::Win32::System::Console::{GetStdHandle, STD_ERROR_HANDLE, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
    for which in [STD_INPUT_HANDLE, STD_OUTPUT_HANDLE, STD_ERROR_HANDLE] {
        unsafe {
            let handle = GetStdHandle(which);
            if !handle.is_null() && handle != INVALID_HANDLE_VALUE {
                SetHandleInformation(handle, HANDLE_FLAG_INHERIT, 0);
            }
        }
    }
}

#[cfg(not(windows))]
pub fn seal_std_handles() {}
