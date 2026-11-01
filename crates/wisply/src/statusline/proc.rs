use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const POLL: Duration = Duration::from_millis(40);

pub struct Done {
    pub ok: bool,
    pub out: String,
    pub err: String,
}

#[cfg(windows)]
fn quiet(cmd: &mut Command) {
    use std::os::windows::process::CommandExt;
    cmd.creation_flags(0x0800_0000);
}

#[cfg(not(windows))]
fn quiet(_cmd: &mut Command) {}

fn drain(mut pipe: impl Read + Send + 'static) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        let mut bytes = Vec::new();
        let _ = pipe.read_to_end(&mut bytes);
        String::from_utf8_lossy(&bytes).into_owned()
    })
}

pub fn capture(cmd: &mut Command, limit: Duration) -> Option<Done> {
    quiet(cmd);
    let mut child = cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().ok()?;
    let out = drain(child.stdout.take()?);
    let err = drain(child.stderr.take()?);
    let started = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if started.elapsed() < limit => thread::sleep(POLL),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    };
    Some(Done { ok: status.success(), out: out.join().unwrap_or_default(), err: err.join().unwrap_or_default() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_program_is_none_and_a_quick_one_is_captured() {
        assert!(capture(&mut Command::new("wisply-no-such-program"), Duration::from_secs(2)).is_none());
        let exe = std::env::current_exe().unwrap();
        let done = capture(Command::new(exe).arg("--list"), Duration::from_secs(20)).unwrap();
        assert!(done.ok);
    }
}
