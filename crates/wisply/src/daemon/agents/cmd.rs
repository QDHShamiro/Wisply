use std::path::Path;
use std::time::Instant;

use serde_json::Value;
use wisply_core::{Call, Fail, ago, local_parts};

use super::board::{now, resume, snapshot, stop, tail};
use super::events::{KINDS, emit};
use super::extra;
use super::model::{Row, Status};
use super::qr;
use super::server;
use crate::daemon::config;

pub const MOBILE_USAGE: &str = "mobile pair [--name handy] | list | revoke <name> | status | on | off";
pub const NOTIFY_USAGE: &str = "notify event <done|waiting|ci_red|ci_green|limit|resumed|disk> <title> [text]";
const SHOWN: usize = 20;
const NAME_SHOWN: usize = 28;

fn short_count(n: i64) -> String {
    match n {
        1_000_000.. => format!("{:.1}M", n as f64 / 1_000_000.0),
        1_000.. => format!("{:.1}k", n as f64 / 1_000.0),
        _ => n.to_string(),
    }
}

fn clock(at: i64) -> String {
    let [_, _, _, h, m, _] = local_parts(at as f64);
    format!("{h:02}:{m:02}")
}

pub fn summary(rows: &[Row]) -> String {
    let count = |s: Status| rows.iter().filter(|r| r.status == s).count();
    let mut bits: Vec<String> = [Status::Running, Status::Waiting, Status::Limit, Status::Error]
        .into_iter()
        .filter(|s| count(*s) > 0)
        .map(|s| format!("{} {}", count(s), s.word()))
        .collect();
    if let Some(at) = rows.iter().filter(|r| r.status == Status::Limit).filter_map(|r| r.reset_at).max() {
        bits.push(format!("back after {}", clock(at)));
    }
    bits.push(format!("{} done", count(Status::Done)));
    format!("agents: {}", bits.join(", "))
}

pub fn line(r: &Row, at: i64) -> String {
    let name: String = if r.name.is_empty() { r.dir_short.clone() } else { r.name.clone() };
    let name: String = name.chars().take(NAME_SHOWN).collect();
    let age = ago(u64::try_from((at - r.since).max(0)).unwrap_or(0));
    let said = if r.last_line.is_empty() { String::new() } else { format!(" | {}", r.last_line) };
    format!(
        "{} {} {age} {name} [{}] {} ${:.2}{said}",
        r.id,
        r.status.word(),
        r.dir_short,
        short_count(r.tokens),
        r.cost
    )
}

fn find(rows: &[Row], id: Option<&String>) -> Result<Row, Fail> {
    let id = id.ok_or_else(|| Fail::hint("agent id needed", "wisply agents"))?;
    rows.iter().find(|r| &r.id == id).cloned().ok_or_else(|| Fail::hint(format!("no agent '{id}'"), "wisply agents"))
}

fn act(home: &Path, c: &Call, secret: Option<&str>) -> Result<String, Fail> {
    if c.pos[0] == "status" {
        return Ok(server::shared(home).backend.status().to_string());
    }
    let rows = snapshot(false)?;
    let row = find(&rows, c.pos.get(1))?;
    match c.pos[0].as_str() {
        "show" => {
            let mut v = row.json();
            extra::flag(&mut v, home, &row);
            v["tail"] = serde_json::json!(tail(&row));
            extra::attach(&mut v, home, &row);
            Ok(v.to_string())
        }
        "say" => {
            let text = secret.map(str::trim).filter(|t| !t.is_empty()).ok_or_else(|| Fail::new("no text to send"))?;
            if !row.background || row.status == Status::Running {
                return Err(Fail::new("not ready for a reply"));
            }
            resume(&row.sid, &row.dir, text).map(|()| "sent".into())
        }
        _ => stop(&row.id).map(|()| "stopped".into()),
    }
}

pub fn agents(home: &Path, c: &Call, secret: Option<&str>) -> Result<String, Fail> {
    if c.pos.first().is_some_and(|s| ["show", "say", "stop", "status"].contains(&s.as_str())) {
        return act(home, c, secret);
    }
    let rows = snapshot(true)?;
    if c.on("json") {
        let wired = rows.iter().map(|r| {
            let mut v = r.json();
            extra::flag(&mut v, home, r);
            v
        });
        return Ok(Value::Array(wired.collect()).to_string());
    }
    let at = now();
    let active = |r: &&Row| r.status != Status::Done;
    let mut shown: Vec<&Row> = rows.iter().filter(active).collect();
    let cap = if c.on("all") { usize::MAX } else { SHOWN };
    shown.extend(rows.iter().filter(|r| r.status == Status::Done).take(cap.saturating_sub(shown.len())));
    shown.truncate(cap);
    let hidden = rows.len() - shown.len();
    let mut out = vec![summary(&rows)];
    out.extend(shown.iter().map(|r| line(r, at)));
    if hidden > 0 {
        out.push(format!("(+{hidden} more: wisply agents --all)"));
    }
    Ok(out.join("\n"))
}

pub fn notify(d_home: &Path, c: &Call) -> Result<String, Fail> {
    let usage = || Fail::hint("notify needs an event", format!("wisply {NOTIFY_USAGE}"));
    if c.pos.first().map(String::as_str) != Some("event") {
        return Err(usage());
    }
    let (Some(kind), Some(title)) = (c.pos.get(1), c.pos.get(2)) else { return Err(usage()) };
    let kind = kind.to_lowercase();
    if !KINDS.contains(&kind.as_str()) {
        return Err(Fail::hint(format!("unknown event kind '{kind}'"), format!("one of: {}", KINDS.join(", "))));
    }
    let body = c.pos[3..].join(" ");
    let toast = config::on(&config::load(d_home), "agents_notify");
    let e = emit(d_home, &kind, c.val("agent"), title, &body, toast);
    Ok(format!("event {} {} sent", e.seq, e.kind))
}

fn set(home: &Path, key: &str, value: &str) -> Result<(), Fail> {
    config::command(home, &[key.to_string(), value.to_string()]).map(|_| ())
}

pub fn mobile(home: &Path, c: &Call) -> Result<String, Fail> {
    let sub = c.pos.first().map_or("status", String::as_str);
    match sub {
        "pair" => pair(home, c),
        "list" => {
            let list = server::shared(home).auth.list();
            if list.is_empty() {
                return Ok("no phone paired: wisply mobile pair".into());
            }
            Ok(list.iter().map(|d| format!("{} (paired {})", d.name, stamp(d.created))).collect::<Vec<_>>().join("\n"))
        }
        "revoke" => {
            let name = c.pos[1..].join(" ");
            if name.is_empty() {
                return Err(Fail::hint("revoke needs a name", "wisply mobile list"));
            }
            if server::shared(home).auth.revoke(&name) {
                Ok(format!("{name}: token revoked, the phone is out"))
            } else {
                Err(Fail::hint(format!("no phone named '{name}'"), "wisply mobile list"))
            }
        }
        "on" => {
            set(home, "phone_api", "on")?;
            Ok(format!("phone API on: {}", server::ensure(home).map_err(Fail::new)?.join(", ")))
        }
        "off" => {
            set(home, "phone_api", "off")?;
            server::stop();
            Ok("phone API off".into())
        }
        "status" => Ok(server::describe(home)),
        _ => Err(Fail::hint(format!("unknown mobile command '{sub}'"), format!("wisply {MOBILE_USAGE}"))),
    }
}

fn stamp(at: i64) -> String {
    let [_, mo, d, h, m, _] = local_parts(at as f64);
    format!("{mo:02}-{d:02} {h:02}:{m:02}")
}

fn pair(home: &Path, c: &Call) -> Result<String, Fail> {
    set(home, "phone_api", "on")?;
    let addrs = server::ensure(home).map_err(Fail::new)?;
    let name = c.val("name").unwrap_or("phone");
    let code = server::shared(home)
        .auth
        .new_code(name, Instant::now())
        .ok_or_else(|| Fail::hint("too many open pairing codes", "wait 10 minutes or use one of them"))?;
    let url = server::pair_url(&addrs);
    let link = qr::pair_link(&url, &code);
    let mut out = vec![format!(
        "pair \"{}\": scan this in the Wisply app (code works once, 10 min)",
        super::auth::clean_name(name)
    )];
    match qr::terminal(&link) {
        Some(art) => out.push(art),
        None => out.push("(QR picture failed, use the link)".into()),
    }
    if let Some(path) = qr::png(&link, &home.join("agents")) {
        out.push(format!("picture: {}", path.display()));
    }
    out.push(format!("link: {link}"));
    out.push(format!("web: {url}/#pair={code}"));
    out.push(format!("listening on {}", addrs.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")));
    Ok(out.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daemon::agents::api::fake::row;

    #[test]
    fn the_summary_counts_by_status_and_names_the_comeback_time() {
        let mut limited = row("c", Status::Limit);
        limited.reset_at = Some(1_730_000_000);
        let rows = vec![
            row("a", Status::Running),
            row("b", Status::Waiting),
            limited,
            row("d", Status::Done),
            row("e", Status::Done),
        ];
        let text = summary(&rows);
        assert!(text.starts_with("agents: 1 running, 1 waiting, 1 limit, back after "), "{text}");
        assert!(text.ends_with(", 2 done"), "{text}");
    }

    #[test]
    fn a_line_is_short_and_keeps_the_path_out() {
        let mut r = row("ab12", Status::Running);
        r.tokens = 12_345;
        let text = line(&r, 70);
        assert!(text.starts_with("ab12 running 60s job [place] "), "{text}");
        assert!(text.ends_with("$0.50 | ok"), "{text}");
        assert!(!text.contains("private"));
    }

    #[test]
    fn notify_checks_the_kind_and_the_shape() {
        let home = std::env::temp_dir().join(format!("wisply-agents-cmd-{}", std::process::id()));
        let call = |pos: &[&str]| Call {
            name: "notify".into(),
            pos: pos.iter().map(|s| (*s).to_string()).collect(),
            flags: Default::default(),
        };
        assert!(notify(&home, &call(&[])).is_err());
        assert!(notify(&home, &call(&["event", "done"])).is_err());
        assert!(notify(&home, &call(&["event", "nope", "t"])).unwrap_err().text().contains("one of"));
        let _ = std::fs::remove_dir_all(&home);
    }
}
