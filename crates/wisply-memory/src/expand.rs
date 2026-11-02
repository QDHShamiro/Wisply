use std::collections::HashMap;
use std::sync::LazyLock;

const GROUPS: &[&str] = &[
    "titel title",
    "inventar inventory",
    "hoehe höhe height",
    "schaden damage",
    "gross groß size big large",
    "klein small little",
    "einstellung einstellungen setting settings config konfiguration",
    "abgebrochen abbruch aborted abort cancelled canceled",
    "loeschen löschen delete remove entfernen",
    "neu new",
    "datenset datensatz dataset",
    "umweg umgehung detour bypass",
    "sicht sight view",
    "luft air",
    "wasser water",
    "notfall emergency",
    "passwort kennwort password",
    "zuruecksetzen zurücksetzen reset",
    "sperren blockieren blocken block blocking blocked",
    "unterbefehl subcommand",
    "hintergrund background",
    "dunkelgrau darkgray dark gray grey",
    "schwelle schwellen threshold thresholds limit",
    "beim bei on",
    "start startup boot starten",
    "fehler error failure bug",
    "behoben behebung fix fixed fixes",
    "erstellt erstellen created create",
    "hinzugefuegt hinzugefügt added add",
    "entfernt removed",
    "umgesetzt implemented implementation",
    "geaendert geändert changed modified change",
    "ueberarbeitet überarbeitet redesigned rewritten",
    "gespeichert speichern saved save store storage persist",
    "serverstart boot startup",
    "spieler player players",
    "waffe waffen weapon weapons gun guns",
    "zombie zombies",
    "mauer wall barricade",
    "bauen build building",
    "fertig done complete completed finished",
    "plan planung plans planning",
    "ziel goal target",
    "test tests testing",
    "anmeldung login signin",
    "benutzer nutzer user users",
    "sprache voice speech language",
    "bild image picture",
    "ordner folder directory",
    "datei file",
    "schnell fast quick speed performance",
    "langsam slow",
    "sicherheit security safety",
    "verschluesselung verschlüsselung encryption",
    "abfrage query",
    "anzeige display show ui",
    "zeile line row",
    "wert werte value values",
    "geschwindigkeit speed",
    "vorlage template",
    "meldung nachricht message messages",
    "befehl befehle command commands",
    "berechtigung berechtigungen permission permissions",
    "sicherung backup",
    "aktualisierung update updates",
    "fenster window",
    "kopie copy",
    "verbindung connection connect",
    "absturz crash",
    "gruppe group",
    "zeit time",
    "runde round",
    "ergebnis result results",
    "bester best",
    "kosten cost price preis",
    "pause break",
    "einzigartig unique",
    "niemals never",
    "immer always",
];

static MAP: LazyLock<HashMap<&'static str, Vec<usize>>> = LazyLock::new(|| {
    let mut map: HashMap<&'static str, Vec<usize>> = HashMap::new();
    for (i, group) in GROUPS.iter().enumerate() {
        for word in group.split_whitespace() {
            map.entry(word).or_default().push(i);
        }
    }
    map
});

const TAILS: &[&str] = &["ungen", "ung", "ing", "ed", "es", "en", "er", "s"];
const MIN_ROOT: usize = 4;

pub fn fold(word: &str) -> String {
    let mut out = String::with_capacity(word.len());
    for c in word.chars() {
        match c {
            'ä' | 'á' | 'à' | 'â' => out.push('a'),
            'ö' | 'ó' | 'ò' | 'ô' => out.push('o'),
            'ü' | 'ú' | 'ù' | 'û' => out.push('u'),
            'é' | 'è' | 'ê' => out.push('e'),
            'í' | 'ì' | 'î' => out.push('i'),
            'ß' => out.push_str("ss"),
            other => out.push(other),
        }
    }
    for (pair, one) in [("ae", "a"), ("oe", "o"), ("ue", "u")] {
        if out.len() > 4 && out.contains(pair) {
            out = out.replace(pair, one);
        }
    }
    out
}

fn strippable(root: &str, tail: &str) -> bool {
    let n = root.chars().count();
    match tail {
        "s" => n >= 3 && !root.ends_with('s'),
        "ed" => n >= 3 && root.contains(['a', 'i', 'o', 'u', 'y']),
        "ing" | "es" => n >= 3,
        _ => n >= MIN_ROOT,
    }
}

pub fn stem_word(word: &str) -> String {
    let mut w = fold(&word.to_lowercase());
    for _ in 0..2 {
        let Some(root) = TAILS.iter().find_map(|t| w.strip_suffix(t).filter(|r| strippable(r, t))) else {
            break;
        };
        w = root.to_string();
    }
    if w.chars().count() > 3 && w.ends_with('e') {
        w.pop();
    }
    let mut tail = w.chars().rev();
    if let (Some(a), Some(b)) = (tail.next(), tail.next())
        && a == b
        && a.is_alphabetic()
        && !matches!(a, 'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 's' | 'z')
        && w.chars().count() > 3
    {
        w.pop();
    }
    w
}

pub fn variants(term: &str) -> Vec<&'static str> {
    let mut out = Vec::new();
    for g in MAP.get(term).into_iter().flatten() {
        for w in GROUPS[*g].split_whitespace() {
            if w != term && !out.contains(&w) {
                out.push(w);
            }
        }
    }
    out
}

pub fn near(a: &str, b: &str) -> bool {
    let (x, y) = (a.as_bytes(), b.as_bytes());
    if x.len().min(y.len()) < 4 || x.len().abs_diff(y.len()) > 1 || !a.is_ascii() || !b.is_ascii() {
        return false;
    }
    let skip = x.iter().zip(y).take_while(|(p, q)| p == q).count();
    let (rx, ry) = (&x[skip..], &y[skip..]);
    match rx.len().cmp(&ry.len()) {
        std::cmp::Ordering::Equal => {
            !rx.is_empty()
                && (rx[1..] == ry[1..] || (rx.len() >= 2 && rx[0] == ry[1] && rx[1] == ry[0] && rx[2..] == ry[2..]))
        }
        std::cmp::Ordering::Greater => rx[1..] == *ry,
        std::cmp::Ordering::Less => *rx == ry[1..],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_cross_languages_and_typos_stay_close() {
        assert!(variants("titel").contains(&"title") && variants("title").contains(&"titel"));
        assert!(variants("zzz").is_empty());
        assert!(!near("prewarm", "voise") && near("voice", "voise") && near("sight", "sigth"));
        assert!(near("ability", "abilty") && !near("insertion", "insertng") && !near("same", "same"));
    }

    #[test]
    fn stems_join_word_forms_and_umlauts() {
        for (a, b) in [("settings", "setting"), ("saved", "save"), ("embedding", "embed"), ("hoehe", "höhe")] {
            assert_eq!(stem_word(a), stem_word(b), "{a} {b}");
        }
        assert_eq!(stem_word("Einstellungen"), stem_word("einstellung"));
        assert_ne!(stem_word("plan"), stem_word("plate"));
    }
}
