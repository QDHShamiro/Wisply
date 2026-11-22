use std::path::Path;
use std::sync::LazyLock;
use std::thread;
use std::time::Duration;

use ureq::tls::{RootCerts, TlsConfig};

use super::events::Event;
use crate::daemon::config;

const WAIT: Duration = Duration::from_secs(15);
const DEFAULT_SERVER: &str = "https://ntfy.sh";

static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::Agent::config_builder()
        .timeout_global(Some(WAIT))
        .http_status_as_error(false)
        .tls_config(TlsConfig::builder().root_certs(RootCerts::PlatformVerifier).build())
        .build()
        .into()
});

pub fn send(home: &Path, e: &Event) {
    let cfg = config::load(home);
    if config::text(&cfg, "push").to_lowercase() != "ntfy" {
        return;
    }
    let topic = config::text(&cfg, "push_topic");
    let topic = topic.trim();
    if topic.is_empty() {
        return;
    }
    let raw = config::text(&cfg, "push_server");
    let server = raw.trim().trim_end_matches('/');
    let server = if server.is_empty() { DEFAULT_SERVER } else { server };
    let url = endpoint(server, topic);
    let title = ascii(&e.title);
    let body = if e.body.is_empty() { e.title.clone() } else { e.body.clone() };
    let (prio, tags) = mark(&e.kind);
    thread::spawn(move || {
        let _ = AGENT.post(&url).header("Title", &title).header("Priority", prio).header("Tags", tags).send(&body);
    });
}

fn endpoint(server: &str, topic: &str) -> String {
    format!("{server}/{topic}")
}

fn mark(kind: &str) -> (&'static str, &'static str) {
    match kind {
        "ci_red" => ("high", "rotating_light"),
        "limit" => ("high", "hourglass"),
        "disk" => ("high", "floppy_disk"),
        "bug" => ("high", "beetle"),
        "waiting" => ("default", "hourglass_flowing_sand"),
        "ci_green" => ("default", "white_check_mark"),
        "resumed" => ("default", "arrow_forward"),
        _ => ("default", "robot"),
    }
}

fn ascii(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            'ä' => out.push_str("ae"),
            'ö' => out.push_str("oe"),
            'ü' => out.push_str("ue"),
            'Ä' => out.push_str("Ae"),
            'Ö' => out.push_str("Oe"),
            'Ü' => out.push_str("Ue"),
            'ß' => out.push_str("ss"),
            c if c.is_ascii() => out.push(c),
            _ => {}
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_url_priority_and_folds_the_title() {
        assert_eq!(endpoint("https://ntfy.sh", "my-secret"), "https://ntfy.sh/my-secret");
        assert_eq!(mark("ci_red"), ("high", "rotating_light"));
        assert_eq!(mark("done"), ("default", "robot"));
        assert_eq!(ascii("Build grün für Größe"), "Build gruen fuer Groesse");
        assert_eq!(ascii("plain ascii 123"), "plain ascii 123");
    }

    #[test]
    fn off_or_blank_topic_sends_nothing() {
        let home = std::env::temp_dir().join(format!("wisply-push-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        std::fs::create_dir_all(&home).unwrap();
        let e = Event { seq: 1, kind: "done".into(), agent_id: None, title: "x".into(), body: "y".into(), at: 0 };
        send(&home, &e);
        config::command(&home, &["push".into(), "ntfy".into()]).unwrap();
        send(&home, &e);
        let _ = std::fs::remove_dir_all(&home);
    }
}
