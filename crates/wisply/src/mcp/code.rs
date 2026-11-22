use serde_json::Value;

const HELP_WORDS: [&str; 3] = ["help", "-h", "--help"];
const QUOTES: [char; 2] = ['"', '\''];

#[derive(Debug, PartialEq)]
pub struct Code {
    pub argv: Vec<String>,
    pub input: Option<String>,
}

pub fn from_args(args: &Value) -> Option<Code> {
    let code = args.get("code").and_then(Value::as_str)?;
    let mut argv = vec!["codemode".to_string()];
    if args.get("yes").and_then(Value::as_bool) == Some(true) {
        argv.push("--yes".into());
    }
    let limit = match args.get("timeout") {
        Some(Value::String(text)) if !text.trim().is_empty() => Some(text.trim().to_string()),
        Some(Value::Number(n)) => Some(n.to_string()),
        _ => None,
    };
    if let Some(limit) = limit {
        argv.push(format!("--timeout={limit}"));
    }
    Some(finish(argv, code.trim()))
}

pub fn from_line(line: &str) -> Option<Code> {
    let mut rest = line.trim();
    if let Some(after) = word(rest, "wisply").or_else(|| word(rest, "lighting")) {
        rest = after;
    }
    rest = word(rest, "codemode")?;
    let mut argv = vec!["codemode".to_string()];
    loop {
        let (first, after) = split(rest);
        let takes = matches!(first, "--timeout" | "--file");
        let flag = matches!(first, "--yes" | "-y") || first.starts_with("--timeout=") || first.starts_with("--file=");
        if !takes && !flag {
            break;
        }
        argv.push(first.to_string());
        rest = after;
        if takes {
            let (value, after) = value(rest);
            argv.push(value);
            rest = after;
        }
    }
    Some(finish(argv, rest))
}

fn finish(mut argv: Vec<String>, code: &str) -> Code {
    let has_file = argv.iter().any(|a| a.starts_with("--file"));
    if HELP_WORDS.contains(&code) || (code.is_empty() && !has_file) {
        argv.push("help".into());
        return Code { argv, input: None };
    }
    if code.is_empty() {
        return Code { argv, input: None };
    }
    argv.push("-".into());
    Code { argv, input: Some(unquote(code)) }
}

fn word<'a>(text: &'a str, name: &str) -> Option<&'a str> {
    let after = text.strip_prefix(name)?;
    (after.is_empty() || after.starts_with(char::is_whitespace)).then(|| after.trim_start())
}

fn split(text: &str) -> (&str, &str) {
    match text.split_once(char::is_whitespace) {
        Some((first, after)) => (first, after.trim_start()),
        None => (text, ""),
    }
}

fn value(text: &str) -> (String, &str) {
    if let Some(q) = text.chars().next().filter(|c| QUOTES.contains(c))
        && let Some(end) = text[1..].find(q)
    {
        return (text[1..=end].to_string(), text[end + 2..].trim_start());
    }
    let (first, after) = split(text);
    (first.to_string(), after)
}

fn unquote(text: &str) -> String {
    for q in QUOTES {
        let Some(inner) = text.strip_prefix(q).and_then(|t| t.strip_suffix(q)) else { continue };
        let escaped = format!("\\{q}");
        if !inner.replace(&escaped, "").contains(q) {
            return inner.replace(&escaped, &q.to_string());
        }
    }
    text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn code(argv: &[&str], input: Option<&str>) -> Option<Code> {
        Some(Code { argv: argv.iter().map(|a| (*a).to_string()).collect(), input: input.map(str::to_string) })
    }

    #[test]
    fn the_code_argument_runs_code_mode_with_its_options() {
        let args = json!({"code": " await $`ls` ", "yes": true, "timeout": "90s"});
        assert_eq!(from_args(&args), code(&["codemode", "--yes", "--timeout=90s", "-"], Some("await $`ls`")));
        assert_eq!(
            from_args(&json!({"code": "1", "timeout": 30})),
            code(&["codemode", "--timeout=30", "-"], Some("1"))
        );
        assert_eq!(from_args(&json!({"code": ""})), code(&["codemode", "help"], None));
        assert_eq!(from_args(&json!({"cmd": "snap"})), None);
    }

    #[test]
    fn a_codemode_line_keeps_the_javascript_as_it_is() {
        let js = "const a = await wisply`snap -f \"x\"`; a.split('\\n').length";
        assert_eq!(from_line(&format!("codemode {js}")), code(&["codemode", "-"], Some(js)));
        assert_eq!(
            from_line(&format!("wisply codemode --yes --timeout 2m {js}")),
            code(&["codemode", "--yes", "--timeout", "2m", "-"], Some(js))
        );
        assert_eq!(
            from_line("codemode \"await $`echo \\\"hi\\\"`\""),
            code(&["codemode", "-"], Some("await $`echo \"hi\"`"))
        );
        assert_eq!(from_line("codemode 'a' + 'b'"), code(&["codemode", "-"], Some("'a' + 'b'")));
        assert_eq!(from_line("codemode --file 'my script.js'"), code(&["codemode", "--file", "my script.js"], None));
        assert_eq!(from_line("codemode"), code(&["codemode", "help"], None));
        assert_eq!(from_line("codemode --help"), code(&["codemode", "help"], None));
        assert_eq!(from_line("codemodes x"), None);
        assert_eq!(from_line("snap -f codemode"), None);
    }
}
