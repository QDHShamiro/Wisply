use std::collections::{HashMap, VecDeque};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use wisply_proto::{random_token, same_token};

pub const CODE_TTL: Duration = Duration::from_secs(600);
const TOKEN_RATE: (usize, Duration) = (20, Duration::from_secs(1));
const PAIR_RATE: (usize, Duration) = (5, Duration::from_secs(60));
const CODES_MAX: usize = 5;
const CODE_CHARS: usize = 32;
const NAME_MAX: usize = 30;
const FILE: &str = "devices.json";

#[derive(Clone, Debug, PartialEq)]
pub struct Device {
    pub name: String,
    pub hash: String,
    pub created: i64,
}

#[derive(Debug, PartialEq)]
pub enum Verdict {
    Allowed(String),
    Denied,
    Limited,
}

#[derive(Debug, PartialEq)]
pub enum PairError {
    Rejected,
    Limited,
    Failed,
}

struct Limiter {
    max: usize,
    window: Duration,
    hits: Mutex<HashMap<String, VecDeque<Instant>>>,
}

impl Limiter {
    fn new((max, window): (usize, Duration)) -> Limiter {
        Limiter { max, window, hits: Mutex::new(HashMap::new()) }
    }

    fn allow(&self, key: &str, now: Instant) -> bool {
        let mut all = self.hits.lock().unwrap_or_else(PoisonError::into_inner);
        if all.len() > 4096 {
            all.retain(|_, q| q.back().is_some_and(|t| now.duration_since(*t) < self.window));
        }
        let q = all.entry(key.to_string()).or_default();
        while q.front().is_some_and(|t| now.duration_since(*t) >= self.window) {
            q.pop_front();
        }
        if q.len() >= self.max {
            return false;
        }
        q.push_back(now);
        true
    }
}

pub struct Auth {
    file: PathBuf,
    devices: Mutex<Vec<Device>>,
    codes: Mutex<HashMap<String, (Instant, String)>>,
    rate: Limiter,
    pair_rate: Limiter,
}

pub fn hash(token: &str) -> String {
    Sha256::digest(token.as_bytes()).iter().map(|b| format!("{b:02x}")).collect()
}

pub fn bearer(header: Option<&str>) -> Option<&str> {
    let value = header?.trim();
    let (scheme, token) = value.split_once(' ')?;
    (scheme.eq_ignore_ascii_case("bearer") && !token.trim().is_empty()).then(|| token.trim())
}

pub fn clean_name(name: &str) -> String {
    let kept: String = name.chars().filter(|c| c.is_alphanumeric() || " -_.".contains(*c)).take(NAME_MAX).collect();
    let kept = kept.trim();
    if kept.is_empty() { "phone".to_string() } else { kept.to_string() }
}

impl Auth {
    pub fn new(home: &std::path::Path) -> Auth {
        let file = home.join("agents").join(FILE);
        let devices = fs::read_to_string(&file)
            .ok()
            .and_then(|t| serde_json::from_str::<Value>(&t).ok())
            .and_then(|v| {
                v.as_array().map(|a| {
                    a.iter()
                        .filter_map(|d| {
                            Some(Device {
                                name: d["name"].as_str()?.to_string(),
                                hash: d["hash"].as_str()?.to_string(),
                                created: d["created"].as_i64().unwrap_or(0),
                            })
                        })
                        .collect()
                })
            })
            .unwrap_or_default();
        Auth {
            file,
            devices: Mutex::new(devices),
            codes: Mutex::new(HashMap::new()),
            rate: Limiter::new(TOKEN_RATE),
            pair_rate: Limiter::new(PAIR_RATE),
        }
    }

    fn devices(&self) -> MutexGuard<'_, Vec<Device>> {
        self.devices.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn save(&self, list: &[Device]) -> bool {
        let rows: Vec<Value> =
            list.iter().map(|d| json!({"name": d.name, "hash": d.hash, "created": d.created})).collect();
        let Some(dir) = self.file.parent() else { return false };
        fs::create_dir_all(dir).and_then(|()| wisply_core::save(&self.file, Value::Array(rows).to_string())).is_ok()
    }

    pub fn check(&self, header: Option<&str>, now: Instant) -> Verdict {
        let Some(token) = bearer(header) else { return Verdict::Denied };
        let digest = hash(token);
        let name = self.devices().iter().find(|d| same_token(&d.hash, &digest)).map(|d| d.name.clone());
        match name {
            None => Verdict::Denied,
            Some(_) if !self.rate.allow(&digest, now) => Verdict::Limited,
            Some(name) => Verdict::Allowed(name),
        }
    }

    pub fn new_code(&self, name: &str, now: Instant) -> Option<String> {
        let code: String = random_token().ok()?.chars().take(CODE_CHARS).collect();
        let mut codes = self.codes.lock().unwrap_or_else(PoisonError::into_inner);
        codes.retain(|_, (at, _)| now.duration_since(*at) < CODE_TTL);
        if codes.len() >= CODES_MAX {
            return None;
        }
        codes.insert(code.clone(), (now, clean_name(name)));
        Some(code)
    }

    pub fn redeem(
        &self,
        code: &str,
        device: &str,
        peer: &str,
        now: Instant,
        created: i64,
    ) -> Result<(String, String), PairError> {
        if !self.pair_rate.allow(peer, now) {
            return Err(PairError::Limited);
        }
        let entry = self.codes.lock().unwrap_or_else(PoisonError::into_inner).remove(code);
        let Some((issued, wanted)) = entry else { return Err(PairError::Rejected) };
        if now.duration_since(issued) >= CODE_TTL {
            return Err(PairError::Rejected);
        }
        let given = clean_name(device);
        let name = if device.trim().is_empty() { wanted } else { given };
        let token = random_token().map_err(|_| PairError::Failed)?;
        let mut list = self.devices();
        list.retain(|d| d.name != name);
        list.push(Device { name: name.clone(), hash: hash(&token), created });
        if !self.save(&list) {
            list.pop();
            return Err(PairError::Failed);
        }
        Ok((token, name))
    }

    pub fn revoke(&self, name: &str) -> bool {
        let mut list = self.devices();
        let before = list.len();
        list.retain(|d| !d.name.eq_ignore_ascii_case(name));
        let gone = list.len() < before;
        if gone {
            self.save(&list);
        }
        gone
    }

    pub fn list(&self) -> Vec<Device> {
        self.devices().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn auth(tag: &str) -> (Auth, PathBuf) {
        let home = std::env::temp_dir().join(format!("wisply-agents-auth-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        (Auth::new(&home), home)
    }

    #[test]
    fn a_code_works_once_and_makes_a_token_that_works_until_revoked() {
        let (a, home) = auth("flow");
        let now = Instant::now();
        let code = a.new_code("handy", now).unwrap();
        assert_eq!(code.len(), CODE_CHARS);
        let (token, name) = a.redeem(&code, "", "1.1.1.1", now, 5).unwrap();
        assert_eq!(name, "handy");
        assert_eq!(a.redeem(&code, "", "1.1.1.1", now, 5), Err(PairError::Rejected));
        let header = format!("Bearer {token}");
        assert_eq!(a.check(Some(&header), now), Verdict::Allowed("handy".into()));
        assert_eq!(Auth::new(&home).check(Some(&header), now), Verdict::Allowed("handy".into()));
        let saved = fs::read_to_string(home.join("agents").join(FILE)).unwrap();
        assert!(!saved.contains(&token));
        assert!(a.revoke("HANDY"));
        assert_eq!(a.check(Some(&header), now), Verdict::Denied);
        assert!(!a.revoke("handy"));
        let _ = fs::remove_dir_all(&home);
    }

    #[test]
    fn wrong_missing_and_expired_credentials_are_denied() {
        let (a, home) = auth("deny");
        let now = Instant::now();
        for header in [None, Some(""), Some("Bearer"), Some("Bearer nope"), Some("Basic abc"), Some("nope")] {
            assert_eq!(a.check(header, now), Verdict::Denied, "{header:?}");
        }
        let code = a.new_code("p", now).unwrap();
        assert_eq!(a.redeem("0".repeat(CODE_CHARS).as_str(), "x", "2.2.2.2", now, 0), Err(PairError::Rejected));
        assert_eq!(a.redeem(&code, "x", "2.2.2.2", now + CODE_TTL, 0), Err(PairError::Rejected));
        let fresh = a.new_code("p", now).unwrap();
        assert_eq!(
            a.redeem(&fresh, "x", "2.2.2.2", now + CODE_TTL - Duration::from_secs(1), 0).map(|r| r.1),
            Ok("x".into())
        );
        assert!(a.list().len() == 1);
        let _ = fs::remove_dir_all(&home);
    }

    #[test]
    fn a_token_gets_twenty_requests_a_second_and_guessing_codes_is_slowed() {
        let (a, home) = auth("rate");
        let now = Instant::now();
        let code = a.new_code("p", now).unwrap();
        let (token, _) = a.redeem(&code, "p", "3.3.3.3", now, 0).unwrap();
        let header = format!("Bearer {token}");
        let ok = (0..30).filter(|_| matches!(a.check(Some(&header), now), Verdict::Allowed(_))).count();
        assert_eq!(ok, 20);
        assert_eq!(a.check(Some(&header), now), Verdict::Limited);
        assert!(matches!(a.check(Some(&header), now + Duration::from_millis(1001)), Verdict::Allowed(_)));
        for _ in 0..5 {
            assert_eq!(a.redeem("bad", "p", "9.9.9.9", now, 0), Err(PairError::Rejected));
        }
        assert_eq!(a.redeem("bad", "p", "9.9.9.9", now, 0), Err(PairError::Limited));
        assert_eq!(a.redeem("bad", "p", "8.8.8.8", now, 0), Err(PairError::Rejected));
        let _ = fs::remove_dir_all(&home);
    }

    #[test]
    fn only_a_few_codes_are_open_at_once_and_names_are_cleaned() {
        let (a, home) = auth("codes");
        let now = Instant::now();
        let made = (0..8).filter(|_| a.new_code("p", now).is_some()).count();
        assert_eq!(made, CODES_MAX);
        assert!(a.new_code("p", now + CODE_TTL).is_some());
        assert_eq!(clean_name("My <b>Phone</b>/../x"), "My bPhoneb..x");
        assert_eq!(clean_name("  "), "phone");
        assert_eq!(clean_name(&"a".repeat(80)).len(), NAME_MAX);
        assert_eq!(bearer(Some("bearer  abc ")), Some("abc"));
        let _ = fs::remove_dir_all(&home);
    }
}
