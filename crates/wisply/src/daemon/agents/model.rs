use serde_json::{Value, json};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Running,
    Waiting,
    Done,
    Limit,
    Error,
}

impl Status {
    pub fn word(self) -> &'static str {
        match self {
            Status::Running => "running",
            Status::Waiting => "waiting",
            Status::Done => "done",
            Status::Limit => "limit",
            Status::Error => "error",
        }
    }

    pub fn parse(word: &str) -> Option<Status> {
        [Status::Running, Status::Waiting, Status::Done, Status::Limit, Status::Error]
            .into_iter()
            .find(|s| s.word() == word)
    }
}

#[derive(Clone, Debug)]
pub struct Row {
    pub id: String,
    pub sid: String,
    pub name: String,
    pub dir: String,
    pub dir_short: String,
    pub background: bool,
    pub status: Status,
    pub since: i64,
    pub last_line: String,
    pub tokens: i64,
    pub cost: f64,
    pub model: String,
    pub effort: String,
    pub reset_at: Option<i64>,
}

impl Row {
    pub fn json(&self) -> Value {
        let mut v = json!({
            "id": self.id,
            "name": self.name,
            "dir_short": self.dir_short,
            "status": self.status.word(),
            "since": self.since,
            "last_line": self.last_line,
            "tokens": self.tokens,
            "cost_est": (self.cost * 100.0).round() / 100.0,
            "model": self.model,
            "effort": self.effort,
        });
        if let Some(at) = self.reset_at {
            v["reset_at"] = json!(at);
        }
        v
    }
}

pub fn last_folder(dir: &str) -> String {
    dir.trim_end_matches(['/', '\\']).rsplit(['/', '\\']).next().unwrap_or("").to_string()
}

pub fn status_of(state: &str, busy: Option<&str>, limit: bool) -> Status {
    match state {
        "working" => Status::Running,
        _ if busy == Some("busy") && state.is_empty() => Status::Running,
        "blocked" => Status::Waiting,
        "failed" => Status::Error,
        _ if limit => Status::Limit,
        _ => Status::Done,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn states_map_to_the_five_words() {
        assert_eq!(status_of("working", Some("busy"), false), Status::Running);
        assert_eq!(status_of("", Some("busy"), false), Status::Running);
        assert_eq!(status_of("blocked", None, false), Status::Waiting);
        assert_eq!(status_of("failed", None, false), Status::Error);
        assert_eq!(status_of("stopped", None, true), Status::Limit);
        assert_eq!(status_of("done", Some("idle"), false), Status::Done);
        assert_eq!(status_of("stopped", None, false), Status::Done);
        for s in [Status::Running, Status::Waiting, Status::Done, Status::Limit, Status::Error] {
            assert_eq!(Status::parse(s.word()), Some(s));
        }
    }

    #[test]
    fn only_the_last_folder_is_shown() {
        assert_eq!(last_folder(r"C:\a\b\proj"), "proj");
        assert_eq!(last_folder("/x/y/"), "y");
        assert_eq!(last_folder(""), "");
    }
}
