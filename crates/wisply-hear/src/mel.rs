use std::f32::consts::PI;
use std::sync::Arc;

use rustfft::num_complex::Complex;
use rustfft::{Fft, FftPlanner};

use crate::config::Mel;

pub struct Spectrogram {
    pub bands: usize,
    pub frames: usize,
    pub db: Vec<f32>,
}

pub struct Extractor {
    settings: Mel,
    window: Vec<f32>,
    filters: Vec<Vec<f32>>,
    fft: Arc<dyn Fft<f32>>,
}

fn hz_to_mel(hz: f32) -> f32 {
    2595.0 * (1.0 + hz / 700.0).log10()
}

fn mel_to_hz(mel: f32) -> f32 {
    700.0 * (10f32.powf(mel / 2595.0) - 1.0)
}

pub fn filterbank(settings: &Mel, sample_rate: u32) -> Vec<Vec<f32>> {
    let bins = settings.n_fft / 2 + 1;
    let nyquist = sample_rate as f32 / 2.0;
    let freqs: Vec<f32> = (0..bins).map(|i| nyquist * i as f32 / (bins - 1) as f32).collect();
    let low = hz_to_mel(settings.f_min);
    let high = hz_to_mel(settings.f_max);
    let points: Vec<f32> = (0..settings.n_mels + 2)
        .map(|i| mel_to_hz(low + (high - low) * i as f32 / (settings.n_mels + 1) as f32))
        .collect();
    (0..settings.n_mels)
        .map(|m| {
            let left = points[m];
            let center = points[m + 1];
            let right = points[m + 2];
            freqs
                .iter()
                .map(|f| {
                    let down = (f - left) / (center - left);
                    let up = (right - f) / (right - center);
                    down.min(up).max(0.0)
                })
                .collect()
        })
        .collect()
}

pub fn reflect_pad(samples: &[f32], pad: usize) -> Vec<f32> {
    let n = samples.len();
    let mut out = Vec::with_capacity(n + 2 * pad);
    let index = |i: isize| -> f32 {
        let period = 2 * (n as isize - 1).max(1);
        let mut j = i.rem_euclid(period);
        if j >= n as isize {
            j = period - j;
        }
        samples[(j as usize).min(n - 1)]
    };
    for i in 0..pad {
        out.push(index(-(pad as isize) + i as isize));
    }
    out.extend_from_slice(samples);
    for i in 0..pad {
        out.push(index(n as isize + i as isize));
    }
    out
}

impl Extractor {
    pub fn new(settings: Mel, sample_rate: u32) -> Extractor {
        let window =
            (0..settings.n_fft).map(|i| 0.5 - 0.5 * (2.0 * PI * i as f32 / settings.n_fft as f32).cos()).collect();
        let fft = FftPlanner::new().plan_fft_forward(settings.n_fft);
        Extractor { settings, window, filters: filterbank(&settings, sample_rate), fft }
    }

    pub fn spectrogram(&self, samples: &[f32]) -> Spectrogram {
        let s = self.settings;
        if samples.is_empty() {
            return Spectrogram { bands: s.n_mels, frames: 0, db: Vec::new() };
        }
        let padded = reflect_pad(samples, s.frame_pad);
        let frames = (padded.len().saturating_sub(s.n_fft)) / s.hop + 1;
        let bins = s.n_fft / 2 + 1;
        let mut db = vec![0.0f32; frames * s.n_mels];
        let mut buffer = vec![Complex::new(0.0f32, 0.0); s.n_fft];
        let mut power = vec![0.0f32; bins];
        for frame in 0..frames {
            let from = frame * s.hop;
            for i in 0..s.n_fft {
                buffer[i] = Complex::new(padded[from + i] * self.window[i], 0.0);
            }
            self.fft.process(&mut buffer);
            for (bin, p) in power.iter_mut().enumerate() {
                *p = buffer[bin].norm_sqr();
            }
            for (m, filter) in self.filters.iter().enumerate() {
                let energy: f32 = filter.iter().zip(&power).map(|(a, b)| a * b).sum();
                db[frame * s.n_mels + m] = 10.0 * energy.max(s.amin).log10();
            }
        }
        Spectrogram { bands: s.n_mels, frames, db }
    }
}

impl Spectrogram {
    pub fn window(&self, from: usize, count: usize, top_db: f32) -> Vec<f32> {
        let to = (from + count).min(self.frames);
        let from = from.min(to);
        let len = to - from;
        let mut peak = f32::NEG_INFINITY;
        for frame in from..to {
            for band in 0..self.bands {
                peak = peak.max(self.db[frame * self.bands + band]);
            }
        }
        let floor = peak - top_db;
        let mut out = vec![0.0f32; self.bands * len];
        for frame in from..to {
            for band in 0..self.bands {
                out[band * len + (frame - from)] = self.db[frame * self.bands + band].max(floor);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn settings() -> Mel {
        Mel { n_fft: 512, hop: 160, n_mels: 64, f_min: 0.0, f_max: 8000.0, amin: 1e-10, top_db: 120.0, frame_pad: 256 }
    }

    #[test]
    fn filters_are_triangles() {
        let filters = filterbank(&settings(), 16000);
        assert_eq!(filters.len(), 64);
        assert_eq!(filters[0].len(), 257);
        assert!(filters.iter().all(|f| f.iter().all(|v| *v >= 0.0 && *v <= 1.0)));
        assert!(filters.iter().all(|f| f.iter().any(|v| *v > 0.0)));
    }

    #[test]
    fn reflect_pad_mirrors_without_edge() {
        let out = reflect_pad(&[1.0, 2.0, 3.0, 4.0], 2);
        assert_eq!(out, vec![3.0, 2.0, 1.0, 2.0, 3.0, 4.0, 3.0, 2.0]);
    }

    #[test]
    fn frame_count_matches_spectrogram() {
        let extractor = Extractor::new(settings(), 16000);
        let samples = vec![0.1f32; 16000];
        let spec = extractor.spectrogram(&samples);
        assert_eq!(spec.frames, 101);
        assert_eq!(spec.db.len(), 101 * 64);
    }

    #[test]
    fn tone_peaks_in_matching_band() {
        let extractor = Extractor::new(settings(), 16000);
        let samples: Vec<f32> = (0..16000).map(|i| (2.0 * PI * 1000.0 * i as f32 / 16000.0).sin()).collect();
        let spec = extractor.spectrogram(&samples);
        let frame = &spec.db[50 * 64..51 * 64];
        let best = frame.iter().enumerate().max_by(|a, b| a.1.total_cmp(b.1)).map(|(i, _)| i).unwrap();
        let center_hz = mel_to_hz(hz_to_mel(0.0) + (hz_to_mel(8000.0) - hz_to_mel(0.0)) * (best + 1) as f32 / 65.0);
        assert!((center_hz - 1000.0).abs() < 150.0, "{center_hz}");
    }

    #[test]
    fn window_clamps_to_top_db_and_transposes() {
        let spec = Spectrogram { bands: 2, frames: 3, db: vec![0.0, -200.0, 10.0, -5.0, 20.0, 0.0] };
        let out = spec.window(0, 3, 100.0);
        assert_eq!(out.len(), 6);
        assert_eq!(out[0..3], [0.0, 10.0, 20.0]);
        assert_eq!(out[3..6], [-80.0, -5.0, 0.0]);
    }

    #[test]
    fn window_past_the_end_is_short() {
        let spec = Spectrogram { bands: 1, frames: 4, db: vec![1.0, 2.0, 3.0, 4.0] };
        assert_eq!(spec.window(3, 5, 100.0), vec![4.0]);
        assert!(spec.window(9, 5, 100.0).is_empty());
    }
}
