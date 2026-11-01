use std::fmt::Write as _;
use std::path::Path;
use std::{env, fs};

fn kind(name: &str) -> Option<&'static str> {
    match name.rsplit('.').next()? {
        "html" => Some("text/html; charset=utf-8"),
        "js" => Some("text/javascript; charset=utf-8"),
        "css" => Some("text/css; charset=utf-8"),
        "svg" => Some("image/svg+xml"),
        _ => None,
    }
}

fn assets() {
    let dist = Path::new(&env::var("CARGO_MANIFEST_DIR").expect("manifest dir")).join("web").join("dist");
    println!("cargo:rerun-if-changed={}", dist.display());
    let mut names: Vec<String> = fs::read_dir(&dist)
        .map(|d| {
            d.filter_map(Result::ok)
                .filter(|e| e.path().is_file())
                .filter_map(|e| e.file_name().into_string().ok())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    let mut out = String::from("&[\n");
    for name in names {
        let Some(kind) = kind(&name) else { continue };
        let path = dist.join(&name).display().to_string();
        let _ = writeln!(out, "    (\"/{name}\", \"{kind}\", include_bytes!({path:?})),");
    }
    out.push(']');
    fs::write(Path::new(&env::var("OUT_DIR").expect("out dir")).join("files.rs"), out).expect("write files.rs");
}

fn main() {
    assets();
    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    res.set_icon("wisply.ico")
        .set("ProductName", "Wisply")
        .set("FileDescription", "Wisply window")
        .set("CompanyName", "QDHShamiro")
        .set("LegalCopyright", "Copyright 2026 QDHShamiro, Apache-2.0");
    if let Err(e) = res.compile() {
        println!("cargo:warning=no Windows version resource: {e}");
    }
}
