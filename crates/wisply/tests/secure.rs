use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use serde_json::Value;

const EXE: &str = env!("CARGO_BIN_EXE_wisply-full");

struct Home {
    path: PathBuf,
    ai: Option<PathBuf>,
}

impl Home {
    fn new(tag: &str) -> Home {
        let base = if cfg!(windows) { std::env::temp_dir() } else { PathBuf::from("/tmp") };
        let path = base.join(format!("ltd-secure-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        fs::write(path.join("v3"), "").unwrap();
        Home { path, ai: None }
    }

    fn run(&self, args: &[&str]) -> (i32, String) {
        let mut cmd = Command::new(EXE);
        cmd.args(args)
            .env("WISPLY_HOME", &self.path)
            .env(if cfg!(windows) { "USERPROFILE" } else { "HOME" }, &self.path)
            .env("CLAUDE_CODE_SESSION_ID", "it-secure")
            .env_remove("MSYSTEM")
            .env("WISPLY_SECURE_ANY_TIME", "1")
            .env_remove("WISPLY_SECURE_AI")
            .stdin(Stdio::null());
        if let Some(ai) = &self.ai {
            cmd.env("WISPLY_SECURE_AI", ai);
        }
        let out = cmd.output().unwrap();
        (out.status.code().unwrap_or(-1), String::from_utf8_lossy(&out.stdout).trim_end().to_string())
    }
}

impl Drop for Home {
    fn drop(&mut self) {
        let _ = self.run(&["stop"]);
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn google_key() -> String {
    format!("AIza{}", "Bq7x".repeat(9).get(..35).unwrap())
}

#[test]
fn secure_finds_a_key_a_known_hole_joined_sql_and_unsafe_patterns() {
    let home = Home::new("code");
    let repo = home.path.join("repo");
    fs::create_dir_all(repo.join(".git")).unwrap();
    fs::write(repo.join(".gitignore"), "target/\n").unwrap();
    fs::write(repo.join("config.yml"), format!("api_key: {}\npassword: changeme\ndebug: true\n", google_key()))
        .unwrap();
    fs::write(repo.join(".env"), "A=1\n").unwrap();
    fs::write(repo.join("requirements.txt"), "requests==2.19.0\n").unwrap();
    fs::write(repo.join("Db.java"), "stmt.executeQuery(\"SELECT * FROM users WHERE name = '\" + name + \"'\");\n")
        .unwrap();
    fs::write(repo.join("app.py"), "import pickle\ndata = pickle.loads(blob)\napp.run(debug=True)\n").unwrap();
    let at = repo.display().to_string();
    let (code, out) = home.run(&["secure", &at]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("config.yml:1 Google API key (AIza…)"), "{out}");
    assert!(out.contains("requirements.txt:1 requests 2.19.0 has ") && out.contains("update to 2."), "{out}");
    assert!(out.contains("high   Db.java:1 SQL built from joined text -> use a PreparedStatement"), "{out}");
    assert!(
        out.contains("app.py:2 unsafe deserialization (pickle)") && out.contains("app.py:3 debug mode is on"),
        "{out}"
    );
    assert!(out.contains("config.yml:3 debug mode is on"), "{out}");
    assert!(out.contains(".gitignore is not ignored") || out.contains(".gitignore .env is not ignored"), "{out}");
    let first = out.lines().nth(1).unwrap_or("");
    assert!(first.starts_with("high"), "{out}");
    let (_, json) = home.run(&["secure", &at, "--no-deps", "--json"]);
    let v: Value = serde_json::from_str(&json).unwrap_or_else(|e| panic!("{e}: {json}"));
    assert!(v["counts"]["high"].as_u64().unwrap() >= 3 && !json.contains("requests 2.19.0"), "{json}");
    assert!(v["findings"].as_array().unwrap().iter().all(|f| f["file"].is_string() && f["fix"].is_string()));
}

fn serve(hits: Arc<Mutex<Vec<(String, Instant)>>>) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let mut stream = stream;
            let mut buf = [0u8; 4096];
            let n = stream.read(&mut buf).unwrap_or(0);
            let head = String::from_utf8_lossy(&buf[..n]).to_string();
            let path = head.split_whitespace().nth(1).unwrap_or("/").to_string();
            hits.lock().unwrap().push((path.clone(), Instant::now()));
            let (status, kind, extra, body) = match path.as_str() {
                "/" => (
                    "200 OK",
                    "text/html",
                    "Server: nginx/1.18.0\r\nSet-Cookie: sid=abc123; Path=/\r\n",
                    "<html><script src=\"/app.js\"></script></html>",
                ),
                "/.env" => ("200 OK", "text/plain", "", "DB_PASS=hunter2\n"),
                "/.git/HEAD" => ("200 OK", "text/plain", "", "ref: refs/heads/main\n"),
                "/app.js.map" => ("200 OK", "application/json", "", "{\"version\":3,\"mappings\":\"AAAA\"}"),
                "/uploads/" => ("200 OK", "text/html", "", "<html><title>Index of /uploads/</title></html>"),
                "/backup.zip" => ("200 OK", "text/html", "", "<html>catch-all page</html>"),
                _ => ("404 Not Found", "text/html", "", "<html>nope</html>"),
            };
            let reply = format!(
                "HTTP/1.1 {status}\r\nContent-Type: {kind}\r\n{extra}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = stream.write_all(reply.as_bytes());
        }
    });
    port
}

#[test]
fn secure_checks_only_allowed_sites_one_request_per_second() {
    let home = Home::new("site");
    let hits = Arc::new(Mutex::new(Vec::new()));
    let port = serve(hits.clone());
    let url = format!("http://127.0.0.1:{port}");
    let (code, out) = home.run(&["secure", &url]);
    assert!(code != 0 && out.contains("not in your allowed sites"), "{code} {out}");
    assert!(hits.lock().unwrap().is_empty(), "a site that is not allowed got a request");
    let (_, listed) = home.run(&["secure", "allow"]);
    assert!(listed.contains("no allowed sites"), "{listed}");
    let (code, out) = home.run(&["secure", "allow", &format!("127.0.0.1:{port}")]);
    assert_eq!(code, 0, "{out}");
    let (code, out) = home.run(&["secure", &url]);
    assert_eq!(code, 0, "{out}");
    for want in [
        "high   /.env the .env file can be downloaded",
        "high   /.git/HEAD the .git folder is public",
        "medium / the site is served over plain http",
        "medium / cookie sid lacks HttpOnly, SameSite",
        "medium /uploads/ directory listing is on",
        "medium / no Content-Security-Policy",
        "low    /app.js.map source map is public",
        "low    / server header shows a version (nginx/1.18.0)",
    ] {
        assert!(out.contains(want), "missing {want}\n{out}");
    }
    assert!(!out.contains("backup.zip") && !out.contains("hunter2"), "{out}");
    let seen = hits.lock().unwrap().clone();
    assert!(seen.len() >= 10, "{seen:?}");
    for pair in seen.windows(2) {
        assert!(pair[1].1.duration_since(pair[0].1) >= Duration::from_millis(900), "faster than 1/s: {seen:?}");
    }
    let (_, json) = home.run(&["secure", &url, "--json"]);
    let v: Value = serde_json::from_str(&json).unwrap_or_else(|e| panic!("{e}: {json}"));
    assert!(v["counts"]["high"].as_u64().unwrap() >= 2, "{json}");
    let (_, removed) = home.run(&["secure", "allow", &format!("127.0.0.1:{port}"), "--remove"]);
    assert!(removed.contains("no longer allowed"), "{removed}");
}

fn stored_zip(files: &[(&str, &[u8])]) -> Vec<u8> {
    let (mut out, mut dir) = (Vec::new(), Vec::new());
    for (name, data) in files {
        let offset = out.len() as u32;
        for central in [false, true] {
            let target = if central { &mut dir } else { &mut out };
            target.extend(if central { 0x0201_4b50u32 } else { 0x0403_4b50u32 }.to_le_bytes());
            if central {
                target.extend([20u8, 0]);
            }
            target.extend([20u8, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            target.extend(0u32.to_le_bytes());
            target.extend((data.len() as u32).to_le_bytes());
            target.extend((data.len() as u32).to_le_bytes());
            target.extend((name.len() as u16).to_le_bytes());
            target.extend([0u8, 0]);
            if central {
                target.extend([0u8; 10]);
                target.extend(offset.to_le_bytes());
            }
            target.extend(name.as_bytes());
            if !central {
                target.extend(*data);
            }
        }
    }
    let at = out.len() as u32;
    out.extend(&dir);
    out.extend(0x0605_4b50u32.to_le_bytes());
    out.extend([0u8; 4]);
    out.extend((files.len() as u16).to_le_bytes());
    out.extend((files.len() as u16).to_le_bytes());
    out.extend((dir.len() as u32).to_le_bytes());
    out.extend(at.to_le_bytes());
    out.extend([0u8; 2]);
    out
}

#[test]
fn secure_reads_plugin_jars_and_apk_manifests() {
    let home = Home::new("app");
    let hook = format!("https://discord.com/api/webhooks/{}/{}", "1".repeat(18), "aB3-".repeat(8));
    let mut class = b"\xCA\xFE\xBA\xBE\x00\x00".to_vec();
    class.extend(b"\x00\x0aconnecting\x00");
    class.extend(format!("ghp_{}", "a1B2".repeat(6)).as_bytes());
    class.push(0);
    let plugin = b"name: Demo\nmain: x.Y\ncommands:\n  heal:\n    permission: demo.heal\n  fly:\n    usage: /fly\n";
    let config = format!("webhook: {hook}\ndebug: false\n");
    let jar = home.path.join("Demo.jar");
    fs::write(&jar, stored_zip(&[("plugin.yml", plugin), ("config.yml", config.as_bytes()), ("x/Y.class", &class)]))
        .unwrap();
    let (code, out) = home.run(&["secure", &jar.display().to_string()]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("Demo.jar!config.yml:1 Discord webhook"), "{out}");
    assert!(out.contains("Demo.jar!x/Y.class GitHub token (ghp_…)"), "{out}");
    assert!(out.contains("Demo.jar!plugin.yml:6 command /fly has no permission"), "{out}");
    assert!(!out.contains("/heal"), "{out}");
    let manifest = b"<manifest><application android:debuggable=\"true\" android:usesCleartextTraffic=\"true\" android:allowBackup=\"false\"><activity android:name=\".Open\" android:exported=\"true\"/></application></manifest>";
    let apk = home.path.join("demo.apk");
    fs::write(
        &apk,
        stored_zip(&[
            ("AndroidManifest.xml", manifest),
            ("assets/keys.json", format!("{{\"k\": \"{}\"}}", google_key()).as_bytes()),
        ]),
    )
    .unwrap();
    let (code, out) = home.run(&["secure", &apk.display().to_string()]);
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("high   AndroidManifest.xml the app is debuggable"), "{out}");
    assert!(
        out.contains("cleartext HTTP traffic is allowed") && out.contains(".Open is exported without a permission"),
        "{out}"
    );
    assert!(out.contains("demo.apk!assets/keys.json") && out.contains("Google API key"), "{out}");
    let exe = home.path.join("tool.exe");
    fs::write(&exe, [b"MZ\x00\x00".as_slice(), format!("token {}", hook).as_bytes(), b"\x00\x00"].concat()).unwrap();
    let (_, out) = home.run(&["secure", &exe.display().to_string()]);
    assert!(out.contains("tool.exe Discord webhook"), "{out}");
}

fn git(dir: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
        .args(["-c", "user.name=t", "-c", "user.email=t@example.test", "-C"])
        .arg(dir)
        .args(args)
        .output()
        .unwrap();
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

#[test]
fn secure_fix_branches_runs_the_ai_and_reports_before_and_after() {
    let mut home = Home::new("fix");
    let repo = home.path.join("repo");
    fs::create_dir_all(&repo).unwrap();
    git(&repo, &["init", "-q", "-b", "main"]);
    fs::write(repo.join(".gitignore"), "target/\n").unwrap();
    fs::write(repo.join("config.yml"), format!("api_key: {}\n", google_key())).unwrap();
    git(&repo, &["add", "-A"]);
    git(&repo, &["commit", "-q", "-m", "start"]);
    let script = home.path.join(if cfg!(windows) { "ai.cmd" } else { "ai.sh" });
    let body = if cfg!(windows) {
        "@echo off\r\n(echo api_key: ${API_KEY})> config.yml\r\n"
    } else {
        "#!/bin/sh\nprintf 'api_key: ${API_KEY}\\n' > config.yml\n"
    };
    fs::write(&script, body).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
    }
    home.ai = Some(script);
    let at = repo.display().to_string();
    let (code, out) = home.run(&["secure", "fix", &at]);
    assert_eq!(code, 0, "{out}");
    assert!(
        out.contains("on branch secure/fix-") && out.contains("before 1 (1 medium) -> after 0 (no findings)"),
        "{out}"
    );
    assert!(out.contains("nothing committed, nothing pushed"), "{out}");
    assert!(git(&repo, &["branch", "--show-current"]).starts_with("secure/fix-"));
    assert_eq!(
        git(&repo, &["rev-parse", "main"]),
        git(&repo, &["rev-parse", "HEAD"]),
        "main moved or a commit was made"
    );
    assert!(fs::read_to_string(repo.join("FIX.md")).unwrap().contains("Google API key (AIza…)"));
    assert!(!fs::read_to_string(repo.join("FIX.md")).unwrap().contains(&google_key()));
    let (code, out) = home.run(&["secure", "fix", &at]);
    assert!(code != 0 && out.contains("uncommitted changes"), "{code} {out}");
}
