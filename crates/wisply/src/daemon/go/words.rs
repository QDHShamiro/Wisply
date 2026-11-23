pub const MSG: &[&str] = &[
    "schreib",
    "schreibe",
    "schreibt",
    "schick",
    "schicke",
    "sende",
    "send",
    "sag",
    "sage",
    "antworte",
    "antwort",
    "frag",
    "frage",
    "write",
    "text",
    "tell",
    "message",
    "msg",
    "ask",
    "dm",
    "reply",
    "nachricht",
];
pub const PLAY: &[&str] = &["spiel", "spiele", "play", "hör", "höre", "hoer", "hoere", "abspielen"];
pub const OPEN: &[&str] = &["öffne", "öffnen", "oeffne", "offne", "starte", "start", "open", "launch", "run"];
pub const SEARCH: &[&str] = &["such", "suche", "suchen", "google", "search", "find", "finde", "look"];
pub const REMIND: &[&str] = &["erinner", "erinnere", "remind"];
pub const SHOT: &[&str] = &["screenshot", "bildschirmfoto", "shot", "screen"];
pub const REMEMBER: &[&str] = &["merk", "merke", "notier", "notiere", "remember", "note"];
pub const RECALL_HEADS: &[&str] = &[
    "was weißt du über",
    "was weisst du über",
    "was weißt du ueber",
    "was weisst du ueber",
    "was weißt du zu",
    "was weisst du zu",
    "erinnerst du dich an",
    "what do you know about",
    "do you remember",
    "recall",
];
pub const FILLER: &[&str] =
    &["bitte", "mal", "please", "kurz", "schnell", "doch", "einfach", "jetzt", "now", "noch", "hey", "wisply", "yo"];
pub const ARTICLES: &[&str] = &["dem", "der", "den", "die", "an", "to", "zu", "mit", "with"];
pub const JOIN: &[&str] = &["und", "and", "&", "+", "sowie"];
pub const APP_LEAD: &[&str] = &["auf", "on", "per", "via", "über", "ueber", "in", "bei", "im"];
pub const TEXT_LEAD: &[&str] = &["dass", "das", "that", ":", "-"];
pub const ME: &[&str] = &["mich", "me", "mir"];
pub const ABOUT: &[&str] = &["an", "dass", "to", "about", "daran", "that", "zu"];
pub const DIR: &[&str] = &["dir", "you", "dass", "that", ":"];
pub const OF: &[&str] = &["von", "vom", "of", "from", "der", "des"];
pub const MSG_NOISE: &[&str] = &["ne", "eine", "nachricht", "message", "a", "msg", "bescheid", "kurz"];
pub const SEARCH_NOISE: &[&str] = &["nach", "for", "mal", "bitte", "up"];
pub const PLAY_NOISE: &[&str] =
    &["spiel", "spiele", "play", "mach", "mache", "make", "turn", "dreh", "stell", "the", "die", "der"];
pub const OPEN_NOISE: &[&str] = &["die", "den", "das", "mir", "mal", "bitte", "the", "up"];
pub const SHOT_LEAD: &[&str] = &["mach", "mache", "nimm", "take", "make", "einen", "ein", "a", "an"];
pub const SITE_LEAD: &[&str] = &["nach", "for", "auf", "on", "in", "bei", "im"];

pub const MEDIA: &[(&[&str], &str)] = &[
    (&["pause", "pausier", "pausiere", "pausieren", "stopp", "stoppen", "stop", "halt", "anhalten"], "playpause"),
    (&["weiter", "fortsetzen", "resume", "continue", "unpause"], "playpause"),
    (&["nächstes", "nächster", "nächste", "naechstes", "naechster", "next", "skip", "überspringen"], "nexttrack"),
    (&["zurück", "zurueck", "vorheriges", "vorheriger", "previous", "prev", "back"], "prevtrack"),
    (&["lauter", "louder", "volume up", "laut"], "volumeup"),
    (&["leiser", "quieter", "volume down", "leise"], "volumedown"),
    (&["stumm", "mute", "ton aus", "unmute"], "mute"),
];
pub const MUSIC_WORDS: &[&str] = &["musik", "music", "song", "lied", "track", "spotify", "das", "den", "it"];

pub const CHAT_APPS: &[(&str, &[&str])] = &[
    ("discord", &["discord", "dc", "disc"]),
    ("whatsapp", &["whatsapp", "wa", "whats", "watsapp", "whatsap", "whatapp"]),
];
pub const PLAY_APPS: &[(&str, &[&str])] = &[("spotify", &["spotify"]), ("youtube", &["youtube", "yt"])];

pub const UNITS: &[(&[&str], u64)] = &[
    (&["s", "sek", "sekunde", "sekunden", "sec", "secs", "second", "seconds"], 1),
    (&["m", "min", "mins", "minute", "minuten", "minutes"], 60),
    (&["h", "std", "stunde", "stunden", "hour", "hours", "hr", "hrs"], 3600),
    (&["tag", "tage", "tagen", "day", "days"], 86_400),
];
pub const AT: &[&str] = &["um", "at", "gegen", "ab"];
pub const IN: &[&str] = &["in"];
pub const TOMORROW: &[&str] = &["morgen", "tomorrow"];
pub const HOUR_WORD: &[&str] = &["uhr", "o'clock", "oclock"];
pub const PM: &[&str] = &["pm", "p.m.", "abends", "nachmittags"];
pub const AM: &[&str] = &["am", "a.m.", "morgens", "früh", "frueh"];

pub fn is(list: &[&str], w: &str) -> bool {
    list.contains(&w)
}

pub fn app_of(table: &[(&'static str, &[&str])], w: &str) -> Option<&'static str> {
    table.iter().find(|(_, names)| names.contains(&w)).map(|(app, _)| *app)
}

pub fn media_of(phrase: &str) -> Option<&'static str> {
    MEDIA.iter().find(|(words, _)| words.contains(&phrase)).map(|(_, key)| *key)
}
