use rusqlite::ErrorCode;

const DAMAGED_HINT: &str =
    "it only holds cached results and counters: delete video.db in the pupil folder and try again";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Source,
    Time,
    Input,
    Unavailable,
    MissingKey,
    KeyRejected,
    Limit,
    Service,
    Rejected,
    ToolMissing,
    Storage,
}

impl Kind {
    pub fn code(self) -> &'static str {
        match self {
            Kind::Source => "invalid_source",
            Kind::Time => "invalid_time",
            Kind::Input => "invalid_input",
            Kind::Unavailable => "video_unavailable",
            Kind::MissingKey => "missing_key",
            Kind::KeyRejected => "key_rejected",
            Kind::Limit => "limit_reached",
            Kind::Service => "service_error",
            Kind::Rejected => "request_rejected",
            Kind::ToolMissing => "tool_missing",
            Kind::Storage => "storage",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fail {
    pub kind: Kind,
    pub message: String,
    pub hint: String,
    pub status: Option<u16>,
}

impl Fail {
    pub fn new(kind: Kind, message: impl Into<String>) -> Fail {
        Fail { kind, message: message.into(), hint: String::new(), status: None }
    }

    pub fn hint(kind: Kind, message: impl Into<String>, hint: impl Into<String>) -> Fail {
        Fail { kind, message: message.into(), hint: hint.into(), status: None }
    }

    pub fn text(&self) -> String {
        wisply_core::Fail::from(self.clone()).text()
    }
}

impl From<rusqlite::Error> for Fail {
    fn from(error: rusqlite::Error) -> Fail {
        match error.sqlite_error_code() {
            Some(ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked) => {
                Fail::hint(Kind::Storage, "the video cache is busy", "another wisply is writing to it: try again")
            }
            _ => Fail::hint(Kind::Storage, format!("the video cache cannot be read ({error})"), DAMAGED_HINT),
        }
    }
}

impl From<serde_json::Error> for Fail {
    fn from(error: serde_json::Error) -> Fail {
        Fail::hint(Kind::Storage, format!("the video cache holds an unreadable entry ({error})"), DAMAGED_HINT)
    }
}

impl From<Fail> for wisply_core::Fail {
    fn from(fail: Fail) -> wisply_core::Fail {
        wisply_core::Fail::hint(fail.message, fail.hint)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_failure_reads_like_every_other_wisply_error() {
        let plain = Fail::new(Kind::Storage, "the video cache cannot be opened");
        assert_eq!(plain.text(), "err: the video cache cannot be opened");
        let hinted = Fail::hint(Kind::Source, "no video was given", "give a YouTube link");
        assert_eq!(hinted.text(), "err: no video was given -> try: give a YouTube link");
        assert_eq!(hinted.kind.code(), "invalid_source");
    }
}
