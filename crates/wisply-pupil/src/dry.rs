use serde_json::{Value, json};

use crate::config::Sampling;

pub fn set(sent: &mut Value, sampling: &Sampling, multiplier: f64) {
    let Some(map) = sent.as_object_mut() else {
        return;
    };
    map.retain(|key, _| !key.starts_with("dry_"));
    if multiplier <= 0.0 {
        return;
    }
    map.insert("dry_multiplier".into(), json!(multiplier));
    map.insert("dry_base".into(), json!(sampling.dry_base));
    map.insert("dry_allowed_length".into(), json!(sampling.dry_allowed_length));
    map.insert("dry_penalty_last_n".into(), json!(sampling.dry_penalty_last_n));
    if !sampling.dry_sequence_breakers.is_empty() {
        map.insert("dry_sequence_breakers".into(), json!(sampling.dry_sequence_breakers));
    }
}

#[cfg(test)]
mod tests {
    use super::set;
    use crate::config::Config;
    use serde_json::json;

    fn config() -> Config {
        toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses")
    }

    #[test]
    fn a_positive_multiplier_sends_every_dry_field() {
        let config = config();
        let mut sent = json!({"model": "2b"});
        set(&mut sent, &config.sampling, 0.8);
        assert_eq!(sent["dry_multiplier"], 0.8);
        assert_eq!(sent["dry_base"], config.sampling.dry_base);
        assert_eq!(sent["dry_allowed_length"], config.sampling.dry_allowed_length);
        assert_eq!(sent["model"], "2b");
    }

    #[test]
    fn zero_takes_every_dry_field_away() {
        let config = config();
        let mut sent = json!({"model": "2b"});
        set(&mut sent, &config.sampling, 0.8);
        set(&mut sent, &config.sampling, 0.0);
        let keys: Vec<&String> = sent.as_object().expect("object").keys().collect();
        assert_eq!(keys, ["model"]);
    }
}
