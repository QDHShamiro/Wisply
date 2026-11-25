mod act;
#[cfg(windows)]
mod assist;
#[cfg(test)]
mod bench;
pub mod cache;
mod contacts;
mod discover;
#[cfg(windows)]
mod draw;
mod hint;
mod history;
#[cfg(windows)]
mod input;
mod later;
mod parse;
mod teach;
mod unknown;
mod words;

use std::path::{Path, PathBuf};
use std::time::Instant;

use wisply_core::{Call, Fail};

use super::ctx::{Ctx, now};
use super::{Daemon, Out, web};
use parse::{Act, Know, Plan, describe, label};

pub const USAGE: &str = "wisply go \"schreib Anna bin gleich da\" [--dry] [--yes] [--now] | go --list | go --cancel N | go --teach \"<order>\" | go --unknown [clear]";

pub fn start(home: PathBuf) {
    #[cfg(windows)]
    input::start(home.clone());
    later::start(home);
}

pub fn hotkey() -> &'static str {
    #[cfg(windows)]
    return input::NAME;
    #[cfg(not(windows))]
    return "none";
}

fn timed(out: &str, t0: Instant) -> String {
    if wisply_proto::handoff(out).is_some() {
        return out.to_string();
    }
    let out = out.trim();
    let took = format!("in {:.1} s", t0.elapsed().as_secs_f64());
    if out.contains('\n') { format!("{out}\n({took})") } else { format!("{out} {took}") }
}

fn cancel(d: &Daemon, c: &Call) -> Result<Out, Fail> {
    let raw = c.val("cancel").or_else(|| c.pos.first().map(String::as_str)).unwrap_or("");
    let id = raw
        .trim_start_matches('#')
        .parse()
        .map_err(|_| Fail::hint("--cancel needs the number from the list", "wisply go --list"))?;
    later::cancel(&d.home, id).map(Out::Text)
}

fn taught(d: &Daemon, ctx: &mut Ctx, said: &str, (dry, yes): (bool, bool), t0: Instant) -> Option<Result<Out, Fail>> {
    let hit = teach::find(&d.home, said)?;
    if dry {
        return Some(Ok(Out::Text(format!("dry: routine {} {}", hit.name, hit.pairs.join(" ")).trim().to_string())));
    }
    let mut argv = vec!["run".to_string(), hit.name];
    argv.extend(hit.pairs);
    if yes {
        argv.push("--yes".into());
    }
    Some(act::reply(d, ctx, &argv).map(|out| match out {
        Out::Text(t) => Out::Text(timed(&t, t0)),
        other => other,
    }))
}

fn parsed(home: &Path, said: &str, t: f64) -> Result<Option<Plan>, Fail> {
    let people = contacts::load(home);
    let person = |w: &str| contacts::knows(&people, w);
    let site = |w: &str| web::site(home, w).is_some();
    parse::parse(said, t, &Know { person: &person, site: &site })
}

fn plan(home: &Path, said: &str, t: f64, learn: bool) -> Result<Plan, Fail> {
    parsed(home, said, t)?.ok_or_else(|| teach::unknown(said, learn.then(|| unknown::note(home, said))))
}

fn unknown_orders(c: &Call) -> Option<bool> {
    if !c.flags.contains_key("unknown") {
        return None;
    }
    let arg = c.val("unknown").or_else(|| c.pos.first().map(String::as_str)).unwrap_or("");
    Some(arg.eq_ignore_ascii_case("clear"))
}

fn shown(home: &Path, plan: Plan, yes: bool, t: f64) -> Result<String, Fail> {
    let who = act::preview(home, &plan.act, yes)?;
    let Act::Msg { text, .. } = &plan.act else { return Ok(describe(&plan, t)) };
    if who.is_empty() {
        return Ok(describe(&plan, t));
    }
    let line = format!("message to {who}: \"{text}\"");
    Ok(match plan.when {
        Some(at) => format!("{line} at {}", label(t, at)),
        None => line,
    })
}

fn dry(home: &Path, said: &str) -> Result<Option<String>, Fail> {
    if let Some(hit) = teach::find(home, said) {
        return Ok(Some(format!("would run routine {} {}", hit.name, hit.pairs.join(" ")).trim().to_string()));
    }
    let t = now();
    let Some(plan) = parsed(home, said, t)? else { return Ok(None) };
    let plan = Plan { when: plan.when.filter(|at| *at > t), act: plan.act };
    Ok(Some(format!("would do: {}", shown(home, plan, false, t)?)))
}

#[cfg_attr(not(windows), allow(dead_code))]
pub fn preview(home: &Path, said: &str) -> String {
    let said = said.trim();
    if said.is_empty() {
        return String::new();
    }
    hint::line(dry(home, said).map_err(|f| f.text()))
}

fn go(d: &Daemon, ctx: &mut Ctx, c: &Call, said: &str) -> Result<Out, Fail> {
    let t0 = Instant::now();
    let (yes, dry) = (c.on("yes"), c.on("dry"));
    if let Some(done) = taught(d, ctx, said, (dry, yes), t0) {
        return done;
    }
    let t = now();
    let plan = plan(&d.home, said, t, !dry)?;
    let later_at = plan.when.filter(|at| !c.on("now") && *at > t);
    if dry {
        let line = shown(&d.home, Plan { act: plan.act, when: later_at }, yes, t)?;
        return Ok(Out::Text(format!("dry: {line}")));
    }
    if let Some(at) = later_at {
        act::preview(&d.home, &plan.act, yes)?;
        let what = describe(&Plan { act: plan.act, when: None }, t);
        let id = later::add(&d.home, at, said, &what)?;
        return Ok(Out::Text(format!("planned #{id} {}: {what}", label(t, at))));
    }
    act::run(d, ctx, &plan.act, yes).map(|out| Out::Text(timed(&out, t0)))
}

pub fn command(d: &Daemon, ctx: &mut Ctx, c: &Call) -> Result<Out, Fail> {
    if c.name != "go" {
        if discover::wants(c) {
            return discover::scan(&d.home, c).map(Out::Text);
        }
        return contacts::command(&d.home, c).map(Out::Text);
    }
    if c.on("list") || c.flags.contains_key("list") {
        return Ok(Out::Text(later::list(&d.home)));
    }
    if c.flags.contains_key("cancel") {
        return cancel(d, c);
    }
    if let Some(clear) = unknown_orders(c) {
        return Ok(Out::Text(if clear { unknown::clear(&d.home) } else { unknown::list(&d.home) }));
    }
    let said = c.pos.join(" ").trim().to_string();
    if said.is_empty() {
        return Err(Fail::hint("say what to do", USAGE));
    }
    if c.on("teach") {
        let out = teach::learn(d, ctx, &said)?;
        unknown::forget(&d.home, &said);
        return Ok(Out::Text(out));
    }
    go(d, ctx, c, &said)
}
