use crate::pat;
use crate::text::{len, strip};
use std::borrow::Cow;
use std::collections::HashSet;

const DUP_MIN: usize = 20;
const BOILER_MAX: usize = 100;
const BOILER: [&str; 26] = [
    "skip to",
    "accept all",
    "accept cookies",
    "we use cookies",
    "this site uses cookies",
    "cookie settings",
    "cookie preferences",
    "manage cookies",
    "subscribe to our",
    "subscribe now",
    "sign up for",
    "sign in",
    "log in",
    "privacy policy",
    "terms of service",
    "terms of use",
    "all rights reserved",
    "copyright",
    "share on",
    "follow us",
    "advertisement",
    "related articles",
    "back to top",
    "toggle navigation",
    "open menu",
    "close menu",
];

fn boiler(line: &str) -> bool {
    let t = strip(line).trim_start_matches(['*', '#', '-', '>', '[', '|', ' ']);
    if len(t) > BOILER_MAX {
        return false;
    }
    let low = t.to_lowercase();
    low.starts_with('\u{a9}') || BOILER.iter().any(|b| low.starts_with(b)) && !pat::kind(line).hot
}

fn tracking(line: &str) -> Cow<'_, str> {
    if !line.contains("utm_") {
        return Cow::Borrowed(line);
    }
    let mut out = String::with_capacity(line.len());
    let mut rest = line;
    while let Some(at) = rest.find("utm_") {
        let lead = rest.as_bytes().get(at.wrapping_sub(1)).copied();
        let end = rest[at..].find(['&', ')', ']', ' ', '"', '>', '\t']).map_or(rest.len(), |n| at + n);
        if !matches!(lead, Some(b'?' | b'&')) {
            out.push_str(&rest[..end]);
            rest = &rest[end..];
            continue;
        }
        let amp = rest.as_bytes().get(end) == Some(&b'&');
        if lead == Some(b'?') && amp {
            out.push_str(&rest[..at]);
            rest = &rest[end + 1..];
        } else {
            out.push_str(&rest[..at - 1]);
            rest = &rest[end..];
        }
    }
    out.push_str(rest);
    Cow::Owned(out)
}

pub(crate) fn web<S: AsRef<str>>(lines: &[S]) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut seen: HashSet<String> = HashSet::new();
    for raw in lines {
        let line = tracking(raw.as_ref());
        let t = strip(&line);
        if t.is_empty() {
            if out.last().is_some_and(|l| !l.is_empty()) {
                out.push(String::new());
            }
            continue;
        }
        if boiler(&line) {
            continue;
        }
        if len(t) >= DUP_MIN && !seen.insert(t.to_string()) {
            continue;
        }
        out.push(line.into_owned());
    }
    while out.last().is_some_and(|l| l.is_empty()) {
        out.pop();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boilerplate_and_repeats_go_and_content_stays() {
        let raw = [
            "Skip to main content",
            "# Release notes",
            "",
            "",
            "",
            "Version 4.2 adds the new export command and fixes the crash on startup.",
            "We use cookies to improve your experience",
            "Version 4.2 adds the new export command and fixes the crash on startup.",
            "Sign in",
            "See the section on errors: Error E0042 means the file is missing.",
            "© 2026 Example Inc.",
        ];
        assert_eq!(
            web(&raw),
            [
                "# Release notes",
                "",
                "Version 4.2 adds the new export command and fixes the crash on startup.",
                "See the section on errors: Error E0042 means the file is missing.",
            ]
        );
    }

    #[test]
    fn long_sentences_that_start_like_boilerplate_stay() {
        let line = format!("Sign in with a token: {}", "the token is created in the settings page and ".repeat(3));
        assert_eq!(web(&[line.as_str()]), [line.as_str()]);
    }

    #[test]
    fn tracking_parameters_are_removed() {
        assert_eq!(tracking("https://x.io/a?utm_source=n&id=5"), "https://x.io/a?id=5");
        assert_eq!(tracking("[a](https://x.io/a?id=5&utm_medium=x&utm_term=y) ok"), "[a](https://x.io/a?id=5) ok");
        assert_eq!(tracking("https://x.io/a?utm_source=n next"), "https://x.io/a next");
        assert_eq!(tracking("plain utm_x text"), "plain utm_x text");
    }
}
