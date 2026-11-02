use regex::Regex;
use std::sync::LazyLock;

const HEAD: usize = 30;
const TAIL: usize = 10;
const OUTLINE_MAX: usize = 60;
const OUTLINE_MIN: usize = 3;
const RUN_MIN: usize = 3;
const LINE_MAX: usize = 100;
const CUT_MAX_SHARE: usize = 60;
pub const BIG_LINES: usize = 150;

static OUTLINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^\s*(?:(?:pub(?:\([^)]*\))?|public|private|protected|static|final|abstract|async|export|default|override|internal|open|data|sealed)\s+)*",
        r"(?:fn|struct|enum|trait|impl|mod|class|interface|def|function|func|type|object|record|namespace)\b",
        r"|^\s*(?:(?:public|private|protected|static|final|abstract|synchronized)\s+)+[\w<>\[\],.? ]+\s+\w+\s*\(",
        r"|^\s*(?:(?:pub(?:\([^)]*\))?|export|public|private|protected|static|final|const|let|var|val|readonly)\s+)*(?:[\w<>\[\],.?]+\s+)?[A-Z][A-Z0-9_]{2,}\s*(?::[^=]{0,40})?=[^=]",
        r"|^#{1,4}\s+\S"
    ))
    .expect("outline pattern")
});

fn clip(line: &str) -> String {
    let t = line.trim_end();
    if t.chars().count() <= LINE_MAX { t.to_string() } else { t.chars().take(LINE_MAX).collect::<String>() + "..." }
}

fn shape(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    for c in line.chars() {
        if !c.is_ascii_digit() {
            out.push(c);
        } else if !out.ends_with('#') {
            out.push('#');
        }
    }
    out
}

fn outline(lines: &[&str], total: usize) -> Vec<String> {
    let hits: Vec<(usize, &str)> = lines
        .iter()
        .enumerate()
        .filter(|(i, l)| *i >= HEAD && *i + TAIL < total && OUTLINE.is_match(l))
        .map(|(i, l)| (i + 1, l.trim()))
        .collect();
    let mut out = Vec::new();
    let mut k = 0;
    while k < hits.len() {
        let key = shape(hits[k].1);
        let mut end = k + 1;
        while end < hits.len() && shape(hits[end].1) == key {
            end += 1;
        }
        let run = &hits[k..end];
        if run.len() >= RUN_MIN {
            let step = run[1].0 - run[0].0;
            let even = run.windows(2).all(|w| w[1].0 - w[0].0 == step);
            let every = if even { format!(", every {step} lines") } else { String::new() };
            out.push(format!("L{}-L{} {}x{every}: {}", run[0].0, run[run.len() - 1].0, run.len(), clip(run[0].1)));
        } else {
            out.extend(run.iter().map(|(n, l)| format!("L{n}: {}", clip(l))));
        }
        k = end;
    }
    out.truncate(OUTLINE_MAX);
    out
}

pub fn cut_view(content: &str, name: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let total = lines.len();
    if total < BIG_LINES {
        return None;
    }
    let outline = outline(&lines, total);
    if outline.len() < OUTLINE_MIN && !outline.iter().any(|o| o.split(':').next().is_some_and(|h| h.contains('-'))) {
        return None;
    }
    let mut out = format!(
        "[wisply] {total} lines (~{} tokens) cut to top, outline and end. For one part: Read with offset/limit or Grep, line numbers below. Whole text: Read it again, or wisply more {name}.\n--- lines 1-{HEAD}\n",
        content.len() / 4
    );
    for (i, l) in lines.iter().take(HEAD).enumerate() {
        out.push_str(&format!("L{}: {}\n", i + 1, clip(l)));
    }
    out.push_str(&format!("--- outline of lines {}-{}\n", HEAD + 1, total - TAIL));
    for o in &outline {
        out.push_str(o);
        out.push('\n');
    }
    out.push_str(&format!("--- lines {}-{total}\n", total - TAIL + 1));
    for (i, l) in lines.iter().enumerate().skip(total - TAIL) {
        out.push_str(&format!("L{}: {}\n", i + 1, clip(l)));
    }
    (out.len() * 100 < content.len() * CUT_MAX_SHARE).then_some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file() -> String {
        let mut l: Vec<String> = vec!["import os".into(), "class Inventory:".into()];
        for i in 1..60 {
            l.push(format!("    def handler_number_{i}(self, incoming_value, scale=1):"));
            l.push(format!("        total = incoming_value * {i} * scale + {i}"));
            l.push(format!("        return total if total > 0 else {i}"));
            l.push(String::new());
        }
        l[140] = "    MAGIC_LIMIT = 7349".into();
        l.push("def main():".into());
        l.join("\n")
    }

    #[test]
    fn constants_show_and_look_alike_lines_fold_into_one() {
        let body = file();
        let view = cut_view(&body, "hr-1").unwrap();
        assert!(view.contains("L141: MAGIC_LIMIT = 7349"), "{view}");
        assert!(view.contains("x, every 4 lines: def handler_number_"), "{view}");
        assert!(view.lines().filter(|l| l.contains("def handler_number_")).count() < 12, "{view}");
        assert!(view.len() * 3 < body.len(), "{} vs {}", view.len(), body.len());
    }

    #[test]
    fn hint_names_free_tools_before_wisply_more() {
        let view = cut_view(&file(), "hr-9").unwrap();
        let first = view.lines().next().unwrap();
        assert!(first.find("offset").unwrap() < first.find("wisply more").unwrap(), "{first}");
    }

    #[test]
    fn comparisons_are_not_taken_for_constants() {
        assert!(!OUTLINE.is_match("    if MAX_SIZE == other {"));
        assert!(OUTLINE.is_match("pub const MAX_SIZE: usize = 4;"));
        assert!(OUTLINE.is_match("    private static final int LIMIT = 5;"));
        assert!(OUTLINE.is_match("export const API_URL = 'x';"));
    }
}
