use super::*;

fn dir(tag: &str) -> PathBuf {
    let dir = env::temp_dir().join(format!("wisply-plugins-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join(".claude-plugin")).unwrap();
    dir
}

fn market(dir: &Path, source: Value) {
    let body = json!({"name": "m", "plugins": [{"name": "other", "source": "./x"}, {"name": NAME, "source": source}]});
    fs::write(dir.join(".claude-plugin").join("marketplace.json"), body.to_string()).unwrap();
}

#[test]
fn installed_reads_the_user_install_of_wisply_from_every_marketplace() {
    let plugins = json!({"version": 2, "plugins": {
        "wisply@wisply": [{"scope": "project", "version": "1.0.0"}, {"scope": "user", "version": "3.0.0"}],
        "wisply@mine": [{"scope": "project", "version": "2.0.0"}],
        "other@wisply": [{"scope": "user", "version": "9.9.9"}],
        "wisply@dev": [{"scope": "user"}]
    }});
    let mut found = installed(&plugins);
    found.sort_by(|a, b| a.market.cmp(&b.market));
    assert_eq!(
        found,
        vec![
            Install { market: "dev".into(), version: String::new() },
            Install { market: "wisply".into(), version: "3.0.0".into() },
        ]
    );
    assert!(installed(&json!({})).is_empty() && installed(&Value::Null).is_empty());
}

#[test]
fn offered_reads_the_tag_of_a_pinned_source_or_the_manifest_of_a_folder() {
    let pinned = dir("pinned");
    market(&pinned, json!({"source": "github", "repo": "QDHShamiro/Wisply", "ref": "v3.6.1"}));
    assert_eq!(offered(&pinned).as_deref(), Some("3.6.1"));
    market(&pinned, json!({"source": "url", "url": "https://github.com/QDHShamiro/Wisply.git", "ref": "v3.7.0"}));
    assert_eq!(offered(&pinned).as_deref(), Some("3.7.0"));
    market(&pinned, json!({"source": "github", "repo": "QDHShamiro/Wisply"}));
    assert_eq!(offered(&pinned), None);
    let folder = dir("folder");
    market(&folder, json!("./"));
    assert_eq!(offered(&folder), None);
    fs::write(folder.join(".claude-plugin").join("plugin.json"), r#"{"name":"wisply","version":"3.0.0"}"#).unwrap();
    assert_eq!(offered(&folder).as_deref(), Some("3.0.0"));
    assert_eq!(offered(&folder.join("missing")), None);
    for d in [pinned, folder] {
        fs::remove_dir_all(d).unwrap();
    }
}

#[test]
fn decide_updates_only_to_exactly_this_version() {
    assert_eq!(decide(VERSION, Some("0.0.1"), "m", None), Step::Current);
    assert_eq!(decide("999.0.0", None, "m", None), Step::Current);
    assert_eq!(decide("0.0.1", Some(VERSION), "m", None), Step::Update);
    assert_eq!(decide("", Some(&format!("v{VERSION}")), "m", None), Step::Update);
    let Step::Skip(why) = decide("0.0.1", Some("0.0.2"), "wisply", Some("E:\\Wisply")) else { panic!("no skip") };
    assert!(why.starts_with("its marketplace wisply is the folder E:\\Wisply and offers only 0.0.2; "), "{why}");
    assert!(
        why.ends_with("claude plugin marketplace add QDHShamiro/Wisply, claude plugin install wisply@wisply"),
        "{why}"
    );
    let Step::Skip(why) = decide("0.0.1", None, "wisply", None) else { panic!("no skip") };
    assert_eq!(why, "its marketplace wisply offers no fixed version");
}

#[test]
fn codex_markets_are_enabled_wisply_plugins_from_git_marketplaces() {
    let config = r#"
model = "x"
[plugins."wisply@git"]
enabled = true
[plugins."wisply@off"]
enabled = false
[plugins."wisply@local"]
enabled = true
[plugins."wisply@gone"]
[plugins."other@git"]
enabled = true
[marketplaces.git]
source_type = "git"
source = "https://example.invalid/w.git"
[marketplaces.off]
source_type = "git"
[marketplaces.local]
source_type = "local"
"#;
    assert_eq!(codex_markets(config), vec!["git".to_string()]);
    assert!(codex_markets("not = [toml").is_empty() && codex_markets("").is_empty());
}

#[test]
fn three_failed_tries_stop_the_automatic_runs_for_this_version() {
    let home = dir("tries");
    for _ in 0..TRIES - 1 {
        failed(&home, "claude", "3.0.0", "boom");
        assert!(!tried_out(&home, "claude"));
    }
    failed(&home, "claude", "3.0.0", "boom");
    assert!(tried_out(&home, "claude") && !tried_out(&home, "codex"));
    let mut s = state::load(&home);
    s["plugins"]["claude"]["to"] = json!("0.0.1");
    state::save(&home, &s);
    assert!(!tried_out(&home, "claude"));
    record(&home, "claude", json!({"ok": true}));
    assert_eq!(state::load(&home)["plugins"]["claude"], json!({"to": VERSION, "ok": true}));
    fs::remove_dir_all(home).unwrap();
}

#[test]
fn the_marketplace_pins_the_plugin_to_this_release() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..");
    let market = read_json(&repo.join(".claude-plugin").join("marketplace.json"));
    let source = market["plugins"].as_array().and_then(|p| p.iter().find(|p| p["name"] == json!(NAME))).unwrap();
    assert_eq!(source["source"]["source"], json!("url"), "https needs no SSH key, a github source does on update");
    assert_eq!(source["source"]["url"], json!("https://github.com/QDHShamiro/Wisply.git"));
    assert_eq!(
        source["source"]["ref"],
        json!(format!("v{VERSION}")),
        "bump the ref in .claude-plugin/marketplace.json"
    );
    assert_eq!(offered(&repo).as_deref(), Some(VERSION));
    let manifest = read_json(&repo.join(".claude-plugin").join("plugin.json"));
    assert_eq!(manifest["version"], json!(VERSION));
}

#[test]
fn short_keeps_the_line_that_names_the_cause() {
    let clone = "Checking for updates…\n✘ Failed to update plugin \"wisply@m\": Failed to clone repository: Cloning into 'C:\\x'...\n\
                 git@github.com: Permission denied (publickey).\nfatal: Could not read from remote repository.\n\n\
                 and the repository exists.\n";
    assert_eq!(short(clone), "git@github.com: Permission denied (publickey).");
    assert_eq!(short("✘ Failed: Command 'git' not found\n"), "Failed: Command 'git' not found");
    assert_eq!(short("one\ntwo\n\n"), "two");
    assert_eq!(short(""), "no output");
}
