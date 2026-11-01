use std::borrow::Cow;

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

pub(crate) fn important(s: &str) -> bool {
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

fn timestamp(s: &str) -> Option<usize> {
    let b = s.as_bytes();
    let d = |at: usize, n: usize| b.len() >= at + n && b[at..at + n].iter().all(u8::is_ascii_digit);
    if !(d(0, 4) && b.get(4) == Some(&b'-') && d(5, 2) && b.get(7) == Some(&b'-') && d(8, 2)) {
        return None;
    }
    if !(matches!(b.get(10), Some(b'T' | b' ')) && d(11, 2) && b.get(13) == Some(&b':') && d(14, 2)) {
        return None;
    }
    if !(b.get(16) == Some(&b':') && d(17, 2)) {
        return None;
    }
    let mut at = 19;
    if matches!(b.get(at), Some(b'.' | b',')) {
        let n = digits(&s[at + 1..]);
        if n > 0 {
            at += 1 + n;
        }
    }
    if b.get(at) == Some(&b'Z') {
        at += 1;
    }
    if b.get(at) != Some(&b' ') {
        return None;
    }
    at += 1;
    let caps = b[at..].iter().take_while(|c| c.is_ascii_uppercase()).count();
    if (3..=8).contains(&caps) && b.get(at + caps) == Some(&b' ') {
        at += caps + 1;
    }
    Some(at)
}

fn bracket(s: &str) -> Option<usize> {
    let rest = s.strip_prefix('[')?;
    let mut end = None;
    for (n, (i, c)) in rest.char_indices().enumerate() {
        if c == ']' {
            end = Some(i);
            break;
        }
        if n >= 60 {
            return None;
        }
    }
    let mut at = 1 + end? + 1;
    if s[at..].starts_with(':') {
        at += 1;
    }
    if s[at..].starts_with(' ') {
        at += 1;
    }
    Some(at)
}

pub(crate) fn body(raw: &str) -> &str {
    let mut at = timestamp(raw).unwrap_or(0);
    for _ in 0..3 {
        match bracket(&raw[at..]) {
            Some(n) => at += n,
            None => break,
        }
    }
    &raw[at..]
}

pub(crate) fn trace(body: &str) -> bool {
    let rest = body.trim_start_matches(char::is_whitespace);
    let indented = rest.len() < body.len();
    if rest.starts_with("File \"") {
        return true;
    }
    if !indented {
        return false;
    }
    if let Some(r) = rest.strip_prefix("at ") {
        return r.chars().next().is_some_and(|c| !c.is_whitespace());
    }
    if rest.starts_with('^') {
        return true;
    }
    if let Some(r) = rest.strip_prefix("... ") {
        let n = digits(r);
        return n > 0 && (r[n..].starts_with(" more") || r[n..].starts_with(" common frames"));
    }
    let n = digits(rest);
    if n > 0 {
        return rest[n..].strip_prefix(": ").and_then(|r| r.chars().next()).is_some_and(|c| !c.is_whitespace());
    }
    if rest.starts_with('/') {
        let token = rest.split(char::is_whitespace).next().unwrap_or("");
        let mut from = 2;
        while let Some(p) = token.get(from..).and_then(|t| t.find(".go:")) {
            let after = &token[from + p + 4..];
            if after.bytes().next().is_some_and(|b| b.is_ascii_digit()) {
                return true;
            }
            from += p + 1;
        }
    }
    false
}

const RUNTIME_PREFIXES: [&str; 25] = [
    "java.",
    "javax.",
    "jdk.",
    "sun.",
    "com.sun.",
    "kotlin.",
    "kotlinx.",
    "scala.",
    "groovy.",
    "org.codehaus.groovy.",
    "org.gradle.",
    "org.apache.maven.",
    "org.junit.",
    "junit.",
    "org.bukkit.",
    "org.spigotmc.",
    "net.minecraft.",
    "io.papermc.",
    "com.destroystokyo.",
    "io.netty.",
    "com.mojang.",
    "net.md_5.",
    "org.springframework.",
    "System.",
    "Microsoft.",
];

const RUNTIME_ANYWHERE: [&str; 12] = [
    "java.base/",
    "node:internal",
    "node_modules/",
    "node_modules\\",
    "site-packages/",
    "site-packages\\",
    "lib/python3",
    "lib\\python3",
    "/rustc/",
    "__rust_",
    "rust_begin_unwind",
    "/usr/local/go/src/",
];

pub(crate) fn runtime(raw: &str) -> bool {
    let lead = raw.trim_start_matches(char::is_whitespace);
    let after_at = lead
        .strip_prefix("at")
        .filter(|r| r.starts_with(char::is_whitespace))
        .map(|r| r.trim_start_matches(char::is_whitespace));
    if RUNTIME_PREFIXES.iter().any(|p| lead.starts_with(p) || after_at.is_some_and(|a| a.starts_with(p))) {
        return true;
    }
    if RUNTIME_ANYWHERE.iter().any(|p| raw.contains(p)) {
        return true;
    }
    for lib in ["std::", "core::", "alloc::", "tokio::"] {
        let mut from = 0;
        while let Some(p) = raw[from..].find(lib) {
            if !raw[..from + p].chars().next_back().is_some_and(word) {
                return true;
            }
            from += p + 1;
        }
    }
    false
}

fn count_then(r: &str, tail: &str) -> bool {
    let n = digits(r);
    n > 0 && sw(&r[n..], tail)
}

pub(crate) fn summary(body: &str) -> bool {
    let r = body.trim_start_matches(char::is_whitespace);
    for p in [
        "test result:",
        "tests run:",
        "test run:",
        "test suite:",
        "test suites:",
        "build success",
        "build failed",
        "build failure",
        "total time:",
        "compiled successfully",
        "error: could not compile",
    ] {
        if sw(r, p) {
            return true;
        }
    }
    for p in ["tests:", "test:"] {
        if let Some(t) = r.get(..p.len()).filter(|h| h.eq_ignore_ascii_case(p)).map(|_| &r[p.len()..]) {
            let gap = skip_ws(t);
            if gap > 0 && digits(&t[gap..]) > 0 {
                return true;
            }
        }
    }
    if sw_word(r, "finished") || sw_word(r, "done in") {
        return true;
    }
    if sw(r, "done (") && digits(&r[6..]) > 0 {
        return true;
    }
    if let Some(t) = r.get(..4).filter(|h| h.eq_ignore_ascii_case("ran ")).map(|_| &r[4..])
        && count_then(t, " test")
    {
        return true;
    }
    if let Some(t) = r.get(..6).filter(|h| h.eq_ignore_ascii_case("added ")).map(|_| &r[6..])
        && count_then(t, " packages")
    {
        return true;
    }
    if let Some(t) = r.get(..6).filter(|h| h.eq_ignore_ascii_case("found ")).map(|_| &r[6..])
        && count_then(t, " vulnerabilit")
    {
        return true;
    }
    let n = digits(r);
    if n > 0 && r[n..].starts_with(' ') {
        let t = &r[n + 1..];
        for p in ["passed", "failed", "test"] {
            if sw_word(t, p) {
                return true;
            }
        }
        if sw_word(t, "tests") {
            return true;
        }
        for p in ["low", "moderate", "high", "critical"] {
            if sw(t, p) && sw(&t[p.len()..], " severity") {
                return true;
            }
        }
        if sw(t, "actionable task") {
            return true;
        }
    }
    if r.starts_with('=') {
        let rest = r.trim_start_matches('=');
        if rest.len() < r.len() && rest.starts_with(' ') {
            return runs(rest).any(|(a, b)| word_in(&rest[a..b], &["passed", "failed", "error", "errors"]));
        }
    }
    false
}

fn hex_run(w: &str) -> bool {
    w.len() >= 8 && w.bytes().all(|b| b.is_ascii_hexdigit())
}

pub(crate) fn numbers(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < s.len() {
        let c = s[i..].chars().next().unwrap_or(' ');
        if !word(c) {
            out.push(c);
            i += c.len_utf8();
            continue;
        }
        let end = run_end(s, i);
        let run = &s[i..end];
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
    out
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

#[cfg(test)]
pub(crate) fn hot(text: &str) -> bool {
    kind(text).hot
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::oracle as old;
    use proptest::prelude::*;

    const PIECES: &[&str] = &[
        "error",
        "Error",
        "ERROR",
        "errors",
        "0",
        "00",
        "no",
        "No",
        "zero",
        "without",
        "warning",
        "warnings",
        "failed",
        "failures",
        "fail",
        "fail-fast",
        "--no-fail-fast",
        "-fail-fast",
        "no-fail",
        "fail_fast",
        "issues",
        "problem",
        "vulnerabilities",
        "none",
        "null",
        "false",
        "[]",
        "{}",
        ":",
        "=",
        " ",
        "  ",
        "\t",
        "-",
        "_",
        "err_",
        "_error",
        "TimeoutError",
        "IOError",
        "xError",
        "Exceptions",
        "NullPointerException",
        "can't",
        "can",
        "'t",
        "not found",
        "not",
        "found",
        "no such",
        "timed out",
        "timed",
        "out",
        "denied",
        "x",
        "foo",
        "12",
        "0x1F",
        "deadbeef",
        "deadbeef12",
        "abcdef1",
        "[",
        "]",
        "[12:00:01 INFO]:",
        "2026-10-30T04:00:00.819Z",
        "INFO",
        "\u{2800}",
        "███",
        "█████",
        "━━━━━",
        "[=====>   ]",
        "[##]",
        "<=====>",
        "<---->",
        "Progress (3):",
        "Downloading from x:",
        "Download",
        "https://x",
        "http://x",
        " from ",
        "npm http",
        "npm ",
        "timing",
        "at ",
        "File \"",
        "...",
        "more",
        "common frames",
        "^",
        "3:",
        "/a/b.go:12",
        "java.lang.",
        "org.bukkit.x",
        "std::",
        "tokio::",
        "mystd::",
        "node:internal",
        "test result:",
        "Tests run:",
        "5 passed",
        "5 tests",
        "ran 3 tests",
        "Finished",
        "Done (",
        "done in",
        "=====",
        "added 3 packages",
        "found 2 vulnerabilities",
        "1 high severity",
        "2 actionable tasks",
        "Total time:",
        "BUILD FAILURE",
        "\"error\"",
        "\"lastWarning\"",
        ": ",
        "\"\"",
        "\u{e9}",
        "\u{2014}",
        "\u{301}",
    ];

    fn line() -> impl Strategy<Value = String> {
        prop::collection::vec(prop::sample::select(PIECES.to_vec()), 0..9).prop_map(|v| v.concat())
    }

    #[test]
    fn known_lines() {
        assert_eq!(strip_ansi("\x1b[1;31mred\x1b[0m \x1b]0;t\x07x\x1b(B"), "red x");
        assert!(important("java.lang.NullPointerException: x"));
        assert!(important("Can't keep up"));
        assert!(important("upstream TIMED OUT"));
        assert!(!important("terrific errand"));
        assert!(trace("\tat a.b.C(C.java:1)"));
        assert!(trace("  File \"x.py\", line 1"));
        assert_eq!(body("2026-10-30T04:00:00.819Z INFO [main] [db]: query"), "query");
        assert_eq!(body("[12:00:01 INFO]: joined"), "joined");
        assert_eq!(numbers("id 0x1F deadbeef12 v1.2"), "id # # v#.#");
        assert_eq!(quiet(r#"{"error": null, "lastWarning": "", "x": 1}"#), r#"{, , "x": 1}"#);
    }

    #[test]
    fn error_words_need_to_mean_an_error() {
        for line in [
            "error[E0308]: mismatched types",
            "TimeoutError: upstream timed out",
            "IOError raised",
            "npm ERR! code ENOENT",
            "E_USER_ERROR in x.php",
            "ERR_CONNECTION_REFUSED",
            "[21:00:00 WARN]: Can't keep up!",
            "test result: FAILED. 8 passed; 2 failed; 0 ignored",
            "found 0 vulnerabilities, 3 errors",
            "Tests: 1 failed, 0 skipped",
        ] {
            assert!(hot(line), "{line}");
        }
        for line in [
            "   Compiling thiserror v2.0.12",
            "fn onError(cb) {",
            "test result: ok. 214 passed; 0 failed; 0 ignored",
            "Compiled with 0 errors and 0 warnings",
            "no errors found",
            "warnings: 0",
            "cargo test --no-fail-fast",
            "errors: none",
        ] {
            assert!(!hot(line), "{line}");
        }
    }

    #[test]
    fn progress_and_download_lines_are_noise() {
        for line in [
            "    Building [=====>          ] 147/312: serde",
            "⠙ reify:lodash: timing reifyNode Completed in 12ms",
            "Progress (1): 2.1 kB",
            "Downloading from central: https://repo.maven.apache.org/x.pom",
            "Downloaded from central: https://repo.maven.apache.org/x.pom (2 kB at 9 kB/s)",
            "Download https://repo.maven.apache.org/maven2/a.pom",
            "<=====--------> 42% CONFIGURING [3s]",
            "   ━━━━━━━━━━━━━━━━━━━━ 1.2/1.2 MB 3.4 MB/s eta 0:00:00",
            "npm http fetch GET 200 https://registry.npmjs.org/a 12ms",
        ] {
            assert!(kind(line).noise, "{line}");
        }
        for line in [
            "tests/test_a.py ....F.. [ 16%]",
            "Downloading failed: https://x 404 Not Found",
            "Preparing spawn area: 42%",
            "- [x] done",
        ] {
            assert!(!kind(line).noise, "{line}");
        }
    }

    #[test]
    fn summaries_and_framework_frames_are_known() {
        for line in [
            "test result: ok. 3 passed; 0 failed",
            "BUILD FAILED in 14s",
            "BUILD SUCCESS",
            "Total time:  6.412 s",
            "    Finished `test` profile [unoptimized] target(s) in 4.21s",
            "====== 2 failed, 386 passed in 12.84s ======",
            "added 1287 packages, and audited 1288 packages in 41s",
            "Tests run: 12, Failures: 0, Errors: 0, Skipped: 0",
            "Done (12.481s)! For help, type \"help\"",
        ] {
            assert!(summary(line), "{line}");
        }
        assert!(!summary("step 3 done"));
        assert!(runtime("\tat java.base/java.lang.Thread.run(Thread.java:1583)"));
        assert!(runtime("\tat org.bukkit.plugin.java.JavaPluginLoader.x(J.java:1) ~[paper.jar:?]"));
        assert!(runtime("    at Module._compile (node:internal/modules/cjs/loader:1105:14)"));
        assert!(runtime("   3: std::panicking::begin_panic"));
        assert!(!runtime("\tat dev.qdhshamiro.shop.ShopGui.open(ShopGui.java:57)"));
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(3000))]

        #[test]
        fn same_as_the_regexes(raw in line(), esc in "[\\x1b\\[\\];0-9 m(B\\x07a-z\\\\]{0,14}") {
            let (hot, noisy) = old::kind(&raw);
            let k = kind(&raw);
            prop_assert_eq!((k.hot, k.noise), (hot, noisy), "{:?}", raw);
            let pre = old::PREFIX.replace(&raw, "");
            prop_assert_eq!(body(&raw), pre.as_ref(), "{:?}", raw);
            prop_assert_eq!(trace(&raw), old::TRACE.is_match(&raw), "{:?}", raw);
            prop_assert_eq!(runtime(&raw), old::RUNTIME.is_match(&raw), "{:?}", raw);
            prop_assert_eq!(summary(&raw), old::SUMMARY.is_match(&raw), "{:?}", raw);
            let nums = old::NUMS.replace_all(&raw, "#");
            prop_assert_eq!(numbers(&raw), nums.as_ref(), "{:?}", raw);
            let quieted = old::QUIET.replace_all(&raw, "");
            let q = quiet(&raw);
            prop_assert_eq!(q.as_ref(), quieted.as_ref(), "{:?}", raw);
            let mixed = format!("{raw}{esc}{raw}");
            let plain = old::ANSI.replace_all(&mixed, "");
            let sa = strip_ansi(&mixed);
            prop_assert_eq!(sa.as_ref(), plain.as_ref(), "{:?}", mixed);
            prop_assert_eq!(important(&raw), old::IMPORTANT.is_match(&raw), "{:?}", raw);
        }
    }
}
