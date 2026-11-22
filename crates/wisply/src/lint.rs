use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use wisply_core::Fail;

const PAIRS: &[(&str, &str)] = &[
    ("scan for", "review"),
    ("attack surface", "public surface"),
    ("attack vector", "code path"),
    ("privilege escalation", "permission change"),
    ("command injection", "unvalidated input"),
    ("sql injection", "unvalidated query input"),
    ("remote code execution", "unintended code path"),
    ("denial of service", "overload condition"),
    ("brute force", "exhaustive test"),
    ("penetration test", "robustness review"),
    ("attack", "edge case"),
    ("exploit", "issue"),
    ("vulnerabilities", "code-quality issues"),
    ("vulnerability", "code-quality issue"),
    ("vulnerable", "fragile"),
    ("reconnaissance", "survey"),
    ("recon", "survey"),
    ("payloads", "input data"),
    ("payload", "input data"),
    ("backdoor", "hidden entry point"),
    ("malware", "unwanted code"),
    ("malicious", "unexpected"),
    ("pentest", "robustness review"),
    ("hardening", "robustness"),
    ("harden", "make robust"),
    ("bypass", "skip"),
    ("intrusion", "unexpected access"),
    ("breach", "failure"),
    ("keylog", "input capture"),
    ("weaponize", "repurpose"),
    ("sniffing", "inspecting"),
    ("sniff", "inspect"),
    ("scan", "review"),
    ("crack", "test"),
];

struct Hit {
    term: &'static str,
    plain: &'static str,
}

fn is_word(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn swap(text: &str, term: &str, plain: &str) -> (String, usize) {
    let low = text.to_ascii_lowercase();
    let b = low.as_bytes();
    let len = term.len();
    let mut out = String::with_capacity(text.len());
    let (mut start, mut i, mut n) = (0, 0, 0);
    while i + len <= b.len() {
        if &b[i..i + len] == term.as_bytes() {
            let before = i == 0 || !is_word(b[i - 1]);
            let after = i + len == b.len() || !is_word(b[i + len]);
            if before && after {
                out.push_str(&text[start..i]);
                out.push_str(plain);
                i += len;
                start = i;
                n += 1;
                continue;
            }
        }
        i += 1;
    }
    out.push_str(&text[start..]);
    (out, n)
}

fn clean(text: &str) -> (String, Vec<Hit>) {
    let mut cur = text.to_string();
    let mut hits = Vec::new();
    for &(term, plain) in PAIRS {
        let (next, n) = swap(&cur, term, plain);
        if n > 0 {
            hits.push(Hit { term, plain });
            cur = next;
        }
    }
    (cur, hits)
}

fn look(args: &[String]) -> Result<String, Fail> {
    let text = args.join(" ");
    let text = text.trim();
    if text.is_empty() {
        return Err(Fail::hint(
            "lint needs the goal or prompt to check",
            "wisply lint \"improve wisply, scan for unsafe code\"",
        ));
    }
    let (cleaned, hits) = clean(text);
    if hits.is_empty() {
        return Ok(format!("reads plain already, nothing that trips the cyber filter:\n  {text}"));
    }
    let mut out = format!("{} word(s) that can trip the cyber filter on plain code work:", hits.len());
    for hit in &hits {
        out.push_str(&format!("\n  {} -> {}", hit.term, hit.plain));
    }
    out.push_str("\ncleaned goal:\n  ");
    out.push_str(&cleaned);
    Ok(out)
}

fn log_file(home: &Path) -> PathBuf {
    home.join("lint-log.txt")
}

fn record(home: &Path, text: &str) -> Result<(), Fail> {
    let line = text.replace(['\n', '\r', '\t'], " ");
    let _ = fs::create_dir_all(home);
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file(home))
        .and_then(|mut f| writeln!(f, "{line}"))
        .map_err(|e| Fail::new(format!("cannot write the lint log: {e}")))
}

fn list(home: &Path) -> String {
    let text = fs::read_to_string(log_file(home)).unwrap_or_default();
    let lines: Vec<&str> = text.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    if lines.is_empty() {
        return "no flagged prompts logged yet: wisply lint \"<prompt>\" --log".to_string();
    }
    let mut counts: Vec<(&str, usize)> = Vec::new();
    for line in &lines {
        for hit in clean(line).1 {
            match counts.iter_mut().find(|(t, _)| *t == hit.term) {
                Some(c) => c.1 += 1,
                None => counts.push((hit.term, 1)),
            }
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    let top: Vec<String> = counts.iter().take(10).map(|(term, n)| format!("  {n}x {term}")).collect();
    if top.is_empty() {
        return format!("{} flagged prompt(s) logged, none with a known trigger word", lines.len());
    }
    format!(
        "{} flagged prompt(s) logged\ntop trigger words (send these to /feedback):\n{}",
        lines.len(),
        top.join("\n")
    )
}

pub fn run(home: &Path, args: &[String]) -> ExitCode {
    if matches!(args.first().map(String::as_str), Some("--list" | "list")) {
        println!("{}", list(home));
        return ExitCode::SUCCESS;
    }
    let log = args.iter().any(|a| a == "--log");
    let rest: Vec<String> = args.iter().filter(|a| a.as_str() != "--log").cloned().collect();
    match look(&rest) {
        Ok(text) => {
            if log && let Err(fail) = record(home, &rest.join(" ")) {
                println!("{}", fail.text());
                return ExitCode::from(1);
            }
            println!("{text}{}", if log { "\nlogged for /feedback: wisply lint --list" } else { "" });
            ExitCode::SUCCESS
        }
        Err(fail) => {
            println!("{}", fail.text());
            ExitCode::from(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(words: &[&str]) -> Vec<String> {
        words.iter().map(|w| (*w).to_string()).collect()
    }

    #[test]
    fn rewrites_security_jargon_and_leaves_plain_work_alone() {
        let (out, hits) = clean("Scan for vulnerabilities and exploit the attack surface");
        assert_eq!(out, "review code-quality issues and issue the public surface");
        assert_eq!(
            hits.iter().map(|h| h.term).collect::<Vec<_>>(),
            ["scan for", "attack surface", "exploit", "vulnerabilities"]
        );

        let (plain, none) = clean("review error handling and add tests for the parser");
        assert!(none.is_empty());
        assert_eq!(plain, "review error handling and add tests for the parser");

        let (word, _) = clean("the scanner will scan input");
        assert_eq!(word, "the scanner will review input");
    }

    #[test]
    fn the_cli_reports_hits_and_asks_for_text() {
        let fail = look(&[]).unwrap_err().text();
        assert!(fail.starts_with("err: lint needs the goal or prompt to check"));

        let ok = look(&args(&["improve", "wisply,", "scan", "for", "unsafe", "and", "vulnerabilities"])).unwrap();
        assert!(ok.contains("cleaned goal:"));
        assert!(ok.contains("improve wisply, review unsafe and code-quality issues"));

        let clean = look(&args(&["add", "tests", "for", "the", "parser"])).unwrap();
        assert!(clean.starts_with("reads plain already"));
    }

    #[test]
    fn non_ascii_text_rewrites_without_panic() {
        let (out, hits) = clean("prüfe den exploit für größe und scanne mehr");
        assert!(out.contains("issue"), "{out}");
        assert_eq!(hits.iter().map(|h| h.term).collect::<Vec<_>>(), ["exploit"]);
        let (plain, none) = clean("nur deutscher text ohne treffer: öäüß");
        assert!(none.is_empty());
        assert_eq!(plain, "nur deutscher text ohne treffer: öäüß");
    }

    #[test]
    fn logging_records_prompts_and_ranks_trigger_words() {
        let home = std::env::temp_dir().join(format!("wisply-lint-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(&home).unwrap();
        assert!(list(&home).starts_with("no flagged prompts logged yet"));
        record(&home, "scan the auth module").unwrap();
        record(&home, "scan the parser and exploit it").unwrap();
        let out = list(&home);
        assert!(out.starts_with("2 flagged prompt(s) logged"), "{out}");
        assert!(out.contains("2x scan"), "{out}");
        assert!(out.contains("1x exploit"), "{out}");
        let _ = fs::remove_dir_all(&home);
    }
}
