use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use serde_json::{Value, json};
use wisply_core::{Call, Fail};

use super::cache::{self, Mark, Slot};
use super::words::{fold, near};

pub const APPS: &[&str] = &["discord", "whatsapp"];
const FILE: &str = "contacts.json";
const PREFIX_MIN: usize = 3;
const FUZZY_MIN: usize = 4;
const USAGE: &str = "wisply contact add Anna discord=anna_99 alias=annie | contact rm Anna | contacts";

static LOCK: Mutex<()> = Mutex::new(());
static PEOPLE: Slot<Mark, Vec<Person>> = Slot::new();

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Person {
    pub name: String,
    pub alias: Vec<String>,
    pub discord: String,
    pub whatsapp: String,
    pub app: String,
    pub dm: String,
    pub sent: u64,
}

pub enum Found {
    One(Person),
    Maybe(Vec<Person>),
    Nobody,
}

fn file(home: &Path) -> PathBuf {
    home.join(FILE)
}

fn s(v: &Value, k: &str) -> String {
    v[k].as_str().unwrap_or("").to_string()
}

impl Person {
    fn of(v: &Value) -> Person {
        Person {
            name: s(v, "name"),
            alias: v["alias"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(str::to_lowercase)
                .collect(),
            discord: s(v, "discord"),
            whatsapp: s(v, "whatsapp"),
            app: s(v, "app"),
            dm: s(v, "dm"),
            sent: v["sent"].as_u64().unwrap_or(0),
        }
    }

    fn json(&self) -> Value {
        json!({"name": self.name, "alias": self.alias, "discord": self.discord, "whatsapp": self.whatsapp,
            "app": self.app, "dm": self.dm, "sent": self.sent})
    }

    fn words(&self) -> Vec<String> {
        let mut all = vec![self.name.to_lowercase(), self.discord.to_lowercase(), self.whatsapp.to_lowercase()];
        all.extend(self.alias.iter().cloned());
        all.retain(|w| !w.is_empty());
        all
    }

    pub fn handle(&self, app: &str) -> String {
        let own = if app == "discord" { &self.discord } else { &self.whatsapp };
        if own.is_empty() { self.name.clone() } else { own.clone() }
    }

    pub fn app_for(&self, asked: Option<&str>) -> Option<String> {
        if let Some(a) = asked {
            return Some(a.to_string());
        }
        if !self.app.is_empty() {
            return Some(self.app.clone());
        }
        match (self.discord.is_empty(), self.whatsapp.is_empty()) {
            (false, true) => Some("discord".into()),
            (true, false) => Some("whatsapp".into()),
            _ => None,
        }
    }
}

pub(super) fn read(home: &Path) -> Vec<Person> {
    let raw = fs::read_to_string(file(home)).ok().and_then(|t| serde_json::from_str::<Value>(&t).ok());
    raw.and_then(|v| v["people"].as_array().cloned()).unwrap_or_default().iter().map(Person::of).collect()
}

pub fn loaded(home: &Path) -> Arc<Vec<Person>> {
    PEOPLE.get(home, cache::mark(&file(home)), || read(home))
}

pub fn load(home: &Path) -> Vec<Person> {
    loaded(home).as_ref().clone()
}

fn save(home: &Path, people: &[Person]) -> Result<(), Fail> {
    let body = json!({"people": people.iter().map(Person::json).collect::<Vec<_>>()});
    let text = serde_json::to_string_pretty(&body).map_err(|e| Fail::new(e.to_string()))?;
    PEOPLE.clear();
    let saved = wisply_core::save(file(home), text).map_err(|e| Fail::new(format!("could not save contacts ({e})")));
    PEOPLE.clear();
    saved
}

fn folded(p: &Person) -> Vec<String> {
    p.words().iter().map(|w| fold(w)).collect()
}

fn hits(people: &[Person], hit: impl Fn(&str) -> bool) -> Vec<Person> {
    people.iter().filter(|p| folded(p).iter().any(|w| hit(w))).cloned().collect()
}

fn fuzzy(people: &[Person], word: &str) -> Vec<Person> {
    if word.chars().count() < FUZZY_MIN { Vec::new() } else { hits(people, |w| near(w, word)) }
}

pub fn knows(people: &[Person], word: &str) -> bool {
    let word = fold(word);
    !hits(people, |w| w == word).is_empty() || fuzzy(people, &word).len() == 1
}

pub fn find(people: &[Person], word: &str) -> Found {
    let word = fold(word);
    let exact = hits(people, |w| w == word);
    match exact.len() {
        1 => return Found::One(exact[0].clone()),
        0 => {}
        _ => return Found::Maybe(exact),
    }
    if word.chars().count() >= PREFIX_MIN {
        let prefix = hits(people, |w| w.starts_with(&word));
        if !prefix.is_empty() {
            return Found::Maybe(prefix);
        }
    }
    let mut close = fuzzy(people, &word);
    match close.len() {
        0 => Found::Nobody,
        1 => Found::One(close.remove(0)),
        _ => Found::Maybe(close),
    }
}

fn title(word: &str) -> String {
    let mut c = word.chars();
    c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
}

fn upsert(people: &mut Vec<Person>, name: &str) -> usize {
    let low = name.to_lowercase();
    if let Some(i) = people.iter().position(|p| p.name.to_lowercase() == low) {
        return i;
    }
    people.push(Person { name: title(name), ..Person::default() });
    people.len() - 1
}

pub fn learn(home: &Path, name: &str, app: &str, handle: &str, dm: Option<&str>) {
    let _guard = LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut people = load(home);
    let i = upsert(&mut people, name);
    let p = &mut people[i];
    match app {
        "discord" if p.discord.is_empty() => p.discord = handle.to_string(),
        "whatsapp" if p.whatsapp.is_empty() => p.whatsapp = handle.to_string(),
        _ => {}
    }
    p.app = app.to_string();
    if let Some(dm) = dm.filter(|d| !d.is_empty()) {
        p.dm = dm.to_string();
    }
    p.sent += 1;
    let _ = save(home, &people);
}

fn pairs(c: &Call) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = c
        .pos
        .iter()
        .skip(2)
        .filter_map(|p| p.split_once('=').map(|(k, v)| (k.trim().to_lowercase(), v.trim().to_string())))
        .collect();
    for k in ["discord", "whatsapp", "alias", "app"] {
        if let Some(v) = c.val(k) {
            out.push((k.to_string(), v.to_string()));
        }
    }
    out
}

fn add(home: &Path, c: &Call) -> Result<String, Fail> {
    let name =
        c.pos.get(1).filter(|n| !n.contains('=')).ok_or_else(|| Fail::hint("contact add needs a name", USAGE))?;
    let _guard = LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut people = load(home);
    let i = upsert(&mut people, name);
    let p = &mut people[i];
    for (k, v) in pairs(c) {
        match k.as_str() {
            "discord" => p.discord = v,
            "whatsapp" | "wa" => p.whatsapp = v,
            "app" if APPS.contains(&v.to_lowercase().as_str()) => p.app = v.to_lowercase(),
            "alias" => {
                for a in v.split(',').map(|a| a.trim().to_lowercase()).filter(|a| !a.is_empty()) {
                    if !p.alias.contains(&a) {
                        p.alias.push(a);
                    }
                }
            }
            _ => return Err(Fail::hint(format!("unknown contact field '{k}'"), USAGE)),
        }
    }
    let line = row(p);
    save(home, &people)?;
    Ok(format!("saved {line}"))
}

fn rm(home: &Path, c: &Call) -> Result<String, Fail> {
    let name = c.pos.get(1).ok_or_else(|| Fail::hint("contact rm needs a name", USAGE))?.to_lowercase();
    let _guard = LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut people = load(home);
    let before = people.len();
    people.retain(|p| p.name.to_lowercase() != name);
    if people.len() == before {
        return Err(Fail::hint(format!("no contact called {name}"), "wisply contacts"));
    }
    save(home, &people)?;
    Ok(format!("removed {name}"))
}

pub fn row(p: &Person) -> String {
    let mut parts = vec![p.name.clone()];
    if !p.discord.is_empty() {
        parts.push(format!("discord={}", quoted(&p.discord)));
    }
    if !p.whatsapp.is_empty() {
        parts.push(format!("whatsapp={}", quoted(&p.whatsapp)));
    }
    if !p.alias.is_empty() {
        parts.push(format!("alias={}", p.alias.join(",")));
    }
    if !p.app.is_empty() {
        parts.push(format!("app={}", p.app));
    }
    if !p.dm.is_empty() {
        parts.push("dm".into());
    }
    if p.sent > 0 {
        parts.push(format!("sent={}", p.sent));
    }
    parts.join(" ")
}

fn quoted(v: &str) -> String {
    if v.contains(char::is_whitespace) { format!("\"{v}\"") } else { v.to_string() }
}

fn list(home: &Path) -> String {
    let people = load(home);
    if people.is_empty() {
        return format!("no contacts, add: {USAGE}");
    }
    people.iter().map(row).collect::<Vec<_>>().join("\n")
}

fn sent(home: &Path, c: &Call) -> Result<String, Fail> {
    let name =
        c.pos.get(1).filter(|n| !n.contains('=')).ok_or_else(|| Fail::hint("contact sent needs a name", USAGE))?;
    let kv = pairs(c);
    let get = |k: &str| kv.iter().find(|(key, _)| key == k).map(|(_, v)| v.as_str());
    let app = get("app").unwrap_or("").to_lowercase();
    if !APPS.contains(&app.as_str()) {
        return Err(Fail::hint(format!("contact sent needs app={}", APPS.join("|")), USAGE));
    }
    learn(home, name, &app, get("handle").unwrap_or(name), get("dm"));
    Ok(format!("noted {name}"))
}

pub fn command(home: &Path, c: &Call) -> Result<String, Fail> {
    if c.name == "contacts" {
        return Ok(list(home));
    }
    match c.pos.first().map(|p| p.to_lowercase()).as_deref() {
        None | Some("list" | "ls") => Ok(list(home)),
        Some("add" | "set") => add(home, c),
        Some("rm" | "remove" | "forget" | "del") => rm(home, c),
        Some("sent") => sent(home, c),
        Some(other) => Err(Fail::hint(format!("unknown contact command '{other}'"), USAGE)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn home(tag: &str) -> PathBuf {
        let h = std::env::temp_dir().join(format!("wisply-contacts-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&h);
        fs::create_dir_all(&h).expect("home");
        h
    }

    fn call(name: &str, pos: &[&str]) -> Call {
        Call { name: name.into(), pos: pos.iter().map(|p| (*p).to_string()).collect(), flags: Default::default() }
    }

    #[test]
    fn add_find_learn_and_remove() {
        let h = home("cycle");
        command(&h, &call("contact", &["add", "anna", "discord=anna_99", "alias=annie,nana"])).expect("add");
        let people = load(&h);
        assert!(knows(&people, "Anna") && knows(&people, "anna_99") && knows(&people, "nana"));
        assert!(matches!(find(&people, "annie"), Found::One(p) if p.name == "Anna"));
        assert!(matches!(find(&people, "ann"), Found::Maybe(v) if v.len() == 1));
        assert!(matches!(find(&people, "ben"), Found::Nobody));
        assert!(matches!(find(&people, "an"), Found::Nobody));
        learn(&h, "Anna", "discord", "anna_99", Some("https://discord.com/channels/@me/42"));
        let p = load(&h).remove(0);
        assert_eq!((p.app.as_str(), p.dm.as_str(), p.sent), ("discord", "https://discord.com/channels/@me/42", 1));
        assert_eq!(p.app_for(None).as_deref(), Some("discord"));
        assert_eq!(p.handle("whatsapp"), "Anna");
        learn(&h, "mama", "whatsapp", "Mama", None);
        let people = load(&h);
        assert!(matches!(find(&people, "mama"), Found::One(p) if p.whatsapp == "Mama" && p.name == "Mama"));
        let dm = "dm=https://discord.com/channels/@me/7";
        command(&h, &call("contact", &["sent", "Tom", "app=discord", "handle=xTOMx", dm])).expect("sent");
        let a = load(&h).into_iter().find(|p| p.name == "Tom").expect("tom");
        assert_eq!((a.discord.as_str(), a.dm.as_str(), a.sent), ("xTOMx", "https://discord.com/channels/@me/7", 1));
        assert!(command(&h, &call("contact", &["sent", "Tom", "app=icq"])).is_err());
        command(&h, &call("contact", &["rm", "anna"])).expect("rm");
        assert_eq!(load(&h).len(), 2);
        assert!(command(&h, &call("contact", &["add", "x", "bogus=1"])).is_err());
        let _ = fs::remove_dir_all(&h);
    }

    #[test]
    fn two_people_with_one_alias_are_ambiguous() {
        let h = home("twins");
        command(&h, &call("contact", &["add", "Ben", "alias=lu"])).expect("a");
        command(&h, &call("contact", &["add", "Lea", "alias=lu"])).expect("b");
        assert!(matches!(find(&load(&h), "lu"), Found::Maybe(v) if v.len() == 2));
        let _ = fs::remove_dir_all(&h);
    }

    #[test]
    fn typos_case_and_umlauts_find_exactly_one() {
        let named = |n: &str| Person { name: n.to_string(), ..Person::default() };
        let people: Vec<Person> =
            ["Jonas", "Jürgen", "Tobias", "Mario", "Marie", "Ben"].into_iter().map(named).collect();
        for typo in ["jonaz", "JONAS", "jonass", "joas", "jnoas"] {
            assert!(matches!(find(&people, typo), Found::One(p) if p.name == "Jonas"), "{typo}");
        }
        for typo in ["juergen", "JUERGEN", "jürgen", "juergn", "jurgen"] {
            assert!(matches!(find(&people, typo), Found::One(p) if p.name == "Jürgen"), "{typo}");
        }
        assert!(matches!(find(&people, "tobiaz"), Found::One(p) if p.name == "Tobias"));
        assert!(matches!(find(&people, "marik"), Found::Maybe(v) if v.len() == 2));
        assert!(matches!(find(&people, "bem"), Found::Nobody));
        assert!(matches!(find(&people, "xyzzy"), Found::Nobody));
        assert!(knows(&people, "jnoas") && knows(&people, "Juergen") && !knows(&people, "marik"));
    }
}
