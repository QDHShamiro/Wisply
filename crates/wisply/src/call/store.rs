use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};
use wisply_core::Fail;

use super::cfg;
use super::text::{now, random_hex};

const KEEP_S: f64 = 86_400.0;
const ID_LEN: usize = 6;

#[derive(Clone, Debug)]
pub struct Ask {
    pub id: String,
    pub question: String,
    pub choices: Vec<String>,
    pub created: f64,
    pub deadline: f64,
    pub state: String,
    pub answer: Option<String>,
    pub via: String,
}

impl Ask {
    fn to_json(&self) -> Value {
        json!({
            "id": self.id, "question": self.question, "choices": self.choices, "created": self.created,
            "deadline": self.deadline, "state": self.state, "answer": self.answer, "via": self.via
        })
    }

    fn from_json(v: &Value) -> Option<Ask> {
        let s = |k: &str| v.get(k).and_then(Value::as_str).unwrap_or("").to_string();
        let choices = v.get("choices").and_then(Value::as_array).into_iter().flatten();
        Some(Ask {
            id: s("id").chars().filter(char::is_ascii_alphanumeric).collect::<String>(),
            question: s("question"),
            choices: choices.filter_map(|c| c.as_str().map(str::to_string)).collect(),
            created: v.get("created").and_then(Value::as_f64)?,
            deadline: v.get("deadline").and_then(Value::as_f64)?,
            state: s("state"),
            answer: v.get("answer").and_then(Value::as_str).map(str::to_string),
            via: s("via"),
        })
        .filter(|a| !a.id.is_empty())
    }

    pub fn open(&self) -> bool {
        self.state == "open" && self.deadline > now()
    }
}

fn dir(home: &Path) -> PathBuf {
    cfg::dir(home).join("asks")
}

fn path(home: &Path, id: &str) -> PathBuf {
    let clean: String = id.chars().filter(char::is_ascii_alphanumeric).collect();
    dir(home).join(format!("{clean}.json"))
}

fn write(home: &Path, ask: &Ask) -> Result<(), Fail> {
    let _ = fs::create_dir_all(dir(home));
    wisply_core::save(path(home, &ask.id), ask.to_json().to_string())
        .map_err(|e| Fail::new(format!("could not write the question ({e})")))
}

pub fn load(home: &Path, id: &str) -> Option<Ask> {
    let raw = fs::read_to_string(path(home, id)).ok()?;
    Ask::from_json(&serde_json::from_str::<Value>(&raw).ok()?)
}

pub fn all(home: &Path) -> Vec<Ask> {
    let mut rows: Vec<Ask> = fs::read_dir(dir(home))
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| fs::read_to_string(e.path()).ok())
        .filter_map(|t| Ask::from_json(&serde_json::from_str::<Value>(&t).ok()?))
        .collect();
    rows.sort_by(|a, b| a.created.total_cmp(&b.created));
    rows
}

pub fn open_list(home: &Path) -> Vec<Ask> {
    all(home).into_iter().filter(Ask::open).collect()
}

pub fn create(home: &Path, question: &str, choices: &[String], wait_s: u64) -> Result<Ask, Fail> {
    for old in all(home).iter().filter(|a| a.created + KEEP_S < now()) {
        let _ = fs::remove_file(path(home, &old.id));
    }
    let created = now();
    let ask = Ask {
        id: random_hex(ID_LEN),
        question: question.to_string(),
        choices: choices.to_vec(),
        created,
        deadline: created + wait_s as f64,
        state: "open".into(),
        answer: None,
        via: String::new(),
    };
    write(home, &ask)?;
    Ok(ask)
}

pub fn close(home: &Path, id: &str, state: &str) {
    if let Some(mut ask) = load(home, id).filter(|a| a.state == "open") {
        ask.state = state.to_string();
        let _ = write(home, &ask);
    }
}

fn words(text: &str) -> Vec<String> {
    text.to_lowercase().split(|c: char| !c.is_alphanumeric()).filter(|w| !w.is_empty()).map(str::to_string).collect()
}

fn hits(spoken: &[String], list: &[String]) -> bool {
    list.iter().any(|entry| {
        let parts = words(entry);
        !parts.is_empty() && spoken.windows(parts.len()).any(|w| w == parts.as_slice())
    })
}

pub fn verdict(cfg: &Map<String, Value>, text: &str) -> Option<bool> {
    let spoken = words(text);
    let no = hits(&spoken, &cfg::list(cfg, "no_words"));
    let yes = hits(&spoken, &cfg::list(cfg, "yes_words"));
    match (yes, no) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        _ => None,
    }
}

pub fn resolve(cfg: &Map<String, Value>, choices: &[String], text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return None;
    }
    if choices.is_empty() {
        return Some(trimmed.to_string());
    }
    let spoken = words(trimmed);
    let named: Vec<&String> = choices.iter().filter(|c| hits(&spoken, &[c.to_lowercase()])).collect();
    if let [one] = named.as_slice() {
        return Some((*one).clone());
    }
    if !named.is_empty() {
        return None;
    }
    let want = verdict(cfg, trimmed)?;
    let mapped: Vec<&String> = choices.iter().filter(|c| verdict(cfg, c) == Some(want)).collect();
    match mapped.as_slice() {
        [one] => Some((*one).clone()),
        _ => None,
    }
}

pub fn submit(home: &Path, which: &str, text: &str, via: &str) -> Result<Ask, Fail> {
    let settings = cfg::load(home);
    let target = if which.is_empty() || which == "last" {
        open_list(home).pop().ok_or_else(|| Fail::new("no question is waiting for an answer"))?
    } else {
        load(home, which).ok_or_else(|| Fail::new(format!("no question '{which}'")))?
    };
    if !target.open() {
        return Err(Fail::new(format!("question {} is already {}", target.id, target.state)));
    }
    let answer = resolve(&settings, &target.choices, text).ok_or_else(|| {
        let hint = if target.choices.is_empty() { "say something".to_string() } else { target.choices.join(" / ") };
        Fail::hint(
            format!("'{}' is not an answer to question {}", text.trim(), target.id),
            format!("answer with: {hint}"),
        )
    })?;
    let mut done = load(home, &target.id).unwrap_or(target);
    if done.state != "open" {
        return Err(Fail::new(format!("question {} is already {}", done.id, done.state)));
    }
    done.state = "answered".into();
    done.answer = Some(answer);
    done.via = via.to_string();
    write(home, &done)?;
    Ok(done)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("wisply-call-store-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    fn yes_no() -> Vec<String> {
        vec!["ja".into(), "nein".into()]
    }

    #[test]
    fn spoken_answers_map_onto_the_choices() {
        let c = cfg::defaults();
        let ch = yes_no();
        assert_eq!(resolve(&c, &ch, "Ja, mach das"), Some("ja".into()));
        assert_eq!(resolve(&c, &ch, "nein danke"), Some("nein".into()));
        assert_eq!(resolve(&c, &ch, "ok klar"), Some("ja".into()));
        assert_eq!(resolve(&c, &ch, "lieber nicht"), Some("nein".into()));
        assert_eq!(resolve(&c, &ch, "ja nein"), None);
        assert_eq!(resolve(&c, &ch, "vielleicht morgen"), None);
        assert_eq!(resolve(&c, &ch, "   "), None);
        assert_eq!(resolve(&c, &[], "  frei geschrieben "), Some("frei geschrieben".into()));
        let colors = vec!["rot".to_string(), "blau".to_string()];
        assert_eq!(resolve(&c, &colors, "nimm Blau"), Some("blau".into()));
        assert_eq!(resolve(&c, &colors, "ja"), None);
        let words = vec!["yes".to_string(), "no".to_string()];
        assert_eq!(resolve(&c, &words, "ja bitte"), Some("yes".into()));
    }

    #[test]
    fn the_first_answer_wins_and_closed_questions_stay_closed() {
        let home = temp("submit");
        let ask = create(&home, "Deploy?", &yes_no(), 60).unwrap();
        assert!(ask.open());
        assert_eq!(open_list(&home).len(), 1);
        assert!(submit(&home, &ask.id, "hmm", "test").is_err());
        let done = submit(&home, "last", "ja", "ntfy").unwrap();
        assert_eq!((done.answer.as_deref(), done.via.as_str()), (Some("ja"), "ntfy"));
        assert!(submit(&home, &ask.id, "nein", "voice").is_err());
        assert_eq!(load(&home, &ask.id).unwrap().answer.as_deref(), Some("ja"));
        assert!(open_list(&home).is_empty());
        assert!(submit(&home, "last", "ja", "x").is_err());
        let _ = fs::remove_dir_all(&home);
    }

    #[test]
    fn expired_and_unknown_questions_are_refused() {
        let home = temp("expire");
        let ask = create(&home, "Q", &[], 60).unwrap();
        close(&home, &ask.id, "expired");
        assert!(submit(&home, &ask.id, "x", "t").is_err());
        assert!(submit(&home, "zzzzzz", "x", "t").is_err());
        let _ = fs::remove_dir_all(&home);
    }
}
