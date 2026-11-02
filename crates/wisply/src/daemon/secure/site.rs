use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;

use super::allow::{self, Site};
use super::fetch::{Client, Resp};
use super::report::{Finding, Report, Sev};

const MAX_HOPS: usize = 4;
const MAX_MAPS: usize = 3;
const LISTING_PATHS: &[&str] = &["/uploads/", "/backup/"];
const HEADERS: &[(&str, &str, Sev, &str)] = &[
    (
        "content-security-policy",
        "no Content-Security-Policy",
        Sev::Medium,
        "send Content-Security-Policy, start with default-src 'self'",
    ),
    ("x-content-type-options", "no X-Content-Type-Options", Sev::Low, "send X-Content-Type-Options: nosniff"),
    ("referrer-policy", "no Referrer-Policy", Sev::Low, "send Referrer-Policy: strict-origin-when-cross-origin"),
    (
        "permissions-policy",
        "no Permissions-Policy",
        Sev::Low,
        "send Permissions-Policy and switch off what you do not use",
    ),
];

struct Probe {
    path: &'static str,
    why: &'static str,
    sev: Sev,
    fix: &'static str,
    proof: fn(&Resp) -> bool,
}

fn html(r: &Resp) -> bool {
    r.header("content-type").is_some_and(|t| t.contains("html"))
}

fn dotenv(r: &Resp) -> bool {
    !html(r) && r.text().lines().any(|l| l.contains('=') && !l.trim_start().starts_with('<'))
}

fn zipped(r: &Resp) -> bool {
    r.body.starts_with(b"PK") || r.body.starts_with(&[0x1f, 0x8b])
}

fn sql(r: &Resp) -> bool {
    let t = r.text().to_lowercase();
    !html(r) && (t.contains("create table") || t.contains("insert into"))
}

const PROBES: &[Probe] = &[
    Probe {
        path: "/.env",
        why: "the .env file can be downloaded",
        sev: Sev::High,
        fix: "block dotfiles in the web server and rotate every key in it",
        proof: dotenv,
    },
    Probe {
        path: "/.git/HEAD",
        why: "the .git folder is public",
        sev: Sev::High,
        fix: "block /.git in the web server or do not deploy it, then rotate secrets found in the history",
        proof: |r| r.text().trim_start().starts_with("ref:"),
    },
    Probe {
        path: "/backup.zip",
        why: "a backup archive can be downloaded",
        sev: Sev::High,
        fix: "delete the backup from the web root",
        proof: zipped,
    },
    Probe {
        path: "/backup.sql",
        why: "a database dump can be downloaded",
        sev: Sev::High,
        fix: "delete the dump from the web root",
        proof: sql,
    },
    Probe {
        path: "/dump.sql",
        why: "a database dump can be downloaded",
        sev: Sev::High,
        fix: "delete the dump from the web root",
        proof: sql,
    },
    Probe {
        path: "/backup.tar.gz",
        why: "a backup archive can be downloaded",
        sev: Sev::High,
        fix: "delete the backup from the web root",
        proof: zipped,
    },
];

static SCRIPT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"(?i)<script[^>]*\ssrc\s*=\s*["']([^"']+)["']"#).expect("script"));
static MIXED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)<(script|img|iframe|link|source|audio|video)\b[^>]*\s(?:src|href)\s*=\s*["']http://[^"'\s>]+"#)
        .expect("mixed")
});

fn add(report: &mut Report, sev: Sev, file: &str, why: impl Into<String>, fix: impl Into<String>) {
    report.found.push(Finding::new(sev, "site", file, 0, why, fix));
}

fn locate(base: &str, from: &str, loc: &str) -> String {
    if loc.starts_with("http://") || loc.starts_with("https://") {
        loc.to_string()
    } else if loc.starts_with('/') {
        format!("{base}{loc}")
    } else {
        let dir = from.rsplit_once('/').map_or("", |(d, _)| d);
        format!("{dir}/{loc}")
    }
}

fn follow(client: &mut Client, home: &Path, base: &str, report: &mut Report) -> Result<(String, Resp), String> {
    let mut url = format!("{base}/");
    for _ in 0..MAX_HOPS {
        let resp = client.get(&url)?;
        let next = match (resp.status, resp.header("location")) {
            (301 | 302 | 303 | 307 | 308, Some(loc)) => locate(base, &url, loc),
            _ => return Ok((url, resp)),
        };
        let Some(target) = allow::parse(&next) else { return Ok((url, resp)) };
        if !allow::permitted(home, &target.host) {
            report.notes.push(format!("redirects to {} which is not in your allowed sites, not followed", target.host));
            return Ok((url, resp));
        }
        if url.starts_with("https://") && next.starts_with("http://") {
            add(report, Sev::Medium, "/", "https redirects to plain http", "redirect only to https URLs");
        }
        url = next;
    }
    Err("too many redirects".into())
}

fn security_headers(report: &mut Report, resp: &Resp, https: bool) {
    if https && resp.header("strict-transport-security").is_none() {
        add(
            report,
            Sev::Medium,
            "/",
            "no Strict-Transport-Security",
            "send Strict-Transport-Security: max-age=31536000; includeSubDomains",
        );
    }
    for (name, why, sev, fix) in HEADERS {
        if resp.header(name).is_none() {
            add(report, *sev, "/", *why, *fix);
        }
    }
    let framed = resp.header("x-frame-options").is_some()
        || resp.header("content-security-policy").is_some_and(|c| c.contains("frame-ancestors"));
    if !framed {
        add(
            report,
            Sev::Low,
            "/",
            "the page can be framed by other sites",
            "send X-Frame-Options: DENY or CSP frame-ancestors",
        );
    }
    for name in ["server", "x-powered-by"] {
        if let Some(v) = resp.header(name).filter(|v| v.chars().any(|c| c.is_ascii_digit())) {
            add(
                report,
                Sev::Low,
                "/",
                format!("{name} header shows a version ({v})"),
                format!("hide the {name} version"),
            );
        }
    }
    if resp.header("access-control-allow-origin") == Some("*") {
        add(report, Sev::Medium, "/", "CORS allows every origin (*)", "allow only the origins that need it");
    }
}

fn cookies(report: &mut Report, resp: &Resp, https: bool) {
    for raw in resp.all("set-cookie") {
        let low = raw.to_lowercase();
        let name = raw.split('=').next().unwrap_or("").trim().to_string();
        let flags: Vec<&str> = low.split(';').skip(1).map(str::trim).collect();
        let has = |f: &str| flags.iter().any(|x| *x == f || x.starts_with(&format!("{f}=")));
        let mut missing = Vec::new();
        if https && !has("secure") {
            missing.push("Secure");
        }
        if !has("httponly") {
            missing.push("HttpOnly");
        }
        let weak = !missing.is_empty();
        if !has("samesite") {
            missing.push("SameSite");
        }
        if !missing.is_empty() {
            let sev = if weak { Sev::Medium } else { Sev::Low };
            add(
                report,
                sev,
                "/",
                format!("cookie {name} lacks {}", missing.join(", ")),
                "set the missing cookie flags",
            );
        }
    }
}

fn mixed(report: &mut Report, page: &str) {
    let hits: Vec<_> = MIXED.captures_iter(page).collect();
    if let Some(first) = hits.first() {
        let sev = if matches!(first[1].to_lowercase().as_str(), "script" | "iframe" | "link") {
            Sev::Medium
        } else {
            Sev::Low
        };
        add(
            report,
            sev,
            "/",
            format!("{} http:// resource(s) on an https page (<{}>)", hits.len(), first[1].to_lowercase()),
            "load every resource over https",
        );
    }
}

fn listing(r: &Resp) -> bool {
    let t = r.text();
    r.status == 200
        && (t.contains("<title>Index of /") || t.contains("Directory listing for") || t.contains("<h1>Index of"))
}

fn exposed(client: &mut Client, base: &str, report: &mut Report, page: &Resp) {
    if listing(page) {
        add(report, Sev::Medium, "/", "directory listing is on", "turn off autoindex / directory browsing");
    }
    for p in PROBES {
        if let Ok(r) = client.get(&format!("{base}{}", p.path))
            && r.status == 200
            && (p.proof)(&r)
        {
            add(report, p.sev, p.path, p.why, p.fix);
        }
    }
    for path in LISTING_PATHS {
        if let Ok(r) = client.get(&format!("{base}{path}"))
            && listing(&r)
        {
            add(report, Sev::Medium, path, "directory listing is on", "turn off autoindex / directory browsing");
        }
    }
    let text = page.text();
    let scripts: Vec<String> = SCRIPT
        .captures_iter(&text)
        .map(|m| m[1].to_string())
        .filter(|s| !s.starts_with("http") && !s.starts_with("//"))
        .take(MAX_MAPS)
        .collect();
    for src in scripts {
        let path = if src.starts_with('/') { src.clone() } else { format!("/{src}") };
        let path = path.split(['?', '#']).next().unwrap_or("").to_string() + ".map";
        if let Ok(r) = client.get(&format!("{base}{path}"))
            && r.status == 200
            && !html(&r)
            && r.text().contains("\"mappings\"")
        {
            add(
                report,
                Sev::Low,
                &path,
                "source map is public (your original source can be rebuilt)",
                "do not deploy .map files",
            );
        }
    }
}

pub fn check(home: &Path, site: &Site) -> Result<Report, String> {
    if !allow::permitted(home, &site.host) {
        return Err(format!("{} is not in your allowed sites", site.host));
    }
    let mut report = Report::new(site.host.clone());
    let mut client = Client::new();
    let (mut scheme, mut tried_https) = (site.scheme.clone().unwrap_or_else(|| "https".into()), false);
    let first = loop {
        let base = format!("{scheme}://{}", site.host);
        match follow(&mut client, home, &base, &mut report) {
            Ok(got) => break (base, got),
            Err(e) if site.scheme.is_none() && scheme == "https" && !tried_https => {
                tried_https = true;
                add(
                    &mut report,
                    Sev::High,
                    "/",
                    format!("https failed: {}", e.chars().take(120).collect::<String>()),
                    "serve the site over https with a valid certificate that matches the domain",
                );
                scheme = "http".into();
            }
            Err(e) => return Err(format!("{} is not reachable: {e}", site.host)),
        }
    };
    let (base, (url, page)) = first;
    let https = url.starts_with("https://");
    if !https && !tried_https {
        add(
            &mut report,
            Sev::Medium,
            "/",
            "the site is served over plain http",
            "serve it over https and redirect http to it",
        );
    }
    if https
        && site.scheme.is_none()
        && let Ok(r) = client.get(&format!("http://{}/", site.host))
        && !(matches!(r.status, 301 | 302 | 307 | 308)
            && r.header("location").is_some_and(|l| l.starts_with("https://")))
    {
        add(&mut report, Sev::Medium, "/", "http:// does not redirect to https", "redirect all http requests to https");
    }
    if page.status >= 400 {
        report.notes.push(format!("the start page answered {}, header checks use that answer", page.status));
    }
    security_headers(&mut report, &page, https);
    cookies(&mut report, &page, https);
    if https {
        mixed(&mut report, &page.text());
    }
    exposed(&mut client, &base, &mut report, &page);
    report.checked.push(format!("{} requests, 1/s", client.sent));
    report.sorted();
    Ok(report)
}
