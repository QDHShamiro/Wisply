use std::ffi::OsStr;
use std::process::Command;

#[cfg(windows)]
const NO_WINDOW: u32 = windows_sys::Win32::System::Threading::CREATE_NO_WINDOW;

pub fn quiet(program: impl AsRef<OsStr>) -> Command {
    let mut command = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(NO_WINDOW);
    }
    command
}

#[cfg(windows)]
fn peak_of(handle: windows_sys::Win32::Foundation::HANDLE) -> Option<f64> {
    use windows_sys::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};
    let mut counters: PROCESS_MEMORY_COUNTERS = unsafe { std::mem::zeroed() };
    counters.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32;
    let ok = unsafe { GetProcessMemoryInfo(handle, &mut counters, counters.cb) };
    (ok != 0).then(|| counters.PeakWorkingSetSize as f64 / 1_048_576.0)
}

#[cfg(windows)]
pub fn peak_ram_mb(child: &std::process::Child) -> Option<f64> {
    use std::os::windows::io::AsRawHandle;
    peak_of(child.as_raw_handle() as _)
}

#[cfg(not(windows))]
pub fn peak_ram_mb(_child: &std::process::Child) -> Option<f64> {
    None
}

#[cfg(windows)]
pub fn own_peak_ram_mb() -> Option<f64> {
    use windows_sys::Win32::System::Threading::GetCurrentProcess;
    peak_of(unsafe { GetCurrentProcess() })
}

#[cfg(not(windows))]
pub fn own_peak_ram_mb() -> Option<f64> {
    None
}
