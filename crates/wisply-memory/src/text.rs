use std::collections::HashSet;
use std::sync::LazyLock;

use regex::{Captures, Regex};

const STOP: &str = "a an and are auf aus bei bin bis bitte can da dann das dass dem den der des die dir do doch du ein \
    eine einen einem einer er es for from für geht gib hab habe hast hat he hey hi i ich ihr im in is ist it ja kann \
    kannst mach mache machen mal me mein meine mich mir mit my nach nicht noch nur oder of on or please sag so soll the \
    then to um und uns unter vom von vor was we wie will wir with wo you zu zum zur über wisply lighting claude okay ok jetzt \
    schnell einfach auch alle alles aber also hier dort mehr sehr gut kurz neu neue schau schaue guck gucke check checke \
    look see warum wieso weshalb welche welcher welches welchen wann wer wen wem woher wohin womit wofür this that \
    these those what when where why how have has had get got want need just also more some any all";
const FTS_TERMS: usize = 16;
const SECRET_PATTERNS: &[&str] = &[
    r"\b(?:sk|pk|rk)-[A-Za-z0-9_-]{16,}",
    r"\bgh[pousr]_[A-Za-z0-9]{20,}",
    r"\bgsk_[A-Za-z0-9]{20,}",
    r"\bAKIA[0-9A-Z]{16}\b",
    r"\bxox[abpr]-[A-Za-z0-9-]{10,}",
    r"\beyJ[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{10,}\.[A-Za-z0-9_-]{10,}",
    r"\bsk-ant-[A-Za-z0-9_-]{10,}",
    r"\bsk_(?:live|test)_[A-Za-z0-9]{16,}",
    r"\bAIza[0-9A-Za-z_-]{30,}",
    r"\b(?:npm|glpat|hf|dop_v1|shpat|pypi)[_-][A-Za-z0-9_-]{16,}",
    r"\bgithub_pat_[A-Za-z0-9_]{20,}",
    r"\b[MNO][A-Za-z0-9_-]{23,25}\.[A-Za-z0-9_-]{6}\.[A-Za-z0-9_-]{27,}",
    r#"(?i)((?:password|passwort|kennwort)\s+(?:ist|is)\s+["'`]?)[^\s"'`,;]{4,}"#,
    r"(?i)((?:discord(?:app)?\.com|hooks\.slack\.com|webhook\.office\.com)/(?:api/)?(?:webhooks|services)/)\S+",
    r"(?i)(\bbearer\s+)[A-Za-z0-9._~+/=-]{12,}",
    r"(?i)(authorization\s*[:=]\s*(?:basic|token)\s+)\S{8,}",
    r"(?i)(://[^\s:/@]+:)[^\s@/]{3,}(@)",
    r"(?s)(-----BEGIN [A-Z ]*PRIVATE KEY-----).*?(?:-----END [A-Z ]*PRIVATE KEY-----|$)",
    r#"(?i)((?:password|passwort|passwd|pwd|kennwort|secret|client[_-]?secret|token|access[_-]?token|auth[_-]?token|api[_-]?key|apikey|license[_-]?key|private[_-]?key)[\w-]*\s*[:=]\s*["'`]?)[^\s"'`,;]{4,}"#,
];

static STOPS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| STOP.split_whitespace().collect());
static TOKEN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[^\W_][\w.-]*").expect("token pattern"));
static SECRETS: LazyLock<Vec<Regex>> =
    LazyLock::new(|| SECRET_PATTERNS.iter().map(|p| Regex::new(p).expect("secret pattern")).collect());

pub fn stop_word(word: &str) -> bool {
    STOPS.contains(word)
}

pub fn scrub(text: &str) -> String {
    let mut out = text.to_string();
    for rx in SECRETS.iter() {
        out = rx
            .replace_all(&out, |m: &Captures| {
                format!("{}[secret]{}", m.get(1).map_or("", |g| g.as_str()), m.get(2).map_or("", |g| g.as_str()))
            })
            .into_owned();
    }
    out
}

static LEAD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)^[\s,:;.\-–—]*(?:(?:bitte|please|mal|doch|dir|dass|daß|that|to)\b[\s,:;.\-–—]*)+")
        .expect("lead pattern")
});

pub fn fact(raw: &str) -> String {
    let flat = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    let body = LEAD.replace(&flat, "");
    let body = body.trim().trim_end_matches(['.', ',', ';', ':', ' ']).trim();
    let mut chars = body.chars();
    match chars.next() {
        Some(c) => format!("{}{}", c.to_uppercase(), chars.as_str()),
        None => String::new(),
    }
}

pub fn oneline(text: &str, n: usize) -> String {
    let s = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if s.chars().count() <= n {
        return s;
    }
    let cut: String = s.chars().take(n.saturating_sub(1)).collect();
    format!("{}…", cut.trim_end())
}

fn tokens(text: &str) -> impl Iterator<Item = String> + '_ {
    TOKEN.find_iter(text).map(|m| m.as_str().trim_matches(['.', '-']).to_string())
}

pub fn fts_query(text: &str) -> String {
    let lower = text.to_lowercase();
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for t in tokens(&lower) {
        if t.chars().count() < 2 || stop_word(&t) || !seen.insert(t.clone()) {
            continue;
        }
        let t = t.replace('"', "");
        out.push(if t.chars().count() >= 4 { format!("\"{t}\"*") } else { format!("\"{t}\"") });
        if out.len() == FTS_TERMS {
            break;
        }
    }
    out.join(" OR ")
}

pub(crate) fn title_words(text: &str) -> HashSet<String> {
    tokens(&text.to_lowercase()).filter(|w| w.chars().count() > 2).collect()
}

pub fn similar(a: &str, b: &str) -> f64 {
    let (x, y) = (title_words(a), title_words(b));
    x.intersection(&y).count() as f64 / x.union(&y).count().max(1) as f64
}

pub fn short(n: usize) -> String {
    for (size, unit) in [(1_000_000, "M"), (1_000, "k")] {
        if n >= size {
            let v = n as f64 / size as f64;
            return if n >= 10 * size { format!("{v:.0}{unit}") } else { format!("{v:.1}{unit}") };
        }
    }
    n.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_helpers_match_2x() {
        assert_eq!(scrub("token: abcdef123 and ghp_abcdefghijklmnopqrstuv1"), "token: [secret] and [secret]");
        assert_eq!(scrub("password=hunter22!"), "password=[secret]");
        let fakes = [
            "key sk-ant-api03-AbCdEfGhIjKlMnOpQrSt",
            "key sk-proj1234567890abcdefghij",
            "ghp_abcdefghijklmnopqrstuvwxyz0123",
            "github_pat_11ABCDEFG0abcdefghijkl_mnop",
            "AKIAABCDEFGHIJKLMNOP",
            "xoxb-1234567890-abcdefghij",
            "Authorization: Bearer abcdefghijklmnop1234",
            "https://discord.com/api/webhooks/123456789/AbCdEfGh_ijkl-MNOP",
            "Passwort ist hunter2secret",
            "db_password = 'S3cr3tValue'",
            "mysql://root:topsecretpw@localhost/db",
            "AIzaSyA1234567890abcdefghijklmnopqrstuv",
            "stripe sk_live_abcdefghij1234567890",
        ];
        for f in fakes {
            let out = scrub(f);
            let secret = f.rsplit(['/', ' ', ':', '=', '\'']).next().unwrap_or("");
            assert!(out.contains("[secret]") && (secret.len() < 8 || !out.contains(secret)), "{f} -> {out}");
        }
        assert_eq!(fact("dass Brave der einzige Browser ist."), "Brave der einzige Browser ist");
        assert_eq!(fact(", bitte dass  total   wichtig"), "Total wichtig");
        assert_eq!(fact("that the port is 25565"), "The port is 25565");
        assert_eq!(fact("to use Maven"), "Use Maven");
        assert_eq!(fact("Tomorrow ist frei"), "Tomorrow ist frei");
        assert_eq!(oneline("  a \n b\tc  ", 10), "a b c");
        assert_eq!(oneline("abcdef ghij", 8), "abcdef…");
        assert_eq!(fts_query("Wie deploye ich den Wächter? a.b-c x"), r#""deploye"* OR "wächter"* OR "a.b-c"*"#);
        assert_eq!(fts_query("the and"), "");
        assert!((similar("Brave is the browser", "Brave browser rules") - 0.5).abs() < 1e-9);
        assert_eq!(short(950), "950");
        assert_eq!(short(1_260), "1.3k");
        assert_eq!(short(12_500), "12k");
    }
}
