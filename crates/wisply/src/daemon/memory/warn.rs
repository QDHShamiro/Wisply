use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

use wisply_core::QUIET;

use super::super::Daemon;
use super::super::ctx::Ctx;
use crate::prewarn::{ARG_SEP, Entry, FRESH_S, file, now};

const KEPT: usize = 30;
const ERR_CHARS: usize = 200;
const STALE: Duration = Duration::from_secs(2 * 86_400);
const RETRIED: &[&str] = &["wait"];
const LOUD: [&str; 2] = ["\n! the same command failed", "\n! STOP:"];
const AGAIN: &str =
    "\n! this exact command already failed earlier in this session: change it instead of running it again";

pub fn after(
    d: &Daemon,
    ctx: &mut Ctx,
    name: &str,
    argv: &[String],
    code: i32,
    lesson: Option<String>,
    out: &mut String,
) {
    if QUIET.contains(&name) || RETRIED.contains(&name) || argv.is_empty() {
        return;
    }
    let before = ctx.failed_cmds.iter().position(|e| e.argv == argv);
    let t = now();
    if code == 0 {
        if let Some(i) = before {
            ctx.failed_cmds.remove(i);
            save(&d.home, &ctx.sid, &ctx.failed_cmds);
        }
        return;
    }
    if let Some(i) = before
        && t - ctx.failed_cmds.remove(i).at <= FRESH_S
        && !LOUD.iter().any(|l| out.contains(l))
    {
        out.push_str(AGAIN);
    }
    let err = first_line(out);
    ctx.failed_cmds.push(Entry { argv: argv.to_vec(), err, lesson: lesson.unwrap_or_default(), at: t });
    if ctx.failed_cmds.len() > KEPT {
        ctx.failed_cmds.remove(0);
    }
    save(&d.home, &ctx.sid, &ctx.failed_cmds);
}

fn first_line(out: &str) -> String {
    let line = out.lines().next().unwrap_or("");
    line.strip_prefix("err: ").unwrap_or(line).chars().take(ERR_CHARS).collect()
}

fn flat(text: &str) -> String {
    text.replace(['\t', '\n', '\r', ARG_SEP], " ")
}

fn plain(argv: &[String]) -> bool {
    argv.iter().all(|a| !a.contains(['\t', '\n', '\r', ARG_SEP]))
}

fn save(home: &Path, sid: &str, failed: &[Entry]) {
    if sid.is_empty() {
        return;
    }
    let path = file(home, sid);
    let rows: Vec<String> = failed
        .iter()
        .filter(|e| plain(&e.argv))
        .map(|e| format!("{}\t{}\t{}\t{}\n", e.at, flat(&e.err), flat(&e.lesson), e.argv.join(&ARG_SEP.to_string())))
        .collect();
    if rows.is_empty() {
        let _ = fs::remove_file(&path);
        return;
    }
    let Some(dir) = path.parent() else { return };
    if !path.exists() {
        prune(dir);
        let _ = fs::create_dir_all(dir);
    }
    let _ = fs::write(&path, rows.concat());
}

fn prune(dir: &Path) {
    let Ok(list) = fs::read_dir(dir) else { return };
    let old = |at: SystemTime| at.elapsed().is_ok_and(|age| age > STALE);
    for entry in list.flatten() {
        if entry.metadata().and_then(|m| m.modified()).is_ok_and(old) {
            let _ = fs::remove_file(entry.path());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prewarn::parse;

    #[test]
    fn failures_land_in_the_session_file_the_hook_reads() {
        let home = std::env::temp_dir().join(format!("wisply-warn-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        let argv = vec!["click".to_string(), "Log in".to_string()];
        let failed = vec![
            Entry { argv: argv.clone(), err: "x\ty".into(), lesson: "snap".into(), at: 5.0 },
            Entry { argv: vec!["type".into(), "a\nb".into()], err: "e".into(), lesson: String::new(), at: 6.0 },
        ];
        save(&home, "", &failed);
        assert!(!file(&home, "").exists());
        save(&home, "s9", &failed);
        let rows = parse(&fs::read_to_string(file(&home, "s9")).unwrap());
        assert_eq!(rows.len(), 1);
        assert_eq!((rows[0].argv.clone(), rows[0].err.as_str(), rows[0].lesson.as_str()), (argv, "x y", "snap"));
        save(&home, "s9", &[]);
        assert!(!file(&home, "s9").exists());
        assert_eq!(first_line("err: no tab t5 -> try: wisply tabs\nnow on x"), "no tab t5 -> try: wisply tabs");
        let _ = fs::remove_dir_all(&home);
    }
}
