use super::{Piece, frame_size, parse_info, parse_size, parse_time, pieces, retry_scan, still_size};
use crate::config::Config;

#[test]
fn a_long_scan_is_cut_into_overlapping_pieces() {
    let made = pieces(10, 4, 2);
    let piece = |lead, from, to, stop| Piece { lead, from, to, stop };
    assert_eq!(
        made,
        vec![
            piece(0, 0, Some(3), Some(5)),
            piece(1, 3, Some(6), Some(8)),
            piece(4, 6, Some(9), Some(10)),
            piece(7, 9, None, None),
        ]
    );
    for pair in made.windows(2) {
        assert_eq!(pair[0].to, Some(pair[1].from));
    }
}

#[test]
fn a_short_scan_stays_one_piece() {
    let whole = |from| Piece { lead: 0, from, to: None, stop: None };
    assert_eq!(pieces(0, 4, 2), vec![whole(0)]);
    assert_eq!(pieces(5, 1, 3), vec![whole(0)]);
    assert_eq!(pieces(3, 8, 0).len(), 3);
}

#[test]
fn the_picture_size_is_read_from_the_probe() {
    let raw = r#"{"streams": [{"width": 1920, "height": 1080}]}"#;
    assert_eq!(parse_size(raw), Some((1920, 1080)));
    assert_eq!(parse_size(r#"{"streams": []}"#), None);
    assert_eq!(parse_size(r#"{"streams": [{"width": 0, "height": 10}]}"#), None);
    assert_eq!(parse_size("no json"), None);
}

#[test]
fn landscape_portrait_and_small_videos_get_patch_sizes() {
    assert_eq!(frame_size(1280, 720, 672, 32), (672, 384));
    assert_eq!(frame_size(1920, 1080, 672, 32), (672, 384));
    assert_eq!(frame_size(1080, 1920, 672, 32), (384, 672));
    assert_eq!(frame_size(640, 480, 672, 32), (640, 480));
    assert_eq!(frame_size(320, 180, 672, 32), (320, 192));
    assert_eq!(frame_size(1000, 1000, 672, 32), (672, 672));
}

#[test]
fn a_small_picture_is_scaled_up_and_a_large_one_down() {
    assert_eq!(still_size(640, 360, 768, 768, 32), (768, 448));
    assert_eq!(still_size(360, 640, 768, 768, 32), (448, 768));
    assert_eq!(still_size(1920, 1080, 768, 768, 32), (768, 448));
    assert_eq!(still_size(100, 100, 768, 768, 32), (768, 768));
    assert_eq!(still_size(640, 360, 0, 768, 32), (640, 352));
    assert_eq!(still_size(640, 360, 1024, 768, 32), (768, 448));
    assert_eq!(still_size(900, 600, 704, 1024, 32), (896, 608));
    assert_eq!(still_size(0, 0, 768, 768, 0), (768, 768));
}

#[test]
fn showinfo_lines_give_their_time() {
    let line = "[Parsed_showinfo_3 @ 0000] n:  12 pts:     12 pts_time:6       duration:1 pos:-1 fmt:gray";
    assert_eq!(parse_time(line), Some(6.0));
    assert_eq!(parse_time("[Parsed_showinfo_3 @ 0] n: 3 pts: 3 pts_time:1.5 x"), Some(1.5));
    assert_eq!(parse_time("no time here"), None);
}

#[test]
fn ffprobe_json_is_read() {
    let raw = r#"{"programs":[],"streams":[{"width":1280,"height":720}],"format":{"duration":"187.354000"}}"#;
    let info = parse_info(raw).expect("info");
    assert_eq!((info.width, info.height), (1280, 720));
    assert!((info.duration - 187.354).abs() < 1e-9);
    assert!(parse_info(r#"{"streams":[],"format":{}}"#).is_none());
    assert!(parse_info("not json").is_none());
}

#[test]
fn a_short_fast_scan_is_retried_without_skipping() {
    let config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
    let mut frames = config.frames;
    frames.skip_frame = "bidir".to_string();
    frames.scan_retry_skip = "default".to_string();
    frames.scan_retry_share = 0.5;
    assert!(retry_scan(&frames, 0, 84.0));
    assert!(retry_scan(&frames, 41, 84.0));
    assert!(!retry_scan(&frames, 42, 84.0));
    assert!(!retry_scan(&frames, 84, 84.0));
    frames.scan_retry_share = 0.0;
    assert!(retry_scan(&frames, 0, 84.0));
    assert!(!retry_scan(&frames, 1, 84.0));
    frames.scan_retry_skip = "bidir".to_string();
    assert!(!retry_scan(&frames, 0, 84.0));
    frames.scan_retry_skip.clear();
    assert!(!retry_scan(&frames, 0, 84.0));
}
