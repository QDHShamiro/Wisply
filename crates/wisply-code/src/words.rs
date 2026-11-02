use std::collections::{HashMap, HashSet};

use std::sync::LazyLock;
pub use wisply_memory::near;

const BAG_WORDS: usize = 64;
const MIN_WORD: usize = 3;
const SUFFIXES: &[&str] = &["ing", "ed", "es", "ion", "ly", "s"];
const NO_DOUBLE: &[char] = &['l', 's', 'z'];
const NOISE: &str = "the and for with from this that into over under not are was were has have had but all any \
    can will use used using let mut pub self super crate impl return match some none ok err true false str string vec \
    usize isize bool char else loop while where when then than out new get set fn def var val const static void int \
    long null nil end begin as in is it of on or to by if do we our how what which does way where function funktion \
    method methode class klasse code file datei called aufgerufen passiert happens happen wird werden gets also each \
    before after please der die das den dem und oder ist ein eine nicht mit von zu wie was wo wir ich";
const GROUPS: &[&str] = &[
    "save write store persist insert add put record keep dump flush commit",
    "strip remove scrub cut sanitize redact mask erase clean trim filter",
    "delete remove forget drop erase discard purge clear",
    "secret key password passwd pwd token credential auth apikey private",
    "changed modified dirty update updated mutated touched stale mtime",
    "index refresh scan crawl reindex rescan sync walk",
    "find search lookup query locate fetch retrieve match seek",
    "list enumerate listing walk all",
    "note memo memory entry",
    "file path document",
    "start begin launch spawn run init boot open",
    "stop close shutdown kill terminate quit halt",
    "send post emit publish push dispatch",
    "read load parse receive",
    "convert transform cast format render encode decode serialize",
    "fold merge dedupe dedup combine collapse group unique duplicate twin",
    "time date clock timestamp epoch day",
    "sentence snippet excerpt line summary",
    "shorten truncate clip abbreviate ellipsis oneline",
    "error fail failure problem exception",
    "check validate verify test assert ensure",
    "config setting option preference",
    "embed vector embedding",
    "project repo root workspace folder directory dir",
    "todo task item job",
    "done complete finish finished status state",
    "mark set flag status state change",
    "word term keyword",
    "recent latest last used active current",
    "ask question prompt request query",
    "server daemon service host",
    "show print display output render",
    "home dir folder",
    "speicher save store",
    "notiz note",
    "suche search find",
    "loeschen delete remove",
    "datei file",
    "ordner folder directory",
];

const ASK_NOISE: &str = "the and for with from this that into over under not are was were has have had but all any can     will where when then than how what which does way each also before after please our der die das den dem und oder     ist ein eine nicht mit von zu wie was wo wir ich code file function funktion method methode class klasse datei";
const LINKS: &[&str] = &[
    "before", "after", "when", "while", "with", "for", "of", "from", "in", "by", "using", "that", "which", "so", "to",
];
const LINKED: f32 = 0.6;

static ASK_STOP: LazyLock<HashSet<&'static str>> = LazyLock::new(|| ASK_NOISE.split_whitespace().collect());
static STOP: LazyLock<HashSet<&'static str>> = LazyLock::new(|| NOISE.split_whitespace().collect());
static SYN: LazyLock<HashMap<String, Vec<usize>>> = LazyLock::new(|| {
    let mut map: HashMap<String, Vec<usize>> = HashMap::new();
    for (i, group) in GROUPS.iter().enumerate() {
        for w in group.split_whitespace() {
            map.entry(stem(w)).or_default().push(i);
        }
    }
    map
});
static MEMBERS: LazyLock<Vec<Vec<String>>> =
    LazyLock::new(|| GROUPS.iter().map(|g| g.split_whitespace().map(stem).collect()).collect());

fn strippable(root: &str, tail: &str) -> bool {
    let n = root.chars().count();
    n >= MIN_WORD && (tail != "ed" || n >= 4 || root.contains(['a', 'i', 'o', 'u', 'y']))
}

pub fn stem(word: &str) -> String {
    let mut w = word.to_lowercase();
    for _ in 0..2 {
        if let Some(root) = w.strip_suffix("ies")
            && root.chars().count() >= 2
        {
            w = format!("{root}y");
            continue;
        }
        let Some(root) = SUFFIXES.iter().find_map(|t| w.strip_suffix(t).filter(|r| strippable(r, t))) else { break };
        w = root.to_string();
    }
    if w.chars().count() > MIN_WORD && w.ends_with('e') {
        w.pop();
    }
    let mut tail = w.chars().rev();
    if let (Some(a), Some(b)) = (tail.next(), tail.next())
        && a == b
        && !NO_DOUBLE.contains(&a)
        && w.chars().count() > MIN_WORD
    {
        w.pop();
    }
    w
}

pub fn noise(word: &str) -> bool {
    word.chars().count() < MIN_WORD || STOP.contains(word) || wisply_memory::stop_word(word)
}

pub fn split_ident(run: &str, out: &mut Vec<String>) {
    for part in run.split('_').filter(|p| !p.is_empty()) {
        let chars: Vec<char> = part.chars().collect();
        let mut start = 0;
        for i in 1..chars.len() {
            let (prev, cur) = (chars[i - 1], chars[i]);
            let next_lower = chars.get(i + 1).is_some_and(|c| c.is_lowercase());
            let boundary = (prev.is_lowercase() && cur.is_uppercase())
                || (prev.is_alphabetic() && cur.is_ascii_digit())
                || (prev.is_ascii_digit() && cur.is_alphabetic())
                || (prev.is_uppercase() && cur.is_uppercase() && next_lower);
            if boundary {
                out.push(chars[start..i].iter().collect::<String>().to_lowercase());
                start = i;
            }
        }
        out.push(chars[start..].iter().collect::<String>().to_lowercase());
    }
}

pub fn words_of(text: &str, limit: usize) -> Vec<String> {
    let mut found = Vec::new();
    let mut seen = HashSet::new();
    let mut parts = Vec::new();
    for run in text.split(|c: char| !(c.is_alphanumeric() || c == '_')) {
        if run.is_empty() {
            continue;
        }
        parts.clear();
        split_ident(run, &mut parts);
        for w in parts.drain(..) {
            if !noise(&w) && seen.insert(w.clone()) {
                found.push(w);
                if found.len() == limit {
                    return found;
                }
            }
        }
    }
    found
}

pub fn bag(text: &str) -> String {
    words_of(text, BAG_WORDS).join(" ")
}

pub fn query_terms(text: &str) -> Vec<(String, f32)> {
    let mut seen = HashSet::new();
    let mut terms = Vec::new();
    let mut parts = Vec::new();
    let mut linked = false;
    for run in text.split(|c: char| !(c.is_alphanumeric() || c == '_')) {
        let low = run.to_lowercase();
        if LINKS.contains(&low.as_str()) {
            linked = true;
        }
        parts.clear();
        split_ident(run, &mut parts);
        for w in parts.drain(..) {
            if w.chars().count() < MIN_WORD || ASK_STOP.contains(w.as_str()) || wisply_memory::stop_word(&w) {
                continue;
            }
            let s = stem(&w);
            if seen.insert(s.clone()) {
                terms.push((s, if linked { LINKED } else { 1.0 }));
            }
        }
    }
    terms.truncate(10);
    terms
}

pub fn synonyms(term: &str) -> Vec<&'static str> {
    let Some(groups) = SYN.get(term) else { return Vec::new() };
    let mut out: Vec<&'static str> = Vec::new();
    for g in groups {
        for m in &MEMBERS[*g] {
            if m != term && !out.contains(&m.as_str()) {
                out.push(m.as_str());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifiers_split_and_stems_agree() {
        let mut out = Vec::new();
        split_ident("addNote_toHTMLPage2", &mut out);
        assert_eq!(out, ["add", "note", "to", "html", "page", "2"]);
        for (a, b) in [("saving", "saved"), ("notes", "note"), ("embedding", "embed"), ("changed", "changes")] {
            assert_eq!(stem(a), stem(b), "{a} {b}");
        }
        assert!(synonyms(&stem("save")).contains(&"writ") && synonyms("scrub").contains(&"strip"));
        assert!(near("refresh", "refrsh") && near("prewarm", "prewram") && !near("refresh", "resume"));
        assert_eq!(query_terms("where do we save notes"), [(stem("save"), 1.0), (stem("notes"), 1.0)]);
    }
}
