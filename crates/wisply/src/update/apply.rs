use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::thread;
use std::time::Duration;

use super::state;
use crate::daemon::net;
use crate::setup::place::{ALIAS, EXES, copy_tree, exe_name};

const TRIES: usize = 3;
const PAUSE: Duration = Duration::from_secs(3);

fn run(exe: &Path, args: &[&str]) -> Result<String, String> {
    let mut cmd = Command::new(exe);
    cmd.args(args).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
    crate::v2::hide(&mut cmd);
    let out = cmd.output().map_err(|e| format!("cannot start {} ({e})", exe.display()))?;
    let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if out.status.success() {
        Ok(text)
    } else {
        let err = String::from_utf8_lossy(&out.stderr);
        Err(format!("{} {} failed: {}", exe_name("wisply"), args.join(" "), err.lines().last().unwrap_or(&text)))
    }
}

fn bin(home: &Path) -> PathBuf {
    home.join("bin").join(exe_name("wisply"))
}

fn serves(home: &Path, version: &str) -> bool {
    for _ in 0..TRIES {
        let first = run(&bin(home), &["status"]).ok().and_then(|t| t.lines().next().map(String::from));
        if first.is_some_and(|l| l.starts_with(&format!("wisply {version} |"))) {
            return true;
        }
        thread::sleep(PAUSE);
    }
    false
}

pub fn swap_back(bin_dir: &Path) -> usize {
    let mut moved = 0;
    for stem in EXES.iter().chain([&ALIAS]) {
        let dst = bin_dir.join(exe_name(stem));
        let old = dst.with_extension("old");
        if !old.is_file() {
            continue;
        }
        let _ = fs::remove_file(&dst);
        if fs::rename(&old, &dst).or_else(|_| fs::copy(&old, &dst).map(|_| ())).is_ok() {
            moved += 1;
        }
    }
    moved
}

pub fn rollback(home: &Path) -> Result<String, String> {
    let exe = bin(home);
    let _ = run(&exe, &["stop"]);
    thread::sleep(Duration::from_secs(1));
    let moved = swap_back(&home.join("bin"));
    if moved == 0 {
        return Err("no earlier version is kept (bin/*.old is missing)".into());
    }
    let prev = home.join("update").join("extension-prev");
    if prev.join("manifest.json").is_file() {
        let _ = copy_tree(&prev, &home.join("extension"));
    }
    let _ = run(&exe, &["status"]);
    let now = run(&exe, &["version"]).unwrap_or_default();
    Ok(format!("back to wisply {now} ({moved} programs restored, the daemon restarts on the next command)"))
}

pub fn launch(home: &Path, root: &Path, from: &str, to: &str) -> Result<(), String> {
    let dir = home.join("update");
    fs::create_dir_all(&dir).map_err(|e| format!("cannot write {} ({e})", dir.display()))?;
    if let Ok(entries) = fs::read_dir(&dir) {
        for e in entries.flatten().filter(|e| e.file_name().to_string_lossy().starts_with("helper-")) {
            let _ = fs::remove_file(e.path());
        }
    }
    let me = std::env::current_exe().map_err(|e| e.to_string())?;
    let helper = dir.join(format!("helper-{}{}", std::process::id(), if cfg!(windows) { ".exe" } else { "" }));
    fs::copy(&me, &helper).map_err(|e| format!("cannot copy the helper ({e})"))?;
    let root_arg = root.to_string_lossy().into_owned();
    let args = ["update", "--apply", root_arg.as_str(), from, to];
    if crate::spawn::detached(&helper, &args, home, &home.join("update.log")) {
        Ok(())
    } else {
        Err("could not start the update helper".into())
    }
}

fn swapped(home: &Path, new: &Path) -> bool {
    let (a, b) = (net::sha256_file(&bin(home)), net::sha256_file(new));
    a.is_some() && a == b
}

fn fail(home: &Path, from: &str, to: &str, why: &str, undo: bool) -> ExitCode {
    let why = if undo {
        match rollback(home) {
            Ok(_) => why.to_string(),
            Err(e) => format!("{why}; rollback: {e}"),
        }
    } else {
        let _ = run(&bin(home), &["status"]);
        why.to_string()
    };
    println!("update to {to} failed, {why}");
    state::finished(home, from, to, false, &why);
    ExitCode::from(1)
}

pub fn helper(home: &Path, args: &[String]) -> ExitCode {
    let [root, from, to] = args else { return ExitCode::from(2) };
    let new = Path::new(root).join(exe_name("wisply"));
    let ext = home.join("extension");
    let prev = home.join("update").join("extension-prev");
    let _ = fs::remove_dir_all(&prev);
    if ext.join("manifest.json").is_file() {
        let _ = copy_tree(&ext, &prev);
    }
    if let Err(e) = run(&new, &["setup", "--manual"]) {
        return fail(home, from, to, &e, swapped(home, &new));
    }
    if !serves(home, to) {
        return fail(home, from, to, "the new daemon did not answer as the new version", true);
    }
    let _ = run(&bin(home), &["ext-reload"]);
    state::finished(home, from, to, true, "");
    let _ = fs::remove_dir_all(home.join("update").join(to));
    println!("updated wisply {from} -> {to}");
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_back_puts_the_kept_programs_over_the_new_ones() {
        let dir = std::env::temp_dir().join(format!("wisply-swap-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let name = exe_name("wisply");
        fs::write(dir.join(&name), "new").unwrap();
        fs::write(dir.join(&name).with_extension("old"), "old").unwrap();
        fs::write(dir.join(exe_name("wisply-ui")), "ui-new").unwrap();
        fs::write(dir.join(exe_name("wisply-full")), "full-new").unwrap();
        fs::write(dir.join(exe_name("wisply-full")).with_extension("old"), "full-old").unwrap();
        assert_eq!(swap_back(&dir), 2);
        assert_eq!(fs::read_to_string(dir.join(&name)).unwrap(), "old");
        assert_eq!(fs::read_to_string(dir.join(exe_name("wisply-ui"))).unwrap(), "ui-new");
        assert_eq!(fs::read_to_string(dir.join(exe_name("wisply-full"))).unwrap(), "full-old");
        assert!(!dir.join(&name).with_extension("old").exists());
        assert_eq!(swap_back(&dir), 0);
        fs::remove_dir_all(&dir).unwrap();
    }
}
