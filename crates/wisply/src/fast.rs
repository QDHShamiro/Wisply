use std::env;
use std::io::{self, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use wisply_proto::{Conn, Reply, Request};

#[path = "hook.rs"]
mod hook;
#[path = "more.rs"]
mod more;

const MARKER: &str = "v3";
const FULL: &str = if cfg!(windows) { "wisply-full.exe" } else { "wisply-full" };
const HOOK_INPUT_WAIT: Duration = Duration::from_millis(1000);
const HOOK_INPUT_MAX: u64 = 64 * 1024;
const BUSY_WAIT: Duration = Duration::from_millis(15);
const BUSY_LIMIT: Duration = Duration::from_secs(12);
const LOCAL: &[&str] = &[
    "update",
    "setup",
    "install",
    "uninstall",
    "headless",
    "doctor",
    "ui",
    "squeeze",
    "mcp",
    "host",
    "statusline",
    "version",
    "--version",
    "daemon",
    "think",
    "voice-session",
    "voice-watch",
    "testapp",
    "claude-start",
];

fn main() -> ExitCode {
    let raw: Vec<String> = env::args_os().skip(1).map(|a| a.to_string_lossy().into_owned()).collect();
    let first = raw.first().map(String::as_str).unwrap_or("");
    let local = LOCAL.contains(&first) || first.starts_with("chrome-extension://") || raw.iter().any(|a| a == "--env");
    if first == "hook" && raw.get(1).is_some_and(|a| a == "post") {
        hook::post();
        return ExitCode::SUCCESS;
    }
    let home = wisply_proto::home();
    if local {
        return full(&raw, None);
    }
    let args: Vec<String> = match msys_root() {
        Some(root) => raw.iter().map(|a| unmangle(a, &root)).collect(),
        None => raw.clone(),
    };
    if first == "more" {
        return more::run(&home, &args[1..]);
    }
    if !home.join(MARKER).is_file() {
        return full(&raw, None);
    }
    let args = if matches!(first, "" | "-h" | "--help") {
        std::iter::once("help".to_string()).chain(args.into_iter().skip(1)).collect()
    } else {
        args
    };
    send(&home, &raw, &args)
}

fn full_path() -> Result<PathBuf, String> {
    let me = env::current_exe().map_err(|e| format!("cannot find itself ({e})"))?;
    let full = me.with_file_name(FULL);
    if full == me || !full.is_file() {
        return Err(format!(
            "{FULL} is missing next to {} -> try: wisply setup (run from the downloaded Wisply folder)",
            me.display()
        ));
    }
    Ok(full)
}

fn full(raw: &[String], input: Option<&str>) -> ExitCode {
    let exe = match full_path() {
        Ok(exe) => exe,
        Err(why) => {
            eprintln!("err: {why}");
            return ExitCode::from(127);
        }
    };
    let mut cmd = Command::new(exe);
    cmd.args(raw);
    if input.is_some() {
        cmd.stdin(Stdio::piped());
    }
    let Ok(mut child) = cmd.spawn() else {
        eprintln!("err: cannot start {FULL} -> try: wisply setup");
        return ExitCode::from(127);
    };
    if let (Some(text), Some(mut stdin)) = (input, child.stdin.take()) {
        let _ = stdin.write_all(text.as_bytes());
    }
    match child.wait() {
        Ok(s) => ExitCode::from(s.code().and_then(|c| u8::try_from(c).ok()).unwrap_or(1)),
        Err(_) => ExitCode::from(1),
    }
}

fn send(home: &Path, raw: &[String], args: &[String]) -> ExitCode {
    let first = args.first().map(String::as_str).unwrap_or("");
    let quiet = args.iter().any(|a| a == "--quiet");
    let event = args.get(1).map(String::as_str).unwrap_or("");
    let silent = matches!(first, "done" | "suggest" | "hook");
    let idle = match first {
        "stop" => Some("wisply daemon not running"),
        "done" if quiet => Some(""),
        "done" => Some("nothing to close"),
        "suggest" => Some(""),
        "hook" if event != "start" => Some(""),
        _ => None,
    };
    let hook = first == "hook" && args.len() == 2;
    let wants_input = (first == "suggest" && args.len() == 1) || (first == "done" && quiet) || hook;
    let hook_input = if wants_input && !io::stdin().is_terminal() { read_stdin(HOOK_INPUT_MAX) } else { String::new() };
    let mut sid = env::var("CLAUDE_CODE_SESSION_ID").unwrap_or_default();
    if sid.is_empty() {
        sid = json_str(&hook_input, "session_id");
    }
    let started = Instant::now();
    let mut stats = false;
    let mut argv = Vec::with_capacity(args.len() + 1);
    for a in args {
        if a == "--stats" {
            stats = true;
        } else {
            argv.push(a.clone());
        }
    }
    if (first == "suggest" || hook) && !hook_input.is_empty() {
        argv.push(hook_input.replace('\0', " "));
    }
    let pass = (!hook_input.is_empty()).then_some(hook_input.as_str());
    let Some(token) = wisply_proto::key_token(home) else {
        return if silent { ExitCode::SUCCESS } else { full(raw, pass) };
    };
    let cwd = env::current_dir().map(|p| p.display().to_string()).unwrap_or_default();
    let req = Request { token: &token, cwd: &cwd, sid: &sid, secret: None, argv: &argv }.encode();
    let deadline = Instant::now() + BUSY_LIMIT;
    let buf = loop {
        match wisply_proto::exchange_once(home, &req) {
            Ok(buf) if !buf.is_empty() => break buf,
            Ok(_) => return full(raw, pass),
            Err(Conn::Busy) if Instant::now() < deadline => thread::sleep(BUSY_WAIT),
            Err(Conn::Busy) => return full(raw, pass),
            Err(Conn::Missing) => match idle {
                Some(text) => break format!("0\0\0{text}").into_bytes(),
                None => return full(raw, pass),
            },
        }
    };
    let mut out = Reply::decode(&buf);
    if stats {
        out.out.push_str(&format!("\n[{} ms, ~{} tokens]", started.elapsed().as_millis(), out.out.len() / 4 + 1));
    }
    if !out.out.is_empty() {
        let mut so = io::stdout().lock();
        let _ = so.write_all(out.out.as_bytes());
        let _ = so.write_all(b"\n");
        let _ = so.flush();
    }
    ExitCode::from(u8::try_from(out.code).unwrap_or(1))
}

fn msys_root() -> Option<String> {
    env::var_os("MSYSTEM")?;
    let exe = env::var("EXEPATH").ok()?.replace('\\', "/");
    let root = exe.trim_end_matches('/');
    let root = root.strip_suffix("/usr/bin").or_else(|| root.strip_suffix("/bin")).unwrap_or(root);
    Some(root.to_string())
}

fn strip_root(value: &str, prefix: &str) -> Option<String> {
    let head = value.get(..prefix.len())?;
    head.eq_ignore_ascii_case(prefix).then(|| format!("/{}", &value[prefix.len()..]))
}

fn unmangle(arg: &str, root: &str) -> String {
    if arg.eq_ignore_ascii_case(root) {
        return "/".to_string();
    }
    let prefix = format!("{root}/");
    if let Some(v) = strip_root(arg, &prefix) {
        return v;
    }
    if let Some(i) = arg.find('=') {
        let (key, value) = arg.split_at(i + 1);
        if let Some(v) = strip_root(value, &prefix) {
            return format!("{key}{v}");
        }
    }
    arg.to_string()
}

fn json_str(text: &str, key: &str) -> String {
    let pat = format!("\"{key}\"");
    let Some(at) = text.find(&pat) else { return String::new() };
    let rest = text[at + pat.len()..].trim_start();
    let Some(rest) = rest.strip_prefix(':') else { return String::new() };
    let Some(rest) = rest.trim_start().strip_prefix('"') else { return String::new() };
    rest.split('"').next().unwrap_or("").to_string()
}

fn read_stdin(max: u64) -> String {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut raw = Vec::new();
        let _ = io::stdin().take(max + 1).read_to_end(&mut raw);
        raw.truncate(usize::try_from(max).unwrap_or(usize::MAX));
        let _ = tx.send(String::from_utf8_lossy(&raw).into_owned());
    });
    rx.recv_timeout(HOOK_INPUT_WAIT).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_git_bash_paths_like_the_full_client() {
        let root = "C:/Program Files/Git";
        assert_eq!(unmangle("C:/Program Files/Git/QDHShamiro", root), "/QDHShamiro");
        assert_eq!(unmangle("a=C:/Program Files/Git/b", root), "a=/b");
        assert_eq!(unmangle("C:/Users/x", root), "C:/Users/x");
    }

    #[test]
    fn reads_the_session_from_hook_json() {
        assert_eq!(json_str(r#"{"session_id": "4717b1cc"}"#, "session_id"), "4717b1cc");
        assert_eq!(json_str("{}", "session_id"), "");
    }
}
