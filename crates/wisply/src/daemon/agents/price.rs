const PER: f64 = 1_000_000.0;
const WRITE_X: f64 = 1.25;
const READ_X: f64 = 0.1;
const RATES: &[(&str, f64, f64)] = &[("opus", 5.0, 25.0), ("haiku", 1.0, 5.0), ("sonnet", 3.0, 15.0)];
const FALLBACK: (f64, f64) = (3.0, 15.0);

pub fn cost(model: &str, input: i64, write: i64, read: i64, out: i64) -> f64 {
    let low = model.to_lowercase();
    let (rin, rout) = RATES.iter().find(|(k, _, _)| low.contains(k)).map_or(FALLBACK, |(_, i, o)| (*i, *o));
    (input as f64 * rin + write as f64 * rin * WRITE_X + read as f64 * rin * READ_X + out as f64 * rout) / PER
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn families_are_priced_and_unknown_models_fall_back() {
        assert!((cost("claude-sonnet-5-5", 1_000_000, 0, 0, 0) - 3.0).abs() < 1e-9);
        assert!((cost("claude-opus-5-5", 0, 0, 0, 1_000_000) - 25.0).abs() < 1e-9);
        assert!((cost("x", 0, 0, 1_000_000, 0) - 0.3).abs() < 1e-9);
        assert!((cost("claude-haiku-5-5", 0, 1_000_000, 0, 0) - 1.25).abs() < 1e-9);
    }
}
