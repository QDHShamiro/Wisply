use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

use serde_json::{Map, Value, json};
use wisply_core::local_now;

use super::script::hash;
use super::store::{file, save, text};

const VERSION: u64 = 4;

struct Shipped {
    name: &'static str,
    about: &'static str,
    guide: &'static str,
    tags: &'static [&'static str],
    params: &'static [&'static str],
    code: &'static str,
    check: &'static str,
    before_ms: u64,
    before_tokens: u64,
}

const SHIPPED: [Shipped; 6] = [
    Shipped {
        name: "send-message",
        about: "send a chat message (WhatsApp Web, Telegram Web, Discord Web, or a chat app on the desktop)",
        guide: "to = the exact chat name (Discord: user name), text = the message, app = whatsapp (default) | telegram | discord | any chat app, dm = a known Discord DM link, list = JSON [{name,to,app,dm}] to send the same text to several chats",
        tags: &[
            "send",
            "message",
            "chat",
            "whatsapp",
            "telegram",
            "discord",
            "nachricht",
            "schicken",
            "schick",
            "senden",
            "schreiben",
            "schreib",
        ],
        params: &["to", "text", "app", "dm", "list"],
        code: include_str!("defaults/send-message.js"),
        check: include_str!("defaults/send-message.check.js"),
        before_ms: 120_000,
        before_tokens: 6000,
    },
    Shipped {
        name: "read-form",
        about: "open a page, fill a form and read the page text",
        guide: "url = the page, fill = \"Label=value; Label2=value\", submit = the button text, confirm=1 clicks a risky button, find = words to read only that part",
        tags: &["page", "form", "fill", "read", "website", "formular", "seite", "ausfuellen", "lesen", "submit"],
        params: &["url", "fill", "submit", "confirm", "find"],
        code: include_str!("defaults/read-form.js"),
        check: include_str!("defaults/read-form.check.js"),
        before_ms: 60_000,
        before_tokens: 4000,
    },
    Shipped {
        name: "find-file",
        about: "find a file or folder by a part of its name",
        guide: "name = a part of the name, in = the folder to search (default: here)",
        tags: &["find", "file", "folder", "datei", "ordner", "finden", "suche", "search", "where", "liegt"],
        params: &["name", "in"],
        code: include_str!("defaults/find-file.js"),
        check: include_str!("defaults/find-file.check.js"),
        before_ms: 30_000,
        before_tokens: 2000,
    },
    Shipped {
        name: "pr-ci",
        about: "wait for the CI of a GitHub pull request and give the verdict",
        guide: "pr = the number (default: this branch), repo = owner/name (default: this repo), timeout = 8m",
        tags: &["pull", "request", "checks", "build", "green", "red", "gruen", "durch", "merge"],
        params: &["pr", "repo", "timeout"],
        code: include_str!("defaults/pr-ci.js"),
        check: include_str!("defaults/pr-ci.check.js"),
        before_ms: 300_000,
        before_tokens: 3000,
    },
    Shipped {
        name: "list-chats",
        about: "read the Discord DM list and the WhatsApp chat list (read only) so contacts can be found",
        guide: "app = all (default) | discord | whatsapp, max = most chats per app; one line per chat: app, name, dm link or chat|group",
        tags: &["contacts", "kontakte", "scan", "dms", "chatliste", "chats"],
        params: &["app", "max"],
        code: include_str!("defaults/list-chats.js"),
        check: include_str!("defaults/list-chats.check.js"),
        before_ms: 90_000,
        before_tokens: 5000,
    },
    Shipped {
        name: "call-briefing",
        about: "ring or message the morning briefing: open to-dos, sessions and new Albion messages",
        guide: "channels = ntfy,callmebot (default: the briefing channels from wisply call set), runs by itself with wisply call schedule on",
        tags: &["call", "briefing", "morning", "morgen", "anruf", "anrufen", "klingeln", "ring", "bericht"],
        params: &["channels"],
        code: include_str!("defaults/call-briefing.js"),
        check: include_str!("defaults/call-briefing.check.js"),
        before_ms: 30_000,
        before_tokens: 1500,
    },
];

static DONE: LazyLock<Mutex<HashSet<PathBuf>>> = LazyLock::new(|| Mutex::new(HashSet::new()));

fn marker(home: &Path) -> PathBuf {
    home.join("skills-defaults.json")
}

fn read_marker(home: &Path) -> Value {
    fs::read_to_string(marker(home)).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or_else(|| json!({}))
}

fn dropped_list(m: &Value) -> Vec<String> {
    m["dropped"].as_array().into_iter().flatten().map(text).collect()
}

fn write_marker(home: &Path, version: u64, code: &str, dropped: &[String]) {
    let _ = wisply_core::save(marker(home), json!({"version": version, "code": code, "dropped": dropped}).to_string());
}

fn shipped_code() -> String {
    hash(&SHIPPED.iter().map(|s| s.code).collect::<Vec<_>>().join("\n"))
}

fn stamp() -> String {
    local_now().chars().take(16).collect()
}

pub fn is_default(name: &str) -> bool {
    SHIPPED.iter().any(|s| s.name == name)
}

pub fn forget(home: &Path, name: &str) {
    let m = read_marker(home);
    let mut dropped = dropped_list(&m);
    if !dropped.iter().any(|d| d == name) {
        dropped.push(name.to_string());
    }
    write_marker(home, m["version"].as_u64().unwrap_or(0), &text(&m["code"]), &dropped);
}

pub fn bring_back(home: &Path, name: &str) {
    let m = read_marker(home);
    let dropped: Vec<String> = dropped_list(&m).into_iter().filter(|d| d != name).collect();
    write_marker(home, m["version"].as_u64().unwrap_or(0), &text(&m["code"]), &dropped);
}

fn learn_of(s: &Shipped) -> Value {
    json!({
        "state": "learned", "about": s.about, "guide": s.guide, "check": s.check, "before_ms": s.before_ms,
        "before_tokens": s.before_tokens, "practice": {"ok": 0, "fail": 0}, "hint": null, "at": stamp(),
        "default": true, "shipped": hash(s.code)
    })
}

fn fresh(s: &Shipped) -> Value {
    let params: Map<String, Value> = s.params.iter().map(|p| ((*p).to_string(), json!(""))).collect();
    json!({
        "name": s.name, "version": 1, "source": "skill", "created": stamp(), "params": params, "steps": [],
        "check": null, "cost": s.before_tokens, "tags": s.tags,
        "script": {"code": s.code, "hash": hash(s.code), "calls": 0, "project": ""},
        "stats": {"runs": 0, "ok": 0, "fail": 0, "ms": 0, "saved": 0, "seen": 1, "recent": []},
        "flaky": false, "learn": learn_of(s)
    })
}

fn untouched(r: &Value) -> bool {
    r["learn"]["default"] == true && r["script"]["hash"] == r["learn"]["shipped"]
}

fn refresh(mut r: Value, s: &Shipped) -> Value {
    let state = r["learn"]["state"].clone();
    r["version"] = json!(r["version"].as_u64().unwrap_or(1) + 1);
    r["script"]["code"] = json!(s.code);
    r["script"]["hash"] = json!(hash(s.code));
    r["tags"] = json!(s.tags);
    for p in s.params {
        if r["params"].get(*p).is_none() {
            r["params"][*p] = json!("");
        }
    }
    r["learn"] = learn_of(s);
    r["learn"]["state"] = state;
    r
}

pub fn ensure(home: &Path) {
    let mut done = DONE.lock().unwrap_or_else(|e| e.into_inner());
    if done.contains(home) {
        return;
    }
    done.insert(home.to_path_buf());
    let m = read_marker(home);
    let code = shipped_code();
    if m["version"].as_u64() == Some(VERSION) && text(&m["code"]) == code {
        return;
    }
    let dropped = dropped_list(&m);
    for s in SHIPPED.iter().filter(|s| !dropped.iter().any(|d| d == s.name)) {
        let old = fs::read_to_string(file(home, s.name)).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok());
        let r = match old {
            None => fresh(s),
            Some(o) if untouched(&o) && text(&o["script"]["hash"]) != hash(s.code) => refresh(o, s),
            Some(_) => continue,
        };
        let _ = save(home, &r);
    }
    write_marker(home, VERSION, &code, &dropped);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_skills_install_once_stay_dropped_and_keep_local_changes() {
        let home = std::env::temp_dir().join(format!("wisply-skill-defaults-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        ensure(&home);
        let all = super::super::store::load_all(&home);
        assert_eq!(all.len(), SHIPPED.len());
        assert!(all.iter().all(|r| r["learn"]["state"] == "learned" && r["source"] == "skill"));
        forget(&home, "find-file");
        let _ = fs::remove_file(file(&home, "find-file"));
        let mut mine = serde_json::from_str::<Value>(&fs::read_to_string(file(&home, "pr-ci")).unwrap()).unwrap();
        mine["script"]["hash"] = json!("changed-here");
        save(&home, &mine).unwrap();
        write_marker(&home, 0, "", &["find-file".to_string()]);
        DONE.lock().unwrap().clear();
        ensure(&home);
        assert!(!file(&home, "find-file").exists());
        let kept = serde_json::from_str::<Value>(&fs::read_to_string(file(&home, "pr-ci")).unwrap()).unwrap();
        assert_eq!(kept["script"]["hash"], "changed-here");
        bring_back(&home, "find-file");
        assert!(dropped_list(&read_marker(&home)).is_empty());
        let _ = fs::remove_dir_all(&home);
    }
}
