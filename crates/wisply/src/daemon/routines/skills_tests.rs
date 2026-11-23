use super::*;

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("wisply-skill-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn parts_of(code: &str) -> Parts {
    Parts {
        code: code.into(),
        check: "return args.result.includes('sent')".into(),
        about: "send a chat message".into(),
        guide: "exact chat name".into(),
        params: Map::new(),
        before_ms: 120_000,
        tokens: 6000,
    }
}

#[test]
fn params_come_from_the_code_and_skip_dry_and_result() {
    let code = "if (args.dry) return; await wisply`chat ${args.to}`; type(args[\"text\"]); log(args.result, args.to)";
    let old = json!({"params": {"text": "hi"}});
    let (params, unset) = params_of(code, &old, &[("to".into(), "Mum".into())]);
    assert_eq!(Value::Object(params), json!({"to": "Mum", "text": "hi"}));
    assert!(unset.is_empty());
    let (params, unset) = params_of(code, &Value::Null, &[]);
    assert!(params.len() == 2 && params.contains_key("to") && params.contains_key("text"));
    assert_eq!(unset, ["to", "text"]);
}

#[test]
fn a_skill_practices_until_ready_then_is_learned_only_after_the_ok() {
    let home = scratch("learn");
    let r = build("send-message", &Value::Null, parts_of("await wisply`ping`; return 'sent ' + args.to"));
    assert!(is_skill(&r) && state(&r) == "practice" && r["version"] == 1 && r["source"] == "skill");
    assert!(r["tags"].as_array().unwrap().iter().any(|t| t == "message"), "{r}");
    save(&home, &r).unwrap();
    assert!(approve(&home, "send-message").is_err());
    let dry = json!({"ok": true, "ms": 40, "checked": true, "rounds": 2, "dry": true});
    assert_eq!(ran(&home, store::load(&home, "send-message").unwrap(), &dry), "skill send-message: 2/3 checks passed");
    let broken = json!({"ok": false, "ms": 40, "error": "err: chat Mum not found", "dry": true});
    let out = ran(&home, store::load(&home, "send-message").unwrap(), &broken);
    assert!(out.starts_with("skill send-message failed: err: chat Mum not found -> try: fix it"), "{out}");
    let kept = store::load(&home, "send-message").unwrap();
    assert_eq!(kept["learn"]["hint"], "err: chat Mum not found");
    assert_eq!(kept["stats"]["runs"], 0);
    assert!(show_lines(&kept).iter().any(|l| l == "last failure (hint for the next AI): err: chat Mum not found"));
    let one = json!({"ok": true, "ms": 40, "checked": true, "rounds": 1, "dry": true});
    let out = ran(&home, kept, &one);
    assert_eq!(out, "skill send-message passed 3 checks -> ask the user, then: wisply learn ok send-message");
    let ready = store::load(&home, "send-message").unwrap();
    assert!(state(&ready) == "ready" && ready["learn"]["hint"].is_null());
    let out = approve(&home, "send-message").unwrap();
    assert_eq!(out, "learned skill send-message -> from now on one call: wisply run send-message");
    let learned = store::load(&home, "send-message").unwrap();
    assert!(fits(&learned, 2) && !fits(&learned, 1));
    let real = json!({"ok": true, "ms": 5000, "checked": true, "rounds": 1, "dry": false});
    assert_eq!(ran(&home, learned, &real), "");
    let after = store::load(&home, "send-message").unwrap();
    assert_eq!((after["stats"]["runs"].clone(), after["stats"]["ok"].clone()), (json!(1), json!(1)));
    let again = build("send-message", &after, parts_of("return 'sent v2'"));
    assert!(again["version"] == 2 && state(&again) == "practice" && again["learn"]["practice"]["ok"] == 0);
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn repeated_routines_get_at_most_two_learn_offers_and_skills_none() {
    let mut r = json!({"name": "pr-status", "stats": {"seen": 1, "runs": 1}});
    assert_eq!(offer_note(&mut r), None);
    r["stats"]["runs"] = json!(2);
    let note = offer_note(&mut r).unwrap();
    assert!(note.starts_with("-> pr-status keeps coming back: if it is a skill"), "{note}");
    assert!(offer_note(&mut r).is_some());
    assert_eq!(offer_note(&mut r), None);
    assert_eq!(r["stats"]["learn_offers"], 2);
    let mut s = build("pr-status", &Value::Null, parts_of("return 1"));
    s["stats"]["runs"] = json!(9);
    assert_eq!(offer_note(&mut s), None);
}

#[test]
fn a_skill_that_fails_often_or_is_not_faster_is_weak() {
    let mut r = build("find-file", &Value::Null, parts_of("return 1"));
    r["stats"] = json!({"runs": 4, "ok": 1, "fail": 3, "ms": 100});
    assert!(!weak(&r));
    r["stats"] = json!({"runs": 5, "ok": 3, "fail": 2, "ms": 100});
    assert!(weak(&r));
    r["stats"] = json!({"runs": 5, "ok": 5, "fail": 0, "ms": 100_000});
    assert!(weak(&r));
    r["stats"] = json!({"runs": 5, "ok": 5, "fail": 0, "ms": 2000});
    assert!(!weak(&r));
    assert!(row(&r).starts_with("find-file | practice | 5/5 ok | "), "{}", row(&r));
}

#[test]
fn the_offer_fills_in_the_values_of_the_request() {
    let mut r = build("csv-rows", &Value::Null, parts_of("return args.month + args.base"));
    r["params"] = json!({"month": "2026-10", "base": "data"});
    let typed: Vec<String> = ["count", "rows", "2026-11"].map(String::from).to_vec();
    let line = offer_line(&r, &typed);
    assert!(line.starts_with("Wisply skill fits this request. Run it instead of doing the steps by hand"), "{line}");
    assert!(line.ends_with(": wisply run csv-rows month=2026-11 base=<…> (exact chat name)"), "{line}");
}

#[test]
fn a_value_with_a_default_in_the_code_is_optional() {
    let code =
        "const m = String(args.month || ''); const r = args.root || \"data\"; const n = args['max'] ?? 5; args.to";
    assert_eq!(defaulted(code), HashSet::from(["root".to_string(), "max".to_string()]));
    let (params, unset) = params_of(code, &Value::Null, &[("month".into(), "2026-10".into())]);
    assert_eq!(params.len(), 4);
    assert_eq!(unset, ["to"]);
    let mut r = build("csv-rows", &Value::Null, parts_of(code));
    r["params"] = json!({"month": "2026-10", "root": "", "max": "", "to": ""});
    let typed: Vec<String> = ["count", "2026-11"].map(String::from).to_vec();
    assert!(offer_line(&r, &typed).ends_with("wisply run csv-rows month=2026-11 to=<…> (exact chat name)"));
    assert_eq!(usage_of(&r), "wisply run csv-rows month=<…> [root=<…>] [max=<…>] to=<…>");
    let mut filled = BTreeMap::new();
    fill_missing(&r, &mut filled);
    assert_eq!(filled.keys().collect::<Vec<_>>(), ["month", "to"]);
}

#[test]
fn an_about_with_an_equals_sign_stays_the_about() {
    assert_eq!(pair("month=2026-10"), Some(("month".into(), "2026-10".into())));
    assert_eq!(pair("counts the rows, call: month=YYYY-MM"), None);
    assert_eq!(pair("a=b c"), Some(("a".into(), "b c".into())));
}

#[test]
fn other_languages_are_refused_with_a_wrapper_to_copy() {
    let dir = scratch("lang");
    fs::write(dir.join("count.py"), "print(1)").unwrap();
    fs::write(dir.join("run.js"), "return 1").unwrap();
    let err = read_file(&dir, "count.py", "code file").unwrap_err().text();
    assert!(err.contains("count.py is .py, a skill is code mode JavaScript"), "{err}");
    assert!(err.contains("return await $`python ") && err.contains("count.py ${args.<name>}`"), "{err}");
    assert_eq!(read_file(&dir, "run.js", "code file").unwrap(), "return 1");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn the_brief_names_skills_that_already_do_the_task() {
    let home = scratch("brief");
    let mut r = build("send-message", &Value::Null, parts_of("return args.to"));
    r["learn"]["state"] = json!("learned");
    r["params"] = json!({"to": ""});
    save(&home, &r).unwrap();
    let out = brief(&home, "send a chat message to Mum");
    assert!(out.contains("\ntask: send a chat message to Mum"), "{out}");
    assert!(out.ends_with("there already, maybe it does this: wisply run send-message to=<…>"), "{out}");
    assert!(!brief(&home, "resize a picture").contains("there already"));
    let _ = fs::remove_dir_all(&home);
}
