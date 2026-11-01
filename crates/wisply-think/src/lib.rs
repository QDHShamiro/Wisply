pub mod files;
pub mod fix;
pub mod flow;
pub mod ideas;
pub mod measure;
pub mod personal;
pub mod prove;
pub mod report;
pub mod rules;
pub mod run;
pub mod safety;
pub mod skill;
pub mod work;
pub mod yours;

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};
use wisply_core::Call;

use files::Pack;
use rules::Scope;

const GENERIC: &[Pack] = &[Pack::Code, Pack::Docs, Pack::Images];
const PROFILE_SHARE: usize = 20;
const QUICK_LINES: usize = 5;
const TOLD_FILES: usize = 400;
const EDITS: &[&str] = &["Edit", "Write", "MultiEdit", "NotebookEdit"];

pub const USAGE: &str = "wisply think <step>   critical thinking without AI: finds, fixes and proves improvements
run      start \"<name>\" [paths] [--goal \"...\"] [--quick] [--size file|feature|project]   status   runs   brief
find     scan [--all] (own checks fitted to the project and your rules)   rules   profile [set web,rust|auto]
ideas    crit \"<criterion>\" ... [--why]   hint \"<doubt>\"   source <url> \"<proof>\"
         idea add \"title | lens | tool | impact | sure | effort | safe/big\" (or lines on stdin)   idea list [--all]
         idea set <ids> --status done|unchecked|dropped|suggestion [--proof|--why|--grade]   idea out \"<why>\"
change   fix [--deep] (Wisply fixes what it safely can, proves it, reverts when worse)
         checkpoint <id> <files>   revert <id> [--why] [--force]   restore --yes   diff [label]
prove    measure <label> [--cmd \"...\"] [--runs 5] [--test] [--metrics]   compare [a b]   save <label> [--file f]
         blind <before> <after>   blind-result <n> 1|2|same   round [--anyway \"<reason>\"]   report
lessons  learn \"<lesson>\" --tag <tag>   learn --list   lint-skill <folder>
any step takes --run <id> for another run";

pub struct Env {
    pub home: PathBuf,
    pub user: PathBuf,
    pub sid: String,
    pub cwd: PathBuf,
    pub playing: String,
    pub cwds: Vec<String>,
    pub notes: RefCell<Vec<String>>,
}

impl Env {
    pub fn path(&self, p: &str) -> PathBuf {
        let p = p.trim().trim_matches(['"', '\'']);
        let drive = cfg!(windows)
            && p.len() >= 2
            && p.starts_with('/')
            && p.as_bytes()[1].is_ascii_alphabetic()
            && (p.len() == 2 || p.as_bytes()[2] == b'/');
        let p = if let Some(rest) = p.strip_prefix("~/").or_else(|| p.strip_prefix("~\\")) {
            self.user.join(rest)
        } else if drive {
            PathBuf::from(format!("{}:/{}", p[1..2].to_uppercase(), p.get(3..).unwrap_or("")))
        } else {
            PathBuf::from(p)
        };
        if p.is_absolute() { p } else { self.cwd.join(p) }
    }

    pub fn note(&self, s: String) {
        self.notes.borrow_mut().push(s);
    }

    fn profile(&self) -> Value {
        read_json(&self.home.join("think").join("profile.json"))
    }

    pub fn chosen(&self) -> Option<BTreeSet<Pack>> {
        chosen(&self.profile())
    }

    pub fn packs(&self, root: &Path, list: &[PathBuf]) -> BTreeSet<Pack> {
        let mut found = files::packs_of(root, list);
        found.extend(files::quick_packs(root));
        limit(found, self.chosen())
    }

    pub fn person(&self) -> BTreeMap<Pack, usize> {
        let mut out = BTreeMap::new();
        let mut seen = BTreeSet::new();
        for c in &self.cwds {
            let root = files::root_of(Path::new(c));
            if !seen.insert(root.clone()) {
                continue;
            }
            for p in files::quick_packs(&root) {
                if !GENERIC.contains(&p) {
                    *out.entry(p).or_insert(0) += 1;
                }
            }
        }
        out
    }

    pub fn profile_packs(&self) -> BTreeSet<Pack> {
        if let Some(set) = self.chosen() {
            return set;
        }
        let counts = self.person();
        let total: usize = counts.values().sum::<usize>().max(1);
        counts.into_iter().filter(|(_, n)| n * 100 / total >= PROFILE_SHARE).map(|(p, _)| p).collect()
    }
}

fn read_json(f: &Path) -> Value {
    fs::read_to_string(f).ok().and_then(|t| serde_json::from_str(&t).ok()).unwrap_or(json!({}))
}

fn chosen(profile: &Value) -> Option<BTreeSet<Pack>> {
    let set: BTreeSet<Pack> =
        profile.get("set")?.as_array()?.iter().filter_map(Value::as_str).filter_map(Pack::from_name).collect();
    (!set.is_empty()).then_some(set)
}

fn limit(found: BTreeSet<Pack>, set: Option<BTreeSet<Pack>>) -> BTreeSet<Pack> {
    match set {
        Some(set) => found.into_iter().filter(|p| GENERIC.contains(p) || set.contains(p)).collect(),
        None => found,
    }
}

fn migrate(home: &run::Home, user: &Path) {
    if home.lessons_file().exists() {
        return;
    }
    let old = user.join(".claude").join("skills").join("critical-thinking-god").join("learnings.md");
    let Ok(text) = fs::read_to_string(old) else { return };
    let items: Vec<String> = text.lines().filter(|l| l.starts_with("- ")).map(str::to_string).collect();
    if !items.is_empty() {
        home.write_lessons(&items);
    }
}

pub fn command(env: &Env, c: &Call) -> Result<String, String> {
    let home = run::Home::new(&env.home, &env.sid);
    migrate(&home, &env.user);
    let sub = c.pos.first().map(|s| s.to_lowercase()).unwrap_or_default();
    let out = match sub.as_str() {
        "start" => flow::start(env, &home, c),
        "scan" => work::scan(env, &home, c),
        "fix" => work::fix(env, &home, c),
        "crit" => flow::crit(&home, c),
        "hint" => flow::hint(&home, c),
        "source" => flow::source(&home, c),
        "idea" | "ideas" => ideas::idea(&home, c),
        "checkpoint" => work::checkpoint(env, &home, c),
        "revert" => work::revert(&home, c),
        "restore" => work::restore(&home, c),
        "diff" => work::diff(&home, c),
        "save" => prove::save(env, &home, c),
        "measure" => prove::measure_cmd(&home, c),
        "compare" => prove::compare(&home, c),
        "blind" => prove::blind(env, &home, c),
        "blind-result" => prove::blind_result(&home, c),
        "round" => flow::round(&home, c),
        "report" => report::report(&home, c),
        "learn" => report::learn(&home, c),
        "status" => report::status(&home, c),
        "runs" => Ok(report::runs(&home)),
        "brief" => Ok(report::brief(env, &home)),
        "profile" => work::profile(env, c),
        "rules" => Ok(work::rules_cmd(env)),
        "lint-skill" => work::lint_skill(env, c),
        "" | "help" => Ok(USAGE.into()),
        other => Err(format!("no step called {other}\n{USAGE}")),
    }?;
    let notes = env.notes.borrow();
    if notes.is_empty() {
        return Ok(out);
    }
    let head: Vec<String> = notes.iter().map(|n| format!("! {n}")).collect();
    Ok(format!("{}\n{out}", head.join("\n")))
}

pub fn blind_open(wisply_home: &Path) -> Value {
    prove::blind_open(&run::Home::new(wisply_home, ""))
}

pub fn blind_pick(wisply_home: &Path, run_id: &str, n: i64, verdict: &str) -> Result<String, String> {
    let home = run::Home::new(wisply_home, "");
    let mut r = home.load(Some(run_id))?;
    let winner = safety::blind_result(&mut r, n, verdict)?;
    r.save()?;
    Ok(winner)
}

fn before_edit(event: &Value, file: &Path) -> Option<String> {
    let input = &event["tool_input"];
    match event["tool_name"].as_str()? {
        "Write" => event["tool_response"]["originalFile"]
            .as_str()
            .map(str::to_string)
            .or_else(|| (event["tool_response"]["type"] == "create").then(String::new)),
        "Edit" => {
            let (old, new) = (input["old_string"].as_str()?, input["new_string"].as_str()?);
            let now = fs::read_to_string(file).ok()?;
            let (old, new) = if !now.contains(new) && now.contains("\r\n") {
                (old.replace('\n', "\r\n"), new.replace('\n', "\r\n"))
            } else {
                (old.to_string(), new.to_string())
            };
            if new.is_empty() || !now.contains(&new) {
                return None;
            }
            Some(if input["replace_all"] == true { now.replace(&new, &old) } else { now.replacen(&new, &old, 1) })
        }
        _ => None,
    }
}

pub fn after_edit(wisply_home: &Path, input: &[u8]) -> Option<String> {
    let event: Value = serde_json::from_slice(input).ok()?;
    if !EDITS.contains(&event["tool_name"].as_str()?) {
        return None;
    }
    let user = std::env::var_os(if cfg!(windows) { "USERPROFILE" } else { "HOME" }).map(PathBuf::from)?;
    let tool = &event["tool_input"];
    let file = PathBuf::from(tool["file_path"].as_str().or_else(|| tool["notebook_path"].as_str())?);
    let before = before_edit(&event, &file);
    let said = std::panic::catch_unwind(|| quick_from(wisply_home, &user, &file, before.as_deref())).ok().flatten();
    Some(said.map_or_else(String::new, |c| {
        json!({"hookSpecificOutput": {"hookEventName": "PostToolUse", "additionalContext": c}}).to_string()
    }))
}

pub fn quick(wisply_home: &Path, user: &Path, file: &Path) -> Option<String> {
    quick_from(wisply_home, user, file, None)
}

fn quick_from(wisply_home: &Path, user: &Path, file: &Path, was: Option<&str>) -> Option<String> {
    let dir = wisply_home.join("think");
    if dir.join("quick-off").exists() || !file.is_file() {
        return None;
    }
    let root = files::root_of(file);
    let profile = read_json(&dir.join("profile.json"));
    let mut packs = files::quick_packs(&root);
    packs.extend(files::packs_of(&root, &[file.to_path_buf()]));
    let packs = limit(packs, chosen(&profile));
    let mine = personal::load(user, &root, &chosen(&profile).unwrap_or_default());
    let scope = Scope { root: &root, packs: &packs, rules: &mine };
    let keep = |f: &rules::Finding| f.impact >= 2 || f.rule == "yours";
    let found: Vec<rules::Finding> = rules::file(&scope, file, false).into_iter().filter(keep).collect();
    let told_file = dir.join("told.json");
    let mut told = read_json(&told_file);
    let key = files::slashed(file);
    let before: BTreeSet<String> = match was {
        Some(text) => rules::text_file(&scope, file, text).into_iter().filter(keep).map(|f| f.exact()).collect(),
        None => told
            .get(&key)
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(Value::as_str).map(str::to_string).collect())
            .unwrap_or_default(),
    };
    let fresh: Vec<&rules::Finding> = found.iter().filter(|f| !f.known(&before)).collect();
    if let Some(m) = told.as_object_mut() {
        if m.len() > TOLD_FILES {
            m.clear();
        }
        m.insert(key, json!(found.iter().map(rules::Finding::exact).collect::<Vec<_>>()));
    }
    let _ = fs::create_dir_all(&dir);
    let _ = fs::write(&told_file, told.to_string());
    if fresh.is_empty() {
        return None;
    }
    let mut out = vec![format!("wisply think: {} new in {}", fresh.len(), files::rel(&root, file))];
    out.extend(fresh.iter().take(QUICK_LINES).map(|f| {
        let at = if f.line > 0 { format!("line {}", f.line) } else { "file".into() };
        format!("  {at} {}: {} -> {}", f.rule, f.why, f.fix)
    }));
    Some(out.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quick_check_speaks_only_about_new_findings() {
        let base = std::env::temp_dir().join(format!("wisply-think-quick-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let user = base.join("user");
        let proj = base.join("proj");
        fs::create_dir_all(user.join(".claude")).unwrap();
        fs::create_dir_all(proj.join("src/main/resources")).unwrap();
        fs::write(proj.join("pom.xml"), "<project/>").unwrap();
        fs::write(proj.join("src/main/resources/plugin.yml"), "name: X\n").unwrap();
        fs::write(
            user.join(".claude/CLAUDE.md"),
            "# Minecraft Plugin\n- `ConcurrentHashMap` for shared state — never `HashMap`.\n",
        )
        .unwrap();
        let java = proj.join("src/A.java");
        fs::write(&java, "class A {\n  void f(HashMap<A, B> m) {}\n}\n").unwrap();
        let home = base.join("wisply");
        let first = quick(&home, &user, &java).unwrap();
        assert!(first.contains("new in src/A.java") && first.contains("`HashMap` used 1x"), "{first}");
        assert_eq!(quick(&home, &user, &java), None);
        fs::write(&java, "class A {\n  void f(HashMap<A, B> m) {}\n  void g(HashMap<A, B> n) {}\n}\n").unwrap();
        let worse = quick(&home, &user, &java).unwrap_or_default();
        assert!(worse.contains("`HashMap` used 2x"), "{worse}");
        fs::write(&java, "class A {\n  void f(HashMap<A, B> m) {}\n}\n").unwrap();
        assert_eq!(quick(&home, &user, &java), None);
        fs::write(&java, "class A {\n  void f(ConcurrentHashMap<A, B> m) {}\n}\n").unwrap();
        assert_eq!(quick(&home, &user, &java), None);
        let env = Env {
            home: home.clone(),
            user: user.clone(),
            sid: "s".into(),
            cwd: proj.clone(),
            playing: String::new(),
            cwds: vec![files::slashed(&proj)],
            notes: RefCell::new(Vec::new()),
        };
        assert_eq!(env.person().keys().map(|p| p.name()).collect::<Vec<_>>(), ["java-plugin", "mc-gui"]);
        assert_eq!(env.path("src/A.java"), proj.join("src/A.java"));
        if cfg!(windows) {
            assert_eq!(env.path("/e/Wisply"), PathBuf::from("E:/Wisply"));
        }
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn quick_check_reports_only_what_the_edit_brought() {
        let base = std::env::temp_dir().join(format!("wisply-think-edit-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let (user, proj, home) = (base.join("user"), base.join("proj"), base.join("wisply"));
        fs::create_dir_all(user.join(".claude")).unwrap();
        fs::create_dir_all(proj.join("src/main/resources")).unwrap();
        fs::write(proj.join("src/main/resources/plugin.yml"), "name: X\n").unwrap();
        fs::write(proj.join("pom.xml"), "<project/>").unwrap();
        fs::write(user.join(".claude/CLAUDE.md"), "# Minecraft Plugin\n- never `HashMap`.\n").unwrap();
        let java = proj.join("src/A.java");
        let edit = |old: &str, new: &str| {
            let now = fs::read_to_string(&java).unwrap().replacen(old, new, 1);
            fs::write(&java, now).unwrap();
            let event = json!({"tool_name": "Edit", "tool_input": {"file_path": files::slashed(&java), "old_string": old, "new_string": new}});
            before_edit(&event, &java)
        };
        let start = "class A {\n  void f(HashMap<A, B> m) {}\n  int x = 1;\n}\n";
        fs::write(&java, start).unwrap();
        let was = edit("int x = 1;", "int x = 2;");
        assert_eq!(was.as_deref(), Some(start));
        assert_eq!(quick_from(&home, &user, &java, None).map(|s| s.contains("used 1x")), Some(true));
        assert_eq!(quick_from(&home, &user, &java, was.as_deref()), None);
        let was = edit("int x = 2;", "void g(HashMap<A, B> n) {}");
        let said = quick_from(&home, &user, &java, was.as_deref()).unwrap_or_default();
        assert!(said.contains("`HashMap` used 2x"), "{said}");
        let create = json!({"tool_name": "Write", "tool_response": {"type": "create"}});
        assert_eq!(before_edit(&create, &java), Some(String::new()));
        fs::write(&java, "a\r\nc\r\n").unwrap();
        let crlf = json!({"tool_name": "Edit", "tool_input": {"old_string": "a\nb", "new_string": "a\nc"}});
        assert_eq!(before_edit(&crlf, &java).as_deref(), Some("a\r\nb\r\n"));
        let _ = fs::remove_dir_all(&base);
    }
}
