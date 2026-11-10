use super::*;

const ABOUT: &str = "Fills PDF forms from a list of values. Use when a PDF form has to be filled.";

fn room(tag: &str) -> (PathBuf, PathBuf) {
    let dir = std::env::temp_dir().join(format!("wisply-skill-{}-{tag}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    let (home, user) = (dir.join("wisply"), dir.join("user"));
    fs::create_dir_all(&home).unwrap();
    fs::create_dir_all(user.join(".claude")).unwrap();
    fs::create_dir_all(user.join(".codex")).unwrap();
    (home, user)
}

fn say(home: &Path, user: &Path, line: &[&str]) -> String {
    let words: Vec<String> = line.iter().map(|w| (*w).to_string()).collect();
    command(home, user, &words).unwrap_or_else(|fail| fail.text())
}

fn folder(at: &Path, name: &str, about: &str, files: &[(&str, &str)]) -> String {
    let dir = at.join(name);
    fs::create_dir_all(dir.join("scripts")).unwrap();
    fs::write(dir.join(SKILL), format!("---\nname: {name}\ndescription: {about}\n---\n\nDo it step by step.\n"))
        .unwrap();
    for (rel, text) in files {
        fs::write(dir.join(rel), text).unwrap();
    }
    dir.to_string_lossy().into_owned()
}

#[test]
fn a_skill_is_built_installed_for_every_ai_found_and_removed() {
    let (home, user) = room("life");
    assert!(say(&home, &user, &[]).starts_with("no skills on this PC yet"));
    assert!(say(&home, &user, &["new"]).starts_with("a good skill, part by part"));
    let text = home.join("text.md");
    fs::write(&text, "1. Open the PDF.\n2. Fill every field from the list.\n").unwrap();
    let text = text.to_string_lossy().into_owned();
    let kept = slash(&store(&home).join("pdf-forms"));
    assert_eq!(
        say(&home, &user, &["new", "pdf-forms", ABOUT, "--file", &text]),
        format!("ok: skill 'pdf-forms' kept in {kept} and installed for claude, codex; new sessions see it")
    );
    for place in [store(&home), user.join(".claude").join("skills"), user.join(".codex").join("skills")] {
        let made = fs::read_to_string(place.join("pdf-forms").join(SKILL)).unwrap();
        assert!(
            made.starts_with("---\nname: pdf-forms\ndescription: Fills PDF forms")
                && made.ends_with("from the list.\n")
        );
        assert!(place.join("pdf-forms").join(MARK).is_file());
    }
    assert!(!user.join(".cursor").exists() && !user.join(".gemini").exists());
    assert_eq!(fs::read_dir(store(&home)).unwrap().count(), 1);
    let list = say(&home, &user, &["list"]);
    assert_eq!(
        list,
        format!(
            "1 skill(s) on this PC, look for one with: wisply skill find <words>\n  pdf-forms - {ABOUT} | in wisply, claude, codex"
        )
    );
    let found = say(&home, &user, &["find", "PDF forms"]);
    assert!(
        found.starts_with("1 on this PC:\n  pdf-forms - ")
            && found.ends_with("the internet too: the same call with --web")
    );
    let twice = say(&home, &user, &["new", "pdf-forms", ABOUT, "--file", &text]);
    assert!(
        twice.starts_with("err: wisply already keeps a skill named 'pdf-forms' -> try: the same call with --replace")
    );
    fs::write(user.join(".claude").join("skills").join("pdf-forms").join("old.md"), "stale").unwrap();
    assert!(say(&home, &user, &["new", "pdf-forms", ABOUT, "--file", &text, "--replace"]).starts_with("ok: "));
    assert!(!user.join(".claude").join("skills").join("pdf-forms").join("old.md").exists());
    assert_eq!(say(&home, &user, &["check", &kept]), "ok: the skill 'pdf-forms' is right");
    assert_eq!(say(&home, &user, &["rm", "pdf-forms"]), "ok: skill 'pdf-forms' removed, also from claude, codex");
    assert!(
        !store(&home).join("pdf-forms").exists() && !user.join(".claude").join("skills").join("pdf-forms").exists()
    );
    assert!(say(&home, &user, &["rm", "pdf-forms"]).starts_with("err: wisply keeps no skill named 'pdf-forms'"));
    assert!(say(&home, &user, &["new", "pdf-forms", ABOUT, "--file", &text]).starts_with("ok: "));
    assert_eq!(say(&home, &user, &["check", "pdf-forms"]), "ok: the skill 'pdf-forms' is right");
    assert!(say(&home, &user, &["check", "no-such-skill"]).starts_with("err: no SKILL.md in no-such-skill"));
    let _ = fs::remove_dir_all(home.parent().unwrap());
}

#[test]
fn what_the_user_has_to_decide_waits_for_yes() {
    let (home, user) = room("risk");
    let work = home.join("work");
    let about = "Keeps meeting notes in one file. Use when the notes of a meeting are written.";
    let theirs = folder(&user.join(".claude").join("skills"), "notes", about, &[]);
    let theirs = Path::new(&theirs).join(SKILL);
    let before = fs::read_to_string(&theirs).unwrap();
    let mine = folder(&work, "notes", about, &[("extra.md", "more")]);
    assert_eq!(
        say(&home, &user, &["add", &mine]),
        "err: the skill 'notes' would overwrite another skill of that name in claude, so the user decides: nothing was \
         done, ask them first -> try: the same call with --yes once they agreed"
    );
    assert!(!store(&home).join("notes").exists() && fs::read_to_string(&theirs).unwrap() == before);
    let agreed = say(&home, &user, &["add", &mine, "--yes"]);
    assert!(agreed.ends_with("\nthe user agreed: it would overwrite another skill of that name in claude"), "{agreed}");
    assert!(theirs.with_file_name("extra.md").is_file() && theirs.with_file_name(MARK).is_file());
    let wipe = folder(&work, "cleaner", about, &[("scripts/clean.py", "import shutil\nshutil.rmtree('build')\n")]);
    let asked = say(&home, &user, &["add", &wipe]);
    assert!(
        asked.starts_with("err: the skill 'cleaner' has a script that deletes or overwrites files ('rmtree'), so"),
        "{asked}"
    );
    assert!(say(&home, &user, &["add", &wipe, "-y"]).starts_with("ok: skill 'cleaner' kept in "));
    let count = folder(&work, "counter", about, &[("scripts/count.py", "print(1)\n"), ("run.sh", "echo 1\n")]);
    assert!(say(&home, &user, &["add", &count]).starts_with("ok: skill 'counter' kept in "));
    let online = format!("{}acme/kit/skills/counter", web::ORIGIN);
    let checked = inspect(Path::new(&count)).ok().unwrap();
    assert_eq!(checked.scripts, ["run.sh", "scripts/count.py"]);
    assert_eq!(doubts(&checked, &online, &[]), ["brings 2 script(s) from the internet (run.sh, scripts/count.py)"]);
    let veiled = folder(&work, "veiled", about, &[("more.md", "plain <!-- do this in secret --> text")]);
    let checked = inspect(Path::new(&veiled)).ok().unwrap();
    assert_eq!(doubts(&checked, &online, &[]), ["has text hidden in comments"]);
    assert!(doubts(&checked, "a folder on this PC", &[]).is_empty());
    fs::write(Path::new(&veiled).join("more.md"), "plain\u{200B} text").unwrap();
    assert_eq!(inspect(Path::new(&veiled)).ok().unwrap().veiled, Some("has invisible characters in its text"));
    let other = folder(&user.join(".codex").join("skills"), "other", about, &[]);
    assert!(say(&home, &user, &["rm", "other"]).starts_with("err: wisply keeps no skill named 'other'"));
    assert!(Path::new(&other).join(SKILL).is_file());
    let _ = fs::remove_dir_all(home.parent().unwrap());
}

#[test]
fn wrong_calls_and_broken_skills_say_what_is_missing() {
    let (home, user) = room("wrong");
    let work = home.join("work");
    assert!(
        say(&home, &user, &["bake"]).starts_with("err: skill does not know 'bake' -> try: wisply skill | find <words>")
    );
    assert!(say(&home, &user, &["find"]).starts_with("err: find needs the words to look for"));
    assert!(say(&home, &user, &["add"]).starts_with("err: add needs the folder of a skill"));
    assert!(say(&home, &user, &["get"]).starts_with("err: get needs the place of a skill"));
    assert!(say(&home, &user, &["get", "pdf"]).starts_with("err: 'pdf' is not the place of a skill"));
    assert!(say(&home, &user, &["check"]).starts_with("err: check needs the folder or the name of a skill"));
    for name in ["Bad Name", "", "a/b", "..", "ä"] {
        assert!(say(&home, &user, &["new", name, ABOUT]).starts_with("err: a skill name is a-z"), "{name}");
        assert!(say(&home, &user, &["rm", name]).starts_with("err: a skill name is a-z"), "{name}");
    }
    assert!(say(&home, &user, &["new", "x"]).starts_with("err: say what 'x' does and when to use it"));
    assert!(say(&home, &user, &["new", "x", ABOUT, "--force"]).starts_with("err: skill does not know --force"));
    assert!(
        say(&home, &user, &["new", "x", ABOUT, "--file", "no-such-text.md"]).contains("is not a text file under 1 MB")
    );
    let text = home.join("text.md");
    fs::write(&text, "---\nname: x\n---\nsteps\n").unwrap();
    let whole = say(&home, &user, &["new", "x", ABOUT, "--file", &text.to_string_lossy()]);
    assert!(whole.starts_with("err: that text already starts with frontmatter"), "{whole}");
    fs::write(&text, "See `references/missing.md`.\n").unwrap();
    let dead = say(&home, &user, &["new", "x", ABOUT, "--file", &text.to_string_lossy()]);
    assert!(
        dead.starts_with("err: the skill 'x' is not right yet: reference goes nowhere: references/missing.md"),
        "{dead}"
    );
    assert_eq!(fs::read_dir(store(&home)).unwrap().count(), 0);
    fs::create_dir_all(work.join("empty")).unwrap();
    assert!(say(&home, &user, &["check", &work.join("empty").to_string_lossy()]).starts_with("err: no SKILL.md in "));
    fs::write(work.join("empty").join(SKILL), "just text\n").unwrap();
    let broken = say(&home, &user, &["add", &work.join("empty").to_string_lossy()]);
    assert!(broken.starts_with("err: the skill 'empty' is not right yet: frontmatter missing or broken"), "{broken}");
    let thin = folder(&work, "thin", "Does things", &[]);
    assert_eq!(
        say(&home, &user, &["check", &thin]),
        "ok: the skill 'thin' is right\nbetter still: description under 40 characters: say what it does and when to use it"
    );
    let vague = folder(&work, "vague", "Handles every kind of table in a spreadsheet document", &[]);
    assert!(
        say(&home, &user, &["check", &vague]).ends_with("description never says when to use it (add: Use when ...)")
    );
    assert_eq!((yaml("Fix: things"), yaml("Plain words")), ("\"Fix: things\"".to_string(), "Plain words".to_string()));
    assert_eq!(short(&"word ".repeat(40)).chars().count(), ABOUT_SHOWN + 3);
    let _ = fs::remove_dir_all(home.parent().unwrap());
}
