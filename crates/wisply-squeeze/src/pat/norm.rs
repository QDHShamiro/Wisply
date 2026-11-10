use std::borrow::Cow;

use super::{digits, run_end, skip_ws, sw, word};

fn hex_run(w: &str) -> bool {
    w.len() >= 8 && w.bytes().all(|b| b.is_ascii_hexdigit())
}

pub(crate) fn numbers(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    numbers_into(s, &mut out);
    out
}

pub(crate) fn numbers_ascii(s: &str, out: &mut String) {
    let b = s.as_bytes();
    let wordb = |c: u8| c.is_ascii_alphanumeric() || c == b'_';
    let mut i = 0;
    while i < b.len() {
        if !wordb(b[i]) {
            out.push(b[i] as char);
            i += 1;
            continue;
        }
        let (mut j, mut digit, mut hex) = (i, false, true);
        while j < b.len() && wordb(b[j]) {
            digit |= b[j].is_ascii_digit();
            hex &= b[j].is_ascii_hexdigit();
            j += 1;
        }
        let run = &b[i..j];
        if hex && run.len() >= 8 {
            out.push('#');
        } else if !digit {
            out.push_str(&s[i..j]);
        } else {
            let mut k = 0;
            while k < run.len() {
                let tail = &run[k..];
                let h = if tail.starts_with(b"0x") {
                    tail[2..].iter().take_while(|c| c.is_ascii_hexdigit()).count()
                } else {
                    0
                };
                if h > 0 {
                    out.push('#');
                    k += 2 + h;
                } else if tail[0].is_ascii_digit() {
                    out.push('#');
                    k += tail.iter().take_while(|c| c.is_ascii_digit()).count();
                } else {
                    out.push(tail[0] as char);
                    k += 1;
                }
            }
        }
        i = j;
    }
}

pub(crate) fn numbers_into(s: &str, out: &mut String) {
    if s.is_ascii() {
        return numbers_ascii(s, out);
    }
    let mut i = 0;
    let b = s.as_bytes();
    while i < s.len() {
        if b[i] < 0x80 && !(b[i].is_ascii_alphanumeric() || b[i] == b'_') {
            out.push(b[i] as char);
            i += 1;
            continue;
        }
        let c = s[i..].chars().next().unwrap_or(' ');
        if !word(c) {
            out.push(c);
            i += c.len_utf8();
            continue;
        }
        let end = run_end(s, i);
        let run = &s[i..end];
        if !run.bytes().any(|x| x.is_ascii_digit()) && !hex_run(run) {
            out.push_str(run);
            i = end;
            continue;
        }
        if !s[..i].chars().next_back().is_some_and(word) && hex_run(run) && !run.starts_with("0x") {
            out.push('#');
            i = end;
            continue;
        }
        let mut k = 0;
        while k < run.len() {
            let tail = &run[k..];
            let h = tail.strip_prefix("0x").map_or(0, |t| t.bytes().take_while(u8::is_ascii_hexdigit).count());
            if h > 0 {
                out.push('#');
                k += 2 + h;
            } else if k == 0 && hex_run(run) {
                out.push('#');
                k = run.len();
            } else if tail.as_bytes()[0].is_ascii_digit() {
                out.push('#');
                k += digits(tail);
            } else {
                let c = tail.chars().next().unwrap_or(' ');
                out.push(c);
                k += c.len_utf8();
            }
        }
        i = end;
    }
}

const QUIET_KEYS: [&str; 4] = ["error", "fail", "exception", "warn"];

fn quiet_at(s: &str, p: usize) -> Option<usize> {
    let key_start = p + 1;
    let key_end = key_start + s[key_start..].find(|c: char| !(word(c) || c == '-'))?;
    if !s[key_end..].starts_with('"') {
        return None;
    }
    let key = &s[key_start..key_end];
    let low = key.to_ascii_lowercase();
    if !QUIET_KEYS.iter().any(|k| low.contains(k)) {
        return None;
    }
    let mut at = key_end + 1;
    at += skip_ws(&s[at..]);
    if !s[at..].starts_with(':') {
        return None;
    }
    at += 1;
    at += skip_ws(&s[at..]);
    for v in ["null", "false", "0", "[]", "{}", "\"\""] {
        if sw(&s[at..], v) {
            return Some(at + v.len());
        }
    }
    None
}

pub(crate) fn quiet(s: &str) -> Cow<'_, str> {
    if !s.contains('"') {
        return Cow::Borrowed(s);
    }
    let mut out = String::new();
    let (mut last, mut i) = (0, 0);
    while let Some(n) = s[i..].find('"') {
        let p = i + n;
        match quiet_at(s, p) {
            Some(end) => {
                out.push_str(&s[last..p]);
                last = end;
                i = end;
            }
            None => i = p + 1,
        }
    }
    if last == 0 && out.is_empty() {
        return Cow::Borrowed(s);
    }
    out.push_str(&s[last..]);
    Cow::Owned(out)
}

fn ansi_len(b: &[u8], p: usize) -> Option<usize> {
    match *b.get(p + 1)? {
        b'[' => {
            let mut j = p + 2;
            while b.get(j).is_some_and(|c| c.is_ascii_digit() || matches!(c, b';' | b'?')) {
                j += 1;
            }
            while b.get(j).is_some_and(|c| (b' '..=b'/').contains(c)) {
                j += 1;
            }
            b.get(j).filter(|c| (b'@'..=b'~').contains(c)).map(|_| j + 1 - p)
        }
        b']' => {
            let mut j = p + 2;
            while b.get(j).is_some_and(|c| !matches!(c, 0x07 | 0x1b)) {
                j += 1;
            }
            match (b.get(j), b.get(j + 1)) {
                (Some(0x07), _) => Some(j + 1 - p),
                (Some(0x1b), Some(b'\\')) => Some(j + 2 - p),
                _ => None,
            }
        }
        b'(' | b')' => b.get(p + 2).filter(|c| c.is_ascii_uppercase() || c.is_ascii_digit()).map(|_| 3),
        _ => None,
    }
}

pub(crate) fn strip_ansi(s: &str) -> Cow<'_, str> {
    if !s.contains('\x1b') {
        return Cow::Borrowed(s);
    }
    let b = s.as_bytes();
    let mut out = String::with_capacity(s.len());
    let (mut last, mut i) = (0, 0);
    while let Some(n) = s[i..].find('\x1b') {
        let p = i + n;
        match ansi_len(b, p) {
            Some(len) => {
                out.push_str(&s[last..p]);
                last = p + len;
                i = last;
            }
            None => i = p + 1,
        }
    }
    out.push_str(&s[last..]);
    Cow::Owned(out)
}
