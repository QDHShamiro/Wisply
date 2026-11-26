use std::io::{BufRead, BufReader, Read, Write};
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU16, Ordering};
use std::thread;
use std::time::Duration;

use super::super::config;
use super::providers::DEFAULT_PORT;
use super::serve;

const HEAD_MAX: usize = 32 * 1024;
const BODY_MAX: usize = 64 * 1024 * 1024;
const READ_WAIT: Duration = Duration::from_secs(30);
const WRITE_WAIT: Duration = Duration::from_secs(60);

static BOUND: AtomicU16 = AtomicU16::new(0);

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

pub fn port() -> u16 {
    BOUND.load(Ordering::Relaxed)
}

pub fn configured_port(home: &Path) -> u16 {
    config::load(home)
        .get("fallback_port")
        .and_then(|v| v.as_u64())
        .filter(|p| *p > 0 && *p < 65_536)
        .unwrap_or(DEFAULT_PORT) as u16
}

pub fn ensure(home: &Path) -> Result<u16, String> {
    if port() != 0 {
        return Ok(port());
    }
    let want = configured_port(home);
    let listener =
        TcpListener::bind(SocketAddr::from((Ipv4Addr::LOCALHOST, want))).map_err(|e| format!("port {want}: {e}"))?;
    BOUND.store(want, Ordering::Relaxed);
    let home: PathBuf = home.to_path_buf();
    thread::Builder::new()
        .name("relay".into())
        .spawn(move || {
            for conn in listener.incoming().flatten() {
                let home = home.clone();
                let _ = thread::Builder::new().name("relay-conn".into()).spawn(move || connection(&home, conn));
            }
        })
        .map_err(|e| e.to_string())?;
    Ok(want)
}

fn connection(home: &Path, mut conn: TcpStream) {
    let _ = conn.set_read_timeout(Some(READ_WAIT));
    let _ = conn.set_write_timeout(Some(WRITE_WAIT));
    let _ = conn.set_nodelay(true);
    match read_request(&conn) {
        Ok(req) => serve::handle(home, &req, &mut conn),
        Err(why) => {
            let _ = serve::plain_error(&mut conn, 400, "invalid_request_error", &why);
        }
    }
    let _ = conn.flush();
    let _ = conn.shutdown(std::net::Shutdown::Both);
}

type Headers = Vec<(String, String)>;
type Head = (String, String, Headers);

pub fn parse_head(lines: &[String]) -> Result<Head, String> {
    let first = lines.first().ok_or("empty request")?;
    let mut parts = first.split_whitespace();
    let method = parts.next().ok_or("no method")?.to_uppercase();
    let path = parts.next().ok_or("no path")?.to_string();
    let headers = lines[1..]
        .iter()
        .filter_map(|l| l.split_once(':').map(|(k, v)| (k.trim().to_string(), v.trim().to_string())))
        .collect();
    Ok((method, path, headers))
}

fn header<'a>(headers: &'a [(String, String)], name: &str) -> Option<&'a str> {
    headers.iter().find(|(k, _)| k.eq_ignore_ascii_case(name)).map(|(_, v)| v.as_str())
}

pub(super) fn read_request(conn: &TcpStream) -> Result<Request, String> {
    let mut reader = BufReader::new(conn);
    let mut lines = Vec::new();
    let mut total = 0;
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line).map_err(|e| e.to_string())?;
        total += n;
        if n == 0 || total > HEAD_MAX {
            return Err("request head is missing or too large".into());
        }
        let line = line.trim_end().to_string();
        if line.is_empty() {
            break;
        }
        lines.push(line);
    }
    let (method, path, headers) = parse_head(&lines)?;
    let body = if header(&headers, "transfer-encoding").is_some_and(|v| v.to_lowercase().contains("chunked")) {
        chunked(&mut reader)?
    } else {
        let len: usize = header(&headers, "content-length").and_then(|v| v.parse().ok()).unwrap_or(0);
        if len > BODY_MAX {
            return Err("body too large".into());
        }
        let mut body = vec![0u8; len];
        reader.read_exact(&mut body).map_err(|e| e.to_string())?;
        body
    };
    Ok(Request { method, path, headers, body })
}

fn chunked(reader: &mut impl BufRead) -> Result<Vec<u8>, String> {
    let mut body = Vec::new();
    loop {
        let mut size = String::new();
        reader.read_line(&mut size).map_err(|e| e.to_string())?;
        let n =
            usize::from_str_radix(size.trim().split(';').next().unwrap_or("0"), 16).map_err(|_| "bad chunk size")?;
        if n == 0 {
            let mut end = String::new();
            let _ = reader.read_line(&mut end);
            return Ok(body);
        }
        if body.len() + n > BODY_MAX {
            return Err("body too large".into());
        }
        let start = body.len();
        body.resize(start + n, 0);
        reader.read_exact(&mut body[start..]).map_err(|e| e.to_string())?;
        let mut crlf = String::new();
        reader.read_line(&mut crlf).map_err(|e| e.to_string())?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn the_request_line_and_headers_are_split() {
        let lines: Vec<String> = ["POST /v1/messages?beta=true HTTP/1.1", "Content-Length: 5", "X-Api-Key:  abc "]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let (m, p, h) = parse_head(&lines).unwrap();
        assert_eq!((m.as_str(), p.as_str()), ("POST", "/v1/messages?beta=true"));
        assert_eq!(header(&h, "x-api-key"), Some("abc"));
        assert!(parse_head(&[]).is_err());
    }

    #[test]
    fn a_chunked_body_is_joined_and_a_bad_size_is_refused() {
        let mut ok = Cursor::new(b"5\r\nhello\r\n6\r\n world\r\n0\r\n\r\n".to_vec());
        assert_eq!(chunked(&mut ok).unwrap(), b"hello world");
        let mut bad = Cursor::new(b"zz\r\n".to_vec());
        assert!(chunked(&mut bad).is_err());
    }
}
