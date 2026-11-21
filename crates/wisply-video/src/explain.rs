use std::sync::LazyLock;

use regex::Regex;

use crate::fail::{Fail, Kind};
use crate::redact;

const ERROR_PREFIX: &str = "ERROR:";
const WARNING_PREFIX: &str = "WARNING:";
const CURLY_APOSTROPHE: char = '\u{2019}';
const ERROR_TEXT_CHARS: usize = 300;
const NO_TEXT: &str = "no error text";
const UPDATE_HINT: &str = "YouTube changes often: update yt-dlp (yt-dlp -U) and try again";
const PUBLIC_ONLY: &str = "only public videos can be watched";
const WAIT: &str = "wait a few minutes and try again";

struct Known {
    markers: &'static [&'static str],
    kind: Kind,
    message: &'static str,
    hint: &'static str,
}

const KNOWN: [Known; 9] = [
    Known { markers: &["private video"], kind: Kind::Unavailable, message: "this video is private", hint: PUBLIC_ONLY },
    Known {
        markers: &["confirm your age", "age-restricted", "inappropriate for some users"],
        kind: Kind::Unavailable,
        message: "this video is age-restricted",
        hint: "wisply never signs in to YouTube, so it cannot open it",
    },
    Known {
        markers: &["members-only", "channel's members", "join this channel"],
        kind: Kind::Unavailable,
        message: "this video is only for channel members",
        hint: PUBLIC_ONLY,
    },
    Known {
        markers: &["available in your country", "blocked it in your country"],
        kind: Kind::Unavailable,
        message: "this video is blocked in this country",
        hint: "only videos that play here can be watched",
    },
    Known {
        markers: &["live event will begin", "premiere will begin", "premieres in"],
        kind: Kind::Unavailable,
        message: "this live stream or premiere has not started yet",
        hint: "try again after it started",
    },
    Known {
        markers: &["not a bot"],
        kind: Kind::Service,
        message: "YouTube asks for a sign-in to prove this is not a bot",
        hint: "wait a few minutes and try again; wisply never signs in to YouTube",
    },
    Known {
        markers: &["http error 429", "too many requests"],
        kind: Kind::Service,
        message: "YouTube is limiting requests from this PC",
        hint: WAIT,
    },
    Known {
        markers: &[
            "video unavailable",
            "video is unavailable",
            "video is not available",
            "has been removed",
            "no longer available",
            "has been terminated",
        ],
        kind: Kind::Unavailable,
        message: "YouTube says this video is unavailable (removed, private or blocked)",
        hint: "check the link in a browser; if it plays there, wait a few minutes and try again",
    },
    Known {
        markers: &[
            "getaddrinfo",
            "name resolution",
            "failed to resolve",
            "timed out",
            "network is unreachable",
            "connection",
            "unable to download api page",
        ],
        kind: Kind::Service,
        message: "YouTube could not be reached",
        hint: "check the internet connection and try again",
    },
];

static BRACKETS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:\[[^\]]+\]\s*)+(?:[A-Za-z0-9_-]{11}:\s*)?").expect("brackets"));

fn known(line: &str) -> Option<&'static Known> {
    let text = line.replace(CURLY_APOSTROPHE, "'").to_lowercase();
    KNOWN.iter().find(|known| known.markers.iter().any(|marker| text.contains(marker)))
}

pub fn tidy_line(line: &str) -> String {
    let bare = [ERROR_PREFIX, WARNING_PREFIX].iter().find_map(|prefix| line.strip_prefix(prefix)).unwrap_or(line);
    let shown = redact::hide_links(&BRACKETS.replace(bare.trim(), ""));
    shown.chars().take(ERROR_TEXT_CHARS).collect()
}

pub fn failure(stderr: &str) -> Fail {
    let lines: Vec<&str> = stderr.lines().map(str::trim).filter(|line| !line.is_empty()).collect();
    let errors: Vec<&str> = lines.iter().copied().filter(|line| line.starts_with(ERROR_PREFIX)).collect();
    let warnings = lines.iter().copied().filter(|line| line.starts_with(WARNING_PREFIX));
    if let Some(known) = errors.iter().rev().copied().chain(warnings.rev()).find_map(known) {
        return Fail::hint(known.kind, known.message, known.hint);
    }
    let shown = errors.last().or(lines.last()).map_or_else(|| NO_TEXT.to_string(), |line| tidy_line(line));
    Fail::hint(Kind::Service, format!("yt-dlp failed: {shown}"), UPDATE_HINT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_yt_dlp_failures_get_plain_messages() {
        let cases = [
            ("ERROR: [youtube] abcdefghijk: Private video. Sign in", Kind::Unavailable, "private"),
            ("ERROR: [youtube] abcdefghijk: Sign in to confirm your age.", Kind::Unavailable, "age-restricted"),
            (
                "ERROR: [youtube] abcdefghijk: Join this channel to get access to members-only content",
                Kind::Unavailable,
                "channel members",
            ),
            (
                "ERROR: [youtube] abcdefghijk: This video is available to this channel\u{2019}s members on level 2",
                Kind::Unavailable,
                "channel members",
            ),
            (
                "ERROR: [youtube] abcdefghijk: The uploader has not made this video available in your country",
                Kind::Unavailable,
                "blocked in this country",
            ),
            ("ERROR: [youtube] abcdefghijk: This live event will begin in 3 hours.", Kind::Unavailable, "not started"),
            ("ERROR: [youtube] abcdefghijk: Sign in to confirm you\u{2019}re not a bot.", Kind::Service, "not a bot"),
            ("ERROR: unable to download: HTTP Error 429: Too Many Requests", Kind::Service, "limiting"),
            ("ERROR: [youtube] abcdefghijk: Video unavailable", Kind::Unavailable, "unavailable"),
            (
                "WARNING: [youtube] This video is unavailable\nWARNING: No video formats found!",
                Kind::Unavailable,
                "unavailable",
            ),
            (
                "ERROR: [youtube] abcdefghijk: Unable to download API page: getaddrinfo failed",
                Kind::Service,
                "could not be reached",
            ),
        ];
        for (stderr, kind, words) in cases {
            let fail = failure(stderr);
            assert_eq!(fail.kind, kind, "{stderr}");
            assert!(fail.message.contains(words), "{stderr}: {}", fail.message);
            assert!(!fail.hint.is_empty(), "{stderr}");
        }
    }

    #[test]
    fn the_last_error_decides_before_any_warning() {
        let stderr =
            "WARNING: [youtube] HTTP Error 429: Too Many Requests\nERROR: [youtube] abcdefghijk: Private video";
        assert_eq!(failure(stderr).message, "this video is private");
    }

    #[test]
    fn an_unknown_failure_shows_the_last_error_without_links() {
        let stderr =
            "WARNING: slow\nERROR: [youtube] abcdefghijk: Strange thing at https://media.example.test/v?ip=1\n";
        let fail = failure(stderr);
        assert_eq!(fail.kind, Kind::Service);
        assert_eq!(fail.message, "yt-dlp failed: Strange thing at <link>");
        assert!(fail.hint.contains("update yt-dlp"), "{}", fail.hint);
        assert_eq!(failure("something odd\n").message, "yt-dlp failed: something odd");
    }

    #[test]
    fn an_empty_failure_text_is_still_explained() {
        assert_eq!(failure("").message, "yt-dlp failed: no error text");
        assert_eq!(failure(" \n\n").message, "yt-dlp failed: no error text");
    }

    #[test]
    fn long_error_lines_are_cut() {
        let line = format!("ERROR: [youtube] [info] {}", "x".repeat(900));
        assert_eq!(tidy_line(&line).chars().count(), ERROR_TEXT_CHARS);
    }
}
