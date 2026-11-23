use std::sync::{LazyLock, Once};

use regex::Regex;
use wisply_core::{APP_ONLY, CHAT, SHARED, WEB_ONLY};
use wisply_memory::{Lesson, now, oneline};

use super::super::Daemon;
use super::super::ctx::Ctx;
use super::with_store;
use crate::prewarn::fix_text;

const WINDOW_S: f64 = 60.0;
const KIND_CHARS: usize = 80;
const FIX_MAX: usize = 200;
const NOT_LEARNED: &[&str] = &["tabs", "downloads", "console", "tools"];
const HELPERS: &[&str] = &["snap", "text", "windows", "tabs", "scroll"];

static QUOTED: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#""[^"]*"|'[^']*'"#).expect("quoted pattern"));
static DIGITS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\d+").expect("digit pattern"));
static REF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[edotw]\d+$").expect("ref pattern"));
static TIDY: Once = Once::new();

pub struct Failed {
    sig: String,
    verb: String,
    fix: String,
    target: String,
    at: f64,
    helper: Option<String>,
}

pub struct Offered {
    sig: String,
    fix: String,
}

fn learnable(name: &str) -> bool {
    let action = SHARED.contains(&name) || WEB_ONLY.contains(&name) || APP_ONLY.contains(&name) || CHAT.contains(&name);
    action && !NOT_LEARNED.contains(&name)
}

fn host(url: &str) -> &str {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let host = rest.split(['/', '?', '#']).next().unwrap_or("");
    host.strip_prefix("www.").unwrap_or(host)
}

pub fn target(ctx: &Ctx) -> String {
    match ctx.target.as_ref() {
        Some((kind, _)) if kind == "web" => host(&ctx.url()).to_lowercase(),
        Some((kind, name)) => format!("{kind}:{}", name.to_lowercase()),
        None => String::new(),
    }
}

pub fn kind(out: &str) -> String {
    let first = out.lines().next().unwrap_or("");
    let first = first.strip_prefix("err: ").unwrap_or(first);
    let first = first.split(" -> try:").next().unwrap_or(first);
    let plain = QUOTED.replace_all(first, "…");
    let plain = DIGITS.replace_all(&plain, "#").to_lowercase();
    plain.trim().chars().take(KIND_CHARS).collect()
}

pub fn worth(verb: &str, fix: &str) -> bool {
    let mut words = fix.split_whitespace();
    let first = words.next().unwrap_or("");
    let step = first == verb || HELPERS.contains(&first);
    step && fix.chars().count() <= FIX_MAX && !words.any(|w| REF.is_match(w.trim_matches('"')))
}

fn learned(f: Failed, name: &str, here: &str, fix: String, t: f64) -> Option<String> {
    if t - f.at > WINDOW_S || f.target != here || f.verb != name || f.fix == fix {
        return None;
    }
    let fix = f.helper.unwrap_or(fix);
    worth(name, &fix).then_some(fix)
}

fn told(l: &Lesson) -> String {
    let times = if l.ups > 1 { format!(", {}x", l.ups) } else { String::new() };
    format!("\n! learned: after this error `wisply {}` worked here before{times}", oneline(&l.fix, 160))
}

pub fn after(d: &Daemon, ctx: &mut Ctx, name: &str, argv: &[String], code: i32, out: &mut String) -> Option<String> {
    let helper = HELPERS.contains(&name);
    if !learnable(name) && !(helper && code == 0) {
        return None;
    }
    TIDY.call_once(|| {
        let _ = with_store(d, |s| s.tidy_lessons(|l| worth(&l.cmd, &l.fix)));
    });
    let (t, here, fix) = (now(), target(ctx), fix_text(argv));
    let offered = ctx.offered.take();
    if code != 0 {
        if let Some(o) = offered.filter(|o| o.fix == fix) {
            let _ = with_store(d, |s| s.vote(&o.sig, false, &ctx.sid));
        }
        let sig = format!("{name}|{here}|{}", kind(out));
        let mut shown = None;
        if let Ok(Some(l)) = with_store(d, |s| s.lesson(&sig))
            && l.fix != fix
        {
            out.push_str(&told(&l));
            shown = Some(l.fix.clone());
            ctx.offered = Some(Offered { sig: sig.clone(), fix: l.fix });
        }
        ctx.failed = Some(Failed { sig, verb: name.to_string(), fix, target: here, at: t, helper: None });
        return shown;
    }
    if let Some(o) = offered.as_ref().filter(|o| o.fix == fix) {
        let _ = with_store(d, |s| s.vote(&o.sig, true, &ctx.sid));
        ctx.failed = None;
        return None;
    }
    if helper && let Some(f) = ctx.failed.as_mut().filter(|f| f.verb != name) {
        if t - f.at <= WINDOW_S && f.target == here {
            f.helper = Some(fix);
        }
        ctx.offered = offered;
        return None;
    }
    let f = ctx.failed.take()?;
    let (sig, target) = (f.sig.clone(), f.target.clone());
    let fix = learned(f, name, &here, fix, t)?;
    let (cmd, rest) = sig.split_once('|').unwrap_or((&sig, ""));
    let kind = rest.split_once('|').map_or("", |(_, k)| k);
    let _ = with_store(d, |s| s.learn(&sig, cmd, &target, kind, &fix, &ctx.sid));
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn failed(verb: &str, fix: &str, helper: Option<&str>) -> Failed {
        let sig = format!("{verb}|github.com|element e# not found");
        let (fix, helper) = (fix.to_string(), helper.map(String::from));
        Failed { sig, verb: verb.into(), fix, target: "github.com".into(), at: 100.0, helper }
    }

    #[test]
    fn errors_get_stable_signatures() {
        assert_eq!(
            kind("err: element e12 not found (\"Log in\") -> try: wisply snap\n! STOP: again"),
            "element e# not found (…)"
        );
        assert_eq!(kind("err: browser did not answer 'click' within 20s"), "browser did not answer … within #s");
        assert_eq!(host("https://www.Example.com:8080/a?b"), "Example.com:8080");
        assert_eq!(fix_text(&["click".into(), "Log in".into(), "".into()]), "click \"Log in\" \"\"");
        assert!(learnable("click") && learnable("open") && !learnable("status") && !learnable("recall"));
        assert!(!learnable("tabs"));
    }

    #[test]
    fn only_real_fixes_are_learned() {
        let hit = |f: Failed, name: &str, fix: &str, t: f64| learned(f, name, "github.com", fix.into(), t);
        assert_eq!(
            hit(failed("click", "click \"Log in\"", None), "click", "click \"Sign in\"", 120.0).as_deref(),
            Some("click \"Sign in\"")
        );
        assert_eq!(
            hit(failed("click", "click e12", Some("snap -f Login")), "click", "click e40", 120.0).as_deref(),
            Some("snap -f Login")
        );
        assert!(hit(failed("click", "click e12", None), "click", "click e40", 120.0).is_none());
        assert!(hit(failed("click", "click x", None), "open", "open https://a.b", 120.0).is_none());
        assert!(hit(failed("click", "click x", None), "click", "click y", 200.0).is_none());
        assert!(hit(failed("click", "click x", None), "click", "click x", 120.0).is_none());
        assert!(
            hit(failed("type", "type x a", None), "type", &format!("type y \"{}\"", "a".repeat(FIX_MAX)), 120.0)
                .is_none()
        );
        assert!(worth("click", "snap") && worth("press", "windows") && worth("type", "type \"Name\" Bob --enter"));
        assert!(!worth("click", "open x") && !worth("click", "scroll e9") && !worth("press", "press w3 Enter"));
    }
}
