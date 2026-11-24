use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use sha2::{Digest, Sha256};

use super::Kind;
use super::state;

const LLAMA: &str = "https://github.com/ggml-org/llama.cpp/releases/download/b12676";
const WHISPER: &str = "https://github.com/ggml-org/whisper.cpp/releases/download/b5907";
const QWEN: &str = "https://huggingface.co/unsloth/Qwen3.5-0.8B-GGUF/resolve/6ab461498e2023f6e3c1baea90a8f0fe38ab64d0";
const SMALL: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/5359861c739e955e79d9a303bcbc70fb988958b1";
const RECV_TIMEOUT_S: u64 = 90;
const STEP_PERCENT: u64 = 20;
const CHUNK: usize = 1 << 16;
const MB: u64 = 1 << 20;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Form {
    Archive,
    File,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub part: Kind,
    pub form: Form,
    pub name: String,
    pub url: String,
    pub sha256: &'static str,
    pub size: u64,
    pub dest: &'static str,
}

fn item(part: Kind, form: Form, name: &str, url: String, sha256: &'static str, size: u64, dest: &'static str) -> Item {
    Item { part, form, name: name.to_string(), url, sha256, size, dest }
}

fn llama_archive() -> Option<Item> {
    let (file, sha256, size) = if cfg!(windows) {
        (
            "llama-b12676-bin-win-cpu-x64.zip",
            "fdf78a3924fe92ccfb24af58544a6dccbc1feeb0587e77fca9ec3bef6737086c",
            25_216_675,
        )
    } else if cfg!(target_os = "macos") {
        (
            "llama-b12676-bin-macos-arm64.tar.gz",
            "a222d1ba06a3f0215fa7d22f372af6ba74c5b4cfca5f6b2aa6974c7b63017b59",
            16_820_228,
        )
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        (
            "llama-b12676-bin-ubuntu-x64.tar.gz",
            "c65369a8574d096d90c593fef0e77a1ba2abfdfa7e7e9426bbf8939781ef8a07",
            23_378_348,
        )
    } else {
        return None;
    };
    Some(item(Kind::Llm, Form::Archive, "llama.cpp b12676", format!("{LLAMA}/{file}"), sha256, size, "bin/llama"))
}

fn whisper_archive() -> Option<Item> {
    let (file, sha256, size) = if cfg!(windows) {
        ("whisper-bin-x64.zip", "ecca07b277d943e9b9f4712856a144b857271a6b333104e511e1988e80db582b", 9_168_428)
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        (
            "whisper-bin-ubuntu-x64.tar.gz",
            "a6199027fe5b4239c84171d2bca4c0ff44ec86a515494d5a39b237b212d429bd",
            10_413_708,
        )
    } else {
        return None;
    };
    Some(item(Kind::Asr, Form::Archive, "whisper.cpp b5907", format!("{WHISPER}/{file}"), sha256, size, "bin/whisper"))
}

pub fn table() -> Vec<Item> {
    let qwen = item(
        Kind::Llm,
        Form::File,
        "Qwen3.5-0.8B Q4_K_M",
        format!("{QWEN}/{}", model_file(Kind::Llm)),
        "bd258782e35f7f458f8aced1adc053e6e92e89bc735ba3be89d38a06121dc517",
        532_517_120,
        "models",
    );
    let small = item(
        Kind::Asr,
        Form::File,
        "whisper small q5_1",
        format!("{SMALL}/{}", model_file(Kind::Asr)),
        "ae85e4a935d7a567bd102fe55afc16bb595bdb618e11b2fc7591bc08120411bb",
        190_085_487,
        "models",
    );
    [llama_archive(), Some(qwen), whisper_archive(), Some(small)].into_iter().flatten().collect()
}

pub fn model_file(kind: Kind) -> &'static str {
    match kind {
        Kind::Llm => "Qwen3.5-0.8B-Q4_K_M.gguf",
        Kind::Asr => "ggml-small-q5_1.bin",
    }
}

pub fn server_name(kind: Kind) -> &'static str {
    match (kind, cfg!(windows)) {
        (Kind::Llm, true) => "llama-server.exe",
        (Kind::Llm, false) => "llama-server",
        (Kind::Asr, true) => "whisper-server.exe",
        (Kind::Asr, false) => "whisper-server",
    }
}

pub fn bin_dir(home: &Path, kind: Kind) -> PathBuf {
    state::dir(home).join(match kind {
        Kind::Llm => "bin/llama",
        Kind::Asr => "bin/whisper",
    })
}

pub fn model_path(home: &Path, kind: Kind) -> PathBuf {
    state::dir(home).join("models").join(model_file(kind))
}

fn find(dir: &Path, name: &str, depth: u32) -> Option<PathBuf> {
    let mut subs = Vec::new();
    for entry in fs::read_dir(dir).ok()?.flatten() {
        let path = entry.path();
        if path.is_file() && path.file_name().is_some_and(|n| n == name) {
            return Some(path);
        }
        if path.is_dir() {
            subs.push(path);
        }
    }
    if depth == 0 {
        return None;
    }
    subs.iter().find_map(|d| find(d, name, depth - 1))
}

fn in_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path).map(|d| d.join(name)).find(|p| p.is_file())
}

pub fn server_exe(home: &Path, kind: Kind) -> Option<PathBuf> {
    find(&bin_dir(home, kind), server_name(kind), 2).or_else(|| in_path(server_name(kind)))
}

fn pinned(dir: &Path, sha: &str) -> bool {
    fs::read_to_string(dir.join(".pin")).is_ok_and(|t| t.trim() == sha)
}

fn have(home: &Path, it: &Item) -> bool {
    let root = state::dir(home).join(it.dest);
    match it.form {
        Form::Archive => pinned(&root, it.sha256),
        Form::File => model_path(home, it.part).metadata().is_ok_and(|m| m.len() == it.size),
    }
}

pub fn installed(home: &Path, kind: Kind) -> bool {
    let wanted: Vec<Item> = table().into_iter().filter(|i| i.part == kind).collect();
    !wanted.is_empty() && wanted.iter().all(|i| have(home, i)) && server_exe(home, kind).is_some()
}

pub fn hash_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|e| format!("cannot read {} ({e})", path.display()))?;
    let mut digest = Sha256::new();
    let mut buf = vec![0u8; CHUNK];
    loop {
        let n = file.read(&mut buf).map_err(|e| format!("cannot read {} ({e})", path.display()))?;
        if n == 0 {
            break;
        }
        digest.update(&buf[..n]);
    }
    Ok(digest.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

pub fn seal(part: &Path, done: &Path, want: &str) -> Result<(), String> {
    let got = hash_file(part)?;
    if got != want {
        let _ = fs::remove_file(part);
        return Err(format!(
            "sha256 mismatch for {} (got {}, want {}), file deleted",
            done.display(),
            &got[..12],
            &want[..12]
        ));
    }
    fs::rename(part, done).map_err(|e| format!("cannot move {} ({e})", done.display()))
}

fn fetch(url: &str, part: &Path, size: u64, name: &str, say: &mut dyn FnMut(String)) -> Result<(), String> {
    let agent: ureq::Agent =
        ureq::Agent::config_builder().timeout_recv_body(Some(Duration::from_secs(RECV_TIMEOUT_S))).build().into();
    let mut resp = agent.get(url).call().map_err(|e| format!("download of {name} failed ({e})"))?;
    let mut reader = resp.body_mut().with_config().limit(u64::MAX).reader();
    let mut out = File::create(part).map_err(|e| format!("cannot write {} ({e})", part.display()))?;
    let (mut got, mut next, mut buf) = (0u64, STEP_PERCENT, vec![0u8; CHUNK]);
    loop {
        let n = reader.read(&mut buf).map_err(|e| format!("download of {name} broke off ({e})"))?;
        if n == 0 {
            break;
        }
        out.write_all(&buf[..n]).map_err(|e| format!("cannot write {} ({e})", part.display()))?;
        got += n as u64;
        while size > 0 && got * 100 / size >= next && next < 100 {
            say(format!("local: {name} {next}% ({} MB)", got / MB));
            next += STEP_PERCENT;
        }
    }
    out.flush().map_err(|e| e.to_string())
}

fn tar_exe() -> PathBuf {
    if cfg!(windows) {
        let root = std::env::var_os("SystemRoot").map_or_else(|| PathBuf::from("C:/Windows"), PathBuf::from);
        let sys = root.join("System32").join("tar.exe");
        if sys.is_file() {
            return sys;
        }
    }
    PathBuf::from("tar")
}

fn unpack(archive: &Path, dest: &Path) -> Result<(), String> {
    let _ = fs::remove_dir_all(dest);
    fs::create_dir_all(dest).map_err(|e| format!("cannot create {} ({e})", dest.display()))?;
    let out = Command::new(tar_exe())
        .arg("-xf")
        .arg(archive)
        .arg("-C")
        .arg(dest)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("cannot run tar ({e})"))?;
    if out.status.success() {
        return Ok(());
    }
    let _ = fs::remove_dir_all(dest);
    Err(format!("tar failed: {}", String::from_utf8_lossy(&out.stderr).lines().last().unwrap_or("")))
}

fn install(home: &Path, it: &Item, say: &mut dyn FnMut(String)) -> Result<(), String> {
    let root = state::dir(home);
    let dl = root.join("dl");
    fs::create_dir_all(&dl).map_err(|e| format!("cannot create {} ({e})", dl.display()))?;
    let leaf = it.url.rsplit('/').next().unwrap_or("download");
    let (part, done) = (dl.join(format!("{leaf}.part")), dl.join(leaf));
    match it.form {
        Form::File => {
            let target = model_path(home, it.part);
            let _ = fs::create_dir_all(target.parent().unwrap_or(&root));
            fetch(&it.url, &part, it.size, &it.name, say)?;
            seal(&part, &done, it.sha256)?;
            fs::rename(&done, &target).map_err(|e| format!("cannot move {} ({e})", target.display()))
        }
        Form::Archive => {
            let dest = root.join(it.dest);
            fetch(&it.url, &part, it.size, &it.name, say)?;
            seal(&part, &done, it.sha256)?;
            let fresh = dest.with_extension("new");
            unpack(&done, &fresh)?;
            let _ = fs::remove_dir_all(&dest);
            fs::rename(&fresh, &dest).map_err(|e| format!("cannot move {} ({e})", dest.display()))?;
            let _ = fs::remove_file(&done);
            fs::write(dest.join(".pin"), it.sha256).map_err(|e| format!("cannot write the pin ({e})"))
        }
    }
}

fn verified(home: &Path, it: &Item) -> bool {
    match it.form {
        Form::File => have(home, it) && hash_file(&model_path(home, it.part)).is_ok_and(|h| h == it.sha256),
        Form::Archive => have(home, it),
    }
}

pub fn run(home: &Path, only: Option<Kind>, say: &mut dyn FnMut(String)) -> Result<(), String> {
    let items: Vec<Item> = table().into_iter().filter(|i| only.is_none_or(|k| k == i.part)).collect();
    if items.is_empty() {
        return Err("no local programs for this system, install llama-server and whisper-server yourself".into());
    }
    for it in &items {
        if verified(home, it) {
            say(format!("local: {} is there", it.name));
            continue;
        }
        say(format!("local: loading {} ({} MB)", it.name, it.size / MB));
        install(home, it, say)?;
        say(format!("local: {} ok", it.name));
    }
    let missing: Vec<&str> = [Kind::Llm, Kind::Asr]
        .into_iter()
        .filter(|k| only.is_none_or(|o| o == *k) && server_exe(home, *k).is_none())
        .map(Kind::label)
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("{} server not found, on macOS: brew install whisper-cpp", missing.join(" and ")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("wisply-local-get-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn a_wrong_hash_deletes_the_file_and_says_so() {
        let dir = temp("hash");
        let (part, done) = (dir.join("a.part"), dir.join("a.bin"));
        fs::write(&part, b"abc").unwrap();
        let want = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
        assert_eq!(hash_file(&part).unwrap(), want);
        let bad = "0".repeat(64);
        let err = seal(&part, &done, &bad).unwrap_err();
        assert!(err.contains("sha256 mismatch") && !part.exists() && !done.exists());
        fs::write(&part, b"abc").unwrap();
        seal(&part, &done, want).unwrap();
        assert!(done.is_file() && !part.exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_table_is_pinned_with_full_hashes_and_commit_urls() {
        for it in table() {
            assert_eq!(it.sha256.len(), 64, "{}", it.name);
            assert!(it.sha256.chars().all(|c| c.is_ascii_hexdigit()));
            assert!(it.url.starts_with("https://"));
            assert!(!it.url.contains("/main/"), "{}", it.url);
            if it.form == Form::File {
                assert!(it.url.contains("/resolve/") && it.url.split("/resolve/").nth(1).is_some_and(|r| r.len() > 40));
            }
        }
        assert!(table().iter().any(|i| i.part == Kind::Llm && i.form == Form::File));
        assert!(table().iter().any(|i| i.part == Kind::Asr && i.form == Form::File));
    }

    #[test]
    fn programs_are_found_inside_the_unpacked_folder() {
        let home = temp("find");
        let inner = bin_dir(&home, Kind::Llm).join("llama-b12676");
        fs::create_dir_all(&inner).unwrap();
        fs::write(inner.join(server_name(Kind::Llm)), b"x").unwrap();
        assert!(server_exe(&home, Kind::Llm).is_some_and(|p| p.starts_with(&inner)));
        let _ = fs::remove_dir_all(&home);
    }

    #[test]
    fn installed_needs_the_pin_the_model_and_the_program() {
        let home = temp("installed");
        assert!(!installed(&home, Kind::Llm));
        let _ = fs::remove_dir_all(&home);
    }
}
