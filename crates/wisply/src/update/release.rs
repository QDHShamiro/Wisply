use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::Value;

use crate::daemon::net;
use crate::setup::place::exe_name;

const REPO: &str = "QDHShamiro/Wisply";
const SUMS: &str = "SHA256SUMS.txt";

pub struct Release {
    pub version: String,
    pub raw: Value,
}

pub fn parse(tag: &str) -> Option<(u64, u64, u64)> {
    let mut parts = tag.trim().trim_start_matches('v').split('.');
    let mut next = || parts.next()?.parse::<u64>().ok();
    let found = (next()?, next()?, next()?);
    parts.next().is_none().then_some(found)
}

pub fn newer(have: &str, tag: &str) -> bool {
    matches!((parse(have), parse(tag)), (Some(a), Some(b)) if b > a)
}

pub fn latest() -> Result<Release, String> {
    let raw = net::api(&format!("{REPO}/releases/latest"))?;
    let tag = raw["tag_name"].as_str().unwrap_or("");
    let version = parse(tag)
        .map(|(a, b, c)| format!("{a}.{b}.{c}"))
        .ok_or_else(|| format!("release tag '{tag}' is no version"))?;
    Ok(Release { version, raw })
}

pub fn pack() -> Option<(&'static str, &'static str)> {
    match (env::consts::OS, env::consts::ARCH) {
        ("windows", "x86_64") => Some(("windows-x64", "zip")),
        ("macos", _) => Some(("macos-universal", "tar.gz")),
        ("linux", "x86_64") => Some(("linux-x64", "tar.gz")),
        ("linux", "aarch64") => Some(("linux-arm64", "tar.gz")),
        _ => None,
    }
}

fn tar() -> PathBuf {
    let system = env::var_os("SystemRoot").map(|r| PathBuf::from(r).join("System32").join("tar.exe"));
    system.filter(|p| cfg!(windows) && p.is_file()).unwrap_or_else(|| PathBuf::from("tar"))
}

fn unpack(archive: &Path, into: &Path) -> Result<(), String> {
    let mut cmd = Command::new(tar());
    cmd.arg("-xf").arg(archive).arg("-C").arg(into).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::piped());
    crate::v2::hide(&mut cmd);
    let out = cmd.output().map_err(|e| format!("cannot run tar ({e})"))?;
    if out.status.success() {
        Ok(())
    } else {
        Err(format!("tar could not unpack the release: {}", String::from_utf8_lossy(&out.stderr).trim()))
    }
}

pub fn runs_as(exe: &Path, version: &str) -> bool {
    let mut cmd = Command::new(exe);
    cmd.arg("--version").stdin(Stdio::null()).stderr(Stdio::null());
    crate::v2::hide(&mut cmd);
    cmd.output().is_ok_and(|o| o.status.success() && String::from_utf8_lossy(&o.stdout).trim() == version)
}

pub fn stage(home: &Path, release: &Release) -> Result<PathBuf, String> {
    let (pack, ext) = pack().ok_or("this system has no release build, update by hand")?;
    let v = &release.version;
    let name = format!("wisply-{v}-{pack}.{ext}");
    let sums = net::text(&net::asset_url(&release.raw, SUMS)?)?;
    let want = net::sum_for(&sums, &name).ok_or_else(|| format!("{SUMS} lists no {name}"))?;
    let work = home.join("update").join(v);
    let _ = fs::remove_dir_all(&work);
    fs::create_dir_all(&work).map_err(|e| format!("cannot write {} ({e})", work.display()))?;
    let archive = work.join(&name);
    net::download(&net::asset_url(&release.raw, &name)?, &archive, &want)?;
    unpack(&archive, &work)?;
    let _ = fs::remove_file(&archive);
    let root = work.join(format!("wisply-{v}-{pack}"));
    let exe = root.join(exe_name("wisply"));
    if !runs_as(&exe, v) {
        let _ = fs::remove_dir_all(&work);
        return Err(format!("the unpacked {} does not run as {v}", exe_name("wisply")));
    }
    Ok(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_compare_by_number_and_odd_tags_are_ignored() {
        assert_eq!(parse("v3.0.1"), Some((3, 0, 1)));
        assert_eq!(parse("3.10.0"), Some((3, 10, 0)));
        assert!(newer("3.0.1", "v3.1.0") && newer("3.9.0", "3.10.0") && newer("3.0.1", "4.0.0"));
        assert!(!newer("3.1.0", "v3.1.0") && !newer("3.1.0", "3.0.9") && !newer("3.1.0", ""));
        for odd in ["v3.1", "v3.1.0-rc1", "nightly", "3.1.0.1", "3.x.0"] {
            assert_eq!(parse(odd), None, "{odd}");
        }
    }

    #[test]
    fn the_release_pack_matches_the_workflow_names() {
        let (pack, ext) = pack().unwrap();
        assert!(["windows-x64", "macos-universal", "linux-x64", "linux-arm64"].contains(&pack));
        assert_eq!(ext, if cfg!(windows) { "zip" } else { "tar.gz" });
    }
}
