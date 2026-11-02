use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use serde_json::{Map, Value, json};
use wisply_code::Index;
use wisply_core::{Call, Fail};

use super::super::ctx::Ctx;
use super::super::{Daemon, web};

const RECHECK: Duration = Duration::from_secs(3);
const USED_EVERY: Duration = Duration::from_secs(60);
const REFRESH: Duration = Duration::from_secs(2);
const BG_REFRESH: Duration = Duration::from_secs(20);
const ACTIVE_DAYS: f64 = 7.0;
const IMPACT_LINES: usize = 60;
const CODE_CHARS: usize = 6000;
const ROOTS: &str = "roots.json";
const BG_EVERY: Duration = Duration::from_secs(600);
const BG_RECENT_S: f64 = 3600.0;

static LAST_BG: Mutex<Option<Instant>> = Mutex::new(None);

struct Seen {
    used: bool,
    checked: bool,
}

thread_local! {
    static STAMPS: RefCell<HashMap<PathBuf, (Instant, Instant)>> = RefCell::new(HashMap::new());
    static OPEN: RefCell<HashMap<PathBuf, Index>> = RefCell::new(HashMap::new());
}

fn throttle(root: &Path) -> Seen {
    STAMPS.with(|cell| {
        let mut stamps = cell.borrow_mut();
        let now = Instant::now();
        match stamps.get_mut(root) {
            Some((checked, written)) => {
                let seen = Seen { used: written.elapsed() < USED_EVERY, checked: checked.elapsed() < RECHECK };
                if !seen.checked {
                    *checked = now;
                }
                if !seen.used {
                    *written = now;
                }
                seen
            }
            None => {
                stamps.insert(root.to_path_buf(), (now, now));
                Seen { used: false, checked: false }
            }
        }
    })
}

fn needs_files(c: &Call) -> bool {
    matches!(c.pos.first().map(|p| p.to_lowercase()).as_deref(), Some("body" | "impact")) || c.pos.is_empty()
}

fn roots_file(home: &Path) -> PathBuf {
    home.join("code").join(ROOTS)
}

fn active(home: &Path) -> Map<String, Value> {
    let now = wisply_memory::now();
    let all: Map<String, Value> =
        fs::read_to_string(roots_file(home)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    all.into_iter().filter(|(_, at)| at.as_f64().is_some_and(|t| now - t < ACTIVE_DAYS * 86400.0)).collect()
}

fn used(home: &Path, root: &Path) {
    let mut all = active(home);
    all.insert(root.to_string_lossy().into_owned(), json!(wisply_memory::now()));
    let _ = fs::create_dir_all(home.join("code"));
    let _ = fs::write(roots_file(home), Value::Object(all).to_string());
}

fn with_index<T>(home: &Path, root: &Path, f: impl FnOnce(&mut Index) -> Result<T, String>) -> Result<T, Fail> {
    OPEN.with(|cell| {
        let mut open = cell.borrow_mut();
        if !open.contains_key(root) {
            let index = Index::open(home, root).map_err(|e| Fail::new(format!("cannot open the code map ({e})")))?;
            open.insert(root.to_path_buf(), index);
        }
        let Some(index) = open.get_mut(root) else { return Err(Fail::new("the code map is not open")) };
        f(index).map_err(|e| Fail::new(format!("code map: {e}")))
    })
}

fn meaning(index: &mut Index, text: &str) -> Result<String, String> {
    index.by_name(text, "closest by name and body").map_err(|e| e.to_string())
}

fn answer(index: &mut Index, c: &Call) -> Result<String, String> {
    let db = |e: wisply_code::DbError| e.to_string();
    let pos = &c.pos;
    let sub = pos.first().map(|p| p.to_lowercase()).unwrap_or_default();
    match (sub.as_str(), pos.len()) {
        (_, 0) => index.overview().map_err(db),
        ("callers", 2) => index.callers(&pos[1]).map_err(db),
        ("outline", 2) => index.outline(&pos[1]).map_err(db),
        ("body", 2) => index.body(&pos[1]),
        ("impact", 1) => index.impact(c.val("base"), c.on("tests")),
        _ => {
            let text = pos.join(" ");
            if text.trim().contains(' ') { meaning(index, text.trim()) } else { index.lookup(text.trim()).map_err(db) }
        }
    }
}

pub fn code(d: &Daemon, ctx: &mut Ctx, c: &Call) -> Result<String, Fail> {
    let place = c.val("project").map(String::from).unwrap_or_else(|| ctx.cwd.clone());
    let Some(root) = wisply_memory::root_of(Path::new(&place)) else {
        return Err(Fail::hint(
            format!("no project here ({place})"),
            "run it inside a project folder, or wisply code ... --project <path>",
        ));
    };
    let fresh = throttle(&root);
    if !fresh.used {
        used(&d.home, &root);
    }
    let (text, left) = with_index(&d.home, &root, |index| {
        if !fresh.checked || needs_files(c) {
            index.refresh(Some(REFRESH)).map_err(|e| e.to_string())?;
        }
        Ok((answer(index, c)?, index.left))
    })?;
    let note = if left > 0 {
        format!("\n({left} changed files are not read yet, the next wisply code reads on)")
    } else {
        String::new()
    };
    let lines = (c.pos.first().map(String::as_str) == Some("impact")).then_some(IMPACT_LINES);
    Ok(web::capped(d, ctx, &format!("{text}{note}"), "code", lines, CODE_CHARS))
}

pub fn background(home: &Path) {
    {
        let Ok(mut last) = LAST_BG.lock() else { return };
        if last.is_some_and(|t| t.elapsed() < BG_EVERY) {
            return;
        }
        *last = Some(Instant::now());
    }
    let now = wisply_memory::now();
    let recent = active(home).into_iter().filter(|(_, at)| at.as_f64().is_some_and(|t| now - t < BG_RECENT_S));
    for root in recent.map(|(p, _)| PathBuf::from(p)).filter(|p| p.is_dir()) {
        if let Ok(mut index) = Index::open(home, &root) {
            let _ = index.refresh(Some(BG_REFRESH));
        }
    }
}
