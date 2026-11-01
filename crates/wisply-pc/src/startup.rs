use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::{Value, json};

use crate::config::dir;
use crate::model::{Sample, short_name, size};
use crate::plat::shell;

const WAIT: Duration = Duration::from_secs(20);
const OFF_SUFFIX: &str = ".off";
const RUN_KEYS: [&str; 3] = [
    "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
    "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run",
    "HKLM\\Software\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Run",
];
const APPROVED: &str = "HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\StartupApproved\\Run";
const ON_BYTES: &str = "020000000000000000000000";
const OFF_BYTES: &str = "030000000000000000000000";
const BACKUP: &str = "startup-backup.json";
const POPUP_HOSTS: [&str; 6] = ["cmd", "powershell", "pwsh", "wscript", "cscript", "conhost"];
const POPUP_PARENTS: [&str; 12] = [
    "windowsterminal",
    "wt",
    "explorer",
    "code",
    "cmd",
    "powershell",
    "pwsh",
    "openconsole",
    "bash",
    "mintty",
    "claude",
    "wisply",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Run,
    Task,
    File,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    pub kind: Kind,
    pub name: String,
    pub command: String,
    pub source: String,
    pub enabled: bool,
}

impl Entry {
    pub fn cost(&self, s: &Sample) -> u64 {
        let exe = exe_name(&self.command);
        s.procs.iter().filter(|p| !exe.is_empty() && short_name(&p.name) == exe).map(|p| p.rss).sum()
    }
}

pub fn exe_name(command: &str) -> String {
    let c = command.trim();
    let first = if let Some(rest) = c.strip_prefix('"') {
        rest.split('"').next().unwrap_or("")
    } else {
        c.split(' ').next().unwrap_or("")
    };
    short_name(first.replace('\\', "/").rsplit('/').next().unwrap_or(""))
}

pub fn parse_reg(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(|l| {
            let l = l.trim_start();
            let (name, rest) = l.split_once("    REG_")?;
            let (_, value) = rest.split_once("    ").unwrap_or((rest, ""));
            let name = name.trim();
            (!(name.starts_with('(') && name.ends_with(')'))).then(|| (name.to_string(), value.trim().to_string()))
        })
        .collect()
}

pub fn approved_off(bytes: &str) -> bool {
    let first = bytes.trim().get(0..2).and_then(|h| u8::from_str_radix(h, 16).ok()).unwrap_or(2);
    first % 2 == 1
}

pub fn parse_tasks(text: &str) -> Vec<(String, bool)> {
    text.lines()
        .filter_map(|l| {
            let cols: Vec<&str> = l.trim().trim_matches('"').split("\",\"").collect();
            let name = cols.first()?.to_string();
            let ready =
                cols.get(2).is_none_or(|s| !s.to_lowercase().contains("disabled") && !s.contains("Deaktiviert"));
            (name.starts_with('\\') && !name.to_lowercase().starts_with("\\microsoft\\")).then_some((name, ready))
        })
        .collect()
}

fn windows_entries() -> Vec<Entry> {
    let approved = shell::run("reg", &["query", APPROVED], WAIT).map(|t| parse_reg(&t)).unwrap_or_default();
    let mut out = Vec::new();
    for key in RUN_KEYS {
        let Some(text) = shell::run("reg", &["query", key], WAIT) else { continue };
        for (name, command) in parse_reg(&text) {
            let off = approved.iter().find(|(n, _)| *n == name).is_some_and(|(_, b)| approved_off(b));
            out.push(Entry { kind: Kind::Run, name, command, source: key.into(), enabled: !off });
        }
    }
    if let Some(text) = shell::run("schtasks", &["/query", "/fo", "csv", "/nh"], WAIT) {
        for (name, enabled) in parse_tasks(&text) {
            out.push(Entry { kind: Kind::Task, command: name.clone(), name, source: "scheduled task".into(), enabled });
        }
    }
    out
}

fn files_in(folder: &Path, ext: &str) -> Vec<Entry> {
    let Ok(read) = fs::read_dir(folder) else { return Vec::new() };
    let mut out: Vec<Entry> = read
        .flatten()
        .filter_map(|e| {
            let file = e.file_name().to_string_lossy().into_owned();
            let enabled = file.ends_with(ext);
            let base = file.strip_suffix(OFF_SUFFIX).unwrap_or(&file);
            base.ends_with(ext).then(|| Entry {
                kind: Kind::File,
                name: base.to_string(),
                command: e.path().to_string_lossy().into_owned(),
                source: folder.to_string_lossy().into_owned(),
                enabled,
            })
        })
        .collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

pub fn file_toggle(entry: &Entry, on: bool) -> Result<(), String> {
    let path = PathBuf::from(&entry.command);
    let target =
        if on { path.with_file_name(&entry.name) } else { path.with_file_name(format!("{}{OFF_SUFFIX}", entry.name)) };
    if path == target {
        return Ok(());
    }
    fs::rename(&path, &target).map_err(|e| format!("cannot rename {}: {e}", path.display()))
}

pub fn list(user: &Path) -> Vec<Entry> {
    let mut out = Vec::new();
    if cfg!(windows) {
        out.extend(windows_entries());
        let roaming =
            std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or_else(|| user.join("AppData").join("Roaming"));
        let startup = roaming.join("Microsoft").join("Windows").join("Start Menu").join("Programs").join("Startup");
        out.extend(files_in(&startup, ".lnk"));
    } else if cfg!(target_os = "macos") {
        out.extend(files_in(&user.join("Library").join("LaunchAgents"), ".plist"));
    } else {
        out.extend(files_in(&user.join(".config").join("autostart"), ".desktop"));
    }
    out
}

pub fn count(user: &Path) -> usize {
    list(user).iter().filter(|e| e.enabled).count()
}

fn save_backup(home: &Path, entry: &Entry, before: Option<String>) {
    let file = dir(home).join(BACKUP);
    let mut rows: Vec<Value> =
        fs::read_to_string(&file).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_default();
    rows.retain(|r| r.get("name").and_then(Value::as_str) != Some(&entry.name));
    rows.push(json!({"name": entry.name, "source": entry.source, "command": entry.command, "approved": before}));
    let _ = fs::create_dir_all(dir(home));
    let _ = fs::write(file, Value::Array(rows).to_string());
}

fn approved_value(name: &str) -> Option<String> {
    let text = shell::run("reg", &["query", APPROVED, "/v", name], WAIT)?;
    parse_reg(&text).into_iter().find(|(n, _)| n == name).map(|(_, v)| v)
}

pub fn toggle(home: &Path, entry: &Entry, on: bool) -> Result<String, String> {
    let word = if on { "on" } else { "off" };
    match entry.kind {
        Kind::File => {
            if !on {
                save_backup(home, entry, None);
            }
            file_toggle(entry, on)?;
        }
        Kind::Run => {
            if !on {
                save_backup(home, entry, approved_value(&entry.name));
            }
            let data = if on { ON_BYTES } else { OFF_BYTES };
            let args = ["add", APPROVED, "/v", &entry.name, "/t", "REG_BINARY", "/d", data, "/f"];
            if !shell::succeeds("reg", &args, WAIT) {
                return Err(format!("could not switch {} {word}", entry.name));
            }
        }
        Kind::Task => {
            if !on {
                save_backup(home, entry, None);
            }
            let flag = if on { "/enable" } else { "/disable" };
            if !shell::succeeds("schtasks", &["/change", "/tn", &entry.name, flag], WAIT) {
                return Err(format!(
                    "could not switch {} {word} (this task may need administrator rights)",
                    entry.name
                ));
            }
        }
    }
    Ok(format!(
        "{} is now {word} at startup; undo with: wisply pc startup {} <n>",
        entry.name,
        if on { "off" } else { "on" }
    ))
}

pub fn popups(s: &Sample) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for p in &s.procs {
        let host = short_name(&p.name);
        let parent = s.proc(p.ppid).map(|x| short_name(&x.name)).unwrap_or_default();
        if POPUP_HOSTS.contains(&host.as_str()) && !POPUP_PARENTS.contains(&parent.as_str()) && !parent.is_empty() {
            let line = format!("{host} opened by {parent} (pid {})", p.pid);
            if !out.contains(&line) {
                out.push(line);
            }
        }
    }
    out
}

pub fn render(entries: &[Entry], s: &Sample) -> String {
    if entries.is_empty() {
        return "pc startup | no autostart entries found".into();
    }
    let on = entries.iter().filter(|e| e.enabled).count();
    let mut lines = vec![format!("pc startup | {on} on of {}", entries.len())];
    for (i, e) in entries.iter().enumerate() {
        let weight = e.cost(s);
        let tail = if weight > 0 { format!(" | running, {}", size(weight)) } else { String::new() };
        lines.push(format!("{}. {} | {} | {}{tail}", i + 1, e.name, if e.enabled { "on" } else { "off" }, e.source));
    }
    let pops = popups(s);
    if !pops.is_empty() {
        lines.push(format!("console windows that may pop up: {}", pops.join("; ")));
    }
    lines.push("switch off: wisply pc startup off <n> | back on: wisply pc startup on <n>".into());
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake::proc;

    #[test]
    fn registry_output_and_approval_bytes_are_read() {
        let out = "\nHKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Run\n    Discord    REG_SZ    \"C:\\Users\\a\\Update.exe\" --processStart Discord.exe\n    OneDrive    REG_SZ    C:\\x\\OneDrive.exe /background\n\n";
        let rows = parse_reg(out);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].0, "Discord");
        assert!(rows[0].1.starts_with("\"C:\\Users\\a\\Update.exe\""));
        assert_eq!(exe_name(&rows[0].1), "update");
        assert_eq!(exe_name(&rows[1].1), "onedrive");
        assert!(
            approved_off("030000000000000000000000")
                && !approved_off("020000000000000000000000")
                && !approved_off("0600")
        );
    }

    #[test]
    fn only_third_party_scheduled_tasks_are_listed() {
        let out = "\"\\GoogleUpdateTaskMachineCore\",\"N/A\",\"Ready\"\n\"\\Microsoft\\Windows\\X\",\"N/A\",\"Ready\"\n\"\\Adobe Sync\",\"N/A\",\"Disabled\"\n";
        assert_eq!(
            parse_tasks(out),
            vec![("\\GoogleUpdateTaskMachineCore".into(), true), ("\\Adobe Sync".into(), false)]
        );
    }

    #[test]
    fn a_file_entry_goes_off_and_on_again_keeping_the_original() {
        let d = std::env::temp_dir().join(format!("wpc-start-{}", std::process::id()));
        let _ = fs::remove_dir_all(&d);
        let auto = d.join(".config").join("autostart");
        fs::create_dir_all(&auto).unwrap();
        fs::write(auto.join("a.desktop"), "[Desktop Entry]").unwrap();
        fs::write(auto.join("b.desktop"), "[Desktop Entry]").unwrap();
        fs::write(auto.join("readme.txt"), "x").unwrap();
        let entries = files_in(&auto, ".desktop");
        assert_eq!(
            entries.iter().map(|e| (e.name.as_str(), e.enabled)).collect::<Vec<_>>(),
            [("a.desktop", true), ("b.desktop", true)]
        );
        let home = d.join("home");
        let text = toggle(&home, &entries[0], false).unwrap();
        assert!(text.contains("a.desktop is now off"), "{text}");
        assert!(auto.join("a.desktop.off").is_file() && !auto.join("a.desktop").exists());
        assert!(home.join("pc").join(BACKUP).is_file());
        let again = files_in(&auto, ".desktop");
        assert_eq!(again.iter().map(|e| e.enabled).collect::<Vec<_>>(), [false, true]);
        toggle(&home, &again[0], true).unwrap();
        assert!(auto.join("a.desktop").is_file());
        assert_eq!(fs::read_to_string(auto.join("a.desktop")).unwrap(), "[Desktop Entry]");
        fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn the_list_shows_the_weight_of_running_entries_and_console_popups() {
        let sample = Sample {
            procs: vec![
                proc(1, 0, "OneDrive.exe", 400 << 20),
                proc(2, 9, "cmd.exe", 1),
                proc(9, 0, "weird.exe", 1),
                proc(3, 0, "cmd.exe", 1),
            ],
            ..Sample::default()
        };
        let e = Entry {
            kind: Kind::Run,
            name: "OneDrive".into(),
            command: "C:\\x\\OneDrive.exe /background".into(),
            source: "HKCU".into(),
            enabled: true,
        };
        let text = render(&[e], &sample);
        assert!(text.contains("1. OneDrive | on | HKCU | running, 400 MB"), "{text}");
        assert!(text.contains("cmd opened by weird (pid 2)") && !text.contains("pid 3"), "{text}");
        assert_eq!(render(&[], &sample), "pc startup | no autostart entries found");
    }
}
