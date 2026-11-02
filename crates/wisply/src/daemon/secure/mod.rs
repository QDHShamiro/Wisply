mod allow;
mod app;
mod asar;
mod axml;
mod deps;
mod deps2;
mod fetch;
mod fix;
mod hygiene;
mod report;
mod rules;
mod scan;
mod site;
mod zip;

use std::path::{Path, PathBuf};
use std::time::Duration;

use wisply_core::{Call, Fail};

use super::ctx::Ctx;
use super::{Daemon, Out, config, game, media, web};
use report::Report;

const WAIT: Duration = Duration::from_secs(600);
const FIX_WAIT: Duration = Duration::from_secs(2400);
const CHARS: usize = 8000;

pub fn scan_code(root: &Path, history: bool, check_deps: bool) -> Report {
    let names = scan::files(root);
    let mut report = Report::new(root.to_string_lossy().replace('\\', "/"));
    report.checked.push(format!("{} files", names.len()));
    report.found.extend(scan::scan_files(root, &names));
    report.found.extend(hygiene::check(root, &names));
    if check_deps {
        let (all, notes) = deps::collect(root, &names);
        match deps::check(root, &all) {
            Ok(found) => {
                report.found.extend(found);
                report.checked.push(format!("{} dependencies on osv.dev", all.len()));
            }
            Err(why) => report.notes.push(format!("dependencies not checked: {why}")),
        }
        report.notes.extend(notes);
    }
    if history {
        report.found.extend(scan::scan_history(root));
        report.checked.push("git history".into());
    }
    report.sorted();
    report
}

fn scan_app(path: &Path, kind: &str) -> Result<Report, String> {
    let mut report = Report::new(path.file_name().and_then(|n| n.to_str()).unwrap_or("app"));
    match kind {
        "jar" => app::jar(path, &mut report)?,
        "apk" => app::apk(path, &mut report)?,
        "asar" => asar::scan(path, &mut report)?,
        _ => app::exe(path, &mut report)?,
    }
    report.sorted();
    Ok(report)
}

fn render(report: &Report, json: bool) -> String {
    if json { report.json() } else { report.text() }
}

fn start_path(ctx: &Ctx, c: &Call) -> PathBuf {
    match c.pos.first() {
        Some(p) => web::local_file(ctx, p),
        None => PathBuf::from(if ctx.cwd.is_empty() { "." } else { ctx.cwd.as_str() }),
    }
}

fn folder_of(start: PathBuf) -> Result<PathBuf, Fail> {
    if !start.is_dir() {
        return Err(Fail::hint(format!("no folder {}", start.display()), "wisply secure <project folder>"));
    }
    Ok(wisply_memory::root_of(&start).unwrap_or(start))
}

fn closed_game() -> Result<(), Fail> {
    if std::env::var_os("WISPLY_SECURE_ANY_TIME").is_some() {
        return Ok(());
    }
    let playing = game::running();
    if playing.is_empty() {
        return Ok(());
    }
    Err(Fail::hint(format!("{playing} is running"), "wisply secure runs when the game is closed"))
}

fn run_fix(d: &Daemon, ctx: &mut Ctx, c: &Call) -> Result<Out, Fail> {
    closed_game()?;
    let mut shifted = c.clone();
    shifted.pos.remove(0);
    let root = folder_of(match shifted.pos.first() {
        Some(p) => web::local_file(ctx, p),
        None => PathBuf::from(if ctx.cwd.is_empty() { "." } else { ctx.cwd.as_str() }),
    })?;
    let value = |key: &str| match c.flags.get(key) {
        Some(wisply_core::Flag::Val(v)) => Some(v.clone()),
        _ => None,
    };
    let (with, model) = (value("with").map(|w| w.to_lowercase()), value("model"));
    let home = d.home.clone();
    let pending = media::background(
        "secure-fix",
        FIX_WAIT,
        "wisply secure (shows what is left)",
        move || fix::run(&root, &home, with.as_deref(), model.as_deref()),
        |d, ctx, text: String| Ok(web::capped(d, ctx, &text, "secure", None, CHARS)),
    );
    Ok(Out::Wait(pending))
}

fn run_site(d: &Daemon, arg: &str, json: bool) -> Result<Out, Fail> {
    let Some(parsed) = allow::parse(arg) else {
        return Err(Fail::hint(format!("'{arg}' is not a domain"), "wisply secure example.com"));
    };
    if !allow::permitted(&d.home, &parsed.host) {
        return Err(Fail::hint(
            format!("{} is not in your allowed sites", parsed.host),
            format!("wisply secure allow {} (only for sites that are yours)", parsed.host),
        ));
    }
    let home = d.home.clone();
    let pending = media::background(
        "secure-site",
        WAIT,
        "wisply secure <site> (1 request per second)",
        move || site::check(&home, &parsed).map(|r| render(&r, json)).map_err(Fail::new),
        move |d, ctx, text: String| Ok(if json { text } else { web::capped(d, ctx, &text, "secure", None, CHARS) }),
    );
    Ok(Out::Wait(pending))
}

pub fn run(d: &Daemon, ctx: &mut Ctx, c: &Call) -> Result<Out, Fail> {
    if !config::on(&config::load(&d.home), "secure") {
        return Err(Fail::hint("secure is switched off", "wisply config secure on"));
    }
    let json = c.on("json");
    match c.pos.first().map(String::as_str) {
        Some("allow") => return Ok(Out::Text(allow::command(&d.home, &c.pos[1..], c.on("remove"))?)),
        Some("fix") => return run_fix(d, ctx, c),
        Some(arg) if allow::looks_like_site(arg) && !web::local_file(ctx, arg).exists() => {
            return run_site(d, arg, json);
        }
        _ => {}
    }
    closed_game()?;
    let start = start_path(ctx, c);
    let (history, check_deps) = (c.on("history"), !c.on("no-deps"));
    if start.is_file() {
        let Some(kind) = app::kind(&start) else {
            return Err(Fail::hint(
                format!("{} is not a jar, apk, exe or asar", start.display()),
                "wisply secure <folder|site|file.jar|file.apk|file.exe>",
            ));
        };
        let pending = media::background(
            "secure-app",
            WAIT,
            "wisply secure <file>",
            move || scan_app(&start, kind).map(|r| render(&r, json)).map_err(Fail::new),
            move |d, ctx, text: String| Ok(if json { text } else { web::capped(d, ctx, &text, "secure", None, CHARS) }),
        );
        return Ok(Out::Wait(pending));
    }
    let root = folder_of(start)?;
    let pending = media::background(
        "secure",
        WAIT,
        "wisply secure --no-deps (skips the online check)",
        move || Ok(render(&scan_code(&root, history, check_deps), json)),
        move |d, ctx, text: String| Ok(if json { text } else { web::capped(d, ctx, &text, "secure", None, CHARS) }),
    );
    Ok(Out::Wait(pending))
}
