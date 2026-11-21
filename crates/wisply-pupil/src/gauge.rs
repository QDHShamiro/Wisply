use std::sync::mpsc::{self, RecvTimeoutError, Sender};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crate::config::Harness;
use crate::spawn;

#[derive(Debug, Default)]
pub struct Readings {
    pub vram: Vec<f64>,
    pub ram: Vec<f64>,
    pub power: Vec<(f64, f64)>,
    pub misses: u32,
}

impl Readings {
    pub fn energy_wh(&self) -> f64 {
        let joules: f64 = self.power.windows(2).map(|w| (w[1].0 - w[0].0) * (w[0].1 + w[1].1) / 2.0).sum();
        joules / 3600.0
    }
}

pub fn peak(values: &[f64]) -> Option<f64> {
    values.iter().copied().reduce(f64::max)
}

pub struct Gauge {
    stop: Sender<()>,
    thread: JoinHandle<Readings>,
}

fn output(args: &[String]) -> Option<String> {
    let (program, rest) = args.split_first()?;
    let done = spawn::quiet(program).args(rest).output().ok()?;
    Some(String::from_utf8_lossy(&done.stdout).into_owned())
}

pub fn gpu(query: &[String]) -> Option<(f64, f64)> {
    let raw = output(query)?;
    let cells: Vec<f64> =
        raw.trim().lines().next()?.split(',').map(|c| c.trim().parse::<f64>()).collect::<Result<_, _>>().ok()?;
    match cells[..] {
        [used, watts] => Some((used, watts)),
        _ => None,
    }
}

pub fn kib(cell: &str) -> f64 {
    let digits: String = cell.chars().filter(char::is_ascii_digit).collect();
    digits.parse::<f64>().unwrap_or(0.0)
}

fn resident(harness: &Harness) -> Option<f64> {
    let mut total = 0.0;
    for name in &harness.processes {
        let args: Vec<String> = harness.ram_query.iter().map(|a| a.replace("{name}", name)).collect();
        for row in output(&args)?.lines() {
            let cells: Vec<&str> = row.trim().trim_matches('"').split("\",\"").collect();
            if cells.len() >= 5 {
                total += kib(cells[4]) / 1024.0;
            }
        }
    }
    Some(total)
}

fn sample(harness: &Harness, begun: Instant, readings: &mut Readings) {
    let at = begun.elapsed().as_secs_f64();
    match (gpu(&harness.gpu_query), resident(harness)) {
        (Some((used, watts)), Some(ram)) => {
            readings.vram.push(used);
            readings.power.push((at, watts));
            readings.ram.push(ram);
        }
        _ => readings.misses += 1,
    }
}

impl Gauge {
    pub fn start(harness: &Harness) -> Gauge {
        let (stop, wait) = mpsc::channel::<()>();
        let harness = harness.clone();
        let every = Duration::from_secs_f64(harness.sample_seconds.max(0.1));
        let thread = std::thread::spawn(move || {
            let begun = Instant::now();
            let mut readings = Readings::default();
            loop {
                sample(&harness, begun, &mut readings);
                match wait.recv_timeout(every) {
                    Err(RecvTimeoutError::Timeout) => continue,
                    _ => break,
                }
            }
            readings
        });
        Gauge { stop, thread }
    }

    pub fn finish(self) -> Readings {
        let _ = self.stop.send(());
        self.thread.join().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::{Readings, kib, peak};

    #[test]
    fn energy_is_the_area_under_the_power_curve() {
        let readings = Readings { power: vec![(0.0, 100.0), (1800.0, 100.0), (3600.0, 300.0)], ..Readings::default() };
        assert!((readings.energy_wh() - 150.0).abs() < 1e-9);
    }

    #[test]
    fn tasklist_memory_cells_become_kib() {
        assert_eq!(kib("2.941.312 K"), 2_941_312.0);
        assert_eq!(kib("1,024 K"), 1024.0);
        assert_eq!(peak(&[3.0, 9.0, 4.0]), Some(9.0));
        assert_eq!(peak(&[]), None);
    }
}
