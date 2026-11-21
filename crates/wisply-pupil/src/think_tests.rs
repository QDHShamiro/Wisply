use super::{Picker, Reply, Row, grade, options, prompt, summary, tail};
use crate::config::{Config, ThinkEval};

fn spec() -> ThinkEval {
    let config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
    config.think_eval
}

fn row(kind: &str, options: &[&str], answer: &str, group: &str) -> Row {
    Row {
        id: format!("{kind}-{answer}"),
        group: group.to_string(),
        kind: kind.to_string(),
        question: "Q?".to_string(),
        options: options.iter().map(|o| o.to_string()).collect(),
        answer: answer.to_string(),
    }
}

fn args(raw: &[&str]) -> Vec<String> {
    raw.iter().map(|a| a.to_string()).collect()
}

#[test]
fn options_need_a_label_and_budget_needs_think() {
    assert!(options(&[]).is_err());
    assert!(options(&args(&["--label", "x", "--budget", "128"])).is_err());
    let found = options(&args(&["--label", "x", "--think", "--budget", "128", "--runs", "3"])).expect("parses");
    assert_eq!((found.think, found.budget, found.runs), (true, Some(128), 3));
    assert_eq!(found.probs, None);
    let odds = options(&args(&["--label", "x", "--probs", "8"])).expect("parses");
    assert_eq!(odds.probs, Some(8));
    assert!(options(&args(&["--label", "x", "--probs", "many"])).is_err());
}

#[test]
fn auto_lets_the_router_choose_and_never_mixes_with_think() {
    let auto = options(&args(&["--label", "x", "--auto", "--budget", "256"])).expect("parses");
    assert_eq!((auto.think, auto.auto, auto.budget), (false, true, Some(256)));
    assert!(options(&args(&["--label", "x", "--auto", "--think"])).is_err());
}

#[test]
fn the_letter_prompt_lists_every_option_with_its_letter() {
    let spec = spec();
    let text = prompt(&spec, &row("letter", &["10", "20", "30"], "B", "mathe"));
    assert!(text.contains("A. 10\nB. 20\nC. 30"));
    assert!(text.starts_with("Q?"));
    let number = prompt(&spec, &row("number", &[], "7", "rechnen"));
    assert!(number.starts_with("Q?") && !number.contains("{"));
}

#[test]
fn letters_are_read_strictly_from_the_answer_line_and_loosely_from_the_end() {
    let picker = Picker::new(&spec()).expect("patterns compile");
    assert_eq!(picker.letter("Reasoning.\nAnswer: c"), (Some("C".into()), Some("C".into())));
    assert_eq!(picker.letter("Answer: (D)."), (Some("D".into()), Some("D".into())));
    assert_eq!(picker.letter("Answer: Because it is B"), (None, Some("B".into())));
    assert_eq!(picker.letter("no letter here"), (None, None));
}

#[test]
fn grading_checks_letters_and_numbers_against_the_gold_answer() {
    let picker = Picker::new(&spec()).expect("patterns compile");
    let letter = row("letter", &["1", "2"], "B", "logik");
    assert!(grade(&letter, "Answer: B", &picker).1);
    assert!(!grade(&letter, "Answer: A", &picker).2);
    let number = row("number", &[], "1,250", "rechnen");
    let (picked, strict, loose) = grade(&number, "So 1250 in total.\nAnswer: 1250", &picker);
    assert!(strict && loose);
    assert_eq!(picked.as_f64(), Some(1250.0));
    let (_, strict, loose) = grade(&number, "It makes 1250", &picker);
    assert!(!strict && loose);
}

#[test]
fn tail_keeps_the_last_characters_even_with_umlauts() {
    assert_eq!(tail("äöü abc", 3), "abc");
    assert_eq!(tail("ab", 5), "ab");
    assert_eq!(tail("ab", 0), "");
}

#[test]
fn summary_counts_errors_as_wrong_and_scores_each_group() {
    let rows = [
        row("letter", &["1", "2"], "A", "logik"),
        row("number", &[], "3", "rechnen"),
        row("number", &[], "4", "rechnen"),
    ];
    let refs: Vec<&Row> = rows.iter().collect();
    let reply = |thought: u64| Reply {
        thought_tokens: thought,
        answer_tokens: 10,
        seconds: 2.0,
        finish: "stop".into(),
        ..Reply::default()
    };
    let marks = [Some((true, true)), Some((false, true)), None];
    let sums = summary(&refs, &marks, &[reply(30), reply(10)], 9.0);
    let number = |key: &str| sums[key].as_f64().expect(key);
    assert!((number("score") - 200.0 / 3.0).abs() < 1e-9);
    assert!((number("score_strict") - 100.0 / 3.0).abs() < 1e-9);
    assert_eq!(number("score_logik"), 100.0);
    assert_eq!(number("score_rechnen"), 50.0);
    assert_eq!(number("thought_per_q"), 20.0);
    assert!((number("error_rate") - 1.0 / 3.0).abs() < 1e-9);
}

#[test]
fn the_shipped_think_settings_are_complete() {
    let spec = spec();
    assert!(spec.max_tokens > 0 && spec.retry_limit > 0 && spec.loose_tail_chars > 0);
    assert_eq!(spec.letters.chars().count(), 10);
    assert!(spec.game_errors.iter().any(|e| e == "game_running"));
    assert!(!spec.budget_field.is_empty() && !spec.warm_prompt.is_empty());
}
