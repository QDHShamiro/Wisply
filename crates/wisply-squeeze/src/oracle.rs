use regex::{Regex, RegexSet};
use std::sync::LazyLock;

fn re(pattern: &str) -> Regex {
    Regex::new(pattern).expect("squeeze pattern")
}

pub(crate) const IMPORTANT_P: &str = concat!(
    r"(?i:\b(?:errors?|exceptions?|err|fail|failed|failing|failure|failures|fatal|traceback|panic|panicked|",
    r"warn|warning|warnings|denied|refused|cannot|can't|unable|not found|no such|missing|undefined|",
    r"segfault|abort|aborted|timeout|timed out|killed|assert|assertion|severe|critical)\b|_(?:error|exception)s?\b|\berr_)",
    r"|[A-Z][A-Za-z0-9]*(?:Error|Exception)s?\b",
);

pub(crate) const NEG_P: &str = concat!(
    r"(?i)\b(?:0|no|zero|without)\s+(?:errors?|warnings?|failures?|failed|failing|issues?|problems?|vulnerabilit(?:y|ies))\b",
    r"|\b(?:errors?|warnings?|failures?|failed|failing)\s*[:=]?\s*0\b",
    r"|\b(?:errors?|warnings?)\s*[:=]\s*(?:none|null|false|\[\]|\{\})",
    r"|--?(?:no-)?fail-fast\b|\bfail_fast\b|\bno-fail\b",
);

pub(crate) const NOISE_P: &str = concat!(
    r"^\s*[\x{2800}-\x{28ff}]|\[[ .\-]*[=#]{2,}[=#>\-. ]*\]|[█▉▊▋▌▍▎▏▓▒░━]{5,}|<[=\-]{5,}>",
    r"|^\s*Progress \(\d+\):|^\s*Download(?:ing|ed)?(?: from \S+:)? https?://|^npm (?:http|timing|sill|verb)\b",
);

pub(crate) const SUMMARY_P: &str = concat!(
    r"(?i)^\s*(?:test result:|tests? run:|tests?:\s+\d|test suites?:|ran \d+ tests?|\d+ (?:passed|failed|tests?)\b|",
    r"=+ .*\b(?:passed|failed|errors?)\b|build (?:success|successful|failed|failure)|total time:|finished\b|",
    r"done (?:in\b|\(\d)|added \d+ packages|found \d+ vulnerabilit|\d+ (?:low|moderate|high|critical) severity|",
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

pub(crate) fn kind(raw: &str) -> (bool, bool) {
    let m = RAW.matches(raw);
    let hot = m.matched(0) && (!m.matched(1) || IMPORTANT.is_match(&NEG.replace_all(raw, " ")));
    (hot, !hot && m.matched(2))
}
