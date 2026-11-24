use std::path::Path;

use serde_json::{Map, Value};
use wisply_core::Fail;

use super::cfg;
use super::net;
use super::ring;
use super::text::{clip, random_hex};

const ROUTINE: &str = "call-briefing";
const TOPIC_PREFIX: &str = "wisply-call-";
const TOPIC_LEN: usize = 20;
const SHOWN: usize = 160;

fn routine(home: &Path, argv: &[&str]) -> Result<String, Fail> {
    let (code, out) =
        crate::client::ask(home, argv).map_err(|e| Fail::new(format!("the Wisply daemon is not reachable ({e})")))?;
    if code == 0 { Ok(out.trim().to_string()) } else { Err(Fail::new(clip(&out, SHOWN))) }
}

pub fn schedule(home: &Path, rest: &[String]) -> Result<String, Fail> {
    let settings = cfg::load(home);
    match rest.first().map(String::as_str) {
        Some("on") => {
            let at = rest.get(1).cloned().unwrap_or_else(|| cfg::text(&settings, "briefing_at"));
            if cfg::clock(&at).is_none() {
                return Err(Fail::hint(format!("'{at}' is not a time"), "wisply call schedule on 07:30"));
            }
            cfg::set(home, &[format!("briefing_at={at}")])?;
            let _ = routine(home, &["routines"]);
            let _ = routine(home, &["routine", "unschedule", ROUTINE]);
            let done = routine(home, &["routine", "schedule", ROUTINE, "daily", &at])?;
            Ok(format!("call: {done}"))
        }
        Some("off") => Ok(format!("call: {}", routine(home, &["routine", "unschedule", ROUTINE])?)),
        Some("status") | None => {
            let listing = routine(home, &["routine", "schedules"])?;
            let mine: Vec<&str> = listing.lines().filter(|l| l.contains(ROUTINE)).collect();
            if mine.is_empty() {
                Ok("call: no morning briefing scheduled (wisply call schedule on 07:30)".to_string())
            } else {
                Ok(format!("call: {}", mine.join("\n").trim()))
            }
        }
        Some(other) => Err(Fail::hint(
            format!("unknown schedule option '{other}'"),
            "wisply call schedule on [HH:MM] | off | status",
        )),
    }
}

#[cfg(windows)]
fn clipboard(text: &str) -> bool {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let Ok(mut child) = Command::new("clip").stdin(Stdio::piped()).spawn() else { return false };
    let wrote = child.stdin.take().is_some_and(|mut s| s.write_all(text.as_bytes()).is_ok());
    wrote && child.wait().is_ok_and(|s| s.success())
}

#[cfg(not(windows))]
fn clipboard(_text: &str) -> bool {
    false
}

fn marked(settings: &Map<String, Value>, key: &str) -> &'static str {
    if cfg::text(settings, key).trim().is_empty() { "missing" } else { "set" }
}

pub fn run(home: &Path, rest: &[String]) -> Result<String, Fail> {
    let mut settings = cfg::load(home);
    let mut lines: Vec<String> = Vec::new();
    if cfg::text(&settings, "ntfy_topic").trim().is_empty() {
        let topic = format!("{TOPIC_PREFIX}{}", random_hex(TOPIC_LEN));
        cfg::set(home, &[format!("ntfy_topic={topic}")])?;
        let copied = clipboard(&topic);
        lines.push(if copied {
            "call: made a private ntfy topic, it is in your clipboard (not printed)".to_string()
        } else {
            "call: made a private ntfy topic (not printed, read it in ~/.wisply/call/config.json)".to_string()
        });
        settings = cfg::load(home);
    } else if rest.first().map(String::as_str) == Some("copy") {
        let topic = cfg::text(&settings, "ntfy_topic");
        lines.push(if clipboard(topic.trim()) {
            "call: topic copied to the clipboard".to_string()
        } else {
            "call: no clipboard here".to_string()
        });
    }
    lines.push(guide(&settings));
    Ok(lines.join("\n"))
}

fn guide(settings: &Map<String, Value>) -> String {
    let server = net::server(settings, &Map::new());
    [
        format!("1. Install the ntfy app (Android/iOS) and subscribe to the topic on {server}."),
        "2. Android: ntfy settings, allow Do not disturb override for the topic, so priority 5 rings through a locked phone.".to_string(),
        "3. Test: wisply call test".to_string(),
        "4. Extra ring (optional): wisply call set callmebot_user=@deinname channels=ntfy,callmebot".to_string(),
        "   Authorize CallMeBot once: send /start to @CallMeBot_txtbot in Telegram.".to_string(),
        format!(
            "channels: {}; ntfy topic {}, CallMeBot user {}, Discord user {}",
            cfg::text(settings, "channels"),
            marked(settings, "ntfy_topic"),
            marked(settings, "callmebot_user"),
            marked(settings, "discord_user_id")
        ),
        format!("known channels: {}", ring::CHANNELS.join(", ")),
    ]
    .join("\n")
}

pub fn status(home: &Path) -> String {
    let settings = cfg::load(home);
    let open = super::store::open_list(home).len();
    let on = if cfg::on(&settings, "enabled") { "on" } else { "off" };
    format!(
        "call: {on}, channels {}, {open} waiting\n{}\n{}",
        cfg::text(&settings, "channels"),
        super::away::status(home, &settings),
        guide(&settings)
    )
}
