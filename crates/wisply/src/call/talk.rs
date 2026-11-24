#![cfg_attr(not(windows), allow(dead_code))]

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value};

use super::cfg::{self, fill};
use super::store::{self, Ask};
use super::text::now;

const CHECK_S: f64 = 1.0;

pub enum Act {
    Say(String),
    Leave,
}

pub struct Desk {
    home: PathBuf,
    settings: Map<String, Value>,
    current: Option<String>,
    announced: HashSet<String>,
    opened: bool,
    leaving: bool,
    quiet_since: f64,
    next_check: f64,
}

impl Desk {
    pub fn new(home: &Path) -> Desk {
        Desk {
            home: home.to_path_buf(),
            settings: cfg::load(home),
            current: None,
            announced: HashSet::new(),
            opened: false,
            leaving: false,
            quiet_since: now(),
            next_check: 0.0,
        }
    }

    fn line(&self, key: &str, pairs: &[(&str, &str)]) -> String {
        fill(&cfg::text(&self.settings, key), pairs)
    }

    fn choices(&self, ask: &Ask) -> String {
        if ask.choices.is_empty() {
            String::new()
        } else {
            self.line("say_choices", &[("choices", &ask.choices.join(", "))])
        }
    }

    fn announce(&mut self, ask: &Ask) -> Vec<Act> {
        let mut acts = Vec::new();
        if !self.opened {
            self.opened = true;
            acts.push(Act::Say(self.line("say_opening", &[])));
        }
        acts.push(Act::Say(self.line("say_ask", &[("question", &ask.question), ("choices", &self.choices(ask))])));
        self.announced.insert(ask.id.clone());
        self.current = Some(ask.id.clone());
        acts
    }

    pub fn tick(&mut self, last_activity: f64) -> Vec<Act> {
        if self.leaving || now() < self.next_check {
            return Vec::new();
        }
        self.next_check = now() + CHECK_S;
        if let Some(id) = self.current.clone() {
            if store::load(&self.home, &id).is_some_and(|a| a.open()) {
                return Vec::new();
            }
            self.current = None;
            self.quiet_since = now();
        }
        let fresh = store::open_list(&self.home).into_iter().find(|a| !self.announced.contains(&a.id));
        if let Some(ask) = fresh {
            return self.announce(&ask);
        }
        let linger = cfg::num(&self.settings, "voice_linger_s") as f64;
        if now() - self.quiet_since.max(last_activity) > linger {
            self.leaving = true;
            return vec![Act::Say(self.line("say_bye", &[])), Act::Leave];
        }
        Vec::new()
    }

    pub fn hear(&mut self, said: &str) -> Vec<Act> {
        let Some(ask) = self.current.as_deref().and_then(|id| store::load(&self.home, id)).filter(Ask::open) else {
            return vec![Act::Say(self.line("say_idle", &[]))];
        };
        match store::submit(&self.home, &ask.id, said, "voice") {
            Ok(done) => {
                self.current = None;
                self.quiet_since = now();
                let answer = done.answer.unwrap_or_default();
                vec![Act::Say(self.line("say_got", &[("answer", &answer)]))]
            }
            Err(_) => vec![Act::Say(self.line("say_unclear", &[("choices", &self.choices(&ask))]))],
        }
    }
}

#[cfg(windows)]
fn start(home: &Path, settings: &Map<String, Value>) {
    let target = cfg::text(settings, "voice_target");
    let _ = crate::voice::start_call(home, target.trim());
}

#[cfg(not(windows))]
fn start(_home: &Path, _settings: &Map<String, Value>) {}

pub fn open(home: &Path, settings: &Map<String, Value>, _ask: &Ask) {
    if cfg::on(settings, "voice") {
        start(home, settings);
    }
}

pub fn briefing(home: &Path, settings: &Map<String, Value>, spoken: &str) {
    if !cfg::on(settings, "voice") {
        return;
    }
    start(home, settings);
    #[cfg(windows)]
    crate::voice::Paths::new(home).drop_line(spoken);
    #[cfg(not(windows))]
    let _ = spoken;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("wisply-call-talk-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn said(acts: &[Act]) -> Vec<&str> {
        acts.iter().filter_map(|a| if let Act::Say(s) = a { Some(s.as_str()) } else { None }).collect()
    }

    #[test]
    fn the_desk_reads_the_question_hears_the_answer_and_hangs_up() {
        let home = temp("desk");
        cfg::set(&home, &["voice_linger_s=0".into()]).unwrap();
        let mut desk = Desk::new(&home);
        let ask = store::create(&home, "Soll ich deployen?", &["ja".into(), "nein".into()], 60).unwrap();
        let first = desk.tick(now());
        let lines = said(&first);
        assert!(
            lines[0].contains("hier ist Wisply")
                && lines[1].contains("Soll ich deployen?")
                && lines[1].contains("ja, nein")
        );
        desk.next_check = 0.0;
        assert!(desk.tick(now()).is_empty());
        assert!(said(&desk.hear("hm weiß nicht")).join(" ").contains("nicht verstanden"));
        assert!(said(&desk.hear("ja klar mach")).join(" ").contains("notiert: ja"));
        assert_eq!(store::load(&home, &ask.id).unwrap().via, "voice");
        desk.next_check = 0.0;
        let end = desk.tick(0.0);
        assert!(matches!(end.last(), Some(Act::Leave)));
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn free_text_answers_and_idle_talk() {
        let home = temp("free");
        let mut desk = Desk::new(&home);
        assert!(said(&desk.hear("hallo")).join(" ").contains("nichts offen"));
        let ask = store::create(&home, "Wie soll das Plugin heißen?", &[], 60).unwrap();
        desk.tick(now());
        assert!(said(&desk.hear("Skyforge")).join(" ").contains("Skyforge"));
        assert_eq!(store::load(&home, &ask.id).unwrap().answer.as_deref(), Some("Skyforge"));
        let _ = std::fs::remove_dir_all(&home);
    }
}
