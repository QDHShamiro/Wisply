use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use serde_json::{Value, json};
use wisply_core::Call;

use crate::files::{self, Pack};
use crate::fix::{self, Plan};
use crate::prove::{build_proof, ok, test_proof, tests_after};
use crate::rules::{self, Finding, Scope};
use crate::run::{Home, Run, int, text};
use crate::{Env, measure, personal, safety, skill};

const SCAN_SHOWN: usize = 25;
const ALL_MAX: usize = 500;

struct Scanned {
    found: Vec<Finding>,
    packs: BTreeSet<Pack>,
    checked: BTreeSet<String>,
    total: usize,
    left: usize,
}

impl Scanned {
    fn complete(&self) -> bool {
        self.left == 0 && self.total < files::WALK_MAX
    }
}

fn findings(env: &Env, r: &Run) -> Scanned {
    let root = r.root();
    let list = files::walk(&r.paths());
    let packs = env.packs(&root, &list);
    let mine = personal::load(&env.user, &root, &env.profile_packs());
    let scope = Scope { root: &root, packs: &packs, rules: &mine };
    let rules::Report { found, checked, left } = rules::scan(&scope, &list);
    Scanned { found, packs, checked, total: list.len(), left }
}

fn as_idea(f: &Finding) -> Value {
    json!({"title": format!("{}: {}", f.rule, f.why), "lens": f.pack.name(), "tool": f.rule, "impact": f.impact,
           "sure": f.sure, "effort": f.effort, "kind": "safe", "by": "wisply",
           "place": f.place(), "fix": f.fix, "fixer": f.fixer, "key": f.key()})
}

fn idea_key(i: &Value) -> String {
    let (key, place, rule) = (text(i, "key"), text(i, "place"), text(i, "tool"));
    if !key.is_empty() || place.is_empty() || text(i, "by") != "wisply" {
        return key.to_string();
    }
    let file = place
        .rsplit_once(':')
        .filter(|(_, n)| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
        .map_or(place, |(f, _)| f);
    let why = text(i, "title").strip_prefix(&format!("{rule}: ")).unwrap_or_default();
    rules::key_of(rule, file, why)
}

fn close_gone(r: &mut Run, s: &Scanned) -> usize {
    if !s.complete() {
        return 0;
    }
    let now: BTreeSet<String> = s.found.iter().map(Finding::key).collect();
    let gone: Vec<i64> = r
        .ideas()
        .iter()
        .filter(|i| {
            let key = idea_key(i);
            let file = key.split('|').nth(1).unwrap_or_default();
            let checked = Pack::from_name(text(i, "lens")).is_some_and(|p| s.packs.contains(&p));
            text(i, "status") == "new" && !key.is_empty() && checked && s.checked.contains(file) && !now.contains(&key)
        })
        .map(|i| int(i, "id"))
        .collect();
    for id in &gone {
        mark(r, *id, "done", "proof", "wisply scan: the finding is gone");
    }
    gone.len()
}

pub fn scan(env: &Env, home: &Home, c: &Call) -> Result<String, String> {
    let mut r = home.load(c.val("run"))?;
    if r.paths().is_empty() {
        return Err("this run has no path; start it with one: wisply think start \"<name>\" <path>".into());
    }
    let scanned = findings(env, &r);
    let closed = close_gone(&mut r, &scanned);
    let stopped = (!scanned.complete()).then(|| {
        format!(
            "stopped early: {} of {}{} files checked (limits: {} s, {} files) -> open ideas stay open; scan a smaller folder for the rest",
            scanned.total - scanned.left,
            scanned.total,
            if scanned.total >= files::WALK_MAX { "+" } else { "" },
            rules::SCAN_BUDGET.as_secs(),
            files::WALK_MAX
        )
    });
    let Scanned { found, packs, .. } = scanned;
    let room = if c.on("all") { ALL_MAX } else { (int(&r.s, "idea_goal").max(10) / 2) as usize };
    let mut listed: BTreeSet<String> = r.ideas().iter().map(idea_key).collect();
    let (mut added, mut known) = (0, 0);
    for f in &found {
        if added >= room {
            break;
        }
        if !listed.insert(f.key()) {
            known += 1;
            continue;
        }
        if r.add_idea(as_idea(f), true).is_ok() {
            added += 1;
        }
    }
    r.map().insert("packs".into(), json!(packs.iter().map(|p| p.name()).collect::<Vec<_>>()));
    r.save()?;
    let mut by: BTreeMap<&str, usize> = BTreeMap::new();
    for f in &found {
        *by.entry(f.pack.name()).or_default() += 1;
    }
    let counts: Vec<String> = by.iter().map(|(k, v)| format!("{k} {v}")).collect();
    let fixable = found.iter().filter(|f| f.fixer.is_some()).count();
    let mut out = vec![format!(
        "[scan] {} findings ({}) | checks: {} | {fixable} Wisply can fix itself",
        found.len(),
        if counts.is_empty() { "none".to_string() } else { counts.join(", ") },
        crate::flow::packs_line(&packs)
    )];
    for f in found.iter().take(SCAN_SHOWN) {
        out.push(format!(
            "  {} {}: {} -> {}{}",
            f.place(),
            f.rule,
            f.why,
            f.fix,
            if f.fixer.is_some() { " [fix]" } else { "" }
        ));
    }
    if found.len() > SCAN_SHOWN {
        out.push(format!("  ... +{} more (wisply think idea list --all)", found.len() - SCAN_SHOWN));
    }
    out.extend(stopped);
    out.push(format!(
        "+{added} ideas from wisply{}{}{}",
        if closed > 0 { format!(", {closed} closed (finding gone)") } else { String::new() },
        if known > 0 { format!(", {known} already listed") } else { String::new() },
        if found.len() > added + known {
            let more = if c.on("all") { String::new() } else { format!(" (--all adds up to {ALL_MAX})") };
            format!(", {} left out{more}", found.len() - added - known)
        } else {
            String::new()
        }
    ));
    if fixable > 0 {
        out.push("next: wisply think fix (backs up, fixes, proves with the build, reverts when worse)".into());
    }
    Ok(out.join("\n"))
}

pub fn fix(env: &Env, home: &Home, c: &Call) -> Result<String, String> {
    let mut r = home.load(c.val("run"))?;
    if r.paths().is_empty() {
        return Err("this run has no path to fix".into());
    }
    let Scanned { found, packs, .. } = findings(env, &r);
    let root = r.root();
    let plans: Vec<Plan> = fix::plans(&root, &found, &packs, c.on("deep"), &r.paths());
    if plans.is_empty() {
        return Ok("nothing Wisply can fix itself; the findings are ideas for Claude (wisply think idea list)".into());
    }
    let before_ok = r.s.pointer("/measures/before").is_none_or(ok);
    let tests_before = test_proof(&r);
    let (mut out, mut kept) = (Vec::new(), Vec::new());
    for plan in plans {
        let id = r.add_idea(
            json!({"title": plan.title, "lens": "fix", "tool": plan.fixer, "impact": 1, "sure": 3, "effort": 1,
                   "kind": "safe", "by": "wisply", "fixer": plan.fixer}),
            true,
        )?;
        r.save()?;
        if let Err(e) = safety::checkpoint(&r, id, &plan.files) {
            set(&mut r, id, "dropped", "why", &e);
            out.push(format!("#{id} {}: not started ({e})", plan.title));
            continue;
        }
        let did = match fix::apply(&plan, &root) {
            Ok(d) => d,
            Err(e) => {
                let _ = safety::revert(&r, id, true);
                set(&mut r, id, "dropped", "why", &e);
                out.push(format!("#{id} {}: failed, reverted ({e})", plan.title));
                continue;
            }
        };
        let proof = if plan.fixer == "png" { None } else { build_proof(&r) };
        match proof {
            Some(m) if m.get("skipped").is_some() => {
                set(&mut r, id, "unchecked", "proof", &did);
                safety::mark_after(&r, id);
                kept.push(id);
                out.push(format!("#{id} {}: {did}; build not run ({}) -> unchecked", plan.title, text(&m, "skipped")));
            }
            Some(m) if !ok(&m) && before_ok => {
                let _ = safety::revert(&r, id, true);
                let why = format!("build broke after the fix: {}", measure::line(&m));
                set(&mut r, id, "dropped", "why", &why);
                out.push(format!("#{id} {}: {why} -> reverted", plan.title));
            }
            Some(m) if !ok(&m) => {
                let p = format!("{did}; build was red before and still is: {}", measure::line(&m));
                set(&mut r, id, "unchecked", "proof", &p);
                safety::mark_after(&r, id);
                kept.push(id);
                out.push(format!("#{id} {}: {p} -> unchecked", plan.title));
            }
            Some(m) => {
                let p = format!("{did}; {}: {}", text(&m, "cmd"), measure::line(&m));
                set(&mut r, id, "done", "proof", &p);
                safety::mark_after(&r, id);
                covered(&mut r, id, plan.fixer);
                kept.push(id);
                out.push(format!("#{id} {}: {p}", plan.title));
            }
            None => {
                set(&mut r, id, "done", "proof", &did);
                safety::mark_after(&r, id);
                covered(&mut r, id, plan.fixer);
                kept.push(id);
                out.push(format!("#{id} {}: {did}", plan.title));
            }
        }
    }
    if let Some(before) = tests_before.filter(|_| !kept.is_empty()) {
        out.extend(tests_after(&mut r, &before, &kept));
    }
    r.save()?;
    out.push("next: wisply think diff, then review the change".into());
    Ok(out.join("\n"))
}

fn covered(r: &mut Run, by: i64, fixer: &str) {
    let ids: Vec<i64> = r
        .ideas()
        .iter()
        .filter(|i| int(i, "id") != by && text(i, "fixer") == fixer && text(i, "status") == "new")
        .map(|i| int(i, "id"))
        .collect();
    for id in ids {
        set(r, id, "done", "proof", &format!("fixed by #{by}"));
    }
}

pub(crate) fn mark(r: &mut Run, id: i64, status: &str, key: &str, val: &str) {
    let round = r.round();
    if let Some(i) = r.idea_mut(id) {
        i["status"] = json!(status);
        i[key] = json!(val);
        i["round_status"] = json!(round);
    }
}

fn set(r: &mut Run, id: i64, status: &str, key: &str, val: &str) {
    mark(r, id, status, key, val);
    let _ = r.save();
}

pub fn checkpoint(env: &Env, home: &Home, c: &Call) -> Result<String, String> {
    let r = home.load(c.val("run"))?;
    let id: i64 = c
        .pos
        .get(1)
        .and_then(|s| s.trim_start_matches('#').parse().ok())
        .ok_or("wisply think checkpoint <idea id> <files...>")?;
    let list: Vec<PathBuf> = c.pos.iter().skip(2).map(|p| env.path(p)).collect();
    if list.is_empty() {
        return Err("which files? wisply think checkpoint <id> <files...>".into());
    }
    let (added, kept) = safety::checkpoint(&r, id, &list)?;
    Ok(format!(
        "checkpoint #{id}: {}{}",
        if added.is_empty() { "nothing new".to_string() } else { added.join(", ") },
        if kept.is_empty() { String::new() } else { format!(" | already saved: {}", kept.join(", ")) }
    ))
}

pub fn revert(home: &Home, c: &Call) -> Result<String, String> {
    let mut r = home.load(c.val("run"))?;
    let id: i64 = c
        .pos
        .get(1)
        .and_then(|s| s.trim_start_matches('#').parse().ok())
        .ok_or("wisply think revert <idea id> --why \"...\"")?;
    let (done, later) = safety::revert(&r, id, c.on("force"))?;
    let why = c.val("why").unwrap_or("not better");
    set(&mut r, id, "dropped", "why", why);
    let mut out = format!(
        "#{id} reverted: {} -> dropped",
        if done.is_empty() { "nothing to do".to_string() } else { done.join(", ") }
    );
    if !later.is_empty() {
        out.push_str(&format!("\nWARNING overwritten (--force): {} - check those ideas again", later.join(", ")));
    }
    Ok(out)
}

pub fn restore(home: &Home, c: &Call) -> Result<String, String> {
    let r = home.load(c.val("run"))?;
    if !c.on("yes") {
        return Err("puts EVERYTHING back to the start. Only when the user says so: wisply think restore --yes".into());
    }
    let (n, removed) = safety::restore_all(&r)?;
    Ok(format!("start state restored: {n} files, {removed} new files removed"))
}

pub fn diff(home: &Home, c: &Call) -> Result<String, String> {
    let r = home.load(c.val("run"))?;
    let label = c.pos.get(1).cloned().unwrap_or_else(|| format!("r{}", r.round()));
    let (file, n, plus, minus) = safety::diff(&r, &label)?;
    Ok(format!(
        "diff: {} ({n} files, +{plus} -{minus}) - read it line by line against the checklist",
        files::slashed(&file)
    ))
}

pub fn profile(env: &Env, c: &Call) -> Result<String, String> {
    let file = env.home.join("think").join("profile.json");
    let mut p: Value = fs::read_to_string(&file).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(json!({}));
    match c.pos.get(1).map(String::as_str) {
        Some("set") => {
            let list: Vec<String> =
                c.pos[2..].join(",").split(',').map(str::trim).filter(|s| !s.is_empty()).map(str::to_string).collect();
            let bad: Vec<&String> = list.iter().filter(|s| Pack::from_name(s).is_none()).collect();
            if !bad.is_empty() || list.is_empty() {
                let names: Vec<&str> = Pack::ALL.iter().map(|p| p.name()).collect();
                return Err(format!("packs are {}", names.join(", ")));
            }
            p["set"] = json!(list);
        }
        Some("auto") => {
            p.as_object_mut().map(|m| m.remove("set"));
        }
        _ => {}
    }
    let counts = env.person();
    p["counts"] =
        json!(counts.iter().map(|(k, v)| (k.name().to_string(), json!(v))).collect::<serde_json::Map<_, _>>());
    let _ = fs::create_dir_all(file.parent().unwrap_or(&env.home));
    fs::write(&file, serde_json::to_string_pretty(&p).unwrap_or_default()).map_err(|e| e.to_string())?;
    let total: usize = counts.values().sum::<usize>().max(1);
    let mut rows: Vec<(&Pack, &usize)> = counts.iter().collect();
    rows.sort_by(|a, b| b.1.cmp(a.1));
    let seen: Vec<String> = rows.iter().map(|(k, v)| format!("{} {} %", k.name(), **v * 100 / total)).collect();
    let set: Vec<&str> = p
        .get("set")
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_str).collect())
        .unwrap_or_default();
    Ok(format!(
        "you build: {}\nchecks used: {}\n(wisply think profile set web,rust limits the checks; profile auto follows your projects again)",
        if seen.is_empty() { "nothing seen yet".to_string() } else { seen.join(", ") },
        if set.is_empty() {
            "whatever each project contains (auto)".to_string()
        } else {
            format!("only {} + general code, docs, images", set.join(", "))
        }
    ))
}

pub fn rules_cmd(env: &Env) -> String {
    let root = env.cwd.clone();
    let list = personal::load(&env.user, &files::root_of(&root), &env.profile_packs());
    if list.is_empty() {
        return "no personal rules found (CLAUDE.md files and feedback memories are read)".into();
    }
    let mut out = vec![format!("{} personal rules Wisply checks:", list.len())];
    out.extend(list.iter().map(|r| format!("  {}", personal::describe(r))));
    out.join("\n")
}

pub fn lint_skill(env: &Env, c: &Call) -> Result<String, String> {
    let dir = env.path(c.pos.get(1).ok_or("wisply think lint-skill <skill folder>")?);
    let (errs, warns) = skill::lint(&dir);
    let mut out: Vec<String> = errs.iter().map(|e| format!("ERROR: {e}")).collect();
    out.extend(warns.iter().map(|w| format!("WARNING: {w}")));
    out.push(if errs.is_empty() && warns.is_empty() {
        "OK".into()
    } else {
        format!("{} errors, {} warnings", errs.len(), warns.len())
    });
    Ok(out.join("\n"))
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use crate::flow;
    use crate::prove::worse;

    fn env_in(base: &std::path::Path, cwd: &std::path::Path) -> Env {
        Env {
            home: base.join("wisply"),
            user: base.join("user"),
            sid: "s".into(),
            cwd: cwd.to_path_buf(),
            playing: String::new(),
            cwds: Vec::new(),
            notes: RefCell::new(Vec::new()),
        }
    }

    fn call(args: &[&str]) -> Call {
        let argv: Vec<String> = std::iter::once("think").chain(args.iter().copied()).map(str::to_string).collect();
        wisply_core::parse(&argv)
    }

    #[test]
    fn scan_lists_each_finding_once_and_closes_what_is_gone() {
        let base = std::env::temp_dir().join(format!("wisply-think-scan-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let proj = base.join("proj");
        fs::create_dir_all(proj.join(".git")).unwrap();
        let doc = proj.join("README.md");
        fs::write(&doc, "# A\n\nThis is is wrong.\nSee [gone](gone.md).\n").unwrap();
        let env = env_in(&base, &proj);
        let home = Home::new(&env.home, "s");
        flow::start(&env, &home, &call(&["start", "t", &files::slashed(&proj)])).unwrap();
        let first = scan(&env, &home, &call(&["scan"])).unwrap();
        assert!(first.contains("+2 ideas from wisply"), "{first}");
        let again = scan(&env, &home, &call(&["scan"])).unwrap();
        assert!(again.contains("+0 ideas from wisply, 2 already listed"), "{again}");
        let mut r = home.load(None).unwrap();
        let link = r.ideas().iter().find(|i| text(i, "tool") == "broken-link").map(|i| int(i, "id")).unwrap();
        r.idea_mut(link).unwrap().as_object_mut().unwrap().remove("key");
        r.save().unwrap();
        fs::write(&doc, "# A\n\nThis is wrong.\nSee [gone](gone.md) and [also](also.md).\n").unwrap();
        let fixed = scan(&env, &home, &call(&["scan"])).unwrap();
        assert!(fixed.contains("+0 ideas from wisply, 1 closed (finding gone), 1 already listed"), "{fixed}");
        let r = home.load(None).unwrap();
        let status: Vec<(String, String)> =
            r.ideas().iter().map(|i| (text(i, "tool").to_string(), text(i, "status").to_string())).collect();
        assert!(status.contains(&("repeated-word".into(), "done".into())), "{status:?}");
        assert!(status.contains(&("broken-link".into(), "new".into())), "{status:?}");
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn fixes_that_make_the_tests_worse_are_taken_back() {
        let a = json!({"exit": 0, "tests": {"passed": 5, "total": 5}});
        let b = json!({"exit": 1, "tests": {"passed": 4, "total": 5}});
        let c = json!({"exit": 1, "tests": {"passed": 3, "total": 5}});
        assert!(worse(&a, &b) && worse(&b, &c) && !worse(&b, &b) && !worse(&b, &a));
        if measure::run("python --version", std::path::Path::new("."), std::time::Duration::from_secs(30)).code != 0 {
            return;
        }
        let base = std::env::temp_dir().join(format!("wisply-think-fixtest-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let proj = base.join("skill");
        fs::create_dir_all(proj.join("scripts")).unwrap();
        fs::write(proj.join("SKILL.md"), "---\nname: t\ndescription: test skill\n---\n\nBody.\n").unwrap();
        fs::write(proj.join("data.yml"), "a: 1  \n").unwrap();
        let check = "import sys\nok = open('data.yml').read().startswith('a: 1  ')\nprint('1/1 green' if ok else '0/1 green')\nsys.exit(0 if ok else 1)\n";
        fs::write(proj.join("scripts/selftest.py"), check).unwrap();
        let env = env_in(&base, &proj);
        let home = Home::new(&env.home, "s");
        flow::start(&env, &home, &call(&["start", "t", &files::slashed(&proj)])).unwrap();
        let said = fix(&env, &home, &call(&["fix"])).unwrap();
        assert!(said.contains("reverted (tests got worse)") && said.contains("tests now: OK"), "{said}");
        assert_eq!(fs::read_to_string(proj.join("data.yml")).unwrap(), "a: 1  \n");
        let r = home.load(None).unwrap();
        assert!(r.ideas().iter().any(|i| text(i, "fixer") == "spaces" && text(i, "status") == "dropped"));
        let _ = fs::remove_dir_all(&base);
    }
}
