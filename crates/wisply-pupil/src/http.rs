use std::io::{BufRead, Read, Write};

pub const OK: u16 = 200;
pub const BAD: u16 = 400;
pub const LOCKED: u16 = 401;
pub const FORBIDDEN: u16 = 403;
pub const MISSING: u16 = 404;
pub const METHOD: u16 = 405;
pub const BUSY: u16 = 409;
pub const LENGTH: u16 = 411;
pub const LARGE: u16 = 413;
pub const KIND: u16 = 415;

pub const HTML: &str = "text/html; charset=utf-8";
pub const SCRIPT: &str = "text/javascript; charset=utf-8";
pub const JSON: &str = "application/json; charset=utf-8";
pub const LINES: &str = "application/x-ndjson; charset=utf-8";

const VERSION: &str = "HTTP/1.";
const COMMON: [(&str, &str); 4] = [
    ("Connection", "close"),
    ("Cache-Control", "no-store"),
    ("X-Content-Type-Options", "nosniff"),
    ("Referrer-Policy", "no-referrer"),
];

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub headers: Vec<(String, String)>,
    pub length: Option<u64>,
}

fn find<'a>(pairs: &'a [(String, String)], name: &str) -> Option<&'a str> {
    pairs.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
}

impl Request {
    pub fn header(&self, name: &str) -> Option<&str> {
        find(&self.headers, name)
    }

    pub fn param(&self, name: &str) -> Option<&str> {
        find(&self.query, name)
    }
}

fn reason(status: u16) -> &'static str {
    match status {
        OK => "OK",
        BAD => "Bad Request",
        LOCKED => "Unauthorized",
        FORBIDDEN => "Forbidden",
        MISSING => "Not Found",
        METHOD => "Method Not Allowed",
        BUSY => "Conflict",
        LENGTH => "Length Required",
        LARGE => "Content Too Large",
        KIND => "Unsupported Media Type",
        _ => "Error",
    }
}

pub fn decode(raw: &str) -> String {
    let bytes = raw.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut at = 0;
    while at < bytes.len() {
        let pair = bytes.get(at + 1..at + 3).and_then(|hex| {
            let text = std::str::from_utf8(hex).ok()?;
            u8::from_str_radix(text, 16).ok()
        });
        match (bytes[at], pair) {
            (b'%', Some(byte)) => {
                out.push(byte);
                at += 3;
            }
            (byte, _) => {
                out.push(byte);
                at += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

pub fn parse_head(head: &str) -> Option<Request> {
    let mut lines = head.lines();
    let mut first = lines.next()?.split(' ');
    let (method, target, version) = (first.next()?, first.next()?, first.next()?);
    if first.next().is_some() || !version.starts_with(VERSION) || !target.starts_with('/') {
        return None;
    }
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    let query = query
        .split('&')
        .filter(|pair| !pair.is_empty())
        .map(|pair| {
            let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
            (decode(name), decode(value))
        })
        .collect();
    let mut headers = Vec::new();
    for line in lines {
        let (name, value) = line.split_once(':')?;
        headers.push((name.trim().to_ascii_lowercase(), value.trim().to_string()));
    }
    if find(&headers, "transfer-encoding").is_some() {
        return None;
    }
    let length = match find(&headers, "content-length") {
        Some(raw) => Some(raw.parse::<u64>().ok()?),
        None => None,
    };
    Some(Request { method: method.to_string(), path: decode(path), query, headers, length })
}

pub fn read_head(reader: &mut impl BufRead, max: u64) -> Option<String> {
    let mut limited = reader.by_ref().take(max);
    let mut head = String::new();
    loop {
        let mut line = String::new();
        if limited.read_line(&mut line).ok()? == 0 {
            return None;
        }
        if line.trim_end_matches(['\r', '\n']).is_empty() {
            return (!head.is_empty()).then_some(head);
        }
        head.push_str(&line);
    }
}

fn head(status: u16, kind: &str, length: Option<usize>, extra: &[(&str, &str)]) -> String {
    let mut head = format!("HTTP/1.1 {status} {}\r\nContent-Type: {kind}\r\n", reason(status));
    if let Some(length) = length {
        head.push_str(&format!("Content-Length: {length}\r\n"));
    }
    for (name, value) in COMMON.iter().chain(extra) {
        head.push_str(&format!("{name}: {value}\r\n"));
    }
    head.push_str("\r\n");
    head
}

pub fn respond(
    wire: &mut impl Write,
    status: u16,
    kind: &str,
    extra: &[(&str, &str)],
    body: &[u8],
) -> std::io::Result<()> {
    wire.write_all(head(status, kind, Some(body.len()), extra).as_bytes())?;
    wire.write_all(body)?;
    wire.flush()
}

pub fn begin(wire: &mut impl Write, status: u16, kind: &str) -> std::io::Result<()> {
    wire.write_all(head(status, kind, None, &[]).as_bytes())?;
    wire.flush()
}

#[cfg(test)]
mod tests {
    use super::{BUSY, JSON, OK, Request, begin, decode, parse_head, read_head, respond};
    use std::io::BufReader;

    #[test]
    fn a_request_head_is_read_into_its_parts() {
        let raw = "POST /file?name=Mein%20Clip%20%231.mp4&chat=c1 HTTP/1.1\r\nHost: 127.0.0.1:9953\r\nX-Pupil-Key:  abc \r\nContent-Length: 12\r\n\r\nbody bytes..";
        let mut reader = BufReader::new(raw.as_bytes());
        let head = read_head(&mut reader, 4096).expect("head");
        let request = parse_head(&head).expect("request");
        assert_eq!(request.method, "POST");
        assert_eq!(request.path, "/file");
        assert_eq!(request.param("name"), Some("Mein Clip #1.mp4"));
        assert_eq!(request.param("chat"), Some("c1"));
        assert_eq!(request.param("other"), None);
        assert_eq!(request.header("host"), Some("127.0.0.1:9953"));
        assert_eq!(request.header("x-pupil-key"), Some("abc"));
        assert_eq!(request.length, Some(12));
        let mut rest = String::new();
        std::io::Read::read_to_string(&mut reader, &mut rest).expect("body");
        assert_eq!(rest, "body bytes..");
    }

    #[test]
    fn broken_heads_are_refused() {
        let parse = |raw: &str| parse_head(raw);
        assert_eq!(parse(""), None);
        assert_eq!(parse("GET /"), None);
        assert_eq!(parse("GET http://evil/ HTTP/1.1"), None);
        assert_eq!(parse("GET / HTTP/1.1 extra"), None);
        assert_eq!(parse("GET / SPDY/3"), None);
        assert_eq!(parse("GET / HTTP/1.1\r\nno colon here"), None);
        assert_eq!(parse("POST /say HTTP/1.1\r\nContent-Length: many"), None);
        assert_eq!(parse("POST /say HTTP/1.1\r\nTransfer-Encoding: chunked"), None);
        assert_eq!(
            parse("GET / HTTP/1.1"),
            Some(Request { method: "GET".to_string(), path: "/".to_string(), ..Request::default() })
        );
        let long = format!("GET / HTTP/1.1\r\nX-Long: {}\r\n\r\n", "a".repeat(500));
        assert_eq!(read_head(&mut BufReader::new(long.as_bytes()), 64), None);
        assert_eq!(read_head(&mut BufReader::new("\r\n".as_bytes()), 64), None);
        assert_eq!(read_head(&mut BufReader::new("GET / HTTP/1.1\r\n".as_bytes()), 64), None);
    }

    #[test]
    fn percent_signs_are_decoded_and_bad_ones_stay() {
        assert_eq!(decode("a%20b"), "a b");
        assert_eq!(decode("%C3%9Cbung.mp4"), "Übung.mp4");
        assert_eq!(decode("100%"), "100%");
        assert_eq!(decode("%zz+x"), "%zz+x");
        assert_eq!(decode("%2"), "%2");
        assert_eq!(decode(""), "");
    }

    #[test]
    fn answers_carry_the_safety_headers() {
        let mut wire = Vec::new();
        respond(&mut wire, OK, JSON, &[("X-Extra", "1")], b"{}").expect("write");
        let text = String::from_utf8(wire).expect("utf8");
        assert!(text.starts_with("HTTP/1.1 200 OK\r\n"));
        assert!(text.contains("Content-Length: 2\r\n"));
        assert!(text.contains("Connection: close\r\n"));
        assert!(text.contains("Cache-Control: no-store\r\n"));
        assert!(text.contains("X-Content-Type-Options: nosniff\r\n"));
        assert!(text.contains("X-Extra: 1\r\n"));
        assert!(!text.contains("Access-Control"));
        assert!(text.ends_with("\r\n\r\n{}"));
        let mut wire = Vec::new();
        begin(&mut wire, BUSY, JSON).expect("write");
        let text = String::from_utf8(wire).expect("utf8");
        assert!(text.starts_with("HTTP/1.1 409 Conflict\r\n"));
        assert!(!text.contains("Content-Length"));
        assert!(text.ends_with("\r\n\r\n"));
    }
}
