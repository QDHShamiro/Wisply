use std::path::PathBuf;
use std::sync::LazyLock;

use regex::Regex;

use crate::fail::{Fail, Kind};
use crate::timecode;

const SHORT_HOST: &str = "youtu.be";
const DOMAINS: [&str; 2] = ["youtube.com", "youtube-nocookie.com"];
const CHANNEL_PATHS: [&str; 3] = ["channel", "c", "user"];
const NOT_VIDEO_IDS: [&str; 2] = ["videoseries", "live_stream"];
const START_NAMES: [&str; 3] = ["t", "start", "time_continue"];
const ECHO_LIMIT: usize = 80;
const WRAPPERS: [(char, char); 4] = [('"', '"'), ('\'', '\''), ('<', '>'), ('`', '`')];
const LINK_TAIL: &str = ".,;)]";
const LOCAL_HOST: &str = "localhost";
const WATCH_LINK: &str = "https://www.youtube.com/watch?v=";
const LINK_HINT: &str = "give a link like https://www.youtube.com/watch?v=VIDEO_ID";
const SOURCE_HINT: &str = "give a link like https://www.youtube.com/watch?v=VIDEO_ID or the full path of a video file";

static VIDEO_ID: LazyLock<Regex> = LazyLock::new(|| Regex::new("^[A-Za-z0-9_-]{11}$").expect("video id"));
static SCHEME: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([A-Za-z][A-Za-z0-9+.\-]+)://").expect("scheme"));
static BARE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^(?:[a-z0-9-]+\.)*(?:youtube\.com|youtube-nocookie\.com|youtu\.be)(?:[/?#]|$)").expect("bare")
});
static MSYS_DRIVE: LazyLock<Regex> = LazyLock::new(|| Regex::new("^/([A-Za-z])/(.*)$").expect("msys drive"));
static URI_DRIVE: LazyLock<Regex> = LazyLock::new(|| Regex::new("^/[A-Za-z]:").expect("uri drive"));
static WINDOWS_ROOT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:[A-Za-z]:[\\/]|[\\/]{2}[^\\/]+[\\/]+[^\\/]+)").expect("windows root"));

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    Watch,
    Short,
    Live,
    Embed,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    YouTube { id: String, form: Form, start_hint: Option<f64> },
    Page(String),
    Local(PathBuf),
}

impl Source {
    pub fn url(&self) -> Option<String> {
        match self {
            Source::YouTube { id, .. } => Some(format!("{WATCH_LINK}{id}")),
            Source::Page(link) => Some(link.clone()),
            Source::Local(_) => None,
        }
    }
}

struct Parts<'a> {
    authority: &'a str,
    path: &'a str,
    query: &'a str,
    fragment: &'a str,
}

type Pairs = Vec<(String, String)>;

fn refuse(message: String, hint: impl Into<String>) -> Fail {
    Fail::hint(Kind::Source, message, hint)
}

fn shorten(text: &str) -> String {
    match text.char_indices().nth(ECHO_LIMIT) {
        Some((cut, _)) => format!("{}...", &text[..cut]),
        None => text.to_string(),
    }
}

fn unwrap(text: &str) -> &str {
    let mut cleaned = text.trim();
    for (opening, closing) in WRAPPERS {
        if cleaned.len() >= 2 && cleaned.starts_with(opening) && cleaned.ends_with(closing) {
            cleaned = cleaned[1..cleaned.len() - 1].trim();
        }
    }
    cleaned
}

fn split(rest: &str) -> Parts<'_> {
    let (before, fragment) = rest.split_once('#').unwrap_or((rest, ""));
    let (before, query) = before.split_once('?').unwrap_or((before, ""));
    let (authority, path) = before.split_at(before.find('/').unwrap_or(before.len()));
    Parts { authority, path, query, fragment }
}

fn host(authority: &str) -> Option<String> {
    let named = authority.rsplit('@').next().unwrap_or_default();
    let (name, port) = match named.strip_prefix('[') {
        Some(inner) => {
            let (address, after) = inner.split_once(']')?;
            (address, after.strip_prefix(':').unwrap_or(after))
        }
        None => named.split_once(':').unwrap_or((named, "")),
    };
    let sound = !name.is_empty() && !name.contains(char::is_whitespace) && port.bytes().all(|b| b.is_ascii_digit());
    sound.then(|| name.to_lowercase().trim_end_matches('.').to_string())
}

fn decode(text: &str, plus: bool) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut at = 0;
    while at < bytes.len() {
        let pair = text.get(at + 1..at + 3).filter(|pair| pair.bytes().all(|b| b.is_ascii_hexdigit()));
        match (bytes[at], pair.and_then(|pair| u8::from_str_radix(pair, 16).ok())) {
            (b'%', Some(byte)) => {
                out.push(byte);
                at += 3;
            }
            (b'+', _) if plus => {
                out.push(b' ');
                at += 1;
            }
            (byte, _) => {
                out.push(byte);
                at += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn pairs(query: &str) -> Pairs {
    let filled = query.split('&').filter_map(|pair| pair.split_once('='));
    filled
        .map(|(name, value)| (decode(name, true), decode(value, true)))
        .filter(|(_, value)| !value.is_empty())
        .collect()
}

fn first<'a>(pairs: &'a Pairs, name: &str) -> Option<&'a str> {
    pairs.iter().find(|(key, _)| key == name).map(|(_, value)| value.as_str())
}

fn is_youtube(host: &str) -> bool {
    host == SHORT_HOST || DOMAINS.iter().any(|domain| host == *domain || host.ends_with(&format!(".{domain}")))
}

fn form_of(segment: &str) -> Option<Form> {
    match segment {
        "shorts" => Some(Form::Short),
        "live" => Some(Form::Live),
        "embed" | "e" | "v" => Some(Form::Embed),
        _ => None,
    }
}

fn candidate<'a>(host: &str, segments: &[&'a str], query: &'a Pairs) -> (Option<&'a str>, Form) {
    if host == SHORT_HOST {
        return (segments.first().copied(), Form::Watch);
    }
    let lead = segments.first().map(|segment| segment.to_lowercase()).unwrap_or_default();
    match (form_of(&lead), segments.get(1).copied()) {
        (Some(form), Some(id)) => (Some(id), form),
        (None, Some(id)) if lead == "watch" => (Some(id), Form::Watch),
        _ => (first(query, "v"), Form::Watch),
    }
}

fn refuse_link(text: &str, segments: &[&str], query: &Pairs, candidate: Option<&str>) -> Fail {
    let lead = segments.first().map(|segment| segment.to_lowercase()).unwrap_or_default();
    if first(query, "list").is_some() || lead == "playlist" {
        let message = "this link points to a playlist, not to one video".to_string();
        return refuse(message, "give the link of a single video from the playlist");
    }
    if lead.starts_with('@') || CHANNEL_PATHS.contains(&lead.as_str()) {
        return refuse("this link points to a channel, not to one video".to_string(), LINK_HINT);
    }
    if lead == "clip" {
        let message = "clip links do not carry the video ID".to_string();
        return refuse(message, "open the clip, then give the link of the full video");
    }
    match candidate {
        Some(id) => {
            refuse(format!("'{}' is not a YouTube video ID (11 letters, digits, - or _)", shorten(id)), LINK_HINT)
        }
        None => refuse(format!("no video ID found in '{}'", shorten(text)), LINK_HINT),
    }
}

fn start_hint(query: &Pairs, fragment: &str) -> Option<f64> {
    let fragment = pairs(fragment);
    let named = START_NAMES.iter().filter_map(|name| first(query, name));
    named.chain(first(&fragment, "t")).find_map(|value| timecode::parse(value).ok())
}

fn youtube(text: &str, host: &str, parts: &Parts, follow: bool) -> Result<Source, Fail> {
    let query = pairs(parts.query);
    let segments: Vec<&str> = parts.path.split('/').filter(|segment| !segment.is_empty()).collect();
    if follow
        && segments.first() == Some(&"attribution_link")
        && let Some(inner) = first(&query, "u")
    {
        return youtube(text, host, &split(&format!("{host}{inner}")), false);
    }
    let (found, form) = candidate(host, &segments, &query);
    match found {
        Some(id) if VIDEO_ID.is_match(id) && !NOT_VIDEO_IDS.contains(&id) => {
            Ok(Source::YouTube { id: id.to_string(), form, start_hint: start_hint(&query, parts.fragment) })
        }
        other => Err(refuse_link(text, &segments, &query, other)),
    }
}

fn web(text: &str, scheme: &str, rest: &str) -> Result<Source, Fail> {
    let rest = rest.trim_end_matches(|c| LINK_TAIL.contains(c));
    let parts = split(rest);
    let Some(host) = host(parts.authority) else {
        return Err(refuse(format!("'{}' is not a valid link", shorten(text)), SOURCE_HINT));
    };
    if is_youtube(&host) { youtube(text, &host, &parts, true) } else { Ok(Source::Page(format!("{scheme}://{rest}"))) }
}

fn file_path(parts: &Parts) -> String {
    let path = decode(parts.path, false);
    if !parts.authority.is_empty() && !parts.authority.eq_ignore_ascii_case(LOCAL_HOST) {
        return format!("//{}{path}", parts.authority);
    }
    if URI_DRIVE.is_match(&path) { path[1..].to_string() } else { path }
}

fn home() -> Option<String> {
    let names = if cfg!(windows) { ["USERPROFILE", "HOME"] } else { ["HOME", "USERPROFILE"] };
    names.iter().find_map(|name| std::env::var(name).ok().filter(|value| !value.is_empty()))
}

fn expand(text: &str) -> String {
    match (text.strip_prefix('~'), home()) {
        (Some(rest), Some(home)) if rest.is_empty() || rest.starts_with(['/', '\\']) => format!("{home}{rest}"),
        _ => text.to_string(),
    }
}

fn local(text: &str, shown: &str, windows: bool) -> Result<Source, Fail> {
    let path = expand(text);
    let drive = MSYS_DRIVE.captures(&path).filter(|_| windows);
    let path = drive.map(|found| format!("{}:/{}", found[1].to_uppercase(), &found[2])).unwrap_or(path);
    let absolute = if windows { WINDOWS_ROOT.is_match(&path) } else { path.starts_with('/') };
    if absolute {
        return Ok(Source::Local(PathBuf::from(path)));
    }
    let message = format!("'{}' is neither a link nor the full path of a file", shorten(shown));
    if VIDEO_ID.is_match(text) {
        return Err(refuse(message, format!("if this is a video ID, give {WATCH_LINK}{text}")));
    }
    Err(refuse(message, SOURCE_HINT))
}

pub fn parse_on(value: &str, windows: bool) -> Result<Source, Fail> {
    let cleaned = unwrap(value);
    if cleaned.is_empty() {
        return Err(refuse("no video was given".to_string(), SOURCE_HINT));
    }
    if cleaned.chars().any(char::is_control) {
        return Err(refuse("the video link or path contains control characters".to_string(), SOURCE_HINT));
    }
    let text = if BARE.is_match(cleaned) { format!("https://{cleaned}") } else { cleaned.to_string() };
    let Some(found) = SCHEME.captures(&text) else {
        return local(&text, &text, windows);
    };
    let rest = &text[found[0].len()..];
    match found[1].to_lowercase().as_str() {
        "file" => local(&file_path(&split(rest)), &text, windows),
        scheme @ ("http" | "https") => web(&text, scheme, rest),
        other => Err(refuse(format!("links starting with {other}:// are not supported"), SOURCE_HINT)),
    }
}

pub fn parse(value: &str) -> Result<Source, Fail> {
    parse_on(value, cfg!(windows))
}

#[cfg(test)]
#[path = "source_tests.rs"]
mod tests;
