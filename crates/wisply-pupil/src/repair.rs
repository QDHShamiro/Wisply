use serde_json::Value;

fn closer(open: char) -> char {
    if open == '{' { '}' } else { ']' }
}

pub fn close_cut(text: &str) -> Option<String> {
    let mut stack: Vec<char> = Vec::new();
    let mut in_string = false;
    let mut escape = false;
    let mut best: Option<(usize, Vec<char>)> = None;
    for (at, c) in text.char_indices() {
        if in_string {
            if escape {
                escape = false;
            } else if c == '\\' {
                escape = true;
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        match c {
            '"' => in_string = true,
            '{' | '[' => stack.push(c),
            '}' | ']' => {
                stack.pop()?;
                if stack.last() == Some(&'[') {
                    best = Some((at + c.len_utf8(), stack.clone()));
                }
            }
            _ => {}
        }
    }
    let (end, open) = best?;
    let mut fixed: String = text[..end].to_string();
    fixed.extend(open.iter().rev().map(|c| closer(*c)));
    serde_json::from_str::<Value>(&fixed).ok()?;
    Some(fixed)
}

pub fn compact(text: &str) -> Option<String> {
    let value: Value = serde_json::from_str(text).ok()?;
    serde_json::to_string(&value).ok()
}

#[cfg(test)]
mod tests {
    use super::{close_cut, compact};

    #[test]
    fn a_cut_answer_keeps_every_finished_step() {
        let cut = r#"{"steps":[{"t":"00:01","do":"Open"},{"t":"00:05","do":"Sa"#;
        assert_eq!(close_cut(cut).as_deref(), Some(r#"{"steps":[{"t":"00:01","do":"Open"}]}"#));
    }

    #[test]
    fn brackets_inside_strings_do_not_count() {
        let cut = r#"{"sum":"a } ] \" {","pts":[{"t":"00:01","x":"[x]"},{"t":"0"#;
        assert_eq!(close_cut(cut).as_deref(), Some(r#"{"sum":"a } ] \" {","pts":[{"t":"00:01","x":"[x]"}]}"#));
    }

    #[test]
    fn nested_lists_close_in_the_right_order() {
        let cut = "{\n  \"steps\": [\n    {\"t\": \"00:01\", \"keys\": [\"Ctrl\", \"B\"]},\n    {\"t\": \"00:02\", \"keys\": [\"Sh";
        let fixed = close_cut(cut).expect("fixed");
        let value: serde_json::Value = serde_json::from_str(&fixed).expect("json");
        assert_eq!(value["steps"][0]["keys"][1], "B");
        assert_eq!(value["steps"].as_array().map(Vec::len), Some(1));
    }

    #[test]
    fn nothing_finished_means_nothing_to_save() {
        assert_eq!(close_cut(r#"{"steps":[{"t":"00:0"#), None);
        assert_eq!(close_cut(""), None);
        assert_eq!(close_cut("}"), None);
    }

    #[test]
    fn pretty_json_is_made_compact() {
        assert_eq!(compact("{\n  \"t\": \"00:01\",\n  \"do\": \"x\"\n}").as_deref(), Some(r#"{"t":"00:01","do":"x"}"#));
        assert_eq!(compact("{broken"), None);
    }
}
