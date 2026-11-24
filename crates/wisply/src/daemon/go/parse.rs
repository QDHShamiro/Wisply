use wisply_core::{Fail, local_parts};

use super::words::{self as w, is};

mod when;

use when::{inner_time, tail_time, time_at};

const VOLUME_STEPS: u32 = 5;
const FUZZY_MIN: usize = 5;
const LOOK_AFTER: &str = "up";
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

fn skip_all(t: &[Tok], mut i: usize, lists: &[&[&str]]) -> usize {
    while i < t.len() && (t[i].low.is_empty() || lists.iter().any(|l| is(l, &t[i].low))) {
        i += 1;
    }
    i
}

fn msg(t: &[Tok], now: f64, k: &Know) -> Result<Option<Plan>, Fail> {
    let reply = is(w::REPLY, &t[0].low);
    let mut i = skip(t, 1, w::LEAN);
    let mut app = chat_app(t, i).map(|(a, n)| {
        i += n;
        a.to_string()
    });
    i = skip(t, i, w::ARTICLES);
    if t.get(i).is_some_and(|x| is(w::ME, &x.low)) {
        return Ok(None);
    }
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
    if let Some((n, at)) = time_at(t, i, now, true)
        && (t[i + n - 1].cut || t.get(i + n).is_some_and(|x| is(w::TEXT_LEAD, &x.low)))
    {
        when = Some(at);
        i += n;
    }
    while i + 1 < t.len() && (is(w::MSG_NOISE, &t[i].low) || (reply && is(w::WITH, &t[i].low))) {
        i += 1;
    }
    if app.is_none()
        && let Some((a, n)) = chat_app(t, i)
        && i + n < t.len()
    {
        app = Some(a.to_string());
        i += n;
    }
    i = skip(t, i, w::TEXT_LEAD);
    let text = join(&t[i..]);
    if text.trim().is_empty() {
        return Err(Fail::hint(format!("what should {} get?", who.join(" + ")), "add the text after the name"));
    }
    Ok(Some(Plan { act: Act::Msg { who, app, text }, when }))
}

fn remind(t: &mut Vec<Tok>, now: f64, lead: Option<f64>) -> Result<Plan, Fail> {
    let mut i = skip_all(t, 1, &[w::LEAN, w::ME]);
    let mut when = None;
    if let Some((n, at)) = time_at(t, i, now, true) {
        when = Some(at);
        i += n;
    }
    let when = lead.or(when).or_else(|| tail_time(t, i, now, true));
    let Some(when) = when else {
        return Err(Fail::hint("when should I remind you?", "wisply go \"erinner mich in 10 min an Pizza\""));
    };
    i = skip(t, i, w::ABOUT);
    let text = join(&t[i.min(t.len())..]);
    let text = if text.trim().is_empty() { "Erinnerung".to_string() } else { text };
    Ok(Plan { act: Act::Remind { text }, when: Some(when) })
}

fn media(t: &[Tok]) -> Option<Act> {
    let noun = t.iter().any(|x| is(w::MUSIC_NOUNS, &x.low));
    let rest: Vec<&str> = t
        .iter()
        .map(|x| x.low.as_str())
        .filter(|l| {
            !l.is_empty()
                && !is(w::FILLER, l)
                && !is(w::MUSIC_WORDS, l)
                && !is(w::PLAY_NOISE, l)
                && !is(w::MEDIA_NOISE, l)
        })
        .collect();
    if rest.is_empty() || rest.len() > 2 {
        return None;
    }
    let phrase = rest.join(" ");
    let key = w::media_of(&phrase)
        .or_else(|| if rest.len() == 2 && is(w::MEDIA_NOUN, rest[0]) { w::media_of(rest[1]) } else { None })
        .or_else(|| if noun { w::noun_media_of(&phrase) } else { None })?;
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
    let i = skip_all(t, 1, &[w::LEAN, w::ME, w::SOME]).min(end);
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
    let mut i = skip(t, 1, w::LEAN);
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
    let i = skip_all(t, 0, &[w::SHOT_LEAD, w::LEAN]);
    if !t.get(i).is_some_and(|x| is(w::SHOT, &x.low)) {
        return None;
    }
    let j = skip_all(t, i + 1, &[w::OF, w::LEAN]);
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

fn open_what(t: &[Tok]) -> Option<Act> {
    let what = join(t).trim_end_matches(PUNCT).to_string();
    (!what.is_empty()).then_some(Act::Open { what })
}

fn open(t: &[Tok]) -> Option<Act> {
    open_what(&t[skip(t, 1, w::OPEN_NOISE)..])
}

fn go_to(t: &[Tok]) -> Option<Act> {
    let i = skip(t, 1, w::GO_LEAD);
    if i == 1 { None } else { open_what(&t[skip(t, i, w::OPEN_NOISE)..]) }
}

fn open_up(t: &[Tok]) -> Option<Act> {
    let last = t.last()?;
    if t.len() < 3 || !is(w::OPEN_TAIL, &last.low) {
        return None;
    }
    open_what(&t[skip(t, 1, w::OPEN_NOISE)..t.len() - 1])
}

fn domain(word: &str) -> bool {
    word.rsplit_once('.')
        .is_some_and(|(name, tld)| !name.is_empty() && tld.len() >= 2 && tld.chars().all(char::is_alphabetic))
}

fn remember(t: &[Tok]) -> Result<Act, Fail> {
    let i = skip(t, 1, w::DIR);
    let text = join(&t[i..]);
    if text.trim().is_empty() {
        return Err(Fail::hint("what should I remember?", "wisply go \"merk dir Franz mag Pizza\""));
    }
    Ok(Act::Remember { text })
}

fn looks_up(t: &[Tok], head: &str) -> bool {
    is(w::SEARCH_UP, head) && t.get(1).is_some_and(|x| x.low == LOOK_AFTER)
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
    if is(w::GO, head) {
        return Ok(go_to(t));
    }
    if is(w::MACH, head)
        && let Some(act) = open_up(t)
    {
        return Ok(Some(act));
    }
    if t.len() == 1 && ((k.site)(head) || domain(head)) {
        return Ok(Some(Act::Open { what: head.to_string() }));
    }
    if is(w::SEARCH, head) || looks_up(t, head) || (k.site)(head) {
        return Ok(search(t, head, k));
    }
    if is(w::REMEMBER, head) {
        return remember(t).map(Some);
    }
    Ok(None)
}

fn strip(t: &mut Vec<Tok>) -> bool {
    let mut polite = false;
    loop {
        let i = skip(t, 0, w::FILLER);
        let n = w::POLITE
            .iter()
            .find(|p| p.iter().enumerate().all(|(j, word)| t.get(i + j).is_some_and(|x| w::same(word, &x.low))))
            .map_or(0, |p| p.len());
        if i + n == 0 {
            return polite;
        }
        t.drain(..i + n);
        polite |= n > 0;
    }
}

fn reshape(t: &mut [Tok], polite: bool) {
    let Some(head) = t.first().map(|x| x.low.clone()) else { return };
    if polite && !is(w::MSG, &head) {
        let verb = (1..t.len().min(4)).find(|&p| is(w::MSG_INF, &t[p].low));
        if let Some(p) = verb {
            t[..=p].rotate_right(1);
            t[0] = Tok { raw: t[0].raw.clone(), low: t[0].low.clone(), cut: false };
        }
    }
}

fn let_know(t: &mut Vec<Tok>) {
    let Some(head) = t.first() else { return };
    if !is(w::LET, &head.low) {
        return;
    }
    if let Some(p) = (2..t.len().min(5)).find(|&p| is(w::LET_TRAIL, &t[p].low)) {
        t.remove(p);
        t[0] = Tok { raw: w::SAY.to_string(), low: w::SAY.to_string(), cut: false };
    }
}

fn bare_lead(t: &[Tok], now: f64) -> Option<(usize, f64)> {
    let (n, at) = time_at(t, 0, now, true)?;
    let next = &t.get(n)?.low;
    (is(w::REMIND, next) || is(w::MSG, next)).then_some((n, at))
}

fn prepare(sentence: &str, now: f64) -> (Vec<Tok>, Option<f64>) {
    let mut t = toks(sentence);
    let mut polite = strip(&mut t);
    let mut lead = None;
    if let Some((n, at)) = time_at(&t, 0, now, false).or_else(|| bare_lead(&t, now)) {
        lead = Some(at);
        t.drain(..n);
        polite |= strip(&mut t);
    }
    reshape(&mut t, polite);
    let_know(&mut t);
    (t, lead)
}

fn known(head: &str, k: &Know) -> bool {
    let lists = [w::MSG, w::PLAY, w::OPEN, w::SEARCH, w::REMIND, w::SHOT, w::REMEMBER, w::GO, w::MACH, w::LET];
    lists.iter().any(|l| is(l, head)) || (k.site)(head)
}

fn fix_head(t: &mut [Tok], k: &Know) {
    let Some(first) = t.first_mut() else { return };
    if first.low.chars().count() < FUZZY_MIN || known(&first.low, k) {
        return;
    }
    if let Some(canon) = w::FUZZY_HEADS.iter().find(|c| w::near(&first.low, c)) {
        first.low = (*canon).to_string();
    }
}

fn own_time(t: &mut Vec<Tok>, head: &str, now: f64, lead: Option<f64>, k: &Know) -> Result<Option<Plan>, Fail> {
    if is(w::MSG, head) {
        return msg(t, now, k);
    }
    if is(w::REMIND, head) {
        return remind(t, now, lead).map(Some);
    }
    Ok(None)
}

pub fn parse(sentence: &str, now: f64, k: &Know) -> Result<Option<Plan>, Fail> {
    let (mut t, lead) = prepare(sentence, now);
    fix_head(&mut t, k);
    let Some(head) = t.first().map(|x| x.low.clone()) else { return Ok(None) };
    if let Some(act) = recall(&t) {
        return Ok(Some(Plan { act, when: lead }));
    }
    if let Some(mut plan) = own_time(&mut t, &head, now, lead, k)? {
        plan.when = lead.or(plan.when);
        return Ok(Some(plan));
    }
    if is(w::MSG, &head) || is(w::REMIND, &head) {
        return Ok(None);
    }
    let keeps_time = is(w::REMEMBER, &head);
    let mut when = lead;
    if when.is_none() && !keeps_time {
        when = tail_time(&mut t, 1, now, false);
    }
    if when.is_none() && !keeps_time && !is(w::SEARCH, &head) && !(k.site)(&head) {
        when = inner_time(&mut t, now);
    }
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

#[cfg(test)]
#[path = "corpus_tests.rs"]
mod corpus;
