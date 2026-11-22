use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::UNIX_EPOCH;

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use wisply_core::Fail;

const STAMP: &str = "built.json";
const BIN: &str = "bin";
const TARGET: &str = "target";
const EDITION: &str = "2021";
const ERR_LINES: usize = 30;
const WALK_DEPTH: usize = 8;
const PRINT_BYTES: usize = 12;
const NO_RUST: &str = "write it as JS in wisply codemode instead, or install Rust from https://rustup.rs";
const WHAT: &str = "--rust takes a .rs file or a folder with a Cargo.toml";

pub enum Source {
    File(PathBuf),
    Cargo(PathBuf),
}

pub fn source(dir: &Path, spec: &Value) -> Option<Source> {
    match spec["rust"].as_str()? {
        "file" => Some(Source::File(dir.join(spec["file"].as_str()?))),
        "cargo" => Some(Source::Cargo(PathBuf::from(spec["crate"].as_str()?))),
        _ => None,
    }
}

pub fn given(path: &str) -> Result<Option<PathBuf>, Fail> {
    let p = Path::new(path);
    if p.join("Cargo.toml").is_file() {
        return std::path::absolute(p).map(Some).map_err(|e| Fail::new(format!("cannot use {path} ({e})")));
    }
    if p.is_file() && p.extension().is_some_and(|e| e.eq_ignore_ascii_case("rs")) {
        return Ok(None);
    }
    Err(Fail::hint(format!("{path} is neither a .rs file nor a Cargo folder"), WHAT))
}

pub fn code(src: &Source) -> String {
    let file = match src {
        Source::File(p) => p.clone(),
        Source::Cargo(p) => p.join("src").join("main.rs"),
    };
    fs::read_to_string(file).unwrap_or_default()
}

fn walk(dir: &Path, depth: usize, out: &mut Vec<(String, u64, u128)>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        let path = e.path();
        if path.is_dir() {
            if depth > 0 && name != TARGET && !name.starts_with('.') {
                walk(&path, depth - 1, out);
            }
            continue;
        }
        let kept = name.ends_with(".rs") || name == "Cargo.toml" || name == "Cargo.lock";
        if let (true, Ok(m)) = (kept, e.metadata()) {
            let at = m.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map_or(0, |d| d.as_nanos());
            out.push((path.to_string_lossy().into_owned(), m.len(), at));
        }
    }
}

fn print(src: &Source) -> Result<String, Fail> {
    let mut h = Sha256::new();
    match src {
        Source::File(p) => h.update(fs::read(p).map_err(|e| Fail::new(format!("cannot read {} ({e})", p.display())))?),
        Source::Cargo(p) => {
            let mut files = Vec::new();
            walk(p, WALK_DEPTH, &mut files);
            if files.is_empty() {
                return Err(Fail::hint(format!("{} has no Rust code any more", p.display()), WHAT));
            }
            files.sort();
            for (path, len, at) in files {
                h.update(format!("{path}\0{len}\0{at}\n").as_bytes());
            }
        }
    }
    Ok(h.finalize().iter().take(PRINT_BYTES).map(|b| format!("{b:02x}")).collect())
}

fn started(tool: &str, cmd: &mut Command) -> Result<Output, Fail> {
    cmd.output().map_err(|e| match e.kind() {
        ErrorKind::NotFound => Fail::hint(format!("{tool} is not installed, so a Rust tool cannot be built"), NO_RUST),
        _ => Fail::new(format!("cannot start {tool} ({e})")),
    })
}

fn failed(name: &str, out: &Output) -> Fail {
    let text = String::from_utf8_lossy(&out.stderr);
    let head: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).take(ERR_LINES).collect();
    Fail::hint(
        format!("the Rust tool '{name}' did not build:\n{}", head.join("\n")),
        "fix the code, then the same kit add with --replace",
    )
}

fn build(dir: &Path, name: &str, src: &Source) -> Result<PathBuf, Fail> {
    match src {
        Source::File(file) => {
            let bin = dir.join(BIN);
            fs::create_dir_all(&bin).map_err(|e| Fail::new(format!("cannot make {} ({e})", bin.display())))?;
            let exe = bin.join(format!("{name}{}", std::env::consts::EXE_SUFFIX));
            let crate_name = name.replace('-', "_");
            let mut cmd = Command::new("rustc");
            cmd.args(["-O", "--edition", EDITION, "--crate-name", &crate_name, "-o"]).arg(&exe).arg(file);
            let out = started("rustc", &mut cmd)?;
            if out.status.success() { Ok(exe) } else { Err(failed(name, &out)) }
        }
        Source::Cargo(krate) => {
            let mut cmd = Command::new("cargo");
            cmd.args(["build", "--release", "--message-format=json-render-diagnostics", "--manifest-path"])
                .arg(krate.join("Cargo.toml"))
                .arg("--target-dir")
                .arg(dir.join(TARGET));
            let out = started("cargo", &mut cmd)?;
            if !out.status.success() {
                return Err(failed(name, &out));
            }
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .filter_map(|l| serde_json::from_str::<Value>(l).ok())
                .filter_map(|v| v["executable"].as_str().map(PathBuf::from))
                .next_back()
                .ok_or_else(|| Fail::hint(format!("{} builds no program", krate.display()), "give it a src/main.rs"))
        }
    }
}

pub fn ready(dir: &Path, name: &str, src: &Source) -> Result<(PathBuf, bool), Fail> {
    let fresh = print(src)?;
    let stamp: Value =
        fs::read_to_string(dir.join(STAMP)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    let kept = stamp["exe"].as_str().map(PathBuf::from).filter(|p| p.is_file());
    if let Some(exe) = kept.filter(|_| stamp["print"] == fresh.as_str()) {
        return Ok((exe, false));
    }
    let exe = build(dir, name, src)?;
    let after = print(src).unwrap_or(fresh);
    let _ = wisply_core::save(dir.join(STAMP), json!({"print": after, "exe": exe.to_string_lossy()}).to_string());
    Ok((exe, true))
}
