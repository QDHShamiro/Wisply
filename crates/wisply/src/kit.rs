use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::sync::atomic::{AtomicU32, Ordering};

use serde_json::{Value, json};
use wisply_core::Fail;

use crate::risk;

const USAGE: &str = "wisply kit add <name> \"what it does\" [--file script] [--replace] -- <program> [args]";
const HOLES: &str = "{1}..{9} = its arguments, {all} = all of them, {file} = the saved script";
const NAME_MAX: usize = 40;
const ABOUT_MAX: usize = 200;
const SCRIPT_MAX: u64 = 1024 * 1024;
const SPEC: &str = "tool.json";

static NEXT: AtomicU32 = AtomicU32::new(0);

enum Done {
    Said(String),
    Ran(i32),
}

struct Tool {
    name: String,
    about: String,
    run: Vec<String>,
    file: Option<String>,
    agreed: bool,
}

pub fn run(home: &Path, args: &[String]) -> ExitCode {
    match command(home, args) {
        Ok(Done::Said(text)) => {
            println!("{text}");
            ExitCode::SUCCESS
        }
        Ok(Done::Ran(code)) => ExitCode::from(u8::try_from(code).unwrap_or(1)),
        Err(fail) => {
            println!("{}", fail.text());
            ExitCode::from(1)
        }
    }
}

fn command(home: &Path, args: &[String]) -> Result<Done, Fail> {
    let rest = args.get(1..).unwrap_or_default();
    match args.first().map(String::as_str) {
        None | Some("list") => Ok(Done::Said(list(home))),
        Some("add") => add(home, rest).map(Done::Said),
        Some("show") => show(home, rest).map(Done::Said),
        Some("rm") => remove(home, rest).map(Done::Said),
        Some("run") => {
            let (argv, dir) = plan(home, rest)?;
            let started = Command::new(&argv[0]).args(&argv[1..]).env("WISPLY_KIT", &dir).status();
            let name = rest.first().map(String::as_str).unwrap_or_default();
            started
                .map(|s| Done::Ran(s.code().unwrap_or(1)))
                .map_err(|e| Fail::hint(format!("cannot start '{}': {e}", argv[0]), format!("wisply kit show {name}")))
        }
        Some(other) => Err(Fail::hint(format!("kit does not know '{other}'"), "wisply kit | add | run | show | rm")),
    }
}

fn root(home: &Path) -> PathBuf {
    home.join("kit")
}

fn named(name: Option<&String>) -> Result<&str, Fail> {
    let name = name.map(String::as_str).unwrap_or_default();
    let fine = !name.is_empty()
        && name.len() <= NAME_MAX
        && !name.starts_with('-')
        && name.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-');
    if fine { Ok(name) } else { Err(Fail::hint("a tool name is a-z, 0-9 and dashes, like count-lines", USAGE)) }
}

fn load(home: &Path, name: &str) -> Result<Tool, Fail> {
    let missing = || Fail::hint(format!("no tool named '{name}'"), "wisply kit");
    let text = fs::read_to_string(root(home).join(name).join(SPEC)).map_err(|_| missing())?;
    let v: Value = serde_json::from_str(&text).map_err(|_| missing())?;
    let run: Vec<String> =
        v["run"].as_array().into_iter().flatten().filter_map(|a| a.as_str().map(String::from)).collect();
    if run.is_empty() {
        return Err(missing());
    }
    Ok(Tool {
        name: name.to_string(),
        about: v["about"].as_str().unwrap_or_default().to_string(),
        run,
        file: v["file"].as_str().map(String::from),
        agreed: v["agreed"].as_bool().unwrap_or(false),
    })
}

fn all(home: &Path) -> Vec<Tool> {
    let mut names: Vec<String> = fs::read_dir(root(home))
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| !n.starts_with('.'))
        .collect();
    names.sort_unstable();
    names.iter().filter_map(|n| load(home, n).ok()).collect()
}

fn list(home: &Path) -> String {
    let tools = all(home);
    if tools.is_empty() {
        return format!("no tools of your own yet. Build one: {USAGE}   ({HOLES})");
    }
    let mut lines = vec![format!("{} tool(s) of your own, run one with: wisply kit run <name> [args]", tools.len())];
    lines.extend(tools.iter().map(|t| format!("  {} - {} | {}", t.name, t.about, t.run.join(" "))));
    lines.join("\n")
}

fn show(home: &Path, args: &[String]) -> Result<String, Fail> {
    let name = named(args.first())?;
    let tool = load(home, name)?;
    let dir = root(home).join(name);
    let mut lines = vec![format!("{} - {}", tool.name, tool.about), format!("runs: {:?}", tool.run)];
    if let Some(file) = &tool.file {
        lines.push(format!("script: {}", dir.join(file).to_string_lossy().replace('\\', "/")));
    }
    if tool.agreed {
        lines.push("the user agreed to what it does".into());
    }
    Ok(lines.join("\n"))
}

fn remove(home: &Path, args: &[String]) -> Result<String, Fail> {
    let name = named(args.first())?;
    load(home, name)?;
    fs::remove_dir_all(root(home).join(name)).map_err(|e| Fail::new(format!("cannot remove '{name}': {e}")))?;
    Ok(format!("ok: tool '{name}' removed"))
}

struct Order<'a> {
    name: &'a str,
    about: &'a str,
    file: Option<&'a str>,
    replace: bool,
    yes: bool,
    run: &'a [String],
}

fn order(args: &[String]) -> Result<Order<'_>, Fail> {
    let cut = args.iter().position(|a| a == "--").unwrap_or(args.len());
    let run = args.get(cut + 1..).unwrap_or_default();
    let (mut plain, mut file, mut replace, mut yes) = (Vec::new(), None, false, false);
    let mut it = args[..cut].iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--replace" => replace = true,
            "--yes" | "-y" => yes = true,
            "--file" => file = it.next().map(String::as_str),
            _ if a.starts_with("--") => return Err(Fail::hint(format!("kit add does not know {a}"), USAGE)),
            _ => plain.push(a),
        }
    }
    let name = named(plain.first().copied())?;
    let about = plain.get(1).map(|a| a.trim()).unwrap_or_default();
    if about.is_empty() || about.chars().count() > ABOUT_MAX {
        return Err(Fail::hint(
            format!("say what '{name}' does in one sentence (up to {ABOUT_MAX} characters)"),
            USAGE,
        ));
    }
    if run.is_empty() {
        return Err(Fail::hint(format!("after -- comes the program '{name}' runs ({HOLES})"), USAGE));
    }
    Ok(Order { name, about, file, replace, yes, run })
}

fn script(path: &str) -> Result<(String, Vec<u8>), Fail> {
    let from = Path::new(path);
    let small = fs::metadata(from).is_ok_and(|m| m.is_file() && m.len() <= SCRIPT_MAX);
    let body = small.then(|| fs::read(from).ok()).flatten();
    let body = body.ok_or_else(|| Fail::new(format!("{path} is not a file under 1 MB that can be read")))?;
    let name = from.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let name = if name.eq_ignore_ascii_case(SPEC) { format!("script-{name}") } else { name };
    Ok((name, body))
}

fn put(dir: &Path, name: &str, body: &[u8]) -> std::io::Result<()> {
    let path = dir.join(name);
    wisply_core::save(&path, body)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}

fn add(home: &Path, args: &[String]) -> Result<String, Fail> {
    let o = order(args)?;
    let saved = o.file.map(script).transpose()?;
    if saved.is_none() && o.run.iter().any(|part| part.contains("{file}")) {
        return Err(Fail::hint("{file} stands for a saved script, and none was given", "add --file <script>"));
    }
    let code = saved.as_ref().map(|(_, body)| String::from_utf8_lossy(body).into_owned()).unwrap_or_default();
    let found = risk::of(&format!("{}\n{code}", o.run.join(" ")));
    if let Some(found) = found.as_ref().filter(|_| !o.yes) {
        return Err(risk::ask(&format!("the tool '{}'", o.name), found, "the same call with --yes"));
    }
    let spec = json!({
        "name": o.name, "about": o.about, "run": o.run,
        "file": saved.as_ref().map(|(name, _)| name), "agreed": found.is_some(),
    });
    let spec = serde_json::to_vec_pretty(&spec).unwrap_or_default();
    let dir = root(home).join(o.name);
    let taken = || Fail::hint(format!("a tool named '{}' is already there", o.name), "add --replace, or another name");
    let broken = |e: std::io::Error| Fail::new(format!("cannot save the tool in {}: {e}", dir.display()));
    let old = load(home, o.name).ok();
    if old.is_some() && !o.replace {
        return Err(taken());
    }
    let store = |into: &Path| -> std::io::Result<()> {
        if let Some((name, body)) = &saved {
            put(into, name, body)?;
        }
        wisply_core::save(into.join(SPEC), &spec)
    };
    if let Some(old) = old {
        store(&dir).map_err(broken)?;
        if let Some(stale) = old.file.filter(|f| saved.as_ref().is_none_or(|(name, _)| name != f)) {
            let _ = fs::remove_file(dir.join(stale));
        }
    } else {
        if dir.is_dir() {
            let _ = fs::remove_dir_all(&dir);
        }
        let fresh = root(home).join(format!(".new-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        let moved = fs::create_dir_all(&fresh).and_then(|()| store(&fresh)).and_then(|()| fs::rename(&fresh, &dir));
        if let Err(e) = moved {
            let _ = fs::remove_dir_all(&fresh);
            return Err(if dir.join(SPEC).is_file() { taken() } else { broken(e) });
        }
    }
    let agreed = found.map(|f| format!(" The user agreed that it {}.", f.risk.what())).unwrap_or_default();
    Ok(format!(
        "ok: tool '{0}' saved for this user, every session can run it: wisply kit run {0} [args].{agreed}",
        o.name
    ))
}

fn fill(part: &str, args: &[String], file: Option<&str>, used: &mut bool) -> Result<String, usize> {
    let mut out = String::new();
    let mut rest = part;
    while let Some(at) = rest.find('{') {
        out.push_str(&rest[..at]);
        rest = &rest[at..];
        let Some(end) = rest.find('}') else { break };
        let key = &rest[1..end];
        let value = match (key, key.parse::<usize>()) {
            ("all", _) => Some(args.join(" ")),
            ("file", _) => file.map(String::from),
            (_, Ok(n)) if (1..=9).contains(&n) => Some(args.get(n - 1).ok_or(n)?.clone()),
            _ => None,
        };
        *used |= value.is_some() && key != "file";
        out.push_str(value.as_deref().unwrap_or(&rest[..=end]));
        rest = &rest[end + 1..];
    }
    out.push_str(rest);
    Ok(out)
}

fn plan(home: &Path, args: &[String]) -> Result<(Vec<String>, PathBuf), Fail> {
    let name = named(args.first())?;
    let yes = args.get(1).is_some_and(|a| a == "--yes");
    let given = &args[1 + usize::from(yes)..];
    let tool = load(home, name)?;
    let dir = root(home).join(name);
    let file = tool.file.as_ref().map(|f| dir.join(f).to_string_lossy().into_owned());
    let (mut argv, mut used) = (Vec::new(), false);
    for part in &tool.run {
        if part == "{all}" {
            argv.extend(given.iter().cloned());
            used = true;
            continue;
        }
        argv.push(fill(part, given, file.as_deref(), &mut used).map_err(|n| {
            Fail::hint(
                format!("'{name}' needs {n} argument(s) and got {}: {}", given.len(), tool.about),
                format!("wisply kit run {name} <{n} argument(s)>"),
            )
        })?);
    }
    if !used {
        argv.extend(given.iter().cloned());
    }
    if !tool.agreed
        && !yes
        && let Some(found) = risk::of(&argv.join(" "))
    {
        let again = format!("wisply kit run {name} --yes {}", given.join(" "));
        return Err(risk::ask(&format!("'{name}' with these arguments"), &found, again.trim_end()));
    }
    Ok((argv, dir))
}

#[cfg(test)]
#[path = "kit/tests.rs"]
mod tests;
