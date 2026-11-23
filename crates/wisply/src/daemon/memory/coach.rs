use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};
use wisply_memory::{Store, now, project_of};

use super::super::config;
use super::coach_read::{Step, steps};
use crate::daemon::doctor::ai::grade::clean_word;
use crate::daemon::doctor::ai::sessions::{codex_files, modified};

const SWITCH: &str = "coach";
const DIR: &str = "coach";
const QUIET_S: f64 = 600.0;
const CLAUDE_MARK: &str = "coach_mark";
const CODEX_MARK: &str = "coach_codex";
const READS_MIN: usize = 3;
const FAILS_MIN: usize = 2;
const BIG_CHARS: usize = 20_000;
const BIG_MIN: usize = 2;
const SHOWN: usize = 3;
const LINE_MAX: usize = 320;
const NAME_MAX: usize = 40;
const HEAD: &str = "Coach (last session):";
const UNTESTED: &str = "code edited and never built or tested after, run the check before calling it done";
const CODE: [&str; 20] = [
    "rs", "java", "kt", "kts", "ts", "tsx", "js", "jsx", "mjs", "py", "go", "c", "cc", "cpp", "h", "cs", "swift", "rb",
    "php", "lua",
];
const CHECKS: [&str; 22] = [
    "cargo", "mvn", "mvnw", "gradle", "gradlew", "pytest", "make", "tsc", "test", "tests", "build", "clippy", "javac",
    "dotnet", "vitest", "jest", "ruff", "mypy", "lint", "check", "compile", "tox",
];

fn name_of(path: &str) -> String {
    let last = path.rsplit(['/', '\\']).next().unwrap_or(path);
    let words: Vec<String> = last.split_whitespace().map(clean_word).collect();
    words.join(" ").chars().take(NAME_MAX).collect()
}

fn rereads(steps: &[Step]) -> Option<String> {
    let mut seen: HashMap<(&str, &str), usize> = HashMap::new();
    let mut worst: Option<(&str, usize)> = None;
    for step in steps {
        match step {
            Step::Read(path, part) => {
                let n = seen.entry((path.as_str(), part.as_str())).or_default();
                *n += 1;
                if worst.is_none_or(|(_, w)| *n > w) {
                    worst = Some((path.as_str(), *n));
                }
            }
            Step::Edit(path) => seen.retain(|(p, _), _| *p != path.as_str()),
            _ => {}
        }
    }
    let (path, n) = worst.filter(|(_, n)| *n >= READS_MIN)?;
    Some(format!("read {} {n}x without editing, keep what the first read showed", name_of(path)))
}

fn repeats(steps: &[Step]) -> Option<String> {
    let mut count: HashMap<&str, usize> = HashMap::new();
    for step in steps {
        if let Step::Out(Some(sig), _) = step
            && !sig.is_empty()
        {
            *count.entry(sig.as_str()).or_default() += 1;
        }
    }
    let (sig, n) =
        count.into_iter().filter(|(_, n)| *n >= FAILS_MIN).max_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(a.0)))?;
    Some(format!("same error {n}x ({sig}), change the approach after the first"))
}

fn is_code(path: &str) -> bool {
    Path::new(path).extension().and_then(|e| e.to_str()).is_some_and(|e| CODE.contains(&e.to_lowercase().as_str()))
}

fn is_check(cmd: &str) -> bool {
    cmd.to_lowercase().split(|c: char| !c.is_ascii_alphanumeric()).any(|w| CHECKS.contains(&w))
}

fn untested(steps: &[Step]) -> Option<String> {
    let last = steps.iter().rposition(|s| matches!(s, Step::Edit(p) if is_code(p)))?;
    let checked = steps[last..].iter().any(|s| matches!(s, Step::Run(cmd) if is_check(cmd)));
    (!checked).then(|| UNTESTED.to_string())
}

fn floods(steps: &[Step]) -> Option<String> {
    let n = steps.iter().filter(|s| matches!(s, Step::Out(_, chars) if *chars > BIG_CHARS)).count();
    (n >= BIG_MIN).then(|| {
        format!("{n} tool outputs over {}k chars, cut them with head, tail or a narrower search", BIG_CHARS / 1000)
    })
}

pub fn hints(steps: &[Step]) -> Vec<String> {
    [repeats(steps), rereads(steps), untested(steps), floods(steps)].into_iter().flatten().take(SHOWN).collect()
}

fn file(home: &Path, project: &str) -> PathBuf {
    let safe: String =
        project.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '_' { c } else { '_' }).collect();
    home.join(DIR).join(format!("{safe}.json"))
}

fn saved(home: &Path, project: &str) -> Option<(f64, Vec<String>)> {
    let v: Value = serde_json::from_str(&fs::read_to_string(file(home, project)).ok()?).ok()?;
    let last = v.get("last").and_then(Value::as_f64).unwrap_or(0.0);
    let hints = v.get("hints")?.as_array()?.iter().filter_map(Value::as_str).map(str::to_string).collect();
    Some((last, hints))
}

fn fresh(home: &Path, project: &str, last: f64) -> bool {
    !project.is_empty() && saved(home, project).is_none_or(|(old, _)| old < last)
}

fn keep(home: &Path, project: &str, last: f64, hints: &[String]) {
    let path = file(home, project);
    if let Some(dir) = path.parent() {
        let _ = fs::create_dir_all(dir);
    }
    let _ = fs::write(&path, json!({ "last": last, "hints": hints }).to_string());
}

pub fn line(home: &Path, project: &str) -> Option<String> {
    if project.is_empty() || !config::on(&config::load(home), SWITCH) {
        return None;
    }
    let (_, hints) = saved(home, project)?;
    let mut out = HEAD.to_string();
    for (i, hint) in hints.iter().enumerate() {
        let sep = if i == 0 { " " } else { "; " };
        if i > 0 && out.chars().count() + sep.len() + hint.chars().count() > LINE_MAX {
            break;
        }
        out.push_str(sep);
        out.push_str(hint);
    }
    (!hints.is_empty()).then(|| out.chars().take(LINE_MAX).collect())
}

fn mark(store: &Store, key: &str, quiet: f64) -> Option<f64> {
    let got = store.meta(key).ok().flatten().and_then(|m| m.parse::<f64>().ok());
    if got.is_none() {
        let _ = store.set_meta(key, &quiet.to_string());
    }
    got
}

fn codex(home: &Path, store: &Store, user: &Path, quiet: f64) {
    let Some(after) = mark(store, CODEX_MARK, quiet) else { return };
    let mut files: Vec<(f64, PathBuf)> = codex_files(user)
        .into_iter()
        .filter_map(|p| {
            let at = modified(&p)? as f64;
            (at > after && at <= quiet).then_some((at, p))
        })
        .collect();
    files.sort_by(|a, b| a.0.total_cmp(&b.0));
    for (at, path) in files {
        let (found, cwd) = steps(&path, true);
        let project = project_of(&cwd);
        if fresh(home, &project, at) {
            keep(home, &project, at, &hints(&found));
        }
    }
    let _ = store.set_meta(CODEX_MARK, &quiet.to_string());
}

pub fn tick(home: &Path, store: &Store) {
    if !config::on(&config::load(home), SWITCH) {
        return;
    }
    let quiet = now() - QUIET_S;
    if let Some(after) = mark(store, CLAUDE_MARK, quiet) {
        for (project, transcript, last) in store.quiet_sessions(after, quiet).unwrap_or_default() {
            if fresh(home, &project, last) {
                keep(home, &project, last, &hints(&steps(Path::new(&transcript), false).0));
            }
        }
        let _ = store.set_meta(CLAUDE_MARK, &quiet.to_string());
    }
    codex(home, store, &wisply_proto::user_dir(), quiet);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(path: &str) -> Step {
        Step::Read(path.into(), ":".into())
    }

    fn fail(sig: &str) -> Step {
        Step::Out(Some(sig.into()), 10)
    }

    #[test]
    fn each_rule_fires_only_past_its_line() {
        let mut steps = vec![read("C:/x/src/main.rs"), read("C:/x/src/main.rs")];
        assert!(rereads(&steps).is_none());
        steps.push(read("C:/x/src/main.rs"));
        let three = "read main.rs 3x without editing, keep what the first read showed";
        assert_eq!(rereads(&steps).as_deref(), Some(three));
        steps.insert(2, Step::Edit("C:/x/src/main.rs".into()));
        assert!(rereads(&steps).is_none());
        assert!(repeats(&[fail("a"), fail("b")]).is_none());
        let twice = "same error 2x (a), change the approach after the first";
        assert_eq!(repeats(&[fail("a"), Step::Out(None, 5), fail("a")]).as_deref(), Some(twice));
        let edit = || Step::Edit("src/lib.rs".into());
        assert_eq!(untested(&[edit(), Step::Run("git status".into())]).as_deref(), Some(UNTESTED));
        assert!(untested(&[edit(), Step::Run("cd x && cargo test -q".into())]).is_none());
        assert!(untested(&[Step::Edit("README.md".into())]).is_none());
        assert!(untested(&[Step::Run("mvn package".into()), edit()]).is_some());
        assert!(floods(&[Step::Out(None, BIG_CHARS + 1)]).is_none());
        let big = floods(&[Step::Out(None, BIG_CHARS + 1), Step::Out(Some("x".into()), BIG_CHARS * 3)]);
        assert_eq!(
            big.as_deref(),
            Some("2 tool outputs over 20k chars, cut them with head, tail or a narrower search")
        );
        let mut all = vec![read("a.rs"), read("a.rs"), read("a.rs"), fail("e"), fail("e"), edit()];
        all.extend([Step::Out(None, BIG_CHARS + 1), Step::Out(None, BIG_CHARS + 1)]);
        let got = hints(&all);
        assert_eq!(got.len(), SHOWN);
        assert!(got[0].starts_with("same error") && got[1].starts_with("read a.rs") && got[2] == UNTESTED);
    }

    #[test]
    fn hints_are_kept_per_project_and_only_newer_sessions_replace_them() {
        let home = std::env::temp_dir().join(format!("wisply-coach-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        assert!(line(&home, "proj").is_none());
        assert!(fresh(&home, "proj", 10.0) && !fresh(&home, "", 10.0));
        keep(&home, "proj", 10.0, &["a".into(), "b".into()]);
        assert!(!fresh(&home, "proj", 10.0) && fresh(&home, "proj", 11.0));
        assert_eq!(line(&home, "proj").as_deref(), Some("Coach (last session): a; b"));
        keep(&home, "proj", 11.0, &[]);
        assert!(line(&home, "proj").is_none());
        keep(&home, "p/q:r", 1.0, &["x".repeat(300)]);
        assert_eq!(line(&home, "p/q:r").map(|l| l.chars().count()), Some(LINE_MAX));
        keep(&home, "proj", 12.0, &["a".repeat(150), "b".repeat(200), "c".into()]);
        let whole = line(&home, "proj").unwrap_or_default();
        assert!(whole.ends_with(&"a".repeat(150)) && !whole.contains('b'), "{whole}");
        assert!(home.join(DIR).join("p_q_r.json").is_file());
        let _ = fs::remove_dir_all(&home);
    }

    #[test]
    fn codex_rollouts_are_coached_after_the_first_mark() {
        let root = std::env::temp_dir().join(format!("wisply-coach-codex-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let (home, user) = (root.join("home"), root.join("user"));
        let day = user.join(".codex/sessions/2026/11/23");
        fs::create_dir_all(&day).unwrap();
        fs::create_dir_all(root.join("demo/.git")).unwrap();
        let cwd = root.join("demo").display().to_string().replace('\\', "/");
        let meta = json!({"type": "session_meta", "payload": {"cwd": cwd}}).to_string();
        let cat = json!({"command": ["bash", "-lc", "cat src/main.rs"]}).to_string();
        let call = json!({"type": "response_item", "payload": {"type": "function_call", "arguments": cat}}).to_string();
        fs::write(day.join("rollout-1.jsonl"), [meta, call.clone(), call.clone(), call].join("\n")).unwrap();
        let store = Store::open(&root.join("db")).unwrap();
        let quiet = now() + 100.0;
        codex(&home, &store, &user, quiet);
        let project = project_of(&cwd);
        assert!(line(&home, &project).is_none());
        store.set_meta(CODEX_MARK, "0").unwrap();
        codex(&home, &store, &user, quiet);
        let shown = line(&home, &project).unwrap_or_default();
        assert!(shown.starts_with("Coach (last session): read main.rs 3x"), "{shown}");
        let _ = fs::remove_dir_all(&root);
    }
}
