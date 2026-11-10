use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use wisply_core::Fail;

use crate::junk::{self, Junk};
use crate::readview::whole_outline;

const WHOLE_BYTES: u64 = 5 * 1024;
const PARTS_BYTES: u64 = 50 * 1024;
const GREP_BYTES: u64 = 500 * 1024;
const COUNT_BYTES: u64 = 4 * 1024 * 1024;
const SNIFF_BYTES: usize = 8 * 1024;
const HEAD_MAX: usize = 50;
const LINE_MAX: usize = 160;
const FOLDER_ROWS: usize = 80;
const OUTLINE_ROWS: usize = 40;
const MAX_CHARS: usize = 25_000;
const USAGE: &str = "wisply peek <file|folder> ... [--head 10] [--outline]";
const LEGEND: &str = "whole = read it all, parts = Read with offset/limit, grep = search it, skip = leave it";

pub fn run(args: &[String]) -> ExitCode {
    match peek(args) {
        Ok(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Err(fail) => {
            println!("{}", fail.text());
            ExitCode::from(1)
        }
    }
}

struct Wish {
    paths: Vec<String>,
    head: usize,
    outline: bool,
}

struct Seen {
    advice: &'static str,
    lines: Option<usize>,
    bytes: u64,
    note: String,
    text: Option<String>,
}

fn wish(args: &[String]) -> Result<Wish, Fail> {
    let mut w = Wish { paths: Vec::new(), head: 0, outline: false };
    let number = |v: Option<&str>| {
        v.and_then(|n| n.parse::<usize>().ok())
            .map(|n| n.min(HEAD_MAX))
            .ok_or_else(|| Fail::hint("--head needs a number of lines", "wisply peek src/main.rs --head 10"))
    };
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--outline" => w.outline = true,
            "--head" => w.head = number(it.next().map(String::as_str))?,
            _ if a.starts_with("--head=") => w.head = number(a.strip_prefix("--head="))?,
            _ if a.starts_with("--") => return Err(Fail::hint(format!("peek does not know {a}"), USAGE)),
            _ => w.paths.push(a.clone()),
        }
    }
    if w.paths.is_empty() {
        return Err(Fail::hint("peek needs at least one file or folder", USAGE));
    }
    Ok(w)
}

fn full(path: &Path) -> PathBuf {
    std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf())
}

fn skip(bytes: u64, note: impl Into<String>) -> Seen {
    Seen { advice: "skip", lines: None, bytes, note: note.into(), text: None }
}

fn junk_note(path: &Path, kind: Junk) -> String {
    match junk::inside(&path.to_string_lossy()) {
        Some(tool) => format!("{}, its file list: {tool} <file>", kind.what()),
        None => kind.what().to_string(),
    }
}

fn look(path: &Path) -> Seen {
    let Ok(meta) = fs::metadata(path) else { return Seen { advice: "gone", ..skip(0, "no such file") } };
    let bytes = meta.len();
    if let Some(kind) = junk::of(&full(path).to_string_lossy()) {
        return skip(bytes, junk_note(path, kind));
    }
    if bytes > COUNT_BYTES {
        return skip(bytes, "too big to read, search it with grep");
    }
    let Ok(raw) = fs::read(path) else { return Seen { advice: "gone", ..skip(bytes, "cannot be opened") } };
    if raw.iter().take(SNIFF_BYTES).any(|b| *b == 0) {
        return skip(bytes, "binary data");
    }
    let text = String::from_utf8_lossy(&raw).into_owned();
    let steps = [(WHOLE_BYTES, "whole"), (PARTS_BYTES, "parts"), (GREP_BYTES, "grep")];
    let advice = steps.iter().find(|(most, _)| bytes <= *most).map_or("skip", |(_, word)| word);
    let note = if advice == "skip" { "too big to read, search it with grep" } else { "" };
    Seen { advice, lines: Some(text.lines().count()), bytes, note: note.into(), text: Some(text) }
}

fn row(indent: &str, name: &str, s: &Seen) -> String {
    let lines = s.lines.map(|n| format!("{n} L")).unwrap_or_default();
    let note = if s.note.is_empty() { String::new() } else { format!(" - {}", s.note) };
    format!("{indent}{:<5} {lines:>8} {:>9}  {name}{note}", s.advice, junk::size(s.bytes))
}

fn clip(line: &str) -> String {
    let t = line.trim_end();
    if t.chars().count() <= LINE_MAX { t.to_string() } else { t.chars().take(LINE_MAX).collect::<String>() + "..." }
}

fn extras(w: &Wish, s: &Seen, out: &mut Vec<String>) {
    let Some(text) = s.text.as_deref().filter(|_| s.advice != "skip") else { return };
    out.extend(text.lines().take(w.head).enumerate().map(|(i, l)| format!("    L{}: {}", i + 1, clip(l))));
    if w.outline {
        let marks = whole_outline(text);
        out.extend(marks.iter().take(OUTLINE_ROWS).map(|m| format!("    {m}")));
        if marks.len() > OUTLINE_ROWS {
            out.push(format!("    ... +{} more outline lines", marks.len() - OUTLINE_ROWS));
        }
    }
}

struct Sum {
    files: usize,
    tokens: u64,
    skipped: usize,
}

impl Sum {
    fn add(&mut self, s: &Seen) {
        self.files += 1;
        if s.advice == "skip" || s.advice == "gone" {
            self.skipped += 1;
        } else {
            self.tokens += s.bytes / 4;
        }
    }
}

fn folder(name: &str, dir: &Path, sum: &mut Sum, out: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        out.push(format!("gone  {name} - cannot be opened"));
        return;
    };
    let (mut dirs, mut files) = (Vec::new(), Vec::new());
    for e in entries.flatten() {
        let label = e.file_name().to_string_lossy().into_owned();
        if e.file_type().is_ok_and(|t| t.is_dir()) { dirs.push(label) } else { files.push(label) }
    }
    dirs.sort_unstable();
    files.sort_unstable();
    out.push(format!("folder {}: {} files, {} folders", name.trim_end_matches(['/', '\\']), files.len(), dirs.len()));
    let whole = full(dir);
    let mut rows = Vec::new();
    for d in &dirs {
        rows.push(match junk::folder(&whole, d) {
            Some(kind) => format!("  skip  {d}/ - {}", kind.what()),
            None => format!("  dir   {d}/"),
        });
    }
    for f in &files {
        if rows.len() >= FOLDER_ROWS {
            break;
        }
        let seen = look(&dir.join(f));
        sum.add(&seen);
        rows.push(row("  ", f, &seen));
    }
    let all = dirs.len() + files.len();
    rows.truncate(FOLDER_ROWS);
    if all > rows.len() {
        rows.push(format!("  ... +{} more, name them or peek a subfolder", all - rows.len()));
    }
    out.append(&mut rows);
}

fn tokens(n: u64) -> String {
    if n < 1000 { n.to_string() } else { format!("{:.1}k", n as f64 / 1000.0) }
}

fn peek(args: &[String]) -> Result<String, Fail> {
    let w = wish(args)?;
    let mut sum = Sum { files: 0, tokens: 0, skipped: 0 };
    let mut out = Vec::new();
    for name in &w.paths {
        let path = Path::new(name);
        if path.is_dir() {
            folder(name, path, &mut sum, &mut out);
            continue;
        }
        let seen = look(path);
        sum.add(&seen);
        out.push(row("", name, &seen));
        extras(&w, &seen, &mut out);
    }
    let left = if sum.skipped == 0 { String::new() } else { format!(", {} to leave alone", sum.skipped) };
    let mut text = format!("{} files, ~{} tokens if read whole{left} | {LEGEND}", sum.files, tokens(sum.tokens));
    for line in &out {
        if text.len() + line.len() > MAX_CHARS {
            text.push_str("\n... cut here, peek fewer paths at once");
            break;
        }
        text.push('\n');
        text.push_str(line);
    }
    Ok(text)
}

#[cfg(test)]
#[path = "peek/tests.rs"]
mod tests;
