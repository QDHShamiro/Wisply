use std::fs;
use std::path::{Path, PathBuf};

use wisply_core::Fail;

const FILE: &str = "secure-allow.txt";

pub struct Site {
    pub scheme: Option<String>,
    pub host: String,
}

pub fn parse(text: &str) -> Option<Site> {
    let text = text.trim();
    let (scheme, rest) = match text.split_once("://") {
        Some((s, r)) if s.eq_ignore_ascii_case("http") || s.eq_ignore_ascii_case("https") => {
            (Some(s.to_lowercase()), r)
        }
        Some(_) => return None,
        None => (None, text),
    };
    let host = rest.split(['/', '?', '#']).next()?.rsplit('@').next()?.trim_end_matches('.').to_lowercase();
    let (name, port) = match host.rsplit_once(':') {
        Some((n, p)) if p.chars().all(|c| c.is_ascii_digit()) && !p.is_empty() => (n.to_string(), Some(p.to_string())),
        _ => (host.clone(), None),
    };
    let valid = !name.is_empty()
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        && !name.starts_with('.')
        && !name.starts_with('-');
    if !valid {
        return None;
    }
    let default =
        matches!((scheme.as_deref(), port.as_deref()), (Some("http"), Some("80")) | (Some("https"), Some("443")));
    let host = match port {
        Some(p) if !default => format!("{name}:{p}"),
        _ => name,
    };
    Some(Site { scheme, host })
}

pub fn looks_like_site(arg: &str) -> bool {
    arg.starts_with("http://")
        || arg.starts_with("https://")
        || (!arg.contains(['/', '\\']) && arg.contains('.') && parse(arg).is_some())
}

fn path(home: &Path) -> PathBuf {
    home.join(FILE)
}

pub fn list(home: &Path) -> Vec<String> {
    fs::read_to_string(path(home))
        .unwrap_or_default()
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect()
}

pub fn permitted(home: &Path, host: &str) -> bool {
    list(home).iter().any(|h| h == host)
}

fn save(home: &Path, hosts: &[String]) -> Result<(), Fail> {
    fs::create_dir_all(home)
        .and_then(|()| wisply_core::save(path(home), hosts.join("\n") + "\n"))
        .map_err(|e| Fail::new(format!("cannot save {}: {e}", path(home).display())))
}

pub fn command(home: &Path, args: &[String], remove: bool) -> Result<String, Fail> {
    let hosts = list(home);
    let Some(arg) = args.first() else {
        return Ok(if hosts.is_empty() {
            "no allowed sites yet: wisply secure allow <your-domain>".into()
        } else {
            format!("allowed sites ({}): {}", hosts.len(), hosts.join(", "))
        });
    };
    let Some(site) = parse(arg) else {
        return Err(Fail::hint(format!("'{arg}' is not a domain"), "wisply secure allow example.com"));
    };
    let mut hosts = hosts;
    if remove {
        hosts.retain(|h| *h != site.host);
        save(home, &hosts)?;
        return Ok(format!("{} is no longer allowed", site.host));
    }
    if hosts.contains(&site.host) {
        return Ok(format!("{} was already allowed", site.host));
    }
    hosts.push(site.host.clone());
    save(home, &hosts)?;
    Ok(format!("{} may now be checked with: wisply secure {}", site.host, site.host))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domains_are_normalised_and_only_exact_hosts_match() {
        let host = |s: &str| parse(s).map(|p| p.host);
        assert_eq!(host("HTTPS://Example.com/path?x=1").as_deref(), Some("example.com"));
        assert_eq!(host("example.com:443").as_deref(), Some("example.com:443"));
        assert_eq!(host("https://example.com:443").as_deref(), Some("example.com"));
        assert_eq!(host("http://127.0.0.1:8080/").as_deref(), Some("127.0.0.1:8080"));
        assert_eq!(host("user@evil.test/").as_deref(), Some("evil.test"));
        assert!(parse("ftp://x.test").is_none() && parse("bad host").is_none() && parse("").is_none());
        assert!(looks_like_site("example.com") && looks_like_site("http://a.b") && !looks_like_site("./a.jar"));
        let home = std::env::temp_dir().join(format!("wsp-allow-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        command(&home, &["Example.com".into()], false).unwrap();
        assert!(permitted(&home, "example.com") && !permitted(&home, "www.example.com"));
        assert!(command(&home, &[], false).unwrap().contains("example.com"));
        command(&home, &["example.com".into()], true).unwrap();
        assert!(!permitted(&home, "example.com"));
        fs::remove_dir_all(&home).unwrap();
    }
}
