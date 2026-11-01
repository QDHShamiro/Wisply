use regex::{Regex, RegexSet};
use std::sync::LazyLock;

fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("squeeze pattern")
}

const IMPORTANT_P: &str = concat!(
    r"(?i:\b(?:errors?|exceptions?|err|fail|failed|failing|failure|failures|fatal|traceback|panic|panicked|",
    r"warn|warning|warnings|denied|refused|cannot|can't|unable|not found|no such|missing|undefined|",
    r"segfault|abort|aborted|timeout|timed out|killed|assert|assertion|severe|critical)\b|_(?:error|exception)s?\b|\berr_)",
    r"|[A-Z][A-Za-z0-9]*(?:Error|Exception)s?\b",
);

const NEG_P: &str = concat!(
    r"(?i)\b(?:0|no|zero|without)\s+(?:errors?|warnings?|failures?|failed|failing|issues?|problems?|vulnerabilit(?:y|ies))\b",
    r"|\b(?:errors?|warnings?|failures?|failed|failing)\s*[:=]?\s*0\b",
    r"|\b(?:errors?|warnings?)\s*[:=]\s*(?:none|null|false|\[\]|\{\})",
    r"|--?(?:no-)?fail-fast\b|\bfail_fast\b|\bno-fail\b",
);

const NOISE_P: &str = concat!(
    r"^\s*[\x{2800}-\x{28ff}]|\[[ .\-]*[=#]{2,}[=#>\-. ]*\]|[█▉▊▋▌▍▎▏▓▒░━]{5,}|<[=\-]{5,}>",
    r"|^\s*Progress \(\d+\):|^\s*Download(?:ing|ed)?(?: from \S+:)? https?://|^npm (?:http|timing|sill|verb)\b",
);

const SUMMARY_P: &str = concat!(
    r"(?i)^\s*(?:test result:|tests? run:|tests?:\s+\d|test suites?:|ran \d+ tests?|\d+ (?:passed|failed|tests?)\b|",
    r"=+ .*\b(?:passed|failed|errors?)\b|build (?:success|successful|failed|failure)|total time:|finished\b|",
    r"done\b|added \d+ packages|found \d+ vulnerabilit|\d+ (?:low|moderate|high|critical) severity|",
    r"compiled successfully|error: could not compile|\d+ actionable tasks?)",
);

pub(crate) static ANSI: LazyLock<Regex> =
    LazyLock::new(|| re(r"\x1b\[[0-9;?]*[ -/]*[@-~]|\x1b\][^\x07\x1b]*(?:\x07|\x1b\\)|\x1b[()][A-Z0-9]"));

pub(crate) static IMPORTANT: LazyLock<Regex> = LazyLock::new(|| re(IMPORTANT_P));

pub(crate) static NEG: LazyLock<Regex> = LazyLock::new(|| re(NEG_P));

pub(crate) static SUMMARY: LazyLock<Regex> = LazyLock::new(|| re(SUMMARY_P));

static RAW: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new([IMPORTANT_P, NEG_P, NOISE_P]).expect("squeeze pattern set"));

pub(crate) static TRACE: LazyLock<Regex> = LazyLock::new(|| {
    re(r#"^\s+at \S|^\s*File "|^\s+\.\.\. \d+ (?:more|common frames)|^\s+\^|^\s+\d+: \S|^\s+/\S+\.go:\d+"#)
});

pub(crate) static RUNTIME: LazyLock<Regex> = LazyLock::new(|| {
    re(concat!(
        r"^\s*(?:at\s+)?(?:java\.|javax\.|jdk\.|sun\.|com\.sun\.|kotlin\.|kotlinx\.|scala\.|groovy\.|org\.codehaus\.groovy\.|",
        r"org\.gradle\.|org\.apache\.maven\.|org\.junit\.|junit\.|org\.bukkit\.|org\.spigotmc\.|net\.minecraft\.|",
        r"io\.papermc\.|com\.destroystokyo\.|io\.netty\.|com\.mojang\.|net\.md_5\.|org\.springframework\.|System\.|Microsoft\.)",
        r"|java\.base/|node:internal|node_modules[/\\]|site-packages[/\\]|lib[/\\]python3|/rustc/|",
        r"\b(?:std|core|alloc|tokio)::|__rust_|rust_begin_unwind|/usr/local/go/src/",
    ))
});

pub(crate) static PREFIX: LazyLock<Regex> = LazyLock::new(|| {
    re(r"^(?:\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}(?:[.,]\d+)?Z? (?:[A-Z]{3,8} )?)?(?:\[[^\]]{0,60}\]:? ?){0,3}")
});

pub(crate) static NUMS: LazyLock<Regex> = LazyLock::new(|| re(r"0x[0-9a-fA-F]+|\b[0-9a-fA-F]{8,}\b|\d+"));

pub(crate) static QUIET: LazyLock<Regex> =
    LazyLock::new(|| re(r#"(?i)"[\w-]*(?:error|fail|exception|warn)[\w-]*"\s*:\s*(?:null|false|0|\[\]|\{\}|"")"#));

pub(crate) struct Kind {
    pub hot: bool,
    pub noise: bool,
}

pub(crate) fn kind(raw: &str) -> Kind {
    let m = RAW.matches(raw);
    let hot = m.matched(0) && (!m.matched(1) || IMPORTANT.is_match(&NEG.replace_all(raw, " ")));
    Kind { hot, noise: !hot && m.matched(2) }
}

#[cfg(test)]
pub(crate) fn hot(text: &str) -> bool {
    kind(text).hot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patterns_match_like_the_python_ones() {
        assert_eq!(ANSI.replace_all("\x1b[1;31mred\x1b[0m \x1b]0;t\x07x\x1b(B", ""), "red x");
        assert!(IMPORTANT.is_match("java.lang.NullPointerException: x"));
        assert!(IMPORTANT.is_match("Can't keep up"));
        assert!(IMPORTANT.is_match("upstream TIMED OUT"));
        assert!(!IMPORTANT.is_match("terrific errand"));
        assert!(TRACE.is_match("\tat a.b.C(C.java:1)"));
        assert!(TRACE.is_match("  File \"x.py\", line 1"));
        assert_eq!(PREFIX.replace("2026-10-30T04:00:00.819Z INFO [main] [db]: query", ""), "query");
        assert_eq!(PREFIX.replace("[12:00:01 INFO]: joined", ""), "joined");
        assert_eq!(NUMS.replace_all("id 0x1F deadbeef12 v1.2", "#"), "id # # v#.#");
        assert_eq!(QUIET.replace_all(r#"{"error": null, "lastWarning": "", "x": 1}"#, ""), r#"{, , "x": 1}"#);
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
            assert!(SUMMARY.is_match(line), "{line}");
        }
        assert!(!SUMMARY.is_match("step 3 done"));
        assert!(RUNTIME.is_match("\tat java.base/java.lang.Thread.run(Thread.java:1583)"));
        assert!(RUNTIME.is_match("\tat org.bukkit.plugin.java.JavaPluginLoader.x(J.java:1) ~[paper.jar:?]"));
        assert!(RUNTIME.is_match("    at Module._compile (node:internal/modules/cjs/loader:1105:14)"));
        assert!(RUNTIME.is_match("   3: std::panicking::begin_panic"));
        assert!(!RUNTIME.is_match("\tat dev.qdhshamiro.shop.ShopGui.open(ShopGui.java:57)"));
    }
}
