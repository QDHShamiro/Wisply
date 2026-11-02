use std::path::PathBuf;
use std::time::Instant;

use serde_json::{Value, json};
use wisply_core::Fail;

use super::auth::{Auth, PairError, Verdict};
use super::board::safe_id;
use super::extra;
use super::model::{Row, Status};

pub const BODY_MAX: usize = 16 * 1024;
const REPLY_MAX: usize = 4000;

pub trait Backend: Send + Sync {
    fn rows(&self) -> Result<Vec<Row>, Fail>;
    fn tail(&self, row: &Row) -> Vec<String>;
    fn reply(&self, row: &Row, text: &str) -> Result<(), Fail>;
    fn stop(&self, row: &Row) -> Result<(), Fail>;
    fn status(&self) -> Value;
}

pub struct Api {
    pub home: PathBuf,
    pub auth: Auth,
    pub backend: Box<dyn Backend>,
}

pub enum Reply {
    Json(u16, Value),
    Stream(u64),
}

fn fail(code: u16, word: &str) -> Reply {
    Reply::Json(code, json!({"error": word}))
}

fn denied() -> Reply {
    fail(401, "unauthorized")
}

fn query_since(query: &str, last_event_id: Option<&str>) -> u64 {
    query
        .split('&')
        .find_map(|kv| kv.strip_prefix("since="))
        .and_then(|v| v.parse().ok())
        .or_else(|| last_event_id.and_then(|v| v.trim().parse().ok()))
        .unwrap_or(0)
}

impl Api {
    pub fn handle(
        &self,
        method: &str,
        target: &str,
        auth: Option<&str>,
        last_event_id: Option<&str>,
        peer: &str,
        body: &[u8],
    ) -> Reply {
        let (path, query) = target.split_once('?').unwrap_or((target, ""));
        let now = Instant::now();
        if path == "/v1/pair" {
            return if method == "POST" { self.pair(peer, body, now) } else { denied() };
        }
        match self.auth.check(auth, now) {
            Verdict::Denied => return denied(),
            Verdict::Limited => return fail(429, "slow down"),
            Verdict::Allowed(_) => {}
        }
        let parts: Vec<&str> = path.trim_matches('/').split('/').collect();
        match (method, parts.as_slice()) {
            ("GET", ["v1", "status"]) => Reply::Json(200, self.backend.status()),
            ("GET", ["v1", "events"]) => Reply::Stream(query_since(query, last_event_id)),
            ("GET", ["v1", "agents"]) => match self.backend.rows() {
                Ok(rows) => Reply::Json(200, Value::Array(rows.iter().map(|r| self.wire(r)).collect())),
                Err(_) => fail(502, "claude is not answering"),
            },
            ("GET", ["v1", "agents", id]) => self.with_row(id, |row| {
                let mut v = self.wire(row);
                v["tail"] = json!(self.backend.tail(row));
                extra::attach(&mut v, &self.home, row);
                Reply::Json(200, v)
            }),
            ("POST", ["v1", "agents", id, "reply"]) => self.with_row(id, |row| self.reply(row, body)),
            ("POST", ["v1", "agents", id, "stop"]) => self.with_row(id, |row| match self.backend.stop(row) {
                Ok(()) => Reply::Json(202, json!({})),
                Err(_) => fail(502, "claude did not stop it"),
            }),
            _ => fail(404, "not found"),
        }
    }

    fn wire(&self, row: &Row) -> Value {
        let mut v = row.json();
        extra::flag(&mut v, &self.home, row);
        v
    }

    fn with_row(&self, id: &str, run: impl FnOnce(&Row) -> Reply) -> Reply {
        if !safe_id(id) {
            return fail(404, "not found");
        }
        match self.backend.rows() {
            Ok(rows) => rows.iter().find(|r| r.id == id).map_or_else(|| fail(404, "not found"), run),
            Err(_) => fail(502, "claude is not answering"),
        }
    }

    fn reply(&self, row: &Row, body: &[u8]) -> Reply {
        let text = serde_json::from_slice::<Value>(body)
            .ok()
            .and_then(|v| v["text"].as_str().map(|t| t.trim().to_string()))
            .filter(|t| !t.is_empty() && t.chars().count() <= REPLY_MAX);
        let Some(text) = text else { return fail(400, "text needed") };
        if !row.background || row.status == Status::Running {
            return fail(409, "not ready for a reply");
        }
        match self.backend.reply(row, &text) {
            Ok(()) => Reply::Json(202, json!({})),
            Err(_) => fail(502, "claude did not take it"),
        }
    }

    fn pair(&self, peer: &str, body: &[u8], now: Instant) -> Reply {
        let v: Value = serde_json::from_slice(body).unwrap_or(Value::Null);
        let (code, device) = (v["code"].as_str().unwrap_or(""), v["device"].as_str().unwrap_or(""));
        let created = super::board::now();
        match self.auth.redeem(code, device, peer, now, created) {
            Ok((token, name)) => Reply::Json(200, json!({"token": token, "name": name})),
            Err(PairError::Limited) => fail(429, "slow down"),
            Err(PairError::Rejected) => denied(),
            Err(PairError::Failed) => fail(500, "could not save"),
        }
    }
}

#[cfg(test)]
pub mod fake {
    use std::sync::Mutex;

    use super::*;

    #[derive(Default)]
    pub struct Fake {
        pub calls: Mutex<Vec<String>>,
    }

    pub fn row(id: &str, status: Status) -> Row {
        Row {
            id: id.into(),
            sid: format!("sid-{id}"),
            name: "job".into(),
            dir: "C:\\private\\place".into(),
            dir_short: "place".into(),
            background: true,
            status,
            since: 10,
            last_line: "ok".into(),
            tokens: 5,
            cost: 0.5,
            model: "sonnet-5-5".into(),
            effort: "high".into(),
            reset_at: None,
        }
    }

    impl Backend for std::sync::Arc<Fake> {
        fn rows(&self) -> Result<Vec<Row>, Fail> {
            Ok(vec![row("aa11", Status::Done), row("bb22", Status::Running)])
        }

        fn tail(&self, _: &Row) -> Vec<String> {
            vec!["line one".into()]
        }

        fn reply(&self, row: &Row, text: &str) -> Result<(), Fail> {
            self.calls.lock().unwrap().push(format!("reply {} {text}", row.id));
            Ok(())
        }

        fn stop(&self, row: &Row) -> Result<(), Fail> {
            self.calls.lock().unwrap().push(format!("stop {}", row.id));
            Ok(())
        }

        fn status(&self) -> Value {
            json!({"version": "t", "pc": {"free_gb_c": 12, "game_mode": false}, "limit": {"hit": false, "reset_at": null}})
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::fake::Fake;
    use super::*;

    fn api(tag: &str) -> (Api, Arc<Fake>, String, PathBuf) {
        let home = std::env::temp_dir().join(format!("wisply-agents-api-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        let fake = Arc::new(Fake::default());
        let api = Api { home: home.clone(), auth: Auth::new(&home), backend: Box::new(Arc::clone(&fake)) };
        let code = api.auth.new_code("handy", Instant::now()).unwrap();
        let body = json!({"code": code, "device": "handy"}).to_string();
        let Reply::Json(200, v) = api.handle("POST", "/v1/pair", None, None, "1.1.1.1", body.as_bytes()) else {
            panic!("pair")
        };
        let header = format!("Bearer {}", v["token"].as_str().unwrap());
        (api, fake, header, home)
    }

    fn code_of(r: Reply) -> u16 {
        match r {
            Reply::Json(c, _) => c,
            Reply::Stream(_) => 200,
        }
    }

    #[test]
    fn every_path_but_pair_needs_a_token_and_says_nothing_else() {
        let (api, _, header, home) = api("auth");
        for (m, p) in [
            ("GET", "/v1/status"),
            ("GET", "/v1/agents"),
            ("GET", "/v1/agents/aa11"),
            ("POST", "/v1/agents/aa11/stop"),
            ("GET", "/v1/events"),
            ("GET", "/nothing"),
            ("GET", "/v1/agents/zz99"),
        ] {
            for h in [None, Some("Bearer wrong")] {
                let Reply::Json(code, v) = api.handle(m, p, h, None, "p", b"") else { panic!("{p}") };
                assert_eq!((code, v), (401, json!({"error": "unauthorized"})), "{m} {p}");
            }
        }
        assert_eq!(code_of(api.handle("GET", "/v1/status", Some(&header), None, "p", b"")), 200);
        assert_eq!(code_of(api.handle("GET", "/nothing", Some(&header), None, "p", b"")), 404);
        assert_eq!(code_of(api.handle("GET", "/v1/pair", None, None, "p", b"")), 401);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn the_board_shows_only_the_contract_fields() {
        let (api, _, header, home) = api("board");
        let Reply::Json(200, v) = api.handle("GET", "/v1/agents", Some(&header), None, "p", b"") else { panic!() };
        let first = v[0].as_object().unwrap();
        let mut keys: Vec<&str> = first.keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "cost_est",
                "dir_short",
                "effort",
                "has_result",
                "id",
                "last_line",
                "model",
                "name",
                "since",
                "status",
                "tokens"
            ]
        );
        assert!(!v.to_string().contains("private"));
        let Reply::Json(200, one) = api.handle("GET", "/v1/agents/aa11", Some(&header), None, "p", b"") else {
            panic!()
        };
        assert_eq!(one["tail"], json!(["line one"]));
        assert_eq!(code_of(api.handle("GET", "/v1/agents/..%2f", Some(&header), None, "p", b"")), 404);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn reply_and_stop_reach_the_backend_and_bad_input_is_refused() {
        let (api, fake, header, home) = api("act");
        let h = Some(header.as_str());
        let go = |p: &str, b: &str| code_of(api.handle("POST", p, h, None, "p", b.as_bytes()));
        assert_eq!(go("/v1/agents/aa11/reply", r#"{"text":" weiter "}"#), 202);
        assert_eq!(go("/v1/agents/aa11/reply", r#"{"text":""}"#), 400);
        assert_eq!(go("/v1/agents/aa11/reply", "garbage"), 400);
        assert_eq!(go("/v1/agents/bb22/reply", r#"{"text":"x"}"#), 409);
        assert_eq!(go("/v1/agents/zz99/reply", r#"{"text":"x"}"#), 404);
        assert_eq!(go("/v1/agents/bb22/stop", ""), 202);
        assert_eq!(*fake.calls.lock().unwrap(), ["reply aa11 weiter", "stop bb22"]);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn events_stream_starts_from_the_asked_number() {
        let (api, _, header, home) = api("ev");
        let h = Some(header.as_str());
        let Reply::Stream(n) = api.handle("GET", "/v1/events?since=7", h, None, "p", b"") else { panic!() };
        assert_eq!(n, 7);
        let Reply::Stream(n) = api.handle("GET", "/v1/events", h, Some("9"), "p", b"") else { panic!() };
        assert_eq!(n, 9);
        let _ = std::fs::remove_dir_all(&home);
    }
}
