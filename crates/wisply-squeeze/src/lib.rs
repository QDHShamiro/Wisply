mod json;
mod log;
#[cfg(test)]
mod oracle;
mod pat;
mod short;
mod text;
mod tool;
mod web;

pub use text::commas;
pub use tool::{apply, extract, squeeze_tool};

const MIN_LINES: usize = 60;
const MIN_CHARS: usize = 4000;
pub const SHORT_CHARS: usize = 1200;
const WEB_CHARS: usize = 1500;
const MID_CHARS: usize = 2000;
const MID_GAIN: usize = 25;

pub struct Squeezed {
    pub plain: String,
    pub body: String,
    pub lines_in: usize,
    pub lines_out: usize,
    pub chars_in: usize,
    pub chars_out: usize,
    pub json: bool,
}

pub fn clean(text: &str) -> String {
    log::lines(&pat::strip_ansi(text)).join("\n")
}

pub fn decode(raw: &[u8]) -> String {
    if raw.starts_with(b"\xff\xfe") || raw.starts_with(b"\xfe\xff") {
        let le = raw.starts_with(b"\xff\xfe");
        let units: Vec<u16> = raw[2..]
            .chunks(2)
            .map(|c| match c {
                [a, b] if le => u16::from_le_bytes([*a, *b]),
                [a, b] => u16::from_be_bytes([*a, *b]),
                _ => 0xFFFD,
            })
            .collect();
        return String::from_utf16_lossy(&units);
    }
    String::from_utf8_lossy(raw.strip_prefix(b"\xef\xbb\xbf").unwrap_or(raw)).into_owned()
}

pub fn read_limited(mut from: impl std::io::Read, max: usize) -> Vec<u8> {
    let keep = max / 2;
    let (mut head, mut data, mut dropped) = (None::<Vec<u8>>, Vec::new(), 0usize);
    let mut chunk = vec![0u8; 1 << 20];
    loop {
        let n = match from.read(&mut chunk) {
            Ok(0) | Err(_) => break,
            Ok(n) => n,
        };
        data.extend_from_slice(&chunk[..n]);
        if head.is_none() && data.len() > keep {
            head = Some(data[..keep].to_vec());
            data.drain(..keep);
        } else if head.is_some() && data.len() > keep {
            let extra = data.len() - keep;
            dropped += extra;
            data.drain(..extra);
        }
    }
    let Some(mut out) = head else { return data };
    if dropped > 0 {
        out.extend(format!("\n... {} MB in the middle not read\n", dropped / 1_000_000).bytes());
    }
    out.extend(data);
    out
}

fn finish(lines: &[&str], out: Vec<String>, json: bool) -> Option<Squeezed> {
    let plain = lines.join(
        "
",
    );
    let body = out.join(
        "
",
    );
    let (chars_in, chars_out) = (text::len(&plain), text::len(&body));
    (chars_out < chars_in).then_some(Squeezed {
        lines_in: lines.len(),
        lines_out: out.len(),
        chars_in,
        chars_out,
        json,
        body,
        plain,
    })
}

fn size<S: AsRef<str>>(lines: &[S]) -> usize {
    lines.iter().map(|l| text::len(l.as_ref())).sum::<usize>() + lines.len().saturating_sub(1)
}

pub fn squeeze(text: &str) -> Option<Squeezed> {
    let stripped = pat::strip_ansi(text);
    let lines = log::lines(&stripped);
    let chars = size(&lines);
    if lines.len() <= MIN_LINES && chars <= MIN_CHARS {
        if chars < SHORT_CHARS && lines.iter().all(|l| text::len(l) <= short::WIDE_CHARS) {
            return None;
        }
        let (tight, grouped) = short::tighten(&lines);
        if chars > MID_CHARS && !grouped {
            let folded = log::squeeze(&tight);
            if size(&folded) * 100 <= chars * (100 - MID_GAIN) {
                return finish(&lines, folded, false);
            }
        }
        return finish(&lines, tight, false);
    }
    let plain = lines.join(
        "
",
    );
    let (mut out, json) = match json::parse(&plain) {
        Some((obj, rows)) => (json::squeeze(&obj, rows), true),
        None => (log::squeeze(&lines), false),
    };
    if !json && let Some(mut grouped) = short::grep(&lines) {
        if grouped.len() > MIN_LINES {
            grouped = log::squeeze(&grouped);
        }
        if size(&grouped) < size(&out) {
            out = grouped;
        }
    }
    finish(&lines, out, json)
}

pub fn squeeze_web(text: &str) -> Option<Squeezed> {
    let stripped = pat::strip_ansi(text);
    let lines = log::lines(&stripped);
    if size(&lines) < WEB_CHARS {
        return None;
    }
    finish(&lines, web::web(&lines), false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn big_input_keeps_head_and_tail() {
        let raw: Vec<u8> = (0..30u8).flat_map(|i| [b'a' + i % 26; 10]).collect();
        let got = read_limited(&raw[..], 100);
        assert_eq!(&got[..50], &raw[..50]);
        assert!(got.ends_with(&raw[250..]));
        assert_eq!(read_limited(&raw[..40], 100), &raw[..40]);
        assert_eq!(decode(b"\xef\xbb\xbfhi"), "hi");
        assert_eq!(decode(b"\xff\xfeh\0i\0"), "hi");
    }

    #[test]
    fn small_outputs_stay_as_they_are() {
        assert!(squeeze("one\ntwo").is_none());
        assert!(squeeze(&"x".repeat(300)).is_none());
        assert!(
            squeeze(
                &format!(
                    "{}
",
                    "word ".repeat(40)
                )
                .repeat(5)
            )
            .is_none()
        );
        assert!(
            squeeze(
                "git status
on branch main
nothing to commit"
            )
            .is_none()
        );
        assert!(squeeze(&"ok\n".repeat(61)).is_some());
    }

    #[test]
    fn reports_sizes_of_both_sides() {
        let text = (0..500).map(|i| format!("row {i} fine")).collect::<Vec<_>>().join("\n") + "\n";
        let s = squeeze(&text).expect("squeezed");
        assert_eq!((s.lines_in, s.chars_in, s.json), (500, text.chars().count() - 1, false));
        assert_eq!((s.lines_out, s.chars_out), (s.body.lines().count(), s.body.chars().count()));
        assert!(s.chars_out < s.chars_in);
    }
}
