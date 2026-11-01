use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};
use wisply_core::Call;

use crate::files::{self, Pack};
use crate::run::{self, Home, Run, STATUSES, int, text};
use crate::{Env, measure, safety};

pub const MAX_ROUNDS: i64 = 5;
pub const IMPORTANT: i64 = 2;
const START_LESSONS: usize = 5;

fn sizes(size: &str) -> (i64, i64) {
    match size {
        "file" => (20, 5),
        "feature" => (50, 12),
        _ => (100, 25),
    }
}

fn tags_for(kind: &str) -> &'static [&'static str] {
    match kind {
        "skill" => &["skill", "prompt", "protokoll", "protocol"],
        "maven" | "gradle" => &["java", "plugin", "code", "folia"],
        "npm" => &["web", "js", "ts", "code"],
        "python" => &["python", "tool", "code"],
        "cargo" => &["rust", "code", "tool"],
        _ => &["text", "plan", "design", "logik", "logic"],
    }
}

pub fn git_info(root: &Path) -> Value {
    let git = |args: &[&str]| -> Option<String> {
        let o = measure::quiet(&mut std::process::Command::new("git")).arg("-C").arg(root).args(args).output().ok()?;
        o.status.success().then(|| String::from_utf8_lossy(&o.stdout).trim().to_string())
    };
    let Some(top) = git(&["rev-parse", "--show-toplevel"]) else { return json!({"repo": false}) };
    let dirty = git(&["status", "--porcelain"]).unwrap_or_default().lines().filter(|l| !l.is_empty()).count();
    json!({"repo": true, "top": top, "branch": git(&["branch", "--show-current"]).unwrap_or_default(),
           "head": git(&["rev-parse", "--short", "HEAD"]).unwrap_or_else(|| "no commits".into()), "dirty": dirty})
}

fn git_hint(g: &Value, name: &str) -> String {
    let branch = text(g, "branch");
    let dirty = int(g, "dirty");
    if (branch == "main" || branch == "master") && dirty == 0 {
        format!("on {branch} and clean -> git checkout -b refactor/{} (fix/ for bug fixes)", run::slug(name, 24))
    } else if dirty > 0 {
        format!("{dirty} changes by someone else -> do not commit or stash them, checkpoints only")
    } else {
        "feature branch -> stay there".into()
    }
}

pub fn matching_lessons(home: &Home, kind: &str) -> Vec<String> {
    let tags = tags_for(kind);
    home.lessons().into_iter().filter(|l| tags.contains(&run::lesson_tag(l).as_str())).take(START_LESSONS).collect()
}

pub fn start(env: &Env, home: &Home, c: &Call) -> Result<String, String> {
    let name = c.pos.get(1).cloned().ok_or("wisply think start \"<name>\" [paths] [--goal \"...\"] [--quick]")?;
    let paths: Vec<PathBuf> = c.pos.iter().skip(2).map(|p| env.path(p)).collect();
    for p in &paths {
        if !p.exists() {
            return Err(format!("{} does not exist", p.display()));
        }
    }
    let quick = c.on("quick");
    let list = if paths.is_empty() { Vec::new() } else { files::walk(&paths) };
    let count = list.len();
    let size = c.val("size").map(str::to_string).unwrap_or_else(|| {
        match count {
            0 | 1 => "file",
            2..=25 => "feature",
            _ => "project",
        }
        .to_string()
    });
    if let Ok(old) = home.load(None)
        && text(&old.s, "status") == "running"
    {
        env.note(format!("open run {} ({}) is paused, back with --run {}", old.id(), text(&old.s, "name"), old.id()));
    }
    let root = files::root_for(&paths);
    let kind = root.as_deref().map_or("", measure::kind_of);
    let (goal_first, _) = sizes(&size);
    let mut r = Run::create(home, &name);
    let packs: Vec<&str> =
        root.as_deref().map(|rt| env.packs(rt, &list)).unwrap_or_default().iter().map(|p| p.name()).collect();
    let git = root.as_deref().map_or(json!({"repo": false}), git_info);
    for (k, v) in [
        ("goal", json!(c.val("goal").unwrap_or(&name))),
        ("paths", json!(paths.iter().map(|p| p.to_string_lossy().to_string()).collect::<Vec<_>>())),
        ("size", json!(size)),
        ("mode", json!(if quick { "quick" } else { "full" })),
        ("idea_goal", json!(if quick { 10 } else { goal_first })),
        ("max_rounds", json!(if quick { 1 } else { MAX_ROUNDS })),
        ("round", json!(1)),
        ("kind", json!(kind)),
        ("root", json!(root.as_deref().map(|r| r.to_string_lossy().to_string()).unwrap_or_default())),
        ("packs", json!(packs)),
        ("git", git.clone()),
    ] {
        r.map().insert(k.into(), v);
    }
    r.save()?;
    home.set_current(r.id());
    let bk = if paths.is_empty() { json!({"ok": false, "mb": 0, "files": 0}) } else { safety::backup(&r, &list) };
    r.map().insert("backup".into(), bk.clone());
    r.save()?;
    let mut out = vec![
        format!("run {} ({})", r.id(), if quick { "quick" } else { "full" }),
        format!("size: {size} -> at least {} ideas, max {} rounds", int(&r.s, "idea_goal"), int(&r.s, "max_rounds")),
    ];
    if let Some(rt) = &root {
        let (build, test) = measure::commands(kind, rt);
        out.push(format!(
            "project: {} in {} | build: {} | test: {} | checks: {}",
            if kind.is_empty() { "unknown" } else { kind },
            files::slashed(rt),
            build.as_deref().unwrap_or("- (use --cmd)"),
            test.as_deref().unwrap_or("-"),
            if packs.is_empty() { "-".to_string() } else { packs.join(", ") }
        ));
    } else {
        out.push("no path: text or plan in the chat -> keep versions with wisply think save <label>".into());
    }
    out.push(if git["repo"] == true {
        format!(
            "git: {} @ {}, {} changed -> {}",
            text(&git, "branch"),
            text(&git, "head"),
            int(&git, "dirty"),
            git_hint(&git, &name)
        )
    } else {
        "git: none".into()
    });
    if !paths.is_empty() {
        out.push(if bk["ok"] == true {
            format!("backup: {} files, {} MB", bk["files"], bk["mb"])
        } else {
            format!("backup: TOO BIG (over {} MB) -> checkpoints per idea only", safety::BACKUP_MAX_MB)
        });
    }
    out.push(if env.playing.is_empty() {
        "game: none".into()
    } else {
        format!("game: {} runs (checks still run)", env.playing)
    });
    let lessons = matching_lessons(home, if kind.is_empty() { "text" } else { kind });
    if !lessons.is_empty() {
        out.push("matching lessons:".into());
        out.extend(lessons.iter().map(|l| format!("  {l}")));
    }
    out.push(
        "next: 2 understand -> wisply think crit \"<criterion>\" ...; wisply think scan finds problems itself".into(),
    );
    Ok(out.join("\n"))
}

pub fn crit(home: &Home, c: &Call) -> Result<String, String> {
    let mut r = home.load(c.val("run"))?;
    let new: Vec<String> = c.pos.iter().skip(1).map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).collect();
    if new.is_empty() {
        return Err("wisply think crit \"<criterion>\" ...".into());
    }
    let started = r.ideas().iter().any(|i| ["done", "unchecked"].contains(&text(i, "status")));
    let why = c.val("why");
    if started && why.is_none() {
        return Err(
            "criteria are fixed since step 2. Add only with a reason: wisply think crit \"<c>\" --why \"<reason>\""
                .into(),
        );
    }
    if let (true, Some(w)) = (started, why) {
        r.list("criteria_later").push(json!({"criteria": new, "why": w}));
    }
    let list = r.list("criteria");
    for n in new {
        if !list.iter().any(|x| x.as_str() == Some(&n)) {
            list.push(json!(n));
        }
    }
    let all: Vec<String> = list.iter().filter_map(Value::as_str).map(str::to_string).collect();
    r.save()?;
    Ok(format!("criteria: {}", all.join(" | ")))
}

pub fn hint(home: &Home, c: &Call) -> Result<String, String> {
    let mut r = home.load(c.val("run"))?;
    let t = c.pos[1..].join(" ");
    if t.trim().is_empty() {
        return Err("wisply think hint \"<doubt about the goal>\"".into());
    }
    r.list("hints").push(json!(t));
    let n = r.list("hints").len();
    r.save()?;
    Ok(format!("hint about the goal saved ({n})"))
}

pub fn source(home: &Home, c: &Call) -> Result<String, String> {
    let mut r = home.load(c.val("run"))?;
    if c.on("list") || c.pos.len() < 2 {
        let rows: Vec<String> =
            r.list("sources").iter().map(|s| format!("- {} - {}", text(s, "url"), text(s, "proof"))).collect();
        return Ok(if rows.is_empty() { "no sources".into() } else { rows.join("\n") });
    }
    let entry = json!({"url": c.pos[1], "proof": c.pos[2..].join(" ")});
    r.list("sources").push(entry);
    let n = r.list("sources").len();
    r.save()?;
    Ok(format!("source {n} saved"))
}

fn round_stats(r: &Run, n: i64) -> (usize, Vec<(&'static str, Vec<Value>)>) {
    let ideas = r.ideas();
    let mine: Vec<Value> = ideas
        .iter()
        .filter(|i| {
            i.get("round_status").and_then(Value::as_i64).unwrap_or(int(i, "round")) == n || int(i, "round") == n
        })
        .cloned()
        .collect();
    let by =
        STATUSES.iter().map(|s| (*s, mine.iter().filter(|i| text(i, "status") == *s).cloned().collect())).collect();
    (ideas.iter().filter(|i| int(i, "round") == n).count(), by)
}

fn of<'a>(by: &'a [(&'static str, Vec<Value>)], st: &str) -> &'a [Value] {
    by.iter().find(|(s, _)| *s == st).map_or(&[], |(_, v)| v.as_slice())
}

pub fn counted(ideas: &[Value], round: Option<i64>) -> i64 {
    let mine = ideas.iter().filter(|i| round.is_none_or(|n| int(i, "round") == n));
    let wisply = mine.clone().filter(|i| text(i, "by") == "wisply").count() as i64;
    mine.count() as i64 - wisply + wisply / 2
}

fn target(r: &Run, n: i64) -> i64 {
    if n == 1 {
        int(&r.s, "idea_goal")
    } else if text(&r.s, "mode") == "quick" {
        0
    } else {
        sizes(text(&r.s, "size")).1
    }
}

fn gaps(r: &Run, n: i64, by: &[(&'static str, Vec<Value>)]) -> Vec<String> {
    let mut out = Vec::new();
    if r.s.get("criteria").and_then(Value::as_array).is_none_or(Vec::is_empty) {
        out.push("no criteria (step 2: wisply think crit)".into());
    }
    if !r.paths().is_empty() && r.s.pointer("/measures/before").is_none() {
        out.push("no baseline (step 3: wisply think measure before)".into());
    }
    let new = counted(&r.ideas(), Some(n));
    if new < target(r, n) && int(&r.s, "ideas_out") != n {
        out.push(format!(
            "idea goal of round {n} not reached ({new}/{}, wisply findings count half): keep collecting or say why (idea out)",
            target(r, n)
        ));
    }
    if !of(by, "done").is_empty() && !r.dir.join(format!("diff-r{n}.txt")).exists() {
        out.push("review missing (step 8: wisply think diff, then check it)".into());
    }
    out
}

pub fn round(home: &Home, c: &Call) -> Result<String, String> {
    let mut r = home.load(c.val("run"))?;
    let n = r.round();
    let (new, by) = round_stats(&r, n);
    let missing = gaps(&r, n, &by);
    let anyway = c.val("anyway");
    if !missing.is_empty() && anyway.is_none() {
        let mut lines: Vec<String> = missing.iter().map(|g| format!("LOCKED: {g}")).collect();
        lines.push(
            "round stays open: fill the gaps; only when impossible: wisply think round --anyway \"<reason>\"".into(),
        );
        return Err(lines.join("\n"));
    }
    let mut out = Vec::new();
    if let (false, Some(why)) = (missing.is_empty(), anyway) {
        r.list("skipped").push(json!({"round": n, "gaps": missing, "why": why}));
        out.push(format!("SKIPPED (goes into the report): {}", missing.join("; ")));
    }
    let productive = of(&by, "done").iter().any(|i| int(i, "impact") >= IMPORTANT);
    let open: usize = r.ideas().iter().filter(|i| text(i, "status") == "new" && int(i, "impact") >= IMPORTANT).count();
    let max = int(&r.s, "max_rounds").max(1);
    let (verdict, why) = if text(&r.s, "mode") == "quick" {
        ("STOP", "quick mode".to_string())
    } else if n >= max {
        ("STOP", format!("round {n} is the maximum"))
    } else if productive {
        ("NEXT", "an important improvement landed -> new round on the new state".to_string())
    } else if open > 0 {
        ("NEXT", format!("{open} important ideas still open -> do them or drop them honestly"))
    } else {
        ("STOP", "round without an important improvement".to_string())
    };
    let count = |s: &str| of(&by, s).len();
    r.list("rounds").push(json!({"n": n, "new": new, "done": count("done"), "dropped": count("dropped"),
        "suggestion": count("suggestion"), "unchecked": count("unchecked"), "verdict": verdict, "why": why}));
    if verdict == "NEXT" {
        r.map().insert("round".into(), json!(n + 1));
    }
    r.save()?;
    out.push(format!(
        "round {n}/{max}: {new} new ideas, {} done, {} dropped, {} suggestions{} -> {verdict} ({why})",
        count("done"),
        count("dropped"),
        count("suggestion"),
        if count("unchecked") > 0 { format!(", {} unchecked", count("unchecked")) } else { String::new() }
    ));
    out.push(if verdict == "NEXT" {
        format!("next: step 5 on the new state (wisply think scan again), idea goal round {}: {} (open important ideas first)", n + 1, target(&r, n + 1))
    } else {
        "next: step 10 -> wisply think report".into()
    });
    Ok(out.join("\n"))
}

pub fn packs_line(packs: &BTreeSet<Pack>) -> String {
    packs.iter().map(|p| p.name()).collect::<Vec<_>>().join(", ")
}
