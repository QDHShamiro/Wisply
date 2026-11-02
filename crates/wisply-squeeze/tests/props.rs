use proptest::collection::{btree_map, vec};
use proptest::prelude::*;
use serde_json::{Map, Value};
use wisply_squeeze::{squeeze, squeeze_tool, squeeze_web};

const SAFE: [&str; 10] = ["alpha", "beta", "gamma", "delta", "step", "done", "row", "value", "ok", "item"];

fn filler() -> impl Strategy<Value = String> {
    vec(prop::sample::select(SAFE.to_vec()), 2..8).prop_map(|w| w.join(" "))
}

fn noisy_line() -> impl Strategy<Value = String> {
    prop_oneof![
        "[ -~]{0,90}",
        "\\PC{0,40}",
        "[a-z ]{0,20}\r[a-z %0-9]{0,20}",
        "\x1b\\[3[0-7]m[a-z]{1,9}\x1b\\[0m error [0-9]{1,4}",
        "\tat [a-z.]{1,30}\\([A-Z][a-z]+\\.java:[0-9]{1,4}\\)",
        "\\{\"[a-z]{1,6}\": [0-9]{1,5}\\}",
        Just(String::new()),
    ]
}

fn json_value() -> impl Strategy<Value = Value> {
    let leaf = prop_oneof![
        Just(Value::Null),
        any::<bool>().prop_map(Value::from),
        any::<i64>().prop_map(Value::from),
        (-1e12f64..1e12).prop_map(Value::from),
        "\\PC{0,60}".prop_map(Value::from),
        Just(Value::from("fatal error: disk full")),
    ];
    leaf.prop_recursive(4, 96, 10, |inner| {
        prop_oneof![
            vec(inner.clone(), 0..14).prop_map(Value::Array),
            btree_map("[a-z]{1,6}", inner, 0..7).prop_map(|m| Value::Object(m.into_iter().collect::<Map<_, _>>())),
        ]
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn never_larger_and_the_same_every_time(lines in vec(noisy_line(), 0..400), crlf in any::<bool>()) {
        let text = lines.join(if crlf { "\r\n" } else { "\n" });
        let first = squeeze(&text);
        let again = squeeze(&text);
        prop_assert_eq!(first.as_ref().map(|s| &s.body), again.as_ref().map(|s| &s.body));
        if let Some(s) = first {
            prop_assert!(s.chars_out < s.chars_in);
            prop_assert!(s.chars_in <= text.chars().count());
            prop_assert_eq!(s.chars_out, s.body.chars().count());
        }
    }

    #[test]
    fn error_lines_stay(lines in vec(filler(), 150..400), errors in btree_map(0usize..400, "[a-z]{6}", 1..8)) {
        let mut lines = lines;
        let mut want = Vec::new();
        for (at, tag) in errors {
            let line = format!("ERROR {tag} failed");
            let at = at % lines.len();
            lines.insert(at, line.clone());
            want.push(line);
        }
        let s = squeeze(&lines.join("\n")).expect("a long log is squeezed");
        for line in want {
            prop_assert!(s.body.lines().any(|l| l.starts_with(&line)), "lost {line}");
        }
    }

    #[test]
    fn json_error_rows_stay(n in 70usize..300, bad in 0usize..300) {
        let bad = bad % n;
        let rows: Vec<String> = (0..n)
            .map(|i| {
                let state = if i == bad { "crashed with a fatal error" } else { "fine" };
                format!("{{\"id\":{i},\"state\":\"{state}\"}}")
            })
            .collect();
        let s = squeeze(&rows.join("\n")).expect("json lines are squeezed");
        prop_assert!(s.json);
        prop_assert!(s.body.contains("crashed with a fatal error"));
    }

    #[test]
    fn short_outputs_keep_the_start_of_error_lines(
        n in 5usize..58,
        wide in 400usize..900,
        errors in btree_map(0usize..58, "[a-z]{6}", 1..5),
    ) {
        let mut lines: Vec<String> = (0..n).map(|i| format!("row {i} {}", "w".repeat(wide))).collect();
        let mut want = Vec::new();
        for (at, tag) in errors {
            let line = format!("ERROR {tag} failed {}", "e".repeat(wide));
            lines.insert(at % lines.len(), line.clone());
            want.push(line);
        }
        let s = squeeze(&lines.join("
")).expect("wide lines are cut");
        prop_assert!(s.chars_out < s.chars_in);
        for line in want {
            let kept: String = line.chars().take(300).collect();
            prop_assert!(s.body.lines().any(|l| l.starts_with(&kept)), "lost {line}");
        }
    }

    #[test]
    fn grep_results_keep_error_hits(
        files in 2usize..8,
        hits in 8usize..30,
        bad in btree_map((0usize..8, 0usize..30), "[a-z]{6}", 1..6),
    ) {
        let mut want = Vec::new();
        let mut lines = Vec::new();
        for f in 0..files {
            for h in 0..hits {
                let text = match bad.get(&(f, h)) {
                    Some(tag) => {
                        let line = format!("src/module{f}/file.rs:{}:    panic!(\"fatal {tag}\");", h + 1);
                        want.push(line.clone());
                        line
                    }
                    None => format!("src/module{f}/file.rs:{}:    let x{h} = call({h}, {f});", h + 1),
                };
                lines.push(text);
            }
        }
        let text = lines.join("
");
        if let Some(s) = squeeze_tool("Grep", &text) {
            prop_assert!(s.chars_out < s.chars_in);
            for line in want {
                let (path, rest) = line.split_once(':').expect("path");
                let (num, body) = rest.split_once(':').expect("number");
                prop_assert!(
                    s.body.lines().any(|l| l == line || l == format!("  {num}:{body}")),
                    "lost {path}:{num}"
                );
            }
        }
    }

    #[test]
    fn web_text_is_never_larger_and_keeps_errors(
        paras in vec(filler(), 20..60),
        errors in btree_map(0usize..60, "[a-z]{6}", 1..5),
    ) {
        let mut lines: Vec<String> = paras.iter().enumerate().map(|(i, p)| format!("{p} part {i} {}", "text ".repeat(20))).collect();
        let mut want = Vec::new();
        for (at, tag) in errors {
            let line = format!("Error {tag} means the request failed.");
            lines.insert(at % lines.len(), line.clone());
            want.push(line);
        }
        let text = lines.join("

");
        if let Some(s) = squeeze_web(&text) {
            prop_assert!(s.chars_out < s.chars_in);
            for line in want {
                prop_assert!(s.body.lines().any(|l| l == line), "lost {line}");
            }
        }
    }

    #[test]
    fn any_json_is_safe(v in json_value()) {
        let text = serde_json::to_string_pretty(&v).expect("json text");
        let first = squeeze(&text);
        let again = squeeze(&text);
        prop_assert_eq!(first.as_ref().map(|s| &s.body), again.as_ref().map(|s| &s.body));
        if let Some(s) = first {
            prop_assert!(s.chars_out < s.chars_in);
        }
    }
}
