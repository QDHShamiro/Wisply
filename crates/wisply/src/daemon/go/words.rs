pub const MSG: &[&str] = &[
    "schreib",
    "schreibe",
    "schreibt",
    "schreiben",
    "schick",
    "schicke",
    "schicken",
    "sende",
    "senden",
    "send",
    "sag",
    "sage",
    "sagen",
    "antworte",
    "antworten",
    "antwort",
    "frag",
    "frage",
    "fragen",
    "write",
    "text",
    "texte",
    "texten",
    "tell",
    "message",
    "msg",
    "ask",
    "dm",
    "ping",
    "reply",
    "nachricht",
];
pub const MSG_INF: &[&str] = &["schreiben", "sagen", "texten", "schicken", "senden", "fragen", "antworten"];
pub const REPLY: &[&str] = &["antworte", "antworten", "antwort", "reply"];
pub const WITH: &[&str] = &["mit", "with"];
pub const SAY: &str = "sag";
pub const LET: &[&str] = &["let", "lass", "lasse", "laß", "gib", "gebe", "give"];
pub const LET_TRAIL: &[&str] = &["know", "wissen", "bescheid"];
pub const POLITE: &[&[&str]] = &[
    &["kannst", "du"],
    &["könntest", "du"],
    &["würdest", "du"],
    &["magst", "du"],
    &["can", "you"],
    &["could", "you"],
    &["would", "you"],
    &["will", "you"],
];
pub const FUZZY_HEADS: &[&str] =
    &["schreib", "schreibe", "erinner", "erinnere", "google", "screenshot", "bildschirmfoto"];
pub const PLAY: &[&str] = &["spiel", "spiele", "play", "hör", "höre", "hoer", "hoere", "abspielen"];
pub const OPEN: &[&str] = &["öffne", "öffnen", "oeffne", "offne", "starte", "start", "open", "launch", "run"];
pub const GO: &[&str] = &["geh", "gehe", "go"];
pub const GO_LEAD: &[&str] = &["auf", "to", "zu", "zur", "on"];
pub const MACH: &[&str] = &["mach", "mache", "make"];
pub const OPEN_TAIL: &[&str] = &["auf", "open"];
pub const SEARCH: &[&str] = &["such", "suche", "suchen", "google", "search", "find", "finde"];
pub const SEARCH_UP: &[&str] = &["look"];
pub const REMIND: &[&str] = &["erinner", "erinnere", "erinnern", "remind"];
pub const SHOT: &[&str] = &["screenshot", "bildschirmfoto", "shot", "screen"];
pub const REMEMBER: &[&str] = &["merk", "merke", "notier", "notiere", "remember", "note"];
pub const RECALL_HEADS: &[&str] = &[
    "was hab ich",
    "was habe ich",
    "was hatte ich",
    "wann hab ich",
    "wann habe ich",
    "what did i",
    "what have i",
    "when did i",
    "was weißt du über",
    "was weisst du über",
    "was weißt du ueber",
    "was weisst du ueber",
    "was weißt du von",
    "was weisst du von",
    "was weißt du zu",
    "was weisst du zu",
    "erinnerst du dich an",
    "what do you know about",
    "what do you remember about",
    "do you remember",
    "recall",
];
pub const FILLER: &[&str] = &[
    "bitte", "mal", "ma", "please", "kurz", "schnell", "doch", "einfach", "jetzt", "now", "noch", "hey", "wisply",
    "yo", "eben", "pls", "plz", "bidde",
];
pub const LEAN: &[&str] = &[
    "bitte", "mal", "ma", "please", "kurz", "schnell", "doch", "einfach", "jetzt", "now", "noch", "eben", "pls", "plz",
    "bidde",
];
pub const LEAD_FILLER: &[&str] = &[
    "ey", "eyy", "eyo", "alter", "digga", "dikka", "bro", "bruder", "ah", "ach", "oh", "hm", "hmm", "hmmm", "ok",
    "okay", "oke", "also", "na", "dann", "hallo", "hi", "moin", "servus",
];
pub const TRAIL_FILL: &[&str] = &[
    "bitte", "please", "pls", "plz", "bidde", "mal", "ma", "kurz", "eben", "schnell", "doch", "einfach", "danke",
    "thx", "thanks", "jetzt", "now",
];
pub const POLITE_TAIL: &[&str] = &["bitte", "please", "pls", "plz", "bidde", "danke", "thx", "thanks"];
pub const SMILEYS: &[&str] = &[":)", ":-)", ":(", ":-(", ":d", ";)", ";-)", ":p", ":/", "xd", "<3", "^^", "^_^", "c:"];
pub const ARTICLES: &[&str] = &["dem", "der", "den", "die", "an", "to", "zu", "mit", "with"];
pub const NOT_NAME: &[&str] = &[
    "ich", "i", "wir", "we", "dass", "that", "ob", "if", "du", "er", "es", "ihr", "you", "he", "she", "it", "they",
    "the", "is", "ist", "sind", "war", "bin", "are", "am", "was", "wie", "wo", "wann", "warum", "wer", "what", "why",
    "how", "who", "when", "where", "ein", "eine", "einen", "einem", "einer", "a", "an", "den", "die", "das", "dem",
    "mein", "meine", "meinen", "my", "your", "dein", "deine", "kein", "keine", "this", "diese", "dieses", "nicht",
    "not", "mal", "ma", "eben", "bitte", "please", "pls", "plz",
];
pub const SOFT_LEAD: &[&str] = &["mal", "ma", "eben", "pls", "plz"];
pub const SOFT_POLITE: &[&str] = &["bitte", "please", "bidde"];
pub const END_LEAD: &[&str] = &["an", "to", "für", "fuer", "for"];
pub const THINK: &[&str] = &[
    "denke", "denk", "denken", "denkt", "dachte", "gedacht", "think", "thinking", "thought", "glaube", "glauben",
    "danke", "dank",
];
pub const TEXT_SOFT: &[&str] = &["das"];
pub const TEXT_SOFT_NEXT: &[&str] = &["ich", "wir", "du", "er", "sie", "ihr", "i", "we"];
pub const STARTERS: &[&str] = &[
    "ich", "i", "wir", "we", "dass", "that", "ob", "if", "whether", "bin", "komm", "komme", "gleich", "sorry", "danke",
    "thx", "hi", "hallo", "moin", "gute", "guten", "alles", "lass", "lasst", "ok", "okay", "ja", "nein", "kannst",
    "kommst", "hast", "hab", "habe", "willst", "wollen", "bis", "wann", "wo", "wie", "was", "will", "wird", "haben",
    "hey", "im", "i'm", "see", "can", "are", "do", "did", "wanna",
];
pub const ALARM: &[&str] = &["wecker", "alarm", "weckruf"];
pub const WAKE: &[&str] = &["weck", "wecke", "wecken", "wake"];
pub const WAKE_NOISE: &[&str] = &["up", "auf"];
pub const ALARM_NOISE: &[&str] = &[
    "stell", "stelle", "stellen", "set", "mir", "me", "einen", "ein", "nen", "ne", "an", "a", "mach", "mache", "make",
    "bitte", "please", "setz", "setze", "create", "erstell", "erstelle", "den", "my", "mein", "meinen",
];
pub const ALARM_VERB: &[&str] = &["stellen", "setzen", "machen", "erstellen", "bitte", "please"];
pub const ALARM_FOR: &[&str] = &["für", "fuer", "for", "auf", "zum", "zur", "to"];
pub const INF_ACT: &[&str] = &["erinnern", "wecken"];
pub const REMIND_TEXT: &str = "Erinnerung";
pub const ALARM_TEXT: &str = "Wecker";
pub const JOIN: &[&str] = &["und", "and", "&", "+", "sowie"];
pub const APP_LEAD: &[&str] = &["auf", "on", "per", "via", "über", "ueber", "in", "bei", "im"];
pub const TEXT_LEAD: &[&str] = &["dass", "that", ":", "-"];
pub const ME: &[&str] = &["mich", "me", "mir"];
pub const ABOUT: &[&str] = &["an", "ans", "dass", "to", "about", "daran", "that", "zu", "für", "fuer", "for"];
pub const DIR: &[&str] = &["dir", "you", "dass", "that", ":", "to"];
pub const OF: &[&str] = &["von", "vom", "of", "from", "der", "des"];
pub const MSG_NOISE: &[&str] = &["ne", "eine", "nachricht", "message", "a", "msg", "bescheid", "kurz"];
pub const SEARCH_NOISE: &[&str] = &["nach", "for", "mal", "bitte", "up"];
pub const PLAY_NOISE: &[&str] =
    &["spiel", "spiele", "play", "mach", "mache", "make", "turn", "dreh", "stell", "the", "die", "der"];
pub const SOME: &[&str] = &["some", "etwas", "paar"];
pub const OPEN_NOISE: &[&str] =
    &["die", "den", "das", "mir", "mal", "bitte", "the", "up", "pls", "plz", "please", "eben", "kurz", "ma"];
pub const SHOT_LEAD: &[&str] =
    &["mach", "mache", "nimm", "nehm", "nehme", "take", "make", "einen", "ein", "a", "an", "nen", "nem", "ne"];
pub const SHOT_TAIL: &[&str] = &[
    "machen",
    "mach",
    "mache",
    "make",
    "take",
    "nehmen",
    "nimm",
    "aufnehmen",
    "erstellen",
    "bitte",
    "please",
    "pls",
    "plz",
];
pub const SITE_LEAD: &[&str] = &["nach", "for", "auf", "on", "in", "bei", "im"];

pub const MEDIA: &[(&[&str], &str)] = &[
    (&["pause", "pausier", "pausiere", "pausieren", "stopp", "stoppen", "stop", "halt", "anhalten"], "playpause"),
    (&["weiter", "fortsetzen", "resume", "continue", "unpause"], "playpause"),
    (
        &[
            "nächstes",
            "nächster",
            "nächste",
            "naechstes",
            "naechster",
            "next",
            "skip",
            "überspringen",
            "überspring",
            "überspringe",
        ],
        "nexttrack",
    ),
    (
        &[
            "zurück",
            "zurueck",
            "vorheriges",
            "vorheriger",
            "vorherige",
            "letztes",
            "letzter",
            "previous",
            "prev",
            "back",
        ],
        "prevtrack",
    ),
    (&["lauter", "louder", "volume up", "up volume", "laut"], "volumeup"),
    (&["leiser", "quieter", "volume down", "down volume", "leise"], "volumedown"),
    (&["stumm", "mute", "ton aus", "unmute"], "mute"),
];
pub const MEDIA_WITH_NOUN: &[(&[&str], &str)] = &[(&["aus", "off", "an", "on"], "playpause")];
pub const MEDIA_NOUN: &[&str] = &["ton", "sound", "lautstärke"];
pub const MEDIA_NOISE: &[&str] =
    &["bisschen", "bissl", "bit", "a", "ein", "etwas", "little", "mir", "this", "dieses", "diesen", "dieser"];
pub const MUSIC_WORDS: &[&str] = &["musik", "music", "song", "lied", "track", "spotify", "das", "den", "it"];
pub const MUSIC_NOUNS: &[&str] = &["musik", "music", "song", "lied", "track", "spotify"];

pub const CHAT_APPS: &[(&str, &[&str])] = &[
    ("discord", &["discord", "dc", "disc"]),
    ("whatsapp", &["whatsapp", "wa", "whats", "watsapp", "whatsap", "whatapp"]),
];
pub const PLAY_APPS: &[(&str, &[&str])] = &[("spotify", &["spotify"]), ("youtube", &["youtube", "yt"])];

pub const NUM_ARTICLES: &[&str] = &["ein", "eine", "einer", "einem", "an", "a", "ner", "ne", "nem"];
pub const NUMBERS: &[(&str, u64)] = &[
    ("ein", 1),
    ("eine", 1),
    ("einer", 1),
    ("einem", 1),
    ("ner", 1),
    ("ne", 1),
    ("nem", 1),
    ("an", 1),
    ("a", 1),
    ("one", 1),
    ("eins", 1),
    ("zwei", 2),
    ("two", 2),
    ("drei", 3),
    ("three", 3),
    ("vier", 4),
    ("four", 4),
    ("fünf", 5),
    ("fuenf", 5),
    ("five", 5),
    ("sechs", 6),
    ("six", 6),
    ("sieben", 7),
    ("seven", 7),
    ("acht", 8),
    ("eight", 8),
    ("neun", 9),
    ("nine", 9),
    ("zehn", 10),
    ("ten", 10),
    ("elf", 11),
    ("eleven", 11),
    ("zwölf", 12),
    ("zwoelf", 12),
    ("twelve", 12),
    ("fünfzehn", 15),
    ("fuenfzehn", 15),
    ("fifteen", 15),
    ("zwanzig", 20),
    ("twenty", 20),
    ("dreißig", 30),
    ("dreissig", 30),
    ("thirty", 30),
    ("vierzig", 40),
    ("forty", 40),
    ("fünfzig", 50),
    ("fuenfzig", 50),
    ("fifty", 50),
];
pub const UNITS: &[(&[&str], u64)] = &[
    (&["s", "sek", "sekunde", "sekunden", "sec", "secs", "second", "seconds"], 1),
    (&["m", "min", "mins", "minute", "minuten", "minutes"], 60),
    (&["viertelstunde", "viertelstd"], 900),
    (&["dreiviertelstunde"], 2700),
    (&["h", "std", "stunde", "stunden", "hour", "hours", "hr", "hrs"], 3600),
    (&["tag", "tage", "tagen", "day", "days"], 86_400),
    (&["woche", "wochen", "week", "weeks"], 604_800),
];
pub const HALF: &[&str] = &["halbe", "halben", "halber", "halbes", "half"];
pub const AT: &[&str] = &["um", "at", "gegen", "ab"];
pub const IN: &[&str] = &["in"];
pub const TODAY: &[&str] = &["heute", "today"];
pub const TOMORROW: &[&str] = &["morgen", "tomorrow"];
pub const AFTER_TOMORROW: &[&str] = &["übermorgen"];
pub const THIS_DAY: &[&str] = &["this", "diesen"];
pub const TONIGHT: &[&str] = &["tonight"];
pub const NIGHT_HOUR: u32 = 22;
pub const DAY_LEAD: &[&str] = &["am", "on", "nächsten", "nächste", "nächster", "next", "kommenden", "kommende"];
pub const WEEKDAYS: &[(&[&str], u32)] = &[
    (&["montag", "monday"], 1),
    (&["dienstag", "tuesday"], 2),
    (&["mittwoch", "wednesday"], 3),
    (&["donnerstag", "thursday"], 4),
    (&["freitag", "friday"], 5),
    (&["samstag", "sonnabend", "saturday"], 6),
    (&["sonntag", "sunday"], 7),
];
pub const PARTS: &[(&[&str], u32, bool)] = &[
    (&["früh", "morgens", "morning"], 8, false),
    (&["vormittag", "vormittags"], 10, false),
    (&["mittag", "mittags", "midday"], 12, false),
    (&["nachmittag", "nachmittags", "afternoon"], 15, true),
    (&["abend", "abends", "evening"], 19, true),
    (&["nacht", "nachts", "night"], 22, true),
];
pub const HOUR_WORD: &[&str] = &["uhr", "o'clock", "oclock"];
pub const PM: &[&str] = &["pm", "p.m.", "abends", "nachmittags", "nachts"];
pub const AM: &[&str] = &["am", "a.m.", "morgens", "früh"];
pub const MIDNIGHT: &[&str] = &["mitternacht", "midnight"];
pub const NOON_WORD: &[&str] = &["noon", "mittag", "mittags", "midday"];
pub const HALF_CLOCK: &[&str] = &["halb", "half"];
pub const THREE_QUARTER: &[&str] = &["dreiviertel"];
pub const QUARTER_CLOCK: &[&str] = &["viertel", "quarter"];
pub const PAST: &[&str] = &["nach", "past", "after"];
pub const TO: &[&str] = &["vor", "to", "before"];

pub fn fold(word: &str) -> String {
    let mut out = String::with_capacity(word.len() + 2);
    for c in word.to_lowercase().chars() {
        match c {
            'ä' => out.push_str("ae"),
            'ö' => out.push_str("oe"),
            'ü' => out.push_str("ue"),
            'ß' => out.push_str("ss"),
            other => out.push(other),
        }
    }
    out
}

pub fn same(a: &str, b: &str) -> bool {
    a == b || ((!a.is_ascii() || !b.is_ascii()) && fold(a) == fold(b))
}

pub fn near(a: &str, b: &str) -> bool {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    if a.len().abs_diff(b.len()) > 1 {
        return false;
    }
    let skip = a.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let (a, b) = (&a[skip..], &b[skip..]);
    match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Equal => {
            a.is_empty() || a[1..] == b[1..] || (a.len() >= 2 && a[0] == b[1] && a[1] == b[0] && a[2..] == b[2..])
        }
        std::cmp::Ordering::Greater => a[1..] == b[..],
        std::cmp::Ordering::Less => a[..] == b[1..],
    }
}

pub fn is(list: &[&str], w: &str) -> bool {
    list.iter().any(|e| same(e, w))
}

pub fn app_of(table: &[(&'static str, &[&str])], w: &str) -> Option<&'static str> {
    table.iter().find(|(_, names)| is(names, w)).map(|(app, _)| *app)
}

pub fn media_of(phrase: &str) -> Option<&'static str> {
    MEDIA.iter().find(|(words, _)| is(words, phrase)).map(|(_, key)| *key)
}

pub fn noun_media_of(phrase: &str) -> Option<&'static str> {
    MEDIA_WITH_NOUN.iter().find(|(words, _)| is(words, phrase)).map(|(_, key)| *key)
}
