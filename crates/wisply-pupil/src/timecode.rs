pub fn format(seconds: f64) -> String {
    let whole = if seconds.is_finite() && seconds > 0.0 { seconds.floor() as u64 } else { 0 };
    let hours = whole / 3600;
    let minutes = (whole % 3600) / 60;
    let secs = whole % 60;
    if hours > 0 { format!("{hours}:{minutes:02}:{secs:02}") } else { format!("{minutes:02}:{secs:02}") }
}

pub fn stamp(template: &str, seconds: f64, places: usize) -> String {
    template.replace("{time}", &format(seconds)).replace("{seconds}", &format!("{seconds:.places$}"))
}

pub fn parse(text: &str) -> Option<f64> {
    let parts: Vec<&str> = text.trim().split(':').collect();
    if parts.is_empty() || parts.len() > 3 {
        return None;
    }
    let mut total = 0.0;
    for (index, part) in parts.iter().enumerate() {
        let value = part.trim().parse::<f64>().ok()?;
        let limited = index > 0 && value >= 60.0;
        if !value.is_finite() || value < 0.0 || limited {
            return None;
        }
        total = total * 60.0 + value;
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::{format, parse, stamp};

    #[test]
    fn labels_can_use_seconds_like_the_training_format() {
        assert_eq!(stamp("<{seconds} seconds>", 3.0, 1), "<3.0 seconds>");
        assert_eq!(stamp("<{seconds} seconds>", 61.26, 1), "<61.3 seconds>");
        assert_eq!(stamp("[{time}]", 61.0, 1), "[01:01]");
        assert_eq!(stamp("{seconds}s", 12.4, 0), "12s");
    }

    #[test]
    fn written_times_are_read_back() {
        assert_eq!(parse("00:00"), Some(0.0));
        assert_eq!(parse("03:10"), Some(190.0));
        assert_eq!(parse("1:12:42"), Some(4362.0));
        assert_eq!(parse(" 42 "), Some(42.0));
        assert_eq!(parse(&format(4362.7)), Some(4362.0));
    }

    #[test]
    fn broken_times_are_refused() {
        assert_eq!(parse(""), None);
        assert_eq!(parse("ab:cd"), None);
        assert_eq!(parse("01:75"), None);
        assert_eq!(parse("-1:00"), None);
        assert_eq!(parse("1:2:3:4"), None);
    }

    #[test]
    fn short_times_use_minutes_and_seconds() {
        assert_eq!(format(0.0), "00:00");
        assert_eq!(format(59.9), "00:59");
        assert_eq!(format(190.2), "03:10");
    }

    #[test]
    fn long_times_add_hours() {
        assert_eq!(format(3600.0), "1:00:00");
        assert_eq!(format(4362.7), "1:12:42");
    }

    #[test]
    fn bad_times_become_zero() {
        assert_eq!(format(-3.0), "00:00");
        assert_eq!(format(f64::NAN), "00:00");
    }
}
