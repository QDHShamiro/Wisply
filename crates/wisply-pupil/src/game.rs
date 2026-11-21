use crate::config::Game;

pub fn classify(paths: &[String], game: &Game) -> Option<String> {
    for path in paths {
        if game.path_excludes.iter().any(|x| path.contains(x.as_str())) {
            continue;
        }
        let name = path.rsplit('\\').next().unwrap_or(path);
        let listed = game.exes.iter().any(|x| x == name);
        let suffix = game.exe_suffixes.iter().any(|x| name.ends_with(x.as_str()));
        let marker = game.path_markers.iter().any(|x| path.contains(x.as_str()));
        if listed || suffix || marker {
            return Some(name.to_string());
        }
    }
    None
}

pub fn is_minecraft(windows: &[(String, String)], game: &Game) -> bool {
    windows
        .iter()
        .any(|(title, exe)| title.starts_with(&game.minecraft_title) && game.minecraft_exes.iter().any(|x| x == exe))
}

pub fn fullscreen(state: i32, game: &Game, front_exe: &str, foreground: impl FnOnce() -> bool) -> bool {
    let not_a_game = game.fullscreen_ignore.iter().any(|x| x == front_exe);
    !not_a_game && game.fullscreen_states.contains(&state) && (state == game.d3d_state || foreground())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Kind {
    Fullscreen,
    Minecraft,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Signals {
    pub full: bool,
    pub afk: bool,
    pub idle: bool,
    pub front_minecraft: bool,
    pub minecraft: bool,
}

pub fn verdict(signals: Signals) -> Option<Kind> {
    let resting = signals.afk && (signals.idle || !signals.front_minecraft);
    if signals.full && !(signals.afk && signals.idle && signals.front_minecraft) {
        return Some(Kind::Fullscreen);
    }
    if signals.minecraft && !resting {
        return Some(Kind::Minecraft);
    }
    None
}

pub fn running(game: &Game, afk: bool) -> Option<String> {
    if !game.enabled {
        return None;
    }
    let front = system::foreground();
    let exe = front.1.clone();
    let signals = Signals {
        full: fullscreen(system::notification_state(), game, &front.1, || system::foreground_fullscreen(game)),
        afk,
        idle: system::idle_seconds() >= game.afk_idle_seconds,
        front_minecraft: is_minecraft(&[front], game),
        minecraft: is_minecraft(&system::visible_windows(), game),
    };
    match verdict(signals) {
        Some(Kind::Fullscreen) => Some(game.fullscreen_name.replace("{exe}", &exe)),
        Some(Kind::Minecraft) => Some(game.minecraft_name.clone()),
        None => classify(&system::process_paths(), game),
    }
}

#[cfg(windows)]
pub mod system {
    use windows_sys::Win32::Foundation::{BOOL, CloseHandle, HWND, LPARAM, RECT, STILL_ACTIVE};
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromWindow,
    };
    use windows_sys::Win32::System::ProcessStatus::K32EnumProcesses;
    use windows_sys::Win32::System::SystemInformation::GetTickCount;
    use windows_sys::Win32::System::Threading::{
        GetExitCodeProcess, OpenProcess, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_TERMINATE,
        QueryFullProcessImageNameW, TerminateProcess,
    };
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
    use windows_sys::Win32::UI::Shell::SHQueryUserNotificationState;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetClassNameW, GetForegroundWindow, GetWindowRect, GetWindowTextLengthW, GetWindowTextW,
        GetWindowThreadProcessId, IsWindowVisible,
    };

    use crate::config::Game;

    pub fn idle_seconds() -> f64 {
        unsafe {
            let mut info = LASTINPUTINFO { cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32, dwTime: 0 };
            if GetLastInputInfo(&mut info) == 0 {
                return 0.0;
            }
            f64::from(GetTickCount().wrapping_sub(info.dwTime)) / 1000.0
        }
    }

    fn window_title(window: HWND) -> String {
        unsafe {
            let length = GetWindowTextLengthW(window);
            if length <= 0 {
                return String::new();
            }
            let mut buffer = vec![0u16; length as usize + 1];
            let copied = GetWindowTextW(window, buffer.as_mut_ptr(), buffer.len() as i32);
            String::from_utf16_lossy(&buffer[..copied.max(0) as usize])
        }
    }

    fn window_exe(window: HWND) -> String {
        let mut pid = 0u32;
        unsafe { GetWindowThreadProcessId(window, &mut pid) };
        let path = process_path(pid);
        path.rsplit('\\').next().unwrap_or("").to_string()
    }

    pub fn foreground() -> (String, String) {
        let window = unsafe { GetForegroundWindow() };
        if window.is_null() {
            return (String::new(), String::new());
        }
        (window_title(window), window_exe(window))
    }

    pub fn notification_state() -> i32 {
        let mut state = 0;
        if unsafe { SHQueryUserNotificationState(&mut state) } == 0 { state } else { 0 }
    }

    pub fn foreground_fullscreen(game: &Game) -> bool {
        unsafe {
            let window = GetForegroundWindow();
            if window.is_null() {
                return false;
            }
            let mut class = [0u16; 64];
            let length = GetClassNameW(window, class.as_mut_ptr(), class.len() as i32);
            let class = String::from_utf16_lossy(&class[..length.max(0) as usize]);
            if game.shell_classes.contains(&class) {
                return false;
            }
            let mut rect: RECT = std::mem::zeroed();
            if GetWindowRect(window, &mut rect) == 0 {
                return false;
            }
            let mut info: MONITORINFO = std::mem::zeroed();
            info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
            if GetMonitorInfoW(MonitorFromWindow(window, MONITOR_DEFAULTTONEAREST), &mut info) == 0 {
                return false;
            }
            let screen = info.rcMonitor;
            rect.left <= screen.left
                && rect.top <= screen.top
                && rect.right >= screen.right
                && rect.bottom >= screen.bottom
        }
    }

    pub fn process_path(pid: u32) -> String {
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
            if handle.is_null() {
                return String::new();
            }
            let mut buffer = [0u16; 1024];
            let mut size = buffer.len() as u32;
            let ok = QueryFullProcessImageNameW(handle, PROCESS_NAME_WIN32, buffer.as_mut_ptr(), &mut size);
            CloseHandle(handle);
            if ok == 0 { String::new() } else { String::from_utf16_lossy(&buffer[..size as usize]).to_lowercase() }
        }
    }

    pub fn alive(pid: u32) -> bool {
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
            if handle.is_null() {
                return false;
            }
            let mut code = 0u32;
            let ok = GetExitCodeProcess(handle, &mut code);
            CloseHandle(handle);
            ok != 0 && code == STILL_ACTIVE as u32
        }
    }

    pub fn kill(pid: u32) -> bool {
        unsafe {
            let handle = OpenProcess(PROCESS_TERMINATE, 0, pid);
            if handle.is_null() {
                return false;
            }
            let ok = TerminateProcess(handle, 1);
            CloseHandle(handle);
            ok != 0
        }
    }

    pub fn process_paths() -> Vec<String> {
        let mut pids = vec![0u32; 8192];
        let mut needed = 0u32;
        let bytes = (pids.len() * std::mem::size_of::<u32>()) as u32;
        if unsafe { K32EnumProcesses(pids.as_mut_ptr(), bytes, &mut needed) } == 0 {
            return Vec::new();
        }
        pids.truncate(needed as usize / std::mem::size_of::<u32>());
        pids.into_iter().filter(|pid| *pid != 0).map(process_path).filter(|p| !p.is_empty()).collect()
    }

    unsafe extern "system" fn collect(window: HWND, data: LPARAM) -> BOOL {
        let found = unsafe { &mut *(data as *mut Vec<(String, u32)>) };
        unsafe {
            if IsWindowVisible(window) == 0 {
                return 1;
            }
            let length = GetWindowTextLengthW(window);
            if length <= 0 {
                return 1;
            }
            let mut buffer = vec![0u16; length as usize + 1];
            let copied = GetWindowTextW(window, buffer.as_mut_ptr(), buffer.len() as i32);
            let mut pid = 0u32;
            GetWindowThreadProcessId(window, &mut pid);
            found.push((String::from_utf16_lossy(&buffer[..copied.max(0) as usize]), pid));
        }
        1
    }

    pub fn visible_windows() -> Vec<(String, String)> {
        let mut found: Vec<(String, u32)> = Vec::new();
        unsafe { EnumWindows(Some(collect), &mut found as *mut _ as LPARAM) };
        found
            .into_iter()
            .map(|(title, pid)| {
                let path = process_path(pid);
                let name = path.rsplit('\\').next().unwrap_or("").to_string();
                (title, name)
            })
            .collect()
    }
}

#[cfg(not(windows))]
pub mod system {
    use crate::config::Game;

    pub fn notification_state() -> i32 {
        0
    }

    pub fn idle_seconds() -> f64 {
        0.0
    }

    pub fn foreground() -> (String, String) {
        (String::new(), String::new())
    }

    pub fn foreground_fullscreen(_game: &Game) -> bool {
        false
    }

    pub fn alive(pid: u32) -> bool {
        std::path::Path::new(&format!("/proc/{pid}")).exists()
    }

    pub fn kill(pid: u32) -> bool {
        std::process::Command::new("kill").arg(pid.to_string()).status().map(|s| s.success()).unwrap_or(false)
    }

    pub fn process_path(pid: u32) -> String {
        std::fs::read_link(format!("/proc/{pid}/exe")).map(|p| p.to_string_lossy().to_lowercase()).unwrap_or_default()
    }

    pub fn process_paths() -> Vec<String> {
        Vec::new()
    }

    pub fn visible_windows() -> Vec<(String, String)> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Kind, Signals, classify, fullscreen, is_minecraft, verdict};
    use crate::config::Game;

    fn game() -> Game {
        Game {
            enabled: true,
            minecraft_title: "Minecraft".into(),
            minecraft_exes: vec!["javaw.exe".into(), "java.exe".into()],
            exes: vec!["cs2.exe".into()],
            exe_suffixes: vec!["-win64-shipping.exe".into()],
            path_markers: vec!["\\steamapps\\common\\".into()],
            path_excludes: vec!["launcher".into(), "redist".into()],
            afk_idle_seconds: 120.0,
            fullscreen_states: vec![2, 3],
            fullscreen_ignore: vec!["discord.exe".into(), "brave.exe".into()],
            d3d_state: 3,
            shell_classes: vec!["Progman".into()],
            fullscreen_name: "fullscreen".into(),
            minecraft_name: "minecraft".into(),
        }
    }

    fn paths(items: &[&str]) -> Vec<String> {
        items.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn listed_games_suffixes_and_steam_folders_count() {
        let game = game();
        assert_eq!(classify(&paths(&["c:\\x\\cs2.exe"]), &game).as_deref(), Some("cs2.exe"));
        assert_eq!(
            classify(&paths(&["d:\\a\\foo-win64-shipping.exe"]), &game).as_deref(),
            Some("foo-win64-shipping.exe")
        );
        assert_eq!(classify(&paths(&["d:\\steamapps\\common\\x\\x.exe"]), &game).as_deref(), Some("x.exe"));
    }

    #[test]
    fn launchers_and_normal_programs_do_not_count() {
        let game = game();
        assert_eq!(classify(&paths(&["d:\\steamapps\\common\\x\\launcher.exe"]), &game), None);
        assert_eq!(classify(&paths(&["c:\\windows\\explorer.exe", "c:\\x\\code.exe"]), &game), None);
    }

    #[test]
    fn minecraft_needs_the_title_and_java() {
        let game = game();
        assert!(is_minecraft(&[("Minecraft* 1.21.11".into(), "javaw.exe".into())], &game));
        assert!(!is_minecraft(&[("Minecraft wiki - Brave".into(), "brave.exe".into())], &game));
        assert!(!is_minecraft(&[("Launcher".into(), "javaw.exe".into())], &game));
    }

    #[test]
    fn fullscreen_needs_a_busy_state() {
        let game = game();
        assert!(fullscreen(3, &game, "game.exe", || false));
        assert!(fullscreen(2, &game, "game.exe", || true));
        assert!(!fullscreen(2, &game, "game.exe", || false));
        assert!(!fullscreen(5, &game, "game.exe", || true));
        assert!(!fullscreen(2, &game, "discord.exe", || true));
        assert!(!fullscreen(3, &game, "brave.exe", || true));
        assert!(fullscreen(2, &game, "javaw.exe", || true));
    }

    #[test]
    fn away_players_may_leave_minecraft_running() {
        let base = Signals { minecraft: true, ..Signals::default() };
        let playing = Signals { front_minecraft: true, ..base };
        assert_eq!(verdict(base), Some(Kind::Minecraft));
        assert_eq!(verdict(Signals { afk: true, ..base }), None);
        assert_eq!(verdict(Signals { afk: true, ..playing }), Some(Kind::Minecraft));
        assert_eq!(verdict(Signals { afk: true, idle: true, ..playing }), None);
        let full = Signals { full: true, ..playing };
        assert_eq!(verdict(Signals { afk: true, ..full }), Some(Kind::Fullscreen));
        assert_eq!(verdict(Signals { afk: true, idle: true, ..full }), None);
        let other_full = Signals { full: true, afk: true, idle: true, ..base };
        assert_eq!(verdict(other_full), Some(Kind::Fullscreen));
        assert_eq!(verdict(Signals::default()), None);
    }
}
