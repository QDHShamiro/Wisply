use std::path::Path;
use std::process::ExitCode;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use wisply_core::Fail;

use crate::daemon::local::{self, Kind, Why, get};

const HELP: &str = "wisply local [status] (what is installed and running)
wisply local get [--only llm|asr] (loads the small models, about 750 MB, checked by sha256)
wisply local stop (ends both programs) | rm [--yes] (deletes everything loaded)";
const MODEL_NAME: &str = "local";
const ANSWER_TOKENS: i64 = 120;
const SCHEMA_NAME: &str = "answer";
const MIN_WAIT: Duration = Duration::from_millis(500);

pub fn run(home: &Path, args: &[String]) -> ExitCode {
    match dispatch(home, args) {
        Ok(out) => {
            if !out.is_empty() {
                println!("{out}");
            }
            ExitCode::SUCCESS
        }
        Err(fail) => {
            println!("{}", fail.text());
            ExitCode::from(1)
        }
    }
}

fn only(args: &[String]) -> Result<Option<Kind>, Fail> {
    let Some(at) = args.iter().position(|a| a == "--only") else { return Ok(None) };
    args.get(at + 1)
        .and_then(|w| Kind::parse(w))
        .map(Some)
        .ok_or_else(|| Fail::hint("--only takes llm or asr", "wisply local get --only llm"))
}

fn dispatch(home: &Path, args: &[String]) -> Result<String, Fail> {
    let (verb, rest) = args.split_first().map_or(("status", &[][..]), |(v, r)| (v.as_str(), r));
    match verb {
        "status" => Ok(local::status(home)),
        "get" => {
            let kind = only(rest)?;
            let mut say = |line: String| println!("{line}");
            get::run(home, kind, &mut say).map_err(|e| Fail::hint(e, "wisply local status"))?;
            Ok("local: ready".to_string())
        }
        "stop" => {
            let n = local::stop_all(home);
            Ok(if n == 0 { "local: nothing was running".to_string() } else { format!("local: stopped {n}") })
        }
        "rm" => local::wipe(home, rest.iter().any(|a| a == "--yes")).map_err(Fail::new),
        "help" | "-h" | "--help" => Ok(HELP.to_string()),
        other => Err(Fail::hint(format!("unknown local command '{other}'"), "wisply local help")),
    }
}

pub fn ensure(home: &Path, kind: Kind) -> Result<String, Why> {
    match crate::client::ask(home, &["local-url", kind.label()]) {
        Ok((0, url)) => Ok(url.trim().to_string()),
        Ok((code, out)) => Err(Why::from_reply(code, &out)),
        Err(e) => Err(Why::Failed(e)),
    }
}

pub fn release(home: &Path, kind: Kind) {
    let _ = crate::client::ask(home, &["local-url", kind.label(), "stop"]);
}

pub fn body(system: &str, user: &str, schema: &Value) -> Value {
    json!({
        "model": MODEL_NAME,
        "messages": [
            {"role": "system", "content": system},
            {"role": "user", "content": user},
        ],
        "temperature": 0,
        "max_tokens": ANSWER_TOKENS,
        "stream": false,
        "response_format": {
            "type": "json_schema",
            "json_schema": {"name": SCHEMA_NAME, "strict": true, "schema": schema},
        },
    })
}

pub fn parse_reply(reply: &Value) -> Result<Value, String> {
    let text = reply.pointer("/choices/0/message/content").and_then(Value::as_str).ok_or("the model sent no answer")?;
    serde_json::from_str(text.trim()).map_err(|e| format!("the model answer is not JSON ({e})"))
}

pub fn ask_json(home: &Path, system: &str, user: &str, schema: &Value, timeout: Duration) -> Result<Value, String> {
    let began = Instant::now();
    let (send, recv) = mpsc::channel();
    let owner = home.to_path_buf();
    let _ = thread::Builder::new().name("local-ensure".into()).spawn(move || {
        let _ = send.send(ensure(&owner, Kind::Llm));
    });
    let url = match recv.recv_timeout(timeout) {
        Ok(Ok(url)) => url,
        Ok(Err(why)) => return Err(why.line()),
        Err(_) => return Err("local: the model is still starting".to_string()),
    };
    let left = timeout.saturating_sub(began.elapsed()).max(MIN_WAIT);
    let payload = body(system, user, schema).to_string();
    let mut resp = ureq::post(&format!("{url}/v1/chat/completions"))
        .config()
        .timeout_global(Some(left))
        .build()
        .header("content-type", "application/json")
        .send(payload.as_bytes())
        .map_err(|e| format!("local: the model did not answer ({e})"))?;
    let reply: Value = resp.body_mut().read_json().map_err(|e| format!("local: bad model reply ({e})"))?;
    parse_reply(&reply)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_request_asks_for_a_schema_bound_answer_without_randomness() {
        let schema = json!({"type": "object"});
        let b = body("rules", "question", &schema);
        assert_eq!(b["temperature"], 0);
        assert_eq!(b["response_format"]["type"], "json_schema");
        assert_eq!(b["response_format"]["json_schema"]["schema"], schema);
        assert_eq!(b["messages"][0]["role"], "system");
        assert_eq!(b["messages"][1]["content"], "question");
    }

    #[test]
    fn the_reply_text_is_read_as_json() {
        let ok = json!({"choices": [{"message": {"content": " {\"choice\": \"ja\", \"note\": \"\"} "}}]});
        assert_eq!(parse_reply(&ok).unwrap()["choice"], "ja");
        assert!(parse_reply(&json!({"choices": []})).is_err());
        let bad = json!({"choices": [{"message": {"content": "ja"}}]});
        assert!(parse_reply(&bad).unwrap_err().contains("not JSON"));
    }

    #[test]
    fn local_commands_are_checked() {
        let home = std::env::temp_dir().join(format!("wisply-local-cli-{}", std::process::id()));
        let args = |w: &[&str]| w.iter().map(|a| (*a).to_string()).collect::<Vec<_>>();
        assert!(dispatch(&home, &args(&["nope"])).unwrap_err().text().contains("unknown local command"));
        assert!(dispatch(&home, &args(&["get", "--only", "x"])).unwrap_err().text().contains("llm or asr"));
        assert!(dispatch(&home, &args(&["help"])).unwrap().contains("wisply local get"));
        assert!(dispatch(&home, &args(&["rm"])).unwrap().contains("nothing to delete"));
    }
}
