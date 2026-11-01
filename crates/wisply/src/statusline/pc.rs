use std::fs;
use std::path::Path;

use serde_json::Value;

use super::render::{paint, tone};

const STALE_AFTER: u64 = 120;
const FILE: &str = "status.json";

pub fn field(home: &Path, now: u64) -> Option<String> {
    let text = fs::read_to_string(home.join("pc").join(FILE)).ok()?;
    part(&serde_json::from_str(&text).ok()?, now)
}

fn part(v: &Value, now: u64) -> Option<String> {
    let at = v.get("t")?.as_u64()?;
    if now.saturating_sub(at) > STALE_AFTER {
        return None;
    }
    let score = v.get("score")?.as_i64()?.clamp(0, 100);
    let brake = v.get("brake").and_then(Value::as_str).unwrap_or("");
    let label = if brake.is_empty() { format!("pc {score}") } else { format!("pc {score} {brake}") };
    Some(paint(tone(100 - score), &label))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::super::render::plain;
    use super::*;

    #[test]
    fn the_field_shows_score_and_biggest_brake_and_goes_quiet_when_old() {
        let v = json!({"t": 1000, "score": 72, "brake": "RAM 87 % (brave 6.1 GB)"});
        assert_eq!(plain(&part(&v, 1010).unwrap()), "pc 72 RAM 87 % (brave 6.1 GB)");
        assert_eq!(plain(&part(&json!({"t": 1000, "score": 100}), 1010).unwrap()), "pc 100");
        assert_eq!(part(&v, 1121), None);
        assert_eq!(part(&json!({"score": 1}), 1), None);
        assert_eq!(part(&Value::Null, 1), None);
    }

    #[test]
    fn a_missing_file_shows_nothing() {
        assert_eq!(field(Path::new("/definitely/not/here"), 5), None);
    }
}
