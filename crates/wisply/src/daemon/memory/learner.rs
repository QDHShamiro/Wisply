use std::collections::VecDeque;
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

use serde_json::{Value, json};
use wisply_memory::{
    Due, NewNote, Record, Store, WIKI_KINDS, chunks_of, condense, now, oneline, points, project_of_work, render_page,
    scrub, similar, template, titled, write_draft, write_session,
};

use super::super::config;
use super::chain::{self, Halt, Links};
use super::chat::Provider;
use super::index::vault;

const KINDS: &[&str] = &["decision", "preference", "bugfix", "feature", "fact", "discovery", "change"];
const SYSTEM: &str = "Long-term memory of a coding assistant. Input: condensed session log (USER, CLAUDE, EDIT, RAN, ERROR lines). Output JSON only. notes (max 6, none is fine): only what helps in a FUTURE session: decision + reason, user preference or rule, bug + root cause + fix, non-obvious fact (path, port, command, version, account). Never secrets. Skip routine steps, progress, counts, test or CI runs, plans, anything already in the code or in 'Known notes'. Title starts with the project or tool ('Wisply: hints need words and meaning'). English, one concrete sentence per body. Record, only what THIS part adds, one short line per point: summary = what changed and where; why = reasons for decisions; rejected = tried and dropped + why; next = still open at the end. lessons (max 2, none is fine): mistake = error + root cause + fix; rule = fixed rule from the user; pattern = reusable way of working.";
const DRAFT_SYSTEM: &str = "Write one page for the user's knowledge base from the note and the session context. Fill each section with \
1 to 4 short lines in the language of the section headings; leave a section empty when the note says nothing \
for it. Concrete names, versions and commands, never passwords, keys or tokens. The title says the lesson in \
a few words.";
const NOTES_PER_CHUNK: usize = 6;
const LESSONS_PER_CHUNK: usize = 2;
const WIKI_PER_SESSION: usize = 2;
pub(super) const MIN_CHARS: usize = 600;
pub(super) const MIN_USERS: usize = 2;
const KNOWN: usize = 6;
const KNOWN_CHARS: usize = 90;
const QUIET_S: f64 = 600.0;
const RETRY_S: f64 = 1800.0;
const POLL_S: f64 = 60.0;
const BACKOFF_STEPS: u32 = 4;
const DUP_SIM: f64 = 0.7;
const DRAFT_TITLE: usize = 100;
const INBOX: &str = "Inbox";
const TICK_S: f64 = 600.0;
const PRIVATE_WAIT_S: f64 = 3600.0;
const PRIVATE_NO_LOCAL_S: f64 = 21_600.0;
const NO_LOCAL: &str =
    "private session learns only with a model on this PC (wisply config memory_api + memory_api_model)";

#[derive(Default, Clone)]
pub struct State {
    pub last: String,
    pub error: String,
    pub learned: usize,
    pub now: Option<Now>,
    pub done: Vec<Done>,
}

#[derive(Default, Clone)]
pub struct Now {
    pub sid: String,
    pub title: String,
    pub project: String,
    pub via: String,
    pub part: usize,
    pub since: f64,
}

#[derive(Default, Clone)]
pub struct Done {
    pub sid: String,
    pub title: String,
    pub project: String,
    pub added: usize,
    pub t: f64,
}

const DONE_KEEP: usize = 12;
static STATE: Mutex<State> =
    Mutex::new(State { last: String::new(), error: String::new(), learned: 0, now: None, done: Vec::new() });

pub fn state() -> State {
    STATE.lock().map(|s| s.clone()).unwrap_or_default()
}

fn set(f: impl FnOnce(&mut State)) {
    if let Ok(mut s) = STATE.lock() {
        f(&mut s);
    }
}

fn schema() -> Value {
    let text = json!({"type": "string"});
    let note = json!({"type": "object", "required": ["kind", "title", "body"], "properties": {
        "kind": {"type": "string", "enum": KINDS}, "title": text, "body": text,
        "files": {"type": "array", "items": text}}});
    let lesson = json!({"type": "object", "required": ["kind", "title", "body"], "properties": {
        "kind": {"type": "string", "enum": WIKI_KINDS}, "title": text, "body": text}});
    json!({"type": "object", "required": ["title", "summary", "why", "rejected", "next", "notes", "lessons"],
        "properties": {"title": text, "summary": text, "why": text, "rejected": text, "next": text,
            "notes": {"type": "array", "maxItems": NOTES_PER_CHUNK, "items": note},
            "lessons": {"type": "array", "maxItems": LESSONS_PER_CHUNK, "items": lesson}}})
}

pub fn parse(text: &str) -> Option<Value> {
    serde_json::from_str(text).ok().or_else(|| {
        let (a, b) = (text.find('{')?, text.rfind('}')?);
        (a < b).then(|| serde_json::from_str(&text[a..=b]).ok()).flatten()
    })
}

fn field(v: &Value, key: &str) -> String {
    match v.get(key) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(a)) => a.iter().filter_map(Value::as_str).collect::<Vec<_>>().join("\n"),
        _ => String::new(),
    }
}

static WAITS: AtomicU32 = AtomicU32::new(0);

fn backoff() -> f64 {
    let n = WAITS.fetch_add(1, Ordering::Relaxed).min(BACKOFF_STEPS);
    (POLL_S * f64::from(1u32 << n)).min(QUIET_S)
}

enum Stop {
    Wait(String, Option<f64>),
    Hold(String, f64),
    Fail(String),
}

impl From<Halt> for Stop {
    fn from(h: Halt) -> Stop {
        match h {
            Halt::Wait(why, until) => Stop::Wait(why, until),
            Halt::Fail(why) => Stop::Fail(why),
        }
    }
}

fn inbox(home: &Path) -> std::path::PathBuf {
    let name = config::text(&config::load(home), "vault_inbox");
    vault(home).join(if name.trim().is_empty() { INBOX } else { name.trim() })
}

fn draft(home: &Path, store: &mut Store, links: &mut Links, lesson: &Value, project: &str, sid: &str, r: &Record) {
    let Some(i) = links.last else { return };
    let (kind, title, body) = (field(lesson, "kind"), field(lesson, "title"), field(lesson, "body"));
    let known = store.search(&title, project, 5, None, 0.0).unwrap_or_default();
    if known.iter().any(|h| h.note.source == "vault" && similar(&h.note.title, &title) >= DUP_SIM) {
        return;
    }
    let root = vault(home);
    let heads = template(&root, &kind);
    let props: serde_json::Map<String, Value> = heads.iter().map(|h| (h.clone(), json!({"type": "string"}))).collect();
    let schema = json!({"type": "object", "required": ["title", "sections"], "properties": {
        "title": {"type": "string"}, "sections": {"type": "object", "required": heads, "properties": props}}});
    let shown = if project.is_empty() { "-" } else { project };
    let user =
        format!("Project: {shown}\nKind: {kind}\nLesson: {title}\n{body}\n\nSession: {}\n{}", r.title, r.summary);
    let Some(answer) = links.ask(i, DRAFT_SYSTEM, &user, &schema) else { return };
    let Some(out) = parse(&answer) else { return };
    let sections: Vec<(String, String)> = out
        .get("sections")
        .and_then(Value::as_object)
        .map(|m| m.iter().map(|(h, v)| (h.clone(), scrub(v.as_str().unwrap_or("")))).collect())
        .unwrap_or_default();
    let named = Some(field(&out, "title")).filter(|t| !t.trim().is_empty()).unwrap_or(title);
    let named = oneline(&scrub(&named), DRAFT_TITLE);
    let page = render_page(&root, &kind, &named, &sections, project, sid);
    write_draft(&inbox(home), &kind, &named, &page);
}

fn known_titles(store: &mut Store, project: &str, title: &str, part: &str, sid: &str) -> String {
    let own = format!("s:{sid}");
    let probe = format!("{title} {}", oneline(part, 300));
    let hits = store.search(&probe, project, KNOWN, None, 0.0).unwrap_or_default();
    let lines: Vec<String> = hits
        .iter()
        .filter(|h| h.note.kind != "draft" && h.note.origin != own)
        .map(|h| format!("- {}", oneline(&h.note.title, KNOWN_CHARS)))
        .collect();
    if lines.is_empty() { String::new() } else { format!("Known notes:\n{}\n", lines.join("\n")) }
}

struct Step<'a> {
    project: &'a str,
    sid: &'a str,
    hidden: bool,
}

fn next_link(links: &Links, hidden: bool) -> Result<usize, Stop> {
    match links.pick() {
        Ok(i) => Ok(i),
        Err(Halt::Wait(why, until)) if hidden => {
            Err(Stop::Hold(format!("private, {why}"), until.unwrap_or(now() + PRIVATE_WAIT_S)))
        }
        Err(h) => Err(h.into()),
    }
}

fn answer(links: &mut Links, i: usize, user: &str, schema: &Value) -> Option<Value> {
    let out = parse(&links.ask(i, SYSTEM, user, schema)?);
    if out.is_none() {
        let name = links.all[i].name();
        links.refuse(i, format!("{name} sent no JSON object"));
    }
    out
}

fn read_parts(
    store: &mut Store,
    links: &mut Links,
    step: &Step,
    text: String,
    r: &mut Record,
) -> Result<(usize, Vec<Value>), Stop> {
    let db = |e: wisply_memory::DbError| Stop::Fail(format!("memory.db: {e}"));
    let (mut added, mut lessons) = (0, Vec::new());
    let schema = schema();
    let mut parts = VecDeque::from([text]);
    while let Some(part) = parts.pop_front() {
        let i = next_link(links, step.hidden)?;
        let size = links.all[i].chars;
        if part.chars().count() > size {
            chunks_of(&part, size).into_iter().rev().for_each(|p| parts.push_front(p));
            continue;
        }
        let shown = if step.project.is_empty() { "-" } else { step.project };
        let title = if r.title.is_empty() { "-" } else { r.title.as_str() };
        let known = known_titles(store, step.project, &r.title, &part, step.sid);
        let user = format!("Project: {shown}\nSession title: {title}\n{known}\nSession log:\n{part}");
        let via = links.all[i].name();
        set(|s| {
            if let Some(n) = s.now.as_mut() {
                n.part += 1;
                n.via.clone_from(&via);
                if !r.title.is_empty() {
                    n.title.clone_from(&r.title);
                }
            }
        });
        let Some(out) = answer(links, i, &user, &schema) else {
            parts.push_front(part);
            continue;
        };
        links.done();
        added += keep_notes(store, &out, (step.project, step.sid), &via).map_err(db)?;
        let found = out.get("lessons").and_then(Value::as_array).into_iter().flatten();
        lessons.extend(
            found.filter(|n| WIKI_KINDS.contains(&field(n, "kind").as_str()) && !field(n, "title").is_empty()).cloned(),
        );
        absorb(r, &out);
    }
    Ok((added, lessons))
}

fn learn(home: &Path, store: &mut Store, all: &[Provider], words: &[Vec<String>], row: &Due) -> Result<usize, Stop> {
    let path = Path::new(&row.transcript);
    if !path.exists() {
        store.session_gone(&row.id).map_err(|e| Stop::Fail(e.to_string()))?;
        return Ok(0);
    }
    let began = now();
    let c = condense(path, row.offset, None).map_err(|e| Stop::Fail(format!("transcript: {e}")))?;
    let project =
        Some(project_of_work(&c.paths, &c.servers)).filter(|p| !p.is_empty()).unwrap_or_else(|| row.project.clone());
    let db = |e: wisply_memory::DbError| Stop::Fail(format!("memory.db: {e}"));
    if project != row.project {
        store.session_project(&row.id, &project).map_err(db)?;
    }
    if c.text.chars().count() < MIN_CHARS || (c.users < MIN_USERS && c.edits == 0) {
        store.session_short(&row.id, c.end, began, &c.title).map_err(db)?;
        store.session_note_gone(&row.id).map_err(db)?;
        return Ok(0);
    }
    let mut r = Record {
        title: if c.title.is_empty() { row.title.clone() } else { c.title.clone() },
        summary: row.summary.clone(),
        why: row.why.clone(),
        rejected: row.rejected.clone(),
        next: row.next.clone(),
    };
    let paths = c.paths.join(" ");
    let hidden = chain::private(words, &[&project, &row.project, &row.transcript, &paths, &r.title, &c.text]);
    let mut links = Links::new(home, all.iter().filter(|p| p.local || !hidden).cloned().collect());
    if hidden && links.all.is_empty() {
        return Err(Stop::Hold(NO_LOCAL.into(), now() + PRIVATE_NO_LOCAL_S));
    }
    let step = Step { project: &project, sid: &row.id, hidden };
    let (added, lessons) = read_parts(store, &mut links, &step, c.text, &mut r)?;
    let room = if store.drafts_full().unwrap_or(true) { 0 } else { WIKI_PER_SESSION };
    for lesson in lessons.iter().take(room) {
        draft(home, store, &mut links, lesson, &project, &row.id, &r);
    }
    let open_since = if c.more { 0.0 } else { began };
    store.session_learned(&row.id, c.end, open_since, &r).map_err(db)?;
    finish(home, store, row, &project, &r).map_err(db)?;
    Ok(added)
}

fn keep_notes(
    store: &mut Store,
    out: &Value,
    (project, session): (&str, &str),
    via: &str,
) -> Result<usize, wisply_memory::DbError> {
    let mut added = 0;
    for n in out.get("notes").and_then(Value::as_array).into_iter().flatten() {
        let title = field(n, "title");
        if title.trim().is_empty() {
            continue;
        }
        let kind = Some(field(n, "kind")).filter(|k| KINDS.contains(&k.as_str())).unwrap_or_else(|| "fact".into());
        let files: Vec<String> = n
            .get("files")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|f| f.as_str().map(String::from))
            .collect();
        let full = titled(project, &title);
        let body = field(n, "body");
        let note = NewNote { project, kind: &kind, title: &full, body: &body, source: "session", session };
        if let Some((id, new)) = store.merge_note(&note, &files)? {
            if new || store.via(id)?.is_empty() {
                store.set_via(id, via)?;
            }
            added += usize::from(new);
        }
    }
    Ok(added)
}

fn absorb(r: &mut Record, out: &Value) {
    r.summary = points(&r.summary, &field(out, "summary"));
    r.why = points(&r.why, &field(out, "why"));
    r.rejected = points(&r.rejected, &field(out, "rejected"));
    let next = points("", &field(out, "next"));
    if !next.is_empty() {
        r.next = next;
    }
    if r.title.is_empty() {
        r.title = field(out, "title");
    }
}

fn finish(home: &Path, store: &mut Store, row: &Due, project: &str, r: &Record) -> Result<(), wisply_memory::DbError> {
    let started = if row.started > 0.0 { row.started } else { now() };
    if write_session(&vault(home), &row.id, project, r, started).is_none() {
        store.session_note(&row.id, project, r)?;
    } else {
        store.session_note_gone(&row.id)?;
    }
    let origin = format!("session:{}", row.id);
    for line in r.next.lines() {
        store.suggest(project, line, &origin)?;
    }
    Ok(())
}

fn watched<T>(row: &Due, run: impl FnOnce() -> T) -> (T, Now) {
    set(|s| {
        s.now = Some(Now {
            sid: row.id.clone(),
            title: row.title.clone(),
            project: row.project.clone(),
            since: now(),
            ..Now::default()
        });
    });
    let got = run();
    let mut seen = None;
    set(|s| seen = s.now.take());
    (got, seen.unwrap_or_default())
}

pub fn tick(home: &Path, store: &mut Store, soon: bool) {
    let cfg = config::load(home);
    if !config::on(&cfg, "memory") || !(soon || config::on(&cfg, "memory_learn")) {
        return;
    }
    let t = now();
    let Ok(due) = store.due_sessions(if soon { t + QUIET_S } else { t }, QUIET_S) else { return };
    if due.is_empty() {
        return;
    }
    let all = chain::links(home);
    match Links::new(home, all.clone()).pick() {
        Ok(_) => {}
        Err(Halt::Wait(..)) => return,
        Err(Halt::Fail(why)) => return set(|s| s.error = why),
    }
    let words = chain::private_words(home);
    for row in &due {
        if now() - t > TICK_S {
            return;
        }
        let (got, seen) = watched(row, || learn(home, store, &all, &words, row));
        match got {
            Ok(n) => set(|s| {
                WAITS.store(0, Ordering::Relaxed);
                s.learned += n;
                let title = if seen.title.is_empty() { row.title.clone() } else { seen.title };
                s.done.insert(0, Done { sid: row.id.clone(), title, project: row.project.clone(), added: n, t: now() });
                s.done.truncate(DONE_KEEP);
                s.last = format!(
                    "{} learned {}",
                    wisply_core::local_now().get(11..16).unwrap_or(""),
                    row.id.chars().take(8).collect::<String>()
                );
                s.error.clear();
            }),
            Err(Stop::Wait(why, until)) => {
                let _ = store.session_retry(&row.id, until.unwrap_or_else(|| now() + backoff()), false);
                set(|s| s.error = why);
                return;
            }
            Err(Stop::Hold(why, until)) => {
                let _ = store.session_retry(&row.id, until, false);
                set(|s| s.error = format!("{}: {why}", row.id.chars().take(8).collect::<String>()));
            }
            Err(Stop::Fail(why)) => {
                let _ = store.session_retry(&row.id, now() + RETRY_S, true);
                set(|s| s.error = format!("{}: {why}", row.id.chars().take(8).collect::<String>()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_are_read_even_with_text_around() {
        assert_eq!(parse("sure: {\"a\": 1} done").unwrap()["a"], 1);
        assert!(parse("no json").is_none());
        let v = json!({"next": ["a", "b"], "why": "c"});
        assert_eq!((field(&v, "next").as_str(), field(&v, "why").as_str(), field(&v, "x").as_str()), ("a\nb", "c", ""));
        let s = schema();
        assert_eq!(s["properties"]["notes"]["maxItems"], 6);
        assert_eq!(s["properties"]["lessons"]["items"]["properties"]["kind"]["enum"][0], "mistake");
    }

    #[test]
    fn a_busy_model_is_asked_less_and_less_often() {
        let waits: Vec<f64> = (0..6).map(|_| backoff()).collect();
        assert_eq!(waits, [60.0, 120.0, 240.0, 480.0, 600.0, 600.0]);
        WAITS.store(0, Ordering::Relaxed);
        assert_eq!(backoff(), POLL_S);
    }

    #[test]
    fn the_model_is_not_shown_the_early_note_of_its_own_session() {
        let dir = std::env::temp_dir().join(format!("lt-known-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut store = Store::open(&dir).unwrap();
        store.session_seen("a", "web", "/work/web", "/work/a.jsonl", false).unwrap();
        let early = Record {
            title: "web: Cart totals".into(),
            summary: "Latest request: fix cart totals".into(),
            ..Record::default()
        };
        store.session_note("a", "web", &early).unwrap().unwrap();
        assert_eq!(known_titles(&mut store, "web", "Cart totals", "fix cart totals", "a"), "");
        assert!(known_titles(&mut store, "web", "Cart totals", "fix cart totals", "b").contains("web: Cart totals"));
        drop(store);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
