use std::io::{BufRead, IsTerminal, Write};

use crate::media::{self, Word};
use crate::session::Session;
use crate::talk::{Out, note, refuse};

pub struct Term {
    json: bool,
}

impl Out for Term {
    fn note(&mut self, line: &str) {
        note(line);
    }

    fn word(&mut self, piece: &str) -> bool {
        if !self.json {
            print!("{piece}");
            let _ = std::io::stdout().flush();
        }
        true
    }

    fn end(&mut self) {
        if !self.json {
            println!();
        }
    }
}

impl Term {
    pub fn new(json: bool) -> Term {
        Term { json }
    }

    pub fn round(&mut self, session: &mut Session, words: &[Word]) -> bool {
        let config = session.config();
        let text = &config.talk.text;
        let (said, sources) = media::split(&config.talk, words, &|path| path.is_file());
        match session.turn(&said, &sources, self) {
            Ok(told) if self.json => {
                println!("{}", serde_json::to_string(&told).unwrap_or_default());
                true
            }
            Ok(told) => {
                let line = text
                    .stats
                    .replace("{first}", &format!("{:.1}", told.first_seconds))
                    .replace("{total}", &format!("{:.1}", told.total_seconds))
                    .replace("{rate}", &format!("{:.0}", told.answer_per_second))
                    .replace("{tokens}", &told.answer_tokens.to_string());
                note(&line);
                true
            }
            Err(failure) => {
                refuse(config, self.json, &failure);
                false
            }
        }
    }

    pub fn repl(&mut self, session: &mut Session) -> i32 {
        let config = session.config();
        let talk = &config.talk;
        let text = &talk.text;
        let stdin = std::io::stdin();
        let shown = stdin.is_terminal();
        if shown {
            note(&text.hello);
        }
        let mut failed = false;
        loop {
            if shown {
                eprint!("{}", text.prompt);
                let _ = std::io::stderr().flush();
            }
            let mut line = String::new();
            if !matches!(stdin.lock().read_line(&mut line), Ok(read) if read > 0) {
                break;
            }
            let line = line.trim();
            let low = line.to_lowercase();
            let named = |list: &[String]| list.contains(&low);
            if line.is_empty() {
                continue;
            }
            if named(&talk.quit_words) {
                break;
            }
            if named(&talk.new_words) {
                session.clear();
                note(&text.fresh);
                continue;
            }
            if named(&talk.help_words) {
                note(&text.help);
                continue;
            }
            failed |= !self.round(session, &media::words(line));
        }
        if shown {
            note(&text.bye);
        }
        i32::from(failed && !shown)
    }
}
