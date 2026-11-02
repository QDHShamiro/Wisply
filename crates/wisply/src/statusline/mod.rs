mod cache;
pub(crate) mod config;
mod git;
mod github;
pub(crate) mod install;
mod pc;
mod proc;
mod render;

use std::env;
use std::io::{self, IsTerminal, Read, Write};
use std::panic;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};
use wisply_proto::home;

use cache::Entry;
use config::Config;
use github::Outcome;
use render::Git;

const INPUT_MAX: u64 = 256 * 1024;
const VERSION_WAIT: Duration = Duration::from_secs(5);
const COMMAND_SHOWN: usize = 90;
const USAGE: &str = "usage: wisply statusline [on [--force] | off | status | set <segment> on|off | preview]";
const GH_OK: &str = "ok";
const GH_MISSING: &str = "missing";
const GH_FAILED: &str = "failed";

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

pub fn run(args: &[String]) -> ExitCode {
    let root = home();
    let rest = &args[args.len().min(1)..];
    match args.first().map(String::as_str).unwrap_or("") {
        "" => {
            panic::set_hook(Box::new(|_| {}));
            let line = panic::catch_unwind(|| draw(&home())).unwrap_or_default();
            let mut out = io::stdout().lock();
            let _ = out.write_all(line.as_bytes());
            let _ = out.flush();
            ExitCode::SUCCESS
        }
        "refresh" => {
            refresh(&root, rest);
            ExitCode::SUCCESS
        }
        "on" => report(switch_on(&root, rest.iter().any(|a| a == "--force"))),
        "off" => report(switch_off(&root)),
        "status" => report(Ok(status(&root))),
        "set" => report(set(&root, rest)),
        "preview" => report(Ok(preview(&root))),
        _ => report(Err(USAGE.to_string())),
    }
}

fn report(result: Result<String, String>) -> ExitCode {
    match result {
        Ok(text) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("err: {e}");
            ExitCode::from(1)
        }
    }
}

fn read_input() -> Value {
    let mut bytes = Vec::new();
    if io::stdin().is_terminal() || io::stdin().lock().take(INPUT_MAX).read_to_end(&mut bytes).is_err() {
        return Value::Null;
    }
    serde_json::from_slice(&bytes).unwrap_or(Value::Null)
}

fn cwd_of(input: &Value) -> Option<PathBuf> {
    let from = |v: Option<&Value>| v.and_then(Value::as_str).filter(|s| !s.is_empty()).map(PathBuf::from);
    from(input.get("workspace").and_then(|w| w.get("current_dir"))).or_else(|| from(input.get("cwd")))
}

fn draw(home: &Path) -> String {
    let cfg = Config::load(home);
    if !cfg.enabled {
        return String::new();
    }
    let input = read_input();
    let time = now();
    let git = cwd_of(&input).and_then(|cwd| git_view(home, &cwd, time, &cfg));
    let line = render::line(&input, git.as_ref(), time, &cfg);
    render::with_extra(line, cfg.shown("pc").then(|| pc::field(home, time)).flatten())
}

fn wants_extras(cfg: &Config) -> bool {
    ["sync", "github", "repo"].iter().any(|s| cfg.shown(s))
}

fn git_view(home: &Path, cwd: &Path, time: u64, cfg: &Config) -> Option<Git> {
    let repo = git::find(cwd)?;
    let head = git::read_head(&repo.head)?;
    let mut view = Git { branch: head.label().to_string(), ..Git::default() };
    let Some(branch) = head.branch().filter(|_| wants_extras(cfg)) else { return Some(view) };
    let key = cache::key(&repo.root, branch);
    let entry = cache::get(&cache::load(home), &key).filter(|e| cache::fresh(e, time));
    let stale = entry.as_ref().is_none_or(|e| e.wants_git(time));
    let busy = entry.as_ref().is_some_and(|e| e.busy(time));
    if let Some(e) = &entry {
        view.sync = e.sync;
        view.pr = e.pr;
        view.repo = Some(e.repo.clone()).filter(|r| !r.is_empty());
    }
    if stale && !busy {
        spawn_refresh(home, &repo.root, branch);
    }
    Some(view)
}

fn spawn_refresh(home: &Path, root: &Path, branch: &str) {
    let Ok(exe) = env::current_exe() else { return };
    let log = config::dir(home).join("refresh.log");
    let _ = std::fs::create_dir_all(config::dir(home));
    let root = root.to_string_lossy();
    crate::spawn::detached(&exe, &["statusline", "refresh", root.as_ref(), branch], home, &log);
}

fn refresh(home: &Path, args: &[String]) {
    let (Some(root), Some(branch)) = (args.first(), args.get(1)) else { return };
    let root = Path::new(root);
    let key = cache::key(root, branch);
    let time = now();
    let mut map = cache::load(home);
    let mut entry = cache::get(&map, &key).unwrap_or_default();
    if entry.busy(time) {
        return;
    }
    entry.started = time;
    map.insert(key.clone(), entry.to_value());
    if cache::save(home, &map).is_err() {
        return;
    }
    let cfg = Config::load(home);
    entry.sync = git::run_status(root).or(entry.sync);
    entry.git_at = now();
    if entry.repo.is_empty() {
        entry.repo = git::remote_repo(root).unwrap_or_default();
    }
    if cfg.shown("github") && entry.wants_pr(time) {
        apply(&mut entry, github::run(root));
        entry.pr_at = now();
    }
    entry.started = 0;
    let mut map = cache::load(home);
    map.insert(key, entry.to_value());
    cache::prune(&mut map, now());
    let _ = cache::save(home, &map);
}

fn apply(entry: &mut Entry, outcome: Outcome) {
    match outcome {
        Outcome::Found(pr) => {
            entry.pr = Some(pr);
            entry.gh = GH_OK.into();
        }
        Outcome::NoPr => {
            entry.pr = None;
            entry.gh = GH_OK.into();
        }
        Outcome::Missing => {
            entry.pr = None;
            entry.gh = GH_MISSING.into();
        }
        Outcome::Failed => {
            entry.pr = None;
            entry.gh = GH_FAILED.into();
        }
    }
}

fn settings() -> PathBuf {
    crate::setup::claude_env::settings_file()
}

fn own_exe() -> Result<PathBuf, String> {
    crate::setup::place::client_exe().map_err(|e| format!("cannot find this program: {e}"))
}

fn save_enabled(home: &Path, on: bool) -> Result<(), String> {
    let mut cfg = Config::load(home);
    cfg.enabled = on;
    cfg.save(home)
}

fn switch_on(home: &Path, force: bool) -> Result<String, String> {
    let file = settings();
    let result = install::turn_on(&file, home, &own_exe()?, force)?;
    let line = match result {
        install::On::Refused(cmd) => {
            return Ok(format!(
                "your own status line is set ({}): run `wisply statusline on --force` to switch, the old one is saved",
                shorten(&cmd)
            ));
        }
        install::On::Set => format!("status line on in {}", file.display()),
        install::On::Same => format!("status line already on in {}", file.display()),
        install::On::Updated => format!("status line in {} now points to this wisply", file.display()),
        install::On::Replaced => {
            format!(
                "status line on in {}, your old one is saved (wisply statusline off brings it back)",
                file.display()
            )
        }
    };
    save_enabled(home, true)?;
    Ok(line)
}

fn switch_off(home: &Path) -> Result<String, String> {
    let file = settings();
    save_enabled(home, false)?;
    Ok(match install::turn_off(&file, home)? {
        install::Off::Removed => format!("status line off, removed from {}", file.display()),
        install::Off::Restored => format!("status line off, your old one is back in {}", file.display()),
        install::Off::NotOurs => "status line off (the one in Claude Code settings is not Wisply's, left alone)".into(),
        install::Off::Nothing => "status line off (nothing was set in Claude Code settings)".into(),
    })
}

fn set(home: &Path, args: &[String]) -> Result<String, String> {
    let (Some(name), Some(word)) = (args.first(), args.get(1)) else { return Err(USAGE.into()) };
    let on = match word.as_str() {
        "on" | "true" | "1" => true,
        "off" | "false" | "0" => false,
        _ => return Err(USAGE.into()),
    };
    let mut cfg = Config::load(home);
    cfg.set(name, on)?;
    cfg.save(home)?;
    Ok(format!("{name} {word}"))
}

fn shorten(text: &str) -> String {
    let mut cut: String = text.chars().take(COMMAND_SHOWN).collect();
    if text.chars().count() > COMMAND_SHOWN {
        cut.push_str("...");
    }
    cut
}

fn tool_line(program: &str, missing: &str) -> String {
    let done = proc::capture(Command::new(program).arg("--version"), VERSION_WAIT);
    match done.filter(|d| d.ok).and_then(|d| d.out.lines().next().map(str::to_string)) {
        Some(first) => first,
        None => missing.to_string(),
    }
}

fn status(home: &Path) -> String {
    let cfg = Config::load(home);
    let file = settings();
    let setting = match install::owner(&file) {
        Ok(install::Owner::Ours) => "Wisply's".to_string(),
        Ok(install::Owner::Nobody) => "not set".to_string(),
        Ok(install::Owner::Foreign(cmd)) => format!("yours ({})", shorten(&cmd)),
        Err(e) => e,
    };
    let lines = [
        format!("statusline | {}", if cfg.enabled { "on" } else { "off" }),
        format!("claude setting | {setting} | {}", file.display()),
        format!("segments | {}", cfg.listing()),
        format!("git | {}", tool_line("git", "missing: branch still shows, dirty and sync stay empty")),
        format!("gh | {}", tool_line("gh", "missing: the GitHub segment stays empty")),
    ];
    lines.join("\n")
}

fn preview(home: &Path) -> String {
    let cfg = Config::load(home);
    let time = now();
    let sample = json!({
        "model": {"display_name": "Opus"},
        "effort": {"level": "high"},
        "context_window": {"used_percentage": 42, "current_usage": {"input_tokens": 34200}},
        "rate_limits": {
            "five_hour": {"used_percentage": 12, "resets_at": time + 11_400},
            "seven_day": {"used_percentage": 40, "resets_at": time + 187_200}}
    });
    let view = Git {
        branch: "feat/example".into(),
        repo: Some("owner/repo".into()),
        sync: Some(git::Sync { dirty: true, ahead: 2, behind: 1 }),
        pr: Some(github::Pr { number: 41, state: github::State::Open, checks: github::Checks::Pass }),
    };
    render::line(&sample, Some(&view), time, &cfg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_working_directory_comes_from_the_workspace_or_cwd() {
        let a = json!({"workspace": {"current_dir": "C:/x"}, "cwd": "C:/y"});
        assert_eq!(cwd_of(&a), Some(PathBuf::from("C:/x")));
        assert_eq!(cwd_of(&json!({"cwd": "C:/y"})), Some(PathBuf::from("C:/y")));
        assert_eq!(cwd_of(&json!({"workspace": {"current_dir": ""}})), None);
        assert_eq!(cwd_of(&Value::Null), None);
    }

    #[test]
    fn a_gh_outcome_is_stored_in_the_entry() {
        let mut e = Entry::default();
        let pr = github::Pr { number: 1, state: github::State::Open, checks: github::Checks::Pass };
        apply(&mut e, Outcome::Found(pr));
        assert_eq!((e.pr, e.gh.as_str()), (Some(pr), GH_OK));
        apply(&mut e, Outcome::NoPr);
        assert_eq!((e.pr, e.gh.as_str()), (None, GH_OK));
        apply(&mut e, Outcome::Missing);
        assert_eq!(e.gh, GH_MISSING);
        apply(&mut e, Outcome::Failed);
        assert_eq!(e.gh, GH_FAILED);
    }

    #[test]
    fn set_validates_and_saves_segments() {
        let home = env::temp_dir().join(format!("lt-sl-set-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        assert_eq!(set(&home, &["tokens".into(), "off".into()]).unwrap(), "tokens off");
        assert!(!Config::load(&home).shown("tokens"));
        assert!(set(&home, &["nope".into(), "on".into()]).is_err());
        assert!(set(&home, &["tokens".into()]).is_err());
        assert!(set(&home, &["tokens".into(), "maybe".into()]).is_err());
        std::fs::remove_dir_all(&home).unwrap();
    }

    #[test]
    fn shorten_cuts_long_commands() {
        assert_eq!(shorten("abc"), "abc");
        assert_eq!(shorten(&"x".repeat(100)).chars().count(), COMMAND_SHOWN + 3);
    }
}
