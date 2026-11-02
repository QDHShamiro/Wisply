use std::sync::LazyLock;

use regex::RegexSet;

use super::report::Sev;

#[derive(Clone, Copy, PartialEq)]
pub enum Scope {
    Code,
    Config,
    Any,
}

pub struct Rule {
    pub pattern: &'static str,
    pub why: &'static str,
    pub fix: &'static str,
    pub sev: Sev,
    pub scope: Scope,
    pub unless: &'static str,
}

const SQL_FIX: &str = "use a PreparedStatement with ? placeholders";
const SHELL_FIX: &str = "pass a fixed program and separate arguments, never one string with input in it";
const PATH_FIX: &str = "resolve the path, then check it stays inside the allowed folder (normalize + startsWith)";
const DESER_FIX: &str = "read plain data (JSON) instead, or allow only known classes";
const CRYPTO_FIX: &str = "use SHA-256 or better (passwords: bcrypt/argon2) and AES-GCM";
const CORS_FIX: &str = "allow only the origins that need it";
const DEBUG_FIX: &str = "turn debug off in anything that is deployed";

const fn rule(
    pattern: &'static str,
    why: &'static str,
    fix: &'static str,
    sev: Sev,
    scope: Scope,
    unless: &'static str,
) -> Rule {
    Rule { pattern, why, fix, sev, scope, unless }
}

const SQL: &str = "SQL built from joined text";
const SHELL: &str = "shell command built from joined text";
const PATH: &str = "path built from request data";
const WEAK_HASH: &str = "weak hash (MD5/SHA-1)";
const WEAK_CIPHER: &str = "weak cipher or ECB mode";
const CORS: &str = "CORS allows every origin (*)";
const DEBUG: &str = "debug mode is on";

pub const RULES: &[Rule] = &[
    rule(
        r#"(?i)"\s*(?:select|insert\s+into|update|delete\s+from|replace\s+into)\b[^"]*"\s*\+"#,
        SQL,
        SQL_FIX,
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(r#"(?i)\+\s*"\s*(?:where|and|or|values|set|order by|limit)\b"#, SQL, SQL_FIX, Sev::High, Scope::Code, ""),
    rule(
        r#"(?i)\.execute\w*\(\s*f["']"#,
        "SQL from an f-string",
        "pass the values as parameters: execute(sql, (a, b))",
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(
        r"(?i)\.query\(\s*`[^`]*\$\{",
        "SQL from a template string",
        "use placeholders and pass the values separately",
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(
        r"Runtime\.getRuntime\(\)\.exec\([^)]*\+",
        SHELL,
        "use ProcessBuilder with a fixed program and separate arguments",
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(
        r#""(?:sh|bash|cmd|cmd\.exe|powershell)"\s*,\s*"(?:-c|/c|-Command)"\s*,[^)]*\+"#,
        SHELL,
        SHELL_FIX,
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(
        r"subprocess\.\w+\([^)]*shell\s*=\s*True",
        "shell=True runs text through the shell",
        "pass a list of arguments",
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(
        r#"os\.(?:system|popen)\(\s*(?:f["']|[^)]*(?:\+|%|\.format\())"#,
        SHELL,
        SHELL_FIX,
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(
        r#"(?:^|[^\w.])(?:exec|execSync)\(\s*(?:`[^`]*\$\{|["'][^"']*["']\s*\+)"#,
        SHELL,
        SHELL_FIX,
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(
        r"\b(?:shell_exec|system|passthru|exec)\(\s*\$_(?:GET|POST|REQUEST)",
        "request data goes straight into a shell command",
        "never pass request data to a shell, use escapeshellarg or a fixed list",
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(r"(?:^|[^\w.])eval\(", "eval runs text as code", "parse the data instead", Sev::Medium, Scope::Code, ""),
    rule(
        r"\bObjectInputStream\s*\(",
        "unsafe deserialization (ObjectInputStream)",
        DESER_FIX,
        Sev::High,
        Scope::Code,
        "ObjectInputFilter",
    ),
    rule(
        r"new\s+Yaml\(\s*\)",
        "SnakeYAML default constructor builds any class",
        "new Yaml(new SafeConstructor(new LoaderOptions()))",
        Sev::High,
        Scope::Code,
        "SafeConstructor",
    ),
    rule(
        r"\b(?:pickle|cPickle|marshal|dill)\.loads?\(",
        "unsafe deserialization (pickle)",
        DESER_FIX,
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(r"\byaml\.load\(", "yaml.load builds any object", "use yaml.safe_load", Sev::High, Scope::Code, "SafeLoader"),
    rule(
        r"\bunserialize\(",
        "unsafe deserialization (unserialize)",
        DESER_FIX,
        Sev::High,
        Scope::Code,
        "allowed_classes",
    ),
    rule(
        r"node-serialize|serialize\.unserialize\(",
        "unsafe deserialization (node-serialize)",
        DESER_FIX,
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(
        r"new\s+(?:File|FileInputStream|FileReader)\([^)]*\b(?:getParameter|getHeader|getPathInfo|getQueryString)\(",
        PATH,
        PATH_FIX,
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(r"Paths\.get\([^)]*\b(?:getParameter|getHeader|getPathInfo)\(", PATH, PATH_FIX, Sev::High, Scope::Code, ""),
    rule(
        r"\b(?:open|send_file|send_from_directory)\(\s*(?:flask\.)?request\.",
        PATH,
        PATH_FIX,
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(r"os\.path\.join\([^)]*\brequest\.", PATH, PATH_FIX, Sev::High, Scope::Code, ""),
    rule(
        r"(?:readFile\w*|createReadStream|sendFile|path\.join|path\.resolve)\([^)]*\breq\.(?:params|query|body)",
        PATH,
        PATH_FIX,
        Sev::High,
        Scope::Code,
        "",
    ),
    rule(r#"MessageDigest\.getInstance\(\s*"(?:MD5|MD2|SHA-?1)""#, WEAK_HASH, CRYPTO_FIX, Sev::Medium, Scope::Code, ""),
    rule(
        r#"Cipher\.getInstance\(\s*"(?:DES|DESede|RC4|RC2|Blowfish|AES|AES/ECB[^"]*)""#,
        WEAK_CIPHER,
        CRYPTO_FIX,
        Sev::Medium,
        Scope::Code,
        "",
    ),
    rule(
        r#"\bhashlib\.(?:md5|sha1)\(|\bcreateHash\(\s*['"](?:md5|sha1)['"]|\b(?:md5|sha1)\(\s*\$"#,
        WEAK_HASH,
        CRYPTO_FIX,
        Sev::Medium,
        Scope::Code,
        "",
    ),
    rule(
        r"\b(?:DES|ARC4|Blowfish)\.new\(|MODE_ECB|createCipher\(",
        WEAK_CIPHER,
        CRYPTO_FIX,
        Sev::Medium,
        Scope::Code,
        "",
    ),
    rule(r#"(?i)Access-Control-Allow-Origin["']?\s*[:,=)]?\s*["']?\*"#, CORS, CORS_FIX, Sev::Medium, Scope::Any, ""),
    rule(
        r#"(?i)allowed?[_-]?origins?(?:Patterns)?\s*(?:\(|[:=])\s*\[?\s*["']\*["']"#,
        CORS,
        CORS_FIX,
        Sev::Medium,
        Scope::Any,
        "",
    ),
    rule(
        r#"\borigin\s*:\s*["']\*["']|\bAllowAnyOrigin\(\)|\bCORS\(\s*\w+\s*\)|\bcors\(\s*\)"#,
        CORS,
        CORS_FIX,
        Sev::Medium,
        Scope::Code,
        "",
    ),
    rule(
        r"\bdebug\s*=\s*True\b|^\s*DEBUG\s*=\s*True\b|\bFLASK_DEBUG\s*=\s*1",
        DEBUG,
        DEBUG_FIX,
        Sev::Medium,
        Scope::Code,
        "",
    ),
    rule(
        r"(?i)^\s*(?:debug|debug[_-]?mode|app[_-]?debug)\s*[:=]\s*(?:true|1|on|yes)\b",
        DEBUG,
        DEBUG_FIX,
        Sev::Low,
        Scope::Config,
        "",
    ),
    rule(
        r"\bwebSecurity\s*:\s*false|\bnodeIntegration\s*:\s*true|\bcontextIsolation\s*:\s*false",
        "Electron window with weakened isolation",
        "keep contextIsolation on and nodeIntegration off, expose only what is needed through a preload script",
        Sev::High,
        Scope::Code,
        "",
    ),
];

pub const CODE_EXT: &[&str] = &[
    "java", "kt", "kts", "scala", "groovy", "py", "js", "mjs", "cjs", "jsx", "ts", "tsx", "php", "rb", "go", "cs",
    "rs", "sh", "html", "htm", "vue", "svelte",
];

static SET: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new(RULES.iter().map(|r| r.pattern)).expect("rule patterns"));

pub fn code_file(name: &str) -> bool {
    name.rsplit_once('.').is_some_and(|(_, ext)| CODE_EXT.contains(&ext.to_lowercase().as_str()))
}

pub fn judge(line: &str, scope: Scope) -> Option<&'static Rule> {
    let code = line.trim_start();
    if code.starts_with("//") || code.starts_with('#') || code.starts_with('*') || code.starts_with("/*") {
        return None;
    }
    if !SET.is_match(line) {
        return None;
    }
    SET.matches(line)
        .iter()
        .map(|i| &RULES[i])
        .find(|r| (r.scope == Scope::Any || r.scope == scope) && (r.unless.is_empty() || !line.contains(r.unless)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn why(line: &str, scope: Scope) -> Option<&'static str> {
        judge(line, scope).map(|r| r.why)
    }

    #[test]
    fn each_language_pattern_fires_and_its_safe_twin_does_not() {
        let sql = r#"stmt.executeQuery("SELECT * FROM users WHERE name = '" + name + "'");"#;
        assert_eq!(why(sql, Scope::Code), Some(SQL));
        assert_eq!(why("cur.execute(f\"select {x}\")", Scope::Code), Some("SQL from an f-string"));
        assert_eq!(why("os.system(\"ping \" + host)", Scope::Code), Some(SHELL));
        assert_eq!(why("exec(`ls ${dir}`)", Scope::Code), Some(SHELL));
        assert!(why("child.exec(\"ls\")", Scope::Code).is_none());
        assert_eq!(why("new ObjectInputStream(in)", Scope::Code), Some("unsafe deserialization (ObjectInputStream)"));
        assert!(why("new ObjectInputStream(in); in.setObjectInputFilter(ObjectInputFilter.x)", Scope::Code).is_none());
        assert_eq!(why("data = pickle.loads(blob)", Scope::Code), Some("unsafe deserialization (pickle)"));
        assert_eq!(why("cfg = yaml.load(f)", Scope::Code), Some("yaml.load builds any object"));
        assert!(why("cfg = yaml.load(f, Loader=yaml.SafeLoader)", Scope::Code).is_none());
        assert_eq!(why("f = new File(base + req.getParameter(\"n\"))", Scope::Code), Some(PATH));
        assert_eq!(why("fs.readFile(path.join(root, req.query.f))", Scope::Code), Some(PATH));
        assert_eq!(why("MessageDigest.getInstance(\"MD5\")", Scope::Code), Some(WEAK_HASH));
        assert!(why("MessageDigest.getInstance(\"SHA-256\")", Scope::Code).is_none());
        assert_eq!(why("Cipher.getInstance(\"AES/ECB/PKCS5Padding\")", Scope::Code), Some(WEAK_CIPHER));
        assert!(why("Cipher.getInstance(\"AES/GCM/NoPadding\")", Scope::Code).is_none());
        assert_eq!(why("res.setHeader('Access-Control-Allow-Origin', '*')", Scope::Code), Some(CORS));
        assert_eq!(why("allowed_origins: \"*\"", Scope::Config), Some(CORS));
        assert_eq!(why("app.run(debug=True)", Scope::Code), Some(DEBUG));
        assert_eq!(why("debug: true", Scope::Config), Some(DEBUG));
        assert!(why("debug: false", Scope::Config).is_none() && why("debug: true", Scope::Code).is_none());
        assert!(why("// os.system(\"a\" + b)", Scope::Code).is_none());
        assert!(code_file("src/Main.java") && !code_file("README.md"));
    }
}
