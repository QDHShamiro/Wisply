use std::fmt;
use std::io::SeekFrom;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use russh::client;
use russh::keys::{HashAlg, PrivateKeyWithHashAlg, PublicKeyOrCertificate, load_secret_key};
use russh_sftp::client::SftpSession;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use super::{Auth, Profile};

const FIRST_TAIL: u64 = 256 * 1024;
const MAX_READ: u64 = 2 * 1024 * 1024;
const HEAD_BYTES: u64 = 240;
const CRASH_BYTES: u64 = 64 * 1024;
const CRASHES_PER_CHECK: usize = 3;
const CRASH_FRESH_SECS: u64 = 86_400;
const IDLE: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, Default)]
pub struct Want<'a> {
    pub log: &'a str,
    pub offset: u64,
    pub head: &'a str,
    pub crash_dir: Option<&'a str>,
    pub crashes_after: &'a str,
    pub host_key: &'a str,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Crash {
    pub name: String,
    pub text: String,
}

#[derive(Debug, Clone, Default)]
pub struct Got {
    pub offset: u64,
    pub head: String,
    pub text: String,
    pub rotated: bool,
    pub crashes: Vec<Crash>,
    pub newest_crash: String,
    pub host_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Problem {
    Unreachable(String),
    Refused,
    HostKeyChanged,
    NoLog(String),
    Timeout,
    Failed(String),
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Problem::Unreachable(why) => write!(f, "unreachable ({why})"),
            Problem::Refused => write!(f, "login refused"),
            Problem::HostKeyChanged => write!(f, "host key changed, check the server before trusting it again"),
            Problem::NoLog(path) => write!(f, "log not found: {path}"),
            Problem::Timeout => write!(f, "timed out"),
            Problem::Failed(why) => write!(f, "{why}"),
        }
    }
}

struct Keys {
    want: String,
    seen: Arc<Mutex<String>>,
}

fn print(key: &PublicKeyOrCertificate) -> String {
    match key {
        PublicKeyOrCertificate::PublicKey { key, .. } => key.fingerprint(HashAlg::Sha256).to_string(),
        PublicKeyOrCertificate::Certificate(c) => c.public_key().fingerprint(HashAlg::Sha256).to_string(),
    }
}

impl client::Handler for Keys {
    type Error = russh::Error;
    async fn check_server_key(&mut self, key: &PublicKeyOrCertificate) -> Result<bool, Self::Error> {
        let got = print(key);
        let ok = self.want.is_empty() || self.want == got;
        if let Ok(mut seen) = self.seen.lock() {
            *seen = got;
        }
        Ok(ok)
    }
}

fn short(e: impl fmt::Display) -> String {
    e.to_string().chars().take(120).collect()
}

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

pub fn fetch(p: &Profile, w: &Want<'_>, limit: Duration) -> Result<Got, Problem> {
    let rt =
        tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|e| Problem::Failed(short(e)))?;
    rt.block_on(async { tokio::time::timeout(limit, run(p, w)).await.unwrap_or(Err(Problem::Timeout)) })
}

async fn login(p: &Profile, w: &Want<'_>, seen: Arc<Mutex<String>>) -> Result<client::Handle<Keys>, Problem> {
    let config = Arc::new(client::Config { inactivity_timeout: Some(IDLE), ..client::Config::default() });
    let keys = Keys { want: w.host_key.to_string(), seen: Arc::clone(&seen) };
    let mut s = match client::connect(config, (p.host.as_str(), p.port), keys).await {
        Ok(s) => s,
        Err(_) if !w.host_key.is_empty() && seen.lock().is_ok_and(|k| !k.is_empty() && *k != w.host_key) => {
            return Err(Problem::HostKeyChanged);
        }
        Err(e) => return Err(Problem::Unreachable(short(e))),
    };
    let result = match &p.auth {
        Auth::Pass(pass) => s.authenticate_password(&p.user, pass).await,
        Auth::Key(path) => {
            let key = load_secret_key(path, None)
                .map_err(|e| Problem::Failed(format!("cannot load the key ({})", short(e))))?;
            let hash = s.best_supported_rsa_hash().await.ok().flatten().flatten();
            s.authenticate_publickey(&p.user, PrivateKeyWithHashAlg::new(Arc::new(key), hash)).await
        }
        Auth::None => return Err(Problem::Refused),
    };
    match result {
        Ok(r) if r.success() => Ok(s),
        Ok(_) => Err(Problem::Refused),
        Err(e) => Err(Problem::Unreachable(short(e))),
    }
}

async fn run(p: &Profile, w: &Want<'_>) -> Result<Got, Problem> {
    let seen = Arc::new(Mutex::new(String::new()));
    let session = login(p, w, Arc::clone(&seen)).await?;
    let channel = session.channel_open_session().await.map_err(|e| Problem::Failed(short(e)))?;
    channel.request_subsystem(true, "sftp").await.map_err(|e| Problem::Failed(short(e)))?;
    let sftp = SftpSession::new(channel.into_stream())
        .await
        .map_err(|e| Problem::Failed(format!("no sftp ({})", short(e))))?;
    let mut got = read_log(&sftp, w).await?;
    if let Some(dir) = w.crash_dir {
        crashes(&sftp, dir, w.crashes_after, &mut got).await;
    }
    got.host_key = seen.lock().map(|k| k.clone()).unwrap_or_default();
    let _ = sftp.close().await;
    Ok(got)
}

async fn read_range(sftp: &SftpSession, path: &str, from: u64, len: u64) -> Result<Vec<u8>, Problem> {
    let mut f = sftp.open(path).await.map_err(|e| Problem::Failed(short(e)))?;
    f.seek(SeekFrom::Start(from)).await.map_err(|e| Problem::Failed(short(e)))?;
    let mut buf = Vec::new();
    f.take(len).read_to_end(&mut buf).await.map_err(|e| Problem::Failed(short(e)))?;
    Ok(buf)
}

pub(crate) fn first_line(bytes: &[u8]) -> String {
    match bytes.iter().position(|b| *b == b'\n') {
        Some(end) => String::from_utf8_lossy(&bytes[..end]).trim_end().to_string(),
        None => String::new(),
    }
}

pub(crate) fn plan(size: u64, offset: u64, rotated: bool, fresh: bool) -> u64 {
    let from = if fresh {
        size.saturating_sub(FIRST_TAIL)
    } else if rotated {
        0
    } else {
        offset
    };
    from.max(size.saturating_sub(MAX_READ))
}

pub(crate) fn whole_lines(buf: &[u8], skipped: bool) -> (usize, usize) {
    let start = if skipped { buf.iter().position(|b| *b == b'\n').map_or(buf.len(), |i| i + 1) } else { 0 };
    let end = buf.iter().rposition(|b| *b == b'\n').map_or(start, |i| i + 1).max(start);
    (start, end)
}

async fn read_log(sftp: &SftpSession, w: &Want<'_>) -> Result<Got, Problem> {
    let meta = sftp.metadata(w.log).await.map_err(|_| Problem::NoLog(w.log.to_string()))?;
    let size = meta.size.unwrap_or(0);
    let head = first_line(&read_range(sftp, w.log, 0, HEAD_BYTES).await?);
    let fresh = w.head.is_empty() && w.offset == 0;
    let rotated = !fresh && (size < w.offset || (!w.head.is_empty() && !head.is_empty() && head != w.head));
    let from = plan(size, w.offset, rotated, fresh);
    let buf = read_range(sftp, w.log, from, size - from).await?;
    let skipped = from > 0 && from != w.offset;
    let (start, end) = whole_lines(&buf, skipped);
    Ok(Got {
        offset: from + end as u64,
        head,
        text: String::from_utf8_lossy(&buf[start..end]).into_owned(),
        rotated,
        ..Got::default()
    })
}

async fn crashes(sftp: &SftpSession, dir: &str, after: &str, got: &mut Got) {
    got.newest_crash = after.to_string();
    let Ok(list) = sftp.read_dir(dir).await else { return };
    let fresh_from = now().saturating_sub(CRASH_FRESH_SECS);
    let mut names: Vec<String> = list
        .filter(|e| e.file_name().ends_with(".txt"))
        .filter(|e| {
            let name = e.file_name();
            if after.is_empty() {
                e.metadata().mtime.is_some_and(|t| u64::from(t) >= fresh_from)
            } else {
                *name > *after
            }
        })
        .map(|e| e.file_name())
        .collect();
    names.sort();
    if let Some(last) = names.last() {
        got.newest_crash.clone_from(last);
    }
    let from = names.len().saturating_sub(CRASHES_PER_CHECK);
    for name in &names[from..] {
        let path = format!("{dir}/{name}");
        if let Ok(buf) = read_range(sftp, &path, 0, CRASH_BYTES).await {
            got.crashes.push(Crash { name: name.clone(), text: String::from_utf8_lossy(&buf).into_owned() });
        }
    }
}
