use super::*;
use crate::run::Home;

#[test]
fn checkpoints_revert_and_guard_later_changes() {
    let base = std::env::temp_dir().join(format!("wisply-think-safety-{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    let work = base.join("work");
    fs::create_dir_all(&work).unwrap();
    let a = work.join("a.txt");
    fs::write(&a, "one").unwrap();
    let home = Home::new(&base, "s");
    let mut run = Run::create(&home, "safety");
    run.map().insert("paths".into(), json!([work.to_string_lossy()]));
    assert_eq!(backup(&run, &files::walk(&run.paths()))["files"], 1);
    let fresh = work.join("new.txt");
    checkpoint(&run, 1, &[a.clone(), fresh.clone()]).unwrap();
    fs::write(&a, "two").unwrap();
    fs::write(&fresh, "x").unwrap();
    mark_after(&run, 1);
    checkpoint(&run, 2, std::slice::from_ref(&a)).unwrap();
    fs::write(&a, "three").unwrap();
    assert!(revert(&run, 1, false).unwrap_err().contains("a.txt"));
    revert(&run, 2, false).unwrap();
    assert_eq!(fs::read_to_string(&a).unwrap(), "two");
    let (done, _) = revert(&run, 1, false).unwrap();
    assert_eq!(done, ["a.txt back", "new.txt deleted (was new)"]);
    assert_eq!(fs::read_to_string(&a).unwrap(), "one");
    fs::write(&a, "four").unwrap();
    assert_eq!(restore_all(&run).unwrap(), (1, 0));
    assert_eq!(fs::read_to_string(&a).unwrap(), "one");
    let b = work.join("b.txt");
    fs::write(&b, "two").unwrap();
    let e = blind(&mut run, &a, &b).unwrap();
    let after_slot = if e["map"]["1"] == "after" { "1" } else { "2" };
    assert_eq!(blind_result(&mut run, 1, after_slot).unwrap(), "after");
    let _ = fs::remove_dir_all(&base);
}

#[test]
fn one_file_counts_once_however_its_path_is_written() {
    let base = std::env::temp_dir().join(format!("wisply-think-spelling-{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    let work = base.join("work");
    fs::create_dir_all(&work).unwrap();
    let a = work.join("a.txt");
    fs::write(&a, "one").unwrap();
    let mut run = Run::create(&Home::new(&base, "s"), "spelling");
    run.map().insert("paths".into(), json!([files::slashed(&work)]));
    backup(&run, &files::walk(&run.paths()));
    fs::write(&a, "two").unwrap();
    checkpoint(&run, 1, std::slice::from_ref(&a)).unwrap();
    fs::write(&a, "three").unwrap();
    checkpoint(&run, 2, &[PathBuf::from(files::slashed(&a))]).unwrap();
    fs::write(&a, "four").unwrap();
    assert!(revert(&run, 1, false).unwrap_err().contains("a.txt (#2)"));
    assert_eq!(diff(&run, "t").unwrap().files, 1);
    assert_eq!(restore_all(&run).unwrap(), (1, 0));
    assert_eq!(fs::read_to_string(&a).unwrap(), "one");
    let _ = fs::remove_dir_all(&base);
}

#[test]
fn a_lost_backup_is_named_and_never_shown_as_no_change() {
    let base = std::env::temp_dir().join(format!("wisply-think-lost-{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    let work = base.join("work");
    fs::create_dir_all(&work).unwrap();
    let (a, b) = (work.join("a.txt"), work.join("b.txt"));
    fs::write(&a, "one").unwrap();
    fs::write(&b, "one").unwrap();
    let home = Home::new(&base, "s");
    let mut run = Run::create(&home, "lost");
    run.map().insert("paths".into(), json!([work.to_string_lossy()]));
    let saved = backup(&run, &files::walk(&run.paths()));
    assert_eq!((saved["files"].as_u64(), saved["failed"].as_array().map(Vec::len)), (Some(2), Some(0)));
    assert!(lost(&run).is_empty());
    checkpoint(&run, 1, std::slice::from_ref(&b)).unwrap();
    fs::write(&a, "two").unwrap();
    fs::write(&b, "two").unwrap();
    let slot = |dir: &Path| {
        let Value::Array(rows) = read_json(&dir.join("manifest.json")) else { panic!("manifest") };
        let row = rows.iter().find(|e| text(e, "abs").ends_with("a.txt")).expect("a.txt is saved");
        dir.join(text(row, "bak"))
    };
    fs::remove_file(slot(&run.dir.join("backup"))).unwrap();
    assert_eq!(lost(&run), ["a.txt"]);
    let d = diff(&run, "t").unwrap();
    assert_eq!((d.files, d.lost.as_slice()), (1, &["a.txt".to_string()][..]));
    let shown = fs::read_to_string(&d.file).unwrap();
    assert!(shown.contains("# start backup of a.txt is gone"), "{shown}");
    assert!(restore_all(&run).unwrap_err().contains("a.txt"));
    assert_eq!(fs::read_to_string(&b).unwrap(), "two", "a refused restore touches nothing");
    fs::remove_file(cp_dir(&run, 1).join("0.bak")).unwrap();
    assert!(revert(&run, 1, false).unwrap_err().contains("b.txt"));
    assert_eq!(fs::read_to_string(&b).unwrap(), "two");
    let missing = [work.join("never.txt")];
    assert_eq!(backup(&run, &missing)["failed"], json!(["never.txt"]));
    let _ = fs::remove_dir_all(&base);
}

#[test]
fn a_lost_backup_falls_back_to_the_last_commit() {
    let base = std::env::temp_dir().join(format!("wisply-think-commit-{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    let work = base.join("work");
    fs::create_dir_all(&work).unwrap();
    let a = work.join("a.txt");
    fs::write(&a, "one\n").unwrap();
    let git = |args: &[&str]| {
        let done = crate::measure::quiet(&mut Command::new("git"))
            .arg("-C")
            .arg(&work)
            .args(["-c", "user.name=t", "-c", "user.email=t@example.com", "-c", "core.autocrlf=false"])
            .args(args)
            .output()
            .expect("git runs");
        assert!(done.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&done.stderr));
    };
    git(&["init", "-q"]);
    git(&["add", "a.txt"]);
    git(&["commit", "-q", "-m", "start"]);
    let home = Home::new(&base, "s");
    let mut run = Run::create(&home, "commit");
    run.map().insert("paths".into(), json!([a.to_string_lossy()]));
    assert_eq!(backup(&run, std::slice::from_ref(&a))["files"], 1);
    fs::remove_file(run.dir.join("backup").join("0.bak")).unwrap();
    fs::write(&a, "two\n").unwrap();
    let d = diff(&run, "t").unwrap();
    let shown = fs::read_to_string(&d.file).unwrap();
    assert!(shown.contains("compared with the last commit") && shown.contains("+two"), "{shown}");
    assert_eq!((d.files, d.plus, d.minus), (1, 1, 1));
    let _ = fs::remove_dir_all(&base);
}
