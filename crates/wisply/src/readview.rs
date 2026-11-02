use regex::Regex;
use std::sync::LazyLock;

const HEAD: usize = 40;
const TAIL: usize = 15;
const OUTLINE_MAX: usize = 60;
const OUTLINE_MIN: usize = 3;
const LINE_MAX: usize = 110;
const CUT_MAX_SHARE: usize = 60;
pub const BIG_LINES: usize = 150;

static OUTLINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"^\s*(?:(?:pub(?:\([^)]*\))?|public|private|protected|static|final|abstract|async|export|default|override|internal|open|data|sealed)\s+)*",
        r"(?:fn|struct|enum|trait|impl|mod|class|interface|def|function|func|type|object|record|namespace)\b",
        r"|^\s*(?:(?:public|private|protected|static|final|abstract|synchronized)\s+)+[\w<>\[\],.? ]+\s+\w+\s*\(",
        r"|^#{1,4}\s+\S"
    ))
    .expect("outline pattern")
});

fn clip(line: &str) -> String {
    let t = line.trim_end();
    if t.chars().count() <= LINE_MAX { t.to_string() } else { t.chars().take(LINE_MAX).collect::<String>() + "..." }
}

pub fn cut_view(content: &str, name: &str) -> Option<String> {
    let lines: Vec<&str> = content.lines().collect();
    let total = lines.len();
    if total < BIG_LINES {
        return None;
    }
    let outline: Vec<String> = lines
        .iter()
        .enumerate()
        .filter(|(i, l)| *i >= HEAD && *i + TAIL < total && OUTLINE.is_match(l))
        .take(OUTLINE_MAX)
        .map(|(i, l)| format!("L{}: {}", i + 1, clip(l.trim_start())))
        .collect();
    if outline.len() < OUTLINE_MIN {
        return None;
    }
    let mut out = format!(
        "[wisply] cut this {total}-line file (~{} tokens) to its top, outline and end. All of it: wisply more {name} (--lines a-b or --grep word), or Read again with offset and limit. Reading the whole file again returns everything.\n--- lines 1-{HEAD}\n",
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
