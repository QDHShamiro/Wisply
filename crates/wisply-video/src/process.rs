use std::env;
use std::io::{ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use crate::fail::{Fail, Kind};

const TICK: Duration = Duration::from_millis(20);
const TAIL_LINES: usize = 20;
const STOPPED: i32 = -1;
const SLOW_HINT: &str = "try again; if it keeps happening, raise the timeout in video.toml";
#[cfg(windows)]
const NO_WINDOW: u32 = 0x0800_0000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ran {
    pub code: i32,
    pub stdout: Vec<u8>,
    pub stderr: String,
}

impl Ran {
    pub fn ok(&self) -> bool {
        self.code == 0
    }

    pub fn stderr_tail(&self) -> String {
        let kept: Vec<&str> = self.stderr.lines().map(str::trim).filter(|line| !line.is_empty()).collect();
        kept[kept.len().saturating_sub(TAIL_LINES)..].join("\n")
    }
}

pub trait Runner {
    fn run(&self, argv: &[String], limit_seconds: f64, label: &str) -> Result<Ran, Fail>;
}

pub struct System;

pub fn file_name(name: &str) -> String {
    if cfg!(windows) { format!("{name}.exe") } else { name.to_string() }
}

pub fn find_programs(name: &str, also: &Path) -> Vec<PathBuf> {
    let file = file_name(name);
    let folders: Vec<PathBuf> = env::var_os("PATH").map(|path| env::split_paths(&path).collect()).unwrap_or_default();
    let mut found: Vec<PathBuf> = Vec::new();
    for folder in folders.iter().map(PathBuf::as_path).chain([also]) {
        let program = folder.join(&file);
        if program.is_file() && !found.contains(&program) {
            found.push(program);
        }
    }
    found
}

fn quiet(program: &str) -> Command {
    #[cfg_attr(not(windows), allow(unused_mut))]
    let mut command = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(NO_WINDOW);
    }
    command
}

fn drain(mut pipe: impl Read + Send + 'static) -> thread::JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut raw = Vec::new();
        let _ = pipe.read_to_end(&mut raw);
        raw
    })
}

fn wait(child: &mut Child, limit: Duration) -> Option<i32> {
    let started = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Some(status.code().unwrap_or(STOPPED)),
            Ok(None) if started.elapsed() < limit => thread::sleep(TICK),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    }
}

fn collected(handle: Option<thread::JoinHandle<Vec<u8>>>) -> Vec<u8> {
    handle.and_then(|handle| handle.join().ok()).unwrap_or_default()
}

impl Runner for System {
    fn run(&self, argv: &[String], limit_seconds: f64, label: &str) -> Result<Ran, Fail> {
        let Some((program, rest)) = argv.split_first() else {
            return Err(Fail::new(Kind::ToolMissing, format!("{label} has no command to start")));
        };
        let mut command = quiet(program);
        command.args(rest).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = command.spawn().map_err(|error| match error.kind() {
            ErrorKind::NotFound => {
                Fail::new(Kind::ToolMissing, format!("{label} could not be started ({program} is missing)"))
            }
            _ => Fail::new(Kind::Service, format!("{label} could not be started ({error})")),
        })?;
        let stdout = child.stdout.take().map(drain);
        let stderr = child.stderr.take().map(drain);
        let code = wait(&mut child, Duration::from_secs_f64(limit_seconds.max(0.0)));
        let (stdout, stderr) = (collected(stdout), collected(stderr));
        match code {
            Some(code) => Ok(Ran { code, stdout, stderr: String::from_utf8_lossy(&stderr).into_owned() }),
            None => {
                let message = format!("{label} took longer than {limit_seconds} s and was stopped");
                Err(Fail::hint(Kind::Service, message, SLOW_HINT))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HELPER: &str = "process::tests::a_helper_that_sleeps";
    const HELPER_SECONDS: u64 = 20;

    fn own(args: &[&str]) -> Vec<String> {
        let exe = env::current_exe().expect("test program").to_string_lossy().into_owned();
        std::iter::once(exe).chain(args.iter().map(|arg| arg.to_string())).collect()
    }

    #[test]
    #[ignore = "started by the timeout test as a slow program"]
    fn a_helper_that_sleeps() {
        thread::sleep(Duration::from_secs(HELPER_SECONDS));
    }

    #[test]
    fn a_program_gives_its_output_and_exit_code() {
        let ran = System.run(&own(&["--list", "--ignored"]), 60.0, "helper").expect("ran");
        assert!(ran.ok(), "{}", ran.stderr);
        assert!(String::from_utf8_lossy(&ran.stdout).contains(HELPER));
        let failed = System.run(&own(&["--no-such-option"]), 60.0, "helper").expect("ran");
        assert!(!failed.ok());
        assert!(!failed.stderr_tail().is_empty());
    }

    #[test]
    fn a_slow_program_is_stopped() {
        let started = Instant::now();
        let fail = System.run(&own(&["--ignored", "--exact", HELPER]), 0.3, "helper").expect_err("stopped");
        assert_eq!(fail.kind, Kind::Service);
        assert_eq!(fail.message, "helper took longer than 0.3 s and was stopped");
        assert!(started.elapsed() < Duration::from_secs(HELPER_SECONDS / 2));
    }

    #[test]
    fn a_missing_program_is_named() {
        let fail = System.run(&["wisply-no-such-program".to_string()], 5.0, "yt-dlp").expect_err("missing");
        assert_eq!(fail.kind, Kind::ToolMissing);
        assert_eq!(fail.message, "yt-dlp could not be started (wisply-no-such-program is missing)");
        assert_eq!(System.run(&[], 5.0, "yt-dlp").expect_err("empty").kind, Kind::ToolMissing);
    }

    #[test]
    fn only_the_end_of_the_error_text_is_kept() {
        let lines: Vec<String> = (1..=30).map(|number| format!("  line {number}  ")).collect();
        let ran = Ran { code: 1, stdout: Vec::new(), stderr: format!("{}\n\n", lines.join("\n\n")) };
        let tail = ran.stderr_tail();
        assert_eq!(tail.lines().count(), TAIL_LINES);
        assert!(tail.starts_with("line 11\n") && tail.ends_with("line 30"));
    }

    #[test]
    fn a_program_is_found_next_to_wisply_when_it_is_not_on_the_path() {
        let folder = env::temp_dir().join(format!("wisply-video-process-{}", std::process::id()));
        std::fs::create_dir_all(&folder).expect("folder");
        assert!(find_programs("wisply-no-such-program", &folder).is_empty());
        let own = folder.join(file_name("wisply-no-such-program"));
        std::fs::write(&own, b"x").expect("write");
        assert_eq!(find_programs("wisply-no-such-program", &folder), [own]);
        std::fs::remove_dir_all(&folder).expect("clean");
    }
}
