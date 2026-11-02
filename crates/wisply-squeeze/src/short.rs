use crate::pat;
use crate::text::{clip, len};

pub(crate) const LINE_CHARS: usize = 300;
pub(crate) const WIDE_CHARS: usize = 340;
const GREP_TEXT: usize = 200;
const GREP_PER_FILE: usize = 6;
const GREP_MIN_LINES: usize = 6;
const GREP_SHARE: usize = 80;

struct Hit<'a> {
    path: &'a str,
    num: &'a str,
    text: &'a str,
}

fn digits(s: &str) -> usize {
    s.bytes().take_while(u8::is_ascii_digit).count()
}

fn parse(line: &str) -> Option<Hit<'_>> {
    let b = line.as_bytes();
    let from = if b.len() > 2 && b[1] == b':' && matches!(b[2], b'\\' | b'/') { 2 } else { 0 };
    let colon = from + line[from..].find(':')?;
    let path = &line[..colon];
    let file = path.contains(['.', '/', '\\']) && !path.contains(['[', ']']);
    if !file || path.starts_with(char::is_whitespace) || path.starts_with('-') {
        return None;
    }
    let rest = &line[colon + 1..];
    let n = digits(rest);
    if n == 0 || rest.as_bytes().get(n) != Some(&b':') {
        return None;
    }
    let mut end = n;
    let after = &rest[n + 1..];
    let col = digits(after);
    if col > 0 && after.as_bytes().get(col) == Some(&b':') {
        end = n + 1 + col;
    }
    Some(Hit { path, num: &rest[..end], text: &rest[end + 1..] })
}

fn flush(group: &mut Vec<(&str, &str, bool)>, path: &str, out: &mut Vec<String>) {
    if group.is_empty() {
        return;
    }
    if group.len() == 1 {
        let (num, text, hot) = group[0];
        let text = if hot { text.to_string() } else { clip(text, GREP_TEXT) };
        out.push(format!("{path}:{num}:{text}"));
        group.clear();
        return;
    }
    out.push(format!("{path} ({})", group.len()));
    let (mut shown, mut dropped) = (0, 0);
    for &(num, text, hot) in group.iter() {
        if !hot && shown >= GREP_PER_FILE {
            dropped += 1;
            continue;
        }
        if !hot {
            shown += 1;
        }
        let text = if hot { text.to_string() } else { clip(text, GREP_TEXT) };
        out.push(format!("  {num}:{text}"));
    }
    if dropped > 0 {
        out.push(format!("  +{dropped} more"));
    }
    group.clear();
}

pub(crate) fn grep<S: AsRef<str>>(lines: &[S]) -> Option<Vec<String>> {
    let real = lines.iter().filter(|l| !l.as_ref().is_empty()).count();
    if real < GREP_MIN_LINES {
        return None;
    }
    let parsed: Vec<Option<Hit>> = lines.iter().map(|l| parse(l.as_ref())).collect();
    let shaped = parsed.iter().flatten().count();
    if shaped * 100 < real * GREP_SHARE {
        return None;
    }
    let mut out = Vec::new();
    let mut group: Vec<(&str, &str, bool)> = Vec::new();
    let mut path = "";
    for (line, hit) in lines.iter().zip(&parsed) {
        match hit {
            Some(h) => {
                if h.path != path {
                    flush(&mut group, path, &mut out);
                    path = h.path;
                }
                group.push((h.num, h.text, pat::kind(h.text).hot));
            }
            None => {
                flush(&mut group, path, &mut out);
                path = "";
                out.push(line.as_ref().to_string());
            }
        }
    }
    flush(&mut group, path, &mut out);
    Some(out)
}

pub(crate) fn tighten<S: AsRef<str>>(lines: &[S]) -> (Vec<String>, bool) {
    if let Some(grouped) = grep(lines) {
        return (grouped, true);
    }
    let cut = lines
        .iter()
        .map(|l| {
            let l = l.as_ref();
            if len(l) > WIDE_CHARS && !pat::kind(l).hot { clip(l, LINE_CHARS) } else { l.to_string() }
        })
        .collect();
    (cut, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(raw: &str) -> Vec<&str> {
        raw.lines().collect()
    }

    #[test]
    fn grep_lines_parse_with_and_without_columns_and_drives() {
        let h = parse("src/a.rs:12:let x = 1;").expect("hit");
        assert_eq!((h.path, h.num, h.text), ("src/a.rs", "12", "let x = 1;"));
        let h = parse("src/a.rs:12:5:fn main() {").expect("hit");
        assert_eq!((h.path, h.num, h.text), ("src/a.rs", "12:5", "fn main() {"));
        let h = parse("C:\\p\\a.rs:3:x").expect("hit");
        assert_eq!((h.path, h.num, h.text), ("C:\\p\\a.rs", "3", "x"));
        assert!(parse("error: 5: nope").is_none());
        assert!(parse("plain text line").is_none());
        assert!(parse("--").is_none());
        assert!(parse("[12:00:00 INFO]: Player 5: joined").is_none());
        assert!(parse("12:00:01 3: tick").is_none());
    }

    #[test]
    fn matches_group_under_one_path_and_cap_with_a_count() {
        let mut raw: Vec<String> = (1..=10).map(|i| format!("src/long/path/a.rs:{i}:let v{i} = {i};")).collect();
        raw.push("src/long/path/b.rs:4:only one".into());
        raw.push("src/long/path/c.rs:1:x".into());
        raw.push("src/long/path/c.rs:2:y".into());
        let out = grep(&raw).expect("grouped");
        assert_eq!(out[0], "src/long/path/a.rs (10)");
        assert_eq!(out[1], "  1:let v1 = 1;");
        assert_eq!(out[6], "  6:let v6 = 6;");
        assert_eq!(out[7], "  +4 more");
        assert_eq!(out[8], "src/long/path/b.rs:4:only one");
        assert_eq!(&out[9..], ["src/long/path/c.rs (2)", "  1:x", "  2:y"]);
    }

    #[test]
    fn error_hits_are_never_dropped_or_clipped() {
        let long = format!("fatal error {}", "x".repeat(400));
        let mut raw: Vec<String> = (1..=12).map(|i| format!("a.rs:{i}:fine {i}")).collect();
        raw.push(format!("a.rs:13:{long}"));
        raw.push("a.rs:14:ERROR boom".into());
        let out = grep(&raw).expect("grouped");
        assert!(out.iter().any(|l| l.ends_with(&long)));
        assert!(out.iter().any(|l| l == "  14:ERROR boom"));
    }

    #[test]
    fn other_output_is_not_grep() {
        let raw = lines("a\nb\nc\nd\ne\nf\ng\nh");
        assert!(grep(&raw).is_none());
        let few = lines("a.rs:1:x\na.rs:2:y");
        assert!(grep(&few).is_none());
    }

    #[test]
    fn long_lines_are_cut_but_error_lines_stay_whole() {
        let wide = "w".repeat(500);
        let bad = format!("failed: {}", "z".repeat(500));
        let (out, grouped) = tighten(&[wide.as_str(), bad.as_str(), "short"]);
        assert!(!grouped);
        assert_eq!(out[0], format!("{}... (+200 chars)", "w".repeat(300)));
        assert_eq!(out[1], bad);
        assert_eq!(out[2], "short");
    }
}
