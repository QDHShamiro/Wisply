use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::JoinHandle;
use std::time::Instant;

use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use serde_json::{Value, json};

use crate::clock;
use crate::config::Trace;
use crate::media::Source;
use crate::talk::{Mode, Out};

static COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
pub struct Log {
    pub sent: Option<Value>,
    pub reply: Option<Value>,
    pub think: Option<bool>,
    pub media: Vec<PathBuf>,
}

pub struct Tape<'o> {
    inner: &'o mut dyn Out,
    begun: Instant,
    taped: Taped,
}

#[derive(Default)]
pub struct Taped {
    pub steps: Vec<(f64, String)>,
    pub thought: String,
    pub answer: String,
    pub first_thought: Option<f64>,
    pub first_word: Option<f64>,
    pub total: f64,
}

impl<'o> Tape<'o> {
    pub fn new(inner: &'o mut dyn Out) -> Tape<'o> {
        Tape { inner, begun: Instant::now(), taped: Taped::default() }
    }

    fn at(&self) -> f64 {
        self.begun.elapsed().as_secs_f64()
    }

    pub fn done(self) -> Taped {
        let total = self.at();
        Taped { total, ..self.taped }
    }
}

impl Out for Tape<'_> {
    fn note(&mut self, line: &str) {
        let at = self.at();
        self.taped.steps.push((at, line.to_string()));
        self.inner.note(line);
    }

    fn word(&mut self, piece: &str) -> bool {
        let at = self.at();
        self.taped.first_word.get_or_insert(at);
        self.taped.answer.push_str(piece);
        self.inner.word(piece)
    }

    fn thought(&mut self, piece: &str) -> bool {
        let at = self.at();
        self.taped.first_thought.get_or_insert(at);
        self.taped.thought.push_str(piece);
        self.inner.thought(piece)
    }

    fn end(&mut self) {
        self.inner.end();
    }

    fn gone(&mut self) -> bool {
        self.inner.gone()
    }
}

pub struct Record {
    pub said: String,
    pub sources: Vec<Source>,
    pub mode: Mode,
    pub model: String,
    pub config: PathBuf,
    pub log: Log,
    pub told: Option<Value>,
    pub error: Option<Value>,
}

fn source_text(source: &Source) -> String {
    match source {
        Source::File(path) => path.display().to_string(),
        Source::Link(url) => url.clone(),
    }
}

fn data_of(part: &Value) -> Option<(String, String)> {
    if let Some(url) = part.pointer("/image_url/url").and_then(Value::as_str) {
        let (head, data) = url.split_once(',')?;
        let kind = head.strip_prefix("data:")?.split(';').next()?;
        return Some((kind.rsplit('/').next()?.to_string(), data.to_string()));
    }
    let data = part.pointer("/input_audio/data").and_then(Value::as_str)?;
    let format = part.pointer("/input_audio/format").and_then(Value::as_str)?;
    Some((format.to_string(), data.to_string()))
}

pub fn strip(sent: &mut Value, dir: Option<&Path>, marker: &str) -> Vec<String> {
    let mut saved = Vec::new();
    let Some(messages) = sent.get_mut("messages").and_then(Value::as_array_mut) else {
        return saved;
    };
    let last = messages.len().saturating_sub(1);
    for (index, message) in messages.iter_mut().enumerate() {
        let Some(parts) = message.get_mut("content").and_then(Value::as_array_mut) else {
            continue;
        };
        for part in parts.iter_mut() {
            let Some((extension, data)) = data_of(part) else {
                continue;
            };
            let name = format!("{:03}.{extension}", saved.len());
            let kept = dir
                .filter(|_| index + 1 >= last)
                .zip(STANDARD.decode(&data).ok())
                .is_some_and(|(dir, bytes)| fs::write(dir.join(&name), bytes).is_ok());
            let shown = if kept {
                saved.push(name.clone());
                name
            } else {
                marker.to_string()
            };
            let kind = part.get("type").cloned().unwrap_or(Value::Null);
            *part = json!({"type": kind, "file": shown});
        }
    }
    saved
}

fn keep_media(spec: &Trace, dir: &Path, paths: &[PathBuf]) -> Vec<Value> {
    let limit = spec.media_max_mb.saturating_mul(1024 * 1024);
    let mut unique: Vec<&PathBuf> = Vec::new();
    for path in paths {
        if !unique.contains(&path) {
            unique.push(path);
        }
    }
    unique
        .into_iter()
        .enumerate()
        .map(|(index, path)| {
            let bytes = fs::metadata(path).map(|m| m.len()).ok();
            let fits = bytes.is_some_and(|size| size <= limit);
            let name = path
                .extension()
                .map_or_else(|| format!("media-{index}"), |ext| format!("media-{index}.{}", ext.to_string_lossy()));
            let kept = (fits && fs::copy(path, dir.join(&name)).is_ok()).then_some(name);
            json!({"path": path.display().to_string(), "bytes": bytes, "kept": kept})
        })
        .collect()
}

pub fn id() -> String {
    format!("{}-{}-{}", clock::local().file(), std::process::id(), COUNT.fetch_add(1, Ordering::Relaxed))
}

pub fn line(spec: &Trace, taped: &Taped, record: Record, id: &str, dir: Option<&Path>) -> Value {
    let mut log = record.log;
    let frames =
        log.sent.as_mut().map(|sent| strip(sent, dir.filter(|_| spec.frames), &spec.marker)).unwrap_or_default();
    let mut media: Vec<PathBuf> = record
        .sources
        .iter()
        .filter_map(|source| match source {
            Source::File(path) => Some(path.clone()),
            Source::Link(_) => None,
        })
        .collect();
    media.extend(log.media);
    let media = dir.filter(|_| spec.media_max_mb > 0).map_or_else(Vec::new, |dir| keep_media(spec, dir, &media));
    let mode = record.mode;
    let steps: Vec<Value> = taped.steps.iter().map(|(at, line)| json!({"s": at, "line": line})).collect();
    json!({
        "id": id,
        "at": clock::local().text(),
        "exe": std::env::current_exe().ok().map(|p| p.display().to_string()),
        "config": record.config.display().to_string(),
        "model": record.model,
        "mode": {"afk": mode.afk, "now": mode.now, "think": mode.think, "auto": mode.auto},
        "think": log.think,
        "question": record.said,
        "sources": record.sources.iter().map(source_text).collect::<Vec<_>>(),
        "media": media,
        "steps": steps,
        "thought": taped.thought,
        "answer": taped.answer,
        "first_thought_s": taped.first_thought,
        "first_word_s": taped.first_word,
        "total_s": taped.total,
        "told": record.told,
        "error": record.error,
        "frames": frames,
        "request": log.sent,
        "reply": log.reply,
    })
}

pub fn keep(spec: &Trace, taped: &Taped, record: Record) -> Option<PathBuf> {
    fs::create_dir_all(&spec.dir).ok()?;
    let id = id();
    let dir = spec.dir.join(&id);
    let dir = (spec.frames || spec.media_max_mb > 0).then(|| fs::create_dir_all(&dir).ok().map(|_| dir)).flatten();
    let row = line(spec, taped, record, &id, dir.as_deref());
    if let Some(dir) = dir.as_deref() {
        let _ = fs::remove_dir(dir);
    }
    let stamp = clock::local().file();
    let day = stamp.split('-').next().unwrap_or(&stamp);
    let file = spec.dir.join(format!("{day}.jsonl"));
    let mut handle = OpenOptions::new().create(true).append(true).open(&file).ok()?;
    writeln!(handle, "{row}").ok()?;
    Some(file)
}

pub fn on(spec: &Trace) -> bool {
    !spec.dir.as_os_str().is_empty()
}

pub fn spawn(spec: &Trace, taped: Taped, record: Record) -> JoinHandle<()> {
    let spec = spec.clone();
    std::thread::spawn(move || {
        let _ = keep(&spec, &taped, record);
    })
}

#[cfg(test)]
#[path = "trace_tests.rs"]
mod tests;
