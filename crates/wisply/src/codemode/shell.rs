use std::env;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use base64::Engine;
use serde_json::Value;

const STREAM_MAX: usize = 4 << 20;
const POLL: Duration = Duration::from_millis(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Bash,
    #[cfg_attr(windows, allow(dead_code))]
    Sh,
    Power,
}

#[derive(Debug, Clone)]
pub struct Shell {
    pub kind: Kind,
    pub path: PathBuf,
}

impl Shell {
    pub fn name(&self) -> &'static str {
        match self.kind {
            Kind::Bash => "bash",
            Kind::Sh => "sh",
            Kind::Power => "powershell",
        }
    }
}

pub struct Output {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

pub struct Job<'a> {
    pub line: &'a str,
    pub cwd: &'a Path,
    pub env: &'a [(String, String)],
    pub stdin: Option<&'a str>,
    pub until: Instant,
    pub stop: &'a AtomicBool,
}

fn on_path(name: &str) -> Option<PathBuf> {
    let exe = if cfg!(windows) { format!("{name}.exe") } else { name.to_string() };
    env::split_paths(&env::var_os("PATH")?).map(|dir| dir.join(&exe)).find(|p| p.is_file())
}

#[cfg(windows)]
fn windows_bash() -> Option<PathBuf> {
    if let Some(path) = env::var_os("CLAUDE_CODE_GIT_BASH_PATH").map(PathBuf::from).filter(|p| p.is_file()) {
        return Some(path);
    }
    let usable = |p: &Path| {
        let low = p.to_string_lossy().to_lowercase();
        !low.contains("\\system32\\") && !low.contains("\\windowsapps\\")
    };
    if let Some(path) = on_path("bash").filter(|p| usable(p)) {
        return Some(path);
    }
    let roots = ["ProgramFiles", "ProgramW6432", "ProgramFiles(x86)", "LOCALAPPDATA"];
    roots
        .iter()
        .filter_map(|key| env::var_os(key).map(PathBuf::from))
        .flat_map(|root| [root.join("Git/bin/bash.exe"), root.join("Programs/Git/bin/bash.exe")])
        .find(|p| p.is_file())
}

pub fn find() -> Shell {
    #[cfg(windows)]
    {
        if let Some(path) = windows_bash() {
            return Shell { kind: Kind::Bash, path };
        }
        let path = on_path("pwsh").or_else(|| on_path("powershell")).unwrap_or_else(|| "powershell".into());
        Shell { kind: Kind::Power, path }
    }
    #[cfg(not(windows))]
    {
        match on_path("bash") {
            Some(path) => Shell { kind: Kind::Bash, path },
            None => Shell { kind: Kind::Sh, path: on_path("sh").unwrap_or_else(|| "/bin/sh".into()) },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Quote {
    Out,
    Single,
    Double,
}

fn state_after(kind: Kind, text: &str, mut state: Quote) -> Quote {
    let escape = if kind == Kind::Power { '`' } else { '\\' };
    let mut chars = text.chars();
    while let Some(ch) = chars.next() {
        state = match (state, ch) {
            (Quote::Single, '\'') => Quote::Out,
            (Quote::Single, _) => Quote::Single,
            (s, c) if c == escape => {
                chars.next();
                s
            }
            (Quote::Double, '"') => Quote::Out,
            (Quote::Out, '"') => Quote::Double,
            (Quote::Out, '\'') => Quote::Single,
            (s, _) => s,
        };
    }
    state
}

fn quote(kind: Kind, value: &str, state: Quote) -> String {
    match (kind, state) {
        (Kind::Power, Quote::Out) => format!("'{}'", value.replace('\'', "''")),
        (Kind::Power, Quote::Single) => value.replace('\'', "''"),
        (Kind::Power, Quote::Double) => value.chars().fold(String::new(), |mut out, c| {
            if matches!(c, '`' | '"' | '$') {
                out.push('`');
            }
            out.push(c);
            out
        }),
        (_, Quote::Out)
            if !value.is_empty() && value.chars().all(|c| c.is_ascii_alphanumeric() || "-_./:=@%+,".contains(c)) =>
        {
            value.to_string()
        }
        (_, Quote::Out) => format!("'{}'", value.replace('\'', r"'\''")),
        (_, Quote::Single) => value.replace('\'', r"'\''"),
        (_, Quote::Double) => value.chars().fold(String::new(), |mut out, c| {
            if matches!(c, '\\' | '"' | '$' | '`') {
                out.push('\\');
            }
            out.push(c);
            out
        }),
    }
}

pub fn line(kind: Kind, parts: &[Value]) -> String {
    let mut out = String::new();
    let mut state = Quote::Out;
    for part in parts {
        if let Some(raw) = part.get("raw").and_then(Value::as_str) {
            out.push_str(raw);
            state = state_after(kind, raw, state);
        } else if let Some(arg) = part.get("arg").and_then(Value::as_str) {
            out.push_str(&quote(kind, arg, state));
        } else if let Some(args) = part.get("args").and_then(Value::as_array) {
            let each: Vec<String> = args.iter().map(|a| quote(kind, a.as_str().unwrap_or_default(), state)).collect();
            out.push_str(&each.join(" "));
        }
    }
    out
}

fn command(shell: &Shell, line: &str) -> Command {
    let mut cmd = Command::new(&shell.path);
    match shell.kind {
        Kind::Power => {
            let script = format!(
                "[Console]::OutputEncoding = [Text.Encoding]::UTF8; $ProgressPreference = 'SilentlyContinue'; {line}\nexit $LASTEXITCODE"
            );
            let wide: Vec<u8> = script.encode_utf16().flat_map(u16::to_le_bytes).collect();
            let encoded = base64::engine::general_purpose::STANDARD.encode(wide);
            cmd.args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-EncodedCommand", &encoded]);
        }
        _ => {
            cmd.arg("-c").arg(line);
        }
    }
    cmd
}

fn pump(from: Option<impl Read + Send + 'static>) -> thread::JoinHandle<String> {
    thread::spawn(move || {
        from.map(|r| wisply_squeeze::decode(&wisply_squeeze::read_limited(r, STREAM_MAX))).unwrap_or_default()
    })
}

pub fn kill_tree(pid: u32) {
    #[cfg(unix)]
    if let Ok(p) = i32::try_from(pid) {
        unsafe extern "C" {
            fn kill(pid: i32, sig: i32) -> i32;
        }
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

fn watch(child: &mut Child, until: Instant, stop: &AtomicBool) -> Result<i32, String> {
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status.code().unwrap_or(1)),
            Ok(None) => {}
            Err(e) => return Err(format!("could not wait for the command ({e})")),
        }
        let late = Instant::now() >= until;
        if late || stop.load(Ordering::Relaxed) {
            kill_tree(child.id());
            let _ = child.wait();
            return Err(if late { "the command ran out of time and was stopped".into() } else { "stopped".into() });
        }
        thread::sleep(POLL);
    }
}

pub fn wait_for(mut cmd: Command, stdin: Option<&str>, until: Instant, stop: &AtomicBool) -> Result<Output, String> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    cmd.stdin(if stdin.is_some() { Stdio::piped() } else { Stdio::null() });
    #[cfg(unix)]
    std::os::unix::process::CommandExt::process_group(&mut cmd, 0);
    crate::v2::hide(&mut cmd);
    let name = cmd.get_program().to_string_lossy().into_owned();
    let mut child = cmd.spawn().map_err(|e| format!("could not start {name} ({e})"))?;
    if let (Some(text), Some(mut pipe)) = (stdin, child.stdin.take()) {
        let text = text.to_string();
        thread::spawn(move || {
            let _ = pipe.write_all(text.as_bytes());
        });
    }
    let (out, err) = (pump(child.stdout.take()), pump(child.stderr.take()));
    let code = watch(&mut child, until, stop);
    let (stdout, stderr) = (out.join().unwrap_or_default(), err.join().unwrap_or_default());
    code.map(|code| Output { code, stdout, stderr })
}

pub fn run(shell: &Shell, job: &Job) -> Result<Output, String> {
    let mut cmd = command(shell, job.line);
    cmd.current_dir(job.cwd).envs(job.env.iter().map(|(k, v)| (k.as_str(), v.as_str())));
    wait_for(cmd, job.stdin, job.until, job.stop)
}
