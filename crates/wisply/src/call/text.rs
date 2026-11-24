use std::collections::hash_map::RandomState;
use std::env;
use std::fs;
use std::hash::{BuildHasher, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

use wisply_core::local_parts;

use super::cfg;

const KEY_FILE: [&str; 3] = [".config", "watch", ".env"];

pub fn now() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0.0, |d| d.as_secs_f64())
}

pub fn random_hex(len: usize) -> String {
    let mut out = String::new();
    while out.len() < len {
        let mut h = RandomState::new().build_hasher();
        h.write_u128(SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_nanos()));
        h.write_u32(std::process::id());
        out.push_str(&format!("{:016x}", h.finish()));
    }
    out.truncate(len);
    out
}

pub fn encode(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len() * 3);
    for b in raw.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

pub fn clip(raw: &str, max: usize) -> String {
    let flat = raw.split_whitespace().collect::<Vec<_>>().join(" ");
    if flat.chars().count() <= max {
        return flat;
    }
    let cut: String = flat.chars().take(max.saturating_sub(1)).collect();
    format!("{}…", cut.trim_end())
}

pub fn safe(raw: &str, max: usize) -> String {
    crate::daemon::scrub(raw, max)
}

pub fn secret(name: &str) -> Option<String> {
    if let Some(k) = env::var(name).ok().filter(|k| !k.trim().is_empty()) {
        return Some(k.trim().to_string());
    }
    let file = KEY_FILE.iter().fold(wisply_proto::user_dir(), |p, part| p.join(part));
    fs::read_to_string(file).ok()?.lines().find_map(|line| {
        let (k, v) = line.split_once('=')?;
        let v = v.trim().trim_matches(['"', '\'']);
        (k.trim() == name && !v.is_empty()).then(|| v.to_string())
    })
}

pub fn minutes_of_day(epoch: f64) -> u32 {
    let [_, _, _, h, m, _] = local_parts(epoch);
    h * 60 + m
}

pub fn quiet_at(cfg: &serde_json::Map<String, serde_json::Value>, minute: u32) -> bool {
    let (Some(from), Some(to)) = (cfg::clock(&cfg::text(cfg, "quiet_from")), cfg::clock(&cfg::text(cfg, "quiet_to")))
    else {
        return false;
    };
    match from.cmp(&to) {
        std::cmp::Ordering::Equal => false,
        std::cmp::Ordering::Less => minute >= from && minute < to,
        std::cmp::Ordering::Greater => minute >= from || minute < to,
    }
}

pub fn quiet_now(cfg: &serde_json::Map<String, serde_json::Value>) -> bool {
    quiet_at(cfg, minutes_of_day(now()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn quiet(from: &str, to: &str) -> serde_json::Map<String, serde_json::Value> {
        let mut c = cfg::defaults();
        c.insert("quiet_from".into(), json!(from));
        c.insert("quiet_to".into(), json!(to));
        c
    }

    #[test]
    fn quiet_hours_wrap_over_midnight() {
        let c = quiet("23:00", "07:00");
        assert!(quiet_at(&c, 23 * 60 + 30));
        assert!(quiet_at(&c, 3 * 60));
        assert!(!quiet_at(&c, 7 * 60));
        assert!(!quiet_at(&c, 12 * 60));
        let day = quiet("13:00", "14:00");
        assert!(quiet_at(&day, 13 * 60 + 5) && !quiet_at(&day, 14 * 60));
        assert!(!quiet_at(&quiet("08:00", "08:00"), 8 * 60));
    }

    #[test]
    fn encoding_clipping_and_ids() {
        assert_eq!(encode("Hallo Welt, ä&="), "Hallo%20Welt%2C%20%C3%A4%26%3D");
        assert_eq!(clip("a  b\nc", 10), "a b c");
        assert_eq!(clip("abcdefghij", 5), "abcd…");
        assert_eq!(random_hex(12).len(), 12);
        assert_ne!(random_hex(12), random_hex(12));
    }
}
