use super::*;

const NOW: f64 = 1_795_000_000.0;
const PEOPLE: &[&str] = &["franz", "lukas", "mama", "papa", "luis", "froxotron", "max mustermann"];
const SITES: &[&str] = &["youtube", "google", "amazon", "github", "reddit", "wikipedia", "twitch"];

fn person(w: &str) -> bool {
    PEOPLE.contains(&w)
}

fn site(w: &str) -> bool {
    SITES.contains(&w)
}

fn go(s: &str) -> Result<Option<Plan>, Fail> {
    parse(s, NOW, &Know { person: &person, site: &site })
}

fn plan(s: &str) -> Plan {
    go(s).unwrap_or_else(|e| panic!("{s}: {}", e.text())).unwrap_or_else(|| panic!("{s}: not understood"))
}

fn act(s: &str) -> Act {
    plan(s).act
}

fn hm(at: f64) -> (u32, u32) {
    let p = local_parts(at);
    (p[3], p[4])
}

fn m(who: &[&str], app: Option<&str>, text: &str) -> Act {
    Act::Msg { who: who.iter().map(|w| (*w).to_string()).collect(), app: app.map(str::to_string), text: text.into() }
}

fn media(key: &'static str) -> Act {
    let times = if key.starts_with("volume") { 5 } else { 1 };
    Act::Media { key, times }
}

#[test]
fn messages_now() {
    let cases: &[(&str, Act)] = &[
        ("schreib Franz ich komm später", m(&["Franz"], None, "ich komm später")),
        ("schreib Franz und Lukas ich bin gleich da", m(&["Franz", "Lukas"], None, "ich bin gleich da")),
        ("schreib Franz, Lukas: wir zocken", m(&["Franz", "Lukas"], None, "wir zocken")),
        ("schreib Franz auf Discord bin gleich on", m(&["Franz"], Some("discord"), "bin gleich on")),
        ("schreib auf whatsapp Mama ich komme", m(&["Mama"], Some("whatsapp"), "ich komme")),
        ("schreib dem Lukas dass ich später komme", m(&["Lukas"], None, "ich später komme")),
        ("sag Franz bescheid dass ich da bin", m(&["Franz"], None, "ich da bin")),
        ("schick Lukas ne Nachricht: bin da", m(&["Lukas"], None, "bin da")),
        ("nachricht an Franz: bin da", m(&["Franz"], None, "bin da")),
        ("schreib Franz whatsapp: hi", m(&["Franz"], Some("whatsapp"), "hi")),
        ("schreib Franz auf WhatsApp: hi", m(&["Franz"], Some("whatsapp"), "hi")),
        ("schreib Max Mustermann hallo", m(&["Max Mustermann"], None, "hallo")),
        ("bitte schreib mal Papa gute Nacht", m(&["Papa"], None, "gute Nacht")),
        ("frag Luis ob er heute kann", m(&["Luis"], None, "ob er heute kann")),
        ("schreib Franz auf geht's", m(&["Franz"], None, "auf geht's")),
        ("schreib @Froxotron gg", m(&["Froxotron"], None, "gg")),
        ("tell Lukas I'm on my way", m(&["Lukas"], None, "I'm on my way")),
        ("text mom and dad: dinner's ready", m(&["mom", "dad"], None, "dinner's ready")),
        ("message Franz on discord hey", m(&["Franz"], Some("discord"), "hey")),
        ("dm froxotron gg", m(&["froxotron"], None, "gg")),
        ("send Luis via dc the link", m(&["Luis"], Some("discord"), "the link")),
    ];
    for (s, want) in cases {
        let p = plan(s);
        assert_eq!(&p.act, want, "{s}");
        assert_eq!(p.when, None, "{s}");
    }
}

#[test]
fn time_words_inside_a_message_stay_text() {
    let cases: &[(&str, &str)] = &[
        ("schreib Franz um 5 bin da", "um 5 bin da"),
        ("schreib Franz in einer Stunde komme ich", "in einer Stunde komme ich"),
        ("schreib Franz in der Schule ist frei", "in der Schule ist frei"),
        ("schreib Lukas wir treffen uns um 18 Uhr", "wir treffen uns um 18 Uhr"),
        ("schreib Franz um 18 Uhr bin auf dem Weg", "um 18 Uhr bin auf dem Weg"),
    ];
    for (s, text) in cases {
        let p = plan(s);
        assert_eq!(p.act, m(&[s.split(' ').nth(1).unwrap_or("")], None, text), "{s}");
        assert_eq!(p.when, None, "{s}");
    }
}

#[test]
fn messages_later() {
    let p = plan("in 10 min schreib Franz hallo");
    assert_eq!((p.act, p.when), (m(&["Franz"], None, "hallo"), Some(NOW + 600.0)));
    let p = plan("schreib Franz in 5 min: hallo");
    assert_eq!((p.act, p.when), (m(&["Franz"], None, "hallo"), Some(NOW + 300.0)));
    let p = plan("schreib Franz um 18 Uhr: bin auf dem Weg");
    assert_eq!(p.act, m(&["Franz"], None, "bin auf dem Weg"));
    assert_eq!(p.when.map(hm), Some((18, 0)));
    let p = plan("at 18:00 send franz on my way");
    assert_eq!((p.act, p.when.map(hm)), (m(&["franz"], None, "on my way"), Some((18, 0))));
    let p = plan("text Franz at 6pm: on my way");
    assert_eq!((p.act, p.when.map(hm)), (m(&["Franz"], None, "on my way"), Some((18, 0))));
    let p = plan("send franz hi at 18:00");
    assert_eq!((p.act, p.when), (m(&["franz"], None, "hi at 18:00"), None));
    let p = plan("schreib Lukas morgen um 9 dass er mich weckt");
    assert_eq!(p.act, m(&["Lukas"], None, "er mich weckt"));
    let at = p.when.expect("when");
    assert!(at > NOW + 3600.0 && hm(at) == (9, 0));
    let p = plan("morgen um 9 schreib Lukas aufstehen");
    assert_eq!(p.act, m(&["Lukas"], None, "aufstehen"));
    assert!(p.when.is_some_and(|at| at > NOW && hm(at) == (9, 0)));
    let p = plan("at 6pm text lukas hi");
    assert_eq!(p.act, m(&["lukas"], None, "hi"));
    assert_eq!(p.when.map(hm), Some((18, 0)));
    let p = plan("um 20:15 schreib Mama auf whatsapp: komme gleich");
    assert_eq!(p.act, m(&["Mama"], Some("whatsapp"), "komme gleich"));
    assert_eq!(p.when.map(hm), Some((20, 15)));
}

#[test]
fn reminders() {
    let p = plan("erinner mich in 10 min an Pizza");
    assert_eq!((p.act, p.when), (Act::Remind { text: "Pizza".into() }, Some(NOW + 600.0)));
    let p = plan("erinner mich an Pizza in 10 min");
    assert_eq!((p.act, p.when), (Act::Remind { text: "Pizza".into() }, Some(NOW + 600.0)));
    let p = plan("remind me in an hour to drink water");
    assert_eq!((p.act, p.when), (Act::Remind { text: "drink water".into() }, Some(NOW + 3600.0)));
    let p = plan("erinner mich in 1h an Wasser");
    assert_eq!(p.when, Some(NOW + 3600.0));
    let p = plan("erinnere mich in 30 sekunden an Test");
    assert_eq!((p.act, p.when), (Act::Remind { text: "Test".into() }, Some(NOW + 30.0)));
    let p = plan("remind me at 5pm to call mom");
    assert_eq!(p.act, Act::Remind { text: "call mom".into() });
    assert_eq!(p.when.map(hm), Some((17, 0)));
    let p = plan("remind me tomorrow at 8 am to stretch");
    assert_eq!(p.act, Act::Remind { text: "stretch".into() });
    assert!(p.when.is_some_and(|at| at > NOW + 3600.0 && hm(at) == (8, 0)));
    let p = plan("erinner mich morgen um 7 Uhr an den Zahnarzt");
    assert_eq!(p.act, Act::Remind { text: "den Zahnarzt".into() });
    assert_eq!(p.when.map(hm), Some((7, 0)));
    let p = plan("erinnere mich um 20:30 an Wäsche");
    assert_eq!(p.act, Act::Remind { text: "Wäsche".into() });
    assert_eq!(p.when.map(hm), Some((20, 30)));
    let p = plan("in 2h erinner mich an Pause");
    assert_eq!((p.act, p.when), (Act::Remind { text: "Pause".into() }, Some(NOW + 7200.0)));
    let p = plan("erinner mich in 5 minuten");
    assert_eq!((p.act, p.when), (Act::Remind { text: "Erinnerung".into() }, Some(NOW + 300.0)));
    assert!(go("erinner mich an Pizza").is_err());
}

#[test]
fn media_keys() {
    let cases: &[(&str, &'static str)] = &[
        ("pause", "playpause"),
        ("musik pausieren", "playpause"),
        ("spiel weiter", "playpause"),
        ("stop the music", "playpause"),
        ("spiel musik", "playpause"),
        ("nächstes lied", "nexttrack"),
        ("skip", "nexttrack"),
        ("next song", "nexttrack"),
        ("play next", "nexttrack"),
        ("zurück", "prevtrack"),
        ("lauter", "volumeup"),
        ("mach lauter", "volumeup"),
        ("volume up", "volumeup"),
        ("leiser bitte", "volumedown"),
        ("mute", "mute"),
        ("ton aus", "mute"),
    ];
    for (s, key) in cases {
        assert_eq!(act(s), media(key), "{s}");
    }
}

#[test]
fn playing() {
    let cases: &[(&str, Option<&str>, &str)] = &[
        ("spiel Bohemian Rhapsody", None, "Bohemian Rhapsody"),
        ("spiel Shape of You auf Spotify", Some("spotify"), "Shape of You"),
        ("play lofi on youtube", Some("youtube"), "lofi"),
        ("hör Drake", None, "Drake"),
        ("spiel die Ärzte", None, "die Ärzte"),
        ("spiel Top 10", None, "Top 10"),
        ("play Blink 182 on spotify", Some("spotify"), "Blink 182"),
    ];
    for (s, app, q) in cases {
        assert_eq!(act(s), Act::Play { app: app.map(str::to_string), q: (*q).into() }, "{s}");
    }
    let p = plan("spiel lofi um 22 Uhr");
    assert_eq!(p.act, Act::Play { app: None, q: "lofi".into() });
    assert_eq!(p.when.map(hm), Some((22, 0)));
}

#[test]
fn opening() {
    let cases: &[(&str, &str)] = &[
        ("öffne Discord", "Discord"),
        ("open youtube.com", "youtube.com"),
        ("starte Minecraft", "Minecraft"),
        ("öffne mir mal die Einstellungen", "Einstellungen"),
        ("youtube", "youtube"),
        ("launch spotify", "spotify"),
    ];
    for (s, what) in cases {
        assert_eq!(act(s), Act::Open { what: (*what).into() }, "{s}");
    }
    let p = plan("öffne YouTube um 8 Uhr");
    assert_eq!(p.act, Act::Open { what: "YouTube".into() });
    assert_eq!(p.when.map(hm), Some((8, 0)));
}

#[test]
fn searching() {
    let cases: &[(&str, Option<&str>, &str)] = &[
        ("google Rust Lifetimes", None, "Rust Lifetimes"),
        ("such auf youtube nach minecraft redstone", Some("youtube"), "minecraft redstone"),
        ("youtube katzen videos", Some("youtube"), "katzen videos"),
        ("search amazon for usb c cable", Some("amazon"), "usb c cable"),
        ("suche nach Pizza Rezept", None, "Pizza Rezept"),
        ("such Pizza Rezept auf youtube", Some("youtube"), "Pizza Rezept"),
        ("wikipedia Albert Einstein", Some("wikipedia"), "Albert Einstein"),
        ("look up rust traits", None, "rust traits"),
        ("find cheap flights on google", Some("google"), "cheap flights"),
        ("such github wisply", Some("github"), "wisply"),
    ];
    for (s, site, q) in cases {
        assert_eq!(act(s), Act::Search { site: site.map(str::to_string), q: (*q).into() }, "{s}");
    }
}

#[test]
fn screenshots_memory_and_recall() {
    assert_eq!(act("screenshot"), Act::Shot { of: None });
    assert_eq!(act("mach einen Screenshot"), Act::Shot { of: None });
    assert_eq!(act("screenshot von Discord"), Act::Shot { of: Some("Discord".into()) });
    assert_eq!(act("take a screenshot of chrome"), Act::Shot { of: Some("chrome".into()) });
    assert_eq!(plan("screenshot in 10 sek").when, Some(NOW + 10.0));
    assert_eq!(act("merk dir Franz mag Pizza"), Act::Remember { text: "Franz mag Pizza".into() });
    let p = plan("merk dir Treffen um 18 Uhr");
    assert_eq!((p.act, p.when), (Act::Remember { text: "Treffen um 18 Uhr".into() }, None));
    assert_eq!(
        act("remember that the wifi password is on the fridge"),
        Act::Remember { text: "the wifi password is on the fridge".into() }
    );
    assert_eq!(act("was weißt du über Franz"), Act::Recall { q: "franz".into() });
    assert_eq!(act("what do you know about rust"), Act::Recall { q: "rust".into() });
    assert_eq!(act("erinnerst du dich an den Server"), Act::Recall { q: "den server".into() });
    assert!(go("merk dir").is_err());
}

#[test]
fn unknown_and_broken() {
    for s in ["mach mir einen Kaffee", "bestell Pizza", "", "   ", "hallo", "geh zurück", "morgen"] {
        assert_eq!(go(s).ok().flatten(), None, "{s}");
    }
    assert!(go("schreib").is_err());
    assert!(go("schreib Franz").is_err());
    assert!(go("schreib Franz und Lukas").is_err());
}

#[test]
fn describe_reads_well() {
    assert_eq!(describe(&plan("schreib Franz auf discord hi"), NOW), "message to Franz on discord: \"hi\"");
    assert_eq!(describe(&plan("lauter"), NOW), "media key volumeup x5");
    assert_eq!(describe(&plan("youtube katzen"), NOW), "search youtube for \"katzen\"");
    let later = describe(&plan("in 10 min screenshot"), NOW);
    assert!(later.starts_with("screenshot of the screen at ") && later.contains("in 10 min"), "{later}");
}
