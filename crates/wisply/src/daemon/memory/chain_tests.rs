use super::*;

fn cfg(pairs: Value) -> Map<String, Value> {
    let mut c = Map::new();
    c.insert("gemini".into(), json!(true));
    c.insert("gemini_model".into(), json!("gemini-2.5-flash"));
    c.extend(pairs.as_object().cloned().unwrap_or_default());
    c
}

#[test]
fn the_chain_is_read_from_names_or_objects() {
    let c = cfg(json!({"memory_chain": "gemini, groq > local"}));
    assert_eq!(entries(&c), [json!("gemini"), json!("groq"), json!("local")]);
    let mine = r#"[{"name": "Mine", "url": "http://127.0.0.1:1/v1/chat/completions", "model": "m"}]"#;
    let c = cfg(json!({ "memory_chain": mine }));
    let p = build(&entries(&c)[0], &c).link.ok().unwrap();
    assert!(p.local && p.model == "m" && p.chars == CHUNK_CHARS && p.key.is_none() && p.link == "mine");
    let cloud = json!({"name": "mine", "url": "http://127.0.0.1:1/x", "model": "m", "cloud": true, "chars": 5000});
    let p = build(&cloud, &c).link.ok().unwrap();
    assert!(!p.local && p.chars == 5000);
    assert_eq!(entries(&cfg(json!({}))), [json!("gemini"), json!("local")]);
    let none = build(&json!("local"), &cfg(json!({}))).link.err().unwrap();
    assert_eq!(none, "no memory_api + memory_api_model");
    let off = build(&json!("gemini"), &cfg(json!({"gemini": false}))).link.err().unwrap();
    assert_eq!(off, "gemini off");
    let keyless = json!({"name": "groq", "key": "WISPLY_TEST_NO_SUCH_KEY"});
    assert_eq!(build(&keyless, &c).link.err().unwrap(), "no WISPLY_TEST_NO_SUCH_KEY");
    let api = json!({"memory_api": "http://127.0.0.1:9930/v1", "memory_api_model": "brain"});
    let p = build(&json!("local"), &cfg(api)).link.ok().unwrap();
    assert!(p.local && p.url == "http://127.0.0.1:9930/v1/chat/completions" && p.name() == "brain at 127.0.0.1:9930");
    let far = build(&json!("local"), &cfg(json!({"memory_api": "https://brain.example.com", "memory_api_model": "b"})));
    assert!(!far.link.ok().unwrap().local);
}

#[test]
fn private_words_match_whole_words_only() {
    let words = vec![tokens("taxes"), tokens("note-app")];
    assert!(private(&words, &["C:/x/.claude/projects/E--Taxes-Year-2030/a.jsonl"]));
    assert!(private(&words, &["opened the Note App installer"]));
    assert!(!private(&words, &["tax text", "note and app"]));
    assert!(!private(&[], &["taxes"]));
    let stems = vec![tokens("Konto"), tokens("app"), tokens("Secret Shop")];
    assert!(private(&stems, &["der Kontoauszug"]) && !private(&stems, &["E--Konten-2030"]));
    assert!(private(&stems, &["the secret shops list"]) && !private(&stems, &["shop secrets"]));
    assert!(!private(&stems, &["apple and application"]) && private(&stems, &["open the app"]));
}

#[test]
fn busy_links_pause_by_their_hint_or_longer_each_time() {
    let m = |status: u16, retry: Option<f64>, why: &str| Miss { status, retry, why: why.into() };
    assert_eq!(pause_for(&m(429, Some(7.5), "slow down"), 0), Some(30.0));
    assert_eq!(pause_for(&m(429, Some(120.0), "slow down"), 3), Some(120.0));
    assert_eq!(pause_for(&m(429, None, "busy"), 0), Some(900.0));
    assert_eq!(pause_for(&m(429, None, "busy"), 2), Some(3600.0));
    assert_eq!(pause_for(&m(429, None, "busy"), 9), Some(21_600.0));
    assert_eq!(pause_for(&m(429, Some(40.0), "GenerateRequestsPerDayPerProjectPerModel"), 0), Some(3600.0));
    assert_eq!(pause_for(&m(503, None, ""), 1), Some(600.0));
    assert_eq!(pause_for(&m(0, None, ""), 5), Some(3600.0));
    assert_eq!(pause_for(&m(401, None, ""), 0), Some(21_600.0));
    assert_eq!(pause_for(&m(413, None, ""), 0), None);
    assert_eq!(pause_for(&m(400, None, "This model's maximum context length is 8192"), 0), None);
    assert_eq!(pause_for(&m(200, None, ""), 0), None);
}

#[test]
fn the_book_counts_per_day_and_keeps_pauses() {
    let home = std::env::temp_dir().join(format!("lt-chain-{}", std::process::id()));
    let _ = fs::remove_dir_all(&home);
    let p = Provider { link: "groq".into(), day: 2, ..Provider::default() };
    assert!(blocked(&home, &p).is_none());
    ok(&home, &p);
    ok(&home, &p);
    assert_eq!(used_today(&home, &p), 2);
    assert!(blocked(&home, &p).is_some_and(|(_, why)| why == "2 of 2 asks used today"));
    let q = Provider { link: "gemini".into(), ..Provider::default() };
    let until = miss(&home, &q, &Miss { status: 429, retry: Some(60.0), why: "Gemini answered 429".into() });
    assert!(until.is_some_and(|u| u > now() + 50.0));
    assert!(blocked(&home, &q).is_some_and(|(_, why)| why == "Gemini answered 429"));
    assert!(miss(&home, &q, &Miss { status: 413, retry: None, why: String::new() }).is_none());
    assert!(matches!(Links::new(&home, vec![p, q.clone()]).pick(), Err(Halt::Wait(_, Some(_)))));
    let mut free = Links::new(&home, vec![q, Provider { link: "spare".into(), ..Provider::default() }]);
    assert_eq!(free.pick().ok(), Some(1));
    free.refuse(1, "spare sent no JSON".into());
    assert!(matches!(free.pick(), Err(Halt::Wait(_, Some(_)))));
    assert!(matches!(Links::new(&home, Vec::new()).pick(), Err(Halt::Fail(_))));
    let mut book = load(&home, now() + DAY_S);
    assert_eq!(f(mark(&mut book, "groq"), "used"), 0.0);
    let _ = fs::remove_dir_all(&home);
}
