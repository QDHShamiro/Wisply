use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use regex::{Captures, Regex};
use serde_json::{Map, Value, json};
use sha2::{Digest, Sha256};
use wisply_core::{Call, Fail, local_now};

use super::super::Daemon;
use super::super::ctx::{Ctx, now};
use super::learn::{TAGS, prompt_at, prompt_words, stamp};
use super::store::{self, file, load_all, save, slug, stem, text, usage};

static QUOTED: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#""[^"\n]*"|'[^'\n]*'|`[^`]*`"#).expect("quoted"));
static VALUE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\b\d+(?:[-.:]\d+)+\b|\b\d{3,}\b").expect("value"));
static MONTH: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:19|20)\d\d-\d\d$").expect("month"));
static DATE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:19|20)\d\d-\d\d-\d\d$").expect("date"));
static YEAR: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(?:19|20)\d\d$").expect("year"));
const PROMPT_WORDS: usize = 64;
const NAME_WORDS: usize = 3;
const REPLACE_WITHIN_S: f64 = 1800.0;
const FEW_RUNS_DAYS: f64 = 30.0;
const MANY_RUNS_DAYS: f64 = 90.0;
const MANY_RUNS: u64 = 5;
const SHOW_LINES: usize = 80;
const DAY_S: f64 = 86_400.0;
const FIT_WORDS: usize = 2;

pub fn hash(code: &str) -> String {
    let flat: Vec<&str> = code.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    Sha256::digest(flat.join("\n").as_bytes()).iter().take(8).map(|b| format!("{b:02x}")).collect()
}

pub fn project_of(cwd: &Path) -> PathBuf {
    cwd.ancestors().find(|p| p.join(".git").exists()).unwrap_or(cwd).to_path_buf()
}

fn same_place(a: &str, b: &str) -> bool {
    let flat = |p: &str| p.replace('\\', "/").trim_end_matches('/').to_lowercase();
    !a.is_empty() && flat(a) == flat(b)
}

pub fn is_script(r: &Value) -> bool {
    r["script"].is_object()
}

pub fn code_of(home: &Path, name: &str) -> Option<String> {
    store::load(home, name).ok().filter(is_script).map(|r| text(&r["script"]["code"]))
}

pub fn fits(other: usize, r: &Value, cwd: &str) -> bool {
    other >= FIT_WORDS && same_place(&text(&r["script"]["project"]), &project_of(Path::new(cwd)).display().to_string())
}

fn values(r: &Value) -> BTreeMap<String, String> {
    r["params"].as_object().into_iter().flatten().map(|(k, v)| (k.clone(), text(v))).collect()
}

fn next_time(r: &Value) -> String {
    format!("-> next time: {}", usage(r, &values(r)))
}

fn guess_name(code: &str) -> String {
    let inside = code.split('`').nth(1).unwrap_or("");
    let name = slug(inside, NAME_WORDS);
    if name.is_empty() { "script".to_string() } else { name }
}

fn task_words(prompt: Vec<String>, code: &str) -> Vec<String> {
    let in_code: HashSet<String> = store::words(code).iter().map(|w| stem(w)).collect();
    let (mut out, rest): (Vec<String>, Vec<String>) = prompt.into_iter().partition(|w| in_code.contains(&stem(w)));
    out.extend(rest);
    out.truncate(TAGS);
    out
}

fn kind_of(value: &str) -> Option<&'static str> {
    if MONTH.is_match(value) {
        Some("month")
    } else if DATE.is_match(value) {
        Some("date")
    } else if YEAR.is_match(value) {
        Some("year")
    } else {
        VALUE.find(value).filter(|m| m.as_str() == value).map(|_| "value")
    }
}

pub fn swap(r: &Value, typed: &[String]) -> BTreeMap<String, String> {
    let found: Vec<&String> = typed.iter().filter(|w| w.starts_with(|c: char| c.is_ascii_digit())).collect();
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    for (k, v) in r["params"].as_object().into_iter().flatten() {
        let Some(kind) = kind_of(&text(v)) else { continue };
        if let Some(w) = found.iter().find(|w| kind_of(w) == Some(kind) && !out.values().any(|o| o == **w)) {
            out.insert(k.clone(), (*w).clone());
        }
    }
    out
}

fn key_for(value: &str, params: &mut Map<String, Value>) -> String {
    if let Some((k, _)) = params.iter().find(|(_, v)| v.as_str() == Some(value)) {
        return k.clone();
    }
    let kind = kind_of(value).unwrap_or("value");
    let (mut key, mut n) = (kind.to_string(), 2);
    while params.contains_key(&key) {
        key = format!("{kind}{n}");
        n += 1;
    }
    params.insert(key.clone(), json!(value));
    key
}

pub fn values_in(text: &str) -> Vec<String> {
    VALUE.find_iter(text).map(|m| m.as_str().to_string()).collect()
}

fn lift(code: &str, typed: &HashSet<String>) -> (String, Map<String, Value>) {
    let mut params = Map::new();
    let out = QUOTED.replace_all(code, |c: &Captures| {
        let lit = &c[0];
        if lit.contains('\\') || lit.contains("${") {
            return lit.to_string();
        }
        let q = &lit[..1];
        VALUE
            .replace_all(lit, |v: &Captures| {
                if !typed.contains(&v[0]) {
                    return v[0].to_string();
                }
                let key = key_for(&v[0], &mut params);
                if q == "`" { format!("${{args.{key}}}") } else { format!("{q} + args.{key} + {q}") }
            })
            .into_owned()
    });
    (out.into_owned(), params)
}

fn free_name(home: &Path, base: &str) -> String {
    let (mut name, mut n) = (base.to_string(), 2);
    while file(home, &name).exists() {
        name = format!("{base}-{n}");
        n += 1;
    }
    name
}

struct Fresh<'a> {
    code: &'a str,
    hash: &'a str,
    args: &'a Value,
    calls: &'a Value,
}

fn renew(d: &Daemon, name: &str, new: &Fresh, word: &str) -> Option<String> {
    let mut r = store::load(&d.home, name).ok().filter(is_script)?;
    let version = r["version"].as_u64().unwrap_or(1) + 1;
    r["version"] = json!(version);
    r["script"]["code"] = json!(new.code);
    r["script"]["hash"] = json!(new.hash);
    r["script"]["calls"] = new.calls.clone();
    r["params"] = new.args.clone();
    r["flaky"] = json!(false);
    r["stats"]["recent"] = json!([]);
    save(&d.home, &r).ok()?;
    Some(format!("{word} routine {name} (now v{version}) {}", next_time(&r)))
}

fn recent(at: f64) -> bool {
    now() - at <= REPLACE_WITHIN_S
}

fn saved(d: &Daemon, ctx: &mut Ctx, body: &Value) -> Result<String, Fail> {
    let code = text(&body["code"]);
    if code.trim().is_empty() {
        return Err(Fail::new("routine script needs the code"));
    }
    let h = hash(&code);
    if let Some(mut same) = load_all(&d.home).into_iter().find(|r| r["script"]["hash"] == h.as_str()) {
        same["stats"]["seen"] = json!(same["stats"]["seen"].as_u64().unwrap_or(1) + 1);
        save(&d.home, &same)?;
        return Ok(format!("this script is saved as routine {} {}", text(&same["name"]), next_time(&same)));
    }
    let t0 = now() - body["ms"].as_f64().unwrap_or(0.0) / 1000.0;
    let prompts = d.prompts_of(&ctx.sid);
    let typed: HashSet<String> =
        prompt_at(&prompts, t0).into_iter().filter(|w| w.starts_with(|c: char| c.is_ascii_digit())).collect();
    let given = body["args"].as_object().cloned().unwrap_or_default();
    let (code, args) = if given.is_empty() { lift(&code, &typed) } else { (code, given) };
    let args = Value::Object(args);
    let new = Fresh { code: &code, hash: &h, args: &args, calls: &body["calls"] };
    if let Some((name, at)) = ctx.broken_script.take()
        && recent(at)
        && let Some(note) = renew(d, &name, &new, "fixed")
    {
        return Ok(note);
    }
    prune(&d.home);
    let tags = task_words(prompt_words(&prompts, t0, &args, PROMPT_WORDS), &code);
    let said = slug(&tags.join(" "), NAME_WORDS);
    let base = if said.is_empty() { guess_name(&code) } else { said };
    if let Some((name, at, last)) = ctx.last_script.clone()
        && last == base
        && recent(at)
        && let Some(note) = renew(d, &name, &new, "updated")
    {
        ctx.last_script = Some((name, now(), base));
        return Ok(note);
    }
    let name = free_name(&d.home, &base);
    ctx.last_script = Some((name.clone(), now(), base));
    let r = json!({
        "name": name, "version": 1, "source": "script", "created": local_now().chars().take(16).collect::<String>(),
        "params": args, "steps": [], "check": null, "cost": code.chars().count() / 4 + 1, "tags": tags,
        "script": {"code": code, "hash": h, "calls": body["calls"], "project": body["project"]},
        "stats": {"runs": 0, "ok": 0, "fail": 0, "ms": 0, "saved": 0, "seen": 1, "recent": []},
        "flaky": false
    });
    save(&d.home, &r)?;
    Ok(format!("saved as routine {name} {}", next_time(&r)))
}

fn ran(d: &Daemon, ctx: &mut Ctx, body: &Value) -> Result<String, Fail> {
    let mut r = store::load(&d.home, &text(&body["routine"]))?;
    let name = text(&r["name"]);
    let ok = body["ok"] == true;
    let typed = usage(&r, &BTreeMap::new()).chars().count() / 4 + 1;
    let err = body["error"].as_str().map(str::to_string);
    store::record(&d.home, &mut r, ok, body["ms"].as_f64().unwrap_or(0.0), typed, err);
    if ok {
        ctx.broken_script = None;
        return Ok(String::new());
    }
    ctx.broken_script = Some((name.clone(), now()));
    Ok(format!(
        "routine {name} failed -> try: fix the script and run it with wisply codemode, the next working one replaces {name}"
    ))
}

pub fn command(d: &Daemon, ctx: &mut Ctx, rest: &[String]) -> Result<String, Fail> {
    let raw = rest.first().ok_or_else(|| Fail::new("routine script needs a report"))?;
    let body: Value = serde_json::from_str(raw).map_err(|e| Fail::new(format!("routine script got bad json ({e})")))?;
    if body["routine"].is_string() {
        return ran(d, ctx, &body);
    }
    ctx.ep = super::Episode::default();
    saved(d, ctx, &body)
}

pub fn handoff(ctx: &Ctx, c: &Call, name: &str, params: &BTreeMap<String, String>) -> Result<String, Fail> {
    if !ctx.run_stack.is_empty() {
        return Err(Fail::hint(
            format!("{name} is a script routine and cannot run inside routine {}", ctx.run_stack.join(" > ")),
            format!("wisply run {name} on its own"),
        ));
    }
    let pairs: Vec<(String, String)> = params.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    Ok(wisply_proto::handoff_line(name, &pairs, c.on("yes")))
}

pub fn show(r: &Value) -> Vec<String> {
    let code = text(&r["script"]["code"]);
    let lines: Vec<&str> = code.lines().collect();
    let mut out: Vec<String> =
        lines.iter().take(SHOW_LINES).enumerate().map(|(i, l)| format!("{:>3} {l}", i + 1)).collect();
    if lines.len() > SHOW_LINES {
        out.push(format!("... {} more lines", lines.len() - SHOW_LINES));
    }
    out
}

pub fn prune(home: &Path) {
    let few = stamp(now() - FEW_RUNS_DAYS * DAY_S);
    let many = stamp(now() - MANY_RUNS_DAYS * DAY_S);
    for r in load_all(home).into_iter().filter(is_script) {
        let last =
            r["stats"]["last"].as_str().filter(|l| !l.is_empty()).map_or_else(|| text(&r["created"]), str::to_string);
        let limit = if r["stats"]["runs"].as_u64().unwrap_or(0) >= MANY_RUNS { &many } else { &few };
        if last.len() == 16 && last < *limit {
            let _ = fs::remove_file(file(home, &text(&r["name"])));
        }
    }
}

#[cfg(test)]
#[path = "script_tests.rs"]
mod tests;
