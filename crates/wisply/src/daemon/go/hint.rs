#![cfg_attr(not(windows), allow(dead_code))]

const ARROW: &str = "\u{2192}";
const DOTS: &str = "\u{2026}";
const UNKNOWN: &str = "not understood \u{00B7} Enter will not run it";
const PREFIX_ERR: &str = "err: ";
const SPLIT_HINT: &str = " -> try:";
const MAX_CHARS: usize = 96;

fn tidy(err: &str) -> String {
    let first = err.lines().next().unwrap_or("");
    let msg = first.strip_prefix(PREFIX_ERR).unwrap_or(first);
    msg.split(SPLIT_HINT).next().unwrap_or(msg).trim().to_string()
}

fn cut(s: &str) -> String {
    if s.chars().count() <= MAX_CHARS {
        return s.to_string();
    }
    let head: String = s.chars().take(MAX_CHARS - 1).collect();
    format!("{}{DOTS}", head.trim_end())
}

pub fn line(res: Result<Option<String>, String>) -> String {
    match res {
        Ok(Some(what)) => cut(&format!("{ARROW} {what}")),
        Ok(None) => UNKNOWN.to_string(),
        Err(e) => cut(&format!("{ARROW} {}", tidy(&e))),
    }
}

fn word_start(text: &str) -> usize {
    text.char_indices().rev().find(|(_, c)| c.is_whitespace()).map_or(0, |(i, c)| i + c.len_utf8())
}

fn pick(words: &[String], low: &str) -> Option<String> {
    let mut all = words.iter().filter(|w| !w.is_empty());
    if all.clone().any(|w| w.to_lowercase() == low) {
        return None;
    }
    all.find(|w| w.to_lowercase().starts_with(low)).cloned()
}

pub fn complete(text: &str, people: &[Vec<String>]) -> Option<String> {
    if text.ends_with(char::is_whitespace) {
        return None;
    }
    let (head, word) = text.split_at(word_start(text));
    let low = word.to_lowercase();
    if low.is_empty() {
        return None;
    }
    let mut hits = people.iter().filter_map(|words| pick(words, &low));
    let one = hits.next()?;
    if hits.next().is_some() {
        return None;
    }
    Some(format!("{head}{one} "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn people() -> Vec<Vec<String>> {
        let p = |w: &[&str]| w.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
        vec![p(&["Hanna", "hanni"]), p(&["Hannes"]), p(&["Mira", "mimi"]), p(&["Mirko"])]
    }

    #[test]
    fn tab_completes_a_unique_prefix_in_the_last_word() {
        assert_eq!(complete("schreib Mirk", &people()).as_deref(), Some("schreib Mirko "));
        assert_eq!(complete("schreib mim", &people()).as_deref(), Some("schreib mimi "));
        assert_eq!(complete("schreib Hanni", &people()), None);
        assert_eq!(complete("schreib hannes", &people()), None);
    }

    #[test]
    fn tab_stays_quiet_when_two_people_match() {
        assert_eq!(complete("schreib Mi", &people()), None);
        assert_eq!(complete("schreib Hann", &people()), None);
    }

    #[test]
    fn tab_needs_a_word_and_a_real_match() {
        assert_eq!(complete("", &people()), None);
        assert_eq!(complete("schreib ", &people()), None);
        assert_eq!(complete("schreib Zed", &people()), None);
        assert_eq!(complete("schreib Hann", &[]), None);
    }

    #[test]
    fn tab_ignores_empty_words_and_keeps_the_head() {
        let p = vec![vec!["Olaf".to_string(), String::new()]];
        assert_eq!(complete("an  ol", &p).as_deref(), Some("an  Olaf "));
        assert_eq!(complete("\u{00E4}h ol", &p).as_deref(), Some("\u{00E4}h Olaf "));
        assert_eq!(complete("OLA", &p).as_deref(), Some("Olaf "));
    }

    fn empty_home(tag: &str) -> std::path::PathBuf {
        let home = std::env::temp_dir().join(format!("wisply-go-hint-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        std::fs::create_dir_all(&home).expect("home");
        home
    }

    #[test]
    fn preview_matches_what_dry_would_say() {
        let home = empty_home("dry");
        let seen = crate::daemon::go::preview(&home, "  lauter ");
        assert!(seen.starts_with("\u{2192} would do: media key volumeup"), "{seen}");
        assert_eq!(crate::daemon::go::preview(&home, "   "), "");
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn preview_names_a_known_person_once() {
        let home = empty_home("known");
        crate::daemon::go::contacts::learn(&home, "Anna", "discord", "anna_99", None);
        let seen = crate::daemon::go::preview(&home, "schreib Anna hi");
        assert_eq!(seen, "\u{2192} would do: message to Anna (discord): \"hi\"");
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn preview_flags_unknown_people_and_nonsense() {
        let home = empty_home("unknown");
        let nobody = crate::daemon::go::preview(&home, "schreib Zorro hi");
        assert!(nobody.starts_with('\u{2192}') && !nobody.contains("err:"), "{nobody}");
        let nonsense = crate::daemon::go::preview(&home, "qwertz asdf yxcv");
        assert_eq!(nonsense, UNKNOWN);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn unknown_sentences_say_so() {
        assert_eq!(line(Ok(None)), UNKNOWN);
    }

    #[test]
    fn plans_get_an_arrow() {
        assert_eq!(
            line(Ok(Some("would do: media key volumeup x5".into()))),
            "\u{2192} would do: media key volumeup x5"
        );
    }

    #[test]
    fn errors_show_only_the_message() {
        let e = "err: send what? -> try: wisply go \"schreib Hanna hi\"".to_string();
        assert_eq!(line(Err(e)), "\u{2192} send what?");
        assert_eq!(line(Err("plain\nsecond".into())), "\u{2192} plain");
    }

    #[test]
    fn long_lines_are_cut() {
        let out = line(Ok(Some("x".repeat(300))));
        assert_eq!(out.chars().count(), MAX_CHARS);
        assert!(out.ends_with(DOTS));
    }
}
