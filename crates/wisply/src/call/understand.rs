use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::{Map, Value, json};

use super::cfg;
use super::store::{mapped, named, refused, resolve, said, sure_no};

const PROMPT: &str = include_str!("understand.txt");
const UNCLEAR: &str = "unclear";
const NOTE_MAX: usize = 80;
const TEXT_MAX: usize = 300;
const SURE_WORDS: usize = 3;
const RULE_WORDS: usize = 6;

const ORDINALS: &[(&str, usize)] = &[
    ("erste", 1),
    ("ersten", 1),
    ("erstes", 1),
    ("erster", 1),
    ("first", 1),
    ("zweite", 2),
    ("zweiten", 2),
    ("zweites", 2),
    ("zweiter", 2),
    ("second", 2),
    ("dritte", 3),
    ("dritten", 3),
    ("drittes", 3),
    ("dritter", 3),
    ("third", 3),
    ("vierte", 4),
    ("vierten", 4),
    ("viertes", 4),
    ("vierter", 4),
    ("fourth", 4),
    ("fünfte", 5),
    ("fünften", 5),
    ("fuenfte", 5),
    ("fuenften", 5),
    ("fifth", 5),
];
const CARDINALS: &[(&str, usize)] =
    &[("eins", 1), ("one", 1), ("zwei", 2), ("two", 2), ("drei", 3), ("three", 3), ("vier", 4), ("four", 4)];
const LASTS: &[&str] = &["letzte", "letzten", "letztes", "letzter", "last"];
const LABELS: &[&str] = &["nummer", "number", "nr", "no", "option", "wahl", "choice"];
pub(super) const NEGATIONS: &[&str] =
    &["nicht", "kein", "keine", "keinen", "niemals", "nie", "not", "dont", "never", "nein"];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Verdict {
    pub choice: String,
    pub note: String,
    pub how: &'static str,
}

pub trait Model {
    fn ask(&self, system: &str, user: &str, schema: &Value) -> Result<Value, String>;
}

impl<F> Model for F
where
    F: Fn(&str, &str, &Value) -> Result<Value, String>,
{
    fn ask(&self, system: &str, user: &str, schema: &Value) -> Result<Value, String> {
        self(system, user, schema)
    }
}

pub struct Local {
    home: PathBuf,
    timeout: Duration,
}

impl Local {
    pub fn new(home: &Path, settings: &Map<String, Value>) -> Local {
        Local {
            home: home.to_path_buf(),
            timeout: Duration::from_secs(cfg::num(settings, "understand_timeout_s").max(1)),
        }
    }
}

impl Model for Local {
    fn ask(&self, system: &str, user: &str, schema: &Value) -> Result<Value, String> {
        crate::local::ask_json(&self.home, system, user, schema, self.timeout)
    }
}

pub fn enabled(settings: &Map<String, Value>) -> bool {
    cfg::text(settings, "understand") == "auto"
}

fn tokens(text: &str) -> Vec<String> {
    text.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_string).collect()
}

fn pick(words: &[String], table: &[(&str, usize)]) -> Option<usize> {
    table.iter().find(|(w, _)| words.iter().any(|t| t == w)).map(|(_, n)| *n)
}

fn candidates(words: &[String], total: usize) -> Option<Vec<usize>> {
    let mut found = Vec::new();
    for (at, word) in words.iter().enumerate() {
        if let Ok(n) = word.parse::<usize>() {
            if n == 0 || n > total {
                return None;
            }
            found.push(n);
        } else if let Some((_, n)) = ORDINALS.iter().find(|(w, _)| w == word) {
            found.push(*n);
        } else if LASTS.contains(&word.as_str()) {
            found.push(total);
        } else if at > 0 && LABELS.contains(&words[at - 1].as_str()) {
            found.extend(pick(std::slice::from_ref(word), CARDINALS));
        }
    }
    found.sort_unstable();
    found.dedup();
    Some(found)
}

pub fn ordinal(choices: &[String], text: &str) -> Option<String> {
    let words = tokens(text);
    if words.is_empty() || words.len() > RULE_WORDS || words.iter().any(|w| NEGATIONS.contains(&w.as_str())) {
        return None;
    }
    match candidates(&words, choices.len())?.as_slice() {
        [n] => choices.get(n - 1).cloned(),
        _ => None,
    }
}

fn yes_no_like(settings: &Map<String, Value>, choices: &[String]) -> bool {
    mapped(settings, choices, true).is_some() || mapped(settings, choices, false).is_some()
}

fn ambiguous(settings: &Map<String, Value>, choices: &[String], text: &str) -> bool {
    if yes_no_like(settings, choices) {
        return false;
    }
    if named(choices, text) > 1 || (choices.len() > 2 && refused(choices, text)) {
        return true;
    }
    let words = tokens(text);
    if words.len() > RULE_WORDS || words.iter().any(|w| NEGATIONS.contains(&w.as_str())) {
        return false;
    }
    candidates(&words, choices.len()).is_none_or(|found| found.len() > 1 || found.iter().any(|&n| n > choices.len()))
}

fn first_try(settings: &Map<String, Value>, choices: &[String], text: &str) -> Option<(String, &'static str)> {
    if let Some(choice) = resolve(settings, choices, text) {
        return Some((choice, "words"));
    }
    ordinal(choices, text).map(|choice| (choice, "rules"))
}

pub fn schema(choices: &[String]) -> Value {
    let mut options: Vec<Value> = choices.iter().map(|c| json!(c)).collect();
    options.push(json!(UNCLEAR));
    json!({
        "type": "object",
        "properties": {
            "choice": {"type": "string", "enum": options},
            "note": {"type": "string", "maxLength": NOTE_MAX},
        },
        "required": ["choice", "note"],
        "additionalProperties": false,
    })
}

fn clip(text: &str) -> String {
    text.trim().chars().take(TEXT_MAX).collect()
}

pub fn prompt(question: &str, choices: &[String], text: &str) -> (String, String) {
    let list: Vec<String> = choices.iter().enumerate().map(|(i, c)| format!("{}. {}", i + 1, clip(c))).collect();
    (PROMPT.trim().to_string(), format!("Question: {}\n{}\nReply: {}", clip(question), list.join("\n"), clip(text)))
}

fn read_reply(reply: &Value, choices: &[String]) -> Option<(String, String)> {
    let choice = reply.get("choice")?.as_str()?;
    let note = reply.get("note").and_then(Value::as_str).unwrap_or("").trim();
    let found = choices.iter().find(|c| c.as_str() == choice)?;
    Some((found.clone(), note.chars().take(NOTE_MAX).collect()))
}

enum Asked {
    Unavailable,
    Unclear,
    Pick(String, String),
}

fn ask_model(model: &dyn Model, question: &str, choices: &[String], text: &str) -> Asked {
    let (system, user) = prompt(question, choices, text);
    let Ok(reply) = model.ask(&system, &user, &schema(choices)) else { return Asked::Unavailable };
    match read_reply(&reply, choices) {
        Some((choice, note)) => Asked::Pick(choice, note),
        None => Asked::Unclear,
    }
}

pub fn decide(
    settings: &Map<String, Value>,
    question: &str,
    choices: &[String],
    text: &str,
    model: Option<&dyn Model>,
) -> Option<Verdict> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    let sure = first_try(settings, choices, trimmed);
    if choices.is_empty() {
        return sure.map(|(choice, how)| Verdict { choice, note: String::new(), how });
    }
    if said(settings, "unsure_words", trimmed) {
        return None;
    }
    if sure_no(settings, trimmed)
        && let Some(choice) = mapped(settings, choices, false)
    {
        return Some(Verdict { choice, note: String::new(), how: "rules" });
    }
    if sure.is_none() && ambiguous(settings, choices, trimmed) {
        return None;
    }
    let asked = model.filter(|_| sure.is_none() || tokens(trimmed).len() > SURE_WORDS);
    let Some(model) = asked else {
        return sure.map(|(choice, how)| Verdict { choice, note: String::new(), how });
    };
    match (sure, ask_model(model, question, choices, trimmed)) {
        (Some((choice, how)), Asked::Unavailable) => Some(Verdict { choice, note: String::new(), how }),
        (Some((choice, how)), Asked::Pick(got, note)) if got == choice => Some(Verdict { choice, note, how }),
        (Some(_), _) | (None, Asked::Unavailable | Asked::Unclear) => None,
        (None, Asked::Pick(choice, note)) => {
            let flipped: Vec<String> = choices.iter().rev().cloned().collect();
            match ask_model(model, question, &flipped, trimmed) {
                Asked::Pick(again, _) if again == choice => Some(Verdict { choice, note, how: "model" }),
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    fn yes_no() -> Vec<String> {
        vec!["ja".into(), "nein".into()]
    }

    fn colors() -> Vec<String> {
        vec!["rot".into(), "blau".into(), "grün".into()]
    }

    fn settings() -> Map<String, Value> {
        cfg::defaults()
    }

    fn says(choice: &'static str, note: &'static str) -> impl Fn(&str, &str, &Value) -> Result<Value, String> {
        move |_, _, _| Ok(json!({"choice": choice, "note": note}))
    }

    fn broken(_: &str, _: &str, _: &Value) -> Result<Value, String> {
        Err("timeout".into())
    }

    #[test]
    fn ordinals_and_numbers_pick_the_choice_in_german_and_english() {
        let c = colors();
        for text in ["den zweiten", "zweite", "nummer 2", "2", "die 2.", "the second", "Number two", "option 2", "#2"] {
            assert_eq!(ordinal(&c, text).as_deref(), Some("blau"), "{text}");
        }
        for text in ["first one", "den ersten", "1"] {
            assert_eq!(ordinal(&c, text).as_deref(), Some("rot"), "{text}");
        }
        for text in ["den letzten", "last", "the last one", "3", "dritte"] {
            assert_eq!(ordinal(&c, text).as_deref(), Some("grün"), "{text}");
        }
    }

    #[test]
    fn ordinals_refuse_what_is_not_clear() {
        let c = colors();
        for text in [
            "4",
            "0",
            "nicht den zweiten",
            "den ersten oder den zweiten",
            "2024",
            "",
            "i was there at 2 and then went home late",
        ] {
            assert_eq!(ordinal(&c, text), None, "{text}");
        }
        assert_eq!(ordinal(&yes_no(), "den dritten"), None);
    }

    #[test]
    fn the_schema_only_allows_a_choice_or_unclear_and_a_short_note() {
        let s = schema(&yes_no());
        assert_eq!(s["properties"]["choice"]["enum"], json!(["ja", "nein", "unclear"]));
        assert_eq!(s["properties"]["note"]["maxLength"], 80);
        assert_eq!(s["required"], json!(["choice", "note"]));
    }

    #[test]
    fn the_prompt_is_short_and_lists_the_choices_by_number() {
        let (system, user) = prompt("Soll ich?", &yes_no(), "mach mal");
        assert_eq!(user, "Question: Soll ich?\n1. ja\n2. nein\nReply: mach mal");
        assert!(system.contains("unclear") && system.contains("erst morgen") && system.contains("mach das bloß nicht"));
        assert!(system.len() < 1800, "{}", system.len());
    }

    #[test]
    fn clear_short_answers_never_ask_the_model() {
        let asked = Cell::new(0);
        let spy = |_: &str, _: &str, _: &Value| {
            asked.set(asked.get() + 1);
            Ok(json!({"choice": "nein", "note": ""}))
        };
        let got = decide(&settings(), "Q", &yes_no(), "ja klar", Some(&spy)).unwrap();
        assert_eq!((got.choice.as_str(), got.how, asked.get()), ("ja", "words", 0));
        let got = decide(&settings(), "Q", &colors(), "den zweiten", Some(&spy)).unwrap();
        assert_eq!((got.choice.as_str(), got.how, asked.get()), ("blau", "rules", 0));
    }

    #[test]
    fn an_unclear_reply_goes_to_the_model_and_its_note_is_kept() {
        let model = says("ja", "erst morgen");
        let got = decide(&settings(), "Q", &yes_no(), "lass uns das morgen erledigen", Some(&model)).unwrap();
        assert_eq!((got.choice.as_str(), got.note.as_str(), got.how), ("ja", "erst morgen", "model"));
    }

    #[test]
    fn a_long_answer_is_double_checked_and_the_note_comes_from_the_model() {
        let agree = says("ja", "erst morgen");
        let got = decide(&settings(), "Q", &yes_no(), "mach mal, aber erst morgen", Some(&agree)).unwrap();
        assert_eq!((got.choice.as_str(), got.note.as_str(), got.how), ("ja", "erst morgen", "words"));
    }

    #[test]
    fn when_the_model_disagrees_with_the_rules_the_answer_is_unclear() {
        let no = says("nein", "");
        assert_eq!(decide(&settings(), "Q", &yes_no(), "ja mach das jetzt sofort", Some(&no)), None);
        let unsure = says("unclear", "");
        assert_eq!(decide(&settings(), "Q", &yes_no(), "ja mach das aber bitte vorsichtig", Some(&unsure)), None);
    }

    #[test]
    fn a_missing_or_slow_model_changes_nothing_for_the_rules() {
        let got = decide(&settings(), "Q", &yes_no(), "mach das bitte jetzt sofort", Some(&broken)).unwrap();
        assert_eq!((got.choice.as_str(), got.note.as_str(), got.how), ("ja", "", "words"));
        assert_eq!(decide(&settings(), "Q", &yes_no(), "vielleicht morgen", Some(&broken)), None);
        assert_eq!(decide(&settings(), "Q", &yes_no(), "vielleicht morgen", None), None);
    }

    #[test]
    fn the_model_may_only_name_a_real_choice() {
        let wrong = says("vielleicht", "");
        assert_eq!(decide(&settings(), "Q", &yes_no(), "hmm schwer zu sagen eigentlich", Some(&wrong)), None);
        let unclear = says("unclear", "");
        assert_eq!(decide(&settings(), "Q", &yes_no(), "frag später nochmal", Some(&unclear)), None);
    }

    #[test]
    fn free_questions_never_use_the_model() {
        let never = |_: &str, _: &str, _: &Value| -> Result<Value, String> { panic!("asked") };
        let got = decide(&settings(), "Q", &[], "  irgendwas frei  ", Some(&never)).unwrap();
        assert_eq!((got.choice.as_str(), got.how), ("irgendwas frei", "words"));
    }

    #[test]
    fn doubt_is_unclear_without_asking_the_model() {
        let never = |_: &str, _: &str, _: &Value| -> Result<Value, String> { panic!("asked") };
        for text in ["not sure yet", "i don't know", "hm ich weiß nicht", "vielleicht morgen", "keine Ahnung", "egal"]
        {
            assert_eq!(decide(&settings(), "Q", &yes_no(), text, Some(&never)), None, "{text}");
        }
        for text in ["den ersten oder den zweiten", "die fünfte", "nummer 7", "blau oder grün", "nicht rot"] {
            assert_eq!(decide(&settings(), "Q", &colors(), text, Some(&never)), None, "{text}");
        }
    }

    #[test]
    fn a_firm_no_wins_without_the_model_unless_a_yes_comes_after_it() {
        let never = |_: &str, _: &str, _: &Value| -> Result<Value, String> { panic!("asked") };
        for text in
            ["auf gar keinen Fall, die Datenbank läuft noch", "mach das bloß nicht jetzt", "no way, the tests are red"]
        {
            let got = decide(&settings(), "Q", &yes_no(), text, Some(&never)).unwrap();
            assert_eq!((got.choice.as_str(), got.how), ("nein", "rules"), "{text}");
        }
        let yes = says("ja", "");
        let got = decide(&settings(), "Q", &yes_no(), "auf keinen fall warten, mach sofort", Some(&yes)).unwrap();
        assert_eq!(got.choice, "ja");
    }

    #[test]
    fn a_model_pick_must_survive_the_choices_in_reverse() {
        let calls = Cell::new(0);
        let fickle = |_: &str, user: &str, _: &Value| {
            calls.set(calls.get() + 1);
            let first = user.lines().nth(1).unwrap_or("").trim_start_matches("1. ").to_string();
            Ok(json!({"choice": first, "note": ""}))
        };
        assert_eq!(decide(&settings(), "Q", &colors(), "gelb", Some(&fickle)), None);
        assert_eq!(calls.get(), 2);
        let steady = says("blau", "");
        assert_eq!(decide(&settings(), "Q", &colors(), "die Farbe vom Himmel", Some(&steady)).unwrap().choice, "blau");
    }

    #[test]
    fn the_rules_alone_are_never_sure_and_wrong_on_the_eval_rows() {
        let mut text = include_str!("../../tests/data/call-answers.tsv").to_string();
        if let Ok(extra) = std::env::var("CALL_EVAL_ROWS") {
            text.push('\n');
            text.push_str(&std::fs::read_to_string(extra).unwrap_or_default());
        }
        let mut wrong = Vec::new();
        for line in text.lines().filter(|l| !l.trim().is_empty()) {
            let cols: Vec<&str> = line.split('\t').collect();
            let choices: Vec<String> = cols[1].split('|').filter(|c| !c.is_empty()).map(str::to_string).collect();
            let got = decide(&settings(), cols[0], &choices, cols[2], None).map_or(UNCLEAR.to_string(), |v| v.choice);
            if got != UNCLEAR && got != cols[3] {
                wrong.push(format!("{} -> {got} (wanted {})", cols[2], cols[3]));
            }
        }
        assert!(wrong.is_empty(), "{wrong:#?}");
    }

    #[test]
    fn a_long_note_is_cut() {
        let long = "x".repeat(200);
        let reply = json!({"choice": "ja", "note": long});
        let model = move |_: &str, _: &str, _: &Value| Ok(reply.clone());
        let got = decide(&settings(), "Q", &yes_no(), "na gut dann halt irgendwie so", Some(&model)).unwrap();
        assert_eq!(got.note.chars().count(), 80);
    }
}
