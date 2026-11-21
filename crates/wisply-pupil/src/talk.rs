use std::io::IsTerminal;
use std::path::Path;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde::Serialize;
use serde_json::{Value, json};

use crate::chat;
use crate::config::{Config, TalkText};
use crate::dry;
use crate::job::Failure;
use crate::load::Temp;
use crate::media::{self, Kind, Word};
use crate::session::Session;
use crate::site;
use crate::term::Term;
use crate::warm;
use crate::watch::Stage;

#[derive(Debug, Default, PartialEq)]
pub struct Options {
    pub afk: bool,
    pub json: bool,
    pub now: bool,
    pub think: bool,
    pub auto: bool,
    pub page: bool,
    pub model: Option<String>,
    pub words: Vec<String>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Mode {
    pub afk: bool,
    pub now: bool,
    pub think: bool,
    pub auto: bool,
}

impl Options {
    pub fn mode(&self) -> Mode {
        Mode { afk: self.afk, now: self.now, think: self.think, auto: self.auto }
    }
}

pub trait Out {
    fn note(&mut self, line: &str);
    fn word(&mut self, piece: &str) -> bool;
    fn thought(&mut self, _piece: &str) -> bool {
        true
    }
    fn end(&mut self) {}
    fn gone(&mut self) -> bool {
        false
    }
}

pub fn options(args: &[String]) -> Result<Options, String> {
    let mut found = Options::default();
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--afk" => found.afk = true,
            "--json" => found.json = true,
            "--now" => found.now = true,
            "--think" => found.think = true,
            "--auto-think" => found.auto = true,
            "--page" => found.page = true,
            "--model" => {
                let name = rest.next().ok_or("--model needs a name")?;
                found.model = Some(name.clone()).filter(|m| !m.is_empty());
            }
            flag if flag.starts_with("--") => return Err(format!("unknown option {flag}")),
            _ => found.words.push(arg.clone()),
        }
    }
    Ok(found)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize)]
pub struct Spent {
    pub load_seconds: f64,
    pub ear_seconds: f64,
    pub frames_seconds: f64,
}

impl Spent {
    pub fn add(&mut self, other: Spent) {
        self.load_seconds += other.load_seconds;
        self.ear_seconds += other.ear_seconds;
        self.frames_seconds += other.frames_seconds;
    }
}

pub fn heard_room(seconds: f64, per_minute: f64) -> u32 {
    (seconds.max(0.0) / 60.0 * per_minute.max(0.0)).ceil() as u32
}

fn part_line(template: &str, at: usize, of: usize) -> String {
    template.replace("{at}", &at.to_string()).replace("{of}", &of.to_string())
}

pub fn told(text: &TalkText, out: &mut dyn Out, stage: Stage) -> bool {
    let line = match stage {
        Stage::Hearing => text.hearing.clone(),
        Stage::Part(at, of) => part_line(&text.part, at, of),
        Stage::Skipped(at, of, why) => part_line(&text.part_skipped, at, of).replace("{why}", why),
    };
    out.note(&line);
    !out.gone()
}

pub fn crowded(reserved: u32, room: u32, share: f64) -> bool {
    f64::from(reserved) > f64::from(room) * share.clamp(0.0, 1.0)
}

#[derive(Debug, Serialize)]
pub struct Told {
    pub ok: bool,
    pub text: String,
    pub finish: String,
    pub model: String,
    pub kinds: Vec<Kind>,
    pub prep_seconds: f64,
    #[serde(flatten)]
    pub spent: Spent,
    pub first_seconds: f64,
    pub total_seconds: f64,
    pub prompt_tokens: u64,
    pub cached_tokens: u64,
    pub answer_tokens: u64,
    pub answer_per_second: f64,
    pub stopped: bool,
}

pub struct Turn {
    pub user: Value,
    pub reply: Value,
    pub cost: u32,
}

pub fn drop_count(costs: &[u32], base: u32, fresh: u32, room: u32) -> Option<usize> {
    let fixed = u64::from(base) + u64::from(fresh);
    let mut kept: u64 = costs.iter().map(|cost| u64::from(*cost)).sum();
    let mut dropped = 0;
    while fixed + kept > u64::from(room) {
        kept -= u64::from(*costs.get(dropped)?);
        dropped += 1;
    }
    Some(dropped)
}

pub fn answer_of(raw: &str, key: &str) -> String {
    serde_json::from_str::<Value>(raw)
        .ok()
        .and_then(|value| value.get(key).and_then(Value::as_str).map(str::to_string))
        .unwrap_or_else(|| raw.trim().to_string())
}

pub fn spoken(args: &[String], is_source: &dyn Fn(&str) -> bool) -> Vec<Word> {
    args.iter().flat_map(|arg| if is_source(arg) { vec![Word::plain(arg)] } else { media::words(arg) }).collect()
}

pub(crate) fn text_part(text: &str) -> Value {
    json!({"type": "text", "text": text})
}

pub(crate) fn push_text(parts: &mut Vec<Value>, text: &str) {
    if !text.trim().is_empty() {
        parts.push(text_part(text));
    }
}

pub(crate) fn picture_part(jpeg: &[u8]) -> Value {
    let url = format!("data:image/jpeg;base64,{}", STANDARD.encode(jpeg));
    json!({"type": "image_url", "image_url": {"url": url}})
}

pub fn body(config: &Config, model: &str, turns: &[Turn], user: &Value, limit: u32, think: bool) -> Value {
    let talk = &config.talk;
    let mut messages = Vec::new();
    if !talk.system.trim().is_empty() {
        messages.push(json!({"role": "system", "content": talk.system}));
    }
    for turn in turns {
        messages.push(turn.user.clone());
        messages.push(turn.reply.clone());
    }
    messages.push(user.clone());
    let mut sent = chat::base(config, model, Value::Array(messages), limit);
    sent["stream"] = json!(true);
    if think {
        sent["chat_template_kwargs"] = json!({"enable_thinking": true});
        for (field, value) in &talk.think_fields {
            sent[field] = value.clone();
        }
    }
    if let Some(multiplier) = talk.dry_multiplier {
        dry::set(&mut sent, &config.sampling, multiplier);
    }
    sent
}

pub(crate) fn refuse(config: &Config, json: bool, failure: &Failure) {
    if json {
        println!("{}", serde_json::to_string(failure).unwrap_or_default());
        return;
    }
    let line = config.talk.text.error.replace("{message}", &failure.message).replace("{hint}", &failure.hint);
    eprintln!("{}", line.trim());
}

pub(crate) fn note(line: &str) {
    if !line.trim().is_empty() {
        eprintln!("{line}");
    }
}

pub fn run(config: &Config, config_path: &Path, args: &[String]) -> i32 {
    let options = match options(args) {
        Ok(options) => options,
        Err(detail) => {
            eprintln!("{detail}");
            note(&config.talk.text.help);
            return 2;
        }
    };
    let fail = |failure: &Failure| {
        refuse(config, options.json, failure);
        1
    };
    let temp = match Temp::new(&config.talk) {
        Ok(temp) => temp,
        Err(detail) => {
            let fields = [("name", config.talk.temp_name.as_str()), ("detail", &detail)];
            return fail(&Failure::new(config, "talk_read", &fields));
        }
    };
    let model = options.model.as_deref();
    let mut session = match Session::open(config, config_path, &temp, model, options.mode()) {
        Ok(session) => session,
        Err(failure) => return fail(&failure),
    };
    if options.page {
        drop(session);
        warm::start(config, options.afk);
        return site::serve(config, config_path, &temp, &options);
    }
    let mut term = Term::new(options.json);
    if options.words.is_empty() {
        if std::io::stdin().is_terminal() {
            warm::start(config, options.afk);
        }
        return term.repl(&mut session);
    }
    let talk = &config.talk;
    let is_source = |arg: &str| media::is_link(talk, arg) || Path::new(arg).is_file();
    let words = spoken(&options.words, &is_source);
    i32::from(!term.round(&mut session, &words))
}

#[cfg(test)]
mod tests {
    use super::{Options, Out, Spent, Turn, answer_of, body, crowded, drop_count, heard_room, options, spoken, told};
    use crate::config::Config;
    use crate::watch::Stage;

    struct Notes(Vec<String>, bool);

    impl Out for Notes {
        fn note(&mut self, line: &str) {
            self.0.push(line.to_string());
        }
        fn word(&mut self, _piece: &str) -> bool {
            true
        }
        fn gone(&mut self) -> bool {
            self.1
        }
    }

    #[test]
    fn long_video_steps_are_told_and_stop_once_the_page_is_gone() {
        let text = config().talk.text;
        assert!(text.part.contains("{at}") && text.part.contains("{of}"));
        assert!(text.part_skipped.contains("{why}") && !text.hearing.is_empty());
        let mut notes = Notes(Vec::new(), false);
        assert!(told(&text, &mut notes, Stage::Part(2, 12)));
        assert!(told(&text, &mut notes, Stage::Skipped(3, 12, "kaputt")));
        assert!(told(&text, &mut notes, Stage::Hearing));
        assert_eq!(notes.0[0], text.part.replace("{at}", "2").replace("{of}", "12"));
        assert!(notes.0[1].contains("kaputt") && notes.0[1].contains("3/12"));
        assert_eq!(notes.0[2], text.hearing);
        notes.1 = true;
        assert!(!told(&text, &mut notes, Stage::Part(4, 12)));
        assert_eq!(notes.0.len(), 4);
    }
    use serde_json::json;

    fn config() -> Config {
        toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses")
    }

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|a| a.to_string()).collect()
    }

    #[test]
    fn flags_and_words_are_told_apart() {
        let found =
            options(&args(&["--json", "Was ist das?", "--model", "2b", "a.png", "--now", "--auto-think", "--page"]))
                .expect("valid");
        assert!(found.mode().auto && !found.mode().think);
        assert_eq!(
            found,
            Options {
                json: true,
                now: true,
                auto: true,
                page: true,
                model: Some("2b".to_string()),
                words: args(&["Was ist das?", "a.png"]),
                ..Options::default()
            }
        );
        assert!(options(&args(&["--model"])).is_err());
        assert!(options(&args(&["--jsn"])).is_err());
        assert_eq!(options(&[]).expect("valid"), Options::default());
    }

    #[test]
    fn a_question_in_one_argument_is_cut_but_a_source_stays_whole() {
        let list = spoken(&args(&["Was ist \"das hier\"?", "C:\\Mein Ordner\\a b.png"]), &|arg| arg.ends_with(".png"));
        let values: Vec<&str> = list.iter().map(|w| w.value.as_str()).collect();
        assert_eq!(values, ["Was", "ist", "das hier?", "C:\\Mein Ordner\\a b.png"]);
    }

    #[test]
    fn the_oldest_turns_go_first_when_the_context_is_full() {
        assert_eq!(drop_count(&[100, 200, 300], 10, 50, 1000), Some(0));
        assert_eq!(drop_count(&[100, 200, 300], 10, 400, 1000), Some(1));
        assert_eq!(drop_count(&[100, 200, 300], 10, 500, 1000), Some(2));
        assert_eq!(drop_count(&[100, 200, 300], 10, 700, 1000), Some(3));
        assert_eq!(drop_count(&[100, 200, 300], 10, 991, 1000), None);
        assert_eq!(drop_count(&[], 0, 0, 0), Some(0));
        assert_eq!(drop_count(&[u32::MAX, u32::MAX], 1, 1, 10), Some(2));
    }

    #[test]
    fn the_transcript_gets_room_by_the_minute_and_times_add_up() {
        assert_eq!(heard_room(60.0, 320.0), 320);
        assert_eq!(heard_room(45.0, 320.0), 240);
        assert_eq!(heard_room(1.0, 320.0), 6);
        assert_eq!(heard_room(-5.0, 320.0), 0);
        assert_eq!(heard_room(60.0, -1.0), 0);
    }

    #[test]
    fn long_transcripts_crowd_out_the_pictures() {
        let room = 32768 - 1500;
        assert!(!crowded(heard_room(600.0, 320.0), room, 0.5));
        assert!(crowded(heard_room(6000.0, 320.0), room, 0.5));
        assert!(crowded(heard_room(60.0, 320.0), 0, 0.5));
        assert!(!crowded(0, 0, 0.5));
        assert!(crowded(1, room, 0.0));
        assert!(!crowded(room, room, 7.0));
        let mut spent = Spent { load_seconds: 1.0, ear_seconds: 2.0, frames_seconds: 3.0 };
        spent.add(Spent { load_seconds: 0.5, ear_seconds: 0.25, frames_seconds: 0.125 });
        assert_eq!(spent, Spent { load_seconds: 1.5, ear_seconds: 2.25, frames_seconds: 3.125 });
    }

    #[test]
    fn a_long_video_answer_is_taken_out_of_its_json() {
        assert_eq!(answer_of(r#"{"answer": "Ein Hund."}"#, "answer"), "Ein Hund.");
        assert_eq!(answer_of(" plain text ", "answer"), "plain text");
        assert_eq!(answer_of(r#"{"other": 1}"#, "answer"), r#"{"other": 1}"#);
    }

    #[test]
    fn the_chat_request_streams_and_keeps_the_history_in_order() {
        let mut config = config();
        config.talk.system = "Be brief.".to_string();
        config.talk.dry_multiplier = Some(0.0);
        let turns = [Turn {
            user: json!({"role": "user", "content": "Hi"}),
            reply: json!({"role": "assistant", "content": "Hello"}),
            cost: 9,
        }];
        let user = json!({"role": "user", "content": "And now?"});
        let sent = body(&config, "2b", &turns, &user, 77, false);
        assert_eq!(sent["stream"], true);
        assert_eq!(sent["max_tokens"], 77);
        assert!(sent.get("dry_multiplier").is_none());
        let roles: Vec<&str> =
            sent["messages"].as_array().expect("messages").iter().filter_map(|m| m["role"].as_str()).collect();
        assert_eq!(roles, ["system", "user", "assistant", "user"]);
        assert_eq!(sent["chat_template_kwargs"]["enable_thinking"], config.sampling.enable_thinking);
        let thinking = body(&config, "2b", &[], &user, 77, true);
        assert_eq!(thinking["chat_template_kwargs"]["enable_thinking"], true);
        for (field, value) in &config.talk.think_fields {
            assert_eq!(&thinking[field], value);
            assert!(sent.get(field).is_none());
        }
        assert_eq!(thinking["messages"].as_array().map(Vec::len), Some(2));
    }
}
