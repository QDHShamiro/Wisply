mod fetch;
mod parse;
#[cfg(test)]
mod tests;

use std::fmt;
use std::path::{Path, PathBuf};

use serde_json::Value;

pub use fetch::{Crash, Got, Problem, Want, fetch};
pub use parse::{Hit, Scan, crash, fingerprint, normalize, scan};

pub const DEFAULT_LOG: &str = "logs/latest.log";
const SSH_PORT: u16 = 22;

#[derive(Clone)]
enum Auth {
    Pass(String),
    Key(PathBuf),
    None,
}

#[derive(Clone)]
pub struct Profile {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    auth: Auth,
}

impl fmt::Debug for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let auth = match self.auth {
            Auth::Pass(_) => "password",
            Auth::Key(_) => "key",
            Auth::None => "none",
        };
        write!(f, "Profile({} {}@{}:{} {auth})", self.name, self.user, self.host, self.port)
    }
}

impl Profile {
    pub fn usable(&self) -> bool {
        !self.host.is_empty() && !self.user.is_empty() && !matches!(self.auth, Auth::None)
    }
}

fn home_path(raw: &str, base: &Path) -> PathBuf {
    let raw = raw.trim();
    if let Some(rest) = raw.strip_prefix("~/").or_else(|| raw.strip_prefix("~\\")) {
        let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).map(PathBuf::from);
        return home.unwrap_or_default().join(rest);
    }
    let p = PathBuf::from(raw);
    if p.is_absolute() { p } else { base.join(p) }
}

fn profile_of(name: &str, v: &Value, base: &Path) -> Option<Profile> {
    let o = v.as_object()?;
    let s = |k: &str| o.get(k).and_then(Value::as_str).unwrap_or("").trim().to_string();
    let port = match o.get("port") {
        Some(Value::Number(n)) => n.as_u64().and_then(|p| u16::try_from(p).ok()),
        Some(Value::String(t)) => t.trim().parse().ok(),
        _ => None,
    };
    let pass = o.get("pass").or_else(|| o.get("password")).and_then(Value::as_str).unwrap_or("");
    let key = s("key");
    let auth = if !key.is_empty() {
        Auth::Key(home_path(&key, base))
    } else if !pass.is_empty() {
        Auth::Pass(pass.to_string())
    } else {
        Auth::None
    };
    Some(Profile { name: name.to_string(), host: s("host"), port: port.unwrap_or(SSH_PORT), user: s("user"), auth })
}

pub fn profiles_from(text: &str, base: &Path) -> Result<Vec<Profile>, String> {
    let v: Value = serde_json::from_str(text).map_err(|e| format!("not valid JSON ({e})"))?;
    let map = v.get("profiles").unwrap_or(&v).as_object().ok_or("expected {\"profiles\": {name: {...}}}")?;
    let mut out: Vec<Profile> = map.iter().filter_map(|(k, p)| profile_of(k, p, base)).collect();
    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

pub fn profiles(path: &Path) -> Result<Vec<Profile>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {} ({e})", path.display()))?;
    profiles_from(&text, path.parent().unwrap_or(Path::new(".")))
}

pub fn crash_dir(log: &str) -> Option<String> {
    let log = log.trim().replace('\\', "/");
    let root = log.strip_suffix("logs/latest.log")?;
    Some(format!("{root}crash-reports"))
}
