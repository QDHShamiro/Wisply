use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use serde_json::json;

use super::super::ctx::now;
use super::parse::{Plan, describe};
use super::{act, contacts, plan, teach};
use crate::daemon::web;

const PEOPLE: usize = 200;
const ROUTINES: usize = 100;
const RUNS: usize = 60;
const SENTENCE: &str = "schreib Person150 bin gleich da";

fn home(tag: &str) -> PathBuf {
    let h = std::env::temp_dir().join(format!("wisply-go-bench-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&h);
    fs::create_dir_all(h.join("routines")).expect("home");
    h
}

fn fill(h: &Path) {
    let people: Vec<_> = (0..PEOPLE)
        .map(|i| {
            json!({"name": format!("Person{i}"), "alias": [format!("pp{i}"), format!("kumpel{i}")],
                "discord": format!("user{i}"), "whatsapp": format!("Chat {i}"), "app": "discord",
                "dm": format!("https://discord.com/channels/@me/{i}"), "sent": i})
        })
        .collect();
    fs::write(h.join("contacts.json"), json!({"people": people}).to_string()).expect("contacts");
    for i in 0..ROUTINES {
        let r = json!({"name": format!("go-routine-{i}"),
            "say": [format!("bestell {{p1}} im Shop{i}"), format!("mach aufgabe{i} {{p1}} fertig")],
            "steps": [{"cmd": "open", "args": [format!("https://shop{i}.example")]},
                {"cmd": "type", "args": ["Suche", "{p1}"]}, {"cmd": "click", "args": ["Bestellen"]}]});
        fs::write(h.join("routines").join(format!("go-routine-{i}.json")), r.to_string()).expect("routine");
    }
    let sites: serde_json::Map<String, serde_json::Value> = (0..50)
        .map(|i| (format!("site{i}.com"), json!({"search": format!("https://site{i}.com/s?q={{q}}")})))
        .collect();
    fs::write(h.join("sites.json"), serde_json::Value::Object(sites).to_string()).expect("sites");
}

fn median(mut f: impl FnMut()) -> f64 {
    f();
    let mut all: Vec<f64> = (0..RUNS)
        .map(|_| {
            let t = Instant::now();
            f();
            t.elapsed().as_secs_f64() * 1e6
        })
        .collect();
    all.sort_by(f64::total_cmp);
    all[RUNS / 2]
}

fn both(name: &str, homes: &[PathBuf; 2], step: impl Fn(&Path)) {
    let mut flip = 0;
    let cold = median(|| {
        flip ^= 1;
        step(&homes[flip]);
    });
    let warm = median(|| step(&homes[0]));
    println!("BENCH {name:<34} cold {:>8.3} ms   warm {:>8.3} ms", cold / 1000.0, warm / 1000.0);
}

fn whole(h: &Path, t: f64) -> Option<String> {
    if teach::find(h, SENTENCE).is_some() {
        return None;
    }
    let p = plan(h, SENTENCE, t).ok()?;
    let who = act::preview(h, &p.act, false).ok()?;
    Some(format!("{}{who}", describe(&Plan { act: p.act, when: None }, t)))
}

#[test]
#[ignore]
fn bench_daemon_side_of_go_dry() {
    let homes = [home("a"), home("b")];
    homes.iter().for_each(|h| fill(h));
    let t = now();
    both("contacts load (200)", &homes, |h| drop(contacts::loaded(h)));
    both("teach::find miss (100 routines)", &homes, |h| drop(teach::find(h, SENTENCE)));
    both("web::site hit (youtube)", &homes, |h| drop(web::site(h, "youtube")));
    both("web::site miss", &homes, |h| drop(web::site(h, "person150")));
    both("plan = load + parse", &homes, |h| drop(plan(h, SENTENCE, t)));
    let p = plan(&homes[0], SENTENCE, t).expect("plan");
    both("describe", &homes, |_| drop(describe(&Plan { act: p.act.clone(), when: None }, t)));
    both("act::preview", &homes, |h| drop(act::preview(h, &p.act, false)));
    both("whole dry", &homes, |h| drop(whole(h, t)));
    let raw = median(|| drop(contacts::read(&homes[0])));
    println!("BENCH {:<34} raw   {:>8.3} ms", "contacts file read + parse", raw / 1000.0);
    homes.iter().for_each(|h| drop(fs::remove_dir_all(h)));
}

#[test]
fn caches_follow_the_files() {
    let h = home("fresh");
    fill(&h);
    assert_eq!(contacts::loaded(&h).len(), PEOPLE);
    assert!(contacts::knows(&contacts::loaded(&h), "KUMPEL7"));
    fs::write(h.join("contacts.json"), json!({"people": [{"name": "Solo"}]}).to_string()).expect("rewrite");
    assert_eq!(contacts::loaded(&h).len(), 1);
    assert!(!contacts::knows(&contacts::loaded(&h), "kumpel7"));
    assert!(teach::find(&h, "bestell tee im Shop7").is_some_and(|x| x.name == "go-routine-7"));
    assert!(teach::find(&h, "mach lampe an").is_none());
    let extra = json!({"name": "lamp", "say": ["mach lampe an"], "steps": []});
    fs::write(h.join("routines").join("lamp.json"), extra.to_string()).expect("routine");
    assert!(teach::find(&h, "mach lampe an").is_some_and(|x| x.name == "lamp"));
    fs::remove_file(h.join("routines").join("lamp.json")).expect("remove");
    assert!(teach::find(&h, "mach lampe an").is_none());
    assert_eq!(web::site(&h, "site3").as_deref(), Some("site3.com"));
    fs::write(h.join("sites.json"), "{}").expect("sites");
    assert_eq!(web::site(&h, "site3"), None);
    let _ = fs::remove_dir_all(&h);
}
