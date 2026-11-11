use std::fs;
use std::path::Path;

const COMMIT_CHARS: usize = 9;

fn commit(git: &Path) -> Option<String> {
    let head = fs::read_to_string(git.join("HEAD")).ok()?;
    let head = head.trim();
    let Some(name) = head.strip_prefix("ref: ") else { return Some(head.to_string()) };
    println!("cargo:rerun-if-changed={}", git.join(name).display());
    if let Ok(hash) = fs::read_to_string(git.join(name)) {
        return Some(hash.trim().to_string());
    }
    let packed = fs::read_to_string(git.join("packed-refs")).ok()?;
    packed.lines().find_map(|l| l.strip_suffix(name).map(|hash| hash.trim().to_string()))
}

fn main() {
    let git = Path::new("../../.git");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=assets/wisply.ico");
    println!("cargo:rerun-if-changed={}", git.join("HEAD").display());
    let hash: String = commit(git)
        .filter(|h| h.len() >= COMMIT_CHARS && h.bytes().all(|b| b.is_ascii_hexdigit()))
        .map(|h| h.chars().take(COMMIT_CHARS).collect())
        .unwrap_or_default();
    println!("cargo:rustc-env=WISPLY_COMMIT={hash}");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("assets/wisply.ico")
        .set("ProductName", "Wisply")
        .set("FileDescription", "Wisply")
        .set("CompanyName", "QDHShamiro")
        .set("LegalCopyright", "Copyright 2026 QDHShamiro, Apache-2.0");
    if let Err(e) = res.compile() {
        println!("cargo:warning=no Windows version resource: {e}");
    }
}
