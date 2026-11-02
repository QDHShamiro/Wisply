use std::sync::LazyLock;

use regex::Regex;

const POOL: u16 = 0x0001;
const START: u16 = 0x0102;
const END: u16 = 0x0103;
const UTF8: u32 = 0x100;
const TYPE_STRING: u8 = 0x03;
const TYPE_BOOL: u8 = 0x12;
const TYPE_DEC: u8 = 0x10;
const MAX_STRINGS: usize = 100_000;

static TAG: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<(/?)([\w.\-]+)([^>]*?)(/?)>").expect("tag"));
static ATTR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"([\w:.\-]+)\s*=\s*"([^"]*)""#).expect("attr"));

#[derive(Debug, PartialEq)]
pub enum Event {
    Start(String, Vec<(String, String)>),
    End(String),
}

fn u16_at(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

fn u32_at(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn len8(b: &[u8], p: usize) -> Option<(usize, usize)> {
    let first = usize::from(*b.get(p)?);
    if first & 0x80 != 0 {
        Some((((first & 0x7f) << 8) | usize::from(*b.get(p + 1)?), p + 2))
    } else {
        Some((first, p + 1))
    }
}

fn len16(b: &[u8], p: usize) -> Option<(usize, usize)> {
    let first = usize::from(u16_at(b, p)?);
    if first & 0x8000 != 0 {
        Some((((first & 0x7fff) << 16) | usize::from(u16_at(b, p + 2)?), p + 4))
    } else {
        Some((first, p + 2))
    }
}

fn pool(b: &[u8], at: usize) -> Option<Vec<String>> {
    let count = u32_at(b, at + 8)? as usize;
    let flags = u32_at(b, at + 16)?;
    let start = at + u32_at(b, at + 20)? as usize;
    let head = at + usize::from(u16_at(b, at + 2)?);
    (0..count.min(MAX_STRINGS))
        .map(|i| {
            let p = start + u32_at(b, head + i * 4)? as usize;
            if flags & UTF8 != 0 {
                let (_, p) = len8(b, p)?;
                let (len, p) = len8(b, p)?;
                Some(String::from_utf8_lossy(b.get(p..p + len)?).into_owned())
            } else {
                let (len, p) = len16(b, p)?;
                let units: Vec<u16> = (0..len).map(|k| u16_at(b, p + k * 2)).collect::<Option<_>>()?;
                Some(String::from_utf16_lossy(&units))
            }
        })
        .collect()
}

fn text(strings: &[String], i: u32) -> String {
    if i == u32::MAX { String::new() } else { strings.get(i as usize).cloned().unwrap_or_default() }
}

fn attribute(b: &[u8], strings: &[String], p: usize) -> Option<(String, String)> {
    let key = text(strings, u32_at(b, p + 4)?);
    let raw = u32_at(b, p + 8)?;
    let (kind, data) = (*b.get(p + 15)?, u32_at(b, p + 16)?);
    let value = match kind {
        TYPE_STRING => text(strings, if raw == u32::MAX { data } else { raw }),
        TYPE_BOOL => (data != 0).to_string(),
        TYPE_DEC => data.to_string(),
        _ => format!("{data:#x}"),
    };
    Some((key, value))
}

pub fn binary(b: &[u8]) -> Option<Vec<Event>> {
    if u16_at(b, 0)? != 0x0003 {
        return None;
    }
    let (mut at, mut strings, mut out) = (usize::from(u16_at(b, 2)?), Vec::new(), Vec::new());
    while at + 8 <= b.len() {
        let (kind, size) = (u16_at(b, at)?, u32_at(b, at + 4)? as usize);
        if size < 8 {
            break;
        }
        match kind {
            POOL => strings = pool(b, at)?,
            START => {
                let name = text(&strings, u32_at(b, at + 20)?);
                let first = usize::from(u16_at(b, at + 24)?);
                let each = usize::from(u16_at(b, at + 26)?);
                let count = usize::from(u16_at(b, at + 28)?);
                let attrs = (0..count)
                    .map(|k| attribute(b, &strings, at + 16 + first + k * each))
                    .collect::<Option<Vec<_>>>()?;
                out.push(Event::Start(name, attrs));
            }
            END => out.push(Event::End(text(&strings, u32_at(b, at + 20)?))),
            _ => {}
        }
        at += size;
    }
    Some(out)
}

pub fn plain(src: &str) -> Vec<Event> {
    let mut out = Vec::new();
    for m in TAG.captures_iter(src) {
        let name = m[2].to_string();
        if &m[1] == "/" {
            out.push(Event::End(name));
            continue;
        }
        let attrs = ATTR
            .captures_iter(&m[3])
            .map(|a| (a[1].rsplit(':').next().unwrap_or("").to_string(), a[2].to_string()))
            .collect();
        out.push(Event::Start(name.clone(), attrs));
        if &m[4] == "/" {
            out.push(Event::End(name));
        }
    }
    out
}

pub fn events(b: &[u8]) -> Option<Vec<Event>> {
    if b.first() == Some(&b'<') || b.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return Some(plain(&String::from_utf8_lossy(b)));
    }
    binary(b)
}

#[cfg(test)]
pub fn build(strings: &[&str], elements: &[(&str, &[(&str, &str)])]) -> Vec<u8> {
    let mut body = Vec::new();
    let mut offsets = Vec::new();
    for s in strings {
        offsets.push(body.len() as u32);
        body.extend((s.encode_utf16().count() as u16).to_le_bytes());
        body.extend(s.encode_utf16().flat_map(u16::to_le_bytes));
        body.extend([0u8, 0]);
    }
    while body.len() % 4 != 0 {
        body.push(0);
    }
    let idx = |s: &str| strings.iter().position(|x| *x == s).unwrap() as u32;
    let head = 28 + offsets.len() * 4;
    let mut chunks = Vec::new();
    chunks.extend(POOL.to_le_bytes());
    chunks.extend(28u16.to_le_bytes());
    chunks.extend(((head + body.len()) as u32).to_le_bytes());
    chunks.extend((strings.len() as u32).to_le_bytes());
    chunks.extend(0u32.to_le_bytes());
    chunks.extend(0u32.to_le_bytes());
    chunks.extend((head as u32).to_le_bytes());
    chunks.extend(0u32.to_le_bytes());
    offsets.iter().for_each(|o| chunks.extend(o.to_le_bytes()));
    chunks.extend(&body);
    for (name, attrs) in elements {
        let closing = name.starts_with('/');
        let n = name.trim_start_matches('/');
        let size = if closing { 24 } else { 36 + attrs.len() * 20 };
        chunks.extend((if closing { END } else { START }).to_le_bytes());
        chunks.extend(16u16.to_le_bytes());
        chunks.extend((size as u32).to_le_bytes());
        chunks.extend(1u32.to_le_bytes());
        chunks.extend(u32::MAX.to_le_bytes());
        chunks.extend(u32::MAX.to_le_bytes());
        chunks.extend(idx(n).to_le_bytes());
        if !closing {
            chunks.extend(20u16.to_le_bytes());
            chunks.extend(20u16.to_le_bytes());
            chunks.extend((attrs.len() as u16).to_le_bytes());
            chunks.extend([0u8; 6]);
            for (k, v) in *attrs {
                chunks.extend(u32::MAX.to_le_bytes());
                chunks.extend(idx(k).to_le_bytes());
                let (kind, data, raw) = match *v {
                    "true" => (TYPE_BOOL, u32::MAX, u32::MAX),
                    "false" => (TYPE_BOOL, 0, u32::MAX),
                    other => (TYPE_STRING, idx(other), idx(other)),
                };
                chunks.extend(raw.to_le_bytes());
                chunks.extend(8u16.to_le_bytes());
                chunks.extend([0u8, kind]);
                chunks.extend(data.to_le_bytes());
            }
        }
    }
    let mut out = Vec::new();
    out.extend(3u16.to_le_bytes());
    out.extend(8u16.to_le_bytes());
    out.extend(((chunks.len() + 8) as u32).to_le_bytes());
    out.extend(chunks);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_and_text_manifests_give_the_same_events() {
        let strings = ["manifest", "application", "debuggable", "name", ".Main"];
        let elements: &[(&str, &[(&str, &str)])] = &[
            ("manifest", &[]),
            ("application", &[("debuggable", "true"), ("name", ".Main")]),
            ("/application", &[]),
            ("/manifest", &[]),
        ];
        let got = events(&build(&strings, elements)).unwrap();
        assert_eq!(got.len(), 4);
        let want = Event::Start(
            "application".into(),
            vec![("debuggable".into(), "true".into()), ("name".into(), ".Main".into())],
        );
        assert_eq!(got[1], want);
        assert_eq!(got[2], Event::End("application".into()));
        let xml = r#"<manifest><application android:debuggable="true" android:name=".Main"/></manifest>"#;
        assert_eq!(events(xml.as_bytes()).unwrap()[1], want);
        assert!(events(b"nope").is_none());
    }
}
