use std::collections::HashSet;
use std::sync::OnceLock;

use regex::Regex;
use serde_json::{Map, Value};

pub const NAMES: &[&str] = &[
    "lines",
    "words_max",
    "words_min",
    "sentences",
    "bullets",
    "numbered",
    "json_keys",
    "json_len",
    "no_commas",
    "lower",
    "upper",
    "starts",
    "ends",
    "title",
    "has",
    "lacks",
    "paragraphs",
    "german",
    "one_of",
    "chars_under",
    "quoted",
    "end_question",
    "placeholders",
    "count",
    "in_order",
    "headers",
    "table_rows",
];

pub struct Language {
    pub german: HashSet<String>,
    pub english: HashSet<String>,
}

impl Language {
    pub fn new(german: &str, english: &str) -> Language {
        let set = |raw: &str| -> HashSet<String> { raw.split_whitespace().map(str::to_lowercase).collect() };
        Language { german: set(german), english: set(english) }
    }
}

fn pattern(cell: &'static OnceLock<Regex>, raw: &str) -> &'static Regex {
    cell.get_or_init(|| Regex::new(raw).expect("valid pattern"))
}

fn word_pattern() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    pattern(&CELL, "[\\w'\u{2019}-]+")
}

fn bullet_pattern() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    pattern(&CELL, r"^\s*[-*•]\s+\S")
}

fn numbered_pattern() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    pattern(&CELL, r"^\s*\d+[.)]\s+\S")
}

fn header_pattern() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    pattern(&CELL, r"^\s*##\s")
}

fn rule_row_pattern() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    pattern(&CELL, r"^\|?[\s:|-]+\|?$")
}

fn paragraph_pattern() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    pattern(&CELL, r"\n\s*\n")
}

fn title_pattern() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    pattern(&CELL, r"<<[^<>\n]+>>")
}

fn placeholder_pattern() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    pattern(&CELL, r"\[[^\[\]\n]+\]")
}

fn number_pattern() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    pattern(&CELL, r"-?[\d,]*\.?\d+")
}

fn tagged_pattern() -> &'static Regex {
    static CELL: OnceLock<Regex> = OnceLock::new();
    pattern(&CELL, r"(?i)answer\s*[:=]\s*\**\s*\$?\s*(-?[\d,]*\.?\d+)")
}

fn lines(text: &str) -> Vec<&str> {
    text.lines().filter(|row| !row.trim().is_empty()).collect()
}

fn words(text: &str) -> usize {
    word_pattern().find_iter(text).count()
}

fn wordy(part: &str) -> bool {
    part.chars().any(|c| c.is_alphanumeric() || c == '_')
}

pub fn sentences(text: &str) -> usize {
    let flat: Vec<char> = text.split_whitespace().collect::<Vec<_>>().join(" ").chars().collect();
    let mut parts = Vec::new();
    let (mut start, mut at) = (0, 0);
    while at < flat.len() {
        if ".!?".contains(flat[at]) {
            let mut next = at + 1;
            if next < flat.len() && "\"\u{201d}'".contains(flat[next]) {
                next += 1;
            }
            if next < flat.len() && flat[next] == ' ' {
                parts.push(flat[start..=at].iter().collect::<String>());
                start = next + 1;
                at = next + 1;
                continue;
            }
        }
        at += 1;
    }
    if start < flat.len() {
        parts.push(flat[start..].iter().collect::<String>());
    }
    parts.iter().filter(|p| wordy(p)).count()
}

fn as_json(text: &str) -> Option<Map<String, Value>> {
    match serde_json::from_str::<Value>(text) {
        Ok(Value::Object(map)) => Some(map),
        _ => None,
    }
}

fn strings(arg: &Value) -> Vec<String> {
    arg.as_array().map(|xs| xs.iter().filter_map(Value::as_str).map(str::to_string).collect()).unwrap_or_default()
}

fn size(arg: &Value) -> usize {
    arg.as_u64().unwrap_or(0) as usize
}

fn pair(arg: &Value) -> (String, usize) {
    let word = arg.get(0).and_then(Value::as_str).unwrap_or_default();
    let least = arg.get(1).and_then(Value::as_u64).unwrap_or(0) as usize;
    (word.to_string(), least)
}

fn german(text: &str, language: &Language) -> bool {
    let tokens: Vec<String> = word_pattern().find_iter(text).map(|m| m.as_str().to_lowercase()).collect();
    let de = tokens.iter().filter(|t| language.german.contains(*t) || t.chars().any(|c| "äöüß".contains(c))).count();
    let en = tokens.iter().filter(|t| language.english.contains(*t)).count();
    de > en || en == 0
}

fn count(text: &str, arg: &Value) -> bool {
    let (word, least) = pair(arg);
    if !word.is_empty() && word.chars().all(char::is_alphanumeric) {
        let found =
            Regex::new(&format!(r"(?i)\b{}\b", regex::escape(&word))).map(|re| re.find_iter(text).count()).unwrap_or(0);
        return found >= least;
    }
    text.matches(word.as_str()).count() >= least
}

fn in_order(text: &str, names: &[String]) -> bool {
    let low = text.to_lowercase();
    let spots: Vec<Option<usize>> = names.iter().map(|n| low.find(&n.to_lowercase())).collect();
    spots.iter().all(Option::is_some) && spots.windows(2).all(|w| w[0] <= w[1])
}

fn headers(text: &str, wanted: usize) -> bool {
    let found: Vec<&str> = text.lines().filter(|row| row.trim_start().starts_with('#')).collect();
    found.len() == wanted && found.iter().all(|row| header_pattern().is_match(row))
}

fn table_rows(text: &str, wanted: usize) -> bool {
    let all = lines(text);
    let rows: Vec<&str> = all.iter().map(|row| row.trim()).filter(|row| row.starts_with('|')).collect();
    let body = rows.iter().filter(|row| !rule_row_pattern().is_match(row)).count();
    rows.len() == all.len() && body == wanted + 1
}

fn paragraphs(text: &str) -> usize {
    paragraph_pattern().split(text.trim()).filter(|p| !p.trim().is_empty()).count()
}

fn letters(text: &str) -> bool {
    text.chars().any(char::is_alphabetic)
}

fn lacks(text: &str, words: &[String]) -> bool {
    !words
        .iter()
        .any(|w| Regex::new(&format!(r"(?i)\b{}", regex::escape(w))).map(|re| re.is_match(text)).unwrap_or(false))
}

fn one_of(text: &str, options: &[String]) -> bool {
    let core = text.trim().trim_matches(|c| c == '.' || c == '!').trim();
    options.iter().any(|o| o == core)
}

fn quoted(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    chars.len() >= 2 && "\"\u{201c}".contains(chars[0]) && "\"\u{201d}".contains(chars[chars.len() - 1])
}

pub fn check(name: &str, text: &str, arg: &Value, language: &Language) -> bool {
    match name {
        "lines" => lines(text).len() == size(arg),
        "words_max" => words(text) <= size(arg),
        "words_min" => words(text) >= size(arg),
        "sentences" => sentences(text) == size(arg),
        "bullets" => lines(text).iter().filter(|r| bullet_pattern().is_match(r)).count() == size(arg),
        "numbered" => lines(text).iter().filter(|r| numbered_pattern().is_match(r)).count() == size(arg),
        "json_keys" => as_json(text).is_some_and(|m| strings(arg).iter().all(|k| m.contains_key(k))),
        "json_len" => {
            let (key, wanted) = pair(arg);
            as_json(text).and_then(|m| m.get(&key).and_then(Value::as_array).map(Vec::len)).is_some_and(|n| n == wanted)
        }
        "no_commas" => !text.contains(',') && !text.contains('\u{ff0c}'),
        "lower" => letters(text) && text.to_lowercase() == text,
        "upper" => letters(text) && text.to_uppercase() == text,
        "starts" => text.starts_with(arg.as_str().unwrap_or_default()),
        "ends" => text.trim_end().ends_with(arg.as_str().unwrap_or_default()),
        "title" => title_pattern().is_match(text),
        "has" => {
            let low = text.to_lowercase();
            strings(arg).iter().all(|x| low.contains(&x.to_lowercase()))
        }
        "lacks" => lacks(text, &strings(arg)),
        "paragraphs" => paragraphs(text) == size(arg),
        "german" => german(text, language),
        "one_of" => one_of(text, &strings(arg)),
        "chars_under" => text.chars().count() < size(arg),
        "quoted" => quoted(text),
        "end_question" => text.trim_end().ends_with('?'),
        "placeholders" => placeholder_pattern().find_iter(text).count() >= size(arg),
        "count" => count(text, arg),
        "in_order" => in_order(text, &strings(arg)),
        "headers" => headers(text, size(arg)),
        "table_rows" => table_rows(text, size(arg)),
        _ => false,
    }
}

pub fn variants(text: &str) -> Vec<String> {
    let rows: Vec<&str> = text.split('\n').collect();
    let n = rows.len();
    let inner: &[&str] = if n > 2 { &rows[1..n - 1] } else { &[] };
    let cuts = [&rows[..], &rows[1.min(n)..], &rows[..n.saturating_sub(1)], inner];
    let mut out = Vec::new();
    for cut in cuts {
        let joined = cut.join("\n").trim().to_string();
        let plain = joined.replace('*', "");
        out.push(joined);
        out.push(plain);
    }
    out.retain(|v| !v.is_empty());
    out
}

pub struct Grade {
    pub strict: Map<String, Value>,
    pub loose: Map<String, Value>,
}

pub fn grade(text: &str, checks: &Map<String, Value>, language: &Language) -> Grade {
    let tries = variants(text);
    let mut strict = Map::new();
    let mut loose = Map::new();
    for (name, arg) in checks {
        strict.insert(name.clone(), Value::Bool(check(name, text, arg, language)));
        let any = tries.iter().any(|v| check(name, v, arg, language));
        loose.insert(name.clone(), Value::Bool(any));
    }
    Grade { strict, loose }
}

pub fn to_number(raw: &str) -> Option<f64> {
    let cleaned: String = raw.chars().filter(|c| *c != ',' && *c != '$' && !c.is_whitespace()).collect();
    cleaned.parse().ok()
}

pub fn math_pick(text: &str) -> (Option<f64>, Option<f64>) {
    let strict = tagged_pattern().captures_iter(text).last().and_then(|c| c.get(1)).and_then(|m| to_number(m.as_str()));
    let loose = strict.or_else(|| number_pattern().find_iter(text).last().and_then(|m| to_number(m.as_str())));
    (strict, loose)
}

pub fn same(a: Option<f64>, b: Option<f64>) -> bool {
    matches!((a, b), (Some(x), Some(y)) if (x - y).abs() < 1e-6)
}

#[cfg(test)]
mod tests {
    use super::{Language, grade, math_pick, sentences, variants};
    use serde_json::{Map, Value, json};

    fn language() -> Language {
        Language::new("der die das und ist ein eine mit", "the and is of to for")
    }

    fn passes(text: &str, checks: Value) -> (bool, bool) {
        let checks: Map<String, Value> = checks.as_object().cloned().expect("object");
        let found = grade(text, &checks, &language());
        let all = |m: &Map<String, Value>| m.values().all(|v| v == &Value::Bool(true));
        (all(&found.strict), all(&found.loose))
    }

    #[test]
    fn good_answers_pass_every_rule() {
        let cases = [
            ("Soft rain\nwhispers low\npuddles hold sky", json!({"lines": 3})),
            ("- a\n- b\n- c\n- d", json!({"bullets": 4, "lines": 4})),
            (r#"{"name": "Ana", "age": 31, "city": "Lyon"}"#, json!({"json_keys": ["name", "age", "city"]})),
            ("The sun sinks low. The sky turns gold and red.", json!({"no_commas": true, "sentences": 2})),
            ("Ein Vulkan ist ein Berg. Aus ihm kommt heiße Lava.", json!({"german": true, "sentences": 2})),
            ("- Gesund\n- Günstig\n- Umweltfreundlich", json!({"german": true, "bullets": 3})),
            ("Yes", json!({"one_of": ["Yes", "No"], "has": ["Yes"]})),
            (r#"{"colors": ["red", "blue", "green"]}"#, json!({"json_len": ["colors", 3]})),
            ("Mercury, Venus, Earth, Mars", json!({"in_order": ["Mercury", "Venus", "Earth", "Mars"]})),
            ("| Animal | Sound |\n|---|---|\n| Dog | Woof |\n| Cat | Meow |", json!({"table_rows": 2})),
            ("## Ingredients\nFlour\n## Steps\nMix.", json!({"headers": 2})),
            (
                "```python\ndef add(a, b):\n    return a + b\n```",
                json!({"has": ["```python", "def "], "count": ["```", 2]}),
            ),
            ("\"The Moon has no air.\"", json!({"quoted": true, "sentences": 1})),
            ("Tall trunks rise. Moss covers rocks. Birds sing.", json!({"lacks": ["tree", "green"], "sentences": 3})),
            ("music is music and more music", json!({"count": ["music", 3]})),
            ("Hi [name], see you at [time] in [room].", json!({"placeholders": 3})),
            ("<<Waves>>\nThe sea is wide", json!({"title": true})),
            ("Para one.\n\nPara two.\n\nPara three.", json!({"paragraphs": 3})),
        ];
        for (text, checks) in cases {
            assert_eq!(passes(text, checks.clone()), (true, true), "{text} {checks}");
        }
    }

    #[test]
    fn bad_answers_fail_strict() {
        assert!(!passes("Trees rise. Green moss.", json!({"lacks": ["tree", "green"]})).0);
        let fenced = "```json\n{\"name\": \"A\", \"age\": 1, \"city\": \"B\"}\n```";
        assert_eq!(passes(fenced, json!({"json_keys": ["name", "age", "city"]})), (false, true));
        assert!(!passes("- Healthy\n- Cheap for the planet", json!({"german": true})).0);
        assert!(!passes("### Deep\nx\n## Two", json!({"headers": 2})).0);
    }

    #[test]
    fn sentences_split_after_quotes_and_marks() {
        assert_eq!(sentences("One. Two! Three?"), 3);
        assert_eq!(sentences("He said \"stop.\" Then left."), 2);
        assert_eq!(sentences("Pi is 3.14 today."), 1);
    }

    #[test]
    fn loose_variants_drop_first_and_last_line() {
        let found = variants("Sure!\nbody\nThanks");
        assert!(found.contains(&"body".to_string()));
        assert!(found.contains(&"Sure!\nbody".to_string()));
    }

    #[test]
    fn math_answers_are_read_from_the_tag_or_the_last_number() {
        assert_eq!(math_pick("16-3-4=9, 9*2=18.\nAnswer: $18"), (Some(18.0), Some(18.0)));
        assert_eq!(math_pick("The total is 1,250 dollars."), (None, Some(1250.0)));
    }
}
