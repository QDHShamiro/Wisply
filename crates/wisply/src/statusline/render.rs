use serde_json::Value;

use super::config::Config;
use super::git::Sync;
use super::github::{Checks, Pr, State};

const ESC: char = '\u{1b}';
const GREY: u8 = 245;
const DIM: u8 = 238;
const GREEN: u8 = 70;
const AMBER: u8 = 136;
const RED: u8 = 160;
const BLUE: u8 = 33;
const WARM_AT: i64 = 60;
const HOT_AT: i64 = 85;
const BAR_CELLS: i64 = 10;
const KILO: f64 = 1000.0;
const DAY: i64 = 86_400;
const HOUR: i64 = 3600;
const MINUTE: i64 = 60;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Git {
    pub branch: String,
    pub repo: Option<String>,
    pub sync: Option<Sync>,
    pub pr: Option<Pr>,
}

fn paint(code: u8, text: &str) -> String {
    format!("{ESC}[38;5;{code}m{text}{ESC}[0m")
}

fn tone(pct: i64) -> u8 {
    if pct < WARM_AT {
        GREEN
    } else if pct < HOT_AT {
        AMBER
    } else {
        RED
    }
}

fn number(v: &Value) -> Option<f64> {
    v.as_f64().or_else(|| v.as_str().and_then(|s| s.trim().parse().ok()))
}

fn percent(v: &Value) -> Option<i64> {
    Some((number(v.get("used_percentage")?)?.round() as i64).clamp(0, 100))
}

fn text<'a>(v: &'a Value, path: &[&str]) -> Option<&'a str> {
    let leaf = path.iter().try_fold(v, |node, key| node.get(*key))?;
    leaf.as_str().filter(|s| !s.is_empty())
}

fn branch_part(git: &Git, cfg: &Config) -> String {
    let mut out = paint(BLUE, &git.branch);
    let Some(sync) = git.sync.filter(|_| cfg.shown("sync")) else { return out };
    if sync.dirty {
        out.push_str(&paint(AMBER, "*"));
    }
    let (up, down) = if cfg.ascii { ("+", "-") } else { ("\u{2191}", "\u{2193}") };
    if sync.ahead > 0 {
        out.push(' ');
        out.push_str(&paint(GREY, &format!("{up}{}", sync.ahead)));
    }
    if sync.behind > 0 {
        out.push(' ');
        out.push_str(&paint(GREY, &format!("{down}{}", sync.behind)));
    }
    out
}

fn github_part(pr: Pr, cfg: &Config) -> String {
    let word = if pr.state == State::Open { "open" } else { pr.state.word() };
    let mut out = paint(GREY, &format!("#{} {word}", pr.number));
    let (mark, color) = match (pr.checks, cfg.ascii) {
        (Checks::Pass, false) => ("\u{2713}", GREEN),
        (Checks::Pass, true) => ("ok", GREEN),
        (Checks::Fail, false) => ("\u{2717}", RED),
        (Checks::Fail, true) => ("x", RED),
        (Checks::Pending, false) => ("\u{2026}", AMBER),
        (Checks::Pending, true) => ("...", AMBER),
        (Checks::None, _) => return out,
    };
    out.push(' ');
    out.push_str(&paint(color, mark));
    out
}

fn context_part(input: &Value) -> Option<String> {
    let pct = percent(input.get("context_window")?)?;
    let filled = ((pct as f64 / 100.0) * BAR_CELLS as f64).round() as i64;
    let color = tone(pct);
    let bar = format!(
        "{}{}",
        paint(color, &"=".repeat(filled as usize)),
        paint(DIM, &"-".repeat((BAR_CELLS - filled) as usize))
    );
    Some(format!("[{bar}] {}", paint(color, &format!("{pct}%"))))
}

fn tokens_part(input: &Value) -> Option<String> {
    let usage = input.get("context_window")?.get("current_usage")?;
    let total: u64 = ["input_tokens", "output_tokens", "cache_creation_input_tokens", "cache_read_input_tokens"]
        .iter()
        .map(|k| usage.get(*k).and_then(Value::as_u64).unwrap_or(0))
        .sum();
    let shown = if total as f64 >= KILO {
        let k = format!("{:.1}", total as f64 / KILO);
        format!("{}k", k.strip_suffix(".0").unwrap_or(&k))
    } else {
        total.to_string()
    };
    Some(paint(GREY, &format!("{shown} tokens")))
}

fn resets(limit: &Value, now: u64, weekly: bool) -> Option<String> {
    let at = number(limit.get("resets_at")?)? as i64;
    let left = at - now as i64;
    if left <= 0 {
        return None;
    }
    let span = if weekly {
        format!("{}d{}h", left / DAY, (left % DAY) / HOUR)
    } else {
        format!("{}h{}m", left / HOUR, (left % HOUR) / MINUTE)
    };
    Some(paint(DIM, &format!("resets in {span}")))
}

fn limit_part(limit: &Value, now: u64, weekly: bool) -> Option<String> {
    let used = percent(limit)?;
    let color = tone(used);
    let mut out = String::new();
    if weekly {
        out.push_str(&paint(GREY, "week "));
    }
    out.push_str(&paint(color, &format!("{used}% used")));
    out.push(' ');
    out.push_str(&paint(DIM, "("));
    out.push_str(&paint(GREEN, &format!("{}% left", 100 - used)));
    out.push_str(&paint(DIM, ")"));
    if let Some(reset) = resets(limit, now, weekly) {
        out.push(' ');
        out.push_str(&reset);
    }
    Some(out)
}

struct Ctx<'a> {
    input: &'a Value,
    git: Option<&'a Git>,
    now: u64,
    cfg: &'a Config,
}

type Part = fn(&Ctx) -> Option<String>;

fn repo_part(c: &Ctx) -> Option<String> {
    Some(paint(GREY, c.git?.repo.as_deref()?))
}

fn branch_slot(c: &Ctx) -> Option<String> {
    Some(branch_part(c.git?, c.cfg))
}

fn github_slot(c: &Ctx) -> Option<String> {
    Some(github_part(c.git?.pr?, c.cfg))
}

fn model_part(c: &Ctx) -> Option<String> {
    Some(paint(GREY, text(c.input, &["model", "display_name"])?))
}

fn effort_part(c: &Ctx) -> Option<String> {
    let level = text(c.input, &["effort", "level"])?;
    Some(paint(AMBER, &format!("effort:{}", level.to_uppercase())))
}

fn context_slot(c: &Ctx) -> Option<String> {
    context_part(c.input)
}

fn tokens_slot(c: &Ctx) -> Option<String> {
    tokens_part(c.input)
}

fn five_hour_part(c: &Ctx) -> Option<String> {
    limit_part(c.input.get("rate_limits")?.get("five_hour")?, c.now, false)
}

fn week_part(c: &Ctx) -> Option<String> {
    limit_part(c.input.get("rate_limits")?.get("seven_day")?, c.now, true)
}

const PARTS: [(&str, Part); 9] = [
    ("repo", repo_part),
    ("branch", branch_slot),
    ("github", github_slot),
    ("model", model_part),
    ("effort", effort_part),
    ("context", context_slot),
    ("tokens", tokens_slot),
    ("limit5h", five_hour_part),
    ("limitweek", week_part),
];

pub fn line(input: &Value, git: Option<&Git>, now: u64, cfg: &Config) -> String {
    let ctx = Ctx { input, git, now, cfg };
    let parts: Vec<String> =
        PARTS.iter().filter(|(name, _)| cfg.shown(name)).filter_map(|(_, make)| make(&ctx)).collect();
    parts.join(&format!(" {} ", paint(DIM, "|")))
}

#[cfg(test)]
pub fn plain(s: &str) -> String {
    let mut out = String::new();
    let mut skipping = false;
    for c in s.chars() {
        if c == ESC {
            skipping = true;
        } else if skipping {
            skipping = c != 'm';
        } else {
            out.push(c);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    const NOW: u64 = 1_800_000_000;

    fn sample() -> Value {
        json!({
            "model": {"display_name": "Opus"},
            "effort": {"level": "high"},
            "context_window": {"used_percentage": 42.4, "current_usage": {
                "input_tokens": 1000, "output_tokens": 200,
                "cache_creation_input_tokens": 3000, "cache_read_input_tokens": 30000}},
            "rate_limits": {
                "five_hour": {"used_percentage": 12.2, "resets_at": NOW + 3 * 3600 + 10 * 60},
                "seven_day": {"used_percentage": 40, "resets_at": NOW + 2 * 86_400 + 4 * 3600}}
        })
    }

    fn git() -> Git {
        Git {
            branch: "feat/x".into(),
            repo: Some("o/r".into()),
            sync: Some(Sync { dirty: true, ahead: 2, behind: 1 }),
            pr: Some(Pr { number: 41, state: State::Open, checks: Checks::Pass }),
        }
    }

    #[test]
    fn the_full_line_matches_the_old_script_plus_github() {
        let out = line(&sample(), Some(&git()), NOW, &Config::defaults());
        assert_eq!(
            plain(&out),
            "feat/x* \u{2191}2 \u{2193}1 | #41 open \u{2713} | Opus | effort:HIGH | [====------] 42% | 34.2k tokens | 12% used (88% left) resets in 3h10m | week 40% used (60% left) resets in 2d4h"
        );
    }

    #[test]
    fn segments_switch_off_and_ascii_replaces_the_glyphs() {
        let mut cfg = Config::defaults();
        cfg.set("repo", true).unwrap();
        cfg.set("tokens", false).unwrap();
        cfg.set("limitweek", false).unwrap();
        cfg.set("limit5h", false).unwrap();
        cfg.set("context", false).unwrap();
        cfg.set("ascii", true).unwrap();
        let out = plain(&line(&sample(), Some(&git()), NOW, &cfg));
        assert_eq!(out, "o/r | feat/x* +2 -1 | #41 open ok | Opus | effort:HIGH");
        cfg.set("sync", false).unwrap();
        cfg.set("github", false).unwrap();
        assert_eq!(plain(&line(&sample(), Some(&git()), NOW, &cfg)), "o/r | feat/x | Opus | effort:HIGH");
    }

    #[test]
    fn missing_data_leaves_segments_out_and_never_fails() {
        let cfg = Config::defaults();
        assert_eq!(line(&Value::Null, None, NOW, &cfg), "");
        assert_eq!(plain(&line(&json!({"model": {"display_name": "Test"}}), None, NOW, &cfg)), "Test");
        let odd = json!({"context_window": {"used_percentage": -5}, "rate_limits": {"five_hour": {"used_percentage": "250"}}});
        let out = plain(&line(&odd, None, NOW, &cfg));
        assert_eq!(out, "[----------] 0% | 100% used (0% left)");
        let bare = Git { branch: "main".into(), ..Git::default() };
        assert_eq!(plain(&line(&Value::Null, Some(&bare), NOW, &cfg)), "main");
        let expired = json!({"rate_limits": {"five_hour": {"used_percentage": 10, "resets_at": NOW - 5}}});
        assert_eq!(plain(&line(&expired, None, NOW, &cfg)), "10% used (90% left)");
    }

    #[test]
    fn colors_follow_the_thresholds() {
        assert_eq!(
            (tone(0), tone(59), tone(60), tone(84), tone(85), tone(100)),
            (GREEN, GREEN, AMBER, AMBER, RED, RED)
        );
        let small = json!({"context_window": {"current_usage": {"input_tokens": 999}}});
        assert_eq!(plain(&line(&small, None, NOW, &Config::defaults())), "999 tokens");
        let round = json!({"context_window": {"current_usage": {"input_tokens": 34_000}}});
        assert_eq!(plain(&line(&round, None, NOW, &Config::defaults())), "34k tokens");
        let draft = Pr { number: 5, state: State::Draft, checks: Checks::Pending };
        assert_eq!(plain(&github_part(draft, &Config::defaults())), "#5 draft \u{2026}");
    }
}
