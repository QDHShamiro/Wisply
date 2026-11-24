use std::collections::BTreeSet;
use std::path::Path;
use std::sync::LazyLock;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, Instant};

use regex::Regex;
use serde_json::json;
use wisply_core::Fail;

use super::super::ctx::Ctx;
use super::super::{Daemon, Out, agents, routines, system, web};
use super::cache;
use super::contacts::{self, Found, Person};
use super::parse::Act;

const RETRY_FOR: Duration = Duration::from_secs(5);
const RETRY_EVERY: Duration = Duration::from_millis(300);
const PLAY_WAIT: Duration = Duration::from_secs(6);
const WEB_SEARCH: &str = "google";
const SEND: &str = "send-message";
const NOT_WEB: &[&str] = &["exe", "lnk", "bat", "cmd", "msi", "txt", "png", "jpg", "pdf", "json", "yml", "jar"];
const YT_FIRST: &str = r#"return (async () => {
  for (let t = 0; t < 8000; t += 200) {
    const a = document.querySelector("ytd-video-renderer a#video-title, ytd-rich-item-renderer a#video-title-link");
    if (a) { a.click(); return a.title || a.textContent.trim(); }
    await new Promise((r) => setTimeout(r, 200));
  }
  return "";
})();"#;

static LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?i)(https?://|www\.)\S+").expect("link"));
static URLISH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^[a-z0-9-]+(\.[a-z0-9-]+)*\.([a-z]{2,10})(/\S*)?$").expect("url"));

struct To {
    person: Person,
    app: String,
}

fn argv(parts: &[&str]) -> Vec<String> {
    parts.iter().map(|p| (*p).to_string()).collect()
}

fn stopped(d: &Daemon) -> bool {
    d.abort.load(Ordering::SeqCst)
}

pub fn reply(d: &Daemon, ctx: &mut Ctx, argv: &[String]) -> Result<Out, Fail> {
    match system::run_one(d, ctx, argv)? {
        (Out::Wait(p), _) => p.settle(d, ctx).map(Out::Text),
        (out, _) => Ok(out),
    }
}

pub fn once(d: &Daemon, ctx: &mut Ctx, argv: &[String]) -> Result<String, Fail> {
    match reply(d, ctx, argv)? {
        Out::Stop(t) => Err(Fail::new(t)),
        Out::Text(t) => Ok(t),
        Out::Wait(_) => Ok(String::new()),
    }
}

fn step(d: &Daemon, ctx: &mut Ctx, argv: &[String]) -> Result<String, Fail> {
    let end = Instant::now() + RETRY_FOR;
    loop {
        if stopped(d) {
            return Err(Fail::new("stopped"));
        }
        match once(d, ctx, argv) {
            Err(f) if routines::transient(&f.msg) && Instant::now() < end => thread::sleep(RETRY_EVERY),
            other => return other,
        }
    }
}

fn pick(people: &[Person], who: &str, yes: bool) -> Result<Person, String> {
    match contacts::find(people, who) {
        Found::One(p) => Ok(p),
        Found::Maybe(list) if !yes => {
            Err(format!("{who}? {}", list.iter().map(|p| p.name.as_str()).collect::<Vec<_>>().join(" / ")))
        }
        Found::Nobody if !yes => Err(format!("{who} unknown")),
        _ => Ok(Person { name: who.to_string(), ..Person::default() }),
    }
}

fn resolve(home: &Path, who: &[String], asked: Option<&str>, yes: bool) -> Result<Vec<To>, Fail> {
    let people = contacts::loaded(home);
    let mut out = Vec::new();
    let mut unsure = Vec::new();
    for w in who {
        match pick(&people, w, yes) {
            Ok(person) => match person.app_for(asked) {
                Some(app) => out.push(To { person, app }),
                None => unsure.push(format!("{}: which app? add \"auf discord\" or \"auf whatsapp\"", person.name)),
            },
            Err(why) => unsure.push(why),
        }
    }
    if unsure.is_empty() {
        return Ok(out);
    }
    Err(Fail::hint(
        format!("not sent, {}", unsure.join("; ")),
        "wisply contact add <name> discord=<user> whatsapp=<chat>, or --yes for the exact chat name",
    ))
}

fn guard(text: &str, yes: bool) -> Result<(), Fail> {
    if LINK.is_match(text) && !yes {
        return Err(Fail::hint("not sent, text has a link", "add --yes if meant"));
    }
    Ok(())
}

fn lists(home: &Path) -> bool {
    routines::ensure_shipped(home);
    cache::routines(home)
        .iter()
        .find(|r| r["name"] == SEND)
        .is_none_or(|r| r["script"]["code"].as_str().is_some_and(|c| c.contains("args.list")))
}

fn message(home: &Path, who: &[String], app: Option<&str>, text: &str, yes: bool) -> Result<String, Fail> {
    guard(text, yes)?;
    let to = resolve(home, who, app, yes)?;
    let (Some(first), Some(last)) = (to.first(), to.last()) else {
        return Err(Fail::hint("not sent, nobody to send to", super::USAGE));
    };
    let rows: Vec<serde_json::Value> = to
        .iter()
        .map(|t| {
            let dm = if t.app == "discord" { t.person.dm.as_str() } else { "" };
            json!({"name": t.person.name, "to": t.person.handle(&t.app), "app": t.app, "dm": dm})
        })
        .collect();
    let mut pairs = vec![("text".to_string(), text.to_string()), ("app".to_string(), last.app.clone())];
    if lists(home) {
        pairs.push(("list".into(), serde_json::Value::Array(rows).to_string()));
    } else if to.len() == 1 {
        pairs.push(("to".into(), first.person.handle(&first.app)));
        if first.app == "discord" && !first.person.dm.is_empty() {
            pairs.push(("dm".into(), first.person.dm.clone()));
        }
    } else {
        return Err(Fail::hint(
            "not sent, send-message routine was changed and takes one chat",
            "send to one person at a time",
        ));
    }
    Ok(wisply_proto::handoff_line(SEND, &pairs, true))
}

fn spotify(d: &Daemon, ctx: &mut Ctx, q: &str) -> Result<String, Fail> {
    step(d, ctx, &argv(&["launch", &format!("spotify:search:{q}")]))?;
    hold_app(d, ctx, "spotify");
    let labels = [format!("{q} wiedergeben"), format!("Play {q}")];
    let end = Instant::now() + PLAY_WAIT;
    loop {
        for label in &labels {
            if once(d, ctx, &argv(&["click", label, "--on=app"])).is_ok() {
                return Ok(format!("playing \"{q}\" on spotify"));
            }
        }
        if Instant::now() >= end || stopped(d) {
            return Err(Fail::hint(
                format!("spotify shows the search for \"{q}\", but no play button matched"),
                "say the exact song name, or press play in spotify",
            ));
        }
        thread::sleep(RETRY_EVERY);
    }
}

fn youtube(d: &Daemon, ctx: &mut Ctx, q: &str) -> Result<String, Fail> {
    step(d, ctx, &argv(&["search", "youtube", q, "--new"]))?;
    let out = step(d, ctx, &argv(&["js", YT_FIRST]))?;
    let title = out.trim().trim_matches('"').trim();
    if title.is_empty() {
        return Err(Fail::hint(format!("youtube found no video for \"{q}\""), "try other words"));
    }
    Ok(format!("playing \"{title}\" on youtube"))
}

fn media(d: &Daemon, ctx: &mut Ctx, key: &str, times: u32) -> Result<String, Fail> {
    for _ in 0..times {
        step(d, ctx, &argv(&["press", key]))?;
    }
    Ok(if times > 1 { format!("pressed {key} x{times}") } else { format!("pressed {key}") })
}

fn url_of(home: &Path, what: &str) -> Option<String> {
    let low = what.to_lowercase();
    if low.starts_with("http://") || low.starts_with("https://") {
        return Some(what.to_string());
    }
    if let Some(m) = URLISH.captures(&low)
        && !NOT_WEB.contains(&&m[2])
    {
        return Some(format!("https://{what}"));
    }
    web::site(home, &low).map(|host| format!("https://{host}"))
}

fn open(d: &Daemon, ctx: &mut Ctx, what: &str) -> Result<String, Fail> {
    match url_of(&d.home, what) {
        Some(url) => {
            step(d, ctx, &argv(&["open", &url, "--new"]))?;
            Ok(format!("opened {url}"))
        }
        None => {
            step(d, ctx, &argv(&["launch", what]))?;
            hold_app(d, ctx, what);
            Ok(format!("started {what}"))
        }
    }
}

fn search(d: &Daemon, ctx: &mut Ctx, site: Option<&str>, q: &str) -> Result<String, Fail> {
    let site = site.unwrap_or(WEB_SEARCH);
    step(d, ctx, &argv(&["search", site, q, "--new"]))?;
    Ok(format!("searched {site} for \"{q}\""))
}

fn shot(d: &Daemon, ctx: &mut Ctx, of: Option<&str>) -> Result<String, Fail> {
    let target = of.map_or_else(|| "screen".to_string(), |o| format!("app:{o}"));
    step(d, ctx, &argv(&["shot", &target]))
}

fn hold(d: &Daemon, ctx: &mut Ctx, before: &BTreeSet<String>) {
    let new: Vec<String> = ctx.owned.difference(before).cloned().collect();
    for tab in new {
        let _ = once(d, ctx, &argv(&["keep", &tab]));
    }
}

fn hold_app(d: &Daemon, ctx: &mut Ctx, name: &str) {
    let _ = once(d, ctx, &argv(&["keep", &format!("app:{name}")]));
}

pub fn run(d: &Daemon, ctx: &mut Ctx, act: &Act, yes: bool) -> Result<String, Fail> {
    let before = ctx.owned.clone();
    let out = match act {
        Act::Msg { who, app, text } => message(&d.home, who, app.as_deref(), text, yes),
        Act::Play { app, q } if app.as_deref() == Some("youtube") => youtube(d, ctx, q),
        Act::Play { q, .. } => spotify(d, ctx, q),
        Act::Media { key, times } => media(d, ctx, key, *times),
        Act::Open { what } => open(d, ctx, what),
        Act::Search { site, q } => search(d, ctx, site.as_deref(), q),
        Act::Remind { text } => {
            agents::remind(&d.home, text, "Wisply");
            Ok(format!("reminded {text}"))
        }
        Act::Shot { of } => shot(d, ctx, of.as_deref()),
        Act::Remember { text } => once(d, ctx, &argv(&["remember", text])),
        Act::Recall { q } => step(d, ctx, &argv(&["recall", q])),
    };
    if matches!(act, Act::Play { .. } | Act::Open { .. } | Act::Search { .. }) {
        hold(d, ctx, &before);
    }
    out
}

pub fn preview(home: &Path, act: &Act, yes: bool) -> Result<String, Fail> {
    let Act::Msg { who, app, text } = act else { return Ok(String::new()) };
    guard(text, yes)?;
    let to = resolve(home, who, app.as_deref(), yes)?;
    let rows: Vec<String> = to.iter().map(|t| format!("{} ({})", t.person.name, t.app)).collect();
    Ok(rows.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_apps_and_links() {
        let home = std::env::temp_dir();
        assert_eq!(url_of(&home, "youtube.com").as_deref(), Some("https://youtube.com"));
        assert_eq!(url_of(&home, "https://x.org/a").as_deref(), Some("https://x.org/a"));
        assert_eq!(url_of(&home, "github.com/QDH").as_deref(), Some("https://github.com/QDH"));
        assert_eq!(url_of(&home, "notepad.exe"), None);
        assert_eq!(url_of(&home, "Minecraft"), None);
        assert!(url_of(&home, "youtube").is_some_and(|u| u.contains("youtube.com")));
        assert!(guard("schau mal https://x.de", false).is_err());
        assert!(guard("schau mal www.x.de", false).is_err());
        assert!(guard("schau mal https://x.de", true).is_ok());
        assert!(guard("bin gleich da", false).is_ok());
    }

    #[test]
    fn unknown_people_block_unless_yes() {
        let home = std::env::temp_dir().join(format!("wisply-go-act-{}", std::process::id()));
        let _ = std::fs::create_dir_all(&home);
        let _ = std::fs::remove_file(home.join("contacts.json"));
        contacts::learn(&home, "Anna", "discord", "anna_99", None);
        let who = |w: &[&str]| w.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
        let ok = resolve(&home, &who(&["anna"]), None, false).expect("anna");
        assert_eq!((ok[0].app.as_str(), ok[0].person.handle("discord").as_str()), ("discord", "anna_99"));
        let err = resolve(&home, &who(&["anna", "olaf"]), None, false).err().expect("olaf blocks");
        assert!(err.msg.starts_with("not sent") && err.msg.contains("olaf"));
        assert!(resolve(&home, &who(&["olaf"]), None, true).is_err());
        let forced = resolve(&home, &who(&["olaf"]), Some("whatsapp"), true).expect("forced");
        assert_eq!(forced[0].person.handle("whatsapp"), "olaf");
        assert!(
            preview(&home, &Act::Msg { who: who(&["anna"]), app: None, text: "hi".into() }, false)
                .is_ok_and(|p| p == "Anna (discord)")
        );
        let _ = std::fs::remove_dir_all(&home);
    }
}
