use std::path::Path;

use crate::config::Config;
use crate::fail::Fail;
use crate::guard::{self, Clock, Guard, System};
use crate::store::Store;
use crate::{CONFIG_FILE, STORE_FILE};

const MINUTE: i64 = 60;
const HOUR: i64 = 3600;
const TODAY: &str = "today";
const ROLLING: &str = "in 24 h";

fn span(seconds: f64) -> String {
    let whole = seconds.max(0.0).round() as i64;
    format!("{}:{:02}:{:02}", whole / HOUR, whole % HOUR / MINUTE, whole % MINUTE)
}

fn requests(guard: &Guard<'_>, scope: &str, period: &str) -> Result<String, Fail> {
    let usage = guard.usage_today(scope)?;
    let block = guard.blocked_until(scope)?.map(|until| format!(", blocked until {}", guard::local_minute(until)));
    Ok(format!("{scope}  {} of {} requests {period}{}", usage.used, usage.allowed, block.unwrap_or_default()))
}

fn report(config: &Config, store: &Store, clock: &dyn Clock) -> Result<String, Fail> {
    let guard = Guard::new(store, &config.limits, clock);
    let mut lines = vec![format!("video limits (free tier), day resets {}", guard::local_minute(guard.day_reset()))];
    for model in &config.gemini.model_ladder {
        lines.push(requests(&guard, model, TODAY)?);
    }
    let (used, allowed) = guard.youtube_today()?;
    lines.push(format!("youtube  {} of {} video time {TODAY}", span(used), span(allowed)));
    lines.push(requests(&guard, guard::GROQ, ROLLING)?);
    lines.push(format!("cache  {} answers", store.count_answers()?));
    Ok(lines.join("\n"))
}

pub fn run(folder: &Path) -> Result<String, Fail> {
    let config = Config::load(&folder.join(CONFIG_FILE))?;
    let store = Store::open(&folder.join(STORE_FILE), config.store.busy_timeout_seconds)?;
    report(&config, &store, &System)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::guard::Ask;

    const NOON: f64 = 1_768_507_200.0;
    const NEXT_DAY: f64 = 1_768_550_400.0;

    struct Fixed(f64);

    impl Clock for Fixed {
        fn now(&self) -> f64 {
            self.0
        }

        fn sleep(&self, _seconds: f64) {}
    }

    fn folder(name: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("wisply-video-limits-{name}-{}", std::process::id()))
    }

    #[test]
    fn video_time_reads_as_hours_minutes_seconds() {
        assert_eq!(span(0.0), "0:00:00");
        assert_eq!(span(759.6), "0:12:40");
        assert_eq!(span(23_040.0), "6:24:00");
        assert_eq!(span(-3.0), "0:00:00");
    }

    #[test]
    fn the_report_names_every_model_the_video_time_the_speech_service_and_the_cache() {
        let (config, store, clock) = (Config::standard(), Store::in_memory(), Fixed(NOON));
        let guard = Guard::new(&store, &config.limits, &clock);
        let ask = Ask { youtube_seconds: 760.0, ..Ask::default() };
        guard.reserve(&["gemini-3.8-flash"], &ask).unwrap();
        guard.note_rejection("gemini-3.6-flash", None, true).unwrap();
        let reset = guard::local_minute(NEXT_DAY);
        let lines = [
            format!("video limits (free tier), day resets {reset}"),
            "gemini-3.8-flash  1 of 16 requests today".to_string(),
            format!("gemini-3.6-flash  0 of 16 requests today, blocked until {reset}"),
            "gemini-3.5-flash-lite  0 of 400 requests today".to_string(),
            "youtube  0:12:40 of 6:24:00 video time today".to_string(),
            "groq  0 of 1600 requests in 24 h".to_string(),
            "cache  0 answers".to_string(),
        ];
        assert_eq!(report(&config, &store, &clock).unwrap(), lines.join("\n"));
    }

    #[test]
    fn a_first_run_creates_the_cache_and_reports_nothing_used() {
        let folder = folder("fresh");
        let text = run(&folder).unwrap();
        assert!(text.starts_with("video limits (free tier), day resets "), "{text}");
        assert!(
            text.contains("gemini-3.8-flash  0 of 16 requests today\n") && text.ends_with("cache  0 answers"),
            "{text}"
        );
        assert!(folder.join(STORE_FILE).is_file());
        assert_eq!(crate::run(&folder, &["limits".to_string()]), 0);
        assert_eq!(crate::run(&folder, &["nothing".to_string()]), 2);
        std::fs::remove_dir_all(&folder).unwrap();
    }

    #[test]
    fn broken_settings_are_reported_instead_of_guessed() {
        let folder = folder("broken");
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join(CONFIG_FILE), "[limits]\nusage_share = 7\n").unwrap();
        let fail = run(&folder).unwrap_err();
        assert!(fail.message.contains("usage_share") && fail.hint.contains(CONFIG_FILE), "{}", fail.text());
        assert_eq!(crate::run(&folder, &["limits".to_string()]), 1);
        std::fs::remove_dir_all(&folder).unwrap();
    }
}
