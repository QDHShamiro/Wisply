use std::collections::HashMap;

use wisply_core::{Call, Fail};
use wisply_memory::{KINDS, NewEntry, PRIOS, Todo, WAITS, date, day, oneline, valid};

use super::super::ctx::Ctx;
use super::super::{Daemon, agents, servers};
use super::{LINE, LONG_TODOS, TASK_SHORT, ident, is_ident, project, todo_digest, todo_list, with_store};

pub const TODO_HELP: &str = "wisply todo | todo add \"task\" [--kind bug|feature|task|idea] [--prio high|mid|low] [--list name] \
     | todo done|doing|drop|open <id> [note] | todo wait <id> restart|you | todo prio|kind <id> <value> \
     | todo note <id> \"text\" | todo undo <id> | todo tidy [--done 3,7] [--drop 4] | todo --full (every line of a long list)";
const BUG_HELP: &str = "wisply bug add <server> \"what is broken\" [--prio high|mid|low] | wisply bug [server]";
const LIST_HELP: &str = "wisply list [--server s] [--kind bug] [--warn] [--done] | list servers | list check [server] \
     | list ignore <id> | list on|off <server> | list log <server> <path> | list accept|reject <id>";
const HISTORY_SHOWN: usize = 6;
const BODY_SHOWN: usize = 600;
const SID_MIN: usize = 16;

pub fn author(ctx: &Ctx) -> (String, &'static str) {
    let sid = ctx.sid.trim();
    if sid.is_empty() || sid == wisply_proto::WINDOW_SID { ("you".into(), "hand") } else { (sid.to_string(), "ai") }
}

fn missing(which: &str) -> Fail {
    Fail::hint(format!("no to-do #{}", which.trim_start_matches('#')), "wisply todo")
}

fn checked(field: &str, value: &str) -> Result<String, Fail> {
    let options = match field {
        "kind" => KINDS.join("|"),
        "priority" => PRIOS.join("|"),
        "wait" => WAITS.join("|"),
        _ => String::new(),
    };
    valid(field, value).ok_or_else(|| Fail::hint(format!("'{value}' is not a {field}"), options))
}

pub fn names(d: &Daemon, rows: &[Todo]) -> HashMap<String, String> {
    who_names(d, rows.iter().map(|t| t.assignee.clone()).collect())
}

pub fn who_names(d: &Daemon, ids: Vec<String>) -> HashMap<String, String> {
    let mut ids: Vec<String> = ids.into_iter().filter(|a| a.len() > SID_MIN).collect();
    ids.sort();
    ids.dedup();
    let titles = with_store(d, |s| s.run_titles(&ids)).unwrap_or_default();
    ids.into_iter()
        .map(|id| {
            let name = titles
                .get(&id)
                .map(|(_, t)| t.clone())
                .filter(|t| !t.is_empty())
                .unwrap_or_else(|| id.chars().take(8).collect());
            (id, name)
        })
        .collect()
}

fn tag(t: &Todo, who: &HashMap<String, String>) -> String {
    match t.status.as_str() {
        "doing" => match who.get(&t.assignee) {
            Some(name) => format!("doing ({})", oneline(name, 30)),
            None if t.assignee == "you" => "doing (you)".into(),
            None => "doing".into(),
        },
        "waiting" if !t.wait.is_empty() => format!("waiting: {}", t.wait),
        other => other.to_string(),
    }
}

pub fn row(t: &Todo, who: &HashMap<String, String>) -> String {
    let place = if t.project.is_empty() { String::new() } else { format!(" [{}]", t.project) };
    let times = if t.count > 1 { format!(" x{}", t.count) } else { String::new() };
    let level = if t.level.is_empty() { String::new() } else { format!(" {}", t.level) };
    format!("#{} {} {} {}{level}{place} {}{times}", t.id, t.kind, t.priority, tag(t, who), oneline(&t.task, LINE))
}

fn full(d: &Daemon, t: &Todo) -> String {
    let who = names(d, std::slice::from_ref(t));
    let project = if t.project.is_empty() { "global" } else { &t.project };
    let list = if t.list.is_empty() { String::new() } else { format!(", {}", t.list) };
    let mut out =
        vec![format!("#{} {} {} {} {} [{project}{list}]", t.id, date(t.updated), t.kind, t.priority, tag(t, &who))];
    out.push(t.task.clone());
    if !t.server.is_empty() || t.count > 1 {
        out.push(format!(
            "server {} | seen {}x | first {} | last {}",
            t.server,
            t.count,
            day(t.first_seen),
            day(t.last_seen)
        ));
    }
    if !t.body.trim().is_empty() {
        out.push(oneline(&t.body, BODY_SHOWN));
    }
    if !t.excerpt.trim().is_empty() {
        out.push("log:".into());
        out.extend(t.excerpt.lines().take(8).map(|l| format!("  {}", oneline(l, LINE))));
    }
    let notes: Vec<&str> = t.note.split('\n').filter(|l| !l.trim().is_empty()).collect();
    if !notes.is_empty() {
        out.push(format!("notes:\n  {}", notes.join("\n  ")));
    }
    let id = t.id;
    if let Ok(h) = with_store(d, |s| s.history(id)) {
        let shown: Vec<String> = h
            .iter()
            .filter(|c| !c.undone)
            .take(HISTORY_SHOWN)
            .map(|c| {
                format!(
                    "  {} {} {}: {} -> {}",
                    day(c.at),
                    c.who.chars().take(8).collect::<String>(),
                    c.field,
                    oneline(&c.old, 30),
                    oneline(&c.new, 30)
                )
            })
            .collect();
        if !shown.is_empty() {
            out.push(format!("history (wisply todo undo {id}):"));
            out.extend(shown);
        }
    }
    out.join("\n")
}

fn alert_new(d: &Daemon, t: &Todo, who: &str) {
    if t.priority == "high" && who != "you" {
        agents::alert(&d.home, &format!("{}: {}", t.kind, oneline(&t.task, 60)), &t.project);
    }
}

pub(super) fn set(d: &Daemon, ctx: &Ctx, which: &str, field: &str, value: &str, note: &str) -> Result<Todo, Fail> {
    let id = ident(which)?;
    let value = checked(field, value)?;
    let (who, _) = author(ctx);
    let t = with_store(d, |s| {
        if field == "wait" {
            s.entry_set(id, "status", "waiting", &who, "")?;
        }
        let t = s.entry_set(id, field, &value, &who, "")?;
        if !note.is_empty() && t.is_some() {
            return s.todo_note(id, note);
        }
        Ok(t)
    })?;
    t.ok_or_else(|| missing(which))
}

pub fn todo(d: &Daemon, ctx: &Ctx, c: &Call) -> Result<String, Fail> {
    let raw = c.pos.first().map(|s| s.to_lowercase()).unwrap_or_default();
    let listing = matches!(raw.as_str(), "list" | "ls" | "show");
    let named = c.pos.get(1).filter(|_| listing);
    let sub = match raw.as_str() {
        "list" | "ls" | "show" => named.filter(|n| is_ident(n)).map_or("", String::as_str),
        "close" | "finish" | "complete" => "done",
        "start" => "doing",
        "rm" | "remove" | "delete" => "drop",
        "reopen" => "open",
        "priority" => "prio",
        other => other,
    };
    if is_ident(sub) {
        let id = ident(sub)?;
        let t = with_store(d, |s| s.todo(id))?;
        return t.map(|t| full(d, &t)).ok_or_else(|| missing(sub));
    }
    let here = project(ctx, c);
    let list = c.val("list").or_else(|| named.map(String::as_str)).unwrap_or("");
    let rest = |from: usize| c.pos.get(from..).map(|r| r.join(" ")).unwrap_or_default();
    match sub {
        "add" => add(d, ctx, c, &here, list, "", &rest(1)),
        "done" | "doing" | "drop" | "dropped" | "todo" | "open" => {
            let Some(which) = c.pos.get(1) else { return Err(Fail::hint("which to-do?", TODO_HELP)) };
            let status = match sub {
                "drop" => "dropped",
                "open" => "todo",
                other => other,
            };
            let t = set(d, ctx, which, "status", status, &rest(2))?;
            Ok(format!("#{} is {}: {}", t.id, t.status, oneline(&t.task, TASK_SHORT)))
        }
        "wait" | "prio" | "kind" => {
            let (Some(which), Some(value)) = (c.pos.get(1), c.pos.get(2)) else {
                return Err(Fail::hint(format!("todo {sub} needs a number and a value"), TODO_HELP));
            };
            let field = if sub == "prio" { "priority" } else { sub };
            let t = set(d, ctx, which, field, value, &rest(3))?;
            Ok(format!("#{} {field} is {}: {}", t.id, t.field(field), oneline(&t.task, TASK_SHORT)))
        }
        "note" => {
            if c.pos.len() < 3 {
                return Err(Fail::hint("todo note needs a number and the text", TODO_HELP));
            }
            let id = ident(&c.pos[1])?;
            let text = rest(2);
            let t = with_store(d, |s| s.todo_note(id, &text))?;
            let t = t.ok_or_else(|| missing(&c.pos[1]))?;
            Ok(format!("#{} noted", t.id))
        }
        "undo" => {
            let Some(which) = c.pos.get(1) else { return Err(Fail::hint("which to-do?", "wisply todo undo 12")) };
            let id = ident(which)?;
            let done = with_store(d, |s| s.undo(id))?;
            if done.is_empty() {
                return Ok(format!("#{id}: nothing to undo"));
            }
            let lines: Vec<String> =
                done.iter().map(|c| format!("#{id} {} back to '{}'", c.field, oneline(&c.old, TASK_SHORT))).collect();
            Ok(lines.join("\n"))
        }
        "tidy" | "stale" => super::stale::tidy(d, ctx, c, &here),
        "" => {
            let scope = if c.on("all") { None } else { Some(here.as_str()) };
            let mut rows = with_store(d, |s| s.todos(scope, true))?;
            if !list.is_empty() {
                rows.retain(|t| t.list == list);
            }
            Ok(if rows.is_empty() {
                "no open to-dos (wisply todo add \"task\")".into()
            } else if list.is_empty() && rows.len() > LONG_TODOS && !c.on("full") {
                todo_digest(&rows)
            } else {
                todo_list(&rows, false)
            })
        }
        other => Err(Fail::hint(format!("unknown todo command '{other}'"), TODO_HELP)),
    }
}

fn add(d: &Daemon, ctx: &Ctx, c: &Call, here: &str, list: &str, server: &str, task: &str) -> Result<String, Fail> {
    if task.trim().is_empty() {
        return Err(Fail::hint("todo add needs the task", TODO_HELP));
    }
    let kind = checked("kind", c.val("kind").unwrap_or(if server.is_empty() { "task" } else { "bug" }))?;
    let priority = checked("priority", c.val("prio").unwrap_or("mid"))?;
    let (who, source) = author(ctx);
    let server = if server.is_empty() { c.val("server").unwrap_or("") } else { server };
    let body = c.val("body").unwrap_or("");
    let new = NewEntry {
        project: here,
        task,
        list,
        kind: &kind,
        priority: &priority,
        source,
        server,
        body,
        who: &who,
        ..NewEntry::default()
    };
    let doing = c.on("doing");
    let t = with_store(d, |s| {
        let id = s.entry_add(&new)?;
        if doing {
            s.entry_set(id, "status", "doing", &who, "")?;
        }
        s.todo(id)
    })?
    .ok_or_else(|| Fail::new("the new to-do is gone"))?;
    alert_new(d, &t, &who);
    let to = if list.is_empty() { String::new() } else { format!(" to {list}") };
    Ok(format!("{} #{} added{to}", if t.kind == "task" { "to-do" } else { t.kind.as_str() }, t.id))
}

pub fn bug(d: &Daemon, ctx: &Ctx, c: &Call) -> Result<String, Fail> {
    match c.pos.first().map(|s| s.to_lowercase()).as_deref() {
        Some("add") => {
            let Some(server) = c.pos.get(1).filter(|s| !s.trim().is_empty()) else {
                return Err(Fail::hint("bug add needs the server and the text", BUG_HELP));
            };
            let task = c.pos.get(2..).map(|r| r.join(" ")).unwrap_or_default();
            if task.trim().is_empty() {
                return Err(Fail::hint("bug add needs the text", BUG_HELP));
            }
            add(d, ctx, c, server, "", server, &task)
        }
        Some("help") => Ok(BUG_HELP.into()),
        other => {
            let scope = other.map(String::from);
            let mut rows = with_store(d, |s| s.entries(scope.as_deref(), false))?;
            rows.retain(|t| t.kind == "bug" && t.open() && t.level != "warn");
            show(d, &rows, "no open bugs (wisply bug add <server> \"text\")")
        }
    }
}

fn show(d: &Daemon, rows: &[Todo], empty: &str) -> Result<String, Fail> {
    if rows.is_empty() {
        return Ok(empty.into());
    }
    let who = names(d, rows);
    let mut sorted: Vec<&Todo> = rows.iter().collect();
    let rank = |p: &str| PRIOS.iter().position(|x| *x == p).unwrap_or(PRIOS.len());
    sorted.sort_by_key(|t| (rank(&t.priority), std::cmp::Reverse(t.updated as i64)));
    Ok(sorted.iter().map(|t| row(t, &who)).collect::<Vec<_>>().join("\n"))
}

pub fn list(d: &Daemon, ctx: &Ctx, c: &Call) -> Result<String, Fail> {
    let sub = c.pos.first().map(|s| s.to_lowercase()).unwrap_or_default();
    let arg = |i: usize| c.pos.get(i).map(String::as_str).unwrap_or("");
    let (who, _) = author(ctx);
    match sub.as_str() {
        "servers" => with_store(d, |s| Ok(servers::status_text(&d.home, s))),
        "check" => {
            let only = Some(arg(1)).filter(|s| !s.is_empty());
            with_store(d, |s| Ok(servers::check_now(&d.home, s, only)))?
        }
        "on" | "off" => with_store(d, |s| Ok(servers::set(&d.home, s, arg(1), "enabled", &sub)))?,
        "log" => {
            let path = c.pos.get(2..).map(|r| r.join(" ")).unwrap_or_default();
            with_store(d, |s| Ok(servers::set(&d.home, s, arg(1), "log", &path)))?
        }
        "ignore" | "mute" => {
            let id = ident(arg(1))?;
            let t = with_store(d, |s| s.mute(id, &who))?.ok_or_else(|| missing(arg(1)))?;
            Ok(format!("#{} ignored for good (wisply todo undo {} brings it back)", t.id, t.id))
        }
        "accept" | "reject" => {
            let id = ident(arg(1))?;
            let status = if sub == "accept" { "todo" } else { "dropped" };
            let t = with_store(d, |s| s.entry_set(id, "status", status, &who, ""))?.ok_or_else(|| missing(arg(1)))?;
            Ok(format!("#{} is {}: {}", t.id, t.status, oneline(&t.task, TASK_SHORT)))
        }
        "help" => Ok(LIST_HELP.into()),
        "" => {
            let scope = c.val("server").or(c.val("project")).map(String::from);
            let mut rows = with_store(d, |s| s.entries(scope.as_deref(), false))?;
            let done = c.on("done");
            let warn = c.on("warn");
            rows.retain(|t| {
                (done || t.open() || t.status == "suggested")
                    && (warn == (t.level == "warn"))
                    && c.val("kind").is_none_or(|k| t.kind == k)
            });
            show(d, &rows, "the list is empty (wisply todo add \"task\" or wisply bug add <server> \"text\")")
        }
        other => Err(Fail::hint(format!("unknown list command '{other}'"), LIST_HELP)),
    }
}
