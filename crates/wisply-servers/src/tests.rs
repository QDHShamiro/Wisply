use std::path::Path;

use super::fetch::{first_line, plan, whole_lines};
use super::*;

const PAPER: &str = "[12:00:01 INFO]: Starting minecraft server version 1.21.11
[12:00:09 INFO]: Done (8.123s)! For help, type \"help\"
[12:01:00 WARN]: Can't keep up! Is the server overloaded? Running 2034ms or 40 ticks behind
[12:02:00 WARN]: Can't keep up! Is the server overloaded? Running 5120ms or 102 ticks behind
[12:03:00 ERROR]: Could not pass event PlayerJoinEvent to AsyncKits v1.4.2
org.bukkit.event.EventException: null
\tat org.bukkit.plugin.java.JavaPluginLoader$1.execute(JavaPluginLoader.java:320)
Caused by: java.lang.NullPointerException: Cannot invoke \"String.length()\" because \"name\" is null
\tat dev.qdhshamiro.kits.Join.on(Join.java:41)
[12:03:01 INFO]: Steve joined the game
[12:04:00 ERROR]: Could not pass event PlayerJoinEvent to AsyncKits v1.4.2
org.bukkit.event.EventException: null
\tat org.bukkit.plugin.java.JavaPluginLoader$1.execute(JavaPluginLoader.java:999)
[12:05:00 WARN]: Steve moved too quickly! 12.5,3.0,-8.1
[12:05:30 WARN]: Alex_99 moved too quickly! 1.0,2.0,3.0
[12:06:00 INFO]: Player ERROR test joined
[12:07:00 ERROR]: The server has stopped responding! This is (probably) not a Paper bug.
";

#[test]
fn paper_log_groups_errors_and_finds_the_start() {
    let s = scan(PAPER);
    assert!(s.started);
    let titles: Vec<(&str, i64, &str)> = s.hits.iter().map(|h| (h.title.as_str(), h.count, h.level)).collect();
    assert_eq!(titles.len(), 4, "{titles:#?}");
    assert_eq!(titles[0].1, 2);
    assert_eq!(titles[0].2, "warn");
    assert!(
        titles[1].0.starts_with("Could not pass event PlayerJoinEvent to AsyncKits")
            && titles[1].0.ends_with("(EventException)")
    );
    assert_eq!((titles[1].1, titles[1].2), (2, "error"));
    assert_eq!((titles[2].0, titles[2].1), ("Steve moved too quickly! 12.5,3.0,-8.1", 2));
    assert_eq!(titles[3].2, "crash");
    assert!(s.hits[1].excerpt.contains("Join.java:41") && s.hits[1].excerpt.lines().count() == 5);
}

#[test]
fn other_log_styles_are_read() {
    let s = scan(
        "[12:00:00] [Server thread/ERROR]: Encountered an unexpected exception\n\
         java.lang.IllegalStateException: boom\n\tat net.minecraft.server.Main.run(Main.java:1)\n\
         12:00:00 [WARNING] Plugin Foo is outdated\n\
         2026-11-21T12:00:00Z ERROR db: connection refused 10.0.0.4:5432\n\
         [12:00:01] [Server thread/INFO]: all fine\n",
    );
    let got: Vec<(&str, &str)> = s.hits.iter().map(|h| (h.level, h.title.as_str())).collect();
    assert_eq!(
        got,
        [
            ("crash", "Encountered an unexpected exception (IllegalStateException)"),
            ("warn", "Plugin Foo is outdated"),
            ("error", "db: connection refused 10.0.0.4:5432")
        ]
    );
    assert!(!s.started);
}

#[test]
fn fingerprints_ignore_numbers_ids_and_names() {
    assert_eq!(fingerprint("took 2034ms", "", ""), fingerprint("took 99ms", "", ""));
    assert_eq!(
        fingerprint("uuid 123e4567-e89b-12d3-a456-426614174000 gone", "", ""),
        fingerprint("uuid 00000000-0000-0000-0000-000000000000 gone", "", "")
    );
    assert_ne!(fingerprint("a", "x.FooException", ""), fingerprint("a", "x.BarException", ""));
    assert_eq!(normalize("Steve moved too quickly! 1,2"), normalize("Alex moved too quickly! 3,4"));
}

#[test]
fn crash_reports_become_one_crash_hit() {
    let text = "---- Minecraft Crash Report ----\n// Oops.\n\nTime: 2026-11-21 12:00\nDescription: Exception in server tick loop\n\n\
                java.lang.OutOfMemoryError: Java heap space\n\tat java.base/java.util.Arrays.copyOf(Arrays.java:3512)\n\n\
                A detailed walkthrough of the error\n-- Head --\n";
    let h = crash("crash-2026-11-21_12.00.00-server.txt", text);
    assert_eq!(h.level, "crash");
    assert_eq!(h.title, "Crash: Exception in server tick loop (OutOfMemoryError)");
    assert!(h.excerpt.contains("Arrays.copyOf") && !h.excerpt.contains("walkthrough"));
}

#[test]
fn reading_plans_only_new_whole_lines() {
    assert_eq!(plan(1_000, 0, false, true), 0);
    assert_eq!(plan(10_000_000, 0, false, true), 10_000_000 - 256 * 1024);
    assert_eq!(plan(5_000, 4_000, false, false), 4_000);
    assert_eq!(plan(3_000, 4_000, true, false), 0);
    assert_eq!(plan(9_000_000, 10, false, false), 9_000_000 - 2 * 1024 * 1024);
    assert_eq!(whole_lines(b"tail\nline one\nhalf", true), (5, 14));
    assert_eq!(whole_lines(b"a\nb\n", false), (0, 4));
    assert_eq!(whole_lines(b"no newline", false), (0, 0));
    assert_eq!(first_line(b"[12:00] boot\nnext"), "[12:00] boot");
    assert_eq!(first_line(b"partial"), "");
}

#[test]
fn profiles_hide_secrets_and_paths_resolve() {
    let text = r#"{"profiles": {"b": {"host": "h", "port": "2022", "user": "u", "pass": "SECRET"},
                   "a": {"host": "h2", "user": "u2", "key": "keys/id"}, "c": {"host": "h3"}}}"#;
    let p = profiles_from(text, Path::new("/base")).unwrap();
    assert_eq!(p.iter().map(|x| x.name.as_str()).collect::<Vec<_>>(), ["a", "b", "c"]);
    assert_eq!((p[1].port, p[0].port), (2022, 22));
    assert!(p[0].usable() && p[1].usable() && !p[2].usable());
    let shown = format!("{p:?}");
    assert!(!shown.contains("SECRET") && shown.contains("password"), "{shown}");
    assert_eq!(crash_dir("logs/latest.log").as_deref(), Some("crash-reports"));
    assert_eq!(crash_dir("srv/logs/latest.log").as_deref(), Some("srv/crash-reports"));
    assert_eq!(crash_dir("app/out.log"), None);
}
