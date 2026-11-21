use std::path::{Path, PathBuf};

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::config::{Config, Talk};
use crate::job::Segment;
use crate::timecode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Picture,
    Video,
    Sound,
    Text,
    Page,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    File(PathBuf),
    Link(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Word {
    pub raw: String,
    pub value: String,
}

impl Word {
    pub fn plain(text: &str) -> Word {
        Word { raw: text.to_string(), value: text.to_string() }
    }
}

#[derive(Debug, Clone)]
pub enum Loaded {
    Picture { name: String, jpeg: Vec<u8>, size: (u32, u32) },
    Video { name: String, path: PathBuf, about: String },
    Words { name: String, kind: Kind, text: String },
}

pub fn is_link(talk: &Talk, word: &str) -> bool {
    let low = word.to_lowercase();
    talk.links.iter().any(|prefix| low.len() > prefix.len() && low.starts_with(prefix.as_str()))
}

pub fn extension(name: &str) -> Option<String> {
    let clean = name.split(['?', '#']).next().unwrap_or(name);
    let last = clean.rsplit(['/', '\\']).next().unwrap_or(clean);
    let (stem, ext) = last.rsplit_once('.')?;
    (!stem.is_empty() && !ext.is_empty()).then(|| ext.to_lowercase())
}

pub fn kind_of(talk: &Talk, name: &str) -> Option<Kind> {
    let ext = extension(name)?;
    let has = |list: &[String]| list.iter().any(|e| e.eq_ignore_ascii_case(&ext));
    [
        (&talk.pictures, Kind::Picture),
        (&talk.videos, Kind::Video),
        (&talk.sounds, Kind::Sound),
        (&talk.pages, Kind::Page),
        (&talk.texts, Kind::Text),
    ]
    .into_iter()
    .find(|(list, _)| has(list))
    .map(|(_, kind)| kind)
}

pub fn kind_of_type(talk: &Talk, content_type: &str) -> Option<Kind> {
    let low = content_type.trim().to_lowercase();
    talk.types.iter().find(|rule| !rule.starts.is_empty() && low.starts_with(&rule.starts)).map(|rule| rule.kind)
}

pub fn host(url: &str) -> String {
    let rest = url.split_once("://").map_or(url, |(_, rest)| rest);
    let authority = rest.split(['/', '?', '#']).next().unwrap_or("");
    let host = authority.rsplit('@').next().unwrap_or(authority);
    host.split(':').next().unwrap_or(host).to_lowercase()
}

pub fn is_tube(talk: &Talk, url: &str) -> bool {
    let host = host(url);
    talk.video_hosts.iter().any(|known| host == *known || host.ends_with(&format!(".{known}")))
}

pub fn words(line: &str) -> Vec<Word> {
    let mut list = Vec::new();
    let (mut raw, mut value, mut quoted) = (String::new(), String::new(), false);
    for letter in line.chars() {
        if letter == '"' {
            quoted = !quoted;
            raw.push(letter);
        } else if letter.is_whitespace() && !quoted {
            if !raw.is_empty() {
                list.push(Word { raw: std::mem::take(&mut raw), value: std::mem::take(&mut value) });
            }
        } else {
            raw.push(letter);
            value.push(letter);
        }
    }
    if !raw.is_empty() {
        list.push(Word { raw, value });
    }
    list
}

pub fn split(talk: &Talk, words: &[Word], exists: &dyn Fn(&Path) -> bool) -> (String, Vec<Source>) {
    let mut text = Vec::new();
    let mut sources = Vec::new();
    for word in words {
        if is_link(talk, &word.value) {
            sources.push(Source::Link(word.value.clone()));
        } else if kind_of(talk, &word.value).is_some() && exists(Path::new(&word.value)) {
            sources.push(Source::File(PathBuf::from(&word.value)));
        } else {
            text.push(word.raw.as_str());
        }
    }
    (text.join(" "), sources)
}

pub fn lift(talk: &Talk, text: &str, exists: &dyn Fn(&Path) -> bool) -> (String, Vec<Source>) {
    let mut kept = String::new();
    let mut rest = text;
    let mut sources = Vec::new();
    for word in words(text) {
        let source = if is_link(talk, &word.value) {
            Source::Link(word.value.clone())
        } else if kind_of(talk, &word.value).is_some() && exists(Path::new(&word.value)) {
            Source::File(PathBuf::from(&word.value))
        } else {
            continue;
        };
        let Some(at) = rest.find(word.raw.as_str()) else {
            continue;
        };
        kept.push_str(&rest[..at]);
        rest = &rest[at + word.raw.len()..];
        sources.push(source);
    }
    kept.push_str(rest);
    (kept.trim().to_string(), sources)
}

pub fn fill(args: &[String], fields: &[(&str, &str)]) -> Vec<String> {
    args.iter()
        .map(|arg| {
            fields
                .iter()
                .find(|(name, _)| *arg == format!("{{{name}}}"))
                .map(|(_, value)| value.to_string())
                .unwrap_or_else(|| {
                    fields
                        .iter()
                        .filter(|(name, _)| *name != "url" && *name != "file")
                        .fold(arg.clone(), |text, (name, value)| text.replace(&format!("{{{name}}}"), value))
                })
        })
        .collect()
}

pub fn clip(text: &str, limit: usize, mark: &str) -> String {
    if limit == 0 || text.chars().count() <= limit {
        return text.to_string();
    }
    let mut cut: String = text.chars().take(limit).collect();
    cut.push_str(mark);
    cut
}

pub fn page_text(talk: &Talk, html: &str) -> Result<String, String> {
    let mut text = html.to_string();
    for pattern in talk.page_drop.iter().chain([&talk.page_tag]) {
        if pattern.is_empty() {
            continue;
        }
        let rule = Regex::new(pattern).map_err(|e| e.to_string())?;
        text = rule.replace_all(&text, " ").into_owned();
    }
    for pair in &talk.page_entities {
        if let [from, to] = pair.as_slice() {
            text = text.replace(from.as_str(), to);
        }
    }
    Ok(text.split_whitespace().collect::<Vec<_>>().join(" "))
}

#[derive(Debug, Clone, Deserialize)]
struct Noise {
    start: f64,
    end: f64,
    label: String,
}

#[derive(Debug, Default, Deserialize)]
struct Heard {
    #[serde(default)]
    speech: Vec<Segment>,
    #[serde(default)]
    sounds: Vec<Noise>,
}

pub fn said_plain(raw: &[u8]) -> Result<String, String> {
    let heard: Heard = serde_json::from_slice(raw).map_err(|e| e.to_string())?;
    let said: Vec<&str> = heard.speech.iter().map(|s| s.text.trim()).filter(|text| !text.is_empty()).collect();
    Ok(said.join(" "))
}

pub fn heard_text(config: &Config, raw: &[u8]) -> Result<String, String> {
    let heard: Heard = serde_json::from_slice(raw).map_err(|e| e.to_string())?;
    let said: Vec<String> = heard
        .speech
        .iter()
        .filter(|s| !s.text.trim().is_empty())
        .map(|s| {
            config.text.transcript_line.replace("{time}", &timecode::format(s.start)).replace("{text}", s.text.trim())
        })
        .collect();
    let noises: Vec<String> = heard
        .sounds
        .iter()
        .filter(|n| !n.label.trim().is_empty())
        .map(|n| {
            config
                .talk
                .text
                .sound_line
                .replace("{from}", &timecode::format(n.start))
                .replace("{to}", &timecode::format(n.end))
                .replace("{label}", n.label.trim())
        })
        .collect();
    let mut blocks = Vec::new();
    if said.is_empty() {
        blocks.push(config.talk.text.no_sound.clone());
    } else {
        blocks.push(format!("{}\n{}", config.text.transcript_header, said.join("\n")));
    }
    if !noises.is_empty() {
        blocks.push(format!("{}\n{}", config.talk.text.sound_header, noises.join("\n")));
    }
    Ok(blocks.join("\n\n"))
}

pub fn last_line(raw: &[u8]) -> String {
    String::from_utf8_lossy(raw).lines().rev().find(|line| !line.trim().is_empty()).unwrap_or("").trim().to_string()
}

pub fn safe_name(url: &str, fallback: &str) -> String {
    let clean = url.split(['?', '#']).next().unwrap_or(url);
    let last = clean.rsplit('/').next().unwrap_or("");
    let name: String =
        last.chars().filter(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_')).take(80).collect();
    if name.trim_matches('.').is_empty() || host(url) == name.to_lowercase() { fallback.to_string() } else { name }
}

#[cfg(test)]
mod tests {
    use super::{
        Kind, Source, Word, clip, extension, fill, heard_text, host, is_link, is_tube, kind_of, kind_of_type,
        page_text, safe_name, said_plain, split, words,
    };
    use crate::config::Config;
    use std::path::{Path, PathBuf};

    fn config() -> Config {
        toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses")
    }

    #[test]
    fn every_kind_is_told_by_its_ending() {
        let talk = config().talk;
        let cases = [
            ("shot.png", Some(Kind::Picture)),
            ("C:\\Bilder\\Foto.JPG", Some(Kind::Picture)),
            ("clip.mp4", Some(Kind::Video)),
            ("D:/Clips/a b/final.MKV", Some(Kind::Video)),
            ("song.mp3", Some(Kind::Sound)),
            ("memo.wav", Some(Kind::Sound)),
            ("notes.txt", Some(Kind::Text)),
            ("data.json", Some(Kind::Text)),
            ("saved.html", Some(Kind::Page)),
            ("https://x.org/a/pic.webp?size=2#top", Some(Kind::Picture)),
            ("archive.zip", None),
            ("word", None),
            (".png", None),
            ("ends.", None),
        ];
        for (name, kind) in cases {
            assert_eq!(kind_of(&talk, name), kind, "{name}");
        }
        assert_eq!(extension("a/b.c/d"), None);
    }

    #[test]
    fn links_and_video_sites_are_told_apart() {
        let talk = config().talk;
        assert!(is_link(&talk, "https://example.org/a"));
        assert!(is_link(&talk, "HTTP://example.org"));
        assert!(!is_link(&talk, "https://"));
        assert!(!is_link(&talk, "ftp://example.org/a.png"));
        assert!(!is_link(&talk, "file.png"));
        assert_eq!(host("https://user@WWW.YouTube.com:443/watch?v=1"), "www.youtube.com");
        assert!(is_tube(&talk, "https://www.youtube.com/watch?v=1"));
        assert!(is_tube(&talk, "https://youtu.be/abc"));
        assert!(!is_tube(&talk, "https://notyoutube.com/watch"));
        assert!(!is_tube(&talk, "https://example.org/youtube.com"));
    }

    #[test]
    fn the_server_type_names_the_kind() {
        let talk = config().talk;
        let cases = [
            ("image/png", Some(Kind::Picture)),
            ("Video/MP4", Some(Kind::Video)),
            ("audio/mpeg", Some(Kind::Sound)),
            ("text/html; charset=utf-8", Some(Kind::Page)),
            ("text/plain", Some(Kind::Text)),
            ("application/json", Some(Kind::Text)),
            ("application/octet-stream", None),
            ("", None),
        ];
        for (given, kind) in cases {
            assert_eq!(kind_of_type(&talk, given), kind, "{given}");
        }
    }

    #[test]
    fn a_line_is_cut_into_words_and_quoted_paths() {
        let list = words("Was ist das? \"C:\\Mein Ordner\\a b.png\"  clip.mp4");
        let values: Vec<&str> = list.iter().map(|w| w.value.as_str()).collect();
        assert_eq!(values, ["Was", "ist", "das?", "C:\\Mein Ordner\\a b.png", "clip.mp4"]);
        assert_eq!(list[3].raw, "\"C:\\Mein Ordner\\a b.png\"");
        assert!(words("   ").is_empty());
    }

    #[test]
    fn files_and_links_leave_the_text() {
        let talk = config().talk;
        let line = "Vergleiche \"C:\\Mein Ordner\\a b.png\" mit https://example.org/x und sag \"Hallo\" zu fehlt.png";
        let exists = |path: &Path| path == Path::new("C:\\Mein Ordner\\a b.png");
        let (text, sources) = split(&talk, &words(line), &exists);
        assert_eq!(text, "Vergleiche mit und sag \"Hallo\" zu fehlt.png");
        assert_eq!(
            sources,
            [
                Source::File(PathBuf::from("C:\\Mein Ordner\\a b.png")),
                Source::Link("https://example.org/x".to_string()),
            ]
        );
        let (text, sources) = split(&talk, &[Word::plain("Nur Text")], &|_| true);
        assert_eq!((text.as_str(), sources.len()), ("Nur Text", 0));
    }

    #[test]
    fn a_web_page_becomes_plain_text() {
        let talk = config().talk;
        let html = "<html><head><title>T</title><style>p{color:red}</style></head><body>\
            <script>alert('x')</script><!-- note --><h1>Gr&ouml;&szlig;e &amp;lt; Mut</h1>\
            <p>Erster   Satz.</p>\n<p>Tom &amp; Jerry</p></body></html>";
        assert_eq!(page_text(&talk, html).expect("patterns are valid"), "Größe &lt; Mut Erster Satz. Tom & Jerry");
    }

    #[test]
    fn long_text_is_cut_with_a_mark() {
        assert_eq!(clip("abcdef", 3, " …"), "abc …");
        assert_eq!(clip("äöü", 3, " …"), "äöü");
        assert_eq!(clip("abc", 0, " …"), "abc");
    }

    #[test]
    fn tool_arguments_take_values_but_never_reread_a_link() {
        let args = ["--max".to_string(), "{mb}M".to_string(), "{url}".to_string()];
        let filled = fill(&args, &[("mb", "5"), ("url", "https://x.org/{mb}")]);
        assert_eq!(filled, ["--max", "5M", "https://x.org/{mb}"]);
    }

    #[test]
    fn a_download_gets_a_harmless_file_name() {
        assert_eq!(safe_name("https://x.org/a/Bild 1.png?x=1", "f"), "Bild1.png");
        assert_eq!(safe_name("https://x.org/", "f"), "f");
        assert_eq!(safe_name("https://x.org", "f"), "f");
        assert_eq!(safe_name("https://x.org/..", "f"), "f");
    }

    #[test]
    fn dictation_keeps_only_the_spoken_words() {
        let raw = br#"{"speech":[{"start":0.0,"end":1.0,"text":" Hallo "},{"start":1.0,"end":2.0,"text":" "},{"start":2.0,"end":3.0,"text":"Welt."}],
            "sounds":[{"start":0.0,"end":3.0,"label":"Music","score":0.9}]}"#;
        assert_eq!(said_plain(raw).expect("valid json"), "Hallo Welt.");
        assert_eq!(said_plain(b"{}").expect("valid json"), "");
        assert!(said_plain(b"nope").is_err());
    }

    #[test]
    fn heard_speech_and_noises_become_a_text_block() {
        let config = config();
        let raw = br#"{"speech":[{"start":1.0,"end":2.0,"text":" Hallo Welt "},{"start":3.0,"end":4.0,"text":" "}],
            "sounds":[{"start":0.0,"end":5.0,"label":"Music","score":0.9}]}"#;
        let text = heard_text(&config, raw).expect("valid json");
        assert!(text.starts_with(&config.text.transcript_header));
        assert!(text.contains("Hallo Welt"));
        assert!(text.contains(&config.talk.text.sound_header));
        assert!(text.contains("Music"));
        let quiet = heard_text(&config, b"{}").expect("valid json");
        assert_eq!(quiet, config.talk.text.no_sound);
        assert!(heard_text(&config, b"nope").is_err());
    }
}
