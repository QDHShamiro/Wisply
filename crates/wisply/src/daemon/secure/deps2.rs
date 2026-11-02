use std::sync::LazyLock;

use regex::Regex;

use super::deps::Dep;

static PNPM: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^ {2}'?/?((?:@[^/@'\s]+/)?[^@'\s/]+)[@/](\d[^\s:'(]*)(?:\([^:]*\))?'?:\s*$").expect("pnpm")
});
static GRADLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"["']([\w.\-]+):([\w.\-]+):(\d[\w.\-]*)["']"#).expect("gradle"));
static YARN_VERSION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"^\s+version:?\s+"?([^"\s]+)"?"#).expect("yarn"));

fn npm(file: &str, name: &str, version: &str) -> Dep {
    Dep { file: file.into(), ecosystem: "npm", name: name.into(), version: version.into() }
}

pub fn pnpm_lock(file: &str, text: &str) -> Vec<Dep> {
    PNPM.captures_iter(text).map(|m| npm(file, &m[1], &m[2])).collect()
}

pub fn yarn_lock(file: &str, text: &str) -> Vec<Dep> {
    let mut out = Vec::new();
    let mut name = String::new();
    for line in text.lines() {
        if !line.starts_with([' ', '#']) && line.trim_end().ends_with(':') {
            let first = line.trim_end().trim_end_matches(':').split(',').next().unwrap_or("").trim().trim_matches('"');
            let at = first.rfind('@').filter(|i| *i > 0);
            name = at.map_or(String::new(), |i| first[..i].to_string());
            if let Some(i) = name.find("@npm") {
                name.truncate(i);
            }
        } else if !name.is_empty()
            && let Some(m) = YARN_VERSION.captures(line)
        {
            out.push(npm(file, &name, &m[1]));
            name.clear();
        }
    }
    out
}

pub fn gradle(file: &str, text: &str) -> Vec<Dep> {
    GRADLE
        .captures_iter(text)
        .map(|m| Dep {
            file: file.into(),
            ecosystem: "Maven",
            name: format!("{}:{}", &m[1], &m[2]),
            version: m[3].into(),
        })
        .collect()
}

pub fn poetry_lock(file: &str, text: &str) -> Vec<Dep> {
    let mut out = Vec::new();
    for block in text.split("[[package]]").skip(1) {
        let field = |key: &str| {
            block.lines().find_map(|l| {
                l.trim()
                    .strip_prefix(key)
                    .and_then(|r| r.trim().strip_prefix('='))
                    .map(|v| v.trim().trim_matches('"').to_string())
            })
        };
        if let (Some(name), Some(version)) = (field("name"), field("version")) {
            out.push(Dep { file: file.into(), ecosystem: "PyPI", name, version });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pairs(deps: Vec<Dep>) -> Vec<String> {
        deps.into_iter().map(|d| format!("{}@{}", d.name, d.version)).collect()
    }

    #[test]
    fn pnpm_yarn_gradle_and_poetry_give_exact_versions() {
        let pnpm = "lockfileVersion: '9.0'\n\npackages:\n\n  lodash@4.17.20:\n    resolution: {x}\n\n  '@babel/core@7.1.0':\n    x: 1\n\n  react-dom@18.2.0(react@18.2.0):\n    y: 2\n\n  /old/1.2.3:\n    z: 3\n";
        assert_eq!(
            pairs(pnpm_lock("pnpm-lock.yaml", pnpm)),
            ["lodash@4.17.20", "@babel/core@7.1.0", "react-dom@18.2.0", "old@1.2.3"]
        );
        let yarn = "# yarn lockfile v1\n\n\"@scope/pkg@^1.0.0\", \"@scope/pkg@^1.1.0\":\n  version \"1.1.5\"\n  resolved \"x\"\n\nlodash@^4.17.15:\n  version \"4.17.20\"\n\n\"chalk@npm:^4.0.0\":\n  version: 4.1.2\n";
        assert_eq!(pairs(yarn_lock("yarn.lock", yarn)), ["@scope/pkg@1.1.5", "lodash@4.17.20", "chalk@4.1.2"]);
        let gradle_text = "implementation 'org.yaml:snakeyaml:1.30'\napi(\"com.google.guava:guava:31.0-jre\")\nimplementation \"a:b:$v\"\n";
        assert_eq!(
            pairs(gradle("build.gradle", gradle_text)),
            ["org.yaml:snakeyaml@1.30", "com.google.guava:guava@31.0-jre"]
        );
        let poetry = "[[package]]\nname = \"requests\"\nversion = \"2.19.0\"\n\n[[package]]\nname = \"urllib3\"\nversion = \"1.24\"\n";
        assert_eq!(pairs(poetry_lock("poetry.lock", poetry)), ["requests@2.19.0", "urllib3@1.24"]);
    }
}
