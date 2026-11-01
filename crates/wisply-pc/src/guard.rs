use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;

const NEVER_PARTS: [&str; 15] = [
    ".minecraft",
    "minecraft",
    "modrinthapp",
    "modrinth app",
    ".claude",
    "vault",
    ".ssh",
    ".gnupg",
    "secrets",
    "steamapps",
    "epic games",
    "games",
    "gog galaxy",
    "obsidian",
    ".aws",
];
const USER_FOLDERS: [&str; 12] = [
    "documents",
    "pictures",
    "downloads",
    "desktop",
    "videos",
    "music",
    "dokumente",
    "bilder",
    "musik",
    "onedrive",
    "iCloud Drive",
    "icloud",
];
const SECRET_NAMES: [&str; 12] = [
    ".env",
    "servers.json",
    "id_rsa",
    "id_ed25519",
    "credentials",
    "credentials.json",
    "secrets.json",
    ".npmrc",
    ".netrc",
    "tokens.json",
    "auth.json",
    "token",
];
const SECRET_EXT: [&str; 6] = ["pem", "key", "pfx", "p12", "kdbx", "jks"];
const SCAN_DEPTH: usize = 3;
const SCAN_LIMIT: usize = 5000;

pub fn norm(path: &Path) -> String {
    let text = path.to_string_lossy().replace('\\', "/").to_lowercase();
    let text = text.strip_prefix("//?/").unwrap_or(&text);
    text.trim_end_matches('/').to_string()
}

pub fn inside(path: &str, root: &str) -> bool {
    !root.is_empty() && (path == root || path.strip_prefix(root).is_some_and(|rest| rest.starts_with('/')))
}

pub struct Guard {
    home: String,
    wisply: String,
    extra: Vec<String>,
}

pub fn secret_name(name: &str) -> bool {
    let n = name.to_lowercase();
    SECRET_NAMES.contains(&n.as_str())
        || n.starts_with(".env.")
        || n.rsplit_once('.').is_some_and(|(_, ext)| SECRET_EXT.contains(&ext))
}

impl Guard {
    pub fn new(user_home: &Path, wisply_home: &Path, cfg: &Config) -> Guard {
        Guard {
            home: norm(user_home),
            wisply: norm(wisply_home),
            extra: cfg.protect_paths.iter().map(|p| norm(Path::new(p))).collect(),
        }
    }

    pub fn blocked(&self, path: &Path) -> Option<&'static str> {
        let p = norm(path);
        if p.is_empty() || p == "/" || p.len() <= 3 && p.ends_with(':') || p.len() == 3 && p.ends_with(":/") {
            return Some("drive root");
        }
        if self.extra.iter().any(|e| inside(&p, e)) {
            return Some("protected by config");
        }
        if p.contains("/build/libs") || p.ends_with("/build/libs") {
            return Some("build/libs holds finished jars");
        }
        if inside(&p, &format!("{}/memory", self.wisply)) {
            return Some("wisply memory");
        }
        if let Some(rest) = p.strip_prefix(&format!("{}/", self.home))
            && let Some(first) = rest.split('/').next()
            && USER_FOLDERS.iter().any(|u| u.to_lowercase() == first)
        {
            return Some("your documents, pictures or downloads");
        }
        for part in p.split('/') {
            if NEVER_PARTS.contains(&part) {
                return Some("games, Minecraft or secrets folder");
            }
            if secret_name(part) {
                return Some("looks like a secret");
            }
        }
        None
    }

    pub fn holds_secret(&self, dir: &Path) -> bool {
        let mut seen = 0;
        scan(dir, 0, &mut seen)
    }
}

fn scan(dir: &Path, depth: usize, seen: &mut usize) -> bool {
    let Ok(entries) = fs::read_dir(dir) else { return false };
    for e in entries.flatten() {
        *seen += 1;
        if *seen > SCAN_LIMIT {
            return true;
        }
        let name = e.file_name().to_string_lossy().into_owned();
        if secret_name(&name) {
            return true;
        }
        let is_dir = e.file_type().is_ok_and(|t| t.is_dir());
        if is_dir && depth + 1 < SCAN_DEPTH && scan(&e.path(), depth + 1, seen) {
            return true;
        }
    }
    false
}

pub fn path_of(base: &Path, parts: &[&str]) -> PathBuf {
    parts.iter().fold(base.to_path_buf(), |p, part| p.join(part))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn guard() -> Guard {
        let cfg = Config { protect_paths: vec!["D:/Keep Me".into()], ..Config::default() };
        Guard::new(Path::new("C:/Users/Ann"), Path::new("C:/Users/Ann/.wisply"), &cfg)
    }

    #[test]
    fn what_may_never_be_deleted_is_blocked() {
        let g = guard();
        let blocked = [
            "C:/Users/Ann/AppData/Roaming/.minecraft/mods",
            "C:\\Users\\Ann\\AppData\\Roaming\\ModrinthApp\\profiles",
            "E:/plugins/Hugo/build/libs",
            "E:/plugins/Hugo/build/libs/Hugo-1.0.jar",
            "C:/Users/Ann/.claude/projects",
            "C:/Users/Ann/.wisply/memory/store.db",
            "C:/Users/Ann/Documents/thesis",
            "C:/Users/Ann/Downloads/setup.exe",
            "C:/Users/Ann/Bilder/urlaub",
            "C:/Program Files (x86)/Steam/steamapps/common/Hades",
            "D:/Games/Foo",
            "E:/Wisply/vault/Sessions",
            "C:/Users/Ann/proj/.env",
            "C:/Users/Ann/proj/servers.json",
            "C:/Users/Ann/proj/cert.PEM",
            "D:/Keep Me/target",
            "D:/keep me",
            "C:/",
            "C:",
            "/",
        ];
        for p in blocked {
            assert!(g.blocked(Path::new(p)).is_some(), "{p} should be blocked");
        }
    }

    #[test]
    fn ordinary_junk_is_allowed() {
        let g = guard();
        for p in [
            "C:/Users/Ann/AppData/Local/Temp/abc",
            "C:/Users/Ann/proj/target",
            "C:/Users/Ann/proj/node_modules",
            "E:/plugins/Hugo/build/classes",
            "C:/Users/Ann/.wisply/out/old.json",
            "C:/Users/Ann/agents-work/run.log",
            "D:/keep me too/target",
        ] {
            assert_eq!(g.blocked(Path::new(p)), None, "{p}");
        }
    }

    #[test]
    fn secrets_are_found_by_name_and_in_folders() {
        assert!(
            secret_name(".env")
                && secret_name(".ENV.local")
                && secret_name("server.key")
                && secret_name("servers.json")
        );
        assert!(!secret_name("main.rs") && !secret_name("keyboard.txt") && !secret_name("monkey"));
        let dir = std::env::temp_dir().join(format!("wpc-guard-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("a/b")).unwrap();
        let g = guard();
        assert!(!g.holds_secret(&dir));
        fs::write(dir.join("a/b/.env"), "X=1").unwrap();
        assert!(g.holds_secret(&dir));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn inside_matches_whole_path_parts_only() {
        assert!(inside("d:/keep me/x", "d:/keep me") && inside("d:/keep me", "d:/keep me"));
        assert!(!inside("d:/keep meme", "d:/keep me") && !inside("d:/x", ""));
    }
}
