use std::sync::LazyLock;

use regex::Regex;

const SECRET: &str = "[secret]";
const PATH: &str = "[path]";
const MAIL: &str = "[mail]";

static TOKENS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r"-----BEGIN [A-Z ]*PRIVATE KEY-----[\s\S]*?(?:-----END [A-Z ]*PRIVATE KEY-----|$)",
        r"\bsk-[A-Za-z0-9_-]{12,}",
        r"\bgh[pousr]_[A-Za-z0-9]{16,}",
        r"\bgithub_pat_\w{16,}",
        r"\bAKIA[0-9A-Z]{16}\b",
        r"\bxox[abprs]-[\w-]{10,}",
        r"\bAIza[\w-]{30,}",
        r"\beyJ[\w-]{8,}\.[\w-]{8,}\.[\w-]*",
        r"(?i)\bbearer\s+[\w.~+/=-]{12,}",
        r"\b[0-9a-fA-F]{32,}\b",
        r"\b[A-Za-z0-9+/_-]{40,}={0,2}",
    ]
    .iter()
    .map(|p| Regex::new(p).expect("secret token"))
    .collect()
});
static ASSIGN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?i)\b(password|passwd|pwd|passphrase|secret|token|api[_-]?key|license[_-]?key|authorization|credential)s?(["']?\s*[:=]\s*["']?)[^\s"',;]+"#,
    )
    .expect("secret assign")
});
static PATHS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r#"(?i)\b[a-z]:[\\/][^\s"'<>|*?]*"#,
        r#"\\\\[\w.$-]+\\[^\s"'<>|]*"#,
        r#"(?:^|[\s("'=])(?:~|\.\.?)?/(?:[\w.@+-]+/)+[\w.@+-]*"#,
    ]
    .iter()
    .map(|p| Regex::new(p).expect("path"))
    .collect()
});
static MAILS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\w.+-]+@[\w-]+(?:\.[\w-]+)+").expect("mail"));

pub fn scrub(text: &str, max: usize) -> String {
    let mut out: String = text.chars().filter(|c| !c.is_control() || c.is_whitespace()).collect();
    for rx in TOKENS.iter() {
        out = rx.replace_all(&out, SECRET).into_owned();
    }
    out = ASSIGN.replace_all(&out, format!("${{1}}${{2}}{SECRET}").as_str()).into_owned();
    out = MAILS.replace_all(&out, MAIL).into_owned();
    for (i, rx) in PATHS.iter().enumerate() {
        out = if i == 2 {
            rx.replace_all(&out, |c: &regex::Captures| {
                let lead = c[0].chars().next().filter(|ch| ch.is_whitespace() || "(\"'=".contains(*ch));
                format!("{}{PATH}", lead.map(String::from).unwrap_or_default())
            })
            .into_owned()
        } else {
            rx.replace_all(&out, PATH).into_owned()
        };
    }
    let flat = out.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= max {
        return flat;
    }
    let cut: String = flat.chars().take(max.saturating_sub(1)).collect();
    format!("{}…", cut.trim_end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_tokens_and_assignments_are_masked() {
        let cases = [
            "key sk-ant-abcdefghijklmnop1234 here",
            "ghp_abcdefghijklmnopqrstuvwx",
            "Authorization: Bearer abcdefghijklmnop12345",
            "password=hunter2hunter2",
            "AKIAABCDEFGHIJKLMNOP",
            "eyJhbGciOiJIUzI1.eyJzdWIiOiIxMjM0.abcdefghij",
            "deadbeefdeadbeefdeadbeefdeadbeef00",
        ];
        for case in cases {
            let out = scrub(case, 200);
            assert!(out.contains(SECRET), "{case} -> {out}");
            for leak in ["abcdefghijklmnop", "hunter2", "deadbeefdeadbeef", "eyJzdWIi"] {
                assert!(!out.contains(leak), "{case} -> {out}");
            }
        }
        assert_eq!(scrub("password: hunter2hunter2", 200), "password: [secret]");
    }

    #[test]
    fn paths_and_mails_never_leave() {
        let out = scrub(r"edited C:\Users\someone\proj\a.rs and /home/x/y/b.rs, mail me@example.com", 200);
        assert!(!out.contains("someone") && !out.contains("/home") && !out.contains("example.com"), "{out}");
        assert!(out.contains(PATH) && out.contains(MAIL));
    }

    #[test]
    fn plain_text_stays_and_is_cut_to_length() {
        assert_eq!(scrub("all tests pass,\n  3 fixed", 80), "all tests pass, 3 fixed");
        let out = scrub(&"word ".repeat(100), 20);
        assert_eq!(out.chars().count(), 20);
        assert!(out.ends_with('…'));
        assert_eq!(scrub("a\u{7}b\tc", 10), "ab c");
    }
}
