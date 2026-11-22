use super::*;

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("wisply-script-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn scripts_hash_by_their_lines_and_fit_only_their_own_project() {
    assert_eq!(hash("a\n  b  \n\n"), hash("a\nb"));
    assert_ne!(hash("a\nb"), hash("a\nc"));
    assert_eq!(hash("x").len(), 16);
    assert!(same_place("E:\\Wisply\\", "e:/wisply"));
    assert!(!same_place("", ""));
    let here = scratch("fit");
    fs::create_dir_all(here.join(".git")).unwrap();
    fs::create_dir_all(here.join("src")).unwrap();
    let r = json!({"script": {"project": here.display().to_string()}});
    let inside = here.join("src").display().to_string();
    assert!(fits(2, &r, &inside));
    assert!(!fits(1, &r, &inside));
    let away = scratch("elsewhere");
    assert!(!fits(2, &r, &away.display().to_string()));
    let _ = fs::remove_dir_all(&away);
    assert_eq!(guess_name("await $`git status --short`"), "git-status-short");
    assert_eq!(guess_name("1 + 1"), "script");
    let _ = fs::remove_dir_all(&here);
}

#[test]
fn unused_scripts_go_after_30_days_and_often_used_ones_after_90() {
    let home = scratch("prune");
    let day = |n: f64| stamp(now() - n * DAY_S);
    let make = |name: &str, runs: u64, last: String| {
        json!({"name": name, "steps": [], "script": {"code": "1"}, "created": "2026-01-01 00:00",
            "stats": {"runs": runs, "last": last}})
    };
    for r in [
        make("old-few", 1, day(31.0)),
        make("new-few", 1, day(29.0)),
        make("old-many", 5, day(60.0)),
        make("gone-many", 5, day(91.0)),
    ] {
        save(&home, &r).unwrap();
    }
    save(&home, &json!({"name": "steps-only", "steps": [], "created": "2020-01-01 00:00", "stats": {}})).unwrap();
    prune(&home);
    let mut left: Vec<String> = load_all(&home).iter().map(|r| text(&r["name"])).collect();
    left.sort();
    assert_eq!(left, ["new-few", "old-many", "steps-only"]);
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn values_from_the_request_become_args_and_the_task_words_name_it() {
    let typed: HashSet<String> = values_in("Monat 2026-10 in data/2026-10, PR 107, sha 27df922").into_iter().collect();
    assert_eq!(typed, HashSet::from(["2026-10".to_string(), "107".to_string()]));
    let code = "const f = await glob(\"data/2026-10/*.csv\");\nawait $`gh pr view 107`;\nconst n = '2026-10';\nreturn \"a\\\"2026-10\" + `${x}2026-10` + '300'";
    let (lifted, params) = lift(code, &typed);
    assert_eq!(params, json!({"month": "2026-10", "value": "107"}).as_object().cloned().unwrap());
    assert!(lifted.contains("glob(\"data/\" + args.month + \"/*.csv\")"), "{lifted}");
    assert!(lifted.contains("$`gh pr view ${args.value}`"), "{lifted}");
    assert!(lifted.contains("const n = args.month;"), "{lifted}");
    assert!(lifted.contains("\"a\\\"2026-10\" + `${x}2026-10` + '300'"), "{lifted}");
    let mut taken = Map::new();
    assert_eq!(key_for("2026-10-05", &mut taken), "date");
    assert_eq!(key_for("2026-11-05", &mut taken), "date2");
    assert_eq!(key_for("2026-10-05", &mut taken), "date");
    assert_eq!(key_for("2027", &mut taken), "year");
    let prompt: Vec<String> =
        ["werkzeug", "hilfe", "rechne", "spalte", "betrag", "gesamtsumme"].map(String::from).into();
    let tags = task_words(prompt, "const i = header.indexOf(\"betrag\"); out.push('Gesamt'); 'Spalte'");
    assert_eq!(tags, ["spalte", "betrag", "gesamtsumme", "werkzeug", "hilfe", "rechne"]);
    assert_eq!(slug(&tags.join(" "), NAME_WORDS), "spalte-betrag-gesamtsumme");
}

#[test]
fn an_offer_swaps_in_the_values_of_the_new_request_by_their_kind() {
    let r = json!({"params": {"month": "2026-10", "value": "107", "word": "x", "day": "2026-10-05"}});
    let typed: Vec<String> = ["summe", "2026-11", "112", "27df922"].map(String::from).into();
    let want = BTreeMap::from([("month".to_string(), "2026-11".to_string()), ("value".to_string(), "112".to_string())]);
    assert_eq!(swap(&r, &typed), want);
    assert_eq!((kind_of("2026"), kind_of("2026-10-05"), kind_of("27df922")), (Some("year"), Some("date"), None));
}

#[test]
fn show_numbers_the_code_and_caps_it() {
    let code: Vec<String> = (1..=85).map(|i| format!("line {i}")).collect();
    let shown = show(&json!({"script": {"code": code.join("\n")}}));
    assert_eq!(shown.len(), 81);
    assert_eq!(shown[0], "  1 line 1");
    assert_eq!(shown[80], "... 5 more lines");
}
