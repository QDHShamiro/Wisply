use serde_json::Value;

const STRING_CHAR: &str = r#"[^"\\\x7F\x00-\x1F] | "\\" (["\\/bfnrt] | "u" [0-9a-fA-F]{4})"#;
const INTEGER: &str = r#""-"? ("0" | [1-9] [0-9]{0,15})"#;
const NUMBER: &str = r#""-"? ("0" | [1-9] [0-9]{0,15}) ("." [0-9]{1,16})? ([eE] [-+]? [0-9]{1,4})?"#;
const BOOLEAN: &str = r#""true" | "false""#;

pub fn compact(schema: &Value) -> Option<String> {
    let mut rules = Rules::default();
    let root = rules.value(schema)?;
    Some(rules.finish(&root))
}

#[derive(Default)]
struct Rules {
    list: Vec<(String, String)>,
}

impl Rules {
    fn add(&mut self, body: String) -> String {
        if let Some((name, _)) = self.list.iter().find(|(_, known)| *known == body) {
            return name.clone();
        }
        let name = format!("r{}", self.list.len());
        self.list.push((name.clone(), body));
        name
    }

    fn finish(self, root: &str) -> String {
        let mut text = format!("root ::= {root}\n");
        for (name, body) in &self.list {
            text.push_str(&format!("{name} ::= {body}\n"));
        }
        text
    }

    fn value(&mut self, schema: &Value) -> Option<String> {
        if let Some(options) = schema.get("enum") {
            return self.choices(options);
        }
        match schema.get("type")?.as_str()? {
            "object" => self.object(schema),
            "array" => self.array(schema),
            "string" => self.string(schema),
            "boolean" => Some(self.add(BOOLEAN.to_string())),
            "integer" => Some(self.add(INTEGER.to_string())),
            "number" => Some(self.add(NUMBER.to_string())),
            _ => None,
        }
    }

    fn choices(&mut self, options: &Value) -> Option<String> {
        let words: Option<Vec<String>> = options
            .as_array()?
            .iter()
            .map(|option| serde_json::to_string(option).ok().map(|raw| literal(&raw)))
            .collect();
        let words = words?;
        if words.is_empty() {
            return None;
        }
        Some(self.add(words.join(" | ")))
    }

    fn string(&mut self, schema: &Value) -> Option<String> {
        let inner = match schema.get("pattern") {
            Some(pattern) => pattern_rule(pattern.as_str()?)?,
            None => format!("{}*", self.add(STRING_CHAR.to_string())),
        };
        Some(self.add(format!(r#""\"" {inner} "\"""#)))
    }

    fn array(&mut self, schema: &Value) -> Option<String> {
        let item = self.value(schema.get("items")?)?;
        let least = schema.get("minItems").map(Value::as_u64).unwrap_or(Some(0))?;
        let most = match schema.get("maxItems") {
            Some(found) => Some(found.as_u64()?),
            None => None,
        };
        if most == Some(0) {
            return Some(self.add(r#""[" "]""#.to_string()));
        }
        if most.is_some_and(|most| most < least) {
            return None;
        }
        let more = match most {
            Some(most) => format!("{{{},{}}}", least.saturating_sub(1), most.saturating_sub(1)),
            None if least > 1 => format!("{{{},}}", least - 1),
            None => "*".to_string(),
        };
        let list = format!(r#"{item} ("," {item}){more}"#);
        let list = if least == 0 { format!("({list})?") } else { list };
        Some(self.add(format!(r#""[" {list} "]""#)))
    }

    fn object(&mut self, schema: &Value) -> Option<String> {
        let empty = serde_json::Map::new();
        let properties = match schema.get("properties") {
            Some(found) => found.as_object()?,
            None => &empty,
        };
        let required: Vec<&str> = schema
            .get("required")
            .and_then(Value::as_array)
            .map(|names| names.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();
        if required.iter().any(|name| !properties.contains_key(*name)) {
            return None;
        }
        let mut must = Vec::new();
        let mut may = Vec::new();
        for (name, field) in properties {
            let key = literal(&format!("{}:", serde_json::to_string(name).ok()?));
            if required.contains(&name.as_str()) {
                must.push(format!("{key} {}", self.value(field)?));
            } else {
                may.push(format!("{key} {}", self.value(field)?));
            }
        }
        let inner = if must.is_empty() {
            optional_only(&may)
        } else {
            let tail: String = may.iter().map(|pair| format!(r#" ("," {pair})?"#)).collect();
            format!(r#"{}{tail}"#, must.join(r#" "," "#))
        };
        Some(self.add(format!(r#""{{" {inner} "}}""#)))
    }
}

fn optional_only(may: &[String]) -> String {
    if may.is_empty() {
        return String::new();
    }
    let starts: Vec<String> = (0..may.len())
        .map(|first| {
            let tail: String = may[first + 1..].iter().map(|pair| format!(r#" ("," {pair})?"#)).collect();
            format!("{}{tail}", may[first])
        })
        .collect();
    format!("({})?", starts.join(" | "))
}

fn literal(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

fn pattern_rule(pattern: &str) -> Option<String> {
    let body = pattern.strip_prefix('^')?.strip_suffix('$')?;
    let chars: Vec<char> = body.chars().collect();
    let mut at = 0;
    let rule = alternatives(&chars, &mut at)?;
    (at == chars.len()).then_some(rule)
}

fn alternatives(chars: &[char], at: &mut usize) -> Option<String> {
    let mut options = vec![sequence(chars, at)?];
    while chars.get(*at) == Some(&'|') {
        *at += 1;
        options.push(sequence(chars, at)?);
    }
    Some(if options.len() == 1 { options.remove(0) } else { format!("({})", options.join(" | ")) })
}

fn sequence(chars: &[char], at: &mut usize) -> Option<String> {
    let mut parts = Vec::new();
    while let Some(&c) = chars.get(*at) {
        if c == '|' || c == ')' {
            break;
        }
        let atom = atom(chars, at)?;
        parts.push(format!("{atom}{}", quantifier(chars, at)?));
    }
    if parts.is_empty() {
        return None;
    }
    Some(parts.join(" "))
}

fn atom(chars: &[char], at: &mut usize) -> Option<String> {
    let c = *chars.get(*at)?;
    *at += 1;
    match c {
        '(' => {
            if chars.get(*at) == Some(&'?') {
                if chars.get(*at + 1) != Some(&':') {
                    return None;
                }
                *at += 2;
            }
            let inner = alternatives(chars, at)?;
            if chars.get(*at) != Some(&')') {
                return None;
            }
            *at += 1;
            Some(format!("({inner})"))
        }
        '[' => class(chars, at),
        '\\' => {
            let escaped = *chars.get(*at)?;
            *at += 1;
            match escaped {
                'd' => Some("[0-9]".to_string()),
                '.' | '-' | ':' | '/' | '(' | ')' | '[' | ']' | '{' | '}' | '?' | '*' | '+' | '|' | '^' | '$' => {
                    Some(literal(&escaped.to_string()))
                }
                _ => None,
            }
        }
        '"' | '.' | '^' | '$' | '*' | '+' | '?' | '{' | '}' | ']' => None,
        _ if c.is_control() => None,
        _ => Some(literal(&c.to_string())),
    }
}

fn class(chars: &[char], at: &mut usize) -> Option<String> {
    let mut inner = String::new();
    loop {
        let c = *chars.get(*at)?;
        *at += 1;
        match c {
            ']' if !inner.is_empty() => return Some(format!("[{inner}]")),
            '^' if inner.is_empty() => return None,
            '"' | '\\' | '[' | ']' => return None,
            _ if c.is_control() => return None,
            _ => inner.push(c),
        }
    }
}

fn quantifier(chars: &[char], at: &mut usize) -> Option<String> {
    match chars.get(*at) {
        Some(&c @ ('?' | '*' | '+')) => {
            *at += 1;
            Some(c.to_string())
        }
        Some('{') => {
            let close = chars[*at..].iter().position(|&c| c == '}')? + *at;
            let inside: String = chars[*at + 1..close].iter().collect();
            let valid = !inside.is_empty()
                && inside.chars().all(|c| c.is_ascii_digit() || c == ',')
                && inside.matches(',').count() <= 1
                && !inside.starts_with(',');
            *at = close + 1;
            valid.then(|| format!("{{{inside}}}"))
        }
        _ => Some(String::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::{compact, pattern_rule};
    use serde_json::json;

    #[test]
    fn the_time_pattern_becomes_a_rule() {
        assert_eq!(
            pattern_rule("^([0-9]{1,2}:)?[0-5][0-9]:[0-5][0-9]$").as_deref(),
            Some(r#"([0-9]{1,2} ":")? [0-5] [0-9] ":" [0-5] [0-9]"#)
        );
    }

    #[test]
    fn unknown_patterns_are_refused() {
        assert_eq!(pattern_rule("[0-9]+"), None);
        assert_eq!(pattern_rule("^[^a]$"), None);
        assert_eq!(pattern_rule("^a\\sb$"), None);
        assert_eq!(pattern_rule("^(?=a)$"), None);
        assert_eq!(pattern_rule("^a\"b$"), None);
    }

    #[test]
    fn the_grammar_has_no_room_for_spaces() {
        let point = json!({"type": "object", "required": ["t", "x"], "properties": {
            "t": {"type": "string", "pattern": "^[0-5][0-9]:[0-5][0-9]$"},
            "x": {"type": "string"},
            "ok": {"type": "boolean"}
        }});
        let list = json!({"type": "array", "maxItems": 3, "items": point});
        let schema = json!({"type": "object", "required": ["sum", "pts"], "properties": {
            "sum": {"type": "string"}, "pts": list
        }});
        let grammar = compact(&schema).expect("grammar");
        assert!(grammar.starts_with("root ::= "));
        assert!(grammar.contains(r#""\"pts\":""#));
        assert!(grammar.contains(r#"("," r"#));
        assert!(grammar.contains("{0,2}"));
        assert!(grammar.contains(r#"("," "\"ok\":" r"#));
        assert!(!grammar.contains("space"));
    }

    #[test]
    fn extras_may_stay_empty() {
        let step = json!({"type": "object", "required": ["do"], "properties": {
            "do": {"type": "string"},
            "keys": {"type": "array", "maxItems": 8, "items": {"type": "string"}},
            "code": {"type": "string"},
            "unsure": {"type": "boolean"}
        }});
        let grammar = compact(&step).expect("grammar");
        assert!(grammar.contains(r#""[" (r1 ("," r1){0,7})? "]""#));
        assert!(grammar.contains(r#""\"" r0* "\"""#));
        assert!(!grammar.contains(r#"r0+"#));
        assert!(grammar.contains(r#""true" | "false""#));
    }

    #[test]
    fn objects_without_required_fields_still_work() {
        let schema = json!({"type": "object", "properties": {
            "a": {"type": "boolean"}, "b": {"type": "boolean"}
        }});
        let grammar = compact(&schema).expect("grammar");
        assert!(grammar.contains(r#"("\"a\":" r0 ("," "\"b\":" r0)? | "\"b\":" r0)?"#));
    }

    #[test]
    fn unknown_types_fall_back_to_the_server() {
        assert_eq!(compact(&json!({"type": "null"})), None);
        assert_eq!(compact(&json!({"anyOf": []})), None);
        assert_eq!(compact(&json!({"type": "object", "properties": {}, "required": ["x"]})), None);
    }
}
