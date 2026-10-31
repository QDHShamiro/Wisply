use std::fs;

use serde_json::{Value, json};
use wisply_core::Call;

use crate::run::{self, Home, Run, int, text};
use crate::work::mark;
use crate::{Env, files, measure, safety, skill};

pub fn build_proof(r: &Run) -> Option<Value> {
    let root = r.root();
    let (build, _) = measure::commands(text(&r.s, "kind"), &root);
    let cmd = build?;
    Some(measure_once(r, &cmd, 1))
}

fn measure_once(r: &Run, cmd: &str, runs: usize) -> Value {
    let root = r.root();
    match cmd {
        "lint-skill" => {
            let (errs, warns) = skill::lint(&root);
            let mut v = json!({"cmd": cmd, "exit": i32::from(!errs.is_empty()), "errors": errs.len(), "warnings": warns.len(), "at": run::now()});
            if !errs.is_empty() || !warns.is_empty() {
                v["tail"] = json!([errs, warns].concat().join("\n"));
            }
            v
        }
        "selftest" => {
            let script = root.join("scripts").join("selftest.py");
            if !script.exists() {
                return json!({"skipped": "no scripts/selftest.py", "cmd": cmd});
            }
            let ran = measure::run(&format!("python -B \"{}\"", script.display()), &root, measure::TIMEOUT);
            measure::result(cmd, &[ran])
        }
        _ => {
            let mut all = Vec::new();
            for _ in 0..runs.max(1) {
                let ran = measure::run(cmd, &root, measure::TIMEOUT);
                let failed = ran.code != 0;
                all.push(ran);
                if failed {
                    break;
                }
            }
            measure::result(cmd, &all)
        }
    }
}

pub fn ok(m: &Value) -> bool {
    m.get("exit").and_then(Value::as_i64) == Some(0)
}

pub fn test_proof(r: &Run) -> Option<Value> {
    let (_, test) = measure::commands(text(&r.s, "kind"), &r.root());
    Some(measure_once(r, &test?, 1)).filter(|m| m.get("skipped").is_none())
}

pub fn worse(before: &Value, after: &Value) -> bool {
    let passed = |m: &Value| m.pointer("/tests/passed").and_then(Value::as_u64);
    match (ok(before), ok(after)) {
        (true, false) => true,
        (false, false) => matches!((passed(before), passed(after)), (Some(a), Some(b)) if b < a),
        _ => false,
    }
}

pub fn tests_after(r: &mut Run, before: &Value, kept: &[i64]) -> Vec<String> {
    let Some(mut now) = test_proof(r) else { return Vec::new() };
    let first = measure::line(&now);
    if !worse(before, &now) {
        for id in kept {
            if let Some(i) = r.idea_mut(*id) {
                i["proof"] = json!(format!("{}; tests: {first}", text(i, "proof")));
            }
        }
        return vec![format!("tests after the fixes: {first} (before: {})", measure::line(before))];
    }
    let why =
        format!("tests got worse after the fixes ({} -> {first}), taken back newest first", measure::line(before));
    let mut out = Vec::new();
    for id in kept.iter().rev() {
        let _ = safety::revert(r, *id, true);
        mark(r, *id, "dropped", "why", &why);
        uncover(r, *id);
        out.push(format!("#{id}: reverted (tests got worse)"));
        match test_proof(r) {
            Some(m) if worse(before, &m) => now = m,
            Some(m) => {
                now = m;
                break;
            }
            None => break,
        }
    }
    out.push(format!("tests now: {}", measure::line(&now)));
    out
}

fn uncover(r: &mut Run, by: i64) {
    let note = format!("fixed by #{by}");
    let ids: Vec<i64> = r.ideas().iter().filter(|i| text(i, "proof") == note).map(|i| int(i, "id")).collect();
    for id in ids {
        if let Some(i) = r.idea_mut(id) {
            i["status"] = json!("new");
            i.as_object_mut().map(|m| m.remove("proof"));
        }
    }
}

pub fn measure_cmd(home: &Home, c: &Call) -> Result<String, String> {
    let mut r = home.load(c.val("run"))?;
    let label =
        c.pos.get(1).cloned().ok_or("wisply think measure <label> [--cmd \"...\"] [--runs 5] [--test] [--metrics]")?;
    let root = r.root();
    let (build, test) = measure::commands(text(&r.s, "kind"), &root);
    let cmd = c.val("cmd").map(str::to_string).or(if c.on("test") { test } else { build });
    let res = match cmd {
        None => json!({"skipped": format!("no check command for project kind '{}', use --cmd", text(&r.s, "kind"))}),
        Some(cmd) => {
            if root.as_os_str().is_empty() {
                return Err("this run has no project folder; use --cmd with a run that has a path".into());
            }
            let runs = c.val("runs").and_then(|n| n.parse().ok()).unwrap_or(1);
            measure_once(&r, &cmd, runs)
        }
    };
    let mut out = vec![format!("{label}: {}", measure::line(&res))];
    if let Some(t) = res.get("tail").and_then(Value::as_str) {
        out.push(t.to_string());
    }
    r.s["measures"][&label] = res;
    if c.on("metrics") && !r.paths().is_empty() {
        let m = measure::metrics(&r.paths());
        out.push(format!("metrics: {}", measure::metrics_line(&m)));
        r.s["metrics"][&label] = m;
    }
    r.save()?;
    Ok(out.join("\n"))
}

pub fn compare(home: &Home, c: &Call) -> Result<String, String> {
    let r = home.load(c.val("run"))?;
    let a = c.pos.get(1).map_or("before", String::as_str);
    let b = c.pos.get(2).map_or("after", String::as_str);
    let m = |l: &str| r.s.pointer(&format!("/measures/{l}")).cloned().unwrap_or(Value::Null);
    let mut out = measure::compare(&m(a), &m(b));
    let mt = |l: &str| r.s.pointer(&format!("/metrics/{l}")).cloned();
    if let (Some(x), Some(y)) = (mt(a), mt(b)) {
        out.push_str(&format!(
            "\nlines: {} -> {} | files: {} -> {} | complexity max: {} -> {} | over 10: {} -> {}",
            x["lines"], y["lines"], x["files"], y["files"], x["ccn_max"], y["ccn_max"], x["ccn_over"], y["ccn_over"]
        ));
    }
    Ok(out)
}

pub fn save(env: &Env, home: &Home, c: &Call) -> Result<String, String> {
    let r = home.load(c.val("run"))?;
    let label = c.pos.get(1).ok_or("wisply think save <label> [--file f] (or the text on stdin)")?;
    let dir = r.dir.join("text");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let ext = c.val("ext").unwrap_or("md").trim_start_matches('.').to_string();
    let target = dir.join(format!("{}.{ext}", run::slug(label, 40)));
    if let Some(f) = c.val("file") {
        fs::copy(env.path(f), &target).map_err(|e| format!("cannot copy {f}: {e}"))?;
    } else {
        let body = c.val("input").unwrap_or("");
        if body.trim().is_empty() {
            return Err("text missing: pipe it in (<<'EOF') or use --file <path>".into());
        }
        fs::write(&target, body).map_err(|e| e.to_string())?;
    }
    Ok(format!("version saved: {}", files::slashed(&target)))
}

pub fn blind(env: &Env, home: &Home, c: &Call) -> Result<String, String> {
    let mut r = home.load(c.val("run"))?;
    let (Some(a), Some(b)) = (c.pos.get(1), c.pos.get(2)) else {
        return Err("wisply think blind <before file> <after file>".into());
    };
    let e = safety::blind(&mut r, &env.path(a), &env.path(b))?;
    r.save()?;
    Ok(format!(
        "blind comparison {} ready: the user picks in the Wisply window (wisply ui -> Blind), or in the chat:\n  version 1 = {}\n  version 2 = {}\nnever say which one is new; then: wisply think blind-result {} 1|2|same",
        e["n"],
        text(&e["paths"], "1"),
        text(&e["paths"], "2"),
        e["n"]
    ))
}

pub fn blind_result(home: &Home, c: &Call) -> Result<String, String> {
    let mut r = home.load(c.val("run"))?;
    let n: i64 = c.pos.get(1).and_then(|s| s.parse().ok()).ok_or("wisply think blind-result <n> 1|2|same")?;
    let verdict = c.pos.get(2).map(String::as_str).unwrap_or("");
    if !["1", "2", "same"].contains(&verdict) {
        return Err("verdict is 1, 2 or same".into());
    }
    let winner = safety::blind_result(&mut r, n, verdict)?;
    r.save()?;
    Ok(match winner.as_str() {
        "after" => "AFTER picked -> better".into(),
        "before" => "BEFORE picked -> revert the change".into(),
        _ => "TIE -> the original stays".into(),
    })
}

pub fn blind_open(home: &Home) -> Value {
    let mut out = Vec::new();
    for r in home.all() {
        for e in r.s.get("blind").and_then(Value::as_array).cloned().unwrap_or_default() {
            if e.get("winner").is_none() {
                out.push(json!({"run": r.id(), "name": text(&r.s, "name"), "n": e["n"], "paths": e["paths"]}));
            }
        }
    }
    Value::Array(out)
}
