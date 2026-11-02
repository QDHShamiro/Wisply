use serde_json::{Value, json};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Sev {
    High,
    Medium,
    Low,
}

impl Sev {
    pub fn word(self) -> &'static str {
        match self {
            Sev::High => "high",
            Sev::Medium => "medium",
            Sev::Low => "low",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Finding {
    pub sev: Sev,
    pub kind: &'static str,
    pub file: String,
    pub line: usize,
    pub why: String,
    pub fix: String,
}

impl Finding {
    pub fn new(
        sev: Sev,
        kind: &'static str,
        file: impl Into<String>,
        line: usize,
        why: impl Into<String>,
        fix: impl Into<String>,
    ) -> Finding {
        Finding { sev, kind, file: file.into(), line, why: why.into(), fix: fix.into() }
    }

    pub fn at(&self) -> String {
        if self.line > 0 { format!("{}:{}", self.file, self.line) } else { self.file.clone() }
    }
}

pub struct Report {
    pub target: String,
    pub checked: Vec<String>,
    pub found: Vec<Finding>,
    pub notes: Vec<String>,
}

impl Report {
    pub fn new(target: impl Into<String>) -> Report {
        Report { target: target.into(), checked: Vec::new(), found: Vec::new(), notes: Vec::new() }
    }

    pub fn count(&self, sev: Sev) -> usize {
        self.found.iter().filter(|f| f.sev == sev).count()
    }

    pub fn sorted(&mut self) {
        self.found.sort_by(|a, b| (a.sev, a.kind, &a.file, a.line).cmp(&(b.sev, b.kind, &b.file, b.line)));
        self.found.dedup_by(|a, b| a.kind == b.kind && a.file == b.file && a.line == b.line && a.why == b.why);
    }

    pub fn summary(&self) -> String {
        let parts: Vec<String> = [Sev::High, Sev::Medium, Sev::Low]
            .into_iter()
            .filter(|s| self.count(*s) > 0)
            .map(|s| format!("{} {}", self.count(s), s.word()))
            .collect();
        if parts.is_empty() { "no findings".into() } else { parts.join(", ") }
    }

    pub fn text(&self) -> String {
        let checked =
            if self.checked.is_empty() { String::new() } else { format!(" (checked: {})", self.checked.join(", ")) };
        let mut lines = vec![format!("[secure] {}: {}{checked}", self.target, self.summary())];
        for f in &self.found {
            lines.push(format!("{:<6} {} {} -> {}", f.sev.word(), f.at(), f.why, f.fix));
        }
        lines.extend(self.notes.iter().map(|n| format!("! {n}")));
        lines.join("\n")
    }

    pub fn json(&self) -> String {
        let found: Vec<Value> = self
            .found
            .iter()
            .map(|f| {
                json!({"severity": f.sev.word(), "kind": f.kind, "file": f.file, "line": f.line, "why": f.why, "fix": f.fix})
            })
            .collect();
        let counts =
            json!({"high": self.count(Sev::High), "medium": self.count(Sev::Medium), "low": self.count(Sev::Low)});
        serde_json::to_string_pretty(&json!({
            "target": self.target, "checked": self.checked, "counts": counts, "findings": found, "notes": self.notes
        }))
        .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_and_json_list_the_worst_first() {
        let mut r = Report::new("repo");
        r.checked.push("3 files".into());
        r.found.push(Finding::new(Sev::Low, "code", "b.py", 4, "debug on", "turn it off"));
        r.found.push(Finding::new(Sev::High, "secret", "a.yml", 1, "key", "rotate"));
        r.sorted();
        let text = r.text();
        assert_eq!(
            text,
            "[secure] repo: 1 high, 1 low (checked: 3 files)\nhigh   a.yml:1 key -> rotate\nlow    b.py:4 debug on -> turn it off"
        );
        let v: Value = serde_json::from_str(&r.json()).unwrap();
        assert_eq!(v["counts"]["high"], 1);
        assert_eq!(v["findings"][0]["file"], "a.yml");
    }
}
