use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Bg<T> {
    value: Arc<Mutex<Option<T>>>,
    busy: Arc<AtomicBool>,
    at: u64,
}

impl<T> Default for Bg<T> {
    fn default() -> Self {
        Bg { value: Arc::new(Mutex::new(None)), busy: Arc::new(AtomicBool::new(false)), at: 0 }
    }
}

impl<T: Clone + Send + 'static> Bg<T> {
    pub fn new() -> Bg<T> {
        Bg::default()
    }

    pub fn get(&self) -> Option<T> {
        self.value.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

    pub fn put(&mut self, v: T, now: u64) {
        *self.value.lock().unwrap_or_else(|e| e.into_inner()) = Some(v);
        self.at = now;
    }

    pub fn age(&self, now: u64) -> Option<u64> {
        self.get().map(|_| now.saturating_sub(self.at))
    }

    pub fn refresh(&mut self, now: u64, every: u64, work: impl FnOnce() -> T + Send + 'static) {
        let due = self.get().is_none() || now.saturating_sub(self.at) >= every;
        if !due || self.busy.swap(true, Ordering::AcqRel) {
            return;
        }
        self.at = now;
        let (slot, busy) = (Arc::clone(&self.value), Arc::clone(&self.busy));
        let spawned = thread::Builder::new().name("pc-bg".into()).spawn(move || {
            let v = work();
            *slot.lock().unwrap_or_else(|e| e.into_inner()) = Some(v);
            busy.store(false, Ordering::Release);
        });
        if spawned.is_err() {
            self.busy.store(false, Ordering::Release);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn wait_for(bg: &Bg<u32>) -> u32 {
        let end = Instant::now() + Duration::from_secs(5);
        while Instant::now() < end {
            if let Some(v) = bg.get() {
                return v;
            }
            thread::sleep(Duration::from_millis(5));
        }
        panic!("no value");
    }

    #[test]
    fn work_runs_once_per_interval_off_the_calling_thread() {
        let mut bg: Bg<u32> = Bg::new();
        assert_eq!(bg.get(), None);
        bg.refresh(100, 60, || 7);
        assert_eq!(wait_for(&bg), 7);
        bg.refresh(120, 60, || 8);
        thread::sleep(Duration::from_millis(50));
        assert_eq!(bg.get(), Some(7));
        bg.refresh(161, 60, || 9);
        let end = Instant::now() + Duration::from_secs(5);
        while bg.get() != Some(9) && Instant::now() < end {
            thread::sleep(Duration::from_millis(5));
        }
        assert_eq!(bg.get(), Some(9));
        assert_eq!(bg.age(170), Some(9));
    }
}
