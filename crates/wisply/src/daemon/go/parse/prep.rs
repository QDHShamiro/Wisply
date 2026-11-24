use super::super::words::{self as w, is};
use super::when::time_at;
use super::{Know, Tok, skip_all, toks};

const FUZZY_MIN: usize = 5;
const WINDOW: usize = 6;
const MIN_NAME_END: usize = 4;
const SYMBOL_FROM: u32 = 0x2190;
const ZWJ: char = '\u{200d}';
const ELLIPSIS: char = '…';
const DOUBLE_QUOTES: &[char] = &['"', '„', '“', '”', '«', '»'];
const SINGLE_QUOTES: &[char] = &['\'', '‘', '’', '‚'];

fn symbol(c: char) -> bool {
    c == ZWJ || (c as u32 >= SYMBOL_FROM && !c.is_alphanumeric())
}

fn trailing(c: char) -> bool {
    c.is_whitespace() || matches!(c, '!' | '?' | '.' | '~' | ELLIPSIS) || symbol(c)
}

fn trail_text(tail: &str) -> String {
    let words = tail.split_whitespace().collect::<Vec<_>>().join(" ");
    if words.is_empty() || !tail.starts_with(char::is_whitespace) { words } else { format!(" {words}") }
}

fn split(sentence: &str) -> (Vec<Tok>, String) {
    let body = sentence.trim_end_matches(trailing);
    let mut trail = trail_text(&sentence[body.len()..]);
    let mut t = toks(body);
    while t.last().is_some_and(|x| w::SMILEYS.iter().any(|s| s.eq_ignore_ascii_case(&x.raw))) {
        let Some(x) = t.pop() else { break };
        trail = format!(" {}{trail}", x.raw);
    }
    (t, trail)
}

pub(super) fn unquote(text: &str) -> String {
    let s = text.trim();
    let mut chars = s.chars();
    let (Some(first), Some(last)) = (chars.next(), chars.next_back()) else { return s.to_string() };
    let inner = chars.as_str().trim();
    let double = DOUBLE_QUOTES.contains(&first) && DOUBLE_QUOTES.contains(&last) && !inner.contains(DOUBLE_QUOTES);
    let single = SINGLE_QUOTES.contains(&first) && SINGLE_QUOTES.contains(&last) && !inner.contains(SINGLE_QUOTES);
    if (double || single) && !inner.is_empty() { inner.to_string() } else { s.to_string() }
}

pub(super) fn attach(text: String, trail: &str) -> String {
    if !text.trim().is_empty() {
        return format!("{text}{trail}");
    }
    let rest = trail.trim();
    if rest.chars().any(|c| !trailing(c)) || rest.chars().any(symbol) { rest.to_string() } else { text }
}

fn strip(t: &mut Vec<Tok>) -> bool {
    let mut polite = false;
    loop {
        let i = skip_all(t, 0, &[w::FILLER, w::LEAD_FILLER]);
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

fn polite_verb(t: &mut [Tok], polite: bool) {
    let Some(head) = t.first().map(|x| x.low.clone()) else { return };
    if polite && !is(w::MSG, &head) {
        let verb = (1..t.len().min(4)).find(|&p| is(w::MSG_INF, &t[p].low));
        if let Some(p) = verb {
            t[..=p].rotate_right(1);
            t[0].cut = false;
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

fn name_len(t: &[Tok], i: usize, k: &Know) -> usize {
    let Some(x) = t.get(i) else { return 0 };
    if t.get(i + 1).is_some_and(|n| (k.person)(&format!("{} {}", x.low, n.low))) {
        2
    } else {
        usize::from((k.person)(&x.low))
    }
}

fn between(x: &Tok, k: &Know) -> bool {
    [w::JOIN, w::MSG_NOISE, w::LEAN, w::APP_LEAD].iter().any(|l| is(l, &x.low))
        || w::app_of(w::CHAT_APPS, &x.low).is_some()
        || (k.person)(&x.low)
}

fn name_first(t: &mut [Tok], k: &Know) {
    if t.first().is_none_or(|x| is(w::MSG, &x.low)) {
        return;
    }
    let n = name_len(t, 0, k);
    if n == 0 {
        return;
    }
    let Some(p) = (n..t.len().min(WINDOW)).find(|&p| is(w::MSG_INF, &t[p].low)) else { return };
    if t[n..p].iter().all(|x| between(x, k)) {
        t[..=p].rotate_right(1);
        t[0].cut = false;
    }
}

fn name_end(t: &mut Vec<Tok>, k: &Know) {
    let n = t.len();
    if n < MIN_NAME_END || !is(w::MSG, &t[0].low) {
        return;
    }
    let len = if name_len(t, n - 2, k) == 2 {
        2
    } else if name_len(t, n - 1, k) == 1 {
        1
    } else {
        return;
    };
    let lead = n - len - 1;
    if lead < 2 || !is(w::END_LEAD, &t[lead].low) || is(w::THINK, &t[lead - 1].low) {
        return;
    }
    let first = skip_all(t, 1, &[w::LEAN, w::TEXT_LEAD]);
    if first >= lead || name_len(t, first, k) > 0 {
        return;
    }
    if !is(w::STARTERS, &t[first].low) {
        return;
    }
    let name: Vec<Tok> = t.drain(lead..).skip(1).collect();
    for (j, x) in name.into_iter().enumerate() {
        t.insert(1 + j, Tok { cut: false, ..x });
    }
}

fn tail_is_time(t: &[Tok], i: usize, now: f64) -> bool {
    i == t.len() || time_at(t, i, now, true).is_some_and(|(n, _)| i + n == t.len())
}

fn inf_last(t: &mut [Tok], now: f64) {
    if t.len() < 2 || !(is(w::ABOUT, &t[0].low) || is(w::ME, &t[0].low)) {
        return;
    }
    if let Some(p) = (1..t.len()).find(|&p| is(w::INF_ACT, &t[p].low) && tail_is_time(t, p + 1, now)) {
        t[..=p].rotate_right(1);
        t[0].cut = false;
    }
}

fn glued(x: &Tok, k: &Know) -> Option<String> {
    let (name, rest) = x.raw.split_once(':')?;
    let named = !name.is_empty() && (k.person)(&name.trim_start_matches('@').to_lowercase());
    (named && rest.chars().next().is_some_and(char::is_alphanumeric)).then(|| format!("{name}: {rest}"))
}

fn unglue(t: &mut Vec<Tok>, k: &Know) {
    if !t.first().is_some_and(|x| is(w::MSG, &x.low)) {
        return;
    }
    for p in 1..t.len().min(WINDOW) {
        if let Some(text) = glued(&t[p], k) {
            t.splice(p..=p, toks(&text));
            return;
        }
        if !(between(&t[p], k) || is(w::ARTICLES, &t[p].low)) {
            return;
        }
    }
}

fn shape(t: &mut Vec<Tok>, polite: bool, now: f64, k: &Know) {
    polite_verb(t, polite);
    let_know(t);
    name_first(t, k);
    name_end(t, k);
    inf_last(t, now);
    unglue(t, k);
}

fn bare_lead(t: &[Tok], now: f64) -> Option<(usize, f64)> {
    let (n, at) = time_at(t, 0, now, true)?;
    let next = &t.get(n)?.low;
    (is(w::REMIND, next) || is(w::MSG, next)).then_some((n, at))
}

pub(super) fn prepare(sentence: &str, now: f64, k: &Know) -> (Vec<Tok>, Option<f64>, String) {
    let (mut t, trail) = split(sentence);
    let mut polite = strip(&mut t);
    let mut lead = None;
    if let Some((n, at)) = time_at(&t, 0, now, false).or_else(|| bare_lead(&t, now)) {
        lead = Some(at);
        t.drain(..n);
        polite |= strip(&mut t);
    }
    shape(&mut t, polite, now, k);
    (t, lead, trail)
}

fn known(head: &str, k: &Know) -> bool {
    let lists = [w::MSG, w::PLAY, w::OPEN, w::SEARCH, w::REMIND, w::SHOT, w::REMEMBER, w::GO, w::MACH, w::LET];
    lists.iter().any(|l| is(l, head)) || (k.site)(head)
}

pub(super) fn fix_head(t: &mut [Tok], k: &Know) {
    let Some(first) = t.first_mut() else { return };
    if first.low.chars().count() < FUZZY_MIN || known(&first.low, k) {
        return;
    }
    if let Some(canon) = w::FUZZY_HEADS.iter().find(|c| w::near(&first.low, c)) {
        first.low = (*canon).to_string();
    }
}

pub(super) fn trim_end(t: &mut Vec<Tok>, head: &str, k: &Know) {
    if is(w::MSG, head) || is(w::REMEMBER, head) {
        return;
    }
    let polite_only = is(w::PLAY, head) || is(w::SEARCH, head) || (k.site)(head);
    let list = if polite_only { w::POLITE_TAIL } else { w::TRAIL_FILL };
    while t.len() > 1 && t.last().is_some_and(|x| x.low.is_empty() || is(list, &x.low)) {
        t.pop();
    }
}
