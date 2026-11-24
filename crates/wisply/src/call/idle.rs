#[cfg(windows)]
pub fn secs() -> Option<u64> {
    use windows::Win32::System::SystemInformation::GetTickCount;
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
    let mut info = LASTINPUTINFO { cbSize: size_of::<LASTINPUTINFO>() as u32, dwTime: 0 };
    if !unsafe { GetLastInputInfo(&raw mut info) }.as_bool() {
        return None;
    }
    Some(u64::from(unsafe { GetTickCount() }.wrapping_sub(info.dwTime)) / 1000)
}

#[cfg(not(windows))]
pub fn secs() -> Option<u64> {
    None
}
