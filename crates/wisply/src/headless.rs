use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

use serde_json::Value;

use crate::daemon::net;

const INDEX: &str =
    "https://googlechromelabs.github.io/chrome-for-testing/last-known-good-versions-with-downloads.json";
const WAIT: Duration = Duration::from_secs(45);
const PID: &str = "headless.pid";
const PROFILE: &str = "headless-profile";
const LOG: &str = "headless.log";
const HOST: &str = "dev.qdhshamiro.wisply.json";

struct Flags {
    browser: Option<String>,
    no_sandbox: bool,
}

fn flags(args: &[String]) -> Flags {
    let mut out = Flags { browser: None, no_sandbox: cfg!(target_os = "linux") && env::var_os("CI").is_some() };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--browser" => out.browser = it.next().cloned(),
            "--no-sandbox" => out.no_sandbox = true,
            _ => {}
        }
    }
    out
}

pub fn run(home: &Path, args: &[String]) -> ExitCode {
    let sub = args.first().map_or("status", String::as_str);
    let rest = args.get(1..).unwrap_or_default();
    let result = match sub {
        "start" => start(home, &flags(rest)),
        "stop" => stop(home),
        "status" => Ok(status(home)),
        "install" => install(home),
        _ => Err("usage: wisply headless [start|stop|status|install] [--browser PATH] [--no-sandbox]".into()),
    };
    match result {
        Ok(line) => {
            println!("{line}");
            ExitCode::SUCCESS
        }
        Err(why) => {
            println!("err: {why}");
            ExitCode::from(1)
        }
    }
}

fn exe_in_download() -> &'static str {
    if cfg!(windows) {
        "chrome-win64/chrome.exe"
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            "chrome-mac-arm64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing"
        } else {
            "chrome-mac-x64/Google Chrome for Testing.app/Contents/MacOS/Google Chrome for Testing"
        }
    } else {
        "chrome-linux64/chrome"
    }
}

fn platform_key() -> Result<&'static str, String> {
    if cfg!(windows) {
        Ok("win64")
    } else if cfg!(target_os = "macos") {
        Ok(if cfg!(target_arch = "aarch64") { "mac-arm64" } else { "mac-x64" })
    } else if cfg!(target_arch = "x86_64") {
        Ok("linux64")
    } else {
        Err("Chrome for Testing has no build for this system, pass --browser PATH".into())
    }
}

fn own_browser(home: &Path) -> PathBuf {
    exe_in_download().split('/').fold(home.join("chrome"), |p, part| p.join(part))
}

fn on_path(name: &str) -> Option<PathBuf> {
    let dirs = env::var_os("PATH")?;
    env::split_paths(&dirs).map(|d| d.join(name)).find(|p| p.is_file())
}

fn find_browser(home: &Path, flag: Option<&str>) -> Result<PathBuf, String> {
    if let Some(p) = flag.map(PathBuf::from).or_else(|| env::var_os("WISPLY_CHROME").map(PathBuf::from)) {
        return if p.is_file() { Ok(p) } else { Err(format!("{} does not exist", p.display())) };
    }
    let own = own_browser(home);
    if own.is_file() {
        return Ok(own);
    }
    ["chromium", "chromium-browser"]
        .iter()
        .find_map(|n| on_path(n))
        .ok_or_else(|| "no Chromium found -> try: wisply headless install".into())
}

fn extract(archive: &Path, into: &Path) -> Result<(), String> {
    if cfg!(target_os = "linux") {
        let mut cmd = Command::new("unzip");
        cmd.arg("-q").arg("-o").arg(archive).arg("-d").arg(into).stdin(Stdio::null());
        if cmd.output().is_ok_and(|o| o.status.success()) {
            return Ok(());
        }
    }
    crate::update::unpack(archive, into)
}

fn install(home: &Path) -> Result<String, String> {
    let key = platform_key()?;
    let index: Value =
        serde_json::from_str(&net::text(INDEX)?).map_err(|_| "the Chrome for Testing list is not JSON")?;
    let version = index["channels"]["Stable"]["version"].as_str().unwrap_or("?").to_string();
    let url = index["channels"]["Stable"]["downloads"]["chrome"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|d| d["platform"].as_str() == Some(key))
        .and_then(|d| d["url"].as_str())
        .ok_or("the Chrome for Testing list has no build for this system")?
        .to_string();
    let work = home.join("chrome-install");
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(&work).map_err(|e| format!("cannot write {} ({e})", work.display()))?;
    let archive = work.join("chrome.zip");
    net::fetch(&url, &archive)?;
    let unpacked = work.join("out");
    fs::create_dir_all(&unpacked).map_err(|e| e.to_string())?;
    extract(&archive, &unpacked)?;
    let _ = fs::remove_file(&archive);
    let target = home.join("chrome");
    let _ = fs::remove_dir_all(&target);
    fs::rename(&unpacked, &target).map_err(|e| format!("cannot place Chrome ({e})"))?;
    let _ = fs::remove_dir_all(&work);
    let exe = own_browser(home);
    if !exe.is_file() {
        return Err(format!("Chrome unpacked but {} is missing", exe.display()));
    }
    Ok(format!("ok (Chrome for Testing {version} in {})", target.display()))
}

fn read_pid(home: &Path) -> Option<u32> {
    fs::read_to_string(home.join(PID)).ok()?.trim().parse().ok()
}

#[cfg(unix)]
unsafe extern "C" {
    fn kill(pid: i32, sig: i32) -> i32;
}

fn alive(pid: u32) -> bool {
    #[cfg(unix)]
    {
        i32::try_from(pid).is_ok_and(|p| unsafe { kill(p, 0) } == 0)
    }
    #[cfg(not(unix))]
    {
        let mut cmd = Command::new("tasklist");
        cmd.args(["/FI", &format!("PID eq {pid}"), "/NH"]).stdin(Stdio::null());
        crate::v2::hide(&mut cmd);
        cmd.output().is_ok_and(|o| String::from_utf8_lossy(&o.stdout).contains(&pid.to_string()))
    }
}

fn kill_tree(pid: u32) {
    #[cfg(unix)]
    if let Ok(p) = i32::try_from(pid) {
        unsafe {
            kill(-p, 9);
            kill(p, 9);
        }
    }
    #[cfg(not(unix))]
    {
        let mut cmd = Command::new("taskkill");
        cmd.args(["/T", "/F", "/PID", &pid.to_string()])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        crate::v2::hide(&mut cmd);
        let _ = cmd.status();
    }
}

fn browsers(home: &Path) -> usize {
    crate::client::ask(home, &["status"])
        .map(|(_, out)| out.lines().filter(|l| l.starts_with("browser ") && l.contains("| ext")).count())
        .unwrap_or(0)
}

fn quoted(s: &str) -> String {
    if cfg!(windows) { format!("\"{s}\"") } else { s.to_string() }
}

fn start(home: &Path, f: &Flags) -> Result<String, String> {
    if let Some(pid) = read_pid(home).filter(|p| alive(*p)) {
        return Ok(format!("ok (headless browser already runs, pid {pid})"));
    }
    let exe = find_browser(home, f.browser.as_deref())?;
    let extension = home.join("extension");
    if !extension.join("manifest.json").is_file() {
        return Err("the extension is missing -> try: wisply setup".into());
    }
    let profile = home.join(PROFILE);
    let hosts = profile.join("NativeMessagingHosts");
    fs::create_dir_all(&hosts).map_err(|e| format!("cannot write {} ({e})", hosts.display()))?;
    fs::copy(home.join(HOST), hosts.join(HOST)).map_err(|_| "the native host file is missing -> try: wisply setup")?;
    let before = browsers(home);
    let mut args: Vec<String> = vec![
        "--headless=new".into(),
        format!("--user-data-dir={}", quoted(&profile.display().to_string())),
        format!("--load-extension={}", quoted(&extension.display().to_string())),
        "--disable-features=DisableLoadExtensionCommandLineSwitch".into(),
        "--no-first-run".into(),
        "--no-default-browser-check".into(),
        "--disable-search-engine-choice-screen".into(),
        "--disable-gpu".into(),
        "--window-size=1300,900".into(),
    ];
    if f.no_sandbox {
        args.push("--no-sandbox".into());
    }
    args.push("about:blank".into());
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    let pid = crate::spawn::detached_pid(&exe, &refs, home, &home.join(LOG)).ok_or("the browser would not start")?;
    fs::write(home.join(PID), pid.to_string()).map_err(|e| e.to_string())?;
    let t0 = Instant::now();
    while t0.elapsed() < WAIT {
        if browsers(home) > before {
            return Ok(format!("ok (headless browser connected, pid {pid})"));
        }
        if !alive(pid) {
            let _ = fs::remove_file(home.join(PID));
            return Err(format!("the browser quit right away, see {}", home.join(LOG).display()));
        }
        sleep(Duration::from_millis(500));
    }
    Err(format!(
        "the browser runs (pid {pid}) but the extension did not connect in {} s, see {}",
        WAIT.as_secs(),
        home.join(LOG).display()
    ))
}

fn stop(home: &Path) -> Result<String, String> {
    let Some(pid) = read_pid(home) else { return Ok("ok (no headless browser was started)".into()) };
    kill_tree(pid);
    let _ = fs::remove_file(home.join(PID));
    Ok(format!("ok (headless browser stopped, pid {pid})"))
}

fn status(home: &Path) -> String {
    let browser = match find_browser(home, None) {
        Ok(p) => p.display().to_string(),
        Err(why) => why,
    };
    match read_pid(home).filter(|p| alive(*p)) {
        Some(pid) => format!("headless browser runs (pid {pid}); browser {browser}"),
        None => format!("headless browser is off; browser {browser}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags_read_browser_and_sandbox() {
        let f = flags(&["--browser".into(), "x".into(), "--no-sandbox".into()]);
        assert_eq!(f.browser.as_deref(), Some("x"));
        assert!(f.no_sandbox);
    }

    #[test]
    fn a_missing_browser_flag_path_is_an_error() {
        let home = env::temp_dir();
        assert!(find_browser(&home, Some("/definitely/not/here")).is_err());
    }

    #[test]
    fn the_download_path_points_inside_the_chrome_folder() {
        let p = own_browser(Path::new("h"));
        assert!(p.starts_with("h") && p.components().count() > 3);
    }
}
