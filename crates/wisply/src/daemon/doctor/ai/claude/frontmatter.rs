const FENCE: &str = "---";
const KEY_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_-";
const NOT_PLAIN: [char; 9] = ['|', '>', '[', '{', '&', '*', '!', '"', '\''];

#[derive(Debug, Default, PartialEq)]
pub struct Front {
    pub keys: Vec<(String, String)>,
}

impl Front {
    pub fn value(&self, key: &str) -> Option<&str> {
        self.keys.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str())
    }

    pub fn has(&self, key: &str) -> bool {
        self.keys.iter().any(|(k, _)| k == key)
    }
}

fn split_key(line: &str) -> Option<(&str, &str)> {
    let (key, rest) = line.split_once(':')?;
    let ok = !key.is_empty() && key.chars().all(|c| KEY_CHARS.contains(c));
    (ok && (rest.is_empty() || rest.starts_with(' '))).then_some((key, rest.trim()))
}

fn quote_problem(value: &str) -> Option<&'static str> {
    let quote = value.chars().next().filter(|c| *c == '"' || *c == '\'')?;
    let body = &value[1..];
    let mut escaped = false;
    for c in body.chars() {
        if quote == '"' && c == '\\' && !escaped {
            escaped = true;
            continue;
        }
        if c == quote && !escaped {
            return None;
        }
        escaped = false;
    }
    Some("a quoted value is never closed")
}

fn value_problem(value: &str) -> Option<&'static str> {
    if value.is_empty() {
        return None;
    }
    if value.starts_with(['"', '\'']) {
        return quote_problem(value);
    }
    if value.starts_with(NOT_PLAIN) || value.starts_with('#') {
        return None;
    }
    (value.contains(": ") || value.ends_with(':')).then_some("an unquoted value contains ': '")
}

pub fn parse(text: &str) -> Result<Option<Front>, String> {
    let text = text.trim_start_matches('\u{feff}');
    let mut lines = text.lines();
    if lines.next().map(str::trim_end) != Some(FENCE) {
        return Ok(None);
    }
    let mut front = Front::default();
    for (n, line) in lines.enumerate() {
        let at = n + 2;
        if line.trim_end() == FENCE {
            return Ok(Some(front));
        }
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            continue;
        }
        if line.starts_with('\t') {
            return Err(format!("line {at} starts with a tab"));
        }
        if line.starts_with([' ', '-']) {
            continue;
        }
        let Some((key, value)) = split_key(line) else { return Err(format!("line {at} is not 'key: value'")) };
        if let Some(why) = value_problem(value) {
            return Err(format!("line {at} ({key}): {why}"));
        }
        front.keys.push((key.to_string(), value.trim_matches(['"', '\'']).to_string()));
    }
    Err("the opening --- has no closing ---".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_blocks_give_their_keys() {
        let text = "---\nname: git-flow\ndescription: Per-feature loop\nallowed-tools:\n  - Bash\nmodel: \"sonnet\"\n---\nbody: not yaml\n";
        let front = parse(text).unwrap().unwrap();
        assert_eq!(front.value("name"), Some("git-flow"));
        assert_eq!(front.value("model"), Some("sonnet"));
        assert!(front.has("allowed-tools") && !front.has("body"));
        assert_eq!(parse("no fence\n").unwrap(), None);
        assert!(parse("---\nname: x\ndescription: >\n  folded: text\n---\n").is_ok());
    }

    #[test]
    fn broken_blocks_name_the_line() {
        let colon = "---\nname: x\ndescription: Use when: it breaks\n---\n";
        assert_eq!(parse(colon).unwrap_err(), "line 3 (description): an unquoted value contains ': '");
        assert_eq!(parse("---\nname: x\nloose words\n---\n").unwrap_err(), "line 3 is not 'key: value'");
        assert_eq!(parse("---\nname: \"x\n---\n").unwrap_err(), "line 2 (name): a quoted value is never closed");
        assert_eq!(parse("---\nname: x\n").unwrap_err(), "the opening --- has no closing ---");
        assert_eq!(parse("---\n\tname: x\n---\n").unwrap_err(), "line 2 starts with a tab");
        assert!(parse("---\nname: \"say \\\"hi\\\"\"\n---\n").is_ok());
    }
}
