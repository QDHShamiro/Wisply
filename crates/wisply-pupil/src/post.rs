use std::fs::File;
use std::io::Read;
use std::net::{Shutdown, TcpStream};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use crate::config::Talk;
use crate::door;
use crate::http::{self, Request};
use crate::job::Failure;
use crate::load;
use crate::media::{self, Kind, Source};
use crate::session::Session;
use crate::site::{Site, lock};
use crate::talk::Mode;
use crate::watch;
use crate::wire::{Wire, deny};

pub type Refusal = (u16, Failure);

pub const MB: u64 = 1024 * 1024;
const STORED: &str = "file";
const UPLOAD: &str = "u";
const NOW: &str = "1";
const UNSAFE: [char; 4] = ['?', '#', '/', '\\'];

pub fn stored(talk: &Talk, given: &str) -> Option<(String, Kind, String)> {
    let plain: String = given.chars().map(|c| if UNSAFE.contains(&c) || c.is_control() { '_' } else { c }).collect();
    let kind = media::kind_of(talk, &plain)?;
    let kept = media::safe_name(&format!("/{plain}"), "");
    let file = if media::kind_of(talk, &kept) == Some(kind) {
        kept
    } else {
        format!("{STORED}.{}", media::extension(&plain)?)
    };
    Some((plain, kind, file))
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Said {
    chat: String,
    text: String,
    files: Vec<String>,
    think: bool,
    auto: bool,
    now: bool,
    redo: bool,
    before: usize,
    history: Vec<(String, String)>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct Gone {
    chat: String,
}

pub struct Upload {
    chat: String,
    dir: PathBuf,
    path: PathBuf,
}

struct Writing<'s>(&'s AtomicBool);

impl Drop for Writing<'_> {
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

impl<'a> Site<'a> {
    fn body<T: DeserializeOwned>(&self, request: &Request, reader: &mut impl Read) -> Result<T, Refusal> {
        let bad = |status: u16, detail: &str| {
            let failure = self.fail("talk_page_request", &[("detail", detail)]);
            (status, failure)
        };
        let Some(length) = request.length else {
            return Err(bad(http::LENGTH, &request.path));
        };
        if length > self.config.talk.page_body_bytes {
            return Err(bad(http::LARGE, &request.path));
        }
        let mut raw = Vec::new();
        let read = reader.by_ref().take(length).read_to_end(&mut raw);
        if !read.is_ok_and(|count| count as u64 == length) {
            return Err(bad(http::BAD, &request.path));
        }
        serde_json::from_slice(&raw).map_err(|error| bad(http::BAD, &error.to_string()))
    }

    fn receive(
        &self,
        request: &Request,
        reader: &mut impl Read,
    ) -> Result<(String, Kind, PathBuf, PathBuf, u64), Refusal> {
        let talk = &self.config.talk;
        let given = request.param("name").unwrap_or_default();
        let mb = talk.upload_max_mb.to_string();
        let refusal = |status: u16, code: &str, name: &str, detail: &str| {
            let fields = [("name", name), ("detail", detail), ("mb", mb.as_str())];
            (status, self.fail(code, &fields))
        };
        let Some((name, kind, file)) = stored(talk, given) else {
            return Err(refusal(http::KIND, "talk_kind", given, ""));
        };
        let fail = |status: u16, detail: &str| refusal(status, "talk_upload", &name, detail);
        let Some(length) = request.length else {
            return Err(fail(http::LENGTH, &request.path));
        };
        if length > talk.upload_max_mb.saturating_mul(MB) {
            return Err(fail(http::LARGE, &format!("{} MB", length / MB)));
        }
        let dir = self.temp.fresh().map_err(|detail| fail(http::BAD, &detail))?;
        let path = dir.join(file);
        let copied =
            File::create(&path).and_then(|mut file| std::io::copy(&mut reader.by_ref().take(length), &mut file));
        if !copied.as_ref().is_ok_and(|bytes| *bytes == length) {
            let _ = std::fs::remove_dir_all(&dir);
            let detail = copied.map_or_else(|error| error.to_string(), |bytes| bytes.to_string());
            return Err(fail(http::BAD, &detail));
        }
        Ok((name, kind, dir, path, length))
    }

    pub fn file(&self, request: &Request, reader: &mut impl Read) -> Result<Value, Refusal> {
        let (name, kind, dir, path, length) = self.receive(request, reader)?;
        let id = format!("{UPLOAD}{}", self.count.fetch_add(1, Ordering::Relaxed));
        let chat = request.param("chat").unwrap_or_default().to_string();
        lock(&self.uploads).insert(id.clone(), Upload { chat, dir, path });
        Ok(json!({"id": id, "name": name, "kind": kind, "bytes": length}))
    }

    pub fn hear(&self, request: &Request, reader: &mut impl Read) -> Result<Value, Refusal> {
        let config = self.config;
        let begun = Instant::now();
        if let Some(failure) = watch::game_failure(config, self.options.afk) {
            return Err((http::BUSY, failure));
        }
        if request.param("now") != Some(NOW)
            && let Some(label) = door::held(&config.talk)
        {
            return Err((http::BUSY, self.fail("talk_held", &[("label", &label)])));
        }
        let (_, _, dir, path, _) = self.receive(request, reader)?;
        let heard = load::dictate(config, &path);
        let _ = std::fs::remove_dir_all(&dir);
        let text = heard.map_err(|detail| (http::BAD, self.fail("talk_dictate", &[("detail", &detail)])))?;
        Ok(json!({"text": text, "seconds": begun.elapsed().as_secs_f64()}))
    }

    fn sources(&self, said: &Said) -> Result<(String, Vec<Source>), Failure> {
        let mut uploads = lock(&self.uploads);
        let mut sources = Vec::new();
        for id in &said.files {
            let upload = uploads
                .get(id)
                .filter(|upload| upload.chat == said.chat)
                .ok_or_else(|| self.fail("talk_upload_gone", &[]))?;
            sources.push(Source::File(upload.path.clone()));
        }
        if !said.redo {
            uploads.retain(|id, upload| {
                let kept = upload.chat != said.chat || said.files.contains(id);
                if !kept {
                    let _ = std::fs::remove_dir_all(&upload.dir);
                }
                kept
            });
        }
        let (text, links) = media::lift(&self.config.talk, &said.text, &|_| false);
        sources.extend(links);
        Ok((text, sources))
    }

    fn take(&self, said: &Said) -> Result<Session<'a>, Failure> {
        let mode = Mode {
            afk: self.options.afk,
            now: self.options.now || said.now,
            think: self.options.think || said.think,
            auto: self.options.auto || said.auto,
        };
        let found = {
            let mut list = lock(&self.sessions);
            let at = list.iter().position(|(chat, _)| *chat == said.chat);
            at.map(|at| list.remove(at).1)
        };
        let before = said.before.min(u32::MAX as usize);
        let prior = before + said.history.len();
        let kept = found.and_then(|mut session| session.keep(prior).then_some(session));
        let mut session = match kept {
            Some(session) => session,
            None => {
                let model = self.options.model.as_deref();
                let mut session = Session::open(self.config, self.config_path, self.temp, model, mode)?;
                let skip = said.history.len().saturating_sub(self.config.talk.page_history);
                session.seed(before + skip, &said.history[skip..]);
                session
            }
        };
        session.set(mode);
        Ok(session)
    }

    fn put(&self, chat: String, session: Session<'a>) {
        let mut list = lock(&self.sessions);
        list.retain(|(name, _)| *name != chat);
        list.push((chat, session));
        let extra = list.len().saturating_sub(self.config.talk.page_sessions.max(1));
        list.drain(..extra);
    }

    pub fn forget(&self, request: &Request, reader: &mut impl Read) -> Result<Value, Refusal> {
        let gone: Gone = self.body(request, reader)?;
        lock(&self.sessions).retain(|(name, _)| *name != gone.chat);
        lock(&self.uploads).retain(|_, upload| {
            let kept = upload.chat != gone.chat;
            if !kept {
                let _ = std::fs::remove_dir_all(&upload.dir);
            }
            kept
        });
        Ok(json!({"ok": true}))
    }

    pub fn say(&self, request: &Request, reader: &mut impl Read, stream: &TcpStream) {
        let mut wire = stream;
        let said: Said = match self.body(request, reader) {
            Ok(said) => said,
            Err((status, failure)) => return deny(&mut wire, status, &failure),
        };
        if self.writing.swap(true, Ordering::AcqRel) {
            return deny(&mut wire, http::BUSY, &self.fail("talk_page_busy", &[]));
        }
        let _writing = Writing(&self.writing);
        let ready = self.sources(&said).and_then(|found| Ok((found, self.take(&said)?)));
        let ((text, sources), mut session) = match ready {
            Ok(ready) => ready,
            Err(failure) => return deny(&mut wire, http::BAD, &failure),
        };
        if http::begin(&mut wire, http::OK, http::LINES).is_ok() {
            let mut out = Wire::new(stream);
            let turned = catch_unwind(AssertUnwindSafe(|| session.turn(&text, &sources, &mut out)));
            let last = match turned {
                Ok(Ok(told)) => json!({"told": told, "turn": session.total()}),
                Ok(Err(failure)) => json!({"error": failure}),
                Err(panic) => {
                    json!({"error": self.fail("talk_page_crash", &[("detail", &crashed(panic.as_ref()))])})
                }
            };
            out.line(&last);
            let _ = stream.shutdown(Shutdown::Write);
        }
        self.put(said.chat, session);
        self.touch();
    }
}

pub(crate) fn crashed(panic: &(dyn std::any::Any + Send)) -> String {
    panic
        .downcast_ref::<&str>()
        .map(|text| text.to_string())
        .or_else(|| panic.downcast_ref::<String>().cloned())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::{Said, stored};
    use crate::config::Config;
    use crate::media::Kind;

    fn config() -> Config {
        toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses")
    }

    #[test]
    fn an_uploaded_file_keeps_its_kind_and_gets_a_safe_name() {
        let talk = config().talk;
        let named = |given: &str| stored(&talk, given);
        let found = |plain: &str, kind: Kind, file: &str| Some((plain.to_string(), kind, file.to_string()));
        assert_eq!(named("Mein Clip #1.mp4"), found("Mein Clip _1.mp4", Kind::Video, "MeinClip_1.mp4"));
        assert_eq!(named("a?b.PNG"), found("a_b.PNG", Kind::Picture, "a_b.PNG"));
        assert_eq!(named("日本.mp3"), found("日本.mp3", Kind::Sound, "file.mp3"));
        let long = format!("{}.txt", "x".repeat(200));
        assert_eq!(named(&long), found(&long, Kind::Text, "file.txt"));
        for sneaky in ["..\\..\\evil.png", "../../evil.png", "C:\\Windows\\evil.png"] {
            let (_, kind, file) = named(sneaky).expect("a picture");
            assert_eq!(kind, Kind::Picture);
            assert!(!file.contains(['/', '\\', ':']), "{file}");
        }
        for unknown in ["notes", "run.exe", "", ".png", "a.png/"] {
            assert_eq!(named(unknown), None, "{unknown}");
        }
    }

    #[test]
    fn a_message_from_the_page_is_read_with_its_defaults() {
        let raw = r#"{"chat":"c1","text":"Hi","history":[["a","b"]],"before":2,"redo":true}"#;
        let said: Said = serde_json::from_str(raw).expect("valid");
        assert_eq!((said.chat.as_str(), said.text.as_str(), said.before), ("c1", "Hi", 2));
        assert_eq!(said.history, [("a".to_string(), "b".to_string())]);
        assert!(said.redo && !said.now && !said.think && said.files.is_empty());
        assert!(serde_json::from_str::<Said>(r#"{"history":[["only one"]]}"#).is_err());
        assert!(serde_json::from_str::<Said>(r#"{"before":-1}"#).is_err());
    }
}
