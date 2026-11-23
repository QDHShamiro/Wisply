use std::fs;
use std::mem;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

pub const FRESH_S: f64 = 900.0;
pub const ARG_SEP: char = '\u{1f}';
const NAMES: [&str; 3] = ["wisply", "lighting", "wisply-full"];
const CHEAP: [&str; 2] = ["wisply", "lighting"];
const STATS: &str = "--stats";
const ENV: &str = "--env";

pub struct Entry {
    pub argv: Vec<String>,
    pub err: String,
    pub lesson: String,
    pub at: f64,
}

pub fn now() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0.0, |d| d.as_secs_f64())
}

pub fn file(root: &Path, sid: &str) -> PathBuf {
    root.join("fails").join(format!("{:016x}", crate::hook::fnv(&[sid])))
}

pub fn fix_text(argv: &[String]) -> String {
    let quote = |a: &String| {
        if a.is_empty() || a.chars().any(char::is_whitespace) {
            format!("\"{}\"", a.replace('"', "'"))
        } else {
            a.clone()
        }
    };
    argv.iter().map(quote).collect::<Vec<_>>().join(" ")
}

pub fn parse(text: &str) -> Vec<Entry> {
    let entry = |line: &str| {
        let mut parts = line.splitn(4, '\t');
        let at = parts.next()?.parse().ok()?;
        let err = parts.next()?.to_string();
        let lesson = parts.next()?.to_string();
        let argv = parts.next()?.split(ARG_SEP).map(String::from).collect();
        Some(Entry { argv, err, lesson, at })
    };
    text.lines().filter_map(entry).collect()
}

pub fn hint(root: &Path, event: &Value) -> Option<String> {
    if event["hook_event_name"].as_str() != Some("PreToolUse") || event["tool_name"].as_str() != Some("Bash") {
        return None;
    }
    let cmd = event["tool_input"]["command"].as_str().filter(|c| CHEAP.iter().any(|n| c.contains(n)))?;
    let sid = event["session_id"].as_str().filter(|s| !s.is_empty())?;
    let entries = parse(&fs::read_to_string(file(root, sid)).ok()?);
    let (asked, t) = (calls(cmd), now());
    let hit = entries.iter().rev().find(|e| t - e.at <= FRESH_S && asked.contains(&e.argv))?;
    let lesson = if hit.lesson.is_empty() {
        String::new()
    } else {
        format!(" After this error `wisply {}` worked here before.", hit.lesson)
    };
    let text = format!(
        "wisply: `wisply {}` already failed in this session: {}.{lesson} Change the command instead of running it again unchanged.",
        fix_text(&hit.argv),
        hit.err.trim_end_matches('.')
    );
    Some(json!({"hookSpecificOutput": {"hookEventName": "PreToolUse", "additionalContext": text}}).to_string())
}

pub fn calls(cmd: &str) -> Vec<Vec<String>> {
    let argv = |seg: Vec<String>| {
        let at = seg.iter().position(|w| own(w))?;
        let mut rest = seg.into_iter().skip(at + 1);
        let mut argv = Vec::new();
        while let Some(a) = rest.next() {
            match a.as_str() {
                STATS => {}
                ENV => {
                    rest.next();
                }
                _ => argv.push(a),
            }
        }
        (!argv.is_empty()).then_some(argv)
    };
    segments(cmd).into_iter().filter_map(argv).collect()
}

fn own(word: &str) -> bool {
    let base = word.rsplit(['/', '\\']).next().unwrap_or(word).to_lowercase();
    NAMES.contains(&base.strip_suffix(".exe").unwrap_or(&base))
}

#[derive(Default)]
struct Words {
    segs: Vec<Vec<String>>,
    cur: Vec<String>,
    word: String,
    open: bool,
    skip: bool,
}

impl Words {
    fn end_word(&mut self) {
        if mem::take(&mut self.open) {
            let w = mem::take(&mut self.word);
            if !mem::take(&mut self.skip) {
                self.cur.push(w);
            }
        }
    }

    fn end_seg(&mut self) {
        self.end_word();
        self.skip = false;
        if !self.cur.is_empty() {
            self.segs.push(mem::take(&mut self.cur));
        }
    }

    fn redirect(&mut self) {
        if self.open && self.word.bytes().all(|b| b.is_ascii_digit()) {
            self.word.clear();
            self.open = false;
        }
        self.end_word();
        self.skip = true;
    }
}

fn segments(cmd: &str) -> Vec<Vec<String>> {
    let mut w = Words::default();
    let mut chars = cmd.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\'' => {
                w.open = true;
                w.word.extend(chars.by_ref().take_while(|&q| q != '\''));
            }
            '"' => {
                w.open = true;
                while let Some(q) = chars.next() {
                    match q {
                        '"' => break,
                        '\\' if matches!(chars.peek(), Some('"' | '\\' | '$' | '`')) => w.word.extend(chars.next()),
                        _ => w.word.push(q),
                    }
                }
            }
            '\\' => match chars.next() {
                Some('\n') | None => {}
                Some(n) => {
                    w.open = true;
                    w.word.push(n);
                }
            },
            ' ' | '\t' | '\r' => w.end_word(),
            '\n' | ';' | '|' | '&' | '(' | ')' | '`' => w.end_seg(),
            '>' | '<' => {
                w.redirect();
                while chars.next_if(|q| matches!(q, '>' | '<' | '&' | '|')).is_some() {}
            }
            '#' if !w.open => {
                w.end_seg();
                chars.by_ref().find(|&q| q == '\n');
            }
            _ => {
                w.open = true;
                w.word.push(c);
            }
        }
    }
    w.end_seg();
    w.segs
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(words: &[&str]) -> Vec<String> {
        words.iter().map(|w| (*w).to_string()).collect()
    }

    #[test]
    fn finds_every_wisply_call_in_a_shell_line() {
        let line = r#"cd /e/x && rtk wisply click "Log in" 2>&1 | tail -3; L=1 ~/.wisply/bin/wisply.exe type e4 'a "b"' --stats > out.txt"#;
        assert_eq!(calls(line), vec![v(&["click", "Log in"]), v(&["type", "e4", "a \"b\""])]);
        assert_eq!(
            calls("wisply --env PW type e2 x # wisply snap\nwisply snap"),
            vec![v(&["type", "e2", "x"]), v(&["snap"])]
        );
        assert_eq!(calls(r#"echo "wisply" ; wisply press Enter\ Key"#), vec![v(&["press", "Enter Key"])]);
        assert!(calls("wisply").is_empty() && calls("git status").is_empty());
        assert_eq!(
            calls("x=$(wisply text -f price) && Lighting.EXE tabs"),
            vec![v(&["text", "-f", "price"]), v(&["tabs"])]
        );
    }

    #[test]
    fn warns_only_about_fresh_failures_of_this_session() {
        let root = std::env::temp_dir().join(format!("wisply-prewarn-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let path = file(&root, "s1");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let line = |at: f64, argv: &str, lesson: &str| {
            format!("{at}\telement e12 not found -> try: wisply snap\t{lesson}\t{argv}\n")
        };
        let old = now() - FRESH_S - 5.0;
        let args = format!("click{ARG_SEP}Log in");
        fs::write(&path, line(now(), &args, "snap") + &line(old, "snap", "")).unwrap();
        let ev = |cmd: &str, sid: &str| json!({"hook_event_name": "PreToolUse", "tool_name": "Bash", "session_id": sid, "tool_input": {"command": cmd}});
        let text = hint(&root, &ev("rtk wisply click \"Log in\"", "s1")).unwrap();
        let said: Value = serde_json::from_str(&text).unwrap();
        let said = said["hookSpecificOutput"]["additionalContext"].as_str().unwrap();
        assert_eq!(
            said,
            "wisply: `wisply click \"Log in\"` already failed in this session: element e12 not found -> try: wisply snap. After this error `wisply snap` worked here before. Change the command instead of running it again unchanged."
        );
        assert!(hint(&root, &ev("wisply snap", "s1")).is_none());
        assert!(hint(&root, &ev("wisply click \"Log in\"", "s2")).is_none());
        assert!(hint(&root, &ev("wisply click Login", "s1")).is_none());
        let write = json!({"hook_event_name": "PreToolUse", "tool_name": "Write", "session_id": "s1"});
        assert!(hint(&root, &write).is_none());
        assert_eq!(parse("bad line\n1\ta\tb\tc").len(), 1);
        let _ = fs::remove_dir_all(&root);
    }
}
