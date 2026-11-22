use crate::captions::{self, Segment};
use crate::config;
use crate::fail::{Fail, Kind};
use crate::store_transcript::{Part, Stored};
use crate::youtube::{Video, VideoInfo, YouTube};

pub const SUBTITLES: &str = "subtitles";
pub const CAPTIONS: &str = "captions";
pub const NO_TEXT: &str = "none";
const FINISHED: [&str; 3] = [SUBTITLES, CAPTIONS, NO_TEXT];
const NO_SOUND: &str = "the video has no sound";
const WHOLE: i64 = 0;

#[derive(Debug, Clone, PartialEq)]
pub struct Transcript {
    pub kind: String,
    pub language: String,
    pub segments: Vec<Segment>,
    pub note: String,
}

pub fn check_finished(info: &VideoInfo) -> Result<(), Fail> {
    if info.not_started() {
        let message = "this live stream or premiere has not started yet";
        return Err(Fail::hint(Kind::Unavailable, message, "try again after it started"));
    }
    if info.is_live() {
        let message = "this video is a running live stream, so it has no fixed timeline yet";
        let hint = "try again after the stream ended and YouTube kept the recording";
        return Err(Fail::hint(Kind::Unavailable, message, hint));
    }
    Ok(())
}

fn stored(youtube: &YouTube<'_>, video: &Video) -> Result<Option<Transcript>, Fail> {
    let Some(found) = youtube.store.transcript(&video.id)? else { return Ok(None) };
    if !FINISHED.contains(&found.kind.as_str()) {
        return Ok(None);
    }
    let parts = youtube.store.transcript_parts(&video.id)?;
    let segments: Vec<Segment> = parts.into_iter().flat_map(|part| part.segments).collect();
    if segments.is_empty() && found.kind != NO_TEXT {
        return Ok(None);
    }
    Ok(Some(Transcript { kind: found.kind, language: found.language, segments, note: found.note }))
}

fn keep(youtube: &YouTube<'_>, video: &Video, found: &Transcript, end: f64) -> Result<(), Fail> {
    let row = Stored {
        kind: found.kind.clone(),
        language: found.language.clone(),
        note: found.note.clone(),
        created_at: youtube.clock.now(),
    };
    let whole = Part { part: WHOLE, start: 0.0, end, segments: found.segments.clone() };
    let parts = if found.segments.is_empty() { Vec::new() } else { vec![whole] };
    youtube.store.start_transcript(&video.id, &row, &parts)
}

pub fn load(youtube: &YouTube<'_>, settings: &config::Transcript, video: &Video) -> Result<Transcript, Fail> {
    if let Some(found) = stored(youtube, video)? {
        return Ok(found);
    }
    let info = youtube.info(video)?;
    check_finished(&info)?;
    if let Some(track) = captions::choose_track(&info) {
        let cues = youtube.subtitles(video, &track)?;
        let segments = captions::group_segments(&cues, settings.segment_seconds, settings.segment_chars);
        if let Some(last) = segments.last() {
            let end = info.duration.unwrap_or(last.end);
            let kind = if track.automatic { CAPTIONS } else { SUBTITLES };
            let found =
                Transcript { kind: kind.to_string(), language: track.language(), segments, note: String::new() };
            keep(youtube, video, &found, end)?;
            return Ok(found);
        }
    }
    if info.has_audio == Some(false) {
        let found = Transcript {
            kind: NO_TEXT.to_string(),
            language: String::new(),
            segments: Vec::new(),
            note: NO_SOUND.to_string(),
        };
        keep(youtube, video, &found, info.duration.unwrap_or(0.0))?;
        return Ok(found);
    }
    let hint = format!("wisply listen {} (writes down the speech)", video.url);
    Err(Fail::hint(Kind::Unavailable, "this video has no subtitles or captions", hint))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::fixture::{self, Bench, VIDEO_ID};

    const MANUAL: &str = r#"{"events": [
        {"tStartMs": 0, "dDurationMs": 4000, "segs": [{"utf8": "select the cube"}]},
        {"tStartMs": 4000, "dDurationMs": 4000, "segs": [{"utf8": "then bevel it"}]},
        {"tStartMs": 60000, "dDurationMs": 4000, "segs": [{"utf8": "much later"}]}]}"#;
    const EMPTY: &str = r#"{"events": []}"#;

    fn texts(found: &Transcript) -> Vec<&str> {
        found.segments.iter().map(|segment| segment.text.as_str()).collect()
    }

    fn load_on(bench: &Bench) -> Result<Transcript, Fail> {
        load(&bench.youtube(), &bench.config.transcript, &fixture::video())
    }

    #[test]
    fn subtitles_are_fetched_once_and_then_read_from_the_cache() {
        let bench = Bench::new("transcript-once");
        bench.runner.gives(fixture::answered(&fixture::yt_dlp_data(json!({}))));
        bench.runner.writes("abcdefghijk.en.json3", MANUAL);
        let first = load_on(&bench).unwrap();
        assert_eq!((first.kind.as_str(), first.language.as_str()), (SUBTITLES, "en"));
        assert_eq!(texts(&first), ["select the cube then bevel it", "much later"]);
        let again = load_on(&bench).unwrap();
        assert_eq!((again.kind.as_str(), texts(&again)), (SUBTITLES, texts(&first)));
        assert_eq!(bench.runner.calls().len(), 2);
        let parts = bench.store.transcript_parts(VIDEO_ID).unwrap();
        assert_eq!((parts.len(), parts[0].start, parts[0].end), (1, 0.0, 187.0));
    }

    #[test]
    fn automatic_captions_are_marked_as_such() {
        let bench = Bench::new("transcript-auto");
        bench.runner.gives(fixture::answered(&fixture::yt_dlp_data(json!({"subtitles": {}}))));
        bench.runner.writes("abcdefghijk.en-orig.json3", MANUAL);
        let found = load_on(&bench).unwrap();
        assert_eq!((found.kind.as_str(), found.language.as_str()), (CAPTIONS, "en"));
    }

    #[test]
    fn a_video_without_sound_is_remembered_as_having_no_text() {
        let bench = Bench::new("transcript-silent");
        let silent =
            json!({"subtitles": {}, "automatic_captions": {}, "formats": [{"acodec": "none", "vcodec": "avc1"}]});
        bench.runner.gives(fixture::answered(&fixture::yt_dlp_data(silent)));
        let found = load_on(&bench).unwrap();
        assert_eq!((found.kind.as_str(), found.note.as_str()), (NO_TEXT, NO_SOUND));
        assert_eq!(load_on(&bench).unwrap(), found);
        assert_eq!(bench.runner.calls().len(), 1);
    }

    #[test]
    fn a_video_without_any_text_points_to_listening() {
        let bench = Bench::new("transcript-none");
        bench
            .runner
            .gives(fixture::answered(&fixture::yt_dlp_data(json!({"subtitles": {}, "automatic_captions": {}}))));
        let fail = load_on(&bench).unwrap_err();
        assert_eq!((fail.kind, fail.message.as_str()), (Kind::Unavailable, "this video has no subtitles or captions"));
        assert_eq!(fail.hint, format!("wisply listen {} (writes down the speech)", fixture::video().url));
        assert_eq!(bench.store.transcript(VIDEO_ID).unwrap(), None);
    }

    #[test]
    fn an_empty_subtitle_file_is_not_kept_as_text() {
        let bench = Bench::new("transcript-empty");
        bench.runner.gives(fixture::answered(&fixture::yt_dlp_data(json!({}))));
        bench.runner.writes("abcdefghijk.en.json3", EMPTY);
        assert_eq!(load_on(&bench).unwrap_err().kind, Kind::Unavailable);
        assert_eq!(bench.store.transcript(VIDEO_ID).unwrap(), None);
    }

    #[test]
    fn running_and_coming_streams_are_refused() {
        let cases = [
            (json!({"live_status": "is_live", "duration": null}), "running live stream"),
            (json!({"live_status": "is_upcoming", "formats": [], "duration": null}), "not started yet"),
        ];
        for (changes, words) in cases {
            let bench = Bench::new("transcript-live");
            bench.runner.gives(fixture::answered(&fixture::yt_dlp_data(changes)));
            let fail = load_on(&bench).unwrap_err();
            assert_eq!(fail.kind, Kind::Unavailable);
            assert!(fail.message.contains(words), "{}", fail.message);
            assert!(!fail.hint.is_empty());
            assert_eq!(bench.runner.calls().len(), 1);
        }
    }

    #[test]
    fn a_kept_transcript_of_an_unknown_kind_is_fetched_again() {
        let bench = Bench::new("transcript-kind");
        let row = Stored { kind: "half".to_string(), language: "en".to_string(), note: String::new(), created_at: 1.0 };
        bench.store.start_transcript(VIDEO_ID, &row, &[]).unwrap();
        bench.runner.gives(fixture::answered(&fixture::yt_dlp_data(json!({}))));
        bench.runner.writes("abcdefghijk.en.json3", MANUAL);
        assert_eq!(load_on(&bench).unwrap().kind, SUBTITLES);
        assert_eq!(bench.store.transcript(VIDEO_ID).unwrap().unwrap().kind, SUBTITLES);
    }
}
