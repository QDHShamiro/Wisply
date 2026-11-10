mod norm;
mod shape;

pub(crate) use norm::{numbers, numbers_ascii, numbers_into, quiet, strip_ansi};
pub(crate) use shape::{body, runtime, summary, trace};

pub(crate) struct Kind {
    pub hot: bool,
    pub noise: bool,
}

fn word(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || matches!(c, '\u{300}'..='\u{36f}' | '\u{200c}' | '\u{200d}')
}

fn run_end(s: &str, from: usize) -> usize {
    s[from..].find(|c| !word(c)).map_or(s.len(), |n| from + n)
}

fn starts_word(s: &str, at: usize) -> bool {
    s[at..].chars().next().is_some_and(word) && !s[..at].chars().next_back().is_some_and(word)
}

fn boundary_after(s: &str, at: usize) -> bool {
    !s[at..].chars().next().is_some_and(word)
}

fn sw(s: &str, prefix: &str) -> bool {
    s.len() >= prefix.len() && s.as_bytes()[..prefix.len()].eq_ignore_ascii_case(prefix.as_bytes())
}

fn sw_word(s: &str, prefix: &str) -> bool {
    sw(s, prefix) && boundary_after(s, prefix.len())
}

fn digits(s: &str) -> usize {
    s.bytes().take_while(u8::is_ascii_digit).count()
}

fn skip_ws(s: &str) -> usize {
    s.len() - s.trim_start_matches(char::is_whitespace).len()
}

struct Runs<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> Iterator for Runs<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let rest = &self.s[self.pos..];
        let start = self.pos + rest.find(word)?;
        let end = run_end(self.s, start);
        self.pos = end;
        Some((start, end))
    }
}

fn runs(s: &str) -> Runs<'_> {
    Runs { s, pos: 0 }
}

const ERROR_WORDS: [&str; 32] = [
    "error",
    "errors",
    "exception",
    "exceptions",
    "err",
    "fail",
    "failed",
    "failing",
    "failure",
    "failures",
    "fatal",
    "traceback",
    "panic",
    "panicked",
    "warn",
    "warning",
    "warnings",
    "denied",
    "refused",
    "cannot",
    "unable",
    "missing",
    "undefined",
    "segfault",
    "abort",
    "aborted",
    "timeout",
    "killed",
    "assert",
    "assertion",
    "severe",
    "critical",
];

fn camel_error(w: &str) -> bool {
    let base = w.strip_suffix('s').unwrap_or(w);
    for candidate in [w, base] {
        for tail in ["Error", "Exception"] {
            let Some(head) = candidate.strip_suffix(tail) else { continue };
            if head.is_empty() {
                continue;
            }
            let chain = head.bytes().rev().take_while(u8::is_ascii_alphanumeric).filter(u8::is_ascii_uppercase).count();
            if chain > 0 {
                return true;
            }
        }
    }
    false
}

fn phrase(rest: &str, tail: &str) -> bool {
    sw(rest, tail) && boundary_after(rest, tail.len())
}

const STARTS: [&[u8; 3]; 22] = [
    b"err", b"exc", b"fai", b"fat", b"tra", b"pan", b"war", b"den", b"ref", b"can", b"una", b"mis", b"und", b"seg",
    b"abo", b"tim", b"kil", b"ass", b"sev", b"cri", b"no ", b"not",
];

const fn slot(a: u8, b: u8, c: u8) -> usize {
    ((a as usize) * 961 + (b as usize) * 31 + c as usize) & 4095
}

const SEEN: [u64; 64] = {
    let mut t = [0u64; 64];
    let mut i = 0;
    while i < STARTS.len() {
        let n = slot(STARTS[i][0], STARTS[i][1], STARTS[i][2]);
        t[n >> 6] |= 1 << (n & 63);
        i += 1;
    }
    t
};

pub(crate) fn may_be_important(s: &str) -> bool {
    s.as_bytes().windows(3).any(|w| {
        let n = slot(w[0] | 0x20, w[1] | 0x20, w[2] | 0x20);
        SEEN[n >> 6] >> (n & 63) & 1 != 0
    })
}

pub(crate) fn important(s: &str) -> bool {
    may_be_important(s) && important_slow(s)
}

fn important_slow(s: &str) -> bool {
    for (a, b) in runs(s) {
        let w = &s[a..b];
        let bytes = w.as_bytes();
        if (3..=10).contains(&w.len()) && ERROR_WORDS.iter().any(|e| e.eq_ignore_ascii_case(w)) {
            return true;
        }
        if bytes.len() >= 4 && bytes[..4].eq_ignore_ascii_case(b"err_") {
            return true;
        }
        let low = w.strip_suffix(['s', 'S']).unwrap_or(w);
        for tail in ["_error", "_exception"] {
            for cand in [w, low] {
                if cand.len() >= tail.len()
                    && cand.as_bytes()[cand.len() - tail.len()..].eq_ignore_ascii_case(tail.as_bytes())
                {
                    return true;
                }
            }
        }
        if camel_error(w) {
            return true;
        }
        let rest = &s[b..];
        let hit = match bytes.len() {
            2 if w.eq_ignore_ascii_case("no") => phrase(rest, " such"),
            3 if w.eq_ignore_ascii_case("not") => phrase(rest, " found"),
            3 if w.eq_ignore_ascii_case("can") => rest.starts_with("'t") && boundary_after(rest, 2),
            5 if w.eq_ignore_ascii_case("timed") => phrase(rest, " out"),
            _ => false,
        };
        if hit {
            return true;
        }
    }
    false
}

const NEG_FIRST: [&str; 4] = ["0", "no", "zero", "without"];
const NEG_SECOND: [&str; 14] = [
    "error",
    "errors",
    "warning",
    "warnings",
    "failure",
    "failures",
    "failed",
    "failing",
    "issue",
    "issues",
    "problem",
    "problems",
    "vulnerability",
    "vulnerabilities",
];
const NEG_ZERO: [&str; 8] = ["error", "errors", "warning", "warnings", "failure", "failures", "failed", "failing"];

fn word_in(w: &str, list: &[&str]) -> bool {
    list.iter().any(|e| e.eq_ignore_ascii_case(w))
}

fn neg_at(s: &str, i: usize) -> Option<usize> {
    let c = s[i..].chars().next()?;
    if word(c) && starts_word(s, i) {
        let e1 = run_end(s, i);
        let w1 = &s[i..e1];
        if word_in(w1, &NEG_FIRST) {
            let gap = skip_ws(&s[e1..]);
            if gap > 0 && starts_word(s, e1 + gap) {
                let e2 = run_end(s, e1 + gap);
                if word_in(&s[e1 + gap..e2], &NEG_SECOND) {
                    return Some(e2);
                }
            }
        }
        if word_in(w1, &NEG_ZERO) {
            let mut at = e1 + skip_ws(&s[e1..]);
            let sep = matches!(s[at..].chars().next(), Some(':' | '='));
            if sep {
                at += 1;
            }
            at += skip_ws(&s[at..]);
            if s[at..].starts_with('0') && boundary_after(s, at + 1) {
                return Some(at + 1);
            }
            if sep && matches!(w1.to_ascii_lowercase().as_str(), "error" | "errors" | "warning" | "warnings") {
                for t in ["none", "null", "false", "[]", "{}"] {
                    if sw(&s[at..], t) {
                        return Some(at + t.len());
                    }
                }
            }
        }
        if w1.eq_ignore_ascii_case("fail_fast") {
            return Some(e1);
        }
        if sw(&s[i..], "no-fail") && boundary_after(s, i + 7) {
            return Some(i + 7);
        }
    }
    if s[i..].starts_with('-') {
        let mut j = i + 1;
        if s[j..].starts_with('-') {
            j += 1;
        }
        for t in ["no-fail-fast", "fail-fast"] {
            if sw(&s[j..], t) && boundary_after(s, j + t.len()) {
                return Some(j + t.len());
            }
        }
    }
    None
}

fn neg_free(s: &str) -> Option<String> {
    let mut out = String::new();
    let (mut last, mut i) = (0, 0);
    while i < s.len() {
        match neg_at(s, i) {
            Some(end) if end > i => {
                out.push_str(&s[last..i]);
                out.push(' ');
                last = end;
                i = end;
            }
            _ => i += s[i..].chars().next().map_or(1, char::len_utf8),
        }
    }
    if last == 0 && out.is_empty() {
        return None;
    }
    out.push_str(&s[last..]);
    Some(out)
}

const BLOCKS: [char; 10] = ['█', '▉', '▊', '▋', '▌', '▍', '▎', '▏', '▓', '▒'];

fn block_run(s: &str) -> bool {
    let mut count = 0;
    for c in s.chars() {
        if BLOCKS.contains(&c) || c == '░' || c == '━' {
            count += 1;
            if count == 5 {
                return true;
            }
        } else {
            count = 0;
        }
    }
    false
}

fn bar(s: &str) -> bool {
    let mut from = 0;
    while let Some(n) = s[from..].find('[') {
        let mut rest = &s[from + n + 1..];
        rest = rest.trim_start_matches([' ', '.', '-']);
        let fill = rest.bytes().take_while(|b| matches!(b, b'=' | b'#')).count();
        if fill >= 2 {
            let tail = &rest[fill..];
            let end = tail.trim_start_matches(['=', '#', '>', '-', '.', ' ']);
            if end.starts_with(']') {
                return true;
            }
        }
        from += n + 1;
    }
    false
}

fn arrows(s: &str) -> bool {
    let mut from = 0;
    while let Some(n) = s[from..].find('<') {
        let rest = &s[from + n + 1..];
        let fill = rest.bytes().take_while(|b| matches!(b, b'=' | b'-')).count();
        if fill >= 5 && rest[fill..].starts_with('>') {
            return true;
        }
        from += n + 1;
    }
    false
}

fn download(rest: &str) -> bool {
    let Some(mut r) = rest.strip_prefix("Download") else { return false };
    for suffix in ["ing", "ed", ""] {
        if let Some(after) = r.strip_prefix(suffix) {
            r = after;
            break;
        }
    }
    let url = |t: &str| t.strip_prefix(" http://").is_some() || t.strip_prefix(" https://").is_some();
    if url(r) {
        return true;
    }
    if let Some(tail) = r.strip_prefix(" from ") {
        let token = tail.split(char::is_whitespace).next().unwrap_or("");
        if token.len() >= 2 && token.ends_with(':') && url(&tail[token.len()..]) {
            return true;
        }
    }
    false
}

fn noise(raw: &str) -> bool {
    let lead = raw.trim_start_matches(char::is_whitespace);
    if lead.chars().next().is_some_and(|c| ('\u{2800}'..='\u{28ff}').contains(&c)) {
        return true;
    }
    if let Some(r) = lead.strip_prefix("Progress (") {
        let n = digits(r);
        if n > 0 && r[n..].starts_with("):") {
            return true;
        }
    }
    if lead.starts_with("Download") && download(lead) {
        return true;
    }
    if let Some(r) = raw.strip_prefix("npm ") {
        for t in ["http", "timing", "sill", "verb"] {
            if r.starts_with(t) && boundary_after(r, t.len()) {
                return true;
            }
        }
    }
    if raw.contains('[') && bar(raw) {
        return true;
    }
    if raw.contains('<') && arrows(raw) {
        return true;
    }
    !raw.is_ascii() && block_run(raw)
}

pub(crate) fn kind(raw: &str) -> Kind {
    if !important(raw) {
        return Kind { hot: false, noise: noise(raw) };
    }
    let hot = match neg_free(raw) {
        None => true,
        Some(clean) => important(&clean),
    };
    Kind { hot, noise: !hot && noise(raw) }
}

#[cfg(test)]
pub(crate) fn hot(text: &str) -> bool {
    kind(text).hot
}

#[cfg(test)]
mod tests;
