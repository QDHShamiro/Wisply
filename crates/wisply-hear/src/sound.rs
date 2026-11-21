use crate::config::Config;
use crate::events::{self, Event, Slot};
use crate::labels;
use crate::mel::Extractor;
use crate::tagger::Tagger;

pub fn hear(config: &Config, model_name: Option<&str>, samples: &[f32], offset: f64) -> Result<Vec<Event>, String> {
    let model = config.sound_model(model_name)?;
    if !model.file.exists() {
        let path = model.file.display().to_string();
        return Err(config.message("model_missing", &[("path", &path)]));
    }
    let names = labels::load(&model.labels).map_err(|detail| {
        let path = model.labels.display().to_string();
        config.message("labels_bad", &[("path", &path), ("detail", &detail)])
    })?;
    let fail = |detail: String| config.message("sound_failed", &[("detail", &detail)]);
    let mut tagger =
        Tagger::load(&config.sound.runtime, &model.file, config.sound.threads, names.len()).map_err(fail)?;
    let rate = config.audio.sample_rate;
    let extractor = Extractor::new(config.mel, rate);
    let spectrogram = extractor.spectrogram(samples);
    let total = samples.len() as f64 / rate as f64;
    let windows = events::windows(total, config.sound.window_seconds, config.sound.hop_seconds);
    let dropped = events::dropped_mask(&names, &config.sound.drop_labels);
    let frames_per_second = rate as f64 / config.mel.hop as f64;
    let mut slots = Vec::with_capacity(windows.len());
    for (index, window) in windows.iter().enumerate() {
        let from = (window.0 * frames_per_second).round() as usize;
        let count = ((window.1 - window.0) * frames_per_second).round() as usize;
        let feats = spectrogram.window(from, count, config.mel.top_db);
        let frames = feats.len() / spectrogram.bands;
        let scores = tagger.run(spectrogram.bands, frames, feats).map_err(fail)?;
        let hits = events::pick_hits(&scores, &dropped, config.sound.threshold, config.sound.max_labels_per_slot);
        let (start, end) = events::slot_span(index, windows.len(), *window, config.sound.hop_seconds);
        slots.push(Slot { start, end, hits });
    }
    let mut found = events::summarize(
        &slots,
        &names,
        config.sound.merge_gap_seconds,
        config.sound.min_seconds,
        config.sound.score_decimals,
    );
    for event in &mut found {
        event.start += offset;
        event.end += offset;
    }
    Ok(found)
}
