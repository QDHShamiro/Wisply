use std::path::PathBuf;

use crate::fail::{Fail, Kind};

pub const VARIABLE: &str = "GEMINI_API_KEY";
const FILE: [&str; 3] = [".config", "watch", ".env"];
const FILE_SHOWN: &str = "~/.config/watch/.env";
const FREE_KEY: &str = "aistudio.google.com/apikey";
const QUOTES: [char; 2] = ['"', '\''];

pub fn file() -> PathBuf {
    FILE.iter().fold(wisply_proto::user_dir(), |path, part| path.join(part))
}

fn cleaned(value: &str) -> Option<String> {
    let value = value.trim().trim_matches(QUOTES).trim();
    (!value.is_empty()).then(|| value.to_string())
}

fn from_lines(text: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let (name, value) = line.split_once('=')?;
        if name.trim() == VARIABLE { cleaned(value) } else { None }
    })
}

fn missing() -> Fail {
    let hint = format!("set {VARIABLE} (free key: {FREE_KEY}) or put it in {FILE_SHOWN}");
    Fail::hint(Kind::MissingKey, format!("{VARIABLE} is not set"), hint)
}

pub fn read() -> Result<String, Fail> {
    let named = std::env::var(VARIABLE).ok().as_deref().and_then(cleaned);
    named.or_else(|| from_lines(&std::fs::read_to_string(file()).ok()?)).ok_or_else(missing)
}

pub fn rejected() -> Fail {
    let hint = format!("check {VARIABLE} in your environment or in {FILE_SHOWN}");
    Fail::hint(Kind::KeyRejected, format!("Gemini rejected the key in {VARIABLE}"), hint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_key_is_read_from_its_own_line() {
        let text = "OTHER=1\n# GEMINI_API_KEY=old\nGEMINI_API_KEY = \"abc-123\" \nGEMINI_API_KEY=later\n";
        assert_eq!(from_lines(text).as_deref(), Some("abc-123"));
        assert_eq!(from_lines("GEMINI_API_KEY='x y'").as_deref(), Some("x y"));
        assert_eq!(from_lines("GEMINI_API_KEY=\nGEMINI_API_KEY=second").as_deref(), Some("second"));
    }

    #[test]
    fn a_file_without_the_key_gives_nothing() {
        assert_eq!(from_lines(""), None);
        assert_eq!(from_lines("GEMINI_API_KEY=''\nGROQ_API_KEY=abc\nno equals sign"), None);
        assert_eq!(from_lines("MY_GEMINI_API_KEY=abc"), None);
    }

    #[test]
    fn the_errors_name_the_variable_and_where_it_goes() {
        let none = missing();
        assert_eq!(none.kind, Kind::MissingKey);
        assert_eq!(none.message, "GEMINI_API_KEY is not set");
        assert!(none.hint.contains("aistudio.google.com/apikey") && none.hint.contains("~/.config/watch/.env"));
        let bad = rejected();
        assert_eq!(bad.kind, Kind::KeyRejected);
        assert!(bad.message.contains(VARIABLE) && bad.hint.starts_with("check "));
        assert!(file().ends_with(".config/watch/.env"));
    }
}
