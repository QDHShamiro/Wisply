use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

#[cfg(windows)]
const NO_WINDOW: u32 = 0x0800_0000;
const POLL: Duration = Duration::from_millis(15);

pub fn command(program: &str, args: &[&str]) -> Command {
    let mut cmd = Command::new(program);
    cmd.args(args).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(NO_WINDOW);
    }
    cmd
}

pub fn run(program: &str, args: &[&str], limit: Duration) -> Option<String> {
    let mut child = command(program, args).spawn().ok()?;
    let mut pipe = child.stdout.take()?;
    let reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = pipe.read_to_end(&mut bytes);
        bytes
    });
    let end = Instant::now() + limit;
    let ok = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status.success(),
            Ok(None) if Instant::now() < end => thread::sleep(POLL),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                break false;
            }
        }
    };
    let bytes = reader.join().ok()?;
    ok.then(|| String::from_utf8_lossy(&bytes).into_owned())
}

pub fn succeeds(program: &str, args: &[&str], limit: Duration) -> bool {
    run(program, args, limit).is_some()
}

pub fn exists(program: &str) -> bool {
    let probe = if cfg!(windows) { "where" } else { "which" };
    succeeds(probe, &[program], Duration::from_secs(3))
}

pub fn detached(program: &str, args: &[&str]) -> bool {
    let mut cmd = command(program, args);
    cmd.stdout(Stdio::null());
    cmd.spawn().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_comes_back_and_failures_are_none() {
        let echo = if cfg!(windows) { ("cmd", vec!["/C", "echo hi"]) } else { ("sh", vec!["-c", "echo hi"]) };
        let out = run(echo.0, &echo.1, Duration::from_secs(10)).unwrap();
        assert_eq!(out.trim(), "hi");
        assert!(run("definitely-not-a-program-xyz", &[], Duration::from_secs(1)).is_none());
    }
}
