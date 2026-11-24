use std::path::Path;

use regex::{Regex, RegexBuilder};
use serde_json::{Value, json};
use wisply_core::Fail;

use super::super::ctx::Ctx;
use super::super::{Daemon, routines};

const SAY: &str = "say";
const MIN_VALUE: usize = 2;
const VALUE_CMDS: &[&str] = &["type", "fill", "search", "open", "launch", "say", "remember", "recall", "go"];

pub struct Hit {
    pub name: String,
    pub pairs: Vec<String>,
}

fn pattern(template: &str) -> Option<(Regex, Vec<String>)> {
    let slot = Regex::new(r"\{(\w+)\}").ok()?;
    let mut re = String::from("^");
    let mut names = Vec::new();
    let mut last = 0;
    let mut literal = String::new();
    for m in slot.captures_iter(template) {
        let whole = m.get(0)?;
        literal.push_str(&template[last..whole.start()]);
        re.push_str(&spaced(&template[last..whole.start()]));
        re.push_str("(.+?)");
        names.push(m[1].to_string());
        last = whole.end();
    }
    literal.push_str(&template[last..]);
    re.push_str(&spaced(&template[last..]));
    re.push('$');
    if literal.split_whitespace().all(|w| w.chars().all(|c| !c.is_alphanumeric())) {
        return None;
    }
    RegexBuilder::new(&re).case_insensitive(true).build().ok().map(|r| (r, names))
}

fn spaced(literal: &str) -> String {
    let words: Vec<String> = literal.split_whitespace().map(regex::escape).collect();
    let mut out = words.join(r"\s+");
    if literal.starts_with(char::is_whitespace) && !words.is_empty() {
        out.insert_str(0, r"\s+");
    }
    if literal.ends_with(char::is_whitespace) && !words.is_empty() {
        out.push_str(r"\s+");
    }
    if words.is_empty() && !literal.is_empty() {
        out = r"\s+".into();
    }
    out
}

fn clean(sentence: &str) -> String {
    sentence.split_whitespace().collect::<Vec<_>>().join(" ").trim_end_matches(['.', '!', '?']).to_string()
}

pub fn matches(routines: &[Value], sentence: &str) -> Option<Hit> {
    let said = clean(sentence);
    for r in routines {
        let Some(name) = r["name"].as_str() else { continue };
        for t in r[SAY].as_array().into_iter().flatten().filter_map(Value::as_str) {
            let Some((re, names)) = pattern(t) else { continue };
            let Some(c) = re.captures(&said) else { continue };
            let pairs = names.iter().enumerate().map(|(i, n)| format!("{n}={}", &c[i + 1])).collect();
            return Some(Hit { name: name.to_string(), pairs });
        }
    }
    None
}

pub fn find(home: &Path, sentence: &str) -> Option<Hit> {
    matches(&routines::load_all(home), sentence)
}

fn word_at(hay: &str, at: usize, len: usize) -> bool {
    let before = hay[..at].chars().next_back().is_none_or(|c| !c.is_alphanumeric());
    let after = hay[at + len..].chars().next().is_none_or(|c| !c.is_alphanumeric());
    before && after
}

fn values(steps: &[Value]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for s in steps {
        let cmd = s["cmd"].as_str().unwrap_or("");
        let args: Vec<&str> = s["args"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
        let picked: Vec<&str> = if cmd == "run" {
            args.iter().filter_map(|a| a.split_once('=').map(|(_, v)| v)).collect()
        } else if VALUE_CMDS.contains(&cmd) {
            args.last().copied().into_iter().collect()
        } else {
            Vec::new()
        };
        for v in picked.into_iter().map(str::trim).filter(|v| v.chars().count() >= MIN_VALUE) {
            if !out.iter().any(|o| o.eq_ignore_ascii_case(v)) {
                out.push(v.to_string());
            }
        }
    }
    out.sort_by_key(|v| std::cmp::Reverse(v.chars().count()));
    out
}

pub fn template(sentence: &str, steps: &[Value]) -> (String, Vec<String>) {
    let mut said = clean(sentence);
    let mut pairs = Vec::new();
    for v in values(steps) {
        let low = said.to_lowercase();
        let want = v.to_lowercase();
        let Some(at) = low.match_indices(&want).map(|(i, _)| i).find(|i| word_at(&low, *i, want.len())) else {
            continue;
        };
        if said.len() != low.len() || said[at..at + want.len()].contains('{') {
            continue;
        }
        let key = format!("p{}", pairs.len() + 1);
        pairs.push(format!("{key}={v}"));
        said.replace_range(at..at + want.len(), &format!("{{{key}}}"));
    }
    (said, pairs)
}

fn slug(template: &str) -> String {
    let words: Vec<String> = template
        .split_whitespace()
        .filter(|w| !w.starts_with('{'))
        .map(|w| w.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase())
        .filter(|w| !w.is_empty())
        .take(4)
        .collect();
    if words.is_empty() { "go-taught".into() } else { format!("go-{}", words.join("-")) }
}

pub fn learn(d: &Daemon, ctx: &mut Ctx, sentence: &str) -> Result<String, Fail> {
    let (steps, _, _) = routines::current_steps(d, ctx)?;
    if steps.is_empty() {
        return Err(Fail::hint("nothing to learn from yet", "do the task once with wisply, then teach the sentence"));
    }
    let (said, pairs) = template(sentence, &steps);
    let mut r = routines::save_episode(d, ctx, &slug(&said), &pairs, None)?;
    let mut say: Vec<Value> = r[SAY].as_array().cloned().unwrap_or_default();
    if !say.iter().any(|s| s.as_str().is_some_and(|s| s.eq_ignore_ascii_case(&said))) {
        say.push(json!(said));
    }
    r[SAY] = Value::Array(say);
    routines::save_one(&d.home, &r)?;
    let name = r["name"].as_str().unwrap_or("").to_string();
    let slots = if pairs.is_empty() { String::new() } else { format!(", fills {}", pairs.join(" ")) };
    Ok(format!("learned \"{said}\" -> routine {name} ({} steps{slots})", steps.len()))
}

pub fn unknown(sentence: &str) -> Fail {
    Fail::hint(
        format!("I don't know \"{}\" yet", clean(sentence)),
        format!("do it once with wisply (one wisply codemode script), then: wisply go --teach \"{}\"", clean(sentence)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn step(cmd: &str, args: &[&str]) -> Value {
        json!({"cmd": cmd, "args": args})
    }

    #[test]
    fn templates_capture_values_from_the_steps() {
        let steps = vec![
            step("open", &["https://shop.example"]),
            step("click", &["Suche"]),
            step("type", &["Suche", "grüner tee"]),
            step("click", &["Bestellen"]),
        ];
        let (t, pairs) = template("bestell grüner tee im Shop", &steps);
        assert_eq!(t, "bestell {p1} im Shop");
        assert_eq!(pairs, vec!["p1=grüner tee".to_string()]);
        let (t, pairs) = template("bestell was", &[step("type", &["Box", "a"])]);
        assert_eq!((t.as_str(), pairs.len()), ("bestell was", 0));
        let (t, _) = template("schick teeee", &[step("type", &["Box", "tee"])]);
        assert_eq!(t, "schick teeee");
    }

    #[test]
    fn said_templates_match_back() {
        let routines = vec![
            json!({"name": "go-bestell-im-shop", "say": ["bestell {p1} im Shop"]}),
            json!({"name": "catch-all", "say": ["{p1}"]}),
            json!({"name": "plain", "say": ["licht an"]}),
        ];
        let hit = matches(&routines, "Bestell  schwarzer Tee im shop!").expect("hit");
        assert_eq!(
            (hit.name.as_str(), hit.pairs.clone()),
            ("go-bestell-im-shop", vec!["p1=schwarzer Tee".to_string()])
        );
        assert!(matches(&routines, "licht an").is_some_and(|h| h.name == "plain" && h.pairs.is_empty()));
        assert!(matches(&routines, "irgendwas anderes").is_none());
        assert_eq!(slug("bestell {p1} im Shop"), "go-bestell-im-shop");
    }
}
