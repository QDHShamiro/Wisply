use std::collections::HashMap;
use std::sync::LazyLock;

const WORDS: &str = include_str!("words.txt");
const ENDINGS: &[&str] = &["ten", "tet", "en", "er", "es", "em", "et", "te", "st", "e", "n", "t", "s"];
const LINKS: &[&str] = &["", "s", "es", "n", "en", "e"];
const MIN_KEY: usize = 4;
const PART_FROM: usize = 8;
const PART_MIN: usize = 4;
const PART_DEPTH: usize = 1;

struct Lexicon {
    groups: Vec<Vec<&'static str>>,
    exact: HashMap<String, Vec<usize>>,
    stems: HashMap<String, Vec<usize>>,
}

static LEXICON: LazyLock<Lexicon> = LazyLock::new(|| {
    let mut lex = Lexicon { groups: Vec::new(), exact: HashMap::new(), stems: HashMap::new() };
    for line in WORDS.lines() {
        let group: Vec<&'static str> = line.split_whitespace().collect();
        if group.is_empty() {
            continue;
        }
        let at = lex.groups.len();
        for word in &group {
            let exact = lex.exact.entry(fold(word)).or_default();
            if !exact.contains(&at) {
                exact.push(at);
            }
            let stem = stem_word(word);
            if stem.chars().count() >= MIN_KEY {
                let stems = lex.stems.entry(stem).or_default();
                if !stems.contains(&at) {
                    stems.push(at);
                }
            }
        }
        lex.groups.push(group);
    }
    lex
});

const TAILS: &[&str] = &["ungen", "ung", "ing", "ed", "es", "en", "er", "s"];
const MIN_ROOT: usize = 4;

pub fn fold(word: &str) -> String {
    let mut out = String::with_capacity(word.len());
    for c in word.chars() {
        match c {
            'ä' | 'á' | 'à' | 'â' => out.push('a'),
            'ö' | 'ó' | 'ò' | 'ô' => out.push('o'),
            'ü' | 'ú' | 'ù' | 'û' => out.push('u'),
            'é' | 'è' | 'ê' => out.push('e'),
            'í' | 'ì' | 'î' => out.push('i'),
            'ß' => out.push_str("ss"),
            other => out.push(other),
        }
    }
    for (pair, one) in [("ae", "a"), ("oe", "o"), ("ue", "u")] {
        if out.len() > 4 && out.contains(pair) {
            out = out.replace(pair, one);
        }
    }
    out
}

fn strippable(root: &str, tail: &str) -> bool {
    let n = root.chars().count();
    match tail {
        "s" => n >= 3 && !root.ends_with('s'),
        "ed" => n >= 3 && root.contains(['a', 'i', 'o', 'u', 'y']),
        "ing" | "es" => n >= 3,
        _ => n >= MIN_ROOT,
    }
}

pub fn stem_word(word: &str) -> String {
    let mut w = fold(&word.to_lowercase());
    for _ in 0..2 {
        let Some(root) = TAILS.iter().find_map(|t| w.strip_suffix(t).filter(|r| strippable(r, t))) else {
            break;
        };
        w = root.to_string();
    }
    if w.chars().count() > 3 && w.ends_with('e') {
        w.pop();
    }
    let mut tail = w.chars().rev();
    if let (Some(a), Some(b)) = (tail.next(), tail.next())
        && a == b
        && a.is_alphabetic()
        && !matches!(a, 'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 's' | 'z')
        && w.chars().count() > 3
    {
        w.pop();
    }
    w
}

fn direct(key: &str) -> Option<&'static [usize]> {
    if let Some(found) = LEXICON.exact.get(key) {
        return Some(found.as_slice());
    }
    let stem = stem_word(key);
    if stem.chars().count() < MIN_KEY {
        return None;
    }
    LEXICON.stems.get(&stem).map(Vec::as_slice)
}

fn lookup(key: &str) -> &'static [usize] {
    direct(key)
        .or_else(|| {
            ENDINGS
                .iter()
                .filter_map(|e| key.strip_suffix(e))
                .filter(|base| base.chars().count() >= MIN_KEY)
                .find_map(direct)
        })
        .unwrap_or(&[])
}

pub fn variants(term: &str) -> Vec<&'static str> {
    let key = fold(&term.to_lowercase());
    let mut out: Vec<&'static str> = Vec::new();
    for g in lookup(&key) {
        for &w in &LEXICON.groups[*g] {
            if fold(w) != key && !out.contains(&w) {
                out.push(w);
            }
        }
    }
    out
}

fn split(word: &str, depth: usize) -> Vec<String> {
    let cuts: Vec<usize> = word.char_indices().map(|(i, _)| i).collect();
    if cuts.len() < PART_FROM {
        return Vec::new();
    }
    for at in (PART_MIN..=cuts.len() - PART_MIN).rev() {
        let (left, right) = word.split_at(cuts[at]);
        let head = LINKS
            .iter()
            .filter_map(|l| left.strip_suffix(l))
            .find(|h| h.chars().count() >= PART_MIN && direct(h).is_some());
        let Some(head) = head else { continue };
        let tail = if direct(right).is_some() {
            vec![right.to_string()]
        } else if depth > 0 {
            split(right, depth - 1)
        } else {
            Vec::new()
        };
        if !tail.is_empty() {
            let mut out = vec![head.to_string()];
            out.extend(tail);
            return out;
        }
    }
    Vec::new()
}

pub fn parts(term: &str) -> Vec<String> {
    let word = fold(&term.to_lowercase());
    if !word.chars().all(char::is_alphabetic) || !lookup(&word).is_empty() {
        return Vec::new();
    }
    split(&word, PART_DEPTH)
}

pub fn near(a: &str, b: &str) -> bool {
    let (x, y) = (a.as_bytes(), b.as_bytes());
    if x.len().min(y.len()) < 4 || x.len().abs_diff(y.len()) > 1 || !a.is_ascii() || !b.is_ascii() {
        return false;
    }
    let skip = x.iter().zip(y).take_while(|(p, q)| p == q).count();
    let (rx, ry) = (&x[skip..], &y[skip..]);
    match rx.len().cmp(&ry.len()) {
        std::cmp::Ordering::Equal => {
            !rx.is_empty()
                && (rx[1..] == ry[1..] || (rx.len() >= 2 && rx[0] == ry[1] && rx[1] == ry[0] && rx[2..] == ry[2..]))
        }
        std::cmp::Ordering::Greater => rx[1..] == *ry,
        std::cmp::Ordering::Less => *rx == ry[1..],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_cross_languages_and_typos_stay_close() {
        assert!(variants("titel").contains(&"title") && variants("title").contains(&"titel"));
        assert!(variants("zzz").is_empty());
        assert!(!near("prewarm", "voise") && near("voice", "voise") && near("sight", "sigth"));
        assert!(near("ability", "abilty") && !near("insertion", "insertng") && !near("same", "same"));
    }

    #[test]
    fn stems_join_word_forms_and_umlauts() {
        for (a, b) in [("settings", "setting"), ("saved", "save"), ("embedding", "embed"), ("hoehe", "höhe")] {
            assert_eq!(stem_word(a), stem_word(b), "{a} {b}");
        }
        assert_eq!(stem_word("Einstellungen"), stem_word("einstellung"));
        assert_ne!(stem_word("plan"), stem_word("plate"));
    }

    #[test]
    fn word_list_is_clean() {
        let mut lines = 0;
        for line in WORDS.lines() {
            let group: Vec<&str> = line.split_whitespace().collect();
            assert!(!group.is_empty(), "empty line");
            for (i, w) in group.iter().enumerate() {
                assert!(w.chars().all(char::is_alphanumeric) && *w == w.to_lowercase(), "{w}");
                assert!(!group[..i].contains(w), "{w} twice in {line}");
            }
            lines += 1;
        }
        assert!(lines > 500);
    }

    #[test]
    fn word_forms_reach_the_other_language() {
        for (asked, wanted) in [
            ("lizenzen", "license"),
            ("loeschen", "delete"),
            ("gestoppt", "stop"),
            ("stoppt", "stop"),
            ("überschreibt", "overwrite"),
            ("spätere", "later"),
            ("licenses", "lizenz"),
            ("villagers", "dorfbewohner"),
        ] {
            assert!(variants(asked).contains(&wanted), "{asked} -> {wanted}: {:?}", variants(asked));
        }
        assert!(!variants("löschen").contains(&"löschen"));
    }

    #[test]
    fn senses_stay_apart() {
        let schild = variants("schild");
        assert!(schild.contains(&"shield") && schild.contains(&"sign"));
        assert!(!variants("shield").contains(&"sign") && !variants("signs").contains(&"shield"));
        for (asked, wrong) in [("tag", "day"), ("chart", "character"), ("meant", "average"), ("dark", "gray")] {
            assert!(!variants(asked).contains(&wrong), "{asked} -> {wrong}: {:?}", variants(asked));
        }
        assert!(variants("laden").contains(&"load") && variants("laden").contains(&"shop"));
    }

    #[test]
    fn compounds_split_into_known_words() {
        for (word, want) in [
            ("mindestgewinn", vec!["mindest", "gewinn"]),
            ("Bildschirmtext", vec!["bildschirm", "text"]),
            ("betäubungsschlag", vec!["betaubungs", "schlag"]),
            ("dateiname", vec!["datei", "name"]),
            ("mindestbestellwert", vec!["mindest", "bestell", "wert"]),
        ] {
            assert_eq!(parts(word), want, "{word}");
        }
        for whole in ["einstellung", "einstellungen", "minecraft", "bildschirm", "kurz", "abc123defgh", "zzzzyyyyxxxx"]
        {
            assert!(parts(whole).is_empty(), "{whole}: {:?}", parts(whole));
        }
    }
}
