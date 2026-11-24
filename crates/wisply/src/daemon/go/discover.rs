use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use wisply_core::{Call, Fail};

use super::contacts::{self, Person};

const ROUTINE: &str = "list-chats";
const SHOWN: usize = 20;
const ROOT: &str = "https://discord.com";
const USAGE: &str = "wisply contacts scan [--add]";

static LABEL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?is)^(.*?)\s*\((direktnachricht|direct message|gruppennachricht|group message)\)")
        .expect("label pattern")
});
static LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^/channels/@me/\d+$").expect("link pattern"));

#[derive(Debug, Clone, PartialEq)]
pub struct Seen {
    pub app: String,
    pub name: String,
    pub dm: String,
    pub group: bool,
}

#[derive(Debug, Default, PartialEq)]
pub struct Scan {
    pub seen: Vec<Seen>,
    pub notes: Vec<String>,
}

pub fn wants(c: &Call) -> bool {
    c.pos.first().is_some_and(|p| p.eq_ignore_ascii_case("scan"))
}

fn tidy(raw: &str) -> String {
    let kept: String = raw
        .chars()
        .filter(|c| !c.is_control() && !matches!(*c, '\u{200b}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2060}'..='\u{2064}' | '\u{feff}'))
        .collect();
    kept.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn usable(name: &str) -> bool {
    name.chars().any(char::is_alphanumeric)
}

fn discord(label: &str, path: &str) -> Option<Seen> {
    let path = path.trim();
    if !LINK.is_match(path) {
        return None;
    }
    let hit = LABEL.captures(label)?;
    let name = tidy(&hit[1]);
    let group = !hit[2].to_lowercase().contains("direkt") && !hit[2].to_lowercase().contains("direct");
    usable(&name).then(|| Seen { app: "discord".into(), name, dm: format!("{ROOT}{path}"), group })
}

fn whatsapp(title: &str, mark: &str) -> Option<Seen> {
    let name = tidy(title);
    let group = !mark.trim().eq_ignore_ascii_case("chat");
    usable(&name).then(|| Seen { app: "whatsapp".into(), name, dm: String::new(), group })
}

pub fn parse(raw: &str) -> Scan {
    let mut scan = Scan::default();
    for line in raw.lines() {
        let cols: Vec<&str> = line.split('\t').collect();
        let [app, a, b] = cols[..] else { continue };
        match app {
            "discord" => scan.seen.extend(discord(a, b)),
            "whatsapp" => scan.seen.extend(whatsapp(a, b)),
            "skip" => scan.notes.push(format!("{a} not readable ({})", tidy(b))),
            _ => {}
        }
    }
    scan
}

fn same(a: &Seen, b: &Seen) -> bool {
    a.app == b.app && a.name.to_lowercase() == b.name.to_lowercase()
}

pub fn chats(seen: &[Seen]) -> Vec<Seen> {
    let mut out: Vec<Seen> = Vec::new();
    for s in seen.iter().filter(|s| !s.group) {
        if !out.iter().any(|o| same(o, s)) {
            out.push(s.clone());
        }
    }
    out
}

fn is_known(people: &[Person], s: &Seen) -> bool {
    let low = s.name.to_lowercase();
    people.iter().any(|p| {
        [&p.name, &p.discord, &p.whatsapp].iter().any(|w| !w.is_empty() && w.to_lowercase() == low)
            || p.alias.contains(&low)
    })
}

pub fn missing(seen: &[Seen], people: &[Person]) -> Vec<Seen> {
    chats(seen).into_iter().filter(|s| !is_known(people, s)).collect()
}

fn call(s: &Seen) -> Call {
    let mut pos = vec!["add".to_string(), s.name.clone(), format!("{}={}", s.app, s.name)];
    if !s.dm.is_empty() {
        pos.push(format!("dm={}", s.dm));
    }
    Call { name: "contact".into(), pos, flags: Default::default() }
}

pub fn apply(home: &Path, found: &[Seen]) -> (usize, usize) {
    let saved = found.iter().filter(|s| contacts::command(home, &call(s)).is_ok()).count();
    (saved, found.len() - saved)
}

fn names(found: &[Seen]) -> String {
    let mut shown: Vec<String> = found.iter().take(SHOWN).map(|s| format!("{} ({})", s.name, s.app)).collect();
    if found.len() > SHOWN {
        shown.push(format!("+{} more", found.len() - SHOWN));
    }
    shown.join(", ")
}

pub fn report(scan: &Scan, people: &[Person], add: Option<(usize, usize)>) -> String {
    let all = chats(&scan.seen);
    let new = missing(&scan.seen, people);
    let mut line = format!("found {}", all.len());
    if new.is_empty() {
        line.push_str(" · nothing new");
    } else {
        match add {
            Some((saved, failed)) => {
                line.push_str(&format!(" · saved {saved}: {}", names(&new)));
                if failed > 0 {
                    line.push_str(&format!(" · {failed} could not be saved"));
                }
            }
            None => line.push_str(&format!(" · new {}: {} · run with --add to save", new.len(), names(&new))),
        }
    }
    let groups = scan.seen.iter().filter(|s| s.group).count();
    if groups > 0 {
        line.push_str(&format!(" · {groups} groups left out"));
    }
    for note in &scan.notes {
        line.push_str(&format!(" · {note}"));
    }
    line
}

pub fn scan(home: &Path, c: &Call) -> Result<String, Fail> {
    if c.pos.len() > 1 {
        return Err(Fail::hint("contacts scan takes no names", USAGE));
    }
    let Some(raw) = c.val("rows") else {
        let mode = if c.on("add") { "add" } else { "show" };
        let pairs = [("app".to_string(), "all".to_string()), ("scan".to_string(), mode.to_string())];
        return Ok(wisply_proto::handoff_line(ROUTINE, &pairs, c.on("yes")));
    };
    let found = parse(raw);
    let people = contacts::load(home);
    let add = c.on("add").then(|| apply(home, &missing(&found.seen, &people)));
    Ok(report(&found, &people, add))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    const RAW: &str = "discord\tMaxi (Direktnachricht), Online\t/channels/@me/1001\n\
discord\tLena (Direktnachricht)\t/channels/@me/1002\n\
discord\tMaxi, Lena, Tom (Gruppennachricht), 3 Mitglieder\t/channels/@me/1003\n\
discord\tZed (Direct Message)\t/channels/@me/1004\n\
discord\tOhne Art\t/channels/@me/1005\n\
discord\tBroken (Direktnachricht)\t/store\n\
discord\t᲼᲼ (Direktnachricht)\t/channels/@me/1006\n\
whatsapp\tAnna\tchat\n\
whatsapp\tMaxi\tchat\n\
whatsapp\tTeam Alpha\tgroup\n\
whatsapp\tOma\tchat\n\
whatsapp\t\u{200e}Bob\u{200f}\tchat\n\
whatsapp\tanna\tchat\n\
! tip: something\n\
skip\twhatsapp\tno chat list (logged out)\n";

    fn person(name: &str, discord: &str, whatsapp: &str, alias: &[&str]) -> Person {
        Person {
            name: name.into(),
            discord: discord.into(),
            whatsapp: whatsapp.into(),
            alias: alias.iter().map(|a| (*a).into()).collect(),
            ..Person::default()
        }
    }

    fn home(tag: &str) -> std::path::PathBuf {
        let h = std::env::temp_dir().join(format!("wisply-discover-{tag}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&h);
        fs::create_dir_all(&h).expect("home");
        h
    }

    #[test]
    fn rows_become_chats_with_groups_apart() {
        let scan = parse(RAW);
        assert_eq!(scan.notes, vec!["whatsapp not readable (no chat list (logged out))"]);
        assert_eq!(scan.seen.iter().filter(|s| s.group).count(), 2);
        let names: Vec<String> = chats(&scan.seen).iter().map(|s| format!("{}:{}", s.app, s.name)).collect();
        assert_eq!(
            names,
            [
                "discord:Maxi",
                "discord:Lena",
                "discord:Zed",
                "whatsapp:Anna",
                "whatsapp:Maxi",
                "whatsapp:Oma",
                "whatsapp:Bob"
            ]
        );
        let maxi = scan.seen.iter().find(|s| s.name == "Maxi" && s.app == "discord").expect("maxi");
        assert_eq!(maxi.dm, "https://discord.com/channels/@me/1001");
    }

    #[test]
    fn only_the_missing_ones_are_suggested() {
        let scan = parse(RAW);
        let people =
            vec![person("Mama", "", "Oma", &[]), person("Max", "Maxi", "", &["bob"]), person("lena", "", "", &[])];
        let new: Vec<String> = missing(&scan.seen, &people).iter().map(|s| format!("{}:{}", s.app, s.name)).collect();
        assert_eq!(new, ["discord:Zed", "whatsapp:Anna"]);
    }

    #[test]
    fn report_is_one_short_line() {
        let scan = parse(RAW);
        let line = report(&scan, &[], None);
        assert!(line.starts_with("found 7 · new 7: Maxi (discord), Lena (discord), Zed (discord), Anna (whatsapp)"));
        assert!(line.ends_with(
            " · run with --add to save · 2 groups left out · whatsapp not readable (no chat list (logged out))"
        ));
        assert!(!line.contains('\n'));
        let known = vec![person("Zed", "", "", &[])];
        assert!(
            report(&parse("discord\tZed (Direktnachricht)\t/channels/@me/9\n"), &known, None).ends_with("nothing new")
        );
        assert_eq!(report(&Scan::default(), &[], None), "found 0 · nothing new");
    }

    #[test]
    fn long_lists_are_cut() {
        let raw: String = (0..30).map(|i| format!("whatsapp\tPerson {i}x\tchat\n")).collect();
        let line = report(&parse(&raw), &[], None);
        assert!(line.contains("found 30 · new 30: Person 0x (whatsapp)") && line.contains("+10 more"));
    }

    #[test]
    fn add_saves_new_ones_and_never_touches_existing() {
        let h = home("add");
        contacts::command(
            &h,
            &Call {
                name: "contact".into(),
                pos: vec!["add".into(), "Anna".into(), "discord=annax".into(), "alias=an".into()],
                flags: Default::default(),
            },
        )
        .expect("seed");
        let scan = parse(RAW);
        let people = contacts::load(&h);
        let new = missing(&scan.seen, &people);
        assert_eq!(apply(&h, &new), (new.len(), 0));
        let after = contacts::load(&h);
        let anna = after.iter().find(|p| p.name == "Anna").expect("anna");
        assert_eq!(
            (anna.discord.as_str(), anna.whatsapp.as_str(), anna.alias.clone()),
            ("annax", "", vec!["an".to_string()])
        );
        let maxi = after.iter().find(|p| p.name == "Maxi").expect("maxi");
        assert_eq!((maxi.discord.as_str(), maxi.whatsapp.as_str()), ("Maxi", "Maxi"));
        assert_eq!(maxi.dm, "https://discord.com/channels/@me/1001");
        assert_eq!(after.iter().find(|p| p.name == "Oma").map(|p| p.whatsapp.as_str()), Some("Oma"));
        assert!(missing(&scan.seen, &after).is_empty());
        let _ = fs::remove_dir_all(&h);
    }

    #[test]
    fn scan_hands_off_to_the_routine_and_reads_its_rows_back() {
        let h = home("scan");
        let ask = |argv: &[&str]| wisply_core::parse(&argv.iter().map(|a| (*a).to_string()).collect::<Vec<_>>());
        let out = scan(&h, &ask(&["contacts", "scan", "--add"])).expect("handoff");
        let (argv, _) = wisply_proto::handoff(&out).expect("a handoff");
        assert_eq!(argv, ["codemode", "--routine", "list-chats", "--arg", "app=all", "--arg", "scan=add"]);
        let shown = scan(&h, &ask(&["contacts", "scan", &format!("--rows={RAW}")])).expect("report");
        assert!(shown.starts_with("found 7 · new 7: Maxi (discord)"), "{shown}");
        assert!(contacts::load(&h).is_empty());
        let saved = scan(&h, &ask(&["contacts", "scan", "--add", &format!("--rows={RAW}")])).expect("report");
        assert!(saved.starts_with("found 7 · saved 7: "), "{saved}");
        assert_eq!(contacts::load(&h).len(), 7);
        assert!(scan(&h, &ask(&["contacts", "scan", "anna"])).is_err());
        let _ = fs::remove_dir_all(&h);
    }

    #[test]
    fn scan_is_recognised_and_bad_rows_are_ignored() {
        let c = Call { name: "contacts".into(), pos: vec!["Scan".into()], flags: Default::default() };
        assert!(wants(&c));
        assert!(!wants(&Call { name: "contacts".into(), pos: vec![], flags: Default::default() }));
        assert_eq!(parse("garbage\nwhatsapp\tonly two"), Scan::default());
    }
}
