pub const HANDOFF: &str = "\u{1}codemode-routine";
const SEP: char = '\u{1f}';
const YES: &str = "--yes";

fn pack(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            SEP => out.push_str("\\u"),
            _ => out.push(ch),
        }
    }
    out
}

fn unpack(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            Some('u') => out.push(SEP),
            Some(other) => out.push(other),
            None => out.push('\\'),
        }
    }
    out
}

pub fn handoff_line(name: &str, args: &[(String, String)], yes: bool) -> String {
    let mut line = format!("{HANDOFF}{SEP}{}", pack(name));
    if yes {
        line.push(SEP);
        line.push_str(YES);
    }
    for (k, v) in args {
        line.push(SEP);
        line.push_str(&pack(&format!("{k}={v}")));
    }
    line
}

pub fn handoff(out: &str) -> Option<(Vec<String>, String)> {
    let (first, rest) = out.split_once('\n').unwrap_or((out, ""));
    let mut parts = first.strip_prefix(HANDOFF)?.strip_prefix(SEP)?.split(SEP);
    let name = unpack(parts.next().filter(|n| !n.is_empty())?);
    let mut argv = vec!["codemode".to_string(), "--routine".to_string(), name];
    for part in parts {
        if part == YES {
            argv.push(YES.to_string());
            continue;
        }
        argv.push("--arg".to_string());
        argv.push(unpack(part));
    }
    Some((argv, rest.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_script_routine_hands_its_name_and_values_over_in_one_line() {
        let args = vec![("pr".to_string(), "101".to_string()), ("note".to_string(), "a\\b\nc\u{1f}d".to_string())];
        let line = handoff_line("ship-pr", &args, false);
        assert!(!line.contains('\n'));
        let (argv, rest) = handoff(&format!("{line}\n! a note")).unwrap();
        assert_eq!(argv, ["codemode", "--routine", "ship-pr", "--arg", "pr=101", "--arg", "note=a\\b\nc\u{1f}d"]);
        assert_eq!(rest, "! a note");
        let (argv, rest) = handoff(&handoff_line("x", &[], true)).unwrap();
        assert_eq!(
            (argv, rest.as_str()),
            (vec!["codemode".into(), "--routine".into(), "x".into(), "--yes".into()], "")
        );
        let (argv, _) = handoff(&handoff_line("x", &[("v".into(), "--yes".into())], false)).unwrap();
        assert_eq!(argv[3..], ["--arg", "v=--yes"]);
        assert_eq!(handoff("ok x: 2 steps"), None);
        assert_eq!(handoff(&format!("{HANDOFF}\u{1f}")), None);
    }
}
