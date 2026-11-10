use std::path::Path;

const LOCKS: &[&str] = &[
    "cargo.lock",
    "package-lock.json",
    "npm-shrinkwrap.json",
    "yarn.lock",
    "pnpm-lock.yaml",
    "bun.lock",
    "deno.lock",
    "poetry.lock",
    "pipfile.lock",
    "uv.lock",
    "pdm.lock",
    "gemfile.lock",
    "composer.lock",
    "go.sum",
    "flake.lock",
    "mix.lock",
    "pubspec.lock",
    "podfile.lock",
    "packages.lock.json",
    "gradle.lockfile",
];
const MINIFIED: &[&str] =
    &[".min.js", ".min.mjs", ".min.cjs", ".min.css", ".js.map", ".mjs.map", ".cjs.map", ".css.map"];
const BINARY: &[&str] = &[
    "jar", "class", "war", "ear", "exe", "dll", "so", "dylib", "o", "obj", "a", "lib", "pdb", "rlib", "rmeta", "pyc",
    "pyo", "wasm", "node", "bin", "zip", "7z", "gz", "tgz", "bz2", "xz", "zst", "rar", "tar", "iso", "dmg", "msi",
    "apk", "aab", "dex", "nupkg", "whl", "lockb", "db", "sqlite", "sqlite3", "mca", "nbt", "schem",
];
const MEDIA: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "webp", "bmp", "ico", "mp3", "wav", "ogg", "flac", "mp4", "mkv", "mov", "avi", "webm",
    "ttf", "otf", "woff", "woff2",
];
const PROJECT: &[&str] = &[
    "Cargo.toml",
    "pom.xml",
    "build.gradle",
    "build.gradle.kts",
    "settings.gradle",
    "settings.gradle.kts",
    "package.json",
    "CMakeLists.txt",
    "pyproject.toml",
    "setup.py",
    "Makefile",
    "meson.build",
    "build.sbt",
    "build.zig",
];
const CACHE_TAG: &str = "CACHEDIR.TAG";
const ZIPPED: &[&str] = &["jar", "war", "ear", "zip", "apk", "aab", "nupkg", "whl"];
const TARRED: &[&str] = &["tar", "tgz", "gz", "bz2", "xz", "zst"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Junk {
    Binary,
    Media,
    GitData,
    Lock,
    Minified,
    Deps,
    Built,
}

impl Junk {
    pub fn what(self) -> &'static str {
        match self {
            Junk::Binary => "a compiled or packed file",
            Junk::Media => "an image, sound, video or font",
            Junk::GitData => "git's own data",
            Junk::Lock => "a lock file",
            Junk::Minified => "minified or generated",
            Junk::Deps => "inside a dependency folder",
            Junk::Built => "build output",
        }
    }

    pub fn text(self) -> bool {
        !matches!(self, Junk::Binary | Junk::Media | Junk::GitData)
    }
}

fn named(name: &str) -> Option<Junk> {
    if LOCKS.contains(&name) {
        return Some(Junk::Lock);
    }
    if MINIFIED.iter().any(|end| name.ends_with(end)) {
        return Some(Junk::Minified);
    }
    let (_, ext) = name.rsplit_once('.')?;
    if BINARY.contains(&ext) {
        Some(Junk::Binary)
    } else if MEDIA.contains(&ext) {
        Some(Junk::Media)
    } else {
        None
    }
}

pub fn folder(parent: &Path, name: &str) -> Option<Junk> {
    match name.to_ascii_lowercase().as_str() {
        "node_modules" => Some(Junk::Deps),
        ".gradle" | "__pycache__" => Some(Junk::Built),
        "target" | "build" => {
            let made = PROJECT.iter().any(|p| parent.join(p).is_file()) || parent.join(name).join(CACHE_TAG).is_file();
            made.then_some(Junk::Built)
        }
        _ => None,
    }
}

pub fn of(path: &str) -> Option<Junk> {
    let file = Path::new(path);
    let name = file.file_name()?.to_string_lossy().to_ascii_lowercase();
    if let Some(kind) = named(&name) {
        return Some(kind);
    }
    let mut below = String::new();
    let mut dir = file.parent();
    while let Some(d) = dir {
        let Some(n) = d.file_name().map(|n| n.to_string_lossy().into_owned()) else { break };
        if n.eq_ignore_ascii_case(".git") && below == "objects" {
            return Some(Junk::GitData);
        }
        if let Some(kind) = d.parent().and_then(|parent| folder(parent, &n)) {
            return Some(kind);
        }
        below = n.to_ascii_lowercase();
        dir = d.parent();
    }
    None
}

pub fn inside(path: &str) -> Option<&'static str> {
    let low = path.to_ascii_lowercase();
    let (_, ext) = low.rsplit_once('.')?;
    if ZIPPED.contains(&ext) {
        Some("unzip -l")
    } else if TARRED.contains(&ext) {
        Some("tar -tf")
    } else {
        None
    }
}

pub fn size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    match bytes {
        0..KB => format!("{bytes} B"),
        KB..MB => format!("{:.1} KB", bytes as f64 / KB as f64),
        _ => format!("{:.1} MB", bytes as f64 / MB as f64),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn names_alone_tell_locks_minified_and_binary_files() {
        let cases = [
            ("C:/p/Cargo.lock", Some(Junk::Lock)),
            ("/p/web/package-lock.json", Some(Junk::Lock)),
            ("/p/go.sum", Some(Junk::Lock)),
            ("/p/static/app.min.js", Some(Junk::Minified)),
            ("/p/static/app.js.map", Some(Junk::Minified)),
            ("/p/libs/Plugin.JAR", Some(Junk::Binary)),
            ("/p/Main.class", Some(Junk::Binary)),
            ("/p/logo.png", Some(Junk::Media)),
            ("/p/.git/objects/ab/cdef", Some(Junk::GitData)),
            ("/p/.git/config", None),
            ("/p/src/lock.rs", None),
            ("/p/src/minify.js", None),
            ("/p/README", None),
            ("", None),
        ];
        for (path, want) in cases {
            assert_eq!(of(path), want, "{path}");
        }
        assert!(Junk::Lock.text() && Junk::Built.text() && !Junk::Binary.text() && !Junk::GitData.text());
        assert_eq!(
            (inside("a/Plugin.JAR"), inside("a/src.tar.gz"), inside("a/app.exe")),
            (Some("unzip -l"), Some("tar -tf"), None)
        );
    }

    #[test]
    fn build_folders_count_only_next_to_a_project_file() {
        let dir = std::env::temp_dir().join(format!("wisply-junk-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        for sub in ["app/target/debug", "app/src/target", "tool/build/x", "web/node_modules/left/lib", "cache/target"] {
            fs::create_dir_all(dir.join(sub)).unwrap();
        }
        fs::write(dir.join("app/Cargo.toml"), "[package]").unwrap();
        fs::write(dir.join("cache/target").join(CACHE_TAG), "Signature").unwrap();
        let at = |rel: &str| of(&dir.join(rel).to_string_lossy());
        assert_eq!(at("app/target/debug/build.rs"), Some(Junk::Built));
        assert_eq!(at("app/src/target/mod.rs"), None);
        assert_eq!(at("tool/build/x/index.js"), None);
        assert_eq!(at("cache/target/out.txt"), Some(Junk::Built));
        assert_eq!(at("web/node_modules/left/lib/index.js"), Some(Junk::Deps));
        assert_eq!(folder(&dir.join("app"), "target"), Some(Junk::Built));
        assert_eq!(folder(&dir.join("app/src"), "target"), None);
        assert_eq!(folder(&dir.join("web"), "node_modules"), Some(Junk::Deps));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn sizes_read_like_a_person_writes_them() {
        assert_eq!(
            (size(0), size(1023), size(1536), size(5 * 1024 * 1024)),
            ("0 B".into(), "1023 B".into(), "1.5 KB".into(), "5.0 MB".into())
        );
    }
}
