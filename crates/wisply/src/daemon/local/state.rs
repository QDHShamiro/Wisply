use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

use super::Kind;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Slot {
    pub pid: u32,
    pub port: u16,
    pub used: f64,
    pub mb: u64,
    pub started: f64,
    pub flight: u32,
    pub flight_at: f64,
}

impl Slot {
    pub fn busy(&self, now: f64) -> bool {
        super::plan::flying(self.flight, self.flight_at, now)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct State {
    pub llm: Option<Slot>,
    pub asr: Option<Slot>,
    pub seen: [u64; 2],
}

pub fn now() -> f64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0.0, |d| d.as_secs_f64())
}

pub fn dir(home: &Path) -> PathBuf {
    home.join("local")
}

fn file(home: &Path) -> PathBuf {
    dir(home).join("state.json")
}

fn slot_json(s: &Slot) -> Value {
    json!({"pid": s.pid, "port": s.port, "used": s.used, "mb": s.mb, "started": s.started, "flight": s.flight, "flight_at": s.flight_at})
}

fn slot_from(v: &Value) -> Option<Slot> {
    let n = |k: &str| v.get(k).and_then(Value::as_f64).unwrap_or(0.0);
    let pid = u32::try_from(v.get("pid")?.as_u64()?).ok().filter(|p| *p > 0)?;
    let port = u16::try_from(v.get("port")?.as_u64()?).ok().filter(|p| *p > 0)?;
    Some(Slot {
        pid,
        port,
        used: n("used"),
        mb: n("mb") as u64,
        started: n("started"),
        flight: n("flight") as u32,
        flight_at: n("flight_at"),
    })
}

impl State {
    pub fn parse(text: &str) -> State {
        let v: Value = serde_json::from_str(text).unwrap_or(Value::Null);
        let seen = |k: &str| v.get("seen").and_then(|s| s.get(k)).and_then(Value::as_u64).unwrap_or(0);
        State {
            llm: v.get("llm").and_then(slot_from),
            asr: v.get("asr").and_then(slot_from),
            seen: [seen("llm"), seen("asr")],
        }
    }

    pub fn render(&self) -> String {
        json!({
            "llm": self.llm.as_ref().map(slot_json),
            "asr": self.asr.as_ref().map(slot_json),
            "seen": {"llm": self.seen[0], "asr": self.seen[1]},
        })
        .to_string()
    }

    pub fn slot(&self, kind: Kind) -> Option<&Slot> {
        match kind {
            Kind::Llm => self.llm.as_ref(),
            Kind::Asr => self.asr.as_ref(),
        }
    }

    pub fn slot_mut(&mut self, kind: Kind) -> Option<&mut Slot> {
        match kind {
            Kind::Llm => self.llm.as_mut(),
            Kind::Asr => self.asr.as_mut(),
        }
    }

    pub fn put(&mut self, kind: Kind, slot: Option<Slot>) {
        match kind {
            Kind::Llm => self.llm = slot,
            Kind::Asr => self.asr = slot,
        }
    }

    pub fn running(&self) -> Vec<(Kind, Slot)> {
        [Kind::Llm, Kind::Asr].into_iter().filter_map(|k| self.slot(k).map(|s| (k, s.clone()))).collect()
    }
}

pub fn load(home: &Path) -> State {
    fs::read_to_string(file(home)).map(|t| State::parse(&t)).unwrap_or_default()
}

pub fn save(home: &Path, state: &State) {
    let _ = fs::create_dir_all(dir(home));
    let _ = wisply_core::save(file(home), state.render());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_state_survives_a_round_trip_and_garbage() {
        let mut s = State::default();
        s.put(
            Kind::Asr,
            Some(Slot { pid: 12, port: 4000, used: 5.5, mb: 300, started: 1.0, flight: 2, flight_at: 3.5 }),
        );
        s.seen = [640, 0];
        let back = State::parse(&s.render());
        assert_eq!(back, s);
        assert_eq!(back.running().len(), 1);
        assert_eq!(State::parse("nonsense"), State::default());
        assert_eq!(State::parse("{\"llm\": {\"pid\": 0, \"port\": 1}}").llm, None);
    }
}
