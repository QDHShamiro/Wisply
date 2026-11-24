use wisply_core::local_parts;

use super::super::words::{self as w, is};
use super::Tok;

const CLOCK_SUFFIXES: [&str; 4] = ["am", "pm", "uhr", "h"];
const MAX_NUM_DIGITS: usize = 4;
const MAX_CLOCK_DIGITS: usize = 2;

fn is_num(s: &str) -> bool {
    (1..=MAX_NUM_DIGITS).contains(&s.len()) && s.bytes().all(|b| b.is_ascii_digit())
}

fn is_minutes(s: &str) -> bool {
    let b = s.as_bytes();
    b.len() == 2 && (b'0'..=b'5').contains(&b[0]) && b[1].is_ascii_digit()
}

fn split_span(s: &str) -> Option<(u64, &str)> {
    let at = s.find(|c: char| !c.is_ascii_digit())?;
    let (digits, unit) = s.split_at(at);
    let word = unit.chars().all(|c| c.is_ascii_lowercase() || matches!(c, 'ä' | 'ö' | 'ü'));
    (is_num(digits) && word).then(|| digits.parse().ok().map(|n| (n, unit)))?
}

fn split_clock(s: &str) -> Option<(u32, Option<u32>, Option<&str>)> {
    let end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    let (h, mut rest) = s.split_at(end);
    if h.is_empty() || h.len() > MAX_CLOCK_DIGITS {
        return None;
    }
    let mut mi = None;
    if let Some(tail) = rest.strip_prefix([':', '.']) {
        let digits = tail.find(|c: char| !c.is_ascii_digit()).unwrap_or(tail.len());
        if digits != MAX_CLOCK_DIGITS {
            return None;
        }
        mi = Some(tail[..digits].parse().ok()?);
        rest = &tail[digits..];
    }
    let word = if rest.is_empty() { None } else { Some(CLOCK_SUFFIXES.iter().find(|x| **x == rest)?.to_owned()) };
    Some((h.parse().ok()?, mi, word))
}

const DAY: f64 = 86_400.0;
const DAY_HOURS: u32 = 24;
const WEEK_DAYS: u32 = 7;
const DEFAULT_HOUR: u32 = 9;
const MAX_SUFFIX: usize = 2;
const NOON: u32 = 12;
const QUARTER: u32 = 15;
const THREE_QUARTERS: u32 = 45;
const HALF_HOUR: u32 = 30;
const HOUR: u32 = 60;

struct Clock {
    used: usize,
    h: u32,
    mi: u32,
    worded: bool,
    meridian: bool,
}

pub(super) fn day_start(now: f64) -> f64 {
    let [_, _, _, h, mi, s] = local_parts(now);
    now.floor() - f64::from(h * 3600 + mi * 60 + s)
}

pub(super) fn weekday(at: f64) -> u32 {
    const OFFSETS: [u32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let [y, m, d, ..] = local_parts(at);
    let y = if m < 3 { y - 1 } else { y };
    let v = (y + y / 4 - y / 100 + y / 400 + OFFSETS[(m.clamp(1, 12) - 1) as usize] + d) % WEEK_DAYS;
    if v == 0 { WEEK_DAYS } else { v }
}

fn stamp(now: f64, days: u32, h: u32, mi: u32) -> f64 {
    let t = day_start(now) + DAY * f64::from(days) + f64::from(h * 3600 + mi * 60);
    let got = local_parts(t)[3];
    if got == h { t } else { t + f64::from((h as i32 - got as i32) * 3600) }
}

fn resolve(now: f64, days: Option<u32>, h: u32, mi: u32) -> f64 {
    let t = stamp(now, days.unwrap_or(0), h, mi);
    if days.is_none() && t <= now { stamp(now, 1, h, mi) } else { t }
}

fn num_of(low: &str) -> Option<u64> {
    if is_num(low) {
        return low.parse().ok();
    }
    w::NUMBERS.iter().find(|(word, _)| w::same(word, low)).map(|(_, n)| *n)
}

fn hour_of(low: &str) -> Option<u32> {
    if is_num(low) {
        return low.parse().ok().filter(|h| (1..=NOON).contains(h));
    }
    if is(w::NUM_ARTICLES, low) {
        return None;
    }
    num_of(low).and_then(|n| u32::try_from(n).ok()).filter(|h| (1..=NOON).contains(h))
}

fn unit_of(low: &str) -> Option<u64> {
    w::UNITS.iter().find(|(names, _)| is(names, low)).map(|(_, s)| *s)
}

fn quantity(t: &[Tok], j: usize) -> Option<(usize, u64)> {
    let a = &t.get(j)?.low;
    if let Some((n, unit)) = split_span(a)
        && let Some(u) = unit_of(unit)
    {
        return Some((1, n * u));
    }
    if is(w::HALF, a) {
        let k = j + 1 + usize::from(t.get(j + 1).is_some_and(|x| is(w::NUM_ARTICLES, &x.low)));
        return Some((k + 1 - j, unit_of(&t.get(k)?.low)? / 2));
    }
    let n = num_of(a)?;
    let next = &t.get(j + 1)?.low;
    if n == 1 && is(w::HALF, next) {
        return Some((3, unit_of(&t.get(j + 2)?.low)? / 2));
    }
    Some((2, n * unit_of(next)?))
}

pub(super) fn span(t: &[Tok], i: usize, now: f64) -> Option<(usize, f64)> {
    let (n, secs) = quantity(t, i + 1)?;
    Some((n + 1, now + secs as f64))
}

fn meridian(t: &[Tok], i: usize, used: usize, h: &mut u32) -> (usize, bool) {
    let (mut taken, mut seen) = (0, false);
    while taken < MAX_SUFFIX {
        let Some(x) = t.get(i + used + taken) else { break };
        let (pm, am) = (is(w::PM, &x.low), is(w::AM, &x.low));
        if !pm && !am && !is(w::HOUR_WORD, &x.low) {
            break;
        }
        if pm && *h < NOON {
            *h += NOON;
        } else if am && *h == NOON {
            *h = 0;
        }
        seen |= pm || am;
        taken += 1;
    }
    (taken, seen)
}

fn digits(t: &[Tok], i: usize) -> Option<Clock> {
    let (mut h, minutes, word) = split_clock(&t.get(i)?.low)?;
    let mut mi = minutes.unwrap_or(0);
    let mut used = 1;
    let mut spoken = word.is_some_and(|s| is(w::HOUR_WORD, s));
    if (spoken || t.get(i + 1).is_some_and(|x| is(w::HOUR_WORD, &x.low)))
        && minutes.is_none()
        && let Some(x) = t.get(i + 1 + usize::from(!spoken)).filter(|x| is_minutes(&x.low))
    {
        mi = x.low.parse().ok()?;
        used += 1 + usize::from(!spoken);
        spoken = true;
    }
    let mut seen = false;
    match word {
        Some(s) if is(w::PM, s) && h < NOON => (h, seen) = (h + NOON, true),
        Some(s) if is(w::AM, s) => (h, seen) = (if h == NOON { 0 } else { h }, true),
        _ => {}
    }
    let (taken, more) = meridian(t, i, used, &mut h);
    let worded = word.is_some() || minutes.is_some() || spoken || taken > 0;
    (h < DAY_HOURS && mi < HOUR).then_some(Clock { used: used + taken, h, mi, worded, meridian: seen || more })
}

fn before(h: u32) -> u32 {
    if h == 1 { NOON } else { h - 1 }
}

fn spoken(t: &[Tok], i: usize) -> Option<Clock> {
    let first = &t.get(i)?.low;
    let read = |at: usize| t.get(at).and_then(|x| hour_of(&x.low));
    let (used, mut h, mi) = if is(w::MIDNIGHT, first) {
        return Some(Clock { used: 1, h: 0, mi: 0, worded: false, meridian: true });
    } else if is(w::NOON_WORD, first) {
        return Some(Clock { used: 1, h: NOON, mi: 0, worded: false, meridian: true });
    } else if is(w::HALF_CLOCK, first) {
        match t.get(i + 1) {
            Some(x) if is(w::PAST, &x.low) => (3, read(i + 2)?, HALF_HOUR),
            _ => (2, before(read(i + 1)?), HALF_HOUR),
        }
    } else if is(w::THREE_QUARTER, first) {
        (2, before(read(i + 1)?), THREE_QUARTERS)
    } else if is(w::QUARTER_CLOCK, first) {
        let dir = &t.get(i + 1)?.low;
        if is(w::PAST, dir) {
            (3, read(i + 2)?, QUARTER)
        } else if is(w::TO, dir) {
            (3, before(read(i + 2)?), THREE_QUARTERS)
        } else {
            return None;
        }
    } else {
        let n = num_of(first).filter(|n| (1..60).contains(n) && !is(w::NUM_ARTICLES, first))?;
        let dir = &t.get(i + 1)?.low;
        let n = u32::try_from(n).ok()?;
        if is(w::PAST, dir) {
            (3, read(i + 2)?, n)
        } else if is(w::TO, dir) {
            (3, before(read(i + 2)?), HOUR - n)
        } else {
            return None;
        }
    };
    let (taken, seen) = meridian(t, i, used, &mut h);
    Some(Clock { used: used + taken, h, mi, worded: true, meridian: seen })
}

fn bare_hour(t: &[Tok], i: usize) -> Option<Clock> {
    let h = hour_of(&t.get(i)?.low)?;
    let mut h = h;
    let (taken, seen) = meridian(t, i, 1, &mut h);
    Some(Clock { used: 1 + taken, h, mi: 0, worded: false, meridian: seen })
}

fn clock(t: &[Tok], i: usize, lead: bool) -> Option<Clock> {
    digits(t, i).or_else(|| spoken(t, i)).or_else(|| if lead { bare_hour(t, i) } else { None })
}

fn part_at(t: &[Tok], j: usize) -> Option<(u32, bool)> {
    let x = &t.get(j)?.low;
    w::PARTS.iter().find(|(names, _, _)| is(names, x)).map(|(_, h, pm)| (*h, *pm))
}

fn day_at(t: &[Tok], j: usize, now: f64) -> Option<(usize, u32)> {
    let first = &t.get(j)?.low;
    if is(w::TODAY, first) {
        return Some((1, 0));
    }
    if is(w::AFTER_TOMORROW, first) {
        return Some((1, 2));
    }
    if is(w::TOMORROW, first) {
        return Some((1, 1));
    }
    if is(w::THIS_DAY, first) && part_at(t, j + 1).is_some() {
        return Some((1, 0));
    }
    let k = j + usize::from(is(w::DAY_LEAD, first));
    let name = &t.get(k)?.low;
    let want = w::WEEKDAYS.iter().find(|(names, _)| is(names, name)).map(|(_, d)| *d)?;
    Some((k + 1 - j, (want + WEEK_DAYS - weekday(now) - 1) % WEEK_DAYS + 1))
}

fn evening_fix(c: &mut Clock, part: Option<(u32, bool)>) {
    if let Some((_, true)) = part
        && !c.meridian
        && c.h < NOON
    {
        c.h += NOON;
    }
}

pub(super) fn time_at(t: &[Tok], i: usize, now: f64, bare: bool) -> Option<(usize, f64)> {
    let first = &t.get(i)?.low;
    if is(w::IN, first) {
        return span(t, i, now);
    }
    let tonight = is(w::TONIGHT, first);
    let day = if tonight { Some((1, 0)) } else { day_at(t, i, now) };
    let mut j = i + day.map_or(0, |d| d.0);
    let part = if tonight { Some((w::NIGHT_HOUR, true)) } else { day.and_then(|_| part_at(t, j)) };
    if part.is_some() && !tonight {
        j += 1;
    }
    let lead = t.get(j).is_some_and(|x| is(w::AT, &x.low));
    let found = clock(t, j + usize::from(lead), lead).filter(|c| c.worded || lead || part.is_some());
    let days = day.map(|d| d.1);
    if let Some(mut c) = found {
        evening_fix(&mut c, part);
        return Some((j + usize::from(lead) + c.used - i, resolve(now, days, c.h, c.mi)));
    }
    if let Some((h, _)) = part {
        return Some((j - i, resolve(now, days, h, 0)));
    }
    (bare && day.is_some()).then(|| (j - i, resolve(now, days, DEFAULT_HOUR, 0)))
}

pub(super) fn tail_time(t: &mut Vec<Tok>, from: usize, now: f64, bare: bool) -> Option<f64> {
    for start in from..t.len() {
        if let Some((n, at)) = time_at(t, start, now, bare)
            && start + n == t.len()
        {
            t.truncate(start);
            return Some(at);
        }
    }
    None
}

pub(super) fn inner_time(t: &mut Vec<Tok>, now: f64) -> Option<f64> {
    for start in 1..t.len() {
        if is(w::IN, &t[start].low)
            && let Some((n, at)) = span(t, start, now)
        {
            t.drain(start..start + n);
            return Some(at);
        }
    }
    None
}
