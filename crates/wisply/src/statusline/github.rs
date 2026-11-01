use std::path::Path;
use std::process::Command;
use std::time::Duration;

use serde_json::{Value, json};

use super::proc::capture;

const LIMIT: Duration = Duration::from_secs(25);
const NO_PR: &str = "no pull requests found";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum State {
    Open,
    Draft,
    Merged,
    Closed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Checks {
    None,
    Pass,
    Fail,
    Pending,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pr {
    pub number: u64,
    pub state: State,
    pub checks: Checks,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Outcome {
    Found(Pr),
    NoPr,
    Missing,
    Failed,
}

impl State {
    pub fn word(self) -> &'static str {
        match self {
            State::Open => "open",
            State::Draft => "draft",
            State::Merged => "merged",
            State::Closed => "closed",
        }
    }

    fn from_word(word: &str) -> Option<Self> {
        [State::Open, State::Draft, State::Merged, State::Closed].into_iter().find(|s| s.word() == word)
    }
}

impl Checks {
    fn word(self) -> &'static str {
        match self {
            Checks::None => "none",
            Checks::Pass => "pass",
            Checks::Fail => "fail",
            Checks::Pending => "pending",
        }
    }

    fn from_word(word: &str) -> Option<Self> {
        [Checks::None, Checks::Pass, Checks::Fail, Checks::Pending].into_iter().find(|c| c.word() == word)
    }
}

impl Pr {
    pub fn to_value(self) -> Value {
        json!({"number": self.number, "state": self.state.word(), "checks": self.checks.word()})
    }

    pub fn from_value(v: &Value) -> Option<Self> {
        Some(Pr {
            number: v.get("number")?.as_u64()?,
            state: State::from_word(v.get("state")?.as_str()?)?,
            checks: Checks::from_word(v.get("checks")?.as_str()?)?,
        })
    }
}

fn judge(item: &Value) -> Checks {
    let text = |key: &str| item.get(key).and_then(Value::as_str).unwrap_or("").to_ascii_uppercase();
    if item.get("conclusion").is_some() || item.get("status").is_some() {
        if text("status") != "COMPLETED" {
            return Checks::Pending;
        }
        return match text("conclusion").as_str() {
            "SUCCESS" | "NEUTRAL" | "SKIPPED" => Checks::Pass,
            _ => Checks::Fail,
        };
    }
    match text("state").as_str() {
        "SUCCESS" => Checks::Pass,
        "PENDING" | "EXPECTED" => Checks::Pending,
        "" => Checks::None,
        _ => Checks::Fail,
    }
}

fn combine(items: &[Value]) -> Checks {
    let all: Vec<Checks> = items.iter().map(judge).collect();
    if all.contains(&Checks::Fail) {
        Checks::Fail
    } else if all.contains(&Checks::Pending) {
        Checks::Pending
    } else if all.contains(&Checks::Pass) {
        Checks::Pass
    } else {
        Checks::None
    }
}

pub fn parse(text: &str) -> Option<Pr> {
    let v: Value = serde_json::from_str(text).ok()?;
    let number = v.get("number")?.as_u64()?;
    let draft = v.get("isDraft").and_then(Value::as_bool).unwrap_or(false);
    let state = match v.get("state")?.as_str()?.to_ascii_uppercase().as_str() {
        "OPEN" if draft => State::Draft,
        "OPEN" => State::Open,
        "MERGED" => State::Merged,
        "CLOSED" => State::Closed,
        _ => return None,
    };
    let items = v.get("statusCheckRollup").and_then(Value::as_array).map(Vec::as_slice).unwrap_or(&[]);
    Some(Pr { number, state, checks: combine(items) })
}

pub fn run(root: &Path) -> Outcome {
    let mut cmd = Command::new("gh");
    cmd.args(["pr", "view", "--json", "number,state,isDraft,statusCheckRollup"])
        .current_dir(root)
        .env("GH_PROMPT_DISABLED", "1")
        .env("NO_COLOR", "1");
    let Some(done) = capture(&mut cmd, LIMIT) else { return Outcome::Missing };
    if done.ok {
        return parse(&done.out).map_or(Outcome::Failed, Outcome::Found);
    }
    if done.err.to_ascii_lowercase().contains(NO_PR) { Outcome::NoPr } else { Outcome::Failed }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pr(text: &str) -> Pr {
        parse(text).unwrap()
    }

    #[test]
    fn states_and_checks_are_read_from_gh_json() {
        let pass = r#"{"number":41,"state":"OPEN","isDraft":false,"statusCheckRollup":[
            {"__typename":"CheckRun","status":"COMPLETED","conclusion":"SUCCESS"},
            {"__typename":"CheckRun","status":"COMPLETED","conclusion":"SKIPPED"},
            {"__typename":"StatusContext","state":"SUCCESS"}]}"#;
        assert_eq!(pr(pass), Pr { number: 41, state: State::Open, checks: Checks::Pass });
        let fail = r#"{"number":2,"state":"OPEN","isDraft":false,"statusCheckRollup":[
            {"status":"COMPLETED","conclusion":"SUCCESS"},{"status":"COMPLETED","conclusion":"FAILURE"},
            {"status":"IN_PROGRESS","conclusion":""}]}"#;
        assert_eq!(pr(fail).checks, Checks::Fail);
        let wait = r#"{"number":3,"state":"OPEN","isDraft":false,"statusCheckRollup":[
            {"status":"COMPLETED","conclusion":"SUCCESS"},{"status":"QUEUED","conclusion":""}]}"#;
        assert_eq!(pr(wait).checks, Checks::Pending);
        let ctx = r#"{"number":4,"state":"OPEN","isDraft":false,"statusCheckRollup":[{"state":"PENDING"}]}"#;
        assert_eq!(pr(ctx).checks, Checks::Pending);
        let draft = r#"{"number":5,"state":"OPEN","isDraft":true,"statusCheckRollup":[]}"#;
        assert_eq!(pr(draft), Pr { number: 5, state: State::Draft, checks: Checks::None });
        let merged = r#"{"number":6,"state":"MERGED","isDraft":false}"#;
        assert_eq!(pr(merged), Pr { number: 6, state: State::Merged, checks: Checks::None });
        assert_eq!(pr(r#"{"number":7,"state":"CLOSED"}"#).state, State::Closed);
        assert!(parse("not json").is_none() && parse(r#"{"number":1,"state":"WEIRD"}"#).is_none());
    }

    #[test]
    fn a_pr_survives_the_cache_round_trip() {
        let p = Pr { number: 9, state: State::Draft, checks: Checks::Fail };
        assert_eq!(Pr::from_value(&p.to_value()), Some(p));
        assert_eq!(Pr::from_value(&json!({"number": 1})), None);
    }
}
