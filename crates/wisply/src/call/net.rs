use std::sync::LazyLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use serde_json::{Map, Value, json};
use ureq::tls::{RootCerts, TlsConfig};
use wisply_core::Fail;

use super::cfg;

const DEFAULT_SERVER: &str = "https://ntfy.sh";
const BODY_MAX: u64 = 1_000_000;

static TIMEOUT: AtomicU64 = AtomicU64::new(15);

static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::Agent::config_builder()
        .http_status_as_error(false)
        .tls_config(TlsConfig::builder().root_certs(RootCerts::PlatformVerifier).build())
        .build()
        .into()
});

pub struct Reply {
    pub status: u16,
    pub body: String,
}

impl Reply {
    pub fn good(&self) -> bool {
        (200..300).contains(&self.status)
    }
}

pub fn tune(cfg: &Map<String, Value>) {
    TIMEOUT.store(cfg::num(cfg, "http_timeout_s").max(1), Ordering::Relaxed);
}

fn wait() -> Duration {
    Duration::from_secs(TIMEOUT.load(Ordering::Relaxed))
}

fn read(resp: Result<ureq::http::Response<ureq::Body>, ureq::Error>) -> Result<Reply, Fail> {
    let mut resp = resp.map_err(|e| Fail::new(format!("no connection ({e})")))?;
    let status = resp.status().as_u16();
    let body = resp.body_mut().with_config().limit(BODY_MAX).read_to_string().unwrap_or_default();
    Ok(Reply { status, body })
}

pub fn get(url: &str) -> Result<Reply, Fail> {
    read(AGENT.get(url).config().timeout_global(Some(wait())).build().call())
}

pub fn post_json(url: &str, body: &Value) -> Result<Reply, Fail> {
    read(
        AGENT
            .post(url)
            .config()
            .timeout_global(Some(wait()))
            .build()
            .header("Content-Type", "application/json")
            .send(body.to_string()),
    )
}

pub fn server(cfg: &Map<String, Value>, home_cfg: &Map<String, Value>) -> String {
    let own = cfg::text(cfg, "ntfy_server");
    let pick = if own.trim().is_empty() { cfg::text(home_cfg, "push_server") } else { own };
    let pick = pick.trim().trim_end_matches('/').to_string();
    if pick.is_empty() { DEFAULT_SERVER.to_string() } else { pick }
}

pub fn topic(cfg: &Map<String, Value>, home_cfg: &Map<String, Value>) -> String {
    let own = cfg::text(cfg, "ntfy_topic");
    let pick = if own.trim().is_empty() { cfg::text(home_cfg, "push_topic") } else { own };
    pick.trim().to_string()
}

pub fn reply_topic(cfg: &Map<String, Value>, topic: &str) -> String {
    let own = cfg::text(cfg, "ntfy_reply_topic");
    if own.trim().is_empty() { format!("{topic}-re") } else { own.trim().to_string() }
}

pub fn publish(server: &str, message: &Value) -> Result<(), Fail> {
    let reply = post_json(server, message)?;
    if reply.good() {
        return Ok(());
    }
    Err(Fail::new(format!("ntfy answered {}", reply.status)))
}

pub fn poll(server: &str, topic: &str, since: u64) -> Result<Vec<(String, u64, String)>, Fail> {
    let reply = get(&format!("{server}/{topic}/json?poll=1&since={since}"))?;
    if !reply.good() {
        return Err(Fail::new(format!("ntfy answered {}", reply.status)));
    }
    Ok(reply
        .body
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .filter(|v| v["event"] == "message")
        .map(|v| {
            let text = |k: &str| v.get(k).and_then(Value::as_str).unwrap_or("").to_string();
            (text("id"), v.get("time").and_then(Value::as_u64).unwrap_or(0), text("message"))
        })
        .collect())
}

pub fn button(label: &str, url: &str, body: &str) -> Value {
    json!({"action": "http", "label": label, "url": url, "method": "POST", "body": body, "clear": true})
}

#[cfg(test)]
pub mod mock {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::{Arc, Mutex};
    use std::thread;

    pub struct Server {
        pub url: String,
        pub seen: Arc<Mutex<Vec<(String, String, String)>>>,
        pub replies: Arc<Mutex<Vec<String>>>,
    }

    pub fn start() -> Server {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let seen = Arc::new(Mutex::new(Vec::new()));
        let replies = Arc::new(Mutex::new(Vec::new()));
        let (log, queue) = (seen.clone(), replies.clone());
        thread::spawn(move || {
            for stream in listener.incoming().flatten() {
                let (log, queue) = (log.clone(), queue.clone());
                thread::spawn(move || handle(stream, &log, &queue));
            }
        });
        Server { url, seen, replies }
    }

    fn handle(mut stream: std::net::TcpStream, log: &Mutex<Vec<(String, String, String)>>, queue: &Mutex<Vec<String>>) {
        let mut buf = Vec::new();
        let mut chunk = [0u8; 4096];
        let split = loop {
            let n = stream.read(&mut chunk).unwrap_or(0);
            if n == 0 {
                return;
            }
            buf.extend_from_slice(&chunk[..n]);
            if let Some(i) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
                break i + 4;
            }
        };
        let head = String::from_utf8_lossy(&buf[..split]).to_string();
        let len: usize = head
            .lines()
            .find_map(|l| l.to_lowercase().strip_prefix("content-length:").map(|v| v.trim().parse().unwrap_or(0)))
            .unwrap_or(0);
        while buf.len() < split + len {
            let n = stream.read(&mut chunk).unwrap_or(0);
            if n == 0 {
                break;
            }
            buf.extend_from_slice(&chunk[..n]);
        }
        let body = String::from_utf8_lossy(&buf[split..]).to_string();
        let mut first = head.lines().next().unwrap_or("").split_whitespace();
        let (method, path) = (first.next().unwrap_or("").to_string(), first.next().unwrap_or("").to_string());
        log.lock().unwrap().push((method.clone(), path.clone(), body));
        let payload = if method == "GET" && path.contains("/json?poll=1") {
            std::mem::take(&mut *queue.lock().unwrap()).join("\n")
        } else {
            "{}".to_string()
        };
        let _ = write!(
            stream,
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n{payload}",
            payload.len()
        );
    }
}
