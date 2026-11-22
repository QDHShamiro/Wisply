use std::collections::HashMap;
use std::sync::LazyLock;

use regex::Regex;

const EXCERPT_LINES: usize = 40;
const TITLE_CHARS: usize = 160;
const PRINT_CHARS: usize = 200;
const MAX_HITS: usize = 200;
const PREFIX_CHARS: usize = 90;

static LEVEL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:^|[\[\s/|])(WARN|WARNING|ERROR|SEVERE|FATAL)(?:\]|:|\s|\||$)").expect("level"));
static INFO: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?:^|[\[\s/])(INFO|DEBUG|TRACE)(?:\]|:|\s)").expect("info"));
static EXCEPTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(?:Caused by: )?([\w$]+(?:\.[\w$]+)+(?:Exception|Error|Throwable))\b").expect("exception")
});
static STARTED: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Done \(\d+[.,]?\d*s\)!|Listening on /|Started .* in \d").expect("started"));
static UUID: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}").expect("uuid")
});
static HEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b(?:0x)?[0-9a-fA-F]*\d[0-9a-fA-F]{7,}\b").expect("hex"));
static NUMBER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"-?\d+(?:[.,]\d+)?").expect("number"));
static PLAYER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[A-Za-z0-9_.*]{2,17} (moved (?:too quickly|wrongly)|was kicked)").expect("player"));
static SPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").expect("space"));
const CRASH_WORDS: [&str; 4] = [
    "exception in server tick loop",
    "server has stopped responding",
    "encountered an unexpected exception",
    "outofmemoryerror",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hit {
    pub fingerprint: String,
    pub title: String,
    pub level: &'static str,
    pub excerpt: String,
    pub count: i64,
}

#[derive(Debug, Default)]
pub struct Scan {
    pub hits: Vec<Hit>,
    pub started: bool,
}

struct Open {
    level: &'static str,
    message: String,
    exception: String,
    frame: String,
    lines: Vec<String>,
}

fn level_of(word: &str, message: &str) -> &'static str {
    let lower = message.to_lowercase();
    if word == "FATAL" || CRASH_WORDS.iter().any(|w| lower.contains(w)) {
        return "crash";
    }
    if matches!(word, "ERROR" | "SEVERE") { "error" } else { "warn" }
}

fn prefix_end(line: &str) -> usize {
    line.char_indices().nth(PREFIX_CHARS).map_or(line.len(), |(i, _)| i)
}

fn leveled(line: &str) -> Option<(&'static str, &str)> {
    let head = &line[..prefix_end(line)];
    let m = LEVEL.captures(head)?;
    let word = m.get(1)?;
    if INFO.find(head).is_some_and(|i| i.start() < word.start()) {
        return None;
    }
    let mut message = line[word.end()..].trim_start_matches([']', ':', '|']).trim_start_matches(' ');
    if message.starts_with('[')
        && let Some(i) = message.find("]: ").filter(|i| *i < 60)
    {
        message = &message[i + 3..];
    }
    let word = if word.as_str() == "WARNING" { "WARN" } else { word.as_str() };
    Some((level_of(word, message), message.trim_end()))
}

fn quiet(line: &str) -> bool {
    INFO.is_match(&line[..prefix_end(line)])
}

fn continuation(message: &str) -> bool {
    let t = message.trim_start();
    t.starts_with("at ")
        || t.starts_with("Caused by:")
        || t.starts_with("... ")
        || t.starts_with("Suppressed:")
        || (message.starts_with([' ', '\t']) && !t.is_empty())
        || EXCEPTION.is_match(t)
}

pub fn normalize(text: &str) -> String {
    let t = UUID.replace_all(text, "<id>");
    let t = HEX.replace_all(&t, "<hex>");
    let t = PLAYER.replace(&t, "<player> $1");
    let t = NUMBER.replace_all(&t, "#");
    let t = SPACE.replace_all(t.trim(), " ");
    t.to_lowercase().chars().take(PRINT_CHARS).collect()
}

fn fnv(text: &str) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in text.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{h:016x}")
}

pub fn fingerprint(message: &str, exception: &str, frame: &str) -> String {
    fnv(&format!("{}|{}|{}", normalize(message), exception, normalize(frame)))
}

fn title(message: &str, exception: &str) -> String {
    let message = SPACE.replace_all(message.trim(), " ");
    let simple = exception.rsplit('.').next().unwrap_or(exception);
    let text = if message.is_empty() {
        exception.to_string()
    } else if simple.is_empty() || message.contains(simple) {
        message.into_owned()
    } else {
        format!("{message} ({simple})")
    };
    text.chars().take(TITLE_CHARS).collect()
}

impl Open {
    fn new(level: &'static str, message: &str, line: &str) -> Self {
        let mut o = Open {
            level,
            message: message.to_string(),
            exception: String::new(),
            frame: String::new(),
            lines: Vec::new(),
        };
        o.take(message, line);
        o
    }

    fn take(&mut self, message: &str, line: &str) {
        let t = message.trim_start();
        if self.exception.is_empty()
            && let Some(c) = EXCEPTION.captures(t)
        {
            self.exception = c[1].to_string();
            if self.level != "crash" && CRASH_WORDS.iter().any(|w| t.to_lowercase().contains(w)) {
                self.level = "crash";
            }
        }
        if self.frame.is_empty()
            && let Some(f) = t.strip_prefix("at ")
        {
            self.frame = f.split('(').next().unwrap_or(f).to_string();
        }
        if self.lines.len() < EXCERPT_LINES {
            self.lines.push(line.trim_end().to_string());
        }
    }

    fn close(self) -> Hit {
        let message = if self.message.trim().is_empty() { self.exception.clone() } else { self.message.clone() };
        Hit {
            fingerprint: fingerprint(&message, &self.exception, &self.frame),
            title: title(&message, &self.exception),
            level: self.level,
            excerpt: self.lines.join("\n"),
            count: 1,
        }
    }
}

fn keep(hits: &mut Vec<Hit>, index: &mut HashMap<String, usize>, hit: Hit) {
    if let Some(&i) = index.get(&hit.fingerprint) {
        hits[i].count += 1;
        return;
    }
    if hits.len() >= MAX_HITS {
        return;
    }
    index.insert(hit.fingerprint.clone(), hits.len());
    hits.push(hit);
}

pub fn scan(text: &str) -> Scan {
    let mut out = Scan::default();
    let mut index = HashMap::new();
    let mut open: Option<Open> = None;
    for line in text.lines() {
        if !out.started && STARTED.is_match(line) {
            out.started = true;
        }
        match leveled(line) {
            Some((_, message)) if continuation(message) && open.is_some() => {
                if let Some(o) = open.as_mut() {
                    o.take(message, line);
                }
            }
            Some((level, message)) => {
                if let Some(o) = open.take() {
                    keep(&mut out.hits, &mut index, o.close());
                }
                open = Some(Open::new(level, message, line));
            }
            None if !quiet(line) && continuation(line) && open.is_some() => {
                if let Some(o) = open.as_mut() {
                    o.take(line, line);
                }
            }
            None => {
                if let Some(o) = open.take() {
                    keep(&mut out.hits, &mut index, o.close());
                }
            }
        }
    }
    if let Some(o) = open.take() {
        keep(&mut out.hits, &mut index, o.close());
    }
    out
}

pub fn crash(name: &str, text: &str) -> Hit {
    let lines: Vec<&str> = text.lines().collect();
    let at = lines.iter().position(|l| l.starts_with("Description:")).unwrap_or(0);
    let description = lines.get(at).and_then(|l| l.strip_prefix("Description:")).map_or(name, str::trim);
    let mut o = Open::new("crash", description, lines.get(at).copied().unwrap_or(name));
    for line in lines.iter().skip(at + 1) {
        if line.starts_with("A detailed walkthrough") || line.starts_with("-- ") {
            break;
        }
        o.take(line, line);
    }
    let mut hit = o.close();
    hit.title = title(&format!("Crash: {description}"), &hit_exception(&hit.excerpt));
    hit
}

fn hit_exception(excerpt: &str) -> String {
    excerpt.lines().find_map(|l| EXCEPTION.captures(l.trim_start()).map(|c| c[1].to_string())).unwrap_or_default()
}
