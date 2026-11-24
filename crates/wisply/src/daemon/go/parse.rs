use std::sync::LazyLock;

use regex::Regex;
use wisply_core::{Fail, local_parts};

use super::words::{self as w, is};

static CLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(\d{1,2})(?:[:.](\d{2}))?(am|pm|uhr|h)?$").expect("clock"));
static SPAN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d{1,4})([a-zäöü]+)$").expect("span"));
static NUM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{1,4}$").expect("num"));
const NUM_WORDS: &[(&str, u64)] = &[
    ("ein", 1),
    ("eine", 1),
    ("einer", 1),
    ("einem", 1),
    ("an", 1),
    ("a", 1),
    ("one", 1),
    ("zwei", 2),
    ("two", 2),
    ("drei", 3),
    ("three", 3),
    ("fünf", 5),
    ("fuenf", 5),
    ("five", 5),
    ("zehn", 10),
    ("ten", 10),
    ("fünfzehn", 15),
    ("fifteen", 15),
    ("zwanzig", 20),
    ("twenty", 20),
    ("dreißig", 30),
    ("dreissig", 30),
    ("thirty", 30),
];
const DAY: f64 = 86_400.0;
const VOLUME_STEPS: u32 = 5;
const PUNCT: &[char] = &[',', '.', '!', '?', ';', ':'];

#[derive(Debug, Clone, PartialEq)]
pub enum Act {
    Msg { who: Vec<String>, app: Option<String>, text: String },
    Play { app: Option<String>, q: String },
    Media { key: &'static str, times: u32 },
    Open { what: String },
    Search { site: Option<String>, q: String },
    Remind { text: String },
    Shot { of: Option<String> },
    Remember { text: String },
    Recall { q: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    pub act: Act,
    pub when: Option<f64>,
}

#[derive(Debug, Clone)]
struct Tok {
    raw: String,
    low: String,
    cut: bool,
}

pub struct Know<'a> {
    pub person: &'a dyn Fn(&str) -> bool,
    pub site: &'a dyn Fn(&str) -> bool,
}

fn toks(s: &str) -> Vec<Tok> {
    s.split_whitespace()
        .map(|raw| {
            let low = raw.to_lowercase();
            let cut = low.ends_with([',', ':']);
            let low = low.trim_end_matches(PUNCT).trim_start_matches('@').to_string();
            Tok { raw: raw.to_string(), low, cut }
        })
        .collect()
}

fn join(t: &[Tok]) -> String {
    t.iter().map(|t| t.raw.as_str()).collect::<Vec<_>>().join(" ")
}

fn skip(t: &[Tok], mut i: usize, list: &[&str]) -> usize {
    while i < t.len() && (t[i].low.is_empty() || is(list, &t[i].low)) {
        i += 1;
    }
    i
}

fn num_of(low: &str) -> Option<u64> {
    if NUM.is_match(low) {
        return low.parse().ok();
    }
    NUM_WORDS.iter().find(|(word, _)| *word == low).map(|(_, n)| *n)
}

fn unit_of(low: &str) -> Option<u64> {
    w::UNITS.iter().find(|(names, _)| names.contains(&low)).map(|(_, s)| *s)
}

fn day_start(now: f64) -> f64 {
    let [_, _, _, h, mi, s] = local_parts(now);
    now.floor() - f64::from(h * 3600 + mi * 60 + s)
}

fn at_clock(now: f64, h: u32, mi: u32, tomorrow: bool) -> f64 {
    let base = day_start(now) + if tomorrow { DAY } else { 0.0 };
    let mut t = base + f64::from(h * 3600 + mi * 60);
    let got = local_parts(t)[3];
    if got != h {
        t += f64::from((h as i32 - got as i32) * 3600);
    }
    if !tomorrow && t <= now {
        return at_clock(now, h, mi, true);
    }
    t
}

fn suffix_at(t: &[Tok], i: usize) -> bool {
    t.get(i).is_some_and(|x| is(w::HOUR_WORD, &x.low) || is(w::PM, &x.low) || is(w::AM, &x.low))
}

fn clock(t: &[Tok], i: usize) -> Option<(usize, u32, u32, bool)> {
    let m = CLOCK.captures(&t.get(i)?.low)?;
    let mut h: u32 = m[1].parse().ok()?;
    let mi: u32 = m.get(2).map_or(Some(0), |x| x.as_str().parse().ok())?;
    let mut used = 1;
    let mut suffix = m.get(3).map(|x| x.as_str().to_string());
    while used < 3 && suffix_at(t, i + used) {
        let next = t[i + used].low.clone();
        if suffix.is_none() || is(w::PM, &next) || is(w::AM, &next) {
            suffix = Some(next);
        }
        used += 1;
    }
    match suffix.as_deref() {
        Some(s) if is(w::PM, s) && h < 12 => h += 12,
        Some(s) if is(w::AM, s) && h == 12 => h = 0,
        _ => {}
    }
    let worded = suffix.is_some() || m.get(2).is_some();
    (h < 24 && mi < 60).then_some((used, h, mi, worded))
}

fn span(t: &[Tok], i: usize, now: f64) -> Option<(usize, f64)> {
    let next = &t.get(i + 1)?.low;
    if let Some(m) = SPAN.captures(next)
        && let (Ok(n), Some(u)) = (m[1].parse::<u64>(), unit_of(&m[2]))
    {
        return Some((2, now + (n * u) as f64));
    }
    let n = num_of(next)?;
    let u = unit_of(&t.get(i + 2)?.low)?;
    Some((3, now + (n * u) as f64))
}

fn time_at(t: &[Tok], i: usize, now: f64, strict: bool) -> Option<(usize, f64)> {
    let first = &t.get(i)?.low;
    if is(w::IN, first) {
        return span(t, i, now);
    }
    let tomorrow = is(w::TOMORROW, first);
    let mut j = i + usize::from(tomorrow);
    let lead = t.get(j).is_some_and(|x| is(w::AT, &x.low));
    j += usize::from(lead);
    let (used, h, mi, worded) = clock(t, j)?;
    let loose = if strict { !worded } else { !worded && !lead };
    if loose {
        return None;
    }
    Some((j + used - i, at_clock(now, h, mi, tomorrow)))
}

fn tail_time(t: &mut Vec<Tok>, from: usize, now: f64) -> Option<f64> {
    for start in from..t.len() {
        if let Some((n, at)) = time_at(t, start, now, false)
            && start + n == t.len()
        {
            t.truncate(start);
            return Some(at);
        }
    }
    None
}

fn bare(raw: &str) -> String {
    raw.trim_end_matches(PUNCT).trim_start_matches('@').to_string()
}

fn names(t: &[Tok], mut i: usize, k: &Know) -> (Vec<String>, usize) {
    let mut who = Vec::new();
    while i < t.len() {
        let two = t.get(i + 1).filter(|n| (k.person)(&format!("{} {}", t[i].low, n.low)));
        let (name, used, cut) = match two {
            Some(n) => (format!("{} {}", bare(&t[i].raw), bare(&n.raw)), 2, n.cut),
            None => (bare(&t[i].raw), 1, t[i].cut),
        };
        if name.is_empty() {
            break;
        }
        who.push(name);
        i += used;
        if t.get(i).is_some_and(|x| is(w::JOIN, &x.low)) && t.get(i + 1).is_some() {
            i += 1;
            continue;
        }
        if cut && t.get(i).is_some_and(|x| (k.person)(&x.low)) {
            continue;
        }
        break;
    }
    (who, i)
}

fn chat_app(t: &[Tok], i: usize) -> Option<(&'static str, usize)> {
    let lead = t.get(i).is_some_and(|x| is(w::APP_LEAD, &x.low));
    let at = i + usize::from(lead);
    let app = w::app_of(w::CHAT_APPS, &t.get(at)?.low)?;
    (lead || t.get(at).is_some_and(|x| x.cut)).then_some((app, at + 1 - i))
}

fn msg(t: &[Tok], now: f64, k: &Know) -> Result<Plan, Fail> {
    let mut i = skip(t, 1, w::FILLER);
    let mut app = chat_app(t, i).map(|(a, n)| {
        i += n;
        a.to_string()
    });
    i = skip(t, i, w::ARTICLES);
    let (who, at) = names(t, i, k);
    if who.is_empty() {
        return Err(Fail::hint("who should get it?", "wisply go \"schreib Franz ich komm später\""));
    }
    i = at;
    if let Some((a, n)) = chat_app(t, i) {
        app = Some(a.to_string());
        i += n;
    }
    let mut when = None;
    if let Some((n, at)) = time_at(t, i, now, false)
        && (t[i + n - 1].cut || t.get(i + n).is_some_and(|x| is(w::TEXT_LEAD, &x.low)))
    {
        when = Some(at);
        i += n;
    }
    while i + 1 < t.len() && is(w::MSG_NOISE, &t[i].low) {
        i += 1;
    }
    i = skip(t, i, w::TEXT_LEAD);
    let text = join(&t[i..]);
    if text.trim().is_empty() {
        return Err(Fail::hint(format!("what should {} get?", who.join(" + ")), "add the text after the name"));
    }
    Ok(Plan { act: Act::Msg { who, app, text }, when })
}

fn remind(t: &mut Vec<Tok>, now: f64, lead: Option<f64>) -> Result<Plan, Fail> {
    let mut i = skip(t, 1, w::FILLER);
    i = skip(t, i, w::ME);
    let mut when = None;
    if let Some((n, at)) = time_at(t, i, now, false) {
        when = Some(at);
        i += n;
    }
    let when = lead.or(when).or_else(|| tail_time(t, i, now));
    let Some(when) = when else {
        return Err(Fail::hint("when should I remind you?", "wisply go \"erinner mich in 10 min an Pizza\""));
    };
    i = skip(t, i, w::ABOUT);
    let text = join(&t[i.min(t.len())..]);
    let text = if text.trim().is_empty() { "Erinnerung".to_string() } else { text };
    Ok(Plan { act: Act::Remind { text }, when: Some(when) })
}

fn media(t: &[Tok]) -> Option<Act> {
    let rest: Vec<&str> = t
        .iter()
        .map(|x| x.low.as_str())
        .filter(|l| !l.is_empty() && !is(w::FILLER, l) && !is(w::MUSIC_WORDS, l) && !is(w::PLAY_NOISE, l))
        .collect();
    if rest.is_empty() || rest.len() > 2 {
        return None;
    }
    let key = w::media_of(&rest.join(" "))?;
    let times = if key.starts_with("volume") { VOLUME_STEPS } else { 1 };
    Some(Act::Media { key, times })
}

fn play(t: &[Tok]) -> Act {
    let mut end = t.len();
    let mut app = None;
    if end > 2
        && let Some(a) = w::app_of(w::PLAY_APPS, &t[end - 1].low)
    {
        app = Some(a.to_string());
        end -= 1;
        if end > 2 && is(w::APP_LEAD, &t[end - 1].low) {
            end -= 1;
        }
    }
    let i = skip(t, 1, w::FILLER).min(end);
    let rest = &t[i..end];
    if rest.iter().all(|x| x.low.is_empty() || is(w::MUSIC_WORDS, &x.low)) {
        return Act::Media { key: "playpause", times: 1 };
    }
    Act::Play { app, q: join(rest) }
}

fn site_after(t: &[Tok], i: usize, k: &Know) -> Option<(String, usize)> {
    let lead = t.get(i).is_some_and(|x| is(w::SITE_LEAD, &x.low));
    let at = i + usize::from(lead);
    let s = t.get(at).filter(|x| (k.site)(&x.low))?;
    (t.len() > at + 1).then(|| (s.low.clone(), at + 1))
}

fn search(t: &[Tok], head: &str, k: &Know) -> Option<Act> {
    let mut i = skip(t, 1, w::FILLER);
    let mut site = (!is(w::SEARCH, head) && (k.site)(head)).then(|| head.to_string());
    if site.is_none()
        && let Some((s, next)) = site_after(t, i, k)
    {
        site = Some(s);
        i = next;
    }
    i = skip(t, i, w::SEARCH_NOISE);
    let mut end = t.len();
    if site.is_none() && end >= i + 3 && is(w::SITE_LEAD, &t[end - 2].low) && (k.site)(&t[end - 1].low) {
        site = Some(t[end - 1].low.clone());
        end -= 2;
    }
    let q = join(&t[i.min(end)..end]);
    (!q.trim().is_empty()).then_some(Act::Search { site, q })
}

fn shot(t: &[Tok]) -> Option<Act> {
    let i = skip(t, 0, w::SHOT_LEAD);
    if !t.get(i).is_some_and(|x| is(w::SHOT, &x.low)) {
        return None;
    }
    let j = skip(t, i + 1, w::OF);
    let of = t.get(j..).map(join).filter(|s| !s.trim().is_empty());
    Some(Act::Shot { of })
}

fn recall(t: &[Tok]) -> Option<Act> {
    let low = t.iter().map(|x| x.low.as_str()).filter(|l| !l.is_empty()).collect::<Vec<_>>().join(" ");
    w::RECALL_HEADS.iter().find_map(|head| {
        let rest = low.strip_prefix(head)?.trim();
        (!rest.is_empty()).then(|| Act::Recall { q: rest.to_string() })
    })
}

fn open(t: &[Tok]) -> Option<Act> {
    let i = skip(t, 1, w::OPEN_NOISE);
    let what = join(&t[i..]).trim_end_matches(PUNCT).to_string();
    (!what.is_empty()).then_some(Act::Open { what })
}

fn remember(t: &[Tok]) -> Result<Act, Fail> {
    let i = skip(t, 1, w::DIR);
    let text = join(&t[i..]);
    if text.trim().is_empty() {
        return Err(Fail::hint("what should I remember?", "wisply go \"merk dir Franz mag Pizza\""));
    }
    Ok(Act::Remember { text })
}

fn simple(t: &[Tok], head: &str, k: &Know) -> Result<Option<Act>, Fail> {
    if let Some(act) = media(t) {
        return Ok(Some(act));
    }
    if is(w::PLAY, head) {
        return Ok(Some(play(t)));
    }
    if let Some(act) = shot(t) {
        return Ok(Some(act));
    }
    if is(w::OPEN, head) {
        return Ok(open(t));
    }
    if t.len() == 1 && (k.site)(head) {
        return Ok(Some(Act::Open { what: head.to_string() }));
    }
    if is(w::SEARCH, head) || (k.site)(head) {
        return Ok(search(t, head, k));
    }
    if is(w::REMEMBER, head) {
        return remember(t).map(Some);
    }
    Ok(None)
}

fn prepare(sentence: &str, now: f64) -> (Vec<Tok>, Option<f64>) {
    let mut t = toks(sentence);
    let start = skip(&t, 0, w::FILLER);
    t.drain(..start);
    let mut lead = None;
    if let Some((n, at)) = time_at(&t, 0, now, false) {
        lead = Some(at);
        t.drain(..n);
        let again = skip(&t, 0, w::FILLER);
        t.drain(..again);
    }
    (t, lead)
}

fn own_time(t: &mut Vec<Tok>, head: &str, now: f64, lead: Option<f64>, k: &Know) -> Result<Option<Plan>, Fail> {
    if is(w::MSG, head) {
        return msg(t, now, k).map(Some);
    }
    if is(w::REMIND, head) {
        return remind(t, now, lead).map(Some);
    }
    Ok(None)
}

pub fn parse(sentence: &str, now: f64, k: &Know) -> Result<Option<Plan>, Fail> {
    let (mut t, lead) = prepare(sentence, now);
    let Some(head) = t.first().map(|x| x.low.clone()) else { return Ok(None) };
    if let Some(act) = recall(&t) {
        return Ok(Some(Plan { act, when: lead }));
    }
    if let Some(mut plan) = own_time(&mut t, &head, now, lead, k)? {
        plan.when = lead.or(plan.when);
        return Ok(Some(plan));
    }
    let keeps_time = is(w::REMEMBER, &head);
    let when = lead.or_else(|| if keeps_time { None } else { tail_time(&mut t, 1, now) });
    Ok(simple(&t, &head, k)?.map(|act| Plan { act, when }))
}

pub fn label(now: f64, at: f64) -> String {
    let [_, mo, d, h, mi, _] = local_parts(at);
    let [_, nmo, nd, ..] = local_parts(now);
    let mins = ((at - now) / 60.0).round().max(0.0) as u64;
    if (mo, d) == (nmo, nd) {
        format!("{h:02}:{mi:02} (in {mins} min)")
    } else {
        format!("{d:02}.{mo:02}. {h:02}:{mi:02}")
    }
}

pub fn describe(plan: &Plan, now: f64) -> String {
    let what = match &plan.act {
        Act::Msg { who, app, text } => {
            format!("message to {} on {}: \"{text}\"", who.join(" + "), app.as_deref().unwrap_or("their usual app"))
        }
        Act::Play { app, q } => format!("play \"{q}\" on {}", app.as_deref().unwrap_or("spotify")),
        Act::Media { key, times } if *times > 1 => format!("media key {key} x{times}"),
        Act::Media { key, .. } => format!("media key {key}"),
        Act::Open { what } => format!("open {what}"),
        Act::Search { site: Some(s), q } => format!("search {s} for \"{q}\""),
        Act::Search { site: None, q } => format!("search the web for \"{q}\""),
        Act::Remind { text } => format!("reminder \"{text}\""),
        Act::Shot { of: Some(o) } => format!("screenshot of {o}"),
        Act::Shot { of: None } => "screenshot of the screen".into(),
        Act::Remember { text } => format!("remember \"{text}\""),
        Act::Recall { q } => format!("recall \"{q}\""),
    };
    match plan.when {
        Some(at) => format!("{what} at {}", label(now, at)),
        None => what,
    }
}

#[cfg(test)]
#[path = "parse_tests.rs"]
mod tests;
