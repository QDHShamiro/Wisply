use super::*;
use std::fs::File;
use std::time::{Duration, SystemTime};

fn set_mtime(path: &Path, when: SystemTime) {
    let mut opts = File::options();
    opts.read(true);
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        opts.access_mode(0x100).custom_flags(0x0200_0000);
    }
    if let Ok(h) = opts.open(path) {
        let _ = h.set_modified(when);
    }
}

struct Lab {
    root: PathBuf,
}

impl Lab {
    fn new(tag: &str) -> Lab {
        let root = std::env::temp_dir().join(format!("wpc-clean-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        Lab { root }
    }

    fn file(&self, rel: &str, bytes: usize) -> PathBuf {
        let p = self.root.join(rel);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::write(&p, vec![1u8; bytes]).unwrap();
        p
    }

    fn age(&self, rel: &str, days: u64) {
        let p = self.root.join(rel);
        let when = SystemTime::now() - Duration::from_secs(days * DAY);
        fn all(p: &Path, out: &mut Vec<PathBuf>) {
            out.push(p.to_path_buf());
            for c in children(p) {
                all(&c, out);
            }
        }
        let mut list = Vec::new();
        all(&p, &mut list);
        for f in list.iter().rev() {
            set_mtime(f, when);
        }
    }

    fn env(&self) -> Env {
        Env {
            user: self.root.join("user"),
            wisply: self.root.join("user/.wisply"),
            temps: vec![self.root.join("temp")],
            caches: vec![(self.root.join("caches"), Kind::Dump)],
            roots: vec![self.root.join("user")],
            logs: vec![self.root.join("user/.wisply/out"), self.root.join("user/agents-work")],
            packages: vec![(self.root.join("user/.cargo/registry/cache"), "cargo")],
            now: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}

impl Drop for Lab {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn names(items: &[Item], root: &Path) -> Vec<String> {
    let r = norm(root);
    items.iter().map(|i| norm(&i.path).replacen(&format!("{r}/"), "", 1)).collect()
}

fn cfg() -> Config {
    Config { stale_days: 30, ..Config::default() }
}

#[test]
fn old_temp_files_go_and_fresh_ones_stay() {
    let lab = Lab::new("temp");
    lab.file("temp/old.tmp", 10);
    lab.file("temp/fresh.tmp", 10);
    lab.file("temp/olddir/x.bin", 20);
    lab.age("temp/old.tmp", 3);
    lab.age("temp/olddir", 3);
    let items = plan(&lab.env(), &cfg(), &[]);
    assert_eq!(names(&items, &lab.root), ["temp/old.tmp", "temp/olddir"]);
    assert_eq!(items.iter().map(|i| i.bytes).sum::<u64>(), 30);
}

#[test]
fn stale_projects_lose_their_build_junk_but_live_and_protected_ones_do_not() {
    let lab = Lab::new("build");
    lab.file("user/old/Cargo.toml", 5);
    lab.file("user/old/target/debug/a.bin", 100);
    lab.file("user/new/Cargo.toml", 5);
    lab.file("user/new/target/a.bin", 100);
    lab.file("user/mod/build.gradle", 5);
    lab.file("user/mod/build/libs/mod.jar", 100);
    lab.file("user/mod/build/classes/A.class", 40);
    lab.file("user/web/package.json", 5);
    lab.file("user/web/node_modules/p/.env", 5);
    lab.file("user/web/node_modules/p/i.js", 50);
    lab.file("user/Documents/proj/Cargo.toml", 5);
    lab.file("user/Documents/proj/target/x", 5);
    lab.file("user/AppData/Roaming/.minecraft/pack/build.gradle", 5);
    lab.file("user/AppData/Roaming/.minecraft/pack/build/x", 5);
    for p in ["user/old", "user/mod", "user/web", "user/Documents/proj", "user/AppData/Roaming/.minecraft/pack"] {
        lab.age(p, 60);
    }
    let items = plan(&lab.env(), &cfg(), &[]);
    let got = names(&items, &lab.root);
    assert_eq!(got, ["user/mod/build/classes", "user/old/target"], "{got:?}");
    assert!(lab.root.join("user/mod/build/libs/mod.jar").exists());
}

#[test]
fn a_project_with_a_running_program_inside_is_left_alone() {
    let lab = Lab::new("run");
    lab.file("user/old/Cargo.toml", 5);
    lab.file("user/old/target/debug/app.exe", 100);
    lab.age("user/old", 60);
    let exe = lab.root.join("user/old/target/debug/app.exe").to_string_lossy().into_owned();
    let running = Proc { pid: 1, name: "app.exe".into(), exe, ..Proc::default() };
    assert!(plan(&lab.env(), &cfg(), &[running]).is_empty());
    assert_eq!(plan(&lab.env(), &cfg(), &[]).len(), 1);
}

#[test]
fn old_logs_go_new_logs_and_other_files_stay() {
    let lab = Lab::new("logs");
    lab.file("user/agents-work/a.log", 10);
    lab.file("user/agents-work/b.json", 10);
    lab.file("user/agents-work/c.log", 10);
    lab.file("user/agents-work/keep.txt", 10);
    lab.file("user/.wisply/out/o1.txt", 10);
    lab.file("user/.wisply/memory/m.db", 10);
    for f in [
        "user/agents-work/a.log",
        "user/agents-work/b.json",
        "user/agents-work/keep.txt",
        "user/.wisply/out/o1.txt",
        "user/.wisply/memory/m.db",
    ] {
        lab.age(f, 20);
    }
    let got = names(&plan(&lab.env(), &cfg(), &[]), &lab.root);
    assert_eq!(got, ["user/.wisply/out/o1.txt", "user/agents-work/a.log", "user/agents-work/b.json"]);
}

#[test]
fn package_caches_wait_while_their_tool_runs() {
    let lab = Lab::new("pkg");
    lab.file("user/.cargo/registry/cache/x/crate.crate", 100);
    let cargo = Proc { pid: 2, name: "cargo.exe".into(), ..Proc::default() };
    assert!(plan(&lab.env(), &cfg(), &[cargo]).is_empty());
    let items = plan(&lab.env(), &cfg(), &[]);
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].kind, Kind::Package);
}

#[test]
fn executing_deletes_for_real_logs_it_and_refuses_protected_paths() {
    let lab = Lab::new("exec");
    lab.file("temp/old.tmp", 1000);
    lab.file("temp/olddir/x.bin", 2000);
    lab.file("user/Documents/precious.txt", 50);
    lab.age("temp/old.tmp", 3);
    lab.age("temp/olddir", 3);
    let env = lab.env();
    let mut items = plan(&env, &cfg(), &[]);
    items.push(Item { path: lab.root.join("user/Documents/precious.txt"), bytes: 50, kind: Kind::Temp, tool: None });
    let log = Log::new(&lab.root.join("pclog"));
    let out = execute(&items, &env, &cfg(), &[], &log);
    assert_eq!(out, Outcome { freed: 3000, removed: 2, skipped: 1 });
    assert!(!lab.root.join("temp/old.tmp").exists() && !lab.root.join("temp/olddir").exists());
    assert!(lab.root.join("user/Documents/precious.txt").exists());
    let shown = log.show(10);
    assert_eq!(shown.lines().count(), 2, "{shown}");
    assert!(shown.contains("deleted temp files") && shown.contains("old.tmp") && shown.contains("(1000 B)"), "{shown}");
}

#[test]
fn dry_output_lists_totals_and_the_biggest_items() {
    let items = vec![
        Item { path: "a/big".into(), bytes: 3 << 30, kind: Kind::Build, tool: None },
        Item { path: "a/small".into(), bytes: 5 << 20, kind: Kind::Temp, tool: None },
    ];
    let text = render(&items, true);
    assert!(text.starts_with("pc clean --dry | would free 3.0 GB in 2 items"), "{text}");
    assert!(text.contains("old build folders | 1 items | 3.0 GB") && text.contains("  3.0 GB | a/big"), "{text}");
    assert_eq!(render(&[], false), "pc clean | nothing to clean up");
}

#[test]
fn tool_folders_in_hidden_dirs_and_node_modules_without_a_lockfile_are_never_listed() {
    let lab = Lab::new("tools");
    lab.file("user/.vscode/extensions/x/package.json", 5);
    lab.file("user/.vscode/extensions/x/node_modules/a/i.js", 50);
    lab.file("user/nolock/package.json", 5);
    lab.file("user/nolock/node_modules/a/i.js", 50);
    lab.file("user/locked/package.json", 5);
    lab.file("user/locked/package-lock.json", 5);
    lab.file("user/locked/node_modules/a/i.js", 50);
    for p in ["user/.vscode", "user/nolock", "user/locked"] {
        lab.age(p, 60);
    }
    let got = names(&plan(&lab.env(), &cfg(), &[]), &lab.root);
    assert_eq!(got, ["user/locked/node_modules"], "{got:?}");
}
