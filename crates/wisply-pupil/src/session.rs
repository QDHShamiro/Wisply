use std::panic::{AssertUnwindSafe, catch_unwind, resume_unwind};
use std::path::{Path, PathBuf};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use serde_json::{Value, json};

use crate::chat;
use crate::config::{Config, Model};
use crate::door;
use crate::job::{Failure, Reply};
use crate::load::{self, Temp};
use crate::loops::Guard;
use crate::media::{self, Kind, Loaded, Source};
use crate::route::{self, Choice};
use crate::server::{self, Busy};
use crate::stream::{self, Piece};
use crate::talk::{Mode, Out, Spent, Told, Turn, body, drop_count, picture_part, push_text, text_part};
use crate::trace::{self, Log, Record, Tape};
use crate::watch;

#[path = "session_video.rs"]
mod session_video;

type Clip = (String, PathBuf);

#[derive(Default)]
struct Draft {
    parts: Vec<Value>,
    notes: Vec<String>,
    kinds: Vec<Kind>,
    cost: u32,
    spent: Spent,
}

enum Seen {
    Parts(Vec<Value>, u32, Spent),
    Whole(String, Box<Reply>),
}

pub struct Session<'a> {
    config: &'a Config,
    config_path: &'a Path,
    temp: &'a Temp,
    mode: Mode,
    model: Model,
    turns: Vec<Turn>,
    dropped: usize,
    log: Log,
    traces: Vec<JoinHandle<()>>,
}

impl<'a> Session<'a> {
    pub fn open(
        config: &'a Config,
        config_path: &'a Path,
        temp: &'a Temp,
        model: Option<&str>,
        mode: Mode,
    ) -> Result<Session<'a>, Failure> {
        let fail = |code: &str, fields: &[(&str, &str)]| Failure::new(config, code, fields);
        let Some(found) = config.model(model).cloned() else {
            let name = model.unwrap_or_default();
            let names = config.model_names();
            return Err(fail("model_unknown", &[("name", name), ("names", &names)]));
        };
        if !found.file.is_file() {
            let path = found.file.display().to_string();
            return Err(fail("model_missing", &[("path", &path)]));
        }
        Ok(Session {
            config,
            config_path,
            temp,
            mode,
            model: found,
            turns: Vec::new(),
            dropped: 0,
            log: Log::default(),
            traces: Vec::new(),
        })
    }

    pub fn config(&self) -> &'a Config {
        self.config
    }

    pub fn set(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn clear(&mut self) {
        self.turns.clear();
        self.dropped = 0;
    }

    pub fn total(&self) -> usize {
        self.dropped + self.turns.len()
    }

    pub fn keep(&mut self, count: usize) -> bool {
        match count.checked_sub(self.dropped) {
            Some(kept) if kept <= self.turns.len() => {
                self.turns.truncate(kept);
                true
            }
            _ => false,
        }
    }

    pub fn seed(&mut self, before: usize, pairs: &[(String, String)]) {
        self.dropped = before;
        self.turns = pairs
            .iter()
            .map(|(user, reply)| Turn {
                cost: self.count(user) + self.count(reply),
                user: json!({"role": "user", "content": user}),
                reply: json!({"role": "assistant", "content": reply}),
            })
            .collect();
    }

    fn limit(&self) -> u32 {
        let talk = &self.config.talk;
        if talk.max_tokens > 0 { talk.max_tokens } else { self.config.sampling.max_tokens }
    }

    fn context(&self) -> u32 {
        server::fit_context(self.config.server.context, server::live_context(self.config))
    }

    fn room(&self) -> u32 {
        self.context().saturating_sub(self.limit()).saturating_sub(self.config.budget.spare_tokens)
    }

    fn count(&self, text: &str) -> u32 {
        chat::estimate_tokens(text, &self.config.budget)
    }

    fn text_room(&self, spent: u32, head: &str) -> usize {
        let free = self.room().saturating_sub(self.base()).saturating_sub(spent).saturating_sub(self.count(head));
        chat::fit_chars(free, &self.config.budget)
    }

    fn base(&self) -> u32 {
        self.count(&self.config.talk.system)
    }

    fn known(&self) -> u32 {
        self.turns.iter().fold(self.base(), |sum, turn| sum.saturating_add(turn.cost))
    }

    fn frame_cost(&self, size: (u32, u32)) -> u32 {
        let frames = &self.config.frames;
        let patch = frames.patch.max(1);
        (size.0 / patch) * (size.1 / patch) + frames.label_tokens
    }

    fn forget(&mut self, fresh: u32, room: u32) -> Result<(), Failure> {
        let costs: Vec<u32> = self.turns.iter().map(|turn| turn.cost).collect();
        if let Some(dropped) = drop_count(&costs, self.base(), fresh, room) {
            self.turns.drain(..dropped);
            self.dropped += dropped;
            return Ok(());
        }
        let tokens = self.base().saturating_add(fresh).to_string();
        let context = self.context().to_string();
        Err(Failure::new(self.config, "talk_full", &[("tokens", &tokens), ("context", &context)]))
    }

    fn thinks(&self, question: &str, out: &mut dyn Out) -> bool {
        let text = &self.config.talk.text;
        match route::choose(self.mode.think, self.mode.auto, &self.config.think_auto, question) {
            Choice::Fixed(on) => on,
            Choice::Auto(on) => {
                out.note(if on { &text.auto_on } else { &text.auto_off });
                on
            }
            Choice::Broken(detail) => {
                out.note(&text.auto_broken.replace("{detail}", &detail));
                false
            }
        }
    }

    fn wait(&self, out: &mut dyn Out) -> Result<(), Failure> {
        let talk = &self.config.talk;
        if self.mode.now || talk.hold_poll_seconds <= 0.0 {
            return Ok(());
        }
        let mut told = false;
        while let Some(label) = door::held(talk) {
            if !told {
                out.note(&talk.text.hold.replace("{label}", &label));
                told = true;
            }
            if out.gone() {
                return Err(Failure::new(self.config, "talk_stopped", &[]));
            }
            std::thread::sleep(Duration::from_secs_f64(talk.hold_poll_seconds));
        }
        Ok(())
    }

    fn add(&self, draft: &mut Draft, loaded: Loaded) {
        let text = &self.config.talk.text;
        match loaded {
            Loaded::Picture { name, jpeg, size } => {
                draft.cost += self.frame_cost(size);
                push_text(&mut draft.parts, &text.picture.replace("{name}", &name));
                draft.parts.push(picture_part(&jpeg));
                draft.kinds.push(Kind::Picture);
            }
            Loaded::Words { name, kind, text: words } => {
                let label = match kind {
                    Kind::Sound => &text.sound,
                    Kind::Page => &text.page,
                    _ => &text.file,
                };
                let head = label.replace("{name}", &name);
                let words = media::clip(&words, self.text_room(draft.cost, &head).max(1), &text.cut);
                let block = format!("{head}\n{words}");
                draft.cost += self.count(&block);
                push_text(&mut draft.parts, &block);
                draft.notes.push(block);
                draft.kinds.push(kind);
            }
            Loaded::Video { name, about, .. } if !about.is_empty() => {
                let block = text.about.replace("{name}", &name);
                let block = block.replace("{about}", &about);
                draft.cost += self.count(&block);
                push_text(&mut draft.parts, &block);
                draft.notes.push(block);
            }
            Loaded::Video { .. } => {}
        }
    }

    fn gather(&self, sources: &[Source], out: &mut dyn Out) -> Result<(Draft, Vec<Clip>), Failure> {
        let mut draft = Draft::default();
        let mut clips = Vec::new();
        for source in sources {
            let shown = match source {
                Source::File(path) => path
                    .file_name()
                    .map_or_else(|| path.display().to_string(), |name| name.to_string_lossy().into_owned()),
                Source::Link(url) => url.clone(),
            };
            out.note(&self.config.talk.text.working.replace("{name}", &shown));
            let begun = Instant::now();
            let loaded = load::load(self.config, self.temp, source)?;
            draft.spent.load_seconds += begun.elapsed().as_secs_f64();
            if let Loaded::Video { name, path, .. } = &loaded {
                clips.push((name.clone(), path.clone()));
            }
            self.add(&mut draft, loaded);
        }
        Ok((draft, clips))
    }

    fn say(&mut self, question: String, mut draft: Draft, total: Instant, out: &mut dyn Out) -> Result<Told, Failure> {
        let config = self.config;
        let afk = self.mode.afk;
        let fresh = draft.cost + self.count(&question);
        self.forget(fresh, self.room())?;
        let content = if draft.parts.is_empty() {
            json!(&question)
        } else {
            draft.parts.push(text_part(&question));
            Value::Array(draft.parts)
        };
        let user = json!({"role": "user", "content": content});
        self.wait(out)?;
        if !server::healthy(config) {
            out.note(&config.talk.text.loading);
        }
        door::attach(config, self.config_path, &self.model, afk, self.mode.now)?;
        let think = self.thinks(&question, out);
        let limit = self.limit();
        let sent = body(config, &self.model.name, &self.turns, &user, limit, think);
        self.log.think = Some(think);
        if trace::on(&config.talk.trace) {
            self.log.sent = Some(sent.clone());
        }
        let prep_seconds = total.elapsed().as_secs_f64();
        let mut guard = Guard::new(&config.talk);
        let answer = {
            let _busy = Busy::mark(config);
            stream::send(config, &sent, &mut |piece| match piece {
                Piece::Word(word) => out.word(word) && guard.feed(word),
                Piece::Thought(thought) => out.thought(thought),
            })
        };
        let answer = answer.map(|acc| guard.settle(acc, out));
        out.end();
        let acc = answer.map_err(|detail| watch::request_failure(config, afk, detail))?;
        let text = acc.text.trim().to_string();
        if text.is_empty() {
            let code = if acc.stopped { "talk_stopped" } else { "empty_answer" };
            return Err(Failure::new(config, code, &[]));
        }
        let seen = acc.prompt_tokens + acc.cached_tokens + acc.answer_tokens;
        let cost = match u32::try_from(seen).unwrap_or(u32::MAX) {
            0 => fresh + self.count(&text),
            seen => seen.saturating_sub(self.known()),
        };
        let reply = json!({"role": "assistant", "content": text});
        self.turns.push(Turn { user, reply, cost });
        Ok(Told {
            ok: true,
            text,
            finish: acc.finish,
            model: self.model.name.clone(),
            kinds: draft.kinds,
            prep_seconds,
            spent: draft.spent,
            first_seconds: prep_seconds + acc.first_seconds,
            total_seconds: total.elapsed().as_secs_f64(),
            prompt_tokens: acc.prompt_tokens,
            cached_tokens: acc.cached_tokens,
            answer_tokens: acc.answer_tokens,
            answer_per_second: acc.answer_per_second,
            stopped: acc.stopped,
        })
    }

    pub fn turn(&mut self, said: &str, sources: &[Source], out: &mut dyn Out) -> Result<Told, Failure> {
        let config = self.config;
        let spec = &config.talk.trace;
        if !trace::on(spec) {
            return self.respond(said, sources, out);
        }
        self.log = Log::default();
        let mut tape = Tape::new(out);
        let result = catch_unwind(AssertUnwindSafe(|| self.respond(said, sources, &mut tape)));
        let (told, error) = match &result {
            Ok(Ok(told)) => (serde_json::to_value(told).ok(), None),
            Ok(Err(failure)) => (None, serde_json::to_value(failure).ok()),
            Err(panic) => (None, Some(json!({"panic": crate::post::crashed(panic.as_ref())}))),
        };
        let record = Record {
            said: said.to_string(),
            sources: sources.to_vec(),
            mode: self.mode,
            model: self.model.name.clone(),
            config: self.config_path.to_path_buf(),
            log: std::mem::take(&mut self.log),
            told,
            error,
        };
        self.traces.retain(|handle| !handle.is_finished());
        self.traces.push(trace::spawn(spec, tape.done(), record));
        result.unwrap_or_else(|panic| resume_unwind(panic))
    }

    fn respond(&mut self, said: &str, sources: &[Source], out: &mut dyn Out) -> Result<Told, Failure> {
        let total = Instant::now();
        let config = self.config;
        if let Some(failure) = watch::game_failure(config, self.mode.afk) {
            return Err(failure);
        }
        let question = if said.trim().is_empty() { config.talk.text.question.clone() } else { said.to_string() };
        if !sources.is_empty() {
            self.wait(out)?;
        }
        let _turn = Busy::mark(config);
        let (mut draft, clips) = self.gather(sources, out)?;
        self.log.media = clips.iter().map(|clip| clip.1.clone()).collect();
        if let Some((answer, reply)) = self.clips(&clips, &question, &mut draft, out)? {
            return Ok(self.whole(&question, answer, &reply, draft, out));
        }
        self.say(question, draft, total, out)
    }
}

impl Drop for Session<'_> {
    fn drop(&mut self) {
        for handle in self.traces.drain(..) {
            let _ = handle.join();
        }
    }
}
