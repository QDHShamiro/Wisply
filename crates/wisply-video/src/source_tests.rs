use super::*;

const ID: &str = "dQw4w9WgXcQ";
const ODD: &str = "a-B_c1D2e3F";
const LIST: &str = "PLrAXtmErZgOeiKm4sgNOknGvNjby9efdf";
const WATCH: &str = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";

fn video(text: &str) -> (String, Form, Option<f64>) {
    match parse_on(text, true) {
        Ok(Source::YouTube { id, form, start_hint }) => (id, form, start_hint),
        other => panic!("{text}: {other:?}"),
    }
}

fn refusal(text: &str) -> Fail {
    match parse_on(text, true) {
        Err(fail) => fail,
        Ok(source) => panic!("{text}: {source:?}"),
    }
}

fn local_path(text: &str, windows: bool) -> String {
    match parse_on(text, windows) {
        Ok(Source::Local(path)) => path.to_string_lossy().replace('\\', "/"),
        other => panic!("{text}: {other:?}"),
    }
}

#[test]
fn every_youtube_link_form_gives_the_video_id() {
    let cases = [
        (format!("https://www.youtube.com/watch?v={ID}"), Form::Watch),
        (format!("http://www.youtube.com/watch?v={ID}"), Form::Watch),
        (format!("https://youtube.com/watch?v={ID}"), Form::Watch),
        (format!("https://m.youtube.com/watch?v={ID}"), Form::Watch),
        (format!("https://music.youtube.com/watch?v={ID}"), Form::Watch),
        (format!("HTTPS://WWW.YOUTUBE.COM/watch?v={ID}"), Form::Watch),
        (format!("https://www.youtube.com/watch?feature=shared&v={ID}"), Form::Watch),
        (format!("https://www.youtube.com/watch?v={ID}&list={LIST}&index=3"), Form::Watch),
        (format!("https://www.youtube.com/watch?v={ID}&si=AbCdEfGh&pp=ygUE"), Form::Watch),
        (format!("https://www.youtube.com/watch/{ID}"), Form::Watch),
        (format!("https://youtu.be/{ID}"), Form::Watch),
        (format!("https://youtu.be/{ID}?si=AbCdEfGh"), Form::Watch),
        (format!("https://youtu.be/{ID}?list={LIST}"), Form::Watch),
        (format!("https://www.youtube.com/shorts/{ID}"), Form::Short),
        (format!("https://www.youtube.com/shorts/{ID}?feature=share"), Form::Short),
        (format!("https://www.youtube.com/live/{ID}"), Form::Live),
        (format!("https://www.youtube.com/embed/{ID}"), Form::Embed),
        (format!("https://www.youtube-nocookie.com/embed/{ID}"), Form::Embed),
        (format!("https://www.youtube.com/v/{ID}"), Form::Embed),
        (format!("https://www.youtube.com/e/{ID}"), Form::Embed),
        (format!("youtube.com/watch?v={ID}"), Form::Watch),
        (format!("www.youtube.com/watch?v={ID}"), Form::Watch),
        (format!("youtu.be/{ID}"), Form::Watch),
        (format!("  https://youtu.be/{ID}  "), Form::Watch),
        (format!("<https://youtu.be/{ID}>"), Form::Watch),
        (format!("\"https://youtu.be/{ID}\""), Form::Watch),
        (format!("https://www.youtube.com/watch?v={ID})."), Form::Watch),
        (format!("https://www.youtube.com/attribution_link?u=%2Fwatch%3Fv%3D{ID}%26feature%3Dshare"), Form::Watch),
    ];
    for (link, form) in cases {
        assert_eq!(video(&link), (ID.to_string(), form, None), "{link}");
    }
    assert_eq!(video(&format!("https://youtu.be/{ODD}")).0, ODD);
    assert_eq!(parse(&format!("https://youtu.be/{ID}")).unwrap().url().as_deref(), Some(WATCH));
}

#[test]
fn a_start_time_in_the_link_is_kept_as_a_hint() {
    let cases = [
        (format!("{WATCH}&t=83"), Some(83.0)),
        (format!("{WATCH}&t=83s"), Some(83.0)),
        (format!("{WATCH}&t=1m23s"), Some(83.0)),
        (format!("https://youtu.be/{ID}?t=1h2m3s"), Some(3723.0)),
        (format!("https://www.youtube.com/embed/{ID}?start=90"), Some(90.0)),
        (format!("{WATCH}&time_continue=12"), Some(12.0)),
        (format!("{WATCH}#t=45"), Some(45.0)),
        (format!("{WATCH}&t=soon"), None),
        (WATCH.to_string(), None),
    ];
    for (link, hint) in cases {
        assert_eq!(video(&link).2, hint, "{link}");
    }
}

#[test]
fn playlists_channels_and_clips_are_refused_with_the_way_out() {
    let playlists = [
        format!("https://www.youtube.com/playlist?list={LIST}"),
        format!("https://www.youtube.com/watch?list={LIST}"),
        format!("https://www.youtube.com/embed/videoseries?list={LIST}"),
        format!("https://music.youtube.com/playlist?list={LIST}"),
    ];
    for link in playlists {
        let fail = refusal(&link);
        assert!(fail.message.contains("playlist") && fail.hint.contains("single video"), "{link}: {}", fail.text());
    }
    let channels = [
        "https://www.youtube.com/@BlenderGuru",
        "https://www.youtube.com/@BlenderGuru/videos",
        "https://www.youtube.com/channel/UCOKHwx1VCdgnxwbjyb9Iu1g",
        "https://www.youtube.com/c/BlenderGuruOfficial",
        "https://www.youtube.com/user/AndrewPPrice",
    ];
    for link in channels {
        assert!(refusal(link).message.contains("channel"), "{link}");
    }
    let clip = refusal("https://www.youtube.com/clip/UgkxU2HSeGL_NvmDJ-nQJrlLwllwMDBdGZFs");
    assert!(clip.message.contains("clip") && clip.hint.contains("full video"), "{}", clip.text());
}

#[test]
fn broken_ids_and_links_without_a_video_are_refused() {
    let broken = [
        "https://www.youtube.com/watch?v=tooshort",
        "https://www.youtube.com/watch?v=waytoolongvideoid123",
        "https://youtu.be/bad$id!1234",
        "https://www.youtube.com/shorts/abc",
        "https://www.youtube.com/embed/live_stream?channel=UCOKHwx1VCdgnxwbjyb9Iu1g",
    ];
    for link in broken {
        let fail = refusal(link);
        assert!(fail.message.contains("is not a YouTube video ID"), "{link}: {}", fail.message);
        assert_eq!(fail.kind, Kind::Source);
    }
    let empty = [
        "https://www.youtube.com/",
        "https://www.youtube.com",
        "https://youtu.be/",
        "https://www.youtube.com/watch",
        "https://www.youtube.com/watch?v=",
        "https://www.youtube.com/results?search_query=blender",
        "https://www.youtube.com/feed/subscriptions",
        "https://www.youtube.com/shorts/",
    ];
    for link in empty {
        assert!(refusal(link).message.contains("no video ID found"), "{link}");
    }
}

#[test]
fn links_to_other_sites_are_pages() {
    let pages = [
        "https://vimeo.com/123456789",
        "https://notyoutube.com/watch?v=dQw4w9WgXcQ",
        "https://youtube.com.example.org/watch?v=dQw4w9WgXcQ",
        "https://example.org:8443/clips/7?quality=best#start",
        "http://[2001:db8::1]:8080/clip",
    ];
    for link in pages {
        assert_eq!(parse_on(link, true).unwrap(), Source::Page(link.to_string()), "{link}");
    }
    let wrapped = parse_on("<HTTPS://vimeo.com/123456789).>", true).unwrap();
    assert_eq!(wrapped.url().as_deref(), Some("https://vimeo.com/123456789"));
}

#[test]
fn other_schemes_and_malformed_links_are_refused() {
    for link in ["ftp://example.org/clip.mp4", "rtmp://example.org/live/stream"] {
        assert!(refusal(link).message.contains(":// are not supported"), "{link}");
    }
    let malformed = ["https://[broken/watch?v=x", "http://youtube.com:port/watch", "https://", "https:///watch?v=x"];
    for link in malformed {
        assert!(refusal(link).message.contains("is not a valid link"), "{link}");
    }
}

#[test]
fn empty_values_and_control_characters_are_refused() {
    for value in ["", "   ", "\"\"", "<>"] {
        assert_eq!(refusal(value).message, "no video was given", "{value:?}");
    }
    for value in ["a\u{0}b", "a\nb"] {
        assert!(refusal(value).message.contains("control characters"), "{value:?}");
    }
}

#[test]
fn windows_paths_become_local_videos() {
    let cases = [
        (r"E:\Videos\clip.mp4", "E:/Videos/clip.mp4"),
        ("E:/Videos/clip.mp4", "E:/Videos/clip.mp4"),
        (r"C:\Users\someone\My Videos\clip (1).mp4", "C:/Users/someone/My Videos/clip (1).mp4"),
        ("\"E:\\Videos\\clip.mp4\"", "E:/Videos/clip.mp4"),
        ("/e/Videos/clip.mp4", "E:/Videos/clip.mp4"),
        (r"\\server\share\clip.mp4", "//server/share/clip.mp4"),
        ("file:///E:/Videos/my%20clip.mp4", "E:/Videos/my clip.mp4"),
        ("file://server/share/clip.mp4", "//server/share/clip.mp4"),
    ];
    for (text, expected) in cases {
        assert_eq!(local_path(text, true), expected, "{text}");
    }
}

#[test]
fn posix_paths_become_local_videos() {
    let cases = [
        ("/home/someone/videos/clip.mp4", "/home/someone/videos/clip.mp4"),
        ("/e/Videos/clip.mp4", "/e/Videos/clip.mp4"),
        ("file:///home/someone/my%20clip.mp4", "/home/someone/my clip.mp4"),
        ("file://localhost/home/someone/clip.mp4", "/home/someone/clip.mp4"),
    ];
    for (text, expected) in cases {
        assert_eq!(local_path(text, false), expected, "{text}");
    }
}

#[test]
fn home_folder_paths_are_expanded() {
    let path = local_path("~/Videos/clip.mp4", cfg!(windows));
    assert!(path.ends_with("/Videos/clip.mp4") && !path.starts_with('~'), "{path}");
}

#[test]
fn relative_paths_and_plain_words_are_refused() {
    for text in ["clip.mp4", "videos/clip.mp4", r"..\clip.mp4", "blender donut tutorial"] {
        for windows in [true, false] {
            let fail = parse_on(text, windows).unwrap_err();
            assert!(fail.message.contains("neither a link nor the full path"), "{text}: {}", fail.message);
        }
    }
}

#[test]
fn a_bare_video_id_is_refused_with_the_full_link_as_hint() {
    assert!(refusal(ID).hint.contains(WATCH));
}

#[test]
fn long_input_is_cut_in_the_message() {
    assert!(refusal(&"word ".repeat(200)).message.len() < 200);
}
