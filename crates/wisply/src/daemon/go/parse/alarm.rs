use super::super::words::{self as w, is};
use super::when::time_at;
use super::{Tok, skip_all};

const MAX_LEAD: usize = 5;

fn start(t: &[Tok]) -> Option<usize> {
    let first = t.first()?;
    if is(w::WAKE, &first.low) {
        return t.get(1).filter(|x| is(w::ME, &x.low)).map(|_| skip_all(t, 1, &[w::ME, w::WAKE_NOISE]));
    }
    let q = (0..t.len().min(MAX_LEAD)).find(|&q| is(w::ALARM, &t[q].low))?;
    t[..q].iter().all(|x| is(w::ALARM_NOISE, &x.low)).then_some(q + 1)
}

fn trim_verbs(rest: &mut Vec<Tok>) {
    while rest.last().is_some_and(|x| is(w::ALARM_VERB, &x.low)) {
        rest.pop();
    }
    while rest.first().is_some_and(|x| is(w::ALARM_VERB, &x.low)) {
        rest.remove(0);
    }
}

fn drop_for(rest: &mut Vec<Tok>, now: f64) {
    let lead = rest.first().is_some_and(|x| is(w::ALARM_FOR, &x.low));
    if lead && time_at(rest, 0, now, true).is_none() && time_at(rest, 1, now, true).is_some() {
        rest.remove(0);
    }
}

pub(super) fn rewrite(t: &[Tok], now: f64) -> Option<Vec<Tok>> {
    let from = start(t)?;
    let mut rest: Vec<Tok> = t[from..].to_vec();
    trim_verbs(&mut rest);
    drop_for(&mut rest, now);
    let head = w::REMIND[0].to_string();
    rest.insert(0, Tok { raw: head.clone(), low: head, cut: false });
    Some(rest)
}

pub(super) fn text(label: &str) -> String {
    if label.trim().is_empty() { w::ALARM_TEXT.to_string() } else { format!("{}: {label}", w::ALARM_TEXT) }
}
