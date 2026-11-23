use std::env;
use std::io;
use std::path::{Path, PathBuf};

use serde_json::Value;
use wisply_core::{Call, Fail};
use wisply_memory::now;
use wisply_memory::projects::{self, Entry, INDEX, PROJECTS};

use super::super::Daemon;
use super::super::ctx::Ctx;

const SHOWN: usize = 3;
const TOLD: &str = "p:";
const TOKEN_CHARS: usize = 4;
const PROMPT_CHARS: usize = 4000;
const ARCHIVE: &str = "archive";

fn claude_home() -> PathBuf {
    env::var_os("CLAUDE_CONFIG_DIR")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| wisply_proto::user_dir().join(".claude"))
}

fn posix(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

fn text<'a>(data: &'a Value, key: &str) -> &'a str {
    data.get(key).and_then(Value::as_str).unwrap_or("")
}

fn dir_of(data: &Value, cwd: &str) -> PathBuf {
    let cwd = Some(text(data, "cwd")).filter(|c| !c.is_empty()).unwrap_or(cwd);
    projects::memory_dir(&claude_home(), cwd, text(data, "transcript_path"))
}

fn told(e: &Entry) -> String {
    format!("{TOLD}{}", e.file)
}

pub(super) fn hint(ctx: &mut Ctx, data: &Value, prompt: &str) -> String {
    let dir = dir_of(data, &ctx.cwd);
    let all = projects::entries(&dir);
    if all.is_empty() {
        return String::new();
    }
    let head: String = prompt.chars().take(PROMPT_CHARS).collect();
    let picked: Vec<&Entry> = projects::matching(&all, &head, all.len())
        .into_iter()
        .filter(|e| !ctx.told.contains_key(&told(e)))
        .take(SHOWN)
        .collect();
    if picked.is_empty() {
        return String::new();
    }
    for e in &picked {
        ctx.told.insert(told(e), now());
    }
    let mut lines = vec![format!("Wisply project notes ({}, open a file there for the details):", posix(&dir))];
    lines.extend(picked.iter().map(|e| e.line.clone()));
    lines.join("\n")
}

pub(super) fn at_end(data: &Value, cwd: &str) {
    let _ = projects::auto(&dir_of(data, cwd));
}

fn io_fail(e: io::Error) -> Fail {
    Fail::new(format!("Claude memory: {e}"))
}

pub(super) fn slim(d: &Daemon, ctx: &Ctx, c: &Call) -> Result<String, Fail> {
    let dir = match c.val("dir").filter(|v| !v.is_empty()) {
        Some(v) => Path::new(&ctx.cwd).join(v),
        None => projects::memory_dir(&claude_home(), &ctx.cwd, ""),
    };
    let shown = posix(&dir);
    if !dir.join(INDEX).is_file() {
        return Err(Fail::hint(
            format!("no Claude memory here ({shown}/{INDEX})"),
            "run it in the folder Claude starts in, or wisply memory slim --dir <memory folder>",
        ));
    }
    if c.on("undo") {
        if !dir.join(PROJECTS).is_file() {
            return Ok(format!("nothing to undo: no {PROJECTS} in {shown}"));
        }
        let n = projects::undo(&dir).map_err(io_fail)?;
        return Ok(format!("put {n} project lines back into {INDEX}, {PROJECTS} is gone ({shown})"));
    }
    let dry = c.on("dry");
    let out = projects::slim(&dir, Some(&d.home.join(ARCHIVE)), dry).map_err(io_fail)?;
    if out.moved.is_empty() {
        return Ok(format!("nothing to move: {INDEX} has no project lines ({shown})"));
    }
    let (n, tokens) = (out.moved.len(), out.chars / TOKEN_CHARS);
    if dry {
        return Ok(format!(
            "would move {n} project lines to {shown}/{PROJECTS} (~{tokens} tokens less at every session start); \
             wisply memory slim does it"
        ));
    }
    let backup = out.backup.map(|b| format!(", backup {}", posix(&b))).unwrap_or_default();
    Ok(format!(
        "moved {n} project lines to {shown}/{PROJECTS} (~{tokens} tokens less at every session start{backup}). New \
         project lines move there at session end, Wisply names the matching ones when a project comes up, wisply \
         memory slim --undo puts them back."
    ))
}
