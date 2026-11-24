pub mod get;
pub mod plan;
mod run;
pub mod state;
mod sys;

use std::path::Path;

pub use run::{ensure, release, start, status, stop_all, wipe};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Llm,
    Asr,
}

impl Kind {
    pub fn label(self) -> &'static str {
        match self {
            Kind::Llm => "llm",
            Kind::Asr => "asr",
        }
    }

    pub fn parse(word: &str) -> Option<Kind> {
        match word.to_lowercase().as_str() {
            "llm" => Some(Kind::Llm),
            "asr" => Some(Kind::Asr),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Why {
    NotInstalled,
    Gaming,
    Failed(String),
}

impl Why {
    pub fn line(&self) -> String {
        match self {
            Why::NotInstalled => "local: not installed".to_string(),
            Why::Gaming => "local: paused while a game runs".to_string(),
            Why::Failed(text) => format!("local: {text}"),
        }
    }

    pub fn code(&self) -> i32 {
        match self {
            Why::Failed(_) => 1,
            Why::NotInstalled => 2,
            Why::Gaming => 3,
        }
    }

    pub fn from_reply(code: i32, out: &str) -> Why {
        match code {
            2 => Why::NotInstalled,
            3 => Why::Gaming,
            _ => Why::Failed(out.trim().trim_start_matches("local: ").to_string()),
        }
    }
}

pub fn url_line(home: &Path, args: &[String]) -> (i32, String) {
    let Some(kind) = args.first().and_then(|a| Kind::parse(a)) else {
        return (1, "local: say asr or llm".to_string());
    };
    if args.get(1).is_some_and(|a| a == "stop") {
        release(home, kind);
        return (0, "ok".to_string());
    }
    match ensure(home, kind) {
        Ok(url) => (0, url),
        Err(why) => (why.code(), why.line()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_reason_survives_the_trip_through_the_daemon_reply() {
        for why in [Why::NotInstalled, Why::Gaming, Why::Failed("did not answer".into())] {
            assert_eq!(Why::from_reply(why.code(), &why.line()), why);
        }
    }

    #[test]
    fn kinds_parse_by_name() {
        assert_eq!(Kind::parse("ASR"), Some(Kind::Asr));
        assert_eq!(Kind::parse("llm"), Some(Kind::Llm));
        assert_eq!(Kind::parse("x"), None);
    }
}
