use std::env;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use wisply_core::Fail;

use super::report::{Report, Sev};
use super::scan_code;
use crate::daemon::claude::{claude_exe, clean};
use crate::daemon::media::tools::which;

const AI_WAIT: Duration = Duration::from_secs(1800);
const TICK: Duration = Duration::from_millis(500);
const TEST_AI: &str = "WISPLY_SECURE_AI";
const LOG: &str = "secure-fix.log";
const PROMPT: &str = "Read FIX.md in this folder and fix every open item in it. Edit only files in this folder. \
    Do not commit, do not push, do not run network commands and never print secret values. \
    For a secret move it to an environment variable or an ignored file and tell me in your last line that it must be rotated. \
    When you are done, tick the items in FIX.md.";
const REMAIN: usize = 8;

fn git(root: &Path, args: &[&str]) -> Command {
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(root).args(args).stdin(Stdio::null());
    crate::v2::hide(&mut cmd);
    cmd
}

fn git_text(root: &Path, args: &[&str]) -> Option<String> {
    let out = git(root, args).stderr(Stdio::null()).output().ok().filter(|o| o.status.success())?;
    Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn program(name: &str) -> Option<PathBuf> {
    which(name).or_else(|| {
        let file = if cfg!(windows) { format!("{name}.cmd") } else { name.to_string() };
        env::split_paths(&env::var_os("PATH")?).map(|d| d.join(&file)).find(|p| p.is_file())
    })
}

fn pick(with: Option<&str>, model: Option<&str>) -> Result<(String, Command), Fail> {
    if let Some(path) = env::var_os(TEST_AI) {
        return Ok(("test ai".into(), Command::new(path)));
    }
    let claude = claude_exe();
    let has_claude = claude.is_file() || which("claude").is_some();
    let want = with.unwrap_or(if has_claude { "claude" } else { "codex" });
    match want {
        "claude" if has_claude => {
            let mut cmd = Command::new(claude);
            clean(&mut cmd);
            cmd.args(["-p", PROMPT, "--permission-mode", "acceptEdits", "--allowedTools", "Read,Edit,Write,Glob,Grep"]);
            if let Some(m) = model {
                cmd.args(["--model", m]);
            }
            Ok(("claude".into(), cmd))
        }
        "codex" => {
            let Some(exe) = program("codex") else {
                return Err(Fail::hint("codex is not installed", "install it or use --with claude"));
            };
            let mut cmd = Command::new(exe);
            cmd.args(["exec", "--full-auto", PROMPT]);
            Ok(("codex".into(), cmd))
        }
        "claude" => Err(Fail::hint("claude is not installed", "install it or use --with codex")),
        other => Err(Fail::hint(format!("--with {other} is not known"), "wisply secure fix --with claude|codex")),
    }
}

fn branch_name(root: &Path) -> String {
    let day = wisply_core::local_now()[..10].replace('-', "");
    let base = format!("secure/fix-{day}");
    let taken = |n: &str| {
        git(root, &["rev-parse", "--verify", "--quiet", n])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|s| s.success())
    };
    if !taken(&base) {
        return base;
    }
    (2..).map(|i| format!("{base}-{i}")).find(|n| !taken(n)).unwrap_or(base)
}

pub fn fix_md(before: &Report) -> String {
    let mut out = String::from("# Security fixes\n\nFix every item below. Values of keys are masked on purpose.\n");
    for sev in [Sev::High, Sev::Medium, Sev::Low] {
        let items: Vec<_> = before.found.iter().filter(|f| f.sev == sev).collect();
        if items.is_empty() {
            continue;
        }
        out.push_str(&format!("\n## {} ({})\n\n", sev.word(), items.len()));
        for f in items {
            out.push_str(&format!("- [ ] `{}` {} -> {}\n", f.at(), f.why, f.fix));
        }
    }
    out
}

fn run_ai(mut cmd: Command, root: &Path, home: &Path) -> Result<String, Fail> {
    let log = home.join(LOG);
    let sink = |_: ()| File::create(&log).map(Stdio::from).unwrap_or_else(|_| Stdio::null());
    cmd.current_dir(root).stdin(Stdio::null()).stdout(sink(())).stderr(sink(()));
    crate::v2::hide(&mut cmd);
    let mut child = cmd.spawn().map_err(|e| Fail::new(format!("cannot start the AI: {e}")))?;
    let t0 = Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                return Ok(if status.success() { "done".into() } else { format!("ended with {status}") });
            }
            Ok(None) if t0.elapsed() > AI_WAIT => {
                let _ = child.kill();
                let _ = child.wait();
                return Ok(format!("stopped after {} min", AI_WAIT.as_secs() / 60));
            }
            Ok(None) => thread::sleep(TICK),
            Err(e) => return Err(Fail::new(format!("lost the AI process: {e}"))),
        }
    }
}

pub fn run(root: &Path, home: &Path, with: Option<&str>, model: Option<&str>) -> Result<String, Fail> {
    if !root.join(".git").exists() {
        return Err(Fail::hint("this folder is not a git repository", "git init, commit, then wisply secure fix"));
    }
    let dirty = git_text(root, &["status", "--porcelain"]).unwrap_or_default();
    if dirty.lines().any(|l| !l.ends_with("FIX.md")) {
        return Err(Fail::hint(
            "there are uncommitted changes",
            "commit or stash them first so the fixes stay separate",
        ));
    }
    let (who, cmd) = pick(with, model)?;
    let before = scan_code(root, false, true);
    if before.found.is_empty() {
        return Ok(format!("[secure fix] {}: nothing to fix", before.target));
    }
    let branch = branch_name(root);
    let made = git(root, &["checkout", "-b", &branch]).stdout(Stdio::null()).stderr(Stdio::null()).status();
    if !made.is_ok_and(|s| s.success()) {
        return Err(Fail::hint(format!("could not create branch {branch}"), "git status"));
    }
    fs::write(root.join("FIX.md"), fix_md(&before)).map_err(|e| Fail::new(format!("cannot write FIX.md: {e}")))?;
    let ended = run_ai(cmd, root, home)?;
    let after = scan_code(root, false, true);
    let mut lines = vec![format!(
        "[secure fix] {} on branch {branch} via {who} ({ended}): before {} ({}) -> after {} ({})",
        before.target,
        before.found.len(),
        before.summary(),
        after.found.len(),
        after.summary()
    )];
    for f in after.found.iter().take(REMAIN) {
        lines.push(format!("{:<6} {} {} -> {}", f.sev.word(), f.at(), f.why, f.fix));
    }
    if after.found.len() > REMAIN {
        lines.push(format!("+{} more: wisply secure", after.found.len() - REMAIN));
    }
    lines.push("nothing committed, nothing pushed: review with git diff, FIX.md is the checklist, AI log: secure-fix.log in the Wisply folder".into());
    Ok(lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::daemon::secure::report::Finding;

    #[test]
    fn fix_md_groups_by_severity_and_never_holds_a_value() {
        let mut r = Report::new("repo");
        r.found.push(Finding::new(Sev::Low, "code", "a.py", 2, "debug mode is on", "turn it off"));
        r.found.push(Finding::new(Sev::High, "secret", "c.yml", 1, "Groq key (gsk_…)", "rotate"));
        r.sorted();
        let md = fix_md(&r);
        assert!(md.contains("## high (1)\n\n- [ ] `c.yml:1` Groq key (gsk_…) -> rotate\n"), "{md}");
        assert!(md.find("## high").unwrap() < md.find("## low").unwrap());
    }
}
