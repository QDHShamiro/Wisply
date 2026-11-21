use serde_json::json;

use super::*;
use crate::fixture::{self, Bench, COMMAND, START_TIME, VIDEO_ID};

const PRIVATE: &str = "ERROR: [youtube] abcdefghijk: Private video. Sign in";
const GONE: &str = "WARNING: [youtube] This video is unavailable\nWARNING: No video formats found!";
const MANUAL: &str = r#"{"events": [{"tStartMs": 1000, "dDurationMs": 2000, "segs": [{"utf8": "hello there"}]}]}"#;
const ROLLING: &str = "WEBVTT\n\n00:00:01.000 --> 00:00:03.000\nhello there\n";

fn sound(language: &str, preference: i64, picture: &str) -> Value {
    json!({"acodec": "opus", "vcodec": picture, "language": language, "language_preference": preference})
}

fn manual() -> Track {
    Track { key: "en".to_string(), automatic: false }
}

fn leftovers(bench: &Bench) -> usize {
    std::fs::read_dir(bench.folder.join(SCRATCH_FOLDER)).map(|entries| entries.count()).unwrap_or(0)
}

#[test]
fn the_details_of_a_video_are_read() {
    let info = info_from_json(&fixture::yt_dlp_data(json!({})), VIDEO_ID, 100);
    assert_eq!((info.title.as_str(), info.channel.as_str()), ("Bevel tutorial", "Example Channel"));
    assert_eq!((info.duration, info.has_audio), (Some(187.0), Some(true)));
    assert_eq!((info.width, info.height, info.age_limit), (Some(1920), Some(1080), 0));
    let chapters = [
        Chapter { start: 0.0, end: 30.0, title: "Intro".to_string() },
        Chapter { start: 30.0, end: 187.0, title: "Bevel".to_string() },
    ];
    assert_eq!(info.chapters, chapters);
    assert_eq!(info.subtitle_languages, ["en"]);
    assert_eq!(info.caption_languages, ["en-orig"]);
    assert_eq!(info.description.chars().count(), 100);
    assert!(!info.is_live() && !info.not_started());
    let kept: VideoInfo = serde_json::from_value(serde_json::to_value(&info).unwrap()).unwrap();
    assert_eq!(kept, info);
}

#[test]
fn odd_fields_do_not_break_the_details() {
    let odd = json!({"duration": "long", "chapters": [{"title": "no times"}, "junk"], "age_limit": true});
    let info = info_from_json(&odd, VIDEO_ID, 100);
    assert_eq!(info.title, VIDEO_ID);
    assert_eq!((info.duration, info.has_audio, info.age_limit), (None, None, 0));
    assert!(info.chapters.is_empty() && info.language.is_empty());
    let silent = fixture::info(json!({"formats": [{"format_id": "1", "acodec": "none", "vcodec": "avc1"}]}));
    assert_eq!(silent.has_audio, Some(false));
    let named = info_from_json(&json!({"uploader": " Someone "}), VIDEO_ID, 100);
    assert_eq!(named.channel, "Someone");
}

#[test]
fn the_spoken_language_is_the_original_sound_track() {
    let dubbed = json!([
        sound("ar", -1, "none"),
        sound("en-US", 10, "none"),
        sound("ar", -1, "avc1"),
        {"format_id": "614", "acodec": "none", "vcodec": "vp09.00.40.08"},
    ]);
    for language in [json!(null), json!(""), json!("ar")] {
        let info = fixture::info(json!({"language": language, "formats": dubbed}));
        assert_eq!(info.language, "en-US");
    }
    assert_eq!(fixture::info(json!({})).language, "en");
}

#[test]
fn every_caption_language_is_listed_when_no_original_is_marked() {
    let info = fixture::info(json!({"automatic_captions": {"fr": [], "de": []}}));
    assert_eq!(info.caption_languages, ["de", "fr"]);
    assert!(fixture::info(json!({"subtitles": {"live_chat": []}})).subtitle_languages.is_empty());
}

#[test]
fn the_command_cannot_be_bent_by_a_link_or_a_settings_file() {
    let (bench, video) = (Bench::new("command"), fixture::video());
    bench.youtube().info(&video).unwrap();
    let calls = bench.runner.calls();
    let argv = &calls[0];
    assert_eq!(argv[0], COMMAND);
    assert_eq!(argv[argv.len() - 2..], ["--".to_string(), video.url.clone()]);
    for flag in ["--ignore-config", "--no-playlist", "--skip-download", "-J"] {
        assert!(argv.iter().any(|arg| arg == flag), "{flag}");
    }
    let cache = bench.folder.join(CACHE_FOLDER).to_string_lossy().into_owned();
    assert_eq!(fixture::value_after(argv, "--cache-dir"), Some(cache));
    assert_eq!(fixture::value_after(argv, "-f"), Some(bench.config.youtube.frame_format.clone()));
    assert_eq!(fixture::value_after(argv, "-S"), Some(bench.config.youtube.frame_sort.clone()));
}

#[test]
fn details_are_kept_until_they_are_too_old() {
    let (bench, video) = (Bench::new("kept"), fixture::video());
    let first = bench.youtube().info(&video).unwrap();
    assert_eq!(bench.youtube().info(&video).unwrap(), first);
    assert_eq!(bench.runner.calls().len(), 1);
    bench.clock.advance(bench.config.youtube.info_hours * HOUR + 1.0);
    bench.youtube().info(&video).unwrap();
    assert_eq!(bench.runner.calls().len(), 2);
}

#[test]
fn a_failure_is_explained_and_not_kept() {
    let (bench, video) = (Bench::new("failure"), fixture::video());
    bench.runner.gives(fixture::failed(PRIVATE));
    let fail = bench.youtube().info(&video).unwrap_err();
    assert_eq!((fail.kind, fail.message.as_str()), (Kind::Unavailable, "this video is private"));
    assert_eq!(bench.youtube().cached(VIDEO_ID).unwrap(), None);
    assert_eq!(bench.youtube().info(&video).unwrap().title, "Bevel tutorial");
    assert_eq!(bench.runner.calls().len(), 2);
}

#[test]
fn a_yt_dlp_that_says_nothing_is_named_as_broken() {
    let (bench, video) = (Bench::new("silent"), fixture::video());
    bench.runner.gives(Ran { code: 1, ..fixture::done() });
    let fail = bench.youtube().info(&video).unwrap_err();
    assert_eq!(fail.kind, Kind::ToolMissing);
    assert_eq!(fail.message, format!("yt-dlp does not work: {COMMAND} ended without any output"));
    assert!(fail.hint.contains("video.toml"), "{}", fail.hint);
    bench.runner.gives(Ran { code: 1, ..fixture::done() });
    assert_eq!(bench.youtube().subtitles(&video, &manual()).unwrap_err().kind, Kind::ToolMissing);
}

#[test]
fn the_next_yt_dlp_is_tried_when_the_first_says_nothing() {
    let bench = Bench::new("next");
    let commands = || vec![vec!["broken".to_string()], vec!["python".to_string(), "-m".to_string()]];
    let rest = ["-J".to_string()];
    bench.runner.gives(Ran { code: 1, ..fixture::done() });
    let ran = first_answer(&bench.runner, commands(), &rest, 5.0).unwrap();
    assert!(ran.ok() && !ran.stdout.is_empty());
    assert_eq!(bench.runner.calls(), [vec!["broken", "-J"], vec!["python", "-m", "-J"]]);
    bench.runner.gives(Ran { code: 1, ..fixture::done() });
    bench.runner.gives(Ran { code: 1, ..fixture::done() });
    let fail = first_answer(&bench.runner, commands(), &rest, 5.0).unwrap_err();
    assert_eq!(fail.message, "yt-dlp does not work: broken and python -m ended without any output");
    bench.runner.gives(fixture::failed(PRIVATE));
    assert_eq!(first_answer(&bench.runner, commands(), &rest, 5.0).unwrap().stderr, PRIVATE);
    assert_eq!(bench.runner.calls().len(), 5);
}

#[test]
fn a_yt_dlp_that_cannot_be_started_is_skipped_and_named_when_none_is_left() {
    let bench = Bench::new("missing");
    let commands = || vec![vec!["gone".to_string()], vec!["python".to_string(), "-m".to_string()]];
    let rest = ["-J".to_string()];
    let gone = || Err(Fail::new(Kind::ToolMissing, "yt-dlp could not be started (gone is missing)"));
    bench.runner.then(move |_| gone());
    assert!(first_answer(&bench.runner, commands(), &rest, 5.0).unwrap().ok());
    assert_eq!(bench.runner.calls().len(), 2);
    bench.runner.then(move |_| gone());
    bench.runner.then(move |_| gone());
    let fail = first_answer(&bench.runner, commands(), &rest, 5.0).unwrap_err();
    assert_eq!(fail.message, "yt-dlp could not be started (gone is missing)");
    assert!(fail.hint.contains("video.toml"), "{}", fail.hint);
    bench.runner.then(|_| Err(Fail::new(Kind::Service, "yt-dlp took longer than 5 s and was stopped")));
    assert_eq!(first_answer(&bench.runner, commands(), &rest, 5.0).unwrap_err().kind, Kind::Service);
    assert_eq!(bench.runner.calls().len(), 5);
}

#[test]
fn an_answer_without_formats_is_explained_and_not_kept() {
    let (bench, video) = (Bench::new("formats"), fixture::video());
    let answer = fixture::answered(&fixture::yt_dlp_data(json!({"formats": []})));
    bench.runner.gives(Ran { stderr: GONE.to_string(), ..answer });
    let fail = bench.youtube().info(&video).unwrap_err();
    assert_eq!(fail.kind, Kind::Unavailable);
    assert!(fail.message.contains("unavailable"), "{}", fail.message);
    assert_eq!(bench.youtube().cached(VIDEO_ID).unwrap(), None);
}

#[test]
fn a_stream_that_has_not_started_keeps_its_details() {
    let (bench, video) = (Bench::new("upcoming"), fixture::video());
    let changes = json!({"formats": [], "live_status": "is_upcoming", "duration": null});
    bench.runner.gives(fixture::answered(&fixture::yt_dlp_data(changes)));
    let info = bench.youtube().info(&video).unwrap();
    assert!(info.not_started() && !info.is_live());
    assert_eq!((info.duration, info.has_audio), (None, None));
    assert!(fixture::info(json!({"live_status": "is_live"})).is_live());
}

#[test]
fn an_answer_that_is_not_video_details_is_named() {
    let (bench, video) = (Bench::new("odd"), fixture::video());
    for (raw, message) in [
        (&b"not json"[..], "yt-dlp gave an answer that is not JSON"),
        (&b"[1, 2]"[..], "yt-dlp gave an answer without video details"),
    ] {
        bench.runner.gives(Ran { stdout: raw.to_vec(), ..fixture::done() });
        let fail = bench.youtube().info(&video).unwrap_err();
        assert_eq!((fail.kind, fail.message.as_str()), (Kind::Service, message));
    }
}

#[test]
fn broken_kept_details_are_fetched_again() {
    let (bench, video) = (Bench::new("broken"), fixture::video());
    bench.store.put_video_info(VIDEO_ID, &json!({"title": "only half"}), START_TIME).unwrap();
    assert_eq!(bench.youtube().info(&video).unwrap().title, "Bevel tutorial");
    assert_eq!(bench.runner.calls().len(), 1);
}

#[test]
fn manual_subtitles_are_fetched_and_read() {
    let (bench, video) = (Bench::new("manual"), fixture::video());
    bench.runner.writes("abcdefghijk.en.json3", MANUAL);
    let cues = bench.youtube().subtitles(&video, &manual()).unwrap();
    assert_eq!(cues, [Segment::new(1.0, 3.0, "hello there")]);
    let calls = bench.runner.calls();
    let argv = &calls[0];
    assert!(argv.iter().any(|arg| arg == "--write-subs") && !argv.iter().any(|arg| arg == "--write-auto-subs"));
    assert_eq!(fixture::value_after(argv, "--sub-langs"), Some("en".to_string()));
    assert_eq!(fixture::value_after(argv, "--sub-format"), Some("json3/vtt".to_string()));
    assert_eq!(argv[argv.len() - 2..], ["--".to_string(), video.url.clone()]);
    assert_eq!(leftovers(&bench), 0);
}

#[test]
fn automatic_captions_are_asked_for_by_their_exact_name() {
    let (bench, video) = (Bench::new("automatic"), fixture::video());
    bench.runner.writes("abcdefghijk.en-orig.vtt", ROLLING);
    let track = Track { key: "en-orig".to_string(), automatic: true };
    assert_eq!(bench.youtube().subtitles(&video, &track).unwrap(), [Segment::new(1.0, 3.0, "hello there")]);
    let calls = bench.runner.calls();
    assert!(calls[0].iter().any(|arg| arg == "--write-auto-subs"));
    assert_eq!(fixture::value_after(&calls[0], "--sub-langs"), Some(r"en\-orig".to_string()));
}

#[test]
fn missing_or_broken_subtitle_files_are_named() {
    let (bench, video) = (Bench::new("missing"), fixture::video());
    let fail = bench.youtube().subtitles(&video, &manual()).unwrap_err();
    assert_eq!((fail.kind, fail.message.as_str()), (Kind::Service, "YouTube gave no en subtitles for abcdefghijk"));
    bench.runner.writes("abcdefghijk.en.json3", "{half");
    let fail = bench.youtube().subtitles(&video, &manual()).unwrap_err();
    assert_eq!(fail.message, "the subtitle file from YouTube is not valid JSON");
    bench.runner.gives(fixture::failed(PRIVATE));
    assert_eq!(bench.youtube().subtitles(&video, &manual()).unwrap_err().kind, Kind::Unavailable);
    assert_eq!(leftovers(&bench), 0);
}
