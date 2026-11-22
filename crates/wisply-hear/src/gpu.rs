use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::Duration;

use crate::config::Gpu;
use crate::spawn;

pub struct Watch {
    stop: Arc<AtomicBool>,
    handle: JoinHandle<(Option<f64>, Option<f64>)>,
}

pub fn parse_used(raw: &str) -> Option<f64> {
    raw.lines().next()?.trim().parse::<f64>().ok()
}

fn sample(query: &[String]) -> Option<f64> {
    let (program, args) = query.split_first()?;
    let output = spawn::quiet(program).args(args).stdin(Stdio::null()).output().ok()?;
    parse_used(&String::from_utf8_lossy(&output.stdout))
}

pub fn start(settings: &Gpu) -> Watch {
    let stop = Arc::new(AtomicBool::new(false));
    let flag = stop.clone();
    let query = settings.query.clone();
    let pause = Duration::from_millis(settings.poll_ms);
    let handle = std::thread::spawn(move || {
        let baseline = sample(&query);
        let mut peak = baseline;
        while !flag.load(Ordering::Relaxed) {
            std::thread::sleep(pause);
            if let Some(used) = sample(&query) {
                peak = Some(peak.map_or(used, |p| p.max(used)));
            }
        }
        (baseline, peak)
    });
    Watch { stop, handle }
}

impl Watch {
    pub fn finish(self) -> (Option<f64>, Option<f64>) {
        self.stop.store(true, Ordering::Relaxed);
        self.handle.join().unwrap_or((None, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_first_line() {
        assert_eq!(parse_used("1057\n"), Some(1057.0));
        assert_eq!(parse_used(" 12 \n34\n"), Some(12.0));
    }

    #[test]
    fn rejects_text() {
        assert_eq!(parse_used("N/A"), None);
        assert_eq!(parse_used(""), None);
    }
}
