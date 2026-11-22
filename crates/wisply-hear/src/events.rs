use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub struct Slot {
    pub start: f64,
    pub end: f64,
    pub hits: Vec<(usize, f32)>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Event {
    pub start: f64,
    pub end: f64,
    pub label: String,
    pub score: f64,
}

pub fn windows(total: f64, window: f64, hop: f64) -> Vec<(f64, f64)> {
    if total <= 0.0 || window <= 0.0 || hop <= 0.0 {
        return Vec::new();
    }
    if total <= window {
        return vec![(0.0, total)];
    }
    let mut out = Vec::new();
    let mut start = 0.0;
    loop {
        if start + window >= total - 1e-9 {
            out.push((total - window, total));
            break;
        }
        out.push((start, start + window));
        start += hop;
    }
    out
}

pub fn slot_span(index: usize, count: usize, window: (f64, f64), hop: f64) -> (f64, f64) {
    let center = (window.0 + window.1) / 2.0;
    let start = if index == 0 { window.0 } else { center - hop / 2.0 };
    let end = if index + 1 == count { window.1 } else { center + hop / 2.0 };
    (start, end)
}

pub fn pick_hits(scores: &[f32], dropped: &[bool], threshold: f32, max_labels: usize) -> Vec<(usize, f32)> {
    let mut hits: Vec<(usize, f32)> = scores
        .iter()
        .enumerate()
        .filter(|(i, s)| **s >= threshold && !dropped.get(*i).copied().unwrap_or(false))
        .map(|(i, s)| (i, *s))
        .collect();
    hits.sort_by(|a, b| b.1.total_cmp(&a.1));
    hits.truncate(max_labels);
    hits
}

pub fn summarize(slots: &[Slot], names: &[String], merge_gap: f64, min_seconds: f64, decimals: u32) -> Vec<Event> {
    let mut open: BTreeMap<usize, (f64, f64, f32)> = BTreeMap::new();
    let mut done: Vec<(usize, f64, f64, f32)> = Vec::new();
    for slot in slots {
        let seen: Vec<usize> = slot.hits.iter().map(|h| h.0).collect();
        let stale: Vec<usize> = open
            .iter()
            .filter(|(label, run)| !seen.contains(label) && slot.start - run.1 > merge_gap)
            .map(|(label, _)| *label)
            .collect();
        for label in stale {
            if let Some(run) = open.remove(&label) {
                done.push((label, run.0, run.1, run.2));
            }
        }
        for (label, score) in &slot.hits {
            match open.get_mut(label) {
                Some(run) if slot.start - run.1 <= merge_gap => {
                    run.1 = slot.end;
                    run.2 = run.2.max(*score);
                }
                Some(_) => {
                    if let Some(run) = open.remove(label) {
                        done.push((*label, run.0, run.1, run.2));
                    }
                    open.insert(*label, (slot.start, slot.end, *score));
                }
                None => {
                    open.insert(*label, (slot.start, slot.end, *score));
                }
            }
        }
    }
    for (label, run) in open {
        done.push((label, run.0, run.1, run.2));
    }
    let scale = 10f64.powi(decimals as i32);
    let mut events: Vec<Event> = done
        .into_iter()
        .filter(|(_, start, end, _)| end - start >= min_seconds)
        .map(|(label, start, end, score)| Event {
            start,
            end,
            label: names.get(label).cloned().unwrap_or_else(|| label.to_string()),
            score: (score as f64 * scale).round() / scale,
        })
        .collect();
    events.sort_by(|a, b| a.start.total_cmp(&b.start).then_with(|| a.label.cmp(&b.label)));
    events
}

pub fn dropped_mask(names: &[String], drop: &[String]) -> Vec<bool> {
    names.iter().map(|n| drop.iter().any(|d| d == n)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot(start: f64, end: f64, hits: &[(usize, f32)]) -> Slot {
        Slot { start, end, hits: hits.to_vec() }
    }

    fn names() -> Vec<String> {
        ["Music", "Applause", "Click"].map(String::from).to_vec()
    }

    #[test]
    fn windows_cover_the_end() {
        let w = windows(10.0, 3.0, 1.0);
        assert_eq!(w.first(), Some(&(0.0, 3.0)));
        assert_eq!(w.last(), Some(&(7.0, 10.0)));
        assert_eq!(w.len(), 8);
    }

    #[test]
    fn windows_short_audio_is_one_window() {
        assert_eq!(windows(2.0, 3.0, 1.0), vec![(0.0, 2.0)]);
        assert!(windows(0.0, 3.0, 1.0).is_empty());
    }

    #[test]
    fn windows_tail_is_aligned_to_the_end() {
        let w = windows(9.5, 3.0, 1.0);
        assert_eq!(w.last(), Some(&(6.5, 9.5)));
    }

    #[test]
    fn slot_spans_tile_the_timeline() {
        let w = windows(6.0, 3.0, 1.0);
        let spans: Vec<(f64, f64)> = w.iter().enumerate().map(|(i, win)| slot_span(i, w.len(), *win, 1.0)).collect();
        assert_eq!(spans.first().unwrap().0, 0.0);
        assert_eq!(spans.last().unwrap().1, 6.0);
        for pair in spans.windows(2) {
            assert!((pair[0].1 - pair[1].0).abs() < 1e-9);
        }
    }

    #[test]
    fn pick_hits_filters_sorts_and_limits() {
        let scores = [0.9, 0.5, 0.4, 0.1];
        let dropped = [false, false, true, false];
        let hits = pick_hits(&scores, &dropped, 0.35, 5);
        assert_eq!(hits, vec![(0, 0.9), (1, 0.5)]);
        assert_eq!(pick_hits(&scores, &dropped, 0.35, 1), vec![(0, 0.9)]);
    }

    #[test]
    fn neighbours_with_same_label_merge() {
        let slots = vec![slot(0.0, 1.0, &[(0, 0.5)]), slot(1.0, 2.0, &[(0, 0.8)]), slot(2.0, 3.0, &[(0, 0.6)])];
        let events = summarize(&slots, &names(), 0.0, 1.0, 2);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].start, 0.0);
        assert_eq!(events[0].end, 3.0);
        assert_eq!(events[0].score, 0.8);
        assert_eq!(events[0].label, "Music");
    }

    #[test]
    fn gap_inside_limit_is_bridged_and_outside_splits() {
        let slots = vec![
            slot(0.0, 1.0, &[(1, 0.5)]),
            slot(1.0, 2.0, &[]),
            slot(2.0, 3.0, &[(1, 0.5)]),
            slot(3.0, 4.0, &[]),
            slot(4.0, 5.0, &[]),
            slot(5.0, 6.0, &[(1, 0.5)]),
        ];
        let events = summarize(&slots, &names(), 1.0, 1.0, 2);
        assert_eq!(events.len(), 2);
        assert_eq!((events[0].start, events[0].end), (0.0, 3.0));
        assert_eq!((events[1].start, events[1].end), (5.0, 6.0));
    }

    #[test]
    fn short_events_are_dropped() {
        let slots = vec![slot(0.0, 1.0, &[(2, 0.9)]), slot(1.0, 2.0, &[])];
        assert!(summarize(&slots, &names(), 0.0, 1.5, 2).is_empty());
        assert_eq!(summarize(&slots, &names(), 0.0, 1.0, 2).len(), 1);
    }

    #[test]
    fn different_labels_overlap_and_sort_by_start() {
        let slots =
            vec![slot(0.0, 1.0, &[(0, 0.7)]), slot(1.0, 2.0, &[(0, 0.7), (1, 0.6)]), slot(2.0, 3.0, &[(1, 0.6)])];
        let events = summarize(&slots, &names(), 0.0, 1.0, 2);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].label, "Music");
        assert_eq!(events[1].label, "Applause");
        assert_eq!((events[1].start, events[1].end), (1.0, 3.0));
    }

    #[test]
    fn dropped_mask_matches_names() {
        let mask = dropped_mask(&names(), &["Click".to_string()]);
        assert_eq!(mask, vec![false, false, true]);
    }

    #[test]
    fn score_is_rounded() {
        let slots = vec![slot(0.0, 2.0, &[(0, 0.87654)])];
        assert_eq!(summarize(&slots, &names(), 0.0, 1.0, 2)[0].score, 0.88);
    }
}
