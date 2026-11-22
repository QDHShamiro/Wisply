use std::collections::HashMap;

use serde_json::{Value, json};
use wisply_core::{Call, Fail};
use wisply_memory::{FIELDS, KINDS, NewEntry, PRIOS, Run, STATUSES, Todo, WAITS, now, oneline, valid};

use super::super::ctx::Ctx;
use super::super::{Daemon, servers};
use super::list::{author, who_names};
use super::with_store;

const USAGE: &str = "wisply board list [--project p] | entry <id> | entry-set <id> <field> --value=v | \
                     entry-add --task=t [--kind=k --priority=p --status=s --project=p] | entry-undo <id> | \
                     entry-note <id> --value=text | mute <id> | unmute <fingerprint> | server <name> on|off|check|log";
const DONE_KEPT: f64 = 30.0 * 86_400.0;
const SESSIONS_SINCE: f64 = 86_400.0;
const SESSIONS: usize = 40;
const HISTORY: usize = 40;
const LAST_NOTE: usize = 140;
const CHANGE_SHOWN: usize = 200;

fn missing(id: i64) -> Fail {
    Fail::hint(format!("no entry #{id}"), "wisply board list")
}

fn number(c: &Call) -> Result<i64, Fail> {
    c.pos
        .get(1)
        .and_then(|p| p.trim().trim_start_matches('#').parse().ok())
        .ok_or_else(|| Fail::hint("which entry?", USAGE))
}

fn name_of(names: &HashMap<String, String>, who: &str) -> String {
    names.get(who).cloned().unwrap_or_else(|| who.to_string())
}

fn row_json(t: &Todo, names: &HashMap<String, String>) -> Value {
    let last_note = t.note.lines().rev().find(|l| !l.trim().is_empty()).map(|l| oneline(l, LAST_NOTE));
    json!({"id": t.id, "task": t.task, "kind": t.kind, "priority": t.priority, "status": t.status, "wait": t.wait,
           "project": t.project, "list": t.list, "assignee": t.assignee, "who": name_of(names, &t.assignee),
           "source": t.source, "server": t.server, "level": t.level, "count": t.count, "origin": t.origin,
           "first_seen": t.first_seen, "last_seen": t.last_seen, "created": t.created, "updated": t.updated,
           "has_body": !t.body.trim().is_empty(), "has_log": !t.excerpt.trim().is_empty(),
           "last_note": last_note.unwrap_or_default()})
}

fn run_json(r: &Run) -> Value {
    json!({"id": r.id, "project": r.project, "title": r.title, "summary": r.summary, "next": r.next,
           "last": r.last, "status": r.status})
}

fn list(d: &Daemon, c: &Call) -> Result<Value, Fail> {
    let scope = c.val("project").map(str::trim).filter(|p| !p.is_empty()).map(String::from);
    let at = now();
    let (mut rows, mutes, runs, projects) = with_store(d, |s| {
        Ok((
            s.entries(scope.as_deref(), false)?,
            s.mutes()?,
            s.runs_page(scope.as_deref().unwrap_or(""), "", at - SESSIONS_SINCE, SESSIONS, 0)?,
            s.note_projects()?,
        ))
    })?;
    rows.retain(|t| t.status != "done" || t.updated >= at - DONE_KEPT);
    let names = who_names(d, rows.iter().map(|t| t.assignee.clone()).collect());
    let servers = with_store(d, |s| Ok(servers::rows(&d.home, s)))?;
    let mutes: Vec<Value> = mutes
        .iter()
        .map(|m| json!({"fingerprint": m.fingerprint, "server": m.server, "title": m.title, "at": m.at}))
        .collect();
    Ok(json!({
        "rows": rows.iter().map(|t| row_json(t, &names)).collect::<Vec<_>>(),
        "mutes": mutes,
        "sessions": runs.iter().map(run_json).collect::<Vec<_>>(),
        "servers": servers,
        "projects": projects.into_iter().map(|(p, _)| p).collect::<Vec<_>>(),
        "kinds": KINDS, "prios": PRIOS, "statuses": STATUSES, "waits": WAITS, "now": at,
    }))
}

fn entry(d: &Daemon, c: &Call) -> Result<Value, Fail> {
    let id = number(c)?;
    let (t, history) = with_store(d, |s| Ok((s.todo(id)?, s.history(id)?)))?;
    let t = t.ok_or_else(|| missing(id))?;
    let fingerprint = t.fingerprint.clone();
    let muted = !fingerprint.is_empty() && with_store(d, |s| s.muted(&fingerprint))?;
    let mut ids: Vec<String> = history.iter().map(|c| c.who.clone()).collect();
    ids.push(t.assignee.clone());
    let names = who_names(d, ids);
    let history: Vec<Value> = history
        .iter()
        .take(HISTORY)
        .map(|c| {
            json!({"id": c.id, "at": c.at, "who": name_of(&names, &c.who), "field": c.field,
                   "old": oneline(&c.old, CHANGE_SHOWN), "new": oneline(&c.new, CHANGE_SHOWN),
                   "batch": c.batch, "undone": c.undone})
        })
        .collect();
    let mut out = row_json(&t, &names);
    out["body"] = json!(t.body);
    out["excerpt"] = json!(t.excerpt);
    out["note"] = json!(t.note);
    out["fingerprint"] = json!(t.fingerprint);
    out["muted"] = json!(muted);
    out["history"] = json!(history);
    Ok(out)
}

fn entry_set(d: &Daemon, ctx: &Ctx, c: &Call) -> Result<Value, Fail> {
    let id = number(c)?;
    let field = c
        .pos
        .get(2)
        .map(String::as_str)
        .filter(|f| FIELDS.contains(f) && *f != "assignee")
        .ok_or_else(|| Fail::hint("which field?", FIELDS.join("|")))?;
    let raw =
        if field == "body" { ctx.secret.clone().unwrap_or_default() } else { c.val("value").unwrap_or("").into() };
    let value = valid(field, &raw).ok_or_else(|| Fail::new(format!("'{}' is not a {field}", oneline(&raw, 40))))?;
    let (who, _) = author(ctx);
    let batch = format!("ui:{id}:{}", now());
    let t = with_store(d, |s| {
        if field == "wait" && !value.is_empty() {
            s.entry_set(id, "status", "waiting", &who, &batch)?;
        }
        s.entry_set(id, field, &value, &who, &batch)
    })?
    .ok_or_else(|| missing(id))?;
    let names = who_names(d, vec![t.assignee.clone()]);
    Ok(row_json(&t, &names))
}

fn entry_add(d: &Daemon, ctx: &Ctx, c: &Call) -> Result<Value, Fail> {
    let pick = |field: &str, fallback: &str| -> Result<String, Fail> {
        let raw = c.val(field).map(str::trim).filter(|v| !v.is_empty()).unwrap_or(fallback);
        valid(field, raw).ok_or_else(|| Fail::new(format!("'{raw}' is not a {field}")))
    };
    let task = pick("task", "").map_err(|_| Fail::hint("what is it?", USAGE))?;
    let (kind, priority, status) = (pick("kind", "task")?, pick("priority", "mid")?, pick("status", "todo")?);
    let project = c.val("project").unwrap_or("").trim().to_string();
    let body = ctx.secret.clone().unwrap_or_default();
    let (who, source) = author(ctx);
    let new = NewEntry {
        project: &project,
        task: &task,
        kind: &kind,
        priority: &priority,
        status: &status,
        source,
        body: &body,
        who: &who,
        assignee: if status == "doing" { who.as_str() } else { "" },
        ..NewEntry::default()
    };
    let t = with_store(d, |s| {
        let id = s.entry_add(&new)?;
        s.todo(id)
    })?
    .ok_or_else(|| Fail::new("the new entry is gone"))?;
    Ok(row_json(&t, &HashMap::new()))
}

fn entry_undo(d: &Daemon, c: &Call) -> Result<Value, Fail> {
    let id = number(c)?;
    let undone = with_store(d, |s| s.undo(id))?;
    Ok(json!({"undone": undone.len()}))
}

fn entry_note(d: &Daemon, c: &Call) -> Result<Value, Fail> {
    let id = number(c)?;
    let text = c.val("value").unwrap_or("").to_string();
    if text.trim().is_empty() {
        return Err(Fail::hint("the note is empty", USAGE));
    }
    let t = with_store(d, |s| s.todo_note(id, &text))?.ok_or_else(|| missing(id))?;
    Ok(json!({"id": t.id, "note": t.note}))
}

fn mute(d: &Daemon, ctx: &Ctx, c: &Call) -> Result<Value, Fail> {
    let id = number(c)?;
    let (who, _) = author(ctx);
    let t = with_store(d, |s| s.mute(id, &who))?.ok_or_else(|| missing(id))?;
    Ok(json!({"id": t.id, "status": t.status}))
}

fn unmute(d: &Daemon, c: &Call) -> Result<Value, Fail> {
    let fingerprint = c.pos.get(1).map(String::as_str).unwrap_or("").trim().to_string();
    if fingerprint.is_empty() {
        return Err(Fail::hint("which one?", USAGE));
    }
    let gone = with_store(d, |s| s.unmute(&fingerprint))?;
    Ok(json!({"unmuted": gone}))
}

fn server(d: &Daemon, c: &Call) -> Result<Value, Fail> {
    let name = c.pos.get(1).map(String::as_str).unwrap_or("");
    let what = c.pos.get(2).map(String::as_str).unwrap_or("");
    let said = match what {
        "on" | "off" => with_store(d, |s| Ok(servers::set(&d.home, s, name, "enabled", what)))??,
        "log" => {
            let path = c.val("value").unwrap_or("").to_string();
            with_store(d, |s| Ok(servers::set(&d.home, s, name, "log", &path)))??
        }
        "check" => {
            let only = Some(name).filter(|n| !n.is_empty() && *n != "all");
            with_store(d, |s| Ok(servers::check_now(&d.home, s, only)))??
        }
        _ => return Err(Fail::hint("on, off, check or log?", USAGE)),
    };
    let rows = with_store(d, |s| Ok(servers::rows(&d.home, s)))?;
    Ok(json!({"said": said, "servers": rows}))
}

pub fn board(d: &Daemon, ctx: &Ctx, c: &Call) -> Option<Result<Value, Fail>> {
    let what = c.pos.first().map(String::as_str).unwrap_or("");
    Some(match what {
        "list" => list(d, c),
        "entry" => entry(d, c),
        "entry-set" => entry_set(d, ctx, c),
        "entry-add" => entry_add(d, ctx, c),
        "entry-undo" => entry_undo(d, c),
        "entry-note" => entry_note(d, c),
        "mute" => mute(d, ctx, c),
        "unmute" => unmute(d, c),
        "server" => server(d, c),
        _ => return None,
    })
}
