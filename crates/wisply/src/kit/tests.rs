use super::*;

fn home(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("wisply-kit-{}-{tag}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn words(line: &[&str]) -> Vec<String> {
    line.iter().map(|w| (*w).to_string()).collect()
}

fn say(home: &Path, line: &[&str]) -> String {
    match command(home, &words(line)) {
        Ok(Done::Said(text)) => text,
        Ok(Done::Ran(code)) => format!("ran {code}"),
        Err(fail) => fail.text(),
    }
}

fn argv(home: &Path, line: &[&str]) -> Result<Vec<String>, String> {
    plan(home, &words(line)).map(|(argv, _)| argv).map_err(|fail| fail.text())
}

#[test]
fn a_tool_is_saved_listed_filled_replaced_and_removed() {
    let home = home("life");
    let script = home.join("count.py");
    fs::write(&script, "import sys\nprint(len(open(sys.argv[1]).read().splitlines()))\n").unwrap();
    let script = script.to_string_lossy().into_owned();
    assert!(say(&home, &[]).starts_with("no tools of your own yet"));
    let made = say(
        &home,
        &["add", "count-lines", "Counts the lines of a file", "--file", &script, "--", "python", "{file}", "{1}"],
    );
    assert_eq!(
        made,
        "ok: tool 'count-lines' saved for this user, every session can run it: wisply kit run count-lines [args]."
    );
    let kept = root(&home).join("count-lines").join("count.py").to_string_lossy().into_owned();
    assert_eq!(argv(&home, &["count-lines", "a.txt"]).unwrap(), ["python", kept.as_str(), "a.txt"]);
    let short = argv(&home, &["count-lines"]).unwrap_err();
    assert!(short.contains("needs 1 argument(s) and got 0: Counts the lines of a file"), "{short}");
    assert!(say(&home, &["show", "count-lines"]).contains("script: "));
    assert!(say(&home, &["add", "hello", "Says hello", "--", "echo", "hello"]).starts_with("ok: "));
    assert_eq!(argv(&home, &["hello", "x", "y"]).unwrap(), ["echo", "hello", "x", "y"]);
    assert!(
        say(&home, &["add", "wrap", "Puts brackets around words", "--", "echo", "[", "{all}", "{2}!", "]"])
            .starts_with("ok: ")
    );
    assert_eq!(argv(&home, &["wrap", "a", "b"]).unwrap(), ["echo", "[", "a", "b", "b!", "]"]);
    let list = say(&home, &["list"]);
    assert!(list.starts_with("3 tool(s) of your own") && list.contains("  hello - Says hello | echo hello"), "{list}");
    let twice = say(&home, &["add", "count-lines", "Counts words", "--", "wc", "-w"]);
    assert!(twice.starts_with("err: a tool named 'count-lines' is already there"), "{twice}");
    assert!(say(&home, &["add", "count-lines", "Counts words", "--replace", "--", "wc", "-w"]).starts_with("ok: "));
    assert_eq!(argv(&home, &["count-lines", "a.txt"]).unwrap(), ["wc", "-w", "a.txt"]);
    assert!(!Path::new(&kept).exists());
    assert_eq!(say(&home, &["rm", "count-lines"]), "ok: tool 'count-lines' removed");
    assert_eq!(say(&home, &["run", "count-lines"]), "err: no tool named 'count-lines' -> try: wisply kit");
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn a_risky_tool_waits_for_the_user_and_an_agreed_one_runs_freely() {
    let home = home("risk");
    let asked = say(&home, &["add", "clean", "Empties the build folder", "--", "rm", "-rf", "build"]);
    assert_eq!(
        asked,
        "err: the tool 'clean' deletes or overwrites files ('rm'), so the user decides: nothing was done, ask them \
         first -> try: the same call with --yes once they agreed"
    );
    assert!(!root(&home).join("clean").exists());
    let script = home.join("wipe.py");
    fs::write(&script, "import shutil\nshutil.rmtree('build')\n").unwrap();
    let script = script.to_string_lossy().into_owned();
    let hidden = say(&home, &["add", "wipe", "Empties the build folder", "--file", &script, "--", "python", "{file}"]);
    assert!(hidden.contains("deletes or overwrites files ('rmtree')"), "{hidden}");
    let agreed = say(&home, &["add", "clean", "Empties the build folder", "--yes", "--", "rm", "-rf", "build"]);
    assert!(agreed.ends_with("The user agreed that it deletes or overwrites files."), "{agreed}");
    assert_eq!(argv(&home, &["clean"]).unwrap(), ["rm", "-rf", "build"]);
    assert!(say(&home, &["show", "clean"]).ends_with("the user agreed to what it does"));
    assert!(say(&home, &["add", "shell", "Runs one shell line", "--", "sh", "-c", "{1}"]).starts_with("ok: "));
    assert_eq!(argv(&home, &["shell", "ls"]).unwrap(), ["sh", "-c", "ls"]);
    assert_eq!(
        argv(&home, &["shell", "git push"]).unwrap_err(),
        "err: 'shell' with these arguments touches money, accounts, keys or uploads ('git push'), so the user decides: \
         nothing was done, ask them first -> try: wisply kit run shell --yes git push once they agreed"
    );
    assert_eq!(argv(&home, &["shell", "--yes", "git push"]).unwrap(), ["sh", "-c", "git push"]);
    let _ = fs::remove_dir_all(&home);
}

#[test]
fn wrong_calls_say_what_is_missing() {
    let home = home("wrong");
    for name in ["Bad Name", "-x", "", "a/b", ".."] {
        assert!(
            say(&home, &["add", name, "Does things", "--", "echo"]).starts_with("err: a tool name is a-z"),
            "{name}"
        );
    }
    assert!(say(&home, &["add", "x"]).starts_with("err: say what 'x' does in one sentence"));
    assert!(say(&home, &["add", "x", "Does things"]).starts_with("err: after -- comes the program 'x' runs"));
    assert!(say(&home, &["add", "x", "Does things", "--", "python", "{file}"]).starts_with("err: {file} stands for"));
    assert!(
        say(&home, &["add", "x", "Does things", "--file", "no-such-file.py", "--", "python", "{file}"])
            .contains("is not a file under 1 MB")
    );
    assert!(
        say(&home, &["add", "x", "Does things", "--force", "--", "echo"])
            .starts_with("err: kit add does not know --force")
    );
    assert_eq!(say(&home, &["bake"]), "err: kit does not know 'bake' -> try: wisply kit | add | run | show | rm");
    assert!(say(&home, &["add", "ghost", "Starts nothing", "--", "wisply-no-such-program-here"]).starts_with("ok: "));
    assert!(say(&home, &["run", "ghost"]).starts_with("err: cannot start 'wisply-no-such-program-here'"));
    assert!(!root(&home).join("x").exists());
    let _ = fs::remove_dir_all(&home);
}
