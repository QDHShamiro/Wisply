use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, OptionalExtension, params};

use crate::config::Limits;
use crate::fail::{Fail, Kind};
use crate::pacific;
use crate::store::Store;

pub const GROQ: &str = "groq";
pub const YOUTUBE: &str = "youtube";
pub const REQUESTS_PER_MINUTE: &str = "requests_per_minute";
pub const TOKENS_PER_MINUTE: &str = "tokens_per_minute";
pub const REQUESTS_PER_DAY: &str = "requests_per_day";
pub const YOUTUBE_SECONDS_PER_DAY: &str = "youtube_seconds_per_day";
pub const AUDIO_SECONDS_PER_HOUR: &str = "audio_seconds_per_hour";
pub const AUDIO_SECONDS_PER_DAY: &str = "audio_seconds_per_day";

const MINUTE: f64 = 60.0;
const HOUR: f64 = 3600.0;
const DAY: f64 = 86_400.0;
const WAIT_MARGIN: f64 = 0.05;
const ROUNDING_SLACK: f64 = 1e-9;
const BLOCKED: &str = "blocked after a rejected request";
const YOUTUBE_TIME: &str = "YouTube video time per day";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Requests,
    Tokens,
    YoutubeSeconds,
    AudioSeconds,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rule {
    pub name: &'static str,
    pub field: Field,
    pub limit: f64,
    pub window: Option<f64>,
    pub scope: String,
    pub all_scopes: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ask {
    pub requests: i64,
    pub tokens: i64,
    pub youtube_seconds: f64,
    pub audio_seconds: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reservation {
    pub id: i64,
    pub scope: String,
    pub ask: Ask,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Refusal {
    pub scope: String,
    pub reason: String,
    pub free_at: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DayUsage {
    pub used: i64,
    pub allowed: i64,
}

pub trait Clock {
    fn now(&self) -> f64;
    fn sleep(&self, seconds: f64);
}

pub struct System;

pub struct Guard<'a> {
    store: &'a Store,
    limits: &'a Limits,
    clock: &'a dyn Clock,
}

enum Attempt {
    Reserved(Reservation),
    Refused(Refusal),
}

impl Field {
    fn column(self) -> &'static str {
        match self {
            Field::Requests => "requests",
            Field::Tokens => "tokens",
            Field::YoutubeSeconds => "youtube_seconds",
            Field::AudioSeconds => "audio_seconds",
        }
    }
}

impl Default for Ask {
    fn default() -> Ask {
        Ask { requests: 1, tokens: 0, youtube_seconds: 0.0, audio_seconds: 0.0 }
    }
}

impl Ask {
    fn amount(&self, field: Field) -> f64 {
        match field {
            Field::Requests => self.requests as f64,
            Field::Tokens => self.tokens as f64,
            Field::YoutubeSeconds => self.youtube_seconds,
            Field::AudioSeconds => self.audio_seconds,
        }
    }
}

impl Clock for System {
    fn now(&self) -> f64 {
        SystemTime::now().duration_since(UNIX_EPOCH).map_or(0.0, |since| since.as_secs_f64())
    }

    fn sleep(&self, seconds: f64) {
        std::thread::sleep(Duration::from_secs_f64(seconds.max(0.0)));
    }
}

fn rule(name: &'static str, field: Field, limit: f64, window: Option<f64>, scope: &str) -> Rule {
    Rule { name, field, limit, window, scope: scope.to_string(), all_scopes: false }
}

fn since(rule: &Rule, moment: f64) -> f64 {
    rule.window.map_or_else(|| pacific::day_start(moment), |window| moment - window)
}

fn pair(row: &rusqlite::Row<'_>) -> rusqlite::Result<(f64, f64)> {
    Ok((row.get(0)?, row.get(1)?))
}

fn describe(refusal: &Refusal) -> String {
    if refusal.scope == YOUTUBE { YOUTUBE_TIME.to_string() } else { format!("{} of {}", refusal.reason, refusal.scope) }
}

fn later(first: Option<Refusal>, second: Refusal) -> Refusal {
    match first {
        None => second,
        Some(first) => match (first.free_at, second.free_at) {
            (Some(held), Some(new)) if new <= held => first,
            (Some(_), _) => second,
            (None, _) => first,
        },
    }
}

pub fn local_minute(at: f64) -> String {
    let [year, month, day, hour, minute, _] = wisply_core::local_parts(at);
    format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}")
}

fn limit_error(refusals: &[Refusal]) -> Fail {
    let Some(first) = refusals.first() else {
        return Fail::new(Kind::Limit, "no model was given to send the request to");
    };
    let timed = refusals.iter().filter_map(|refusal| refusal.free_at.map(|at| (at, refusal)));
    match timed.min_by(|one, other| one.0.total_cmp(&other.0)) {
        Some((at, soonest)) => {
            let message = format!("the free limit is used up ({}), nothing was sent", describe(soonest));
            Fail::hint(Kind::Limit, message, format!("wait until {}", local_minute(at)))
        }
        None => {
            let message =
                format!("the request is larger than the limit allows ({}), nothing was sent", describe(first));
            Fail::hint(Kind::Limit, message, "use a shorter section or a lower quality")
        }
    }
}

pub(crate) fn blocked(scope: &str, until: f64) -> Fail {
    limit_error(&[Refusal { scope: scope.to_string(), reason: BLOCKED.to_string(), free_at: Some(until) }])
}

impl<'a> Guard<'a> {
    pub fn new(store: &'a Store, limits: &'a Limits, clock: &'a dyn Clock) -> Guard<'a> {
        Guard { store, limits, clock }
    }

    fn youtube_rule(&self) -> Rule {
        let limit = self.limits.youtube_seconds_per_day;
        Rule { all_scopes: true, ..rule(YOUTUBE_SECONDS_PER_DAY, Field::YoutubeSeconds, limit, None, YOUTUBE) }
    }

    pub fn rules_for(&self, scope: &str) -> Result<Vec<Rule>, Fail> {
        if scope == GROQ {
            let groq = &self.limits.groq;
            return Ok(vec![
                rule(REQUESTS_PER_MINUTE, Field::Requests, groq.requests_per_minute, Some(MINUTE), scope),
                rule(REQUESTS_PER_DAY, Field::Requests, groq.requests_per_day, Some(DAY), scope),
                rule(AUDIO_SECONDS_PER_HOUR, Field::AudioSeconds, groq.audio_seconds_per_hour, Some(HOUR), scope),
                rule(AUDIO_SECONDS_PER_DAY, Field::AudioSeconds, groq.audio_seconds_per_day, Some(DAY), scope),
            ]);
        }
        let Some(model) = self.limits.models.get(scope) else {
            let message = format!("no limits are configured for model {scope}");
            return Err(Fail::hint(Kind::Input, message, "add a [limits.models] entry to video.toml"));
        };
        Ok(vec![
            rule(REQUESTS_PER_MINUTE, Field::Requests, model.requests_per_minute, Some(MINUTE), scope),
            rule(TOKENS_PER_MINUTE, Field::Tokens, model.tokens_per_minute, Some(MINUTE), scope),
            rule(REQUESTS_PER_DAY, Field::Requests, model.requests_per_day, None, scope),
            self.youtube_rule(),
        ])
    }

    fn allowed(&self, conn: &Connection, rule: &Rule) -> Result<f64, Fail> {
        let sql = "SELECT value FROM quota_learned WHERE scope = ?1 AND rule = ?2";
        let learned: Option<f64> = conn.query_row(sql, params![rule.scope, rule.name], |row| row.get(0)).optional()?;
        let share = learned.map_or(rule.limit, |value| value.min(rule.limit)) * self.limits.usage_share;
        Ok(if rule.field == Field::Requests { (share + ROUNDING_SLACK).floor().max(1.0) } else { share })
    }

    fn events(&self, conn: &Connection, rule: &Rule, since: f64) -> Result<Vec<(f64, f64)>, Fail> {
        let column = rule.field.column();
        let sql = format!("SELECT at, {column} FROM quota_event WHERE at >= ?1 AND (?2 OR scope = ?3) ORDER BY at");
        let mut statement = conn.prepare(&sql)?;
        let rows = statement.query_map(params![since, rule.all_scopes, rule.scope], pair)?;
        Ok(rows.collect::<Result<Vec<(f64, f64)>, _>>()?)
    }

    fn check_rule(&self, conn: &Connection, rule: &Rule, amount: f64, moment: f64) -> Result<Option<Refusal>, Fail> {
        if amount <= 0.0 {
            return Ok(None);
        }
        let refusal =
            |free_at| Some(Refusal { scope: rule.scope.clone(), reason: rule.name.replace('_', " "), free_at });
        let allowed = self.allowed(conn, rule)?;
        if amount > allowed {
            return Ok(refusal(None));
        }
        let events = self.events(conn, rule, since(rule, moment))?;
        let mut used: f64 = events.iter().map(|(_, spent)| spent).sum();
        if used + amount <= allowed {
            return Ok(None);
        }
        let Some(window) = rule.window else {
            return Ok(refusal(Some(pacific::next_day_start(moment))));
        };
        for (at, spent) in events {
            used -= spent;
            if used + amount <= allowed {
                return Ok(refusal(Some(at + window)));
            }
        }
        Ok(refusal(Some(moment + window)))
    }

    fn check_block(&self, conn: &Connection, scope: &str, moment: f64) -> Result<Option<Refusal>, Fail> {
        let sql = "SELECT until FROM quota_block WHERE scope = ?1 AND until > ?2";
        let until: Option<f64> = conn.query_row(sql, params![scope, moment], |row| row.get(0)).optional()?;
        Ok(until.map(|until| Refusal { scope: scope.to_string(), reason: BLOCKED.to_string(), free_at: Some(until) }))
    }

    fn attempt(&self, scope: &str, ask: &Ask) -> Result<Attempt, Fail> {
        let rules = self.rules_for(scope)?;
        let tx = self.store.write()?;
        let moment = self.clock.now();
        let mut worst = self.check_block(&tx, scope, moment)?;
        for rule in &rules {
            if let Some(refusal) = self.check_rule(&tx, rule, ask.amount(rule.field), moment)? {
                worst = Some(later(worst, refusal));
            }
        }
        if let Some(refusal) = worst {
            return Ok(Attempt::Refused(refusal));
        }
        let sql = "INSERT INTO quota_event (scope, at, requests, tokens, youtube_seconds, audio_seconds) \
                   VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
        tx.execute(sql, params![scope, moment, ask.requests, ask.tokens, ask.youtube_seconds, ask.audio_seconds])?;
        let id = tx.last_insert_rowid();
        tx.commit()?;
        Ok(Attempt::Reserved(Reservation { id, scope: scope.to_string(), ask: *ask }))
    }

    pub fn reserve(&self, scopes: &[&str], ask: &Ask) -> Result<Reservation, Fail> {
        let deadline = self.clock.now() + self.limits.max_wait_seconds;
        let mut refusals = Vec::new();
        for scope in scopes {
            loop {
                let refusal = match self.attempt(scope, ask)? {
                    Attempt::Reserved(reservation) => return Ok(reservation),
                    Attempt::Refused(refusal) => refusal,
                };
                match refusal.free_at {
                    Some(free_at) if free_at <= deadline => {
                        self.clock.sleep((free_at - self.clock.now()).max(0.0) + WAIT_MARGIN);
                    }
                    _ => {
                        refusals.push(refusal);
                        break;
                    }
                }
            }
        }
        Err(limit_error(&refusals))
    }

    pub fn settle(&self, reservation: &Reservation, tokens: i64) -> Result<(), Fail> {
        self.store
            .read()
            .execute("UPDATE quota_event SET tokens = ?1 WHERE id = ?2", params![tokens, reservation.id])?;
        Ok(())
    }

    pub fn cancel(&self, reservation: &Reservation) -> Result<(), Fail> {
        self.store.read().execute("DELETE FROM quota_event WHERE id = ?1", params![reservation.id])?;
        Ok(())
    }

    pub fn note_rejection(&self, scope: &str, retry_after: Option<f64>, until_reset: bool) -> Result<f64, Fail> {
        let moment = self.clock.now();
        let pause =
            retry_after.filter(|seconds| *seconds > 0.0).map_or(self.limits.rejection_block_seconds, |s| s.min(DAY));
        let until = if until_reset { pacific::next_day_start(moment) } else { moment + pause };
        let tx = self.store.write()?;
        let sql = "INSERT INTO quota_block (scope, until, reason) VALUES (?1, ?2, ?3) \
                   ON CONFLICT (scope) DO UPDATE SET until = MAX(until, excluded.until)";
        tx.execute(sql, params![scope, until, BLOCKED])?;
        let stored =
            tx.query_row("SELECT until FROM quota_block WHERE scope = ?1", params![scope], |row| row.get(0))?;
        tx.commit()?;
        Ok(stored)
    }

    pub fn learn_limit(&self, scope: &str, rule: &str, value: f64) -> Result<(), Fail> {
        if value <= 0.0 || !value.is_finite() {
            return Ok(());
        }
        let sql = "INSERT INTO quota_learned (scope, rule, value, learned_at) VALUES (?1, ?2, ?3, ?4) \
                   ON CONFLICT (scope, rule) DO UPDATE SET value = MIN(value, excluded.value), \
                   learned_at = excluded.learned_at";
        self.store.read().execute(sql, params![scope, rule, value, self.clock.now()])?;
        Ok(())
    }

    fn spent(&self, rule: &Rule) -> Result<(f64, f64), Fail> {
        let conn = self.store.read();
        let events = self.events(conn, rule, since(rule, self.clock.now()))?;
        Ok((events.iter().map(|(_, spent)| spent).sum(), self.allowed(conn, rule)?))
    }

    pub fn usage_today(&self, scope: &str) -> Result<DayUsage, Fail> {
        let rules = self.rules_for(scope)?;
        let Some(daily) = rules.iter().find(|rule| rule.name == REQUESTS_PER_DAY) else {
            return Err(Fail::new(Kind::Input, format!("{scope} has no daily request limit")));
        };
        let (used, allowed) = self.spent(daily)?;
        Ok(DayUsage { used: used as i64, allowed: allowed as i64 })
    }

    pub fn youtube_today(&self) -> Result<(f64, f64), Fail> {
        self.spent(&self.youtube_rule())
    }

    pub fn blocked_until(&self, scope: &str) -> Result<Option<f64>, Fail> {
        Ok(self.check_block(self.store.read(), scope, self.clock.now())?.and_then(|refusal| refusal.free_at))
    }

    pub fn day_reset(&self) -> f64 {
        pacific::next_day_start(self.clock.now())
    }

    pub fn prune(&self) -> Result<(), Fail> {
        let moment = self.clock.now();
        let tx = self.store.write()?;
        tx.execute("DELETE FROM quota_event WHERE at < ?1", params![moment - self.limits.history_days * DAY])?;
        tx.execute("DELETE FROM quota_block WHERE until <= ?1", params![moment])?;
        Ok(tx.commit()?)
    }
}

#[cfg(test)]
#[path = "guard_tests.rs"]
mod tests;
