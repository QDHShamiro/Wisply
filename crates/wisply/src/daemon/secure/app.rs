use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::asar;
use super::axml::{self, Event};
use super::report::{Finding, Report, Sev};
use super::rules::{self, Scope};
use super::scan::{secret_sev, secrets_in_line};
use super::zip::Zip;

const ENTRY_LIMIT: u64 = 40_000_000;
const TEXT_EXT: &[&str] = &["yml", "yaml", "json", "properties", "txt", "xml", "toml", "cfg", "conf", "ini", "env"];
const COMPONENTS: &[&str] = &["activity", "activity-alias", "service", "receiver", "provider"];
const MIN_RUN: usize = 12;
const MAX_RUN: usize = 4096;
const CHUNK: usize = 1 << 20;
const SECRET_FIX: &str = "remove it from the build, load it at runtime from a server or the user, then rotate it";

pub fn kind(path: &Path) -> Option<&'static str> {
    match path.extension()?.to_str()?.to_lowercase().as_str() {
        "apk" => Some("apk"),
        "jar" => Some("jar"),
        "exe" => Some("exe"),
        "asar" => Some("asar"),
        _ => None,
    }
}

pub fn strings(mut source: impl Read, min: usize, mut each: impl FnMut(&str)) {
    let mut buf = vec![0u8; CHUNK];
    let mut run: Vec<u8> = Vec::new();
    while let Ok(n) = source.read(&mut buf) {
        if n == 0 {
            break;
        }
        for byte in &buf[..n] {
            if (0x20..0x7f).contains(byte) {
                run.push(*byte);
                if run.len() >= MAX_RUN {
                    each(&String::from_utf8_lossy(&run));
                    run.clear();
                }
            } else {
                if run.len() >= min {
                    each(&String::from_utf8_lossy(&run));
                }
                run.clear();
            }
        }
    }
    if run.len() >= min {
        each(&String::from_utf8_lossy(&run));
    }
}

fn secret(found: &mut Vec<Finding>, file: &str, line: usize, why: String) {
    found.push(Finding::new(secret_sev(&why), "secret", file, line, why, SECRET_FIX));
}

pub fn scan_text(found: &mut Vec<Finding>, file: &str, text: &str, config: bool, code: bool) {
    let scope = if config { Scope::Config } else { Scope::Code };
    for (i, line) in text.lines().enumerate() {
        for why in secrets_in_line(line, config) {
            secret(found, file, i + 1, why);
        }
        if (config || code)
            && let Some(rule) = rules::judge(line, scope)
        {
            found.push(Finding::new(rule.sev, "code", file, i + 1, rule.why, rule.fix));
        }
    }
}

pub fn scan_bytes(found: &mut Vec<Finding>, file: &str, data: &[u8], min: usize) {
    strings(data, min, |s| {
        for why in secrets_in_line(s, false) {
            secret(found, file, 0, why);
        }
    });
}

fn extension(name: &str) -> String {
    name.rsplit_once('.').map(|(_, e)| e.to_lowercase()).unwrap_or_default()
}

fn commands_without_permission(found: &mut Vec<Finding>, file: &str, text: &str) {
    let (mut inside, mut current, mut line_at, mut has) = (false, String::new(), 0, true);
    let flush = |found: &mut Vec<Finding>, current: &str, has: bool, at: usize| {
        if !current.is_empty() && !has {
            found.push(Finding::new(
                Sev::Low,
                "app",
                file,
                at,
                format!("command /{current} has no permission"),
                "add permission: <node> under the command so only allowed players can run it",
            ));
        }
    };
    for (i, line) in text.lines().enumerate() {
        let indent = line.len() - line.trim_start().len();
        let body = line.trim();
        if indent == 0 && !body.is_empty() {
            flush(found, &current, has, line_at);
            current.clear();
            inside = body.starts_with("commands:");
        } else if inside && indent == 2 && body.ends_with(':') {
            flush(found, &current, has, line_at);
            current = body.trim_end_matches(':').to_string();
            (line_at, has) = (i + 1, false);
        } else if inside && body.starts_with("permission:") {
            has = true;
        }
    }
    flush(found, &current, has, line_at);
}

pub fn jar(path: &Path, report: &mut Report) -> Result<(), String> {
    let mut zip = Zip::open(path)?;
    let jar = path.file_name().and_then(|n| n.to_str()).unwrap_or("plugin.jar").to_string();
    let (mut classes, mut other) = (0, 0);
    for at in 0..zip.entries.len() {
        let name = zip.entries[at].name.clone();
        let ext = extension(&name);
        let file = format!("{jar}!{name}");
        let is_class = ext == "class";
        let is_text = TEXT_EXT.contains(&ext.as_str());
        if !is_class && !is_text {
            continue;
        }
        let Some(data) = zip.read(at, ENTRY_LIMIT) else { continue };
        if is_class {
            classes += 1;
            scan_bytes(&mut report.found, &file, &data, MIN_RUN);
        } else {
            other += 1;
            let text = String::from_utf8_lossy(&data);
            scan_text(&mut report.found, &file, &text, true, false);
            if matches!(name.as_str(), "plugin.yml" | "paper-plugin.yml" | "bungee.yml") {
                commands_without_permission(&mut report.found, &file, &text);
            }
        }
    }
    report.checked.push(format!("{classes} classes, {other} config files"));
    Ok(())
}

struct Part {
    name: String,
    exported: Option<String>,
    permission: bool,
    filter: bool,
    launcher: bool,
}

fn manifest(found: &mut Vec<Finding>, events: &[Event]) {
    let mut part: Option<Part> = None;
    for event in events {
        match event {
            Event::Start(tag, attrs) => {
                let get = |k: &str| attrs.iter().find(|(n, _)| n == k).map(|(_, v)| v.clone());
                match tag.as_str() {
                    "application" => application(found, &get),
                    t if COMPONENTS.contains(&t) => {
                        part = Some(Part {
                            name: get("name").unwrap_or_else(|| t.to_string()),
                            exported: get("exported"),
                            permission: get("permission").is_some(),
                            filter: false,
                            launcher: false,
                        });
                    }
                    "intent-filter" => {
                        if let Some(p) = part.as_mut() {
                            p.filter = true;
                        }
                    }
                    "action" if get("name").as_deref() == Some("android.intent.action.MAIN") => {
                        if let Some(p) = part.as_mut() {
                            p.launcher = true;
                        }
                    }
                    _ => {}
                }
            }
            Event::End(tag) if COMPONENTS.contains(&tag.as_str()) => {
                if let Some(p) = part.take() {
                    component(found, &p);
                }
            }
            Event::End(_) => {}
        }
    }
}

fn application(found: &mut Vec<Finding>, get: &dyn Fn(&str) -> Option<String>) {
    let on = |k: &str| get(k).as_deref() == Some("true");
    if on("debuggable") {
        found.push(Finding::new(
            Sev::High,
            "app",
            "AndroidManifest.xml",
            0,
            "the app is debuggable",
            "remove android:debuggable from release builds",
        ));
    }
    if on("usesCleartextTraffic") {
        found.push(Finding::new(
            Sev::Medium,
            "app",
            "AndroidManifest.xml",
            0,
            "cleartext HTTP traffic is allowed",
            "set android:usesCleartextTraffic=\"false\" and use HTTPS",
        ));
    }
    if get("allowBackup").as_deref() != Some("false") {
        found.push(Finding::new(
            Sev::Low,
            "app",
            "AndroidManifest.xml",
            0,
            "backups are allowed (adb backup can read app data)",
            "set android:allowBackup=\"false\" unless you need it",
        ));
    }
}

fn component(found: &mut Vec<Finding>, p: &Part) {
    let exported = p.exported.as_deref() == Some("true");
    if exported && !p.permission && !p.launcher {
        found.push(Finding::new(
            Sev::Medium,
            "app",
            "AndroidManifest.xml",
            0,
            format!("{} is exported without a permission", p.name),
            "set android:exported=\"false\" or protect it with android:permission",
        ));
    } else if p.exported.is_none() && p.filter && !p.launcher {
        found.push(Finding::new(
            Sev::Low,
            "app",
            "AndroidManifest.xml",
            0,
            format!("{} has an intent-filter but no android:exported", p.name),
            "set android:exported explicitly (required from Android 12)",
        ));
    }
}

pub fn apk(path: &Path, report: &mut Report) -> Result<(), String> {
    let mut zip = Zip::open(path)?;
    let apk = path.file_name().and_then(|n| n.to_str()).unwrap_or("app.apk").to_string();
    let mut parsed = false;
    let mut files = 0;
    for at in 0..zip.entries.len() {
        let name = zip.entries[at].name.clone();
        let ext = extension(&name);
        let scanned = name.ends_with(".dex")
            || name == "resources.arsc"
            || name.starts_with("assets/")
            || name.starts_with("res/raw/")
            || TEXT_EXT.contains(&ext.as_str()) && !name.starts_with("res/") && name != "AndroidManifest.xml";
        if name == "AndroidManifest.xml" {
            if let Some(data) = zip.read(at, ENTRY_LIMIT)
                && let Some(events) = axml::events(&data)
            {
                manifest(&mut report.found, &events);
                parsed = true;
            }
        } else if scanned && let Some(data) = zip.read(at, ENTRY_LIMIT) {
            files += 1;
            let file = format!("{apk}!{name}");
            scan_bytes(&mut report.found, &file, &data, MIN_RUN);
            if TEXT_EXT.contains(&ext.as_str()) {
                scan_text(&mut report.found, &file, &String::from_utf8_lossy(&data), true, false);
            }
        }
    }
    if !parsed {
        report.notes.push("AndroidManifest.xml could not be read, manifest checks skipped".into());
    }
    report.checked.push(format!("manifest, {files} code/asset files"));
    Ok(())
}

pub fn exe(path: &Path, report: &mut Report) -> Result<(), String> {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("app.exe").to_string();
    let file = File::open(path).map_err(|e| format!("cannot open {}: {e}", path.display()))?;
    let mut electron = false;
    strings(file, 16, |s| {
        electron |= s.contains("app.asar");
        for why in secrets_in_line(s, false) {
            secret(&mut report.found, &name, 0, why);
        }
    });
    report.checked.push("strings".into());
    let side = path.parent().map(|p| p.join("resources").join("app.asar")).filter(|p| p.is_file());
    match side {
        Some(a) => {
            asar::scan(&a, report)?;
        }
        None if electron => report.notes.push("Electron app but no resources/app.asar next to the exe".into()),
        None => {}
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strings_cross_chunk_borders_and_manifest_findings_have_severities() {
        let mut seen = Vec::new();
        strings(&b"\x00\x01short\x00a-long-enough-string\x02xx"[..], 8, |s| seen.push(s.to_string()));
        assert_eq!(seen, ["a-long-enough-string"]);
        let events = axml::plain(
            r#"<application android:debuggable="true"><activity android:name=".A" android:exported="true"/><service android:name=".S"><intent-filter/></service></application>"#,
        );
        let mut found = Vec::new();
        manifest(&mut found, &events);
        let why: Vec<&str> = found.iter().map(|f| f.why.as_str()).collect();
        assert_eq!(
            why,
            [
                "the app is debuggable",
                "backups are allowed (adb backup can read app data)",
                ".A is exported without a permission",
                ".S has an intent-filter but no android:exported"
            ]
        );
        let mut cmds = Vec::new();
        commands_without_permission(
            &mut cmds,
            "p.jar!plugin.yml",
            "name: X\ncommands:\n  heal:\n    permission: x.heal\n  fly:\n    usage: /fly\nother: 1\n",
        );
        assert_eq!(cmds.len(), 1);
        assert_eq!((cmds[0].why.as_str(), cmds[0].line), ("command /fly has no permission", 5));
    }
}
