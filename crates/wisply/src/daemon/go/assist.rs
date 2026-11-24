use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::thread;
use std::time::Duration;

use super::history::History;
use super::{contacts, hint, preview};

const DEBOUNCE: Duration = Duration::from_millis(80);

static HIST: Mutex<History> = Mutex::new(History::empty());
static PEOPLE: Mutex<Vec<Vec<String>>> = Mutex::new(Vec::new());
static ASK: Mutex<String> = Mutex::new(String::new());
static LINE: Mutex<(String, String)> = Mutex::new((String::new(), String::new()));
static GEN: AtomicU64 = AtomicU64::new(0);
static WORKING: AtomicBool = AtomicBool::new(false);

fn lock<T>(m: &'static Mutex<T>) -> MutexGuard<'static, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

fn words(p: &contacts::Person) -> Vec<String> {
    let mut all = vec![p.name.clone()];
    all.extend(p.alias.iter().cloned());
    all
}

pub fn open(home: &Path) {
    *lock(&HIST) = History::load(home);
    *lock(&PEOPLE) = contacts::load(home).iter().map(words).collect();
    clear();
}

pub fn clear() {
    GEN.fetch_add(1, Ordering::SeqCst);
    lock(&ASK).clear();
    *lock(&LINE) = (String::new(), String::new());
}

pub fn line_for(text: &str) -> String {
    let line = lock(&LINE);
    if text.trim().is_empty() || line.0.is_empty() { String::new() } else { line.1.clone() }
}

pub fn older(current: &str) -> Option<String> {
    lock(&HIST).older(current)
}

pub fn newer() -> Option<String> {
    lock(&HIST).newer()
}

pub fn typed() {
    lock(&HIST).reset();
}

pub fn remember(home: &Path, said: &str) {
    let mut h = lock(&HIST);
    h.push(said);
    h.save(home);
}

pub fn tab(text: &str) -> Option<String> {
    hint::complete(text, &lock(&PEOPLE))
}

fn work(home: PathBuf, done: fn()) {
    loop {
        thread::sleep(DEBOUNCE);
        let seen = GEN.load(Ordering::SeqCst);
        let said = lock(&ASK).clone();
        let known = lock(&LINE).0 == said;
        if !said.is_empty() && !known {
            let out = preview(&home, &said);
            if GEN.load(Ordering::SeqCst) == seen {
                *lock(&LINE) = (said, out);
                done();
            }
        }
        if GEN.load(Ordering::SeqCst) == seen {
            WORKING.store(false, Ordering::SeqCst);
            if GEN.load(Ordering::SeqCst) == seen || WORKING.swap(true, Ordering::SeqCst) {
                return;
            }
        }
    }
}

pub fn ask(home: &Path, text: &str, done: fn()) {
    let said = text.trim().to_string();
    GEN.fetch_add(1, Ordering::SeqCst);
    *lock(&ASK) = said;
    if WORKING.swap(true, Ordering::SeqCst) {
        return;
    }
    let home = home.to_path_buf();
    let spawned = thread::Builder::new().name("go-hint".into()).spawn(move || work(home, done));
    if spawned.is_err() {
        WORKING.store(false, Ordering::SeqCst);
    }
}

pub const VK_TAB: u32 = 0x09;
pub const VK_UP: u32 = 0x26;
pub const VK_DOWN: u32 = 0x28;

pub fn nav(vk: u32, text: &mut String) {
    let next = match vk {
        VK_UP => older(text),
        VK_DOWN => newer(),
        VK_TAB => tab(text),
        _ => None,
    };
    if let Some(next) = next {
        *text = next;
    }
}
