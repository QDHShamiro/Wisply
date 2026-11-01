use std::fs;

use serde_json::{Value, json};
use wisply_core::Call;

use crate::flow::{IMPORTANT, counted};
use crate::run::{self, Home, Run, STATUSES, cut, int, points, table, text};
use crate::{Env, measure};

const REPORT_ROWS: usize = 10;
const RUNS_KEEP: usize = 10;
const BACKUPS_KEEP: usize = 3;
const BRIEF_LESSONS: usize = 10;

pub fn report(home: &Home, c: &Call) -> Result<String, String> {
    let mut r = home.load(c.val("run"))?;
    let ideas = r.ideas();
    let by_points = |mut v: Vec<Value>| {
        v.sort_by(|a, b| points(b).total_cmp(&points(a)));
        v
    };
    let pick = |st: &str| ideas.iter().filter(|i| text(i, "status") == st).cloned().collect::<Vec<_>>();
    let done = by_points(pick("done"));
    let dropped = pick("dropped");
    let unchecked = pick("unchecked");
    let open = by_points(
        ideas
            .iter()
            .filter(|i| {
                text(i, "status") == "suggestion" || (text(i, "status") == "new" && int(i, "impact") >= IMPORTANT)
            })
            .cloned()
            .collect(),
    );
    let rounds = r.s.get("rounds").and_then(Value::as_array).map_or(1, |a| a.len().max(1));
    let mut out = vec![format!(
        "**Critical thinking - {}** | {rounds} rounds | {} ideas ({} by wisply) | {} done | {} dropped",
        text(&r.s, "name"),
        ideas.len(),
        ideas.iter().filter(|i| text(i, "by") == "wisply").count(),
        done.len(),
        dropped.len()
    )];
    let rows = |list: &[Value], f: &dyn Fn(&Value) -> String| -> Vec<Vec<String>> {
        list.iter().take(REPORT_ROWS).map(|i| vec![cut(text(i, "title"), 60), cut(&f(i), 70)]).collect()
    };
    if done.is_empty() {
        out.push("No real improvement found - everything was checked anyway (wisply think idea list --all).".into());
    } else {
        out.push(table(
            &["Improved", "Proof"],
            &rows(&done, &|i| {
                let g = i.get("grade").map(|g| format!(" (grade {g})")).unwrap_or_default();
                format!("{}{g}", text(i, "proof"))
            }),
        ));
    }
    if !unchecked.is_empty() {
        out.push(table(
            &["Unchecked (no proof yet)", "What happened"],
            &rows(&unchecked, &|i| text(i, "proof").to_string()),
        ));
    }
    if !dropped.is_empty() {
        out.push(table(&["Dropped", "Reason"], &rows(&dropped, &|i| text(i, "why").to_string())));
    }
    if !open.is_empty() {
        out.push(table(
            &["Open / suggestion", "Why not done"],
            &rows(&open, &|i| {
                let why = text(i, "why");
                if !why.is_empty() {
                    why.to_string()
                } else if text(i, "kind") == "big" {
                    "big - you decide".into()
                } else {
                    "not done in time".into()
                }
            }),
        ));
    }
    let m = r.s.get("measures").cloned().unwrap_or(json!({}));
    if m.get("before").is_some() || m.get("after").is_some() {
        out.push(format!("measured: {}", measure::compare(&m["before"], &m["after"]).replace('\n', " | ")));
    }
    for h in r.s.get("hints").and_then(Value::as_array).cloned().unwrap_or_default() {
        out.push(format!("hint about the goal: {}", h.as_str().unwrap_or("")));
    }
    if let Some(why) = r.s.get("ideas_out_why").and_then(Value::as_object) {
        out.extend(why.iter().map(|(n, w)| format!("ideas ran out (round {n}): {}", w.as_str().unwrap_or(""))));
    }
    for s in r.s.get("skipped").and_then(Value::as_array).cloned().unwrap_or_default() {
        out.push(format!("skipped (round {}): {} - reason: {}", s["round"], s["gaps"], text(&s, "why")));
    }
    for k in r.s.get("criteria_later").and_then(Value::as_array).cloned().unwrap_or_default() {
        out.push(format!("criteria added later: {} - reason: {}", k["criteria"], text(&k, "why")));
    }
    let blind = r.s.get("blind").and_then(Value::as_array).cloned().unwrap_or_default();
    if !blind.is_empty() {
        let won = |w: &str| blind.iter().filter(|b| text(b, "winner") == w).count();
        let open = blind.iter().filter(|b| b.get("winner").is_none()).count();
        out.push(format!(
            "blind choice: new better {} | old better {} | no difference {} | open {open}",
            won("after"),
            won("before"),
            won("same")
        ));
    }
    let small = ideas.iter().filter(|i| text(i, "status") == "new" && int(i, "impact") < IMPORTANT).count();
    let sources = r.s.get("sources").and_then(Value::as_array).map_or(0, Vec::len);
    out.push(format!(
        "sources: {sources}{}",
        if small > 0 { format!(" | {small} small ideas open") } else { String::new() }
    ));
    r.map().insert("status".into(), json!("done"));
    r.map().insert("end".into(), json!(run::now()));
    r.save()?;
    prune(home);
    Ok(out.into_iter().filter(|x| !x.is_empty()).collect::<Vec<_>>().join("\n\n"))
}

fn prune(home: &Home) {
    let finished: Vec<Run> = home.all().into_iter().filter(|r| text(&r.s, "status") == "done").collect();
    let drop = finished.len().saturating_sub(RUNS_KEEP);
    for r in &finished[..drop] {
        let _ = fs::remove_dir_all(&r.dir);
    }
    let kept = &finished[drop..];
    for r in &kept[..kept.len().saturating_sub(BACKUPS_KEEP)] {
        for sub in ["backup", "cp", "blind", "text"] {
            let _ = fs::remove_dir_all(r.dir.join(sub));
        }
    }
}

pub fn status(home: &Home, c: &Call) -> Result<String, String> {
    let r = home.load(c.val("run"))?;
    let ideas = r.ideas();
    let counts: Vec<String> = STATUSES
        .iter()
        .map(|s| (s, ideas.iter().filter(|i| text(i, "status") == *s).count()))
        .filter(|(_, n)| *n > 0)
        .map(|(s, n)| format!("{s} {n}"))
        .collect();
    let crits: Vec<&str> =
        r.s.get("criteria")
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
    let measures: Vec<String> =
        r.s.get("measures")
            .and_then(Value::as_object)
            .map(|m| m.iter().map(|(k, v)| format!("{k}: {}", measure::line(v))).collect())
            .unwrap_or_default();
    let mut out = vec![
        format!(
            "run {} | {} | {} | round {}/{} | {}",
            r.id(),
            text(&r.s, "mode"),
            text(&r.s, "size"),
            r.round(),
            int(&r.s, "max_rounds"),
            text(&r.s, "status")
        ),
        format!("goal: {}", text(&r.s, "goal")),
        format!(
            "ideas {} ({} toward the goal of {}): {}",
            ideas.len(),
            counted(&ideas, None),
            int(&r.s, "idea_goal"),
            counts.join(", ")
        ),
        format!("criteria: {}", if crits.is_empty() { "-".to_string() } else { crits.join(" | ") }),
        format!("measures: {}", if measures.is_empty() { "-".to_string() } else { measures.join(", ") }),
    ];
    let new = ideas.iter().filter(|i| text(i, "status") == "new").count();
    let next = if crits.is_empty() {
        "step 2: wisply think crit \"<criterion>\" ..."
    } else if r.s.pointer("/measures/before").is_none() && !r.paths().is_empty() {
        "step 3: wisply think measure before"
    } else if counted(&ideas, None) < int(&r.s, "idea_goal") && r.round() == 1 {
        "step 4/5: research + ideas (wisply think scan, wisply think idea add)"
    } else if new > 0 {
        "step 6/7: pick + do (wisply think fix, wisply think idea list)"
    } else {
        "step 8/9: review (wisply think diff), then wisply think round"
    };
    out.push(format!("next: {next}"));
    Ok(out.join("\n"))
}

pub fn runs(home: &Home) -> String {
    let cur = home.current_id().unwrap_or_default();
    let all = home.all();
    let rows: Vec<String> = all
        .iter()
        .rev()
        .take(RUNS_KEEP)
        .rev()
        .map(|r| {
            format!(
                "{} {} | {} | round {} | {} ideas",
                if r.id() == cur { "*" } else { " " },
                r.id(),
                text(&r.s, "status"),
                r.round(),
                r.ideas().len()
            )
        })
        .collect();
    if rows.is_empty() { "no runs yet".into() } else { rows.join("\n") }
}

pub fn brief(env: &Env, home: &Home) -> String {
    let items: Vec<String> = home.lessons().into_iter().take(BRIEF_LESSONS).collect();
    let mut out =
        vec![if items.is_empty() { "lessons: none yet".to_string() } else { "lessons from earlier runs:".to_string() }];
    out.extend(items);
    out.push(if env.playing.is_empty() { "game: none".into() } else { format!("game: {} runs", env.playing) });
    if let Ok(r) = home.load(None)
        && text(&r.s, "status") == "running"
    {
        out.push(format!(
            "open run: {} ({}), round {}/{}, {} ideas -> go on with wisply think status or start a new one",
            r.id(),
            text(&r.s, "name"),
            r.round(),
            int(&r.s, "max_rounds"),
            r.ideas().len()
        ));
    }
    let finished: Vec<Run> = home.all().into_iter().filter(|r| text(&r.s, "status") == "done").collect();
    for r in finished.iter().rev().take(BACKUPS_KEEP) {
        let n = r.ideas().iter().filter(|i| text(i, "status") == "unchecked").count();
        if n > 0 {
            out.push(format!(
                "unchecked from {} ({}): {n} ideas -> prove them: wisply think measure after --run {}",
                r.id(),
                text(&r.s, "name"),
                r.id()
            ));
        }
    }
    out.join("\n")
}

pub fn learn(home: &Home, c: &Call) -> Result<String, String> {
    let body = c.pos[1..].join(" ");
    if c.on("list") || body.trim().is_empty() {
        let items = home.lessons();
        return Ok(if items.is_empty() { "no lessons yet".into() } else { items.join("\n") });
    }
    Ok(run::learn(home, body.trim(), c.val("tag").unwrap_or("")))
}
