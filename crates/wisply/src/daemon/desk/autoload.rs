use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use serde_json::json;
use wisply_core::{Call, Fail};

use super::super::Daemon;
use super::super::config;
use super::super::hosts::Host;
use super::super::web::{browser_exe, installed_browsers};
use super::uia::{self, El};
use super::{input, win};

const PAGES: &[(&str, &str)] =
    &[("brave", "brave://extensions/"), ("chrome", "chrome://extensions/"), ("edge", "edge://extensions/")];
const TITLES: &[&str] = &["erweiterungen", "extensions", "extensiones", "extensões"];
const DEV: (&str, &[&str]) = ("devMode", &["entwicklermodus", "developer mode"]);
const LOAD: (&str, &[&str]) = ("loadUnpacked", &["entpackte erweiterung laden", "load unpacked"]);
const OK_NAMES: &[&str] = &["ordner auswählen", "select folder", "auswählen", "select"];
const FOLDER_EDITS: &[&str] = &["1152", "1148"];
const FOLDER_WORDS: &[&str] = &["ordner", "folder"];
const DIALOG_CLASS: &str = "#32770";
const HOST_WAIT: Duration = Duration::from_secs(8);
const PAGE_WAIT: Duration = Duration::from_secs(15);
const FIND_WAIT: Duration = Duration::from_secs(12);
const DIALOG_WAIT: Duration = Duration::from_secs(6);
const CONNECT_WAIT: Duration = Duration::from_secs(20);
const STEP: Duration = Duration::from_millis(300);
const TOGGLE_SETTLE: Duration = Duration::from_millis(800);
const CLOSE_WAIT: Duration = Duration::from_secs(5);
const ROW_GAP: i32 = 60;
const ADDRESS_WORDS: &[&str] = &["address", "adresse", "url"];
const VK_RETURN: u16 = 0x0D;
const MENU_NAMES: &[&str] = &["menu", "menü", "menú"];
const MENU_WAIT: Duration = Duration::from_secs(4);
const SEEN_MAX: usize = 30;

fn wait<T>(limit: Duration, mut f: impl FnMut() -> Option<T>) -> Option<T> {
    let end = Instant::now() + limit;
    loop {
        if let Some(v) = f() {
            return Some(v);
        }
        if Instant::now() >= end {
            return None;
        }
        thread::sleep(STEP);
    }
}

fn page(name: &str) -> &'static str {
    PAGES.iter().find(|(n, _)| *n == name).map_or("chrome://extensions/", |(_, p)| *p)
}

fn host_for(d: &Daemon, name: &str) -> Option<Arc<Host>> {
    d.hosts.alive().into_iter().find(|h| h.brand.contains(name))
}

fn exe_file(exe: &Path) -> String {
    exe.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default()
}

fn browser_window(exe: &str, titles: bool) -> Option<isize> {
    win::windows()
        .into_iter()
        .filter(|w| w.exe.to_lowercase() == exe)
        .find(|w| !titles || TITLES.iter().any(|t| w.title.to_lowercase().contains(t)))
        .map(|w| w.hwnd)
}

fn centre((l, t, r, b): win::Rect) -> (i32, i32) {
    ((l + r) / 2, (t + b) / 2)
}

fn matches(el: &El, (aid, names): (&str, &[&str])) -> bool {
    let name = el.name().to_lowercase();
    el.aid() == aid || names.iter().any(|n| name.contains(n))
}

fn dev_switch(hwnd: isize) -> Option<El> {
    let items = uia::collect(hwnd, true, None);
    let (toggles, others): (Vec<&El>, Vec<&El>) = items.iter().partition(|e| e.toggleable());
    if let Some(t) = toggles.iter().find(|t| matches(t, DEV)) {
        return Some((*t).clone());
    }
    let labels: Vec<(i32, i32)> = others.iter().filter(|e| matches(e, DEV)).map(|e| centre(e.rect())).collect();
    toggles
        .iter()
        .flat_map(|t| labels.iter().map(move |l| (t, centre(t.rect()), *l)))
        .filter(|(_, (_, ty), (_, ly))| (ty - ly).abs() < ROW_GAP)
        .min_by_key(|(_, (tx, ty), (lx, ly))| (tx - lx).abs() + 3 * (ty - ly).abs())
        .map(|(t, _, _)| (*t).clone())
}

fn address_bar(hwnd: isize) -> Option<El> {
    let items = uia::collect(hwnd, false, None);
    let edits: Vec<&El> = items.iter().filter(|e| e.role() == "edit").collect();
    edits
        .iter()
        .find(|e| ADDRESS_WORDS.iter().any(|w| e.name().to_lowercase().contains(w)))
        .or_else(|| edits.iter().min_by_key(|e| e.rect().1))
        .map(|e| (*e).clone())
}

fn open_by_address_bar(exe_name: &str, url: &str) -> Option<isize> {
    let hwnd = browser_window(exe_name, false)?;
    let bar = wait(FIND_WAIT, || address_bar(hwnd))?;
    win::set_foreground(hwnd);
    bar.focus();
    if !bar.set_value(url) {
        return None;
    }
    thread::sleep(STEP);
    input::key(VK_RETURN, false, false);
    input::key(VK_RETURN, true, false);
    wait(PAGE_WAIT, || browser_window(exe_name, true))
}

fn open_side_menu(hwnd: isize) -> bool {
    let items = uia::collect(hwnd, false, None);
    let page = uia::page_rect(hwnd).map_or(0, |r| r.1);
    let Some(menu) = items
        .iter()
        .filter(|e| e.role() == "button" && e.rect().1 >= page)
        .find(|e| MENU_NAMES.iter().any(|n| e.name().to_lowercase().contains(n)))
    else {
        return false;
    };
    menu.act(false);
    thread::sleep(TOGGLE_SETTLE);
    true
}

fn seen(hwnd: isize) -> String {
    let page = uia::page_rect(hwnd).map_or(0, |r| r.1);
    let names: Vec<String> = uia::collect(hwnd, true, None)
        .iter()
        .filter(|e| e.rect().1 >= page)
        .map(|e| format!("{}:{}", e.role(), e.name()))
        .take(SEEN_MAX)
        .collect();
    format!("page top {page}: {}", names.join(" | "))
}

fn switched_on(el: &El) -> bool {
    el.describe().2.contains("[x]")
}

fn dialogs(pid: u32) -> HashSet<isize> {
    win::all()
        .into_iter()
        .filter(|h| win::visible(*h) && win::class_of(*h) == DIALOG_CLASS && win::pid_of(*h) == pid)
        .collect()
}

fn pick_folder(dlg: isize, folder: &Path) -> Result<(), Fail> {
    let items = uia::collect(dlg, false, None);
    let edits: Vec<&El> = items.iter().filter(|e| e.role() == "edit").collect();
    let edit = edits
        .iter()
        .find(|e| FOLDER_EDITS.contains(&e.aid().as_str()))
        .or_else(|| edits.iter().find(|e| FOLDER_WORDS.iter().any(|w| e.name().to_lowercase().contains(w))))
        .or_else(|| edits.last())
        .ok_or_else(|| Fail::new("could not type the folder path into the dialog"))?;
    if !edit.set_value(&folder.display().to_string()) {
        return Err(Fail::new("could not type the folder path into the dialog"));
    }
    thread::sleep(STEP);
    let buttons: Vec<&El> = items.iter().filter(|e| e.role() == "button").collect();
    let ok = buttons
        .iter()
        .find(|b| b.aid() == "1")
        .or_else(|| buttons.iter().find(|b| OK_NAMES.contains(&b.name().to_lowercase().as_str())))
        .ok_or_else(|| Fail::new("the dialog's select button was not found"))?;
    ok.act(false).map(|_| ()).ok_or_else(|| Fail::new("the dialog's select button did not react"))
}

fn load(d: &Daemon, name: &str, log: &mut Vec<String>) -> Result<Arc<Host>, Fail> {
    let exe = browser_exe(name).ok_or_else(|| Fail::new(format!("{name} is not installed")))?;
    let exe_name = exe_file(&exe);
    let started = browser_window(&exe_name, false).is_none();
    crate::spawn::detached(&exe, &[page(name)], exe.parent().unwrap_or(&d.home), &d.home.join("daemon.log"));
    if started && let Some(host) = wait(HOST_WAIT, || host_for(d, name)) {
        let _ = host.call("close-url", json!({"prefix": page(name)}), None, CLOSE_WAIT, None);
        log.push(format!("{name} started, the extension was already installed"));
        return Ok(host);
    }
    let hwnd = wait(PAGE_WAIT, || browser_window(&exe_name, true))
        .or_else(|| {
            browser_window(&exe_name, false)?;
            crate::spawn::detached(&exe, &[page(name)], exe.parent().unwrap_or(&d.home), &d.home.join("daemon.log"));
            log.push(format!("{name} opened a start page first, asked again for the extensions page"));
            wait(PAGE_WAIT, || browser_window(&exe_name, true))
        })
        .or_else(|| {
            log.push(format!("{name} ignored the page on its command line, typed it into the address bar"));
            open_by_address_bar(&exe_name, page(name))
        })
        .ok_or_else(|| Fail::new(format!("the extensions page did not open in {name}")))?;
    let dev = wait(MENU_WAIT, || dev_switch(hwnd))
        .or_else(|| {
            open_side_menu(hwnd);
            wait(FIND_WAIT, || dev_switch(hwnd))
        })
        .ok_or_else(|| {
            Fail::new(format!("developer mode switch not found on the extensions page (saw: {})", seen(hwnd)))
        })?;
    if !switched_on(&dev) {
        dev.act(false);
        log.push("developer mode switched on".into());
        thread::sleep(TOGGLE_SETTLE);
    }
    let button = wait(FIND_WAIT, || uia::collect(hwnd, false, None).into_iter().find(|e| matches(e, LOAD)))
        .ok_or_else(|| Fail::new("'Load unpacked' button not found"))?;
    let pid = win::pid_of(hwnd);
    let before = dialogs(pid);
    button.act(false);
    let dlg = wait(DIALOG_WAIT, || dialogs(pid).into_iter().find(|h| !before.contains(h)))
        .ok_or_else(|| Fail::new("the folder dialog did not open"))?;
    let folder = d.home.join("extension");
    log.push(format!("folder dialog open, selecting {}", folder.display()));
    pick_folder(dlg, &folder)?;
    let host = wait(CONNECT_WAIT, || host_for(d, name)).ok_or_else(|| {
        Fail::hint("extension loaded, but it did not connect", format!("check {} for errors", page(name)))
    })?;
    let _ = host.call("close-url", json!({"prefix": page(name)}), None, CLOSE_WAIT, None);
    log.push("extension loaded and connected".into());
    Ok(host)
}

pub fn autoload(d: &Daemon, c: &Call) -> Result<String, Fail> {
    let pref = config::text(&config::load(&d.home), "browser").to_lowercase();
    let wanted = c.pos.first().cloned().or_else(|| installed_browsers().contains(&pref.as_str()).then_some(pref));
    let names: Vec<String> = match wanted {
        Some(w) => vec![w],
        None => installed_browsers().into_iter().take(1).map(String::from).collect(),
    };
    let mut lines = Vec::new();
    for name in &names {
        if host_for(d, name).is_some() {
            lines.push(format!("{name} already connected"));
            break;
        }
        match load(d, name, &mut lines) {
            Ok(_) => {
                d.hosts.push_config(&d.home);
                break;
            }
            Err(e) => lines.push(format!("{name}: {}", e.text().trim_start_matches("err: "))),
        }
    }
    Ok(if lines.is_empty() { "no supported browser found".into() } else { lines.join("\n") })
}
