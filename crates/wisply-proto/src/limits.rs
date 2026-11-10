use std::time::Duration;

const SHORT: Duration = Duration::from_millis(3000);
const SESSION: Duration = Duration::from_millis(8000);
const CLEANUP: Duration = Duration::from_millis(12_000);
const START: Duration = Duration::from_millis(16_000);

pub fn hook_limit<S: AsRef<str>>(argv: &[S]) -> Option<Duration> {
    let word = |i: usize| argv.get(i).map_or("", AsRef::as_ref);
    match (word(0), word(1)) {
        ("suggest", _) | ("hook", "post") => Some(SHORT),
        ("hook", "start") => Some(START),
        ("hook", _) => Some(SESSION),
        ("done", _) if argv.iter().any(|a| a.as_ref() == "--quiet") => Some(CLEANUP),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_silent_hook_has_a_deadline_and_plain_commands_have_none() {
        assert_eq!(hook_limit(&["suggest"]), Some(SHORT));
        assert_eq!(hook_limit(&["suggest", "fix the build"]), Some(SHORT));
        assert_eq!(hook_limit(&["hook", "post"]), Some(SHORT));
        assert_eq!(hook_limit(&["hook", "start"]), Some(START));
        for event in ["stop", "end", "compact"] {
            assert_eq!(hook_limit(&["hook", event]), Some(SESSION), "{event}");
        }
        assert_eq!(hook_limit(&["done", "--quiet"]), Some(CLEANUP));
        for plain in [&["done"][..], &["recall", "x"], &["wait", "10m"], &[]] {
            assert_eq!(hook_limit(plain), None, "{plain:?}");
        }
    }
}
