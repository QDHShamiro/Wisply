use super::*;

fn spec() -> ThinkAuto {
    ThinkAuto {
        quiet_patterns: vec!["(?i)\\bplaintiff\\b".to_string()],
        think_patterns: vec!["(?i)\\b(how many|solve)\\b".to_string()],
        option_patterns: Vec::new(),
        number_pattern: "\\b[0-9][0-9,.]*\\b".to_string(),
        min_numbers: 3,
        with_options: true,
        think_sees_options: true,
        goal_wrong_rate: 10.0,
    }
}

fn case(group: &str, think: bool, question: &str, options: &[&str]) -> Case {
    Case {
        group: group.to_string(),
        think,
        question: question.to_string(),
        options: options.iter().map(|o| o.to_string()).collect(),
    }
}

#[test]
fn only_auto_mode_asks_the_router() {
    let rules = spec();
    assert_eq!(choose(true, true, &rules, "hey"), Choice::Fixed(true));
    assert_eq!(choose(false, false, &rules, "Solve x."), Choice::Fixed(false));
    assert_eq!(choose(false, true, &rules, "hey"), Choice::Auto(false));
    assert_eq!(choose(false, true, &rules, "Solve x."), Choice::Auto(true));
    let mut broken = spec();
    broken.think_patterns = vec!["(".to_string()];
    assert!(matches!(choose(false, true, &broken, "hey"), Choice::Broken(_)));
}

#[test]
fn quiet_beats_think() {
    let router = Router::new(&spec()).unwrap();
    assert_eq!(router.decide::<&str>("How many dollars did the plaintiff get?", &[]), (false, Why::Quiet));
}

#[test]
fn pattern_and_numbers_turn_thinking_on() {
    let router = Router::new(&spec()).unwrap();
    assert_eq!(router.decide::<&str>("Solve for x.", &[]), (true, Why::Pattern));
    assert_eq!(router.decide::<&str>("Add 3 apples, 4 pears and 5 plums.", &[]), (true, Why::Numbers));
    assert_eq!(router.decide::<&str>("Who wrote Faust in 1808?", &[]), (false, Why::Plain));
}

#[test]
fn options_feed_patterns_but_not_numbers() {
    let router = Router::new(&spec()).unwrap();
    assert_eq!(router.decide("Pick one.", &["how many cats", "none"]), (true, Why::Pattern));
    assert_eq!(router.decide("Pick one.", &["1", "2", "3"]), (false, Why::Plain));
    let mut plain = spec();
    plain.with_options = false;
    let router = Router::new(&plain).unwrap();
    assert_eq!(router.decide("Pick one.", &["how many cats"]), (false, Why::Plain));
}

#[test]
fn tally_counts_both_error_sides_per_group() {
    let router = Router::new(&spec()).unwrap();
    let cases = vec![
        case("mathe", true, "Solve 2x = 4.", &[]),
        case("mathe", true, "Which group is cyclic?", &[]),
        case("recht", false, "Was the contract valid?", &[]),
        case("gesundheit", false, "Give 1, 2 and 3 doses: how many?", &[]),
    ];
    let found = tally(&router, &cases);
    assert_eq!(found.total, 4);
    assert_eq!(found.on, 2);
    assert_eq!(found.wrong_on, 1);
    assert_eq!(found.wrong_off, 1);
    assert_eq!(found.groups["mathe"], (2, 1));
    assert_eq!(found.groups["gesundheit"], (1, 1));
    assert_eq!(found.groups["recht"], (1, 0));
}

#[test]
fn bad_rules_are_refused() {
    let mut broken = spec();
    broken.think_patterns = vec!["(".to_string()];
    assert!(Router::new(&broken).is_err());
    let mut empty = spec();
    empty.think_patterns.clear();
    empty.min_numbers = 0;
    assert!(Router::new(&empty).is_err());
}

#[test]
fn rules_hash_changes_with_any_rule() {
    let base = rules_hash(&spec());
    let mut other = spec();
    other.min_numbers = 4;
    assert_ne!(base, rules_hash(&other));
    let mut other = spec();
    other.with_options = false;
    assert_ne!(base, rules_hash(&other));
    assert_eq!(base, rules_hash(&spec()));
}

#[test]
fn set_flag_is_required() {
    assert!(set_path(&[]).is_err());
    assert!(set_path(&["--set".to_string()]).is_err());
    assert!(set_path(&["--bogus".to_string()]).is_err());
    assert_eq!(set_path(&["--set".to_string(), "a.jsonl".to_string()]).unwrap(), PathBuf::from("a.jsonl"));
}

#[test]
fn option_words_only_count_when_allowed() {
    let mut rules = spec();
    rules.think_sees_options = false;
    rules.option_patterns = vec!["(?i)(?:^|[^a-z0-9])(ad lazarum|straw man)(?:$|[^a-z0-9])".to_string()];
    let router = Router::new(&rules).unwrap();
    assert_eq!(router.decide("Which statement is right?", &["How many there are", "None"]), (false, Why::Plain));
    assert_eq!(router.decide("Which fallacy is this?", &["Straw man", "None"]), (true, Why::Pattern));
    assert_eq!(router.decide::<&str>("\" _Ad lazarum_ \" is", &[]), (true, Why::Pattern));
    let wide = Router::new(&spec()).unwrap();
    assert_eq!(wide.decide("Which statement is right?", &["How many there are", "None"]), (true, Why::Pattern));
    assert_ne!(rules_hash(&rules), rules_hash(&spec()));
}
