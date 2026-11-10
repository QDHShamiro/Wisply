use super::*;
use crate::oracle as old;
use proptest::prelude::*;

const PIECES: &[&str] = &[
    "error",
    "Error",
    "ERROR",
    "errors",
    "0",
    "00",
    "no",
    "No",
    "zero",
    "without",
    "warning",
    "warnings",
    "failed",
    "failures",
    "fail",
    "fail-fast",
    "--no-fail-fast",
    "-fail-fast",
    "no-fail",
    "fail_fast",
    "issues",
    "problem",
    "vulnerabilities",
    "none",
    "null",
    "false",
    "[]",
    "{}",
    ":",
    "=",
    " ",
    "  ",
    "\t",
    "-",
    "_",
    "err_",
    "_error",
    "TimeoutError",
    "IOError",
    "xError",
    "Exceptions",
    "NullPointerException",
    "can't",
    "can",
    "'t",
    "not found",
    "not",
    "found",
    "no such",
    "timed out",
    "timed",
    "out",
    "denied",
    "x",
    "foo",
    "12",
    "0x1F",
    "deadbeef",
    "deadbeef12",
    "abcdef1",
    "[",
    "]",
    "[12:00:01 INFO]:",
    "2026-10-30T04:00:00.819Z",
    "INFO",
    "\u{2800}",
    "███",
    "█████",
    "━━━━━",
    "[=====>   ]",
    "[##]",
    "<=====>",
    "<---->",
    "Progress (3):",
    "Downloading from x:",
    "Download",
    "https://x",
    "http://x",
    " from ",
    "npm http",
    "npm ",
    "timing",
    "at ",
    "File \"",
    "...",
    "more",
    "common frames",
    "^",
    "3:",
    "/a/b.go:12",
    "java.lang.",
    "org.bukkit.x",
    "std::",
    "tokio::",
    "mystd::",
    "node:internal",
    "test result:",
    "Tests run:",
    "5 passed",
    "5 tests",
    "ran 3 tests",
    "Finished",
    "Done (",
    "done in",
    "=====",
    "added 3 packages",
    "found 2 vulnerabilities",
    "1 high severity",
    "2 actionable tasks",
    "Total time:",
    "BUILD FAILURE",
    "\"error\"",
    "\"lastWarning\"",
    ": ",
    "\"\"",
    "\u{e9}",
    "\u{2014}",
    "\u{301}",
];

fn line() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(PIECES.to_vec()), 0..9).prop_map(|v| v.concat())
}

#[test]
fn known_lines() {
    assert_eq!(strip_ansi("\x1b[1;31mred\x1b[0m \x1b]0;t\x07x\x1b(B"), "red x");
    assert!(important("java.lang.NullPointerException: x"));
    assert!(important("Can't keep up"));
    assert!(important("upstream TIMED OUT"));
    assert!(!important("terrific errand"));
    assert!(trace("\tat a.b.C(C.java:1)"));
    assert!(trace("  File \"x.py\", line 1"));
    assert_eq!(body("2026-10-30T04:00:00.819Z INFO [main] [db]: query"), "query");
    assert_eq!(body("[12:00:01 INFO]: joined"), "joined");
    assert_eq!(numbers("id 0x1F deadbeef12 v1.2"), "id # # v#.#");
    assert_eq!(quiet(r#"{"error": null, "lastWarning": "", "x": 1}"#), r#"{, , "x": 1}"#);
}

#[test]
fn error_words_need_to_mean_an_error() {
    for line in [
        "error[E0308]: mismatched types",
        "TimeoutError: upstream timed out",
        "IOError raised",
        "npm ERR! code ENOENT",
        "E_USER_ERROR in x.php",
        "ERR_CONNECTION_REFUSED",
        "[21:00:00 WARN]: Can't keep up!",
        "test result: FAILED. 8 passed; 2 failed; 0 ignored",
        "found 0 vulnerabilities, 3 errors",
        "Tests: 1 failed, 0 skipped",
    ] {
        assert!(hot(line), "{line}");
    }
    for line in [
        "   Compiling thiserror v2.0.12",
        "fn onError(cb) {",
        "test result: ok. 214 passed; 0 failed; 0 ignored",
        "Compiled with 0 errors and 0 warnings",
        "no errors found",
        "warnings: 0",
        "cargo test --no-fail-fast",
        "errors: none",
    ] {
        assert!(!hot(line), "{line}");
    }
}

#[test]
fn progress_and_download_lines_are_noise() {
    for line in [
        "    Building [=====>          ] 147/312: serde",
        "⠙ reify:lodash: timing reifyNode Completed in 12ms",
        "Progress (1): 2.1 kB",
        "Downloading from central: https://repo.maven.apache.org/x.pom",
        "Downloaded from central: https://repo.maven.apache.org/x.pom (2 kB at 9 kB/s)",
        "Download https://repo.maven.apache.org/maven2/a.pom",
        "<=====--------> 42% CONFIGURING [3s]",
        "   ━━━━━━━━━━━━━━━━━━━━ 1.2/1.2 MB 3.4 MB/s eta 0:00:00",
        "npm http fetch GET 200 https://registry.npmjs.org/a 12ms",
    ] {
        assert!(kind(line).noise, "{line}");
    }
    for line in [
        "tests/test_a.py ....F.. [ 16%]",
        "Downloading failed: https://x 404 Not Found",
        "Preparing spawn area: 42%",
        "- [x] done",
    ] {
        assert!(!kind(line).noise, "{line}");
    }
}

#[test]
fn summaries_and_framework_frames_are_known() {
    for line in [
        "test result: ok. 3 passed; 0 failed",
        "BUILD FAILED in 14s",
        "BUILD SUCCESS",
        "Total time:  6.412 s",
        "    Finished `test` profile [unoptimized] target(s) in 4.21s",
        "====== 2 failed, 386 passed in 12.84s ======",
        "added 1287 packages, and audited 1288 packages in 41s",
        "Tests run: 12, Failures: 0, Errors: 0, Skipped: 0",
        "Done (12.481s)! For help, type \"help\"",
    ] {
        assert!(summary(line), "{line}");
    }
    assert!(!summary("step 3 done"));
    assert!(runtime("\tat java.base/java.lang.Thread.run(Thread.java:1583)"));
    assert!(runtime("\tat org.bukkit.plugin.java.JavaPluginLoader.x(J.java:1) ~[paper.jar:?]"));
    assert!(runtime("    at Module._compile (node:internal/modules/cjs/loader:1105:14)"));
    assert!(runtime("   3: std::panicking::begin_panic"));
    assert!(!runtime("\tat dev.qdhshamiro.shop.ShopGui.open(ShopGui.java:57)"));
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(3000))]

    #[test]
    fn same_as_the_regexes(raw in line(), esc in "[\\x1b\\[\\];0-9 m(B\\x07a-z\\\\]{0,14}") {
        let (hot, noisy) = old::kind(&raw);
        let k = kind(&raw);
        prop_assert_eq!((k.hot, k.noise), (hot, noisy), "{:?}", raw);
        let pre = old::PREFIX.replace(&raw, "");
        prop_assert_eq!(body(&raw), pre.as_ref(), "{:?}", raw);
        prop_assert_eq!(trace(&raw), old::TRACE.is_match(&raw), "{:?}", raw);
        prop_assert_eq!(runtime(&raw), old::RUNTIME.is_match(&raw), "{:?}", raw);
        prop_assert_eq!(summary(&raw), old::SUMMARY.is_match(&raw), "{:?}", raw);
        let nums = old::NUMS.replace_all(&raw, "#");
        prop_assert_eq!(numbers(&raw), nums.as_ref(), "{:?}", raw);
        let quieted = old::QUIET.replace_all(&raw, "");
        let q = quiet(&raw);
        prop_assert_eq!(q.as_ref(), quieted.as_ref(), "{:?}", raw);
        let mixed = format!("{raw}{esc}{raw}");
        let plain = old::ANSI.replace_all(&mixed, "");
        let sa = strip_ansi(&mixed);
        prop_assert_eq!(sa.as_ref(), plain.as_ref(), "{:?}", mixed);
        prop_assert_eq!(important(&raw), old::IMPORTANT.is_match(&raw), "{:?}", raw);
        prop_assert_eq!(important(&mixed), important_slow(&mixed), "{:?}", mixed);
    }
}
