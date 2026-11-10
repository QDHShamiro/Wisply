use super::{digits, runs, skip_ws, sw, sw_word, word, word_in};

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
