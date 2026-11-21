#[derive(Debug, Clone)]
pub struct Scanned {
    pub at: f64,
    pub pixels: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct PickSettings {
    pub change: f64,
    pub pixel_delta: u8,
    pub settle_steps: usize,
    pub min_gap: f64,
    pub max_gap: f64,
    pub max_frames: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Change {
    pub share: f64,
    pub mean: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pick {
    pub at: f64,
    pub key: bool,
}

impl Change {
    const FIRST: Change = Change { share: f64::INFINITY, mean: f64::INFINITY };
}

pub fn difference(first: &[u8], second: &[u8], delta: u8) -> Change {
    if first.is_empty() || first.len() != second.len() {
        return Change { share: 1.0, mean: 1.0 };
    }
    let mut moved = 0usize;
    let mut sum = 0u64;
    for (a, b) in first.iter().zip(second) {
        let step = a.abs_diff(*b);
        sum += u64::from(step);
        if step > delta {
            moved += 1;
        }
    }
    let count = first.len() as f64;
    Change { share: moved as f64 / count, mean: sum as f64 / (count * 255.0) }
}

pub fn pick(frames: &[Scanned], settings: &PickSettings) -> Vec<Pick> {
    if frames.is_empty() || settings.max_frames == 0 {
        return Vec::new();
    }
    let mut chosen: Vec<(f64, Change, bool)> = vec![(frames[0].at, Change::FIRST, true)];
    let mut last = 0;
    let mut index = 1;
    while index < frames.len() {
        let gap = frames[index].at - frames[last].at;
        let change = difference(&frames[index].pixels, &frames[last].pixels, settings.pixel_delta);
        if change.share >= settings.change && gap >= settings.min_gap {
            let settled = (index + settings.settle_steps).min(frames.len() - 1);
            chosen.push((frames[settled].at, change, true));
            last = settled;
            index = settled + 1;
            continue;
        }
        if gap >= settings.max_gap {
            chosen.push((frames[index].at, change, false));
            last = index;
        }
        index += 1;
    }
    if chosen.len() > settings.max_frames {
        chosen.sort_by(|a, b| b.1.share.total_cmp(&a.1.share).then(b.1.mean.total_cmp(&a.1.mean)));
        chosen.truncate(settings.max_frames);
    }
    let mut picks: Vec<Pick> = chosen.into_iter().map(|(at, _, key)| Pick { at, key }).collect();
    picks.sort_by(|a, b| a.at.total_cmp(&b.at).then(b.key.cmp(&a.key)));
    picks.dedup_by(|later, earlier| later.at == earlier.at);
    picks
}

#[cfg(test)]
mod tests {
    use super::{PickSettings, Scanned, difference};

    fn pick(frames: &[Scanned], settings: &PickSettings) -> Vec<f64> {
        super::pick(frames, settings).iter().map(|p| p.at).collect()
    }

    fn settings() -> PickSettings {
        PickSettings { change: 0.03, pixel_delta: 12, settle_steps: 1, min_gap: 1.5, max_gap: 20.0, max_frames: 64 }
    }

    fn frame(at: f64, shade: u8) -> Scanned {
        Scanned { at, pixels: vec![shade; 16] }
    }

    fn still(seconds: usize, shade: u8) -> Vec<Scanned> {
        (0..seconds * 2).map(|i| frame(i as f64 * 0.5, shade)).collect()
    }

    #[test]
    fn difference_counts_clearly_changed_pixels() {
        assert_eq!(difference(&[10, 20], &[10, 20], 12).share, 0.0);
        assert_eq!(difference(&[0, 0], &[255, 255], 12).share, 1.0);
        assert_eq!(difference(&[0, 0], &[255, 255], 12).mean, 1.0);
        assert_eq!(difference(&[0, 0, 0, 0], &[200, 5, 0, 12], 12).share, 0.25);
        assert_eq!(difference(&[], &[], 12).share, 1.0);
        assert_eq!(difference(&[1], &[1, 2], 12).share, 1.0);
    }

    #[test]
    fn a_moving_mouse_is_not_a_change() {
        let frames: Vec<Scanned> = (0..20)
            .map(|i| {
                let mut pixels = vec![100u8; 64 * 36];
                pixels[i * 37..i * 37 + 4].fill(255);
                Scanned { at: i as f64 * 0.5, pixels }
            })
            .collect();
        assert_eq!(pick(&frames, &settings()), vec![0.0]);
    }

    #[test]
    fn a_still_screen_keeps_the_first_frame_and_gap_fillers() {
        let times = pick(&still(50, 100), &settings());
        assert_eq!(times, vec![0.0, 20.0, 40.0]);
    }

    #[test]
    fn a_change_takes_the_settled_frame_after_it() {
        let mut frames = still(10, 100);
        for item in frames.iter_mut().skip(8) {
            item.pixels = vec![200; 16];
        }
        let times = pick(&frames, &settings());
        assert_eq!(times, vec![0.0, 4.5]);
    }

    #[test]
    fn changes_closer_than_the_minimum_gap_wait() {
        let mut frames = still(4, 0);
        for (number, item) in frames.iter_mut().enumerate() {
            item.pixels = vec![(number * 40 % 250) as u8; 16];
        }
        let times = pick(&frames, &settings());
        assert!(times.windows(2).all(|pair| pair[1] - pair[0] >= 1.5));
    }

    #[test]
    fn too_many_frames_keep_the_first_and_the_biggest_changes() {
        let frames: Vec<Scanned> = (0..40)
            .map(|i| {
                let mut pixels = vec![0u8; 16];
                let changed = if i == 21 { 16 } else { 4 };
                if i % 2 == 1 {
                    pixels[..changed].fill(200);
                }
                Scanned { at: i as f64 * 2.0, pixels }
            })
            .collect();
        let limited = PickSettings { settle_steps: 0, max_frames: 3, ..settings() };
        let times = pick(&frames, &limited);
        assert_eq!(times.len(), 3);
        assert_eq!(times[0], 0.0);
        assert!(times.contains(&42.0));
    }

    #[test]
    fn changes_are_key_frames_and_gap_fillers_are_not() {
        let mut frames = still(60, 100);
        for item in frames.iter_mut().skip(60) {
            item.pixels = vec![200; 16];
        }
        let picks = super::pick(&frames, &settings());
        let keys: Vec<(f64, bool)> = picks.iter().map(|p| (p.at, p.key)).collect();
        assert_eq!(keys, vec![(0.0, true), (20.0, false), (30.5, true), (50.5, false)]);
    }

    #[test]
    fn nothing_in_nothing_out() {
        assert!(pick(&[], &settings()).is_empty());
        let none = PickSettings { max_frames: 0, ..settings() };
        assert!(pick(&still(3, 1), &none).is_empty());
    }
}
