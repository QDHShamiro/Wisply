use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::Regex;
use serde_json::{Map, Value, json};
use wisply_core::{Call, Fail, local_now};

use crate::codemode::files;

use super::super::ctx::Ctx;
use super::super::web::parse_ms;
use super::super::{Daemon, Out};
use super::script::{self, hash};
use super::store::{self, file, load_all, save, slug, text, words};
use super::{defaults, took};

pub const PRACTICE_ROUNDS: usize = 3;
const WEAK_RUNS: u64 = 5;
const WEAK_FAIL: f64 = 0.3;
const WEAK_SPEED: f64 = 0.8;
const OFFER_AT: u64 = 3;
const OFFERS_MAX: u64 = 2;
const HINT_CHARS: usize = 200;
const FIT_WORDS: usize = 2;
const NAME_WORDS: usize = 4;
const CHECK_LINES: usize = 20;
const NOT_PARAMS: [&str; 2] = ["dry", "result"];
const OTHER_LANGS: [(&str, &str); 9] = [
    ("py", "python "),
    ("sh", "bash "),
    ("ps1", "powershell -File "),
    ("bat", "cmd /c "),
    ("cmd", "cmd /c "),
    ("rb", "ruby "),
    ("pl", "perl "),
    ("php", "php "),
    ("lua", "lua "),
];
const BRIEF: &str = include_str!("learn_brief.txt");
const SUBS: &str = "wisply learn [list] | \"<task>\" | save <name> \"<about>\" --file run.js --check check.js | practice <name> | ok <name> | show <name> | drop <name>";

const ARG_RE: &str = r#"\bargs(?:\.([A-Za-z_]\w*)|\[\s*["']([A-Za-z_]\w*)["']\s*\])(\s*(?:\|\||\?\?)\s*(?:"[^"]+"|'[^']+'|`[^`]+`|\d|true\b|\[|\{))?"#;
static ARG: LazyLock<Regex> = LazyLock::new(|| Regex::new(ARG_RE).expect("arg"));

struct Parts {
    code: String,
    check: String,
    about: String,
    guide: String,
    params: Map<String, Value>,
    before_ms: u64,
    tokens: u64,
}

pub fn is_skill(r: &Value) -> bool {
    r["learn"].is_object()
}

fn state(r: &Value) -> String {
    text(&r["learn"]["state"])
}

fn count(v: &Value) -> u64 {
    v.as_u64().unwrap_or(0)
}

fn stamp() -> String {
    local_now().chars().take(16).collect()
}

fn ident(k: &str) -> bool {
    k.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn pair(p: &str) -> Option<(String, String)> {
    p.split_once('=').filter(|(k, _)| ident(k)).map(|(k, v)| (k.to_string(), v.to_string()))
}

pub fn check_of(home: &Path, name: &str) -> Option<String> {
    store::load(home, name).ok().filter(is_skill).map(|r| text(&r["learn"]["check"])).filter(|c| !c.trim().is_empty())
}

fn defaulted(code: &str) -> HashSet<String> {
    ARG.captures_iter(code)
        .filter(|c| c.get(3).is_some())
        .filter_map(|c| c.get(1).or_else(|| c.get(2)).map(|m| m.as_str().to_string()))
        .collect()
}

pub fn optional(r: &Value) -> HashSet<String> {
    defaulted(&text(&r["script"]["code"]))
}

pub fn usage_of(r: &Value) -> String {
    let opt = optional(r);
    let args: String = r["params"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(k, _)| if opt.contains(k) { format!(" [{k}=<…>]") } else { format!(" {k}=<…>") })
        .collect();
    format!("wisply run {}{args}", text(&r["name"]))
}

pub fn fill_missing(r: &Value, params: &mut BTreeMap<String, String>) {
    let opt = optional(r);
    for (k, _) in r["params"].as_object().into_iter().flatten().filter(|(k, _)| !opt.contains(*k)) {
        params.entry(k.clone()).or_default();
    }
}

pub fn fits(r: &Value, other: usize) -> bool {
    state(r) == "learned" && other >= FIT_WORDS
}

pub fn offer_line(r: &Value, typed: &[String]) -> String {
    let found = script::swap(r, typed);
    let opt = optional(r);
    let args: String = r["params"]
        .as_object()
        .into_iter()
        .flatten()
        .filter(|(k, _)| found.contains_key(*k) || !opt.contains(*k))
        .map(|(k, _)| found.get(k).map_or_else(|| format!(" {k}=<…>"), |v| format!(" {}", store::kv(k, v))))
        .collect();
    let guide = text(&r["learn"]["guide"]);
    let guide = if guide.is_empty() { String::new() } else { format!(" ({guide})") };
    format!(
        "Wisply skill fits this request. Run it instead of doing the steps by hand, also for a small task (one call, checked after every run, fewer tokens): wisply run {}{args}{guide}",
        text(&r["name"])
    )
}

fn repeats(r: &Value) -> bool {
    count(&r["stats"]["seen"]) + count(&r["stats"]["runs"]) >= OFFER_AT
}

pub fn offer_note(r: &mut Value) -> Option<String> {
    if is_skill(r) || !repeats(r) || count(&r["stats"]["learn_offers"]) >= OFFERS_MAX {
        return None;
    }
    r["stats"]["learn_offers"] = json!(count(&r["stats"]["learn_offers"]) + 1);
    let name = text(&r["name"]);
    Some(format!(
        "-> {name} keeps coming back: if it is a skill (same task, other values), ask the user, then write check.js and run: wisply learn save {name} \"<what it does>\" --check check.js"
    ))
}

fn weak(r: &Value) -> bool {
    let st = &r["stats"];
    let runs = count(&st["runs"]);
    if runs < WEAK_RUNS {
        return false;
    }
    let before = count(&r["learn"]["before_ms"]) as f64;
    count(&st["fail"]) as f64 / runs as f64 > WEAK_FAIL
        || (before > 0.0 && count(&st["ms"]) as f64 >= WEAK_SPEED * before)
}

fn skill(home: &Path, name: &str) -> Result<Value, Fail> {
    let r = store::load(home, name)?;
    if is_skill(&r) {
        return Ok(r);
    }
    let name = text(&r["name"]);
    Err(Fail::hint(
        format!("{name} is a routine, not a skill"),
        format!("wisply learn save {name} \"<what it does>\" --check check.js"),
    ))
}

pub fn command(d: &Daemon, ctx: &mut Ctx, c: &Call) -> Result<Out, Fail> {
    defaults::ensure(&d.home);
    let sub = c.pos.first().map_or(String::new(), |s| s.to_lowercase());
    let rest = c.pos.get(1..).unwrap_or(&[]);
    let name =
        || rest.first().map(String::as_str).ok_or_else(|| Fail::hint(format!("learn {sub} needs a skill name"), SUBS));
    let text_out = match sub.as_str() {
        "" | "list" | "ls" => listing(&d.home),
        "save" => save_skill(d, ctx, c, rest)?,
        "practice" | "try" => practice_line(ctx, c, &skill(&d.home, name()?)?, rest.get(1..).unwrap_or(&[]))?,
        "ok" | "approve" => approve(&d.home, name()?)?,
        "show" => super::show(&skill(&d.home, name()?)?),
        "drop" | "rm" => drop_skill(&d.home, name()?)?,
        "help" => SUBS.to_string(),
        _ => match named_skill(&d.home, &c.pos) {
            Some(r) => super::show(&r),
            None => brief(&d.home, &c.pos.join(" ")),
        },
    };
    Ok(Out::Text(text_out))
}

fn named_skill(home: &Path, pos: &[String]) -> Option<Value> {
    let [one] = pos else { return None };
    let name = one.trim().to_lowercase();
    if name.is_empty() || name.contains(char::is_whitespace) {
        return None;
    }
    store::load(home, &name).ok().filter(|r| is_skill(r) && text(&r["name"]) == name)
}

fn brief(home: &Path, task: &str) -> String {
    let ws = words(task);
    let near: Vec<String> = load_all(home)
        .iter()
        .filter(|r| is_skill(r) && store::score(&ws, r).1 >= FIT_WORDS)
        .take(2)
        .map(usage_of)
        .collect();
    let mut out = format!("{}\ntask: {task}", BRIEF.trim_end());
    if !near.is_empty() {
        out.push_str(&format!("\nthere already, maybe it does this: {}", near.join(" | ")));
    }
    out
}

fn read_file(cwd: &Path, path: &str, what: &str) -> Result<String, Fail> {
    let full = files::resolve(cwd, path);
    let ext = full.extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
    if let Some((_, runner)) = OTHER_LANGS.iter().find(|(e, _)| *e == ext) {
        return Err(Fail::hint(
            format!("the {what} {} is .{ext}, a skill is code mode JavaScript", files::shown(&full)),
            format!("a run.js that calls it: return await $`{runner}{} ${{args.<name>}}`", files::shown(&full)),
        ));
    }
    let body = fs::read_to_string(&full)
        .map_err(|e| Fail::new(format!("cannot read the {what} {} ({e})", files::shown(&full))))?;
    if body.trim().is_empty() {
        return Err(Fail::new(format!("the {what} {} is empty", files::shown(&full))));
    }
    Ok(body)
}

fn params_of(code: &str, old: &Value, given: &[(String, String)]) -> (Map<String, Value>, Vec<String>) {
    let mut params = Map::new();
    let mut unset = Vec::new();
    let opt = defaulted(code);
    for cap in ARG.captures_iter(code) {
        let Some(k) = cap.get(1).or_else(|| cap.get(2)).map(|m| m.as_str().to_string()) else { continue };
        if NOT_PARAMS.contains(&k.as_str()) || params.contains_key(&k) {
            continue;
        }
        let v = given.iter().find(|(g, _)| *g == k).map(|(_, v)| v.clone()).or_else(|| old["params"].get(&k).map(text));
        if v.is_none() && !opt.contains(&k) {
            unset.push(k.clone());
        }
        params.insert(k, json!(v.unwrap_or_default()));
    }
    (params, unset)
}

fn tags_of(name: &str, about: &str, old: &Value) -> Vec<String> {
    let mut tags: Vec<String> = old["tags"].as_array().into_iter().flatten().map(text).collect();
    for w in words(&format!("{} {about}", name.replace('-', " "))) {
        if !tags.contains(&w) {
            tags.push(w);
        }
    }
    tags
}

fn build(name: &str, old: &Value, p: Parts) -> Value {
    let at = stamp();
    let fresh = json!({"runs": 0, "ok": 0, "fail": 0, "ms": 0, "saved": 0, "seen": 1, "recent": []});
    let mut r = if old.is_null() {
        json!({"name": name, "created": at, "steps": [], "check": null, "stats": fresh})
    } else {
        old.clone()
    };
    let cost = if p.tokens > 0 { p.tokens } else { count(&old["cost"]).max(p.code.chars().count() as u64 / 4 + 1) };
    let l = &old["learn"];
    r["version"] = json!(if old.is_null() { 1 } else { count(&old["version"]).max(1) + 1 });
    r["source"] = json!("skill");
    r["params"] = Value::Object(p.params);
    r["tags"] = json!(tags_of(name, &p.about, old));
    r["cost"] = json!(cost);
    r["flaky"] = json!(false);
    r["script"] =
        json!({"code": p.code, "hash": hash(&p.code), "calls": count(&old["script"]["calls"]), "project": ""});
    r["learn"] = json!({
        "state": "practice", "about": p.about, "guide": p.guide, "check": p.check, "before_ms": p.before_ms,
        "before_tokens": p.tokens, "practice": {"ok": 0, "fail": 0}, "hint": l["hint"].clone(), "at": at,
        "default": l["default"] == true, "shipped": l["shipped"].clone()
    });
    r
}

fn parts(c: &Call, cwd: &Path, old: &Value, rest: &[String]) -> Result<(Parts, Vec<String>), Fail> {
    let code = match c.val("file") {
        Some(f) => read_file(cwd, f, "script")?,
        None if script::is_script(old) => text(&old["script"]["code"]),
        None => return Err(Fail::hint("learn save needs the code", SUBS)),
    };
    let check = match c.val("check") {
        Some(f) => read_file(cwd, f, "check")?,
        None => text(&old["learn"]["check"]),
    };
    if check.trim().is_empty() {
        return Err(Fail::hint("learn save needs --check check.js: a skill is only learned with a check", SUBS));
    }
    let given: Vec<(String, String)> = rest.iter().filter_map(|p| pair(p)).collect();
    let said: Vec<&str> = rest.iter().filter(|p| pair(p).is_none()).map(String::as_str).collect();
    let about = if said.is_empty() { text(&old["learn"]["about"]) } else { said.join(" ") };
    let guide = c.val("guide").map_or_else(|| text(&old["learn"]["guide"]), str::to_string);
    let before_ms = parse_ms(c.flags.get("before"), count(&old["learn"]["before_ms"]))?;
    let tokens =
        c.val("tokens").and_then(|t| t.trim().parse().ok()).unwrap_or_else(|| count(&old["learn"]["before_tokens"]));
    let (params, unset) = params_of(&code, old, &given);
    Ok((Parts { code, check, about, guide, params, before_ms, tokens }, unset))
}

fn save_skill(d: &Daemon, ctx: &mut Ctx, c: &Call, rest: &[String]) -> Result<String, Fail> {
    let raw = rest.first().ok_or_else(|| Fail::hint("learn save needs a name", SUBS))?;
    let name = slug(raw, NAME_WORDS);
    if name.is_empty() {
        return Err(Fail::hint(format!("'{raw}' is not a usable skill name"), "use words like send-message"));
    }
    let old = fs::read_to_string(file(&d.home, &name))
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok())
        .unwrap_or(Value::Null);
    if !old.is_null() && !script::is_script(&old) {
        return Err(Fail::hint(format!("a step routine {name} exists already"), "pick another name"));
    }
    let (p, unset) = parts(c, &PathBuf::from(&ctx.cwd), &old, rest.get(1..).unwrap_or(&[]))?;
    let r = build(&name, &old, p);
    save(&d.home, &r)?;
    if defaults::is_default(&name) {
        defaults::bring_back(&d.home, &name);
    }
    let gone = script::absorb(&d.home, ctx, &r);
    let was = if gone.is_empty() { String::new() } else { format!("; it replaces routine {}", gone.join(", ")) };
    let head = format!("saved skill {name} v{}", count(&r["version"]));
    if !unset.is_empty() {
        let need: Vec<String> = unset.iter().map(|k| format!("{k}=<…>")).collect();
        return Ok(format!("{head}{was}. Practice it with values: wisply learn practice {name} {}", need.join(" ")));
    }
    let line = practice_line(ctx, c, &r, &[])?;
    Ok(format!("{line}\n{head} (the dry runs above are its practice){was}"))
}

fn practice_line(ctx: &Ctx, c: &Call, r: &Value, extra: &[String]) -> Result<String, Fail> {
    let opt = optional(r);
    let mut params: BTreeMap<String, String> = r["params"]
        .as_object()
        .into_iter()
        .flatten()
        .map(|(k, v)| (k.clone(), text(v)))
        .filter(|(k, v)| !v.is_empty() || !opt.contains(k))
        .collect();
    for p in extra {
        if let Some((k, v)) = p.split_once('=') {
            params.insert(k.trim().to_string(), v.to_string());
        }
    }
    params.insert("dry".into(), "1".into());
    script::handoff(ctx, c, &text(&r["name"]), &params)
}

pub fn ran(home: &Path, mut r: Value, body: &Value) -> String {
    let name = text(&r["name"]);
    let ok = body["ok"] == true;
    let err = text(&body["error"]);
    if body["dry"] != true {
        let typed = usage_of(&r).chars().count() / 4 + 1;
        store::record(home, &mut r, ok, body["ms"].as_f64().unwrap_or(0.0), typed, (!ok).then(|| err.clone()));
    }
    let l = &mut r["learn"];
    if ok && body["checked"] == true {
        l["practice"]["ok"] = json!(count(&l["practice"]["ok"]) + count(&body["rounds"]).max(1));
        l["hint"] = Value::Null;
    }
    if !ok {
        l["practice"]["fail"] = json!(count(&l["practice"]["fail"]) + 1);
        l["hint"] = json!(err.chars().take(HINT_CHARS).collect::<String>());
    }
    let passed = count(&l["practice"]["ok"]);
    if ok && l["state"] == "practice" && passed >= PRACTICE_ROUNDS as u64 {
        l["state"] = json!("ready");
    }
    let now = text(&l["state"]);
    let _ = save(home, &r);
    match (ok, now.as_str()) {
        (false, _) => format!(
            "skill {name} failed: {err} -> try: fix it (wisply learn show {name}), then wisply learn save {name} --file run.js --check check.js; the hint stays for the next AI"
        ),
        (true, "ready") => format!("skill {name} passed {passed} checks -> ask the user, then: wisply learn ok {name}"),
        (true, "practice") => format!("skill {name}: {passed}/{PRACTICE_ROUNDS} checks passed"),
        _ => String::new(),
    }
}

fn approve(home: &Path, name: &str) -> Result<String, Fail> {
    let mut r = skill(home, name)?;
    let name = text(&r["name"]);
    match state(&r).as_str() {
        "learned" => return Ok(format!("skill {name} is learned already -> {}", usage_of(&r))),
        "ready" => {}
        _ => {
            let passed = count(&r["learn"]["practice"]["ok"]);
            return Err(Fail::hint(
                format!("skill {name} is still practicing ({passed}/{PRACTICE_ROUNDS} checks passed)"),
                format!("wisply learn practice {name}"),
            ));
        }
    }
    r["learn"]["state"] = json!("learned");
    r["learn"]["at"] = json!(stamp());
    save(home, &r)?;
    Ok(format!("learned skill {name} -> from now on one call: {}", usage_of(&r)))
}

fn drop_skill(home: &Path, name: &str) -> Result<String, Fail> {
    let r = skill(home, name)?;
    let name = text(&r["name"]);
    fs::remove_file(file(home, &name)).map_err(|e| Fail::new(format!("cannot remove {name} ({e})")))?;
    if r["learn"]["default"] != true {
        return Ok(format!("removed skill {name}"));
    }
    defaults::forget(home, &name);
    Ok(format!("removed skill {name} (a default skill: it stays away until wisply learn save {name})"))
}

fn measure(r: &Value) -> String {
    let (st, l) = (&r["stats"], &r["learn"]);
    let before = count(&l["before_ms"]);
    let by_hand = if before > 0 { format!(" (by hand ~{})", took(before as f64)) } else { String::new() };
    let runs = count(&st["runs"]);
    if runs == 0 {
        return format!("not run yet{by_hand}");
    }
    let saved = count(&st["saved"]);
    let saved = if saved > 0 { format!(" | ~{saved} tokens saved") } else { String::new() };
    format!("{}/{runs} ok | {}{by_hand}{saved}", count(&st["ok"]), took(count(&st["ms"]) as f64))
}

fn row(r: &Value) -> String {
    let args: String = r["params"].as_object().into_iter().flatten().map(|(k, _)| format!(" {k}=")).collect();
    let name = text(&r["name"]);
    let mut out = format!("{name}{args} | {} | {}", state(r), measure(r));
    if r["learn"]["default"] == true {
        out.push_str(" | default");
    }
    if weak(r) {
        out.push_str(&format!(" | WEAK -> ask the user: wisply learn drop {name}"));
    }
    out
}

fn listing(home: &Path) -> String {
    let all = load_all(home);
    let mut lines: Vec<String> = all.iter().filter(|r| is_skill(r)).map(row).collect();
    if lines.is_empty() {
        lines.push("no skills yet".into());
    }
    let again: Vec<String> =
        all.iter().filter(|r| script::is_script(r) && !is_skill(r) && repeats(r)).map(|r| text(&r["name"])).collect();
    if !again.is_empty() {
        lines.push(format!(
            "repeats, could become skills: {} (wisply learn save <name> \"<about>\" --check check.js keeps the code)",
            again.join(", ")
        ));
    }
    lines.push("(teach one: wisply learn \"<task>\" | details: wisply learn show <name>)".into());
    lines.join("\n")
}

pub fn show_lines(r: &Value) -> Vec<String> {
    let l = &r["learn"];
    let mut out = vec![format!("skill: {} | {}", state(r), text(&l["about"]))];
    let guide = text(&l["guide"]);
    if !guide.is_empty() {
        out.push(format!("guide: {guide}"));
    }
    out.push(format!("use: {}", usage_of(r)));
    out.push(format!(
        "measured: {} | practice {} ok, {} failed",
        measure(r),
        count(&l["practice"]["ok"]),
        count(&l["practice"]["fail"])
    ));
    if let Some(hint) = l["hint"].as_str().filter(|h| !h.is_empty()) {
        out.push(format!("last failure (hint for the next AI): {hint}"));
    }
    let check = text(&l["check"]);
    out.push("check:".into());
    out.extend(check.lines().take(CHECK_LINES).enumerate().map(|(i, line)| format!("{:>3} {line}", i + 1)));
    out.push("code:".into());
    out
}

#[cfg(test)]
#[path = "skills_tests.rs"]
mod tests;
