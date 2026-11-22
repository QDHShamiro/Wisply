use crate::config::{Loops, Talk};
use crate::stream::Acc;
use crate::talk::Out;

fn boundary(text: &str, mut at: usize) -> usize {
    while at < text.len() && !text.is_char_boundary(at) {
        at += 1;
    }
    at.min(text.len())
}

fn worded(text: &str, from: usize) -> bool {
    text[boundary(text, from)..].chars().any(char::is_alphanumeric)
}

pub fn cut(text: &str, rule: &Loops) -> Option<usize> {
    if rule.repeats < 2 {
        return None;
    }
    let bytes = text.as_bytes();
    let len = bytes.len();
    for unit in 1..=rule.unit_chars.min(len / rule.repeats) {
        let span = unit * rule.repeats.max(rule.min_chars.div_ceil(unit));
        if span > len {
            continue;
        }
        let from = len - span;
        if !(from + unit..len).all(|at| bytes[at] == bytes[at - unit]) {
            continue;
        }
        let mut start = from;
        while start > 0 && bytes[start - 1] == bytes[start - 1 + unit] {
            start -= 1;
        }
        if worded(text, start) {
            return Some(boundary(text, start + unit));
        }
    }
    None
}

pub struct Guard<'a> {
    rule: &'a Loops,
    mark: &'a str,
    text: String,
    cut: Option<usize>,
}

impl<'a> Guard<'a> {
    pub fn new(talk: &'a Talk) -> Guard<'a> {
        Guard { rule: &talk.loops, mark: &talk.text.looped, text: String::new(), cut: None }
    }

    pub fn feed(&mut self, piece: &str) -> bool {
        self.text.push_str(piece);
        if self.cut.is_none() {
            self.cut = cut(&self.text, self.rule);
        }
        self.cut.is_none()
    }

    pub fn settle(&self, mut acc: Acc, out: &mut dyn Out) -> Acc {
        let Some(at) = self.cut else {
            return acc;
        };
        let keep = self.text[..at].trim_end().len();
        acc.text = format!("{}{}", &self.text[..keep], self.mark);
        acc.finish = self.rule.finish.clone();
        acc.stopped = false;
        out.word(self.mark);
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::{Guard, cut};
    use crate::config::{Config, Loops};
    use crate::stream::Acc;
    use crate::talk::Out;

    fn config() -> Config {
        toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses")
    }

    fn rule() -> Loops {
        Loops { repeats: 3, min_chars: 150, unit_chars: 400, finish: "loop".to_string() }
    }

    #[derive(Default)]
    struct Seen(String);

    impl Out for Seen {
        fn note(&mut self, _line: &str) {}

        fn word(&mut self, piece: &str) -> bool {
            self.0.push_str(piece);
            true
        }
    }

    #[test]
    fn a_word_that_repeats_without_end_is_cut_after_its_first_use() {
        let text = format!("Tiere: Camino, {}", "Pinguin, ".repeat(20));
        let at = cut(&text, &rule()).expect("loop");
        assert_eq!(&text[..at], "Tiere: Camino, Pinguin");
        let partial = format!("{text}Pin");
        let at = cut(&partial, &rule()).expect("loop");
        assert_eq!(&partial[..at], "Tiere: Camino, Pinguin");
    }

    #[test]
    fn a_repeated_sentence_with_umlauts_keeps_one_copy() {
        let sentence = "Camina ist ein Fuchs, der „Caminand“ heißt und größer wird. ";
        let text = format!("Im Video: {}", sentence.repeat(3));
        let at = cut(&text, &rule()).expect("loop");
        assert!(text.is_char_boundary(at));
        assert_eq!(text[..at].matches("Camina ist").count(), 1);
        assert!(text[..at].starts_with("Im Video: Camina ist ein Fuchs"));
        assert!(cut(&format!("Im Video: {}", sentence.repeat(2)), &rule()).is_none());
    }

    #[test]
    fn normal_answers_lines_and_short_echoes_are_left_alone() {
        let counting: Vec<String> = (1..=120).map(|n| n.to_string()).collect();
        let table = format!("| a | b |\n|{}", "---|".repeat(60));
        for text in [
            "Das Video zeigt das Programm Blender.".to_string(),
            "Ja, ja, ja, ja. Nein, nein, nein.".to_string(),
            "Hallo ".repeat(20),
            "-".repeat(600),
            "=== ".repeat(120),
            "— ".repeat(200),
            counting.join(", "),
            table,
            String::new(),
        ] {
            assert_eq!(cut(&text, &rule()), None, "{text}");
        }
    }

    #[test]
    fn the_rule_can_be_switched_off() {
        let text = "Pinguin, ".repeat(60);
        for repeats in [0, 1] {
            let off = Loops { repeats, ..rule() };
            assert_eq!(cut(&text, &off), None);
        }
        let short = Loops { unit_chars: 4, ..rule() };
        assert_eq!(cut(&text, &short), None);
        assert!(cut(&text, &rule()).is_some());
    }

    #[test]
    fn the_guard_stops_the_stream_and_marks_the_answer() {
        let config = config();
        assert!(config.talk.loops.repeats >= 2);
        assert!(config.talk.loops.unit_chars > 0);
        assert!(!config.talk.loops.finish.is_empty());
        assert!(!config.talk.text.looped.trim().is_empty());
        let mut guard = Guard::new(&config.talk);
        let mut fed = String::from("Tiere: ");
        assert!(guard.feed("Tiere: "));
        let mut pieces = 0;
        while guard.feed("Gnu, ") {
            fed.push_str("Gnu, ");
            pieces += 1;
            assert!(pieces < 1000);
        }
        fed.push_str("Gnu, ");
        assert!(!guard.feed("Gnu, "));
        let mut seen = Seen::default();
        let acc = Acc { text: fed, finish: "length".to_string(), stopped: true, ..Acc::default() };
        let told = guard.settle(acc, &mut seen);
        assert_eq!(told.text, format!("Tiere: Gnu,{}", config.talk.text.looped));
        assert_eq!(told.finish, config.talk.loops.finish);
        assert!(!told.stopped);
        assert_eq!(seen.0, config.talk.text.looped);
    }

    #[test]
    fn a_calm_answer_passes_the_guard_untouched() {
        let config = config();
        let mut guard = Guard::new(&config.talk);
        for piece in ["Das ", "ist ", "ein ", "Lama ", "und ", "ein ", "Pinguin."] {
            assert!(guard.feed(piece));
        }
        let acc =
            Acc { text: "Das ist ein Lama und ein Pinguin.".to_string(), finish: "stop".to_string(), ..Acc::default() };
        let mut seen = Seen::default();
        let told = guard.settle(acc.clone(), &mut seen);
        assert_eq!(told, acc);
        assert!(seen.0.is_empty());
    }
}
