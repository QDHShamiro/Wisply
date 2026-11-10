use super::*;

fn room(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("wisply-peek-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn args(dir: &Path, names: &[&str], more: &[&str]) -> Vec<String> {
    names
        .iter()
        .map(|n| dir.join(n).to_string_lossy().replace('\\', "/"))
        .chain(more.iter().map(|m| (*m).to_string()))
        .collect()
}

fn code(fns: usize) -> String {
    (0..fns).map(|i| format!("pub fn step_{i}(input: u32) -> u32 {{\n    input + {i}\n}}\n\n")).collect()
}

#[test]
fn many_files_come_back_as_one_table_with_advice() {
    let dir = room("table");
    fs::write(dir.join("small.rs"), code(3)).unwrap();
    fs::write(dir.join("middle.rs"), code(400)).unwrap();
    fs::write(dir.join("huge.log"), "one line of a log that nobody reads\n".repeat(20_000)).unwrap();
    fs::write(dir.join("Cargo.lock"), "[[package]]\n".repeat(50)).unwrap();
    fs::write(dir.join("plugin.jar"), [80u8, 75, 3, 4, 0, 0]).unwrap();
    fs::write(dir.join("data.raw"), [1u8, 0, 2, 0]).unwrap();
    let names = ["small.rs", "middle.rs", "huge.log", "Cargo.lock", "plugin.jar", "data.raw", "missing.rs"];
    let out = peek(&args(&dir, &names, &[])).unwrap();
    let rows: Vec<&str> = out.lines().collect();
    assert_eq!(rows.len(), 8, "{out}");
    assert!(rows[0].starts_with("7 files, ~") && rows[0].contains("5 to leave alone"), "{out}");
    assert!(rows[1].starts_with("whole     12 L") && rows[1].ends_with("small.rs"), "{out}");
    assert!(rows[2].starts_with("parts   1600 L") && rows[2].ends_with("middle.rs"), "{out}");
    assert!(
        rows[3].starts_with("skip   20000 L") && rows[3].ends_with("too big to read, search it with grep"),
        "{out}"
    );
    assert!(rows[4].starts_with("skip ") && rows[4].ends_with("Cargo.lock - a lock file"), "{out}");
    assert!(rows[5].ends_with("plugin.jar - a compiled or packed file, its file list: unzip -l <file>"), "{out}");
    assert!(rows[6].ends_with("data.raw - binary data"), "{out}");
    assert!(rows[7].starts_with("gone ") && rows[7].ends_with("missing.rs - no such file"), "{out}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn head_and_outline_show_only_for_readable_files() {
    let dir = room("extras");
    fs::write(dir.join("a.rs"), code(60)).unwrap();
    fs::write(dir.join("yarn.lock"), "left-pad@1.3.0:\n".repeat(10)).unwrap();
    let out = peek(&args(&dir, &["a.rs", "yarn.lock"], &["--head", "2", "--outline"])).unwrap();
    assert!(out.contains("\n    L1: pub fn step_0(input: u32) -> u32 {\n    L2:     input + 0\n"), "{out}");
    assert!(out.contains("\n    L1-L237 60x, every 4 lines: pub fn step_0(input: u32) -> u32 {"), "{out}");
    assert!(out.lines().last().unwrap().ends_with("yarn.lock - a lock file"), "{out}");
    let capped = peek(&args(&dir, &["a.rs"], &["--head=900"])).unwrap();
    assert_eq!(capped.lines().count(), 2 + HEAD_MAX, "{capped}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_folder_lists_its_files_and_marks_build_output() {
    let dir = room("folder");
    for sub in ["src", "target/debug", "node_modules/left"] {
        fs::create_dir_all(dir.join(sub)).unwrap();
    }
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"x\"\n").unwrap();
    fs::write(dir.join("build.rs"), code(2)).unwrap();
    let out = peek(&args(&dir, &[""], &[])).unwrap();
    let rows: Vec<&str> = out.lines().collect();
    assert!(rows[0].starts_with("2 files, ~"), "{out}");
    assert!(rows[1].starts_with("folder ") && rows[1].ends_with(": 2 files, 3 folders"), "{out}");
    assert_eq!(
        rows[2..5],
        ["  skip  node_modules/ - inside a dependency folder", "  dir   src/", "  skip  target/ - build output"]
    );
    assert!(rows[5].starts_with("  whole") && rows[5].ends_with("Cargo.toml"), "{out}");
    assert!(rows[6].starts_with("  whole      8 L") && rows[6].ends_with("build.rs"), "{out}");
    for i in 0..FOLDER_ROWS + 5 {
        fs::write(dir.join("src").join(format!("f{i:03}.rs")), "fn a() {}\n").unwrap();
    }
    let many = peek(&args(&dir, &["src"], &[])).unwrap();
    assert_eq!(many.lines().count(), 2 + FOLDER_ROWS + 1, "{many}");
    assert_eq!(many.lines().last().unwrap(), "  ... +5 more, name them or peek a subfolder");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn wrong_calls_say_how_to_call_it() {
    let fail = |a: &[&str]| peek(&a.iter().map(|s| (*s).to_string()).collect::<Vec<_>>()).err().map(|f| f.text());
    assert!(fail(&[]).is_some_and(|t| t.starts_with("err: peek needs at least one file or folder")));
    assert!(fail(&["x", "--deep"]).is_some_and(|t| t.starts_with("err: peek does not know --deep")));
    assert!(fail(&["x", "--head", "many"]).is_some_and(|t| t.starts_with("err: --head needs a number")));
}
