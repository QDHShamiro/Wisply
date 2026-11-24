use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value, json};
use wisply_core::Fail;

pub fn defaults() -> Map<String, Value> {
    [ring(), talk(), guard(), hook(), brief(), words(), spoken()]
        .into_iter()
        .filter_map(|v| v.as_object().cloned())
        .flatten()
        .collect()
}

fn ring() -> Value {
    json!({
        "enabled": true,
        "channels": "ntfy",
        "wait_s": 300,
        "text_max": 400,
        "http_timeout_s": 15,
        "ntfy_server": "",
        "ntfy_topic": "",
        "ntfy_reply_topic": "",
        "ntfy_priority": 5,
        "ntfy_notice_priority": 3,
        "ntfy_briefing_priority": 4,
        "ntfy_tags": "telephone_receiver",
        "ntfy_poll_s": 3,
        "callmebot_url": "https://api.callmebot.com/start.php",
        "callmebot_user": "",
        "callmebot_lang": "de-DE-Standard-B",
        "callmebot_repeat": 2,
        "callmebot_text_max": 250,
        "discord_user_id": "",
        "discord_webhook_key": "WISPLY_CALL_DISCORD_WEBHOOK",
        "app_kind": "waiting"
    })
}

fn talk() -> Value {
    json!({
        "voice": false,
        "voice_target": "",
        "voice_join_url": "",
        "voice_wait_s": 240,
        "voice_linger_s": 25,
        "understand": "auto",
        "understand_timeout_s": 8
    })
}

fn guard() -> Value {
    json!({
        "away_after_min": 15,
        "min_gap_s": 300,
        "quiet_from": "23:00",
        "quiet_to": "07:00"
    })
}

fn hook() -> Value {
    json!({
        "hook_wait_s": 90,
        "hook_events": "permission,stop,idle",
        "hook_tool_chars": 160,
        "hook_stop_chars": 200,
        "hook_yes": "ja",
        "hook_no": "nein",
        "yes_words": "ja,jo,jep,jap,klar,okay,ok,mach,genau,passt,gerne,yes,yep,sure,erlaubt,freigegeben,weiter,los,gut",
        "no_words": "nein,nee,nö,noe,nope,no,stopp,stop,abbrechen,ablehnen,verboten,lieber nicht"
    })
}

fn brief() -> Value {
    json!({
        "briefing_at": "07:30",
        "briefing_items": 5,
        "briefing_channels": "",
        "albion_dir": "",
        "albion_hours": 24
    })
}

fn words() -> Value {
    json!({
        "title_ask": "Wisply ruft an",
        "title_notice": "Wisply meldet sich",
        "title_briefing": "Guten Morgen",
        "text_choices": "Antworte mit: {choices}",
        "text_reply": "Antwort per Text: an {reply} senden: {id} <Antwort>",
        "text_hook_permission": "Claude will {tool} ausführen: {what}",
        "text_hook_stop": "Claude ist fertig: {what}",
        "text_hook_idle": "Claude wartet auf dich: {what}",
        "text_hook_denied": "QDHShamiro hat per Anruf abgelehnt",
        "text_briefing_todo": "{n} offene To-dos",
        "text_briefing_agents": "{running} Sessions laufen, {waiting} warten auf dich",
        "text_briefing_albion": "{n} neue Nachrichten von Albion",
        "text_briefing_empty": "Nichts Neues.",
        "text_test": "Testanruf von Wisply. Wenn du das liest, klappt das Klingeln.",
        "label_yes": "Ja",
        "label_no": "Nein",
        "label_join": "Call beitreten"
    })
}

fn spoken() -> Value {
    json!({
        "say_opening": "Hi, hier ist Wisply. Ich habe eine Frage an dich.",
        "say_ask": "{question} {choices}",
        "say_choices": "Sag {choices}.",
        "say_unclear": "Das habe ich nicht verstanden. {choices}",
        "say_got": "Okay, notiert: {answer}.",
        "say_got_note": "Okay, notiert: {answer}, {note}.",
        "say_idle": "Gerade ist nichts offen.",
        "say_bye": "Das war es von mir. Bis später."
    })
}

pub fn dir(home: &Path) -> PathBuf {
    home.join("call")
}

fn file(home: &Path) -> PathBuf {
    dir(home).join("config.json")
}

pub fn load(home: &Path) -> Map<String, Value> {
    let mut cfg = defaults();
    if let Some(saved) = fs::read_to_string(file(home))
        .ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v.as_object().cloned())
    {
        cfg.extend(saved);
    }
    cfg
}

fn save(home: &Path, cfg: &Map<String, Value>) -> Result<(), Fail> {
    let base = defaults();
    let keep: Map<String, Value> =
        cfg.iter().filter(|(k, v)| base.get(*k) != Some(*v)).map(|(k, v)| (k.clone(), v.clone())).collect();
    let _ = fs::create_dir_all(dir(home));
    wisply_core::save(file(home), serde_json::to_string_pretty(&Value::Object(keep)).unwrap_or_default())
        .map_err(|e| Fail::new(format!("could not save the call settings: {e}")))
}

pub fn text(cfg: &Map<String, Value>, key: &str) -> String {
    match cfg.get(key) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Null) | None => String::new(),
        Some(v) => v.to_string(),
    }
}

pub fn num(cfg: &Map<String, Value>, key: &str) -> u64 {
    cfg.get(key)
        .and_then(|v| v.as_u64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
        .unwrap_or_else(|| defaults().get(key).and_then(Value::as_u64).unwrap_or(0))
}

pub fn on(cfg: &Map<String, Value>, key: &str) -> bool {
    match cfg.get(key) {
        Some(Value::Bool(b)) => *b,
        Some(Value::String(s)) => truthy(s),
        _ => false,
    }
}

pub fn list(cfg: &Map<String, Value>, key: &str) -> Vec<String> {
    text(cfg, key).split(',').map(|s| s.trim().to_lowercase()).filter(|s| !s.is_empty()).collect()
}

pub fn fill(template: &str, pairs: &[(&str, &str)]) -> String {
    let mut out = template.to_string();
    for (k, v) in pairs {
        out = out.replace(&format!("{{{k}}}"), v);
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn truthy(raw: &str) -> bool {
    matches!(raw.to_lowercase().as_str(), "1" | "on" | "true" | "ja" | "yes")
}

fn coerce(default: &Value, raw: &str) -> Option<Value> {
    match default {
        Value::Bool(_) => Some(json!(truthy(raw))),
        Value::Number(_) => raw.parse::<u64>().ok().map(|v| json!(v)),
        Value::String(_) => Some(json!(raw)),
        _ => None,
    }
}

pub fn clock(raw: &str) -> Option<u32> {
    let (h, m) = raw.trim().split_once(':')?;
    let (h, m): (u32, u32) = (h.parse().ok()?, m.parse().ok()?);
    ((h < 24 && m < 60) || (h == 24 && m == 0)).then_some(h * 60 + m)
}

pub fn set(home: &Path, pairs: &[String]) -> Result<String, Fail> {
    let base = defaults();
    let mut cfg = load(home);
    for kv in pairs {
        let (k, v) = kv.split_once('=').unwrap_or((kv.as_str(), ""));
        let Some(d) = base.get(k) else {
            let all: Vec<&str> = base.keys().map(String::as_str).collect();
            return Err(Fail::hint(format!("unknown setting '{k}'"), format!("settings: {}", all.join(", "))));
        };
        let value = coerce(d, v).ok_or_else(|| Fail::new(format!("'{v}' does not fit {k}")))?;
        if (k == "quiet_from" || k == "quiet_to" || k == "briefing_at") && clock(v).is_none() {
            return Err(Fail::hint(format!("'{v}' is not a time"), format!("wisply call set {k}=07:30")));
        }
        if k == "understand" && !["auto", "off"].contains(&v) {
            return Err(Fail::hint(format!("'{v}' is not auto or off"), "wisply call set understand=auto"));
        }
        cfg.insert(k.to_string(), value);
    }
    save(home, &cfg)?;
    Ok(format!("call: saved {}", pairs.join(", ")))
}

pub fn show(home: &Path) -> String {
    let cfg = load(home);
    let base = defaults();
    let hidden = |k: &str| {
        ["text_", "title_", "label_", "say_"].iter().any(|p| k.starts_with(p))
            && base.get(k).is_some_and(Value::is_string)
    };
    let mut lines =
        vec!["call settings (wisply call set key=value; texts: set text_*, title_*, label_*, say_*):".to_string()];
    for k in cfg.keys() {
        if hidden(k) {
            continue;
        }
        let shown = match k.as_str() {
            "ntfy_topic" | "ntfy_reply_topic" | "callmebot_user" | "discord_user_id" if !text(&cfg, k).is_empty() => {
                "(set)".to_string()
            }
            _ => text(&cfg, k),
        };
        lines.push(format!("  {k} = {shown}"));
    }
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("wisply-call-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn settings_save_only_what_differs_and_check_types() {
        let home = temp("cfg");
        set(&home, &["wait_s=60".into(), "voice=on".into(), "channels=ntfy,callmebot".into()]).unwrap();
        let cfg = load(&home);
        assert_eq!(num(&cfg, "wait_s"), 60);
        assert!(on(&cfg, "voice"));
        assert_eq!(list(&cfg, "channels"), ["ntfy", "callmebot"]);
        let saved = fs::read_to_string(file(&home)).unwrap();
        assert!(saved.contains("wait_s") && !saved.contains("text_max"));
        assert!(set(&home, &["nope=1".into()]).is_err());
        assert!(set(&home, &["quiet_from=25:00".into()]).is_err());
        assert!(set(&home, &["wait_s=abc".into()]).is_err());
        let _ = fs::remove_dir_all(&home);
    }

    #[test]
    fn templates_fill_and_clocks_parse() {
        assert_eq!(fill("a {x}  b {y}", &[("x", "1"), ("y", "")]), "a 1 b");
        assert_eq!(clock("07:30"), Some(450));
        assert_eq!(clock("24:00"), Some(1440));
        assert_eq!(clock("25:00"), None);
        assert_eq!(clock("x"), None);
    }

    #[test]
    fn the_secret_settings_are_not_printed() {
        let home = temp("show");
        set(&home, &["ntfy_topic=very-secret-topic".into()]).unwrap();
        let shown = show(&home);
        assert!(shown.contains("ntfy_topic = (set)") && !shown.contains("very-secret-topic"));
        let _ = fs::remove_dir_all(&home);
    }
}
