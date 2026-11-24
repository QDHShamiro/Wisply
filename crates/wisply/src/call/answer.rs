use std::collections::HashSet;
use std::path::Path;
use std::thread;
use std::time::Duration;

use serde_json::{Map, Value};
use wisply_core::Fail;

use super::cfg;
use super::net;
use super::ring::{self, Kind, Spec};
use super::store::{self, Ask};
use super::text::{now, quiet_now};

const STEP: Duration = Duration::from_millis(250);

pub struct Opts {
    pub question: String,
    pub choices: Vec<String>,
    pub wait_s: Option<u64>,
    pub channels: Option<String>,
    pub force: bool,
}

pub fn reply_parts(open: &[Ask], raw: &str) -> (String, String) {
    let trimmed = raw.trim();
    let (first, rest) = trimmed.split_once(char::is_whitespace).unwrap_or((trimmed, ""));
    let named = open.iter().find(|a| a.id.eq_ignore_ascii_case(first.trim_matches(['#', ':'])));
    match named {
        Some(a) => (a.id.clone(), rest.trim().to_string()),
        None => (String::new(), trimmed.to_string()),
    }
}

pub fn collect(home: &Path, settings: &Map<String, Value>, since: u64, seen: &mut HashSet<String>) {
    let home_cfg = crate::daemon::load_config(home);
    let topic = net::topic(settings, &home_cfg);
    if topic.is_empty() {
        return;
    }
    let server = net::server(settings, &home_cfg);
    let Ok(messages) = net::poll(&server, &net::reply_topic(settings, &topic), since) else { return };
    for (mid, time, body) in messages {
        if !seen.insert(mid) || time + 1 < since {
            continue;
        }
        let open = store::open_list(home);
        let (id, text) = reply_parts(&open, &body);
        let _ = store::submit(home, &id, &text, "ntfy");
    }
}

fn wait_for(home: &Path, settings: &Map<String, Value>, ask: &Ask) -> Result<Ask, Fail> {
    let since = (ask.created as u64).saturating_sub(1);
    let poll = Duration::from_secs(cfg::num(settings, "ntfy_poll_s").max(1));
    let mut seen = HashSet::new();
    let mut next = now();
    loop {
        if let Some(done) = store::load(home, &ask.id).filter(|a| a.state == "answered") {
            return Ok(done);
        }
        if now() >= ask.deadline {
            store::close(home, &ask.id, "expired");
            return Err(Fail::new(format!(
                "no answer within {} s (nobody picked up)",
                (ask.deadline - ask.created) as u64
            )));
        }
        if now() >= next {
            collect(home, settings, since, &mut seen);
            next = now() + poll.as_secs_f64();
        }
        thread::sleep(STEP);
    }
}

pub fn ask(home: &Path, opts: &Opts) -> Result<(Ask, String), Fail> {
    let settings = cfg::load(home);
    if !cfg::on(&settings, "enabled") {
        return Err(Fail::hint("calls are off", "wisply call set enabled=on"));
    }
    if quiet_now(&settings) && !opts.force {
        return Err(Fail::hint(
            format!(
                "quiet hours ({}-{}), nobody is rung",
                cfg::text(&settings, "quiet_from"),
                cfg::text(&settings, "quiet_to")
            ),
            "ask again with --force, or decide yourself",
        ));
    }
    let wait = opts.wait_s.unwrap_or_else(|| cfg::num(&settings, "wait_s"));
    let ask = store::create(home, &opts.question, &opts.choices, wait.max(1))?;
    let spec = Spec {
        kind: Kind::Ask,
        title: cfg::text(&settings, "title_ask"),
        text: opts.question.clone(),
        ask: Some(&ask),
        channels: ring::pick(&settings, "channels", opts.channels.as_deref()),
    };
    super::talk::open(home, &settings, &ask);
    let (rang, line) = ring::report(&ring::send(home, &settings, &spec));
    if !rang {
        store::close(home, &ask.id, "failed");
        return Err(Fail::hint(line, "wisply call setup"));
    }
    if wait == 0 {
        return Ok((ask, line));
    }
    let done = wait_for(home, &settings, &ask)?;
    Ok((done, line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::call::net::mock;
    use serde_json::json;
    use std::path::PathBuf;

    fn temp(name: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("wisply-call-answer-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn setup(home: &Path, server: &str) {
        cfg::set(
            home,
            &[
                format!("ntfy_server={server}"),
                "ntfy_topic=t-test".into(),
                "ntfy_poll_s=1".into(),
                "quiet_from=00:00".into(),
                "quiet_to=00:00".into(),
            ],
        )
        .unwrap();
    }

    #[test]
    fn replies_name_a_question_or_fall_back_to_the_latest() {
        let home = temp("parts");
        let one = store::create(&home, "A?", &["ja".into(), "nein".into()], 60).unwrap();
        let open = store::open_list(&home);
        assert_eq!(reply_parts(&open, &format!("{} ja", one.id)), (one.id.clone(), "ja".to_string()));
        assert_eq!(reply_parts(&open, &format!("#{}: nein", one.id)), (one.id.clone(), "nein".to_string()));
        assert_eq!(reply_parts(&open, "klar mach"), (String::new(), "klar mach".to_string()));
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn a_button_press_on_the_reply_topic_answers_the_blocked_ask() {
        let home = temp("press");
        let srv = mock::start();
        setup(&home, &srv.url);
        let t = now() as u64 + 5;
        srv.replies
            .lock()
            .unwrap()
            .push(json!({"event": "message", "id": "m1", "time": t, "message": "nein"}).to_string());
        let opts = Opts {
            question: "Deploy?".into(),
            choices: vec!["ja".into(), "nein".into()],
            wait_s: Some(20),
            channels: None,
            force: false,
        };
        let (done, line) = ask(&home, &opts).unwrap();
        assert_eq!((done.answer.as_deref(), done.via.as_str()), (Some("nein"), "ntfy"));
        assert!(line.contains("rang via ntfy"));
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn nobody_answering_ends_with_a_clear_error() {
        let home = temp("late");
        let srv = mock::start();
        setup(&home, &srv.url);
        let opts = Opts { question: "Hallo?".into(), choices: vec![], wait_s: Some(2), channels: None, force: false };
        let err = ask(&home, &opts).unwrap_err();
        assert!(err.text().contains("no answer within"));
        assert_eq!(store::all(&home)[0].state, "expired");
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn quiet_hours_and_a_missing_setup_stop_before_any_ring() {
        let home = temp("quiet");
        cfg::set(&home, &["quiet_from=00:00".into(), "quiet_to=24:00".into()]).unwrap();
        let opts = Opts { question: "x".into(), choices: vec![], wait_s: Some(1), channels: None, force: false };
        assert!(ask(&home, &opts).unwrap_err().text().contains("quiet hours"));
        let forced = Opts { force: true, ..opts };
        assert!(ask(&home, &forced).unwrap_err().text().contains("no ntfy topic yet"));
        assert!(store::open_list(&home).is_empty());
        let _ = std::fs::remove_dir_all(&home);
    }
}
