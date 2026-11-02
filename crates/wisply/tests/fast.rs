use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const THIN: &str = env!("CARGO_BIN_EXE_wisply");
const FULL: &str = env!("CARGO_BIN_EXE_wisply-full");

fn dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("wisply-fast-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn exe(dir: &std::path::Path, stem: &str) -> PathBuf {
    dir.join(if cfg!(windows) { format!("{stem}.exe") } else { stem.to_string() })
}

fn run(program: &PathBuf, home: &std::path::Path, args: &[&str]) -> (i32, String, String) {
    let out = Command::new(program)
        .args(args)
        .env("WISPLY_HOME", home)
        .env_remove("MSYSTEM")
        .stdin(Stdio::null())
        .output()
        .unwrap();
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).trim_end().to_string(),
        String::from_utf8_lossy(&out.stderr).trim_end().to_string(),
    )
}

#[test]
fn a_missing_full_client_is_reported_once_and_never_loops() {
    let dir = dir("alone");
    let thin = exe(&dir, "wisply");
    fs::copy(THIN, &thin).unwrap();
    let home = dir.join("home");
    for args in [&["version"][..], &["setup"], &["mcp"], &["status"]] {
        let (code, _, err) = run(&thin, &home, args);
        assert_eq!(code, 127, "{args:?}: {err}");
        assert!(err.contains("wisply-full") && err.contains("wisply setup"), "{err}");
    }
    fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn the_small_client_hands_the_rest_to_the_full_one() {
    let dir = dir("pair");
    let thin = exe(&dir, "wisply");
    fs::copy(THIN, &thin).unwrap();
    fs::copy(FULL, exe(&dir, "wisply-full")).unwrap();
    let home = dir.join("home");
    let (code, out, _) = run(&thin, &home, &["version"]);
    assert_eq!((code, out.as_str()), (0, env!("CARGO_PKG_VERSION")));
    fs::remove_dir_all(&dir).unwrap();
}
