use std::path::Path;

use serde_json::{Value, json};
use wisply_core::{Call, Fail};
use wisply_memory::Note;

use super::super::routines::{is_skill, load_all, weak};
use super::super::{Daemon, config};
use super::{chain, learner, room, with_store};

const USES_SHOWN: usize = 12;
const POLL_MS: u64 = 3_000;

pub fn board(d: &Daemon, c: &Call) -> Option<Result<Value, Fail>> {
    Some(match c.pos.first().map(String::as_str)? {
        "headroom" => room::board(&d.home, c.pos.get(1).map_or(room::DEFAULT_SPAN, String::as_str)),
        "learning" => learning(d),
        "skills" => Ok(skills(&d.home)),
        _ => return None,
    })
}

fn files_of(raw: &str) -> Vec<String> {
    serde_json::from_str::<Vec<String>>(raw).unwrap_or_default()
}

pub fn note(d: &Daemon, n: &Note) -> Result<Value, Fail> {
    let (origin, via, uses, counts) = with_store(d, |s| {
        Ok((s.origin(&n.session)?, s.via(n.id)?, s.uses_of(n.id, USES_SHOWN)?, s.use_counts(n.id)?))
    })?;
    let origin = origin.map(|o| {
        json!({"session": o.id, "title": o.title, "project": o.project, "cwd": o.cwd, "ai": o.agent,
               "started": o.started, "last": o.last})
    });
    let uses: Vec<Value> = uses
        .iter()
        .map(|u| {
            json!({"session": u.session, "how": u.how, "n": u.n, "t": u.t, "title": u.title,
                   "project": u.project, "cwd": u.cwd, "ai": u.agent})
        })
        .collect();
    let counts: Vec<Value> =
        counts.iter().map(|(how, sessions, times)| json!({"how": how, "sessions": sessions, "times": times})).collect();
    Ok(json!({"origin": origin, "session": n.session, "via": via, "source": n.source, "kind": n.kind,
              "created": n.created, "updated": n.updated, "hits": n.hits, "files": files_of(&n.files),
              "file": n.path().unwrap_or(""), "uses": uses, "counts": counts}))
}

fn learning(d: &Daemon) -> Result<Value, Fail> {
    let st = learner::state();
    let queued = with_store(d, |s| s.counts())?.queued;
    let cfg = config::load(&d.home);
    let now = st.now.map(|n| {
        json!({"session": n.sid, "title": n.title, "project": n.project, "via": n.via, "part": n.part,
               "since": n.since})
    });
    let done: Vec<Value> = st
        .done
        .iter()
        .map(|x| json!({"session": x.sid, "title": x.title, "project": x.project, "added": x.added, "t": x.t}))
        .collect();
    Ok(json!({"on": config::on(&cfg, "memory"), "auto": config::on(&cfg, "memory_learn"), "queued": queued,
              "learned": st.learned, "last": st.last, "error": st.error, "now": now, "done": done,
              "chain": chain::report(&d.home), "poll_ms": POLL_MS}))
}

fn skill_row(r: &Value) -> Value {
    let (st, l) = (&r["stats"], &r["learn"]);
    let params: Vec<&String> = r["params"].as_object().map(|p| p.keys().collect()).unwrap_or_default();
    json!({"name": r["name"], "about": l["about"], "state": l["state"], "guide": l["guide"], "params": params,
           "runs": st["runs"], "ok": st["ok"], "fail": st["fail"], "ms": st["ms"], "saved": st["saved"],
           "before_ms": l["before_ms"], "default": l["default"] == true, "weak": weak(r), "hint": l["hint"],
           "practice": l["practice"]})
}

fn skills(home: &Path) -> Value {
    let rows: Vec<Value> = load_all(home).iter().filter(|r| is_skill(r)).map(skill_row).collect();
    json!({"skills": rows})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_skill_row_carries_what_the_page_shows() {
        let r = json!({"name": "pr-ci", "params": {"pr": "", "repo": ""},
                       "stats": {"runs": 6, "ok": 2, "fail": 4, "ms": 900},
                       "learn": {"state": "learned", "about": "checks a PR", "before_ms": 60000, "default": true}});
        let row = skill_row(&r);
        assert_eq!(row["params"], json!(["pr", "repo"]));
        assert_eq!((row["default"].as_bool(), row["weak"].as_bool()), (Some(true), Some(true)));
        assert_eq!(files_of("[\"a.rs\",\"b.rs\"]"), ["a.rs", "b.rs"]);
        assert!(files_of("").is_empty());
    }
}
