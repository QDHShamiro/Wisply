mod answer;
mod away;
mod brief;
mod cfg;
mod hook;
mod idle;
mod net;
mod ring;
mod setup;
mod store;
mod talk;
mod text;

use std::path::Path;
use std::process::ExitCode;

use wisply_core::Fail;

use answer::Opts;
use cfg::fill;
use ring::{Kind, Spec};

#[cfg(windows)]
pub use talk::{Act, Desk};

const HELP: &str = "wisply call ask \"<question>\" [--choices ja,nein] [--wait 300] [--channels ntfy,callmebot] [--force] (rings him, waits, prints the answer)
wisply call notice \"<text>\" [--title t] [--channels x] [--force] (rings, does not wait)
wisply call answer <id|last> \"<text>\" | result <id> | list (answers and open questions)
wisply call away on|off|status (the hook only rings while he is away; automatic after away_after_min without input)
wisply call briefing [--channels x] [--dry] | schedule on [HH:MM] | schedule off | schedule status
wisply call setup [copy] | test [channel] | set key=value ... | config | hook-config | status";

pub fn run(home: &Path, args: &[String]) -> ExitCode {
    if args.first().map(String::as_str) == Some("hook") {
        print!("{}", hook::run(home));
        return ExitCode::SUCCESS;
    }
    match dispatch(home, args) {
        Ok(out) => {
            if !out.is_empty() {
                println!("{out}");
            }
            ExitCode::SUCCESS
        }
        Err(fail) => {
            println!("{}", fail.text());
            ExitCode::from(1)
        }
    }
}

fn flag(args: &mut Vec<String>, name: &str) -> Option<String> {
    let at = args.iter().position(|a| a == name)?;
    args.remove(at);
    (at < args.len()).then(|| args.remove(at))
}

fn switch(args: &mut Vec<String>, name: &str) -> bool {
    let before = args.len();
    args.retain(|a| a != name);
    args.len() != before
}

fn words(args: &[String]) -> String {
    args.join(" ").trim().to_string()
}

fn number(raw: Option<String>, name: &str) -> Result<Option<u64>, Fail> {
    raw.map(|v| v.trim().parse::<u64>().map_err(|_| Fail::new(format!("{name} needs a number of seconds, got '{v}'"))))
        .transpose()
}

fn dispatch(home: &Path, args: &[String]) -> Result<String, Fail> {
    let mut rest: Vec<String> = args.get(1..).unwrap_or(&[]).to_vec();
    match args.first().map(String::as_str) {
        Some("ask") => ask(home, &mut rest),
        Some("notice") => notice(home, &mut rest),
        Some("answer") => reply(home, &rest),
        Some("result") => result(home, &rest),
        Some("list") => Ok(list(home)),
        Some("away") => away::run(home, &rest),
        Some("briefing") => briefing(home, &mut rest),
        Some("schedule") => setup::schedule(home, &rest),
        Some("setup") => setup::run(home, &rest),
        Some("test") => test(home, &rest),
        Some("set") => cfg::set(home, &rest),
        Some("config") => Ok(cfg::show(home)),
        Some("hook-config") => Ok(hook::snippet(home)),
        Some("status") => Ok(setup::status(home)),
        Some("help") | None => Ok(HELP.to_string()),
        Some(other) => Err(Fail::hint(format!("unknown call command '{other}'"), "wisply call help")),
    }
}

fn ask(home: &Path, rest: &mut Vec<String>) -> Result<String, Fail> {
    let choices =
        flag(rest, "--choices").map(|c| c.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect());
    let wait_s = number(flag(rest, "--wait"), "--wait")?;
    let channels = flag(rest, "--channels");
    let force = switch(rest, "--force");
    let question = words(rest);
    if question.is_empty() {
        return Err(Fail::hint("what should I ask?", "wisply call ask \"Soll ich deployen?\" --choices ja,nein"));
    }
    let opts = Opts { question, choices: choices.unwrap_or_default(), wait_s, channels, force };
    let (done, line) = answer::ask(home, &opts)?;
    match done.answer {
        Some(a) => Ok(a),
        None => Ok(format!("call: asked {} ({line}); the answer: wisply call result {}", done.id, done.id)),
    }
}

fn notice(home: &Path, rest: &mut Vec<String>) -> Result<String, Fail> {
    let settings = cfg::load(home);
    let title = flag(rest, "--title").unwrap_or_else(|| cfg::text(&settings, "title_notice"));
    let channels = flag(rest, "--channels");
    let force = switch(rest, "--force");
    let body = words(rest);
    if body.is_empty() {
        return Err(Fail::hint("what should I say?", "wisply call notice \"Build fertig\""));
    }
    if !cfg::on(&settings, "enabled") {
        return Err(Fail::hint("calls are off", "wisply call set enabled=on"));
    }
    if text::quiet_now(&settings) && !force {
        return Err(Fail::hint("quiet hours, nobody is rung", "use --force"));
    }
    let spec = Spec {
        kind: Kind::Notice,
        title,
        text: body,
        ask: None,
        channels: ring::pick(&settings, "channels", channels.as_deref()),
    };
    let (rang, line) = ring::report(&ring::send(home, &settings, &spec));
    if rang { Ok(format!("call: {line}")) } else { Err(Fail::hint(line, "wisply call setup")) }
}

fn reply(home: &Path, rest: &[String]) -> Result<String, Fail> {
    let (Some(which), true) = (rest.first(), rest.len() > 1) else {
        return Err(Fail::hint("answer needs an id and the text", "wisply call answer <id|last> \"ja\""));
    };
    let done = store::submit(home, which, &words(&rest[1..]), "cli")?;
    Ok(format!("call: {} answered {}", done.id, done.answer.unwrap_or_default()))
}

fn result(home: &Path, rest: &[String]) -> Result<String, Fail> {
    let id = rest.first().ok_or_else(|| Fail::hint("which question?", "wisply call result <id>"))?;
    let ask = store::load(home, id).ok_or_else(|| Fail::new(format!("no question '{id}'")))?;
    match (ask.state.as_str(), ask.answer.clone()) {
        ("answered", Some(a)) => Ok(a),
        ("open", _) if ask.open() => Ok(format!("call: {} is still waiting", ask.id)),
        ("open", _) => Ok(format!("call: {} expired without an answer", ask.id)),
        (state, _) => Ok(format!("call: {} is {state}", ask.id)),
    }
}

fn list(home: &Path) -> String {
    let open = store::open_list(home);
    if open.is_empty() {
        return "call: nothing is waiting".to_string();
    }
    let rows: Vec<String> =
        open.iter().map(|a| format!("{} {} [{}]", a.id, text::clip(&a.question, 80), a.choices.join("/"))).collect();
    format!("call: {} waiting\n{}", open.len(), rows.join("\n"))
}

fn briefing(home: &Path, rest: &mut Vec<String>) -> Result<String, Fail> {
    let settings = cfg::load(home);
    let channels = flag(rest, "--channels");
    let dry = switch(rest, "--dry");
    let spoken = brief::compose(home, &settings);
    if dry {
        return Ok(format!("call: briefing (dry, nobody is rung)\n{spoken}"));
    }
    if !cfg::on(&settings, "enabled") {
        return Err(Fail::hint("calls are off", "wisply call set enabled=on"));
    }
    let spec = Spec {
        kind: Kind::Briefing,
        title: cfg::text(&settings, "title_briefing"),
        text: spoken.clone(),
        ask: None,
        channels: ring::pick(&settings, "briefing_channels", channels.as_deref()),
    };
    let (rang, line) = ring::report(&ring::send(home, &settings, &spec));
    if !rang {
        return Err(Fail::hint(line, "wisply call setup"));
    }
    brief::done(home);
    talk::briefing(home, &settings, &spoken);
    Ok(format!("call: briefing {line}\n{spoken}"))
}

fn test(home: &Path, rest: &[String]) -> Result<String, Fail> {
    let settings = cfg::load(home);
    let over = rest.first().map(String::as_str);
    let spec = Spec {
        kind: Kind::Ask,
        title: cfg::text(&settings, "title_ask"),
        text: fill(&cfg::text(&settings, "text_test"), &[]),
        ask: None,
        channels: ring::pick(&settings, "channels", over),
    };
    let (rang, line) = ring::report(&ring::send(home, &settings, &spec));
    if rang { Ok(format!("call: test {line}")) } else { Err(Fail::hint(line, "wisply call setup")) }
}
