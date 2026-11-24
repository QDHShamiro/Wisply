use std::path::Path;

use serde_json::{Map, Value, json};
use wisply_core::Fail;

use super::cfg::{self, fill};
use super::net;
use super::store::Ask;
use super::text::{clip, encode, safe, secret};

pub const CHANNELS: [&str; 4] = ["ntfy", "callmebot", "discord", "app"];
const ACTION_SLOTS: usize = 3;
const FAIL_WORDS: [&str; 4] = ["not authorized", "warning", "error", "failed"];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Ask,
    Notice,
    Briefing,
}

pub struct Spec<'a> {
    pub kind: Kind,
    pub title: String,
    pub text: String,
    pub ask: Option<&'a Ask>,
    pub channels: Vec<String>,
}

pub fn pick(cfg: &Map<String, Value>, key: &str, over: Option<&str>) -> Vec<String> {
    let mut want: Vec<String> = match over {
        Some(list) => list.split(',').map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect(),
        None => cfg::list(cfg, key),
    };
    if want.is_empty() && key != "channels" {
        want = cfg::list(cfg, "channels");
    }
    want.dedup();
    want
}

fn priority(cfg: &Map<String, Value>, kind: Kind) -> u64 {
    match kind {
        Kind::Ask => cfg::num(cfg, "ntfy_priority"),
        Kind::Notice => cfg::num(cfg, "ntfy_notice_priority"),
        Kind::Briefing => cfg::num(cfg, "ntfy_briefing_priority"),
    }
}

fn buttons(cfg: &Map<String, Value>, server: &str, topic: &str, ask: Option<&Ask>) -> Vec<Value> {
    let join = cfg::text(cfg, "voice_join_url");
    let mut out: Vec<Value> = Vec::new();
    let wants_join = cfg::on(cfg, "voice") && !join.trim().is_empty();
    let room = ACTION_SLOTS - usize::from(wants_join);
    if let Some(ask) = ask {
        let reply = format!("{server}/{}", net::reply_topic(cfg, topic));
        for choice in ask.choices.iter().take(room) {
            let label = choice
                .chars()
                .next()
                .map_or(String::new(), |c| c.to_uppercase().collect::<String>() + &choice[c.len_utf8()..]);
            out.push(net::button(&label, &reply, &format!("{} {choice}", ask.id)));
        }
    }
    if wants_join {
        out.push(json!({"action": "view", "label": cfg::text(cfg, "label_join"), "url": join.trim()}));
    }
    out
}

fn ntfy(cfg: &Map<String, Value>, home_cfg: &Map<String, Value>, spec: &Spec) -> Result<(), Fail> {
    let topic = net::topic(cfg, home_cfg);
    if topic.is_empty() {
        return Err(Fail::hint("no ntfy topic yet", "wisply call setup"));
    }
    let server = net::server(cfg, home_cfg);
    let mut body = spec.text.clone();
    if let Some(ask) = spec.ask {
        let reply = net::reply_topic(cfg, &topic);
        body = format!("{body}\n{}", fill(&cfg::text(cfg, "text_reply"), &[("reply", &reply), ("id", &ask.id)]));
    }
    let tags: Vec<String> = cfg::list(cfg, "ntfy_tags");
    let mut msg = json!({
        "topic": topic, "title": spec.title, "message": body,
        "priority": priority(cfg, spec.kind), "tags": tags
    });
    let actions = buttons(cfg, &server, &topic, spec.ask);
    if !actions.is_empty() {
        msg["actions"] = Value::Array(actions);
    }
    net::publish(&server, &msg)
}

fn callmebot(cfg: &Map<String, Value>, spec: &Spec) -> Result<(), Fail> {
    let user = cfg::text(cfg, "callmebot_user");
    if user.trim().is_empty() {
        return Err(Fail::hint("no Telegram name yet", "wisply call set callmebot_user=@deinname"));
    }
    let max = cfg::num(cfg, "callmebot_text_max") as usize;
    let mut say = spec.text.clone();
    if let Some(ask) = spec.ask.filter(|a| !a.choices.is_empty()) {
        say = format!("{say} {}", fill(&cfg::text(cfg, "text_choices"), &[("choices", &ask.choices.join(", "))]));
    }
    let url = format!(
        "{}?user={}&text={}&lang={}&rpt={}",
        cfg::text(cfg, "callmebot_url").trim(),
        encode(user.trim()),
        encode(&clip(&say, max)),
        encode(&cfg::text(cfg, "callmebot_lang")),
        cfg::num(cfg, "callmebot_repeat")
    );
    let reply = net::get(&url)?;
    let lower = reply.body.to_lowercase();
    if !reply.good() || FAIL_WORDS.iter().any(|w| lower.contains(w)) {
        return Err(Fail::hint(
            "CallMeBot did not start the call",
            "authorize it once in Telegram: send /start to @CallMeBot_txtbot",
        ));
    }
    Ok(())
}

fn discord(cfg: &Map<String, Value>, spec: &Spec) -> Result<(), Fail> {
    let key = cfg::text(cfg, "discord_webhook_key");
    let url = secret(&key).ok_or_else(|| {
        Fail::hint("no Discord webhook yet", format!("put {key}=<webhook url> into ~/.config/watch/.env"))
    })?;
    let user = cfg::text(cfg, "discord_user_id");
    let ping = if user.trim().is_empty() { String::new() } else { format!("<@{}> ", user.trim()) };
    let mut content = format!("{ping}**{}**\n{}", spec.title, spec.text);
    if let Some(ask) = spec.ask {
        content.push_str(&format!("\n`{}` {}", ask.id, ask.choices.join(" / ")));
    }
    let allowed = if user.trim().is_empty() { json!({"parse": []}) } else { json!({"users": [user.trim()]}) };
    let reply = net::post_json(&url, &json!({"content": content, "allowed_mentions": allowed}))?;
    if reply.good() { Ok(()) } else { Err(Fail::new(format!("Discord answered {}", reply.status))) }
}

fn app(home: &Path, cfg: &Map<String, Value>, spec: &Spec) -> Result<(), Fail> {
    let kind = cfg::text(cfg, "app_kind");
    let (code, out) = crate::client::ask(home, &["notify", "event", &kind, &spec.title, &spec.text])
        .map_err(|e| Fail::new(format!("the Wisply daemon is not reachable ({e})")))?;
    if code == 0 { Ok(()) } else { Err(Fail::new(clip(&out, 120))) }
}

pub fn send(home: &Path, cfg: &Map<String, Value>, spec: &Spec) -> Vec<(String, Result<(), Fail>)> {
    net::tune(cfg);
    let home_cfg = crate::daemon::load_config(home);
    let max = cfg::num(cfg, "text_max") as usize;
    let clean = Spec {
        kind: spec.kind,
        title: safe(&spec.title, max),
        text: safe(&spec.text, max),
        ask: spec.ask,
        channels: spec.channels.clone(),
    };
    spec.channels
        .iter()
        .map(|name| {
            let result = match name.as_str() {
                "ntfy" => ntfy(cfg, &home_cfg, &clean),
                "callmebot" => callmebot(cfg, &clean),
                "discord" => discord(cfg, &clean),
                "app" => app(home, cfg, &clean),
                other => Err(Fail::hint(format!("unknown channel '{other}'"), CHANNELS.join(", "))),
            };
            (name.clone(), result)
        })
        .collect()
}

pub fn report(results: &[(String, Result<(), Fail>)]) -> (bool, String) {
    let ok: Vec<&str> = results.iter().filter(|(_, r)| r.is_ok()).map(|(n, _)| n.as_str()).collect();
    let bad: Vec<String> =
        results.iter().filter_map(|(n, r)| r.as_ref().err().map(|e| format!("{n}: {}", e.text()))).collect();
    let mut line = if ok.is_empty() { "no channel rang".to_string() } else { format!("rang via {}", ok.join(", ")) };
    for b in &bad {
        line.push_str(&format!("\n  {b}"));
    }
    (!ok.is_empty(), line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call::net::mock;
    use crate::call::store;
    use serde_json::json;
    use std::path::PathBuf;

    fn temp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("wisply-call-ring-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn ntfy_gets_priority_5_buttons_and_the_reply_hint() {
        let home = temp("ntfy");
        let srv = mock::start();
        let mut c = cfg::defaults();
        c.insert("ntfy_server".into(), json!(srv.url));
        c.insert("ntfy_topic".into(), json!("t-secret"));
        c.insert("voice".into(), json!(true));
        c.insert("voice_join_url".into(), json!("https://discord.com/channels/1/2"));
        let ask = store::create(&home, "Deploy?", &["ja".into(), "nein".into()], 60).unwrap();
        let spec = Spec {
            kind: Kind::Ask,
            title: "T".into(),
            text: "Deploy?".into(),
            ask: Some(&ask),
            channels: vec!["ntfy".into()],
        };
        let out = send(&home, &c, &spec);
        assert!(out[0].1.is_ok(), "{:?}", out[0].1.as_ref().err().map(Fail::text));
        let seen = srv.seen.lock().unwrap();
        let body: Value = serde_json::from_str(&seen[0].2).unwrap();
        assert_eq!(seen[0].0, "POST");
        assert_eq!((body["topic"].as_str(), body["priority"].as_u64()), (Some("t-secret"), Some(5)));
        let actions = body["actions"].as_array().unwrap();
        assert_eq!(actions.len(), 3);
        assert_eq!(actions[0]["body"], format!("{} ja", ask.id));
        assert!(actions[0]["url"].as_str().unwrap().ends_with("/t-secret-re"));
        assert_eq!(actions[2]["action"], "view");
        assert!(body["message"].as_str().unwrap().contains(&ask.id));
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn callmebot_builds_a_clipped_encoded_call_and_reads_failures() {
        let home = temp("cmb");
        let srv = mock::start();
        let mut c = cfg::defaults();
        c.insert("callmebot_url".into(), json!(format!("{}/start.php", srv.url)));
        c.insert("callmebot_user".into(), json!("@wisply_test"));
        c.insert("callmebot_text_max".into(), json!(40));
        let long = "Bitte antworte jetzt, es geht um etwas sehr Wichtiges & Eiliges";
        let spec = Spec {
            kind: Kind::Notice,
            title: "T".into(),
            text: long.into(),
            ask: None,
            channels: vec!["callmebot".into()],
        };
        assert!(send(&home, &c, &spec)[0].1.is_ok());
        let path = srv.seen.lock().unwrap()[0].1.clone();
        assert!(
            path.starts_with("/start.php?user=%40wisply_test&text=") && path.contains("&lang=de-DE-Standard-B&rpt=2")
        );
        assert!(path.contains("%E2%80%A6"));
        c.insert("callmebot_user".into(), json!(""));
        assert!(send(&home, &c, &spec)[0].1.is_err());
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn a_missing_topic_or_unknown_channel_is_a_clear_error() {
        let home = temp("miss");
        let c = cfg::defaults();
        let spec = Spec {
            kind: Kind::Notice,
            title: "T".into(),
            text: "x".into(),
            ask: None,
            channels: vec!["ntfy".into(), "fax".into()],
        };
        let out = send(&home, &c, &spec);
        let (rang, line) = report(&out);
        assert!(!rang && line.contains("no ntfy topic yet"));
        assert!(line.contains("unknown channel 'fax'"));
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn channel_lists_fall_back_to_the_default() {
        let mut c = cfg::defaults();
        assert_eq!(pick(&c, "briefing_channels", None), ["ntfy"]);
        c.insert("briefing_channels".into(), json!("ntfy, callmebot"));
        assert_eq!(pick(&c, "briefing_channels", None), ["ntfy", "callmebot"]);
        assert_eq!(pick(&c, "channels", Some("discord")), ["discord"]);
    }
}
