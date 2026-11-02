use std::io::{ErrorKind, Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use super::api::{Api, BODY_MAX, Reply};
use super::events;
use super::web;

const HEAD_MAX: usize = 8 * 1024;
const READ_WAIT: Duration = Duration::from_secs(5);
const WRITE_WAIT: Duration = Duration::from_secs(10);
const PING_EVERY: Duration = Duration::from_secs(15);
const ACCEPT_NAP: Duration = Duration::from_millis(30);
const CONNS_MAX: usize = 64;
const STREAMS_MAX: usize = 8;
const TAILSCALE: &[&str] = &["tailscale", "C:\\Program Files\\Tailscale\\tailscale.exe", "/usr/local/bin/tailscale"];
#[cfg(windows)]
const NO_WINDOW: u32 = 0x0800_0000;

pub fn allowed_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_loopback() || (v4.octets()[0] == 100 && (64..128).contains(&v4.octets()[1])),
        IpAddr::V6(v6) => {
            let s = v6.segments();
            v6.is_loopback() || (s[0] == 0xfd7a && s[1] == 0x115c && s[2] == 0xa1e0)
        }
    }
}

pub fn parse_binds(list: &str) -> (Vec<IpAddr>, Vec<String>) {
    let (mut ok, mut refused) = (vec![IpAddr::V4(Ipv4Addr::LOCALHOST)], Vec::new());
    for word in list.split([',', ' ', ';']).map(str::trim).filter(|w| !w.is_empty()) {
        match word.parse::<IpAddr>() {
            Ok(ip) if allowed_ip(ip) => {
                if !ok.contains(&ip) {
                    ok.push(ip);
                }
            }
            _ => refused.push(word.to_string()),
        }
    }
    (ok, refused)
}

pub fn tailscale_ips() -> Vec<IpAddr> {
    let mut found = Vec::new();
    for exe in TAILSCALE {
        let mut cmd = Command::new(exe);
        cmd.args(["ip"]).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null());
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(NO_WINDOW);
        }
        let Ok(out) = cmd.output() else { continue };
        found = String::from_utf8_lossy(&out.stdout)
            .lines()
            .filter_map(|l| l.trim().parse::<IpAddr>().ok())
            .filter(|ip| allowed_ip(*ip) && !ip.is_loopback())
            .collect();
        break;
    }
    found
}

pub struct Server {
    pub addrs: Vec<SocketAddr>,
    stop: Arc<AtomicBool>,
}

impl Server {
    pub fn stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}

pub fn serve(api: Arc<Api>, ips: &[IpAddr], port: u16) -> Result<Server, String> {
    let stop = Arc::new(AtomicBool::new(false));
    let live = Arc::new(AtomicUsize::new(0));
    let streams = Arc::new(AtomicUsize::new(0));
    let mut addrs = Vec::new();
    let mut last_err = String::new();
    for ip in ips.iter().copied().filter(|ip| allowed_ip(*ip)) {
        let listener = match TcpListener::bind(SocketAddr::new(ip, port)) {
            Ok(l) => l,
            Err(e) => {
                last_err = format!("{ip}:{port}: {e}");
                continue;
            }
        };
        let _ = listener.set_nonblocking(true);
        addrs.push(listener.local_addr().map_err(|e| e.to_string())?);
        let (api, stop, live, streams) = (Arc::clone(&api), Arc::clone(&stop), Arc::clone(&live), Arc::clone(&streams));
        let _ = thread::Builder::new()
            .name("phone-api".into())
            .spawn(move || accept(&listener, &api, &stop, &live, &streams));
    }
    if addrs.is_empty() {
        return Err(if last_err.is_empty() { "no allowed address to listen on".into() } else { last_err });
    }
    Ok(Server { addrs, stop })
}

fn accept(
    listener: &TcpListener,
    api: &Arc<Api>,
    stop: &AtomicBool,
    live: &Arc<AtomicUsize>,
    streams: &Arc<AtomicUsize>,
) {
    while !stop.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((conn, peer)) => {
                if !allowed_ip(peer.ip()) || live.load(Ordering::SeqCst) >= CONNS_MAX {
                    continue;
                }
                live.fetch_add(1, Ordering::SeqCst);
                let (api, mine, streams) = (Arc::clone(api), Arc::clone(live), Arc::clone(streams));
                let spawned = thread::Builder::new().name("phone-conn".into()).spawn(move || {
                    let _ = conn.set_nonblocking(false);
                    one(&api, conn, peer, &streams);
                    mine.fetch_sub(1, Ordering::SeqCst);
                });
                if spawned.is_err() {
                    live.fetch_sub(1, Ordering::SeqCst);
                }
            }
            Err(e) if e.kind() == ErrorKind::WouldBlock => thread::sleep(ACCEPT_NAP),
            Err(_) => thread::sleep(ACCEPT_NAP),
        }
    }
}

struct Request {
    method: String,
    target: String,
    auth: Option<String>,
    last_event_id: Option<String>,
    body: Vec<u8>,
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

fn read_request(conn: &mut TcpStream) -> Result<Request, u16> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 2048];
    let end = loop {
        if let Some(i) = find(&buf, b"\r\n\r\n") {
            break i;
        }
        if buf.len() > HEAD_MAX {
            return Err(431);
        }
        match conn.read(&mut chunk) {
            Ok(0) | Err(_) => return Err(400),
            Ok(n) => buf.extend_from_slice(&chunk[..n]),
        }
    };
    let head = String::from_utf8_lossy(&buf[..end]).to_string();
    let mut lines = head.split("\r\n");
    let mut first = lines.next().unwrap_or("").split(' ');
    let (method, target) = (first.next().unwrap_or(""), first.next().unwrap_or(""));
    if method.is_empty() || !target.starts_with('/') {
        return Err(400);
    }
    let (mut auth, mut last_event_id, mut length) = (None, None, 0usize);
    for line in lines {
        let Some((k, v)) = line.split_once(':') else { continue };
        let v = v.trim();
        match k.trim().to_ascii_lowercase().as_str() {
            "authorization" => auth = Some(v.to_string()),
            "last-event-id" => last_event_id = Some(v.to_string()),
            "content-length" => length = v.parse().map_err(|_| 400u16)?,
            "transfer-encoding" => return Err(400),
            _ => {}
        }
    }
    if length > BODY_MAX {
        return Err(413);
    }
    let mut body = buf[end + 4..].to_vec();
    while body.len() < length {
        match conn.read(&mut chunk) {
            Ok(0) | Err(_) => return Err(400),
            Ok(n) => body.extend_from_slice(&chunk[..n]),
        }
    }
    body.truncate(length);
    Ok(Request { method: method.to_string(), target: target.to_string(), auth, last_event_id, body })
}

fn reason(code: u16) -> &'static str {
    match code {
        200 => "OK",
        202 => "Accepted",
        400 => "Bad Request",
        401 => "Unauthorized",
        404 => "Not Found",
        409 => "Conflict",
        413 => "Payload Too Large",
        429 => "Too Many Requests",
        431 => "Request Header Fields Too Large",
        503 => "Service Unavailable",
        _ => "Error",
    }
}

fn send(conn: &mut TcpStream, code: u16, body: &str) {
    let retry = if code == 429 { "Retry-After: 1\r\n" } else { "" };
    let head = format!(
        "HTTP/1.1 {code} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\n{retry}Connection: close\r\n\r\n",
        reason(code),
        body.len()
    );
    let _ = conn.write_all(head.as_bytes());
    let _ = conn.write_all(body.as_bytes());
    let _ = conn.flush();
}

fn send_page(conn: &mut TcpStream, head_only: bool, kind: &str, body: &[u8]) {
    let head = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {kind}\r\nContent-Length: {}\r\nCache-Control: no-cache\r\nX-Content-Type-Options: nosniff\r\nX-Frame-Options: DENY\r\nReferrer-Policy: no-referrer\r\nContent-Security-Policy: {}\r\nConnection: close\r\n\r\n",
        body.len(),
        web::CSP
    );
    let _ = conn.write_all(head.as_bytes());
    if !head_only {
        let _ = conn.write_all(body);
    }
    let _ = conn.flush();
}

fn one(api: &Api, mut conn: TcpStream, peer: SocketAddr, streams: &AtomicUsize) {
    let _ = conn.set_read_timeout(Some(READ_WAIT));
    let _ = conn.set_write_timeout(Some(WRITE_WAIT));
    let req = match read_request(&mut conn) {
        Ok(r) => r,
        Err(code) => return send(&mut conn, code, "{\"error\":\"bad request\"}"),
    };
    if let Some((kind, body)) = web::page(&req.method, &req.target) {
        return send_page(&mut conn, req.method == "HEAD", kind, body);
    }
    let reply = api.handle(
        &req.method,
        &req.target,
        req.auth.as_deref(),
        req.last_event_id.as_deref(),
        &peer.ip().to_string(),
        &req.body,
    );
    match reply {
        Reply::Json(code, v) => send(&mut conn, code, &v.to_string()),
        Reply::Stream(since) => {
            if streams.fetch_add(1, Ordering::SeqCst) >= STREAMS_MAX {
                send(&mut conn, 503, "{\"error\":\"busy\"}");
            } else {
                stream(api, &mut conn, since);
            }
            streams.fetch_sub(1, Ordering::SeqCst);
        }
    }
}

fn stream(api: &Api, conn: &mut TcpStream, mut seq: u64) {
    let head = "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-store\r\nX-Content-Type-Options: nosniff\r\nConnection: close\r\n\r\n";
    if conn.write_all(head.as_bytes()).is_err() {
        return;
    }
    let _ = conn.set_read_timeout(Some(Duration::from_millis(1)));
    loop {
        let list = events::wait(&api.home, seq, PING_EVERY);
        let chunk = if list.is_empty() {
            ": ping\n\n".to_string()
        } else {
            list.iter().map(|e| format!("id: {}\ndata: {}\n\n", e.seq, e.json())).collect()
        };
        if let Some(last) = list.last() {
            seq = last.seq;
        }
        if conn.write_all(chunk.as_bytes()).and_then(|()| conn.flush()).is_err() {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;
    use std::sync::Arc;
    use std::time::Instant;

    use serde_json::json;

    use super::super::api::fake::Fake;
    use super::super::auth::Auth;
    use super::*;

    fn up(tag: &str) -> (Server, SocketAddr, String, std::path::PathBuf) {
        let home = std::env::temp_dir().join(format!("wisply-agents-net-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        let api = Api { home: home.clone(), auth: Auth::new(&home), backend: Box::new(Arc::new(Fake::default())) };
        let code = api.auth.new_code("handy", Instant::now()).unwrap();
        let server = serve(Arc::new(api), &[IpAddr::V4(Ipv4Addr::LOCALHOST)], 0).unwrap();
        let addr = server.addrs[0];
        let body = json!({"code": code, "device": "handy"}).to_string();
        let (status, text) = http(addr, "POST", "/v1/pair", &[], &body);
        assert_eq!(status, 200, "{text}");
        let token = serde_json::from_str::<serde_json::Value>(&text).unwrap()["token"].as_str().unwrap().to_string();
        (server, addr, format!("Bearer {token}"), home)
    }

    fn http(addr: SocketAddr, method: &str, path: &str, headers: &[(&str, &str)], body: &str) -> (u16, String) {
        let mut conn = TcpStream::connect(addr).unwrap();
        conn.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let extra: String = headers.iter().map(|(k, v)| format!("{k}: {v}\r\n")).collect();
        let req = format!("{method} {path} HTTP/1.1\r\nHost: x\r\n{extra}Content-Length: {}\r\n\r\n{body}", body.len());
        conn.write_all(req.as_bytes()).unwrap();
        let mut raw = String::new();
        let _ = conn.read_to_string(&mut raw);
        let status = raw.split(' ').nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
        (status, raw.split("\r\n\r\n").nth(1).unwrap_or("").to_string())
    }

    #[test]
    fn only_loopback_and_tailscale_addresses_are_allowed() {
        for ok in [
            "127.0.0.1",
            "127.5.5.5",
            "::1",
            "100.64.0.1",
            "100.100.1.2",
            "100.127.255.255",
            "fd7a:115c:a1e0::1",
            "fd7a:115c:a1e0:ab12::9",
        ] {
            assert!(allowed_ip(ok.parse().unwrap()), "{ok}");
        }
        for bad in [
            "0.0.0.0",
            "::",
            "192.168.1.5",
            "10.0.0.2",
            "172.16.0.1",
            "100.63.255.255",
            "100.128.0.1",
            "8.8.8.8",
            "fd7a:115c:a1e1::1",
            "fe80::1",
            "2001:db8::1",
        ] {
            assert!(!allowed_ip(bad.parse().unwrap()), "{bad}");
        }
    }

    #[test]
    fn the_bind_list_drops_every_foreign_address_and_never_listens_on_all() {
        let (ok, refused) = parse_binds("0.0.0.0, 192.168.1.5; 100.90.1.1 ::  fd7a:115c:a1e0::5 nonsense 127.0.0.1");
        assert_eq!(
            ok,
            [
                "127.0.0.1".parse::<IpAddr>().unwrap(),
                "100.90.1.1".parse().unwrap(),
                "fd7a:115c:a1e0::5".parse().unwrap()
            ]
        );
        assert_eq!(refused, ["0.0.0.0", "192.168.1.5", "::", "nonsense"]);
        assert_eq!(parse_binds("").0.len(), 1);
        let home = std::env::temp_dir().join("wisply-agents-net-bind");
        let api =
            Arc::new(Api { home: home.clone(), auth: Auth::new(&home), backend: Box::new(Arc::new(Fake::default())) });
        let any: IpAddr = "0.0.0.0".parse().unwrap();
        let lan: IpAddr = "192.168.1.5".parse().unwrap();
        assert!(serve(Arc::clone(&api), &[any, lan], 0).is_err());
        let s = serve(api, &[any, "127.0.0.1".parse().unwrap()], 0).unwrap();
        assert!(s.addrs.iter().all(|a| a.ip().is_loopback()));
        s.stop();
    }

    #[test]
    fn over_the_wire_no_token_is_401_and_a_good_token_reads_the_board() {
        let (server, addr, auth, home) = up("wire");
        for path in ["/v1/agents", "/v1/status", "/v1/events", "/v1/other", "/v1/agents/aa11"] {
            assert_eq!(http(addr, "GET", path, &[], "").0, 401, "{path}");
            assert_eq!(
                http(addr, "GET", path, &[("Authorization", "Bearer 00"), ("X-Forwarded-For", "127.0.0.1")], "").0,
                401
            );
        }
        let (code, body) = http(addr, "GET", "/", &[], "");
        assert_eq!(code, 200);
        assert!(body.contains("phone.js") && !body.contains("aa11"));
        assert_eq!(http(addr, "POST", "/", &[], "{}").0, 401);
        assert_eq!(http(addr, "GET", "/phone.js", &[], "").0, 200);
        let (code, body) = http(addr, "GET", "/v1/agents", &[("Authorization", &auth)], "");
        assert_eq!(code, 200);
        assert!(body.contains("\"aa11\"") && !body.contains("private"), "{body}");
        assert_eq!(http(addr, "POST", "/v1/pair", &[], "{\"code\":\"x\",\"device\":\"y\"}").0, 401);
        assert_eq!(http(addr, "POST", "/v1/pair", &[], &"x".repeat(BODY_MAX + 1)).0, 413);
        assert_eq!(http(addr, "GET", "/nothing", &[("Authorization", &auth)], "").0, 404);
        server.stop();
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn over_the_wire_the_twenty_first_request_in_a_second_is_429() {
        let (server, addr, auth, home) = up("rate");
        let jobs: Vec<_> = (0..30)
            .map(|_| {
                let auth = auth.clone();
                thread::spawn(move || http(addr, "GET", "/v1/status", &[("Authorization", &auth)], "").0)
            })
            .collect();
        let codes: Vec<u16> = jobs.into_iter().map(|j| j.join().unwrap()).collect();
        let ok = codes.iter().filter(|c| **c == 200).count();
        assert!((20..=25).contains(&ok) && codes.contains(&429), "{codes:?}");
        server.stop();
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn the_event_stream_delivers_new_events() {
        let (server, addr, auth, home) = up("sse");
        let mut conn = TcpStream::connect(addr).unwrap();
        conn.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let req = format!("GET /v1/events?since={} HTTP/1.1\r\nAuthorization: {auth}\r\n\r\n", events::last_seq(&home));
        conn.write_all(req.as_bytes()).unwrap();
        thread::sleep(Duration::from_millis(200));
        let sent = events::emit(&home, "done", Some("aa11"), "build ok", "all green", false);
        let mut got = String::new();
        let mut buf = [0u8; 1024];
        while !got.contains("all green") {
            let n = conn.read(&mut buf).unwrap();
            assert!(n > 0, "stream closed: {got}");
            got.push_str(&String::from_utf8_lossy(&buf[..n]));
        }
        assert!(got.contains("text/event-stream") && got.contains(&format!("id: {}", sent.seq)), "{got}");
        server.stop();
        let _ = std::fs::remove_dir_all(&home);
    }
}
