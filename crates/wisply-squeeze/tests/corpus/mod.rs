#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind {
    Dense,
    Bash,
    Grep,
    Web,
}

pub struct Case {
    pub name: String,
    pub kind: Kind,
    pub text: String,
    pub must: Vec<String>,
}

fn case(name: &str, kind: Kind, text: String, must: &[&str]) -> Case {
    Case { name: name.into(), kind, text, must: must.iter().map(|m| (*m).into()).collect() }
}

fn word(i: usize) -> &'static str {
    const W: [&str; 12] =
        ["alpha", "beta", "gamma", "delta", "orbit", "quartz", "lumen", "ember", "tidal", "cedar", "nimbus", "vale"];
    W[i % W.len()]
}

fn dense() -> Vec<Case> {
    let mut out = Vec::new();
    for n in 0..4 {
        let files: String = (0..8 + n).map(|i| format!("\tmodified:   src/{}/{}.rs\n", word(i), word(i + n))).collect();
        out.push(case(
            &format!("git-status-{n}"),
            Kind::Dense,
            format!(
                "On branch feat/{}\nChanges not staged for commit:\n{files}\nno changes added to commit\n",
                word(n)
            ),
            &[],
        ));
    }
    for n in 0..4 {
        let rows: String = (0..10 + n)
            .map(|i| format!("-rw-r--r-- 1 user group {:>6} Nov  2 10:{:02} {}.txt\n", 100 * i + n, i, word(i)))
            .collect();
        out.push(case(&format!("ls-{n}"), Kind::Dense, rows, &[]));
    }
    for n in 0..4 {
        let rows: String = (0..12)
            .map(|i| format!("{:07x} {} {} the {} module\n", 0xabc000 + i * 17 + n, word(i), word(i + 1), word(i + 2)))
            .collect();
        out.push(case(&format!("git-log-{n}"), Kind::Dense, rows, &[]));
    }
    out
}

fn long_lines() -> Vec<Case> {
    let mut out = Vec::new();
    for n in 0..5 {
        let blob = "QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo".repeat(14 + n);
        let text = format!("HTTP/2 200\ncontent-type: text/plain\nx-token: {blob}\nserver: edge\n\nbody follows\n");
        out.push(case(&format!("curl-headers-{n}"), Kind::Bash, text, &["HTTP/2 200", "server: edge", "body follows"]));
    }
    for n in 0..5 {
        let args: String = (0..60 + n * 5).map(|i| format!(" --opt-{}={}", word(i), i)).collect();
        let rows: String = (0..6)
            .map(|i| format!("user {} 0.{} 1.{} /usr/bin/{} --serve{}\n", 1000 + i, i, i, word(i), args))
            .collect();
        out.push(case(
            &format!("ps-wide-{n}"),
            Kind::Bash,
            format!("USER PID %CPU %MEM COMMAND\n{rows}"),
            &["USER PID %CPU %MEM COMMAND"],
        ));
    }
    out
}

fn logs() -> Vec<Case> {
    let mut out = Vec::new();
    for n in 0..5 {
        let mut t: String = (0..45)
            .map(|i| format!("   Compiling {}-{} v1.{i}.0 (/work/crates/{}-{})\n", word(i), n, word(i), n))
            .collect();
        t.push_str("error[E0308]: mismatched types\n  --> src/main.rs:12:5\nerror: could not compile `app`\n");
        out.push(case(
            &format!("cargo-fail-{n}"),
            Kind::Bash,
            t,
            &["error[E0308]: mismatched types", "error: could not compile `app`"],
        ));
    }
    for n in 0..5 {
        let mut t: String = (0..40)
            .map(|i| {
                format!(
                    "Downloading from central: https://repo.example.org/maven/{}/{}/{n}.{i}/{}.pom\n",
                    word(i),
                    word(i + 3),
                    word(i)
                )
            })
            .collect();
        t.push_str(&format!("[ERROR] Failed to execute goal compile on project shop{n}\n[INFO] BUILD FAILURE\n"));
        out.push(case(
            &format!("maven-{n}"),
            Kind::Bash,
            t,
            &["[INFO] BUILD FAILURE", &format!("[ERROR] Failed to execute goal compile on project shop{n}")],
        ));
    }
    for n in 0..5 {
        let mut t: String = (0..50).map(|i| format!("test {}_{i} ... ok\n", word(i + n))).collect();
        t.push_str("test result: ok. 50 passed; 0 failed; 0 ignored\n");
        out.push(case(&format!("tests-ok-{n}"), Kind::Bash, t, &["test result: ok. 50 passed"]));
    }
    for n in 0..5 {
        let mut t =
            format!("[10:00:01 ERROR]: Could not load plugin shop{n}\njava.lang.IllegalStateException: no config\n");
        t.push_str(
            &(0..30)
                .map(|i| format!("\tat org.bukkit.plugin.Loader{i}.load(Loader.java:{})\n", 100 + i))
                .collect::<String>(),
        );
        t.push_str(&format!("\tat dev.example.shop{n}.Main.onEnable(Main.java:41)\n"));
        out.push(case(
            &format!("stack-{n}"),
            Kind::Bash,
            t,
            &[
                "java.lang.IllegalStateException: no config",
                &format!("dev.example.shop{n}.Main.onEnable(Main.java:41)"),
            ],
        ));
    }
    out
}

fn grep_cases() -> Vec<Case> {
    let mut out = Vec::new();
    for n in 0..10usize {
        let mut t = String::new();
        for f in 0..4 + n % 4 {
            for h in 0..3 + (n + f) % 9 {
                t.push_str(&format!(
                    "crates/{}/src/{}/{}.rs:{}:    let {} = self.{}({}, {});\n",
                    word(f),
                    word(f + 1),
                    word(f + 2),
                    10 + h * 7,
                    word(h),
                    word(h + n),
                    h,
                    f
                ));
            }
        }
        let mut must = Vec::new();
        if n % 2 == 0 {
            let line = format!("crates/{}/src/lib.rs:99:    panic!(\"fatal error in {}\");", word(n), word(n));
            t.push_str(&line);
            t.push('\n');
            must.push(line);
        }
        if n % 3 == 0 {
            t.push_str(&format!("src/wide.rs:5:{}\n", "x ".repeat(200)));
        }
        let kind = if n < 5 { Kind::Bash } else { Kind::Grep };
        out.push(Case { name: format!("grep-{n}"), kind, text: t, must });
    }
    out
}

fn web_cases() -> Vec<Case> {
    let mut out = Vec::new();
    for n in 0..5usize {
        let mut t = String::from("Skip to main content\nWe use cookies to improve your experience\nSign in\n");
        t.push_str(&format!("# {} release notes\n\n", word(n)));
        for p in 0..12 {
            t.push_str(&format!(
                "Version {n}.{p} changes the {} subsystem: the {} setting is now {} by default and the {} path is removed. See https://docs.example.org/{}/{}?utm_source=feed&utm_medium=mail for details.\n\n\n",
                word(p),
                word(p + 1),
                word(p + 2),
                word(p + 3),
                word(p),
                p
            ));
        }
        let repeat = format!("Error E{n}042 means the file is missing; run the repair command to fix it.");
        t.push_str("Related articles\nSubscribe to our newsletter\n");
        t.push_str(&format!("{repeat}\n© 2026 Example Inc. All rights reserved.\nPrivacy Policy\n{repeat}\n"));
        out.push(case(&format!("web-{n}"), Kind::Web, t, &[&format!("# {} release notes", word(n)), &repeat]));
    }
    out
}

pub fn all() -> Vec<Case> {
    let mut out = dense();
    out.extend(long_lines());
    out.extend(logs());
    out.extend(grep_cases());
    out.extend(web_cases());
    out
}
