use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use serde_json::{Value, json};
use wisply_core::Fail;

use super::super::agents;
use super::super::ctx::now;
use super::parse::label;

const FILE: &str = "go-later.json";
const TICK: Duration = Duration::from_secs(5);
const LATE_OK: f64 = 600.0;
const HIDDEN: u32 = 0x0800_0000;

static LOCK: Mutex<()> = Mutex::new(());

#[derive(Debug, Clone, PartialEq)]
pub struct Job {
    pub id: u64,
    pub at: f64,
    pub say: String,
    pub what: String,
}

fn file(home: &Path) -> PathBuf {
    home.join(FILE)
}

fn load(home: &Path) -> Vec<Job> {
    let raw = fs::read_to_string(file(home)).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok());
    let rows = raw.and_then(|v| v["jobs"].as_array().cloned()).unwrap_or_default();
    rows.iter()
        .filter_map(|r| {
            Some(Job {
                id: r["id"].as_u64()?,
                at: r["at"].as_f64()?,
                say: r["say"].as_str()?.to_string(),
                what: r["what"].as_str().unwrap_or("").to_string(),
            })
        })
        .collect()
}

fn save(home: &Path, jobs: &[Job]) -> Result<(), Fail> {
    let rows: Vec<Value> = jobs.iter().map(|j| json!({"id": j.id, "at": j.at, "say": j.say, "what": j.what})).collect();
    let text = serde_json::to_string_pretty(&json!({"jobs": rows})).map_err(|e| Fail::new(e.to_string()))?;
    wisply_core::save(file(home), text).map_err(|e| Fail::new(format!("could not save the plan ({e})")))
}

fn guard() -> std::sync::MutexGuard<'static, ()> {
    LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

pub fn add(home: &Path, at: f64, say: &str, what: &str) -> Result<u64, Fail> {
    let _g = guard();
    let mut jobs = load(home);
    let id = jobs.iter().map(|j| j.id).max().unwrap_or(0) + 1;
    jobs.push(Job { id, at, say: say.to_string(), what: what.to_string() });
    jobs.sort_by(|a, b| a.at.total_cmp(&b.at));
    save(home, &jobs)?;
    Ok(id)
}

pub fn list(home: &Path) -> String {
    let jobs = load(home);
    if jobs.is_empty() {
        return "nothing planned".into();
    }
    let t = now();
    jobs.iter().map(|j| format!("#{} {} · {}", j.id, label(t, j.at), j.what)).collect::<Vec<_>>().join("\n")
}

pub fn cancel(home: &Path, id: u64) -> Result<String, Fail> {
    let _g = guard();
    let mut jobs = load(home);
    let Some(pos) = jobs.iter().position(|j| j.id == id) else {
        return Err(Fail::hint(format!("nothing planned with #{id}"), "wisply go --list"));
    };
    let gone = jobs.remove(pos);
    save(home, &jobs)?;
    Ok(format!("cancelled #{id} ({})", gone.what))
}

pub fn take_due(home: &Path, t: f64) -> Vec<Job> {
    let _g = guard();
    let jobs = load(home);
    let (due, rest): (Vec<Job>, Vec<Job>) = jobs.into_iter().partition(|j| j.at <= t);
    if !due.is_empty() {
        let _ = save(home, &rest);
    }
    due
}

fn quiet(cmd: &mut Command) -> &mut Command {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(HIDDEN);
    }
    cmd
}

pub fn spawn_go(home: &Path, say: &str, extra: &[&str]) -> Result<std::process::Child, Fail> {
    let exe = env::current_exe().map_err(|e| Fail::new(format!("cannot find wisply itself ({e})")))?;
    let mut cmd = Command::new(exe);
    cmd.arg("go").arg(say).args(extra).env("WISPLY_HOME", home);
    quiet(cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()))
        .spawn()
        .map_err(|e| Fail::new(format!("could not start wisply go ({e})")))
}

fn fire(home: &Path, job: Job, t: f64) {
    if t - job.at > LATE_OK {
        agents::remind(home, &format!("missed: {}", job.what), "Wisply was not running at that time");
        return;
    }
    let home = home.to_path_buf();
    let _ = thread::Builder::new().name("go-job".into()).spawn(move || {
        let out = spawn_go(&home, &job.say, &["--now", "--yes"])
            .and_then(|c| c.wait_with_output().map_err(|e| Fail::new(format!("wisply go did not finish ({e})"))));
        match out {
            Ok(o) if o.status.success() => {}
            Ok(o) => {
                let why =
                    String::from_utf8_lossy(if o.stderr.is_empty() { &o.stdout } else { &o.stderr }).trim().to_string();
                agents::remind(&home, &format!("failed: {}", job.what), &why);
            }
            Err(f) => agents::remind(&home, &format!("failed: {}", job.what), &f.text()),
        }
    });
}

pub fn start(home: PathBuf) {
    let _ = thread::Builder::new().name("go-later".into()).spawn(move || {
        loop {
            thread::sleep(TICK);
            let t = now();
            for job in take_due(&home, t) {
                fire(&home, job, t);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_list_cancel_and_take() {
        let home = env::temp_dir().join(format!("wisply-go-later-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(&home).expect("home");
        let t = now();
        let a = add(&home, t + 600.0, "schreib Franz hi", "message to Franz").expect("a");
        let b = add(&home, t - 1.0, "lauter", "media key volumeup").expect("b");
        assert_eq!((a, b), (1, 2));
        assert!(list(&home).starts_with("#2 "));
        let due = take_due(&home, t);
        assert_eq!(due.len(), 1);
        assert_eq!(due[0].say, "lauter");
        assert_eq!(load(&home).len(), 1);
        assert!(cancel(&home, 9).is_err());
        assert!(cancel(&home, 1).is_ok_and(|s| s.contains("message to Franz")));
        assert_eq!(list(&home), "nothing planned");
        let _ = fs::remove_dir_all(&home);
    }
}
