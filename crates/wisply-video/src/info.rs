use crate::asked::Asked;
use crate::captions;
use crate::fail::Fail;
use crate::timecode;
use crate::youtube::{Video, VideoInfo, YouTube};

pub const VERB: &str = "info";
pub const USAGE: &str = "wisply pupil info <YouTube link>";
const PUBLIC: &str = "public";

fn text_line(info: &VideoInfo, video: &Video) -> String {
    let read = format!("read with: wisply pupil captions {}", video.url);
    match captions::choose_track(info) {
        Some(track) if track.automatic => format!("captions {} (automatic), {read}", track.language()),
        Some(track) => format!("subtitles {}, {read}", track.language()),
        None if info.has_audio == Some(false) => "none (no sound)".to_string(),
        None => format!("none, speech with: wisply listen {}", video.url),
    }
}

fn report(info: &VideoInfo, video: &Video) -> String {
    let mut lines = vec![format!("video  {}  {}", info.video_id, info.title)];
    if !info.channel.is_empty() {
        lines.push(format!("channel  {}", info.channel));
    }
    if info.is_live() {
        lines.push("state  live now, no fixed timeline yet".to_string());
    } else if info.not_started() {
        lines.push("state  not started yet".to_string());
    }
    if !info.availability.is_empty() && info.availability != PUBLIC {
        lines.push(format!("open to  {}", info.availability.replace('_', " ")));
    }
    if let Some(duration) = info.duration {
        lines.push(format!("length  {}", timecode::format(duration)));
    }
    if let (Some(width), Some(height)) = (info.width, info.height) {
        lines.push(format!("picture  {width}x{height}"));
    }
    if let Some(sound) = info.has_audio {
        lines.push(format!("sound  {}", if sound { "yes" } else { "no" }));
    }
    if !info.language.is_empty() {
        lines.push(format!("language  {}", info.language));
    }
    if !info.is_live() && !info.not_started() {
        lines.push(format!("text  {}", text_line(info, video)));
    }
    if !info.chapters.is_empty() {
        lines.push(format!("chapters  {}", info.chapters.len()));
        lines.extend(info.chapters.iter().map(|part| format!("{}  {}", timecode::format(part.start), part.title)));
    }
    lines.join("\n")
}

pub fn run(youtube: &YouTube<'_>, args: &[String]) -> Result<String, Fail> {
    let video = Asked::read(VERB, USAGE, args, &[])?.video()?;
    Ok(report(&youtube.info(&video)?, &video))
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;
    use crate::fixture::{self, Bench};

    const LINK: &str = "https://www.youtube.com/watch?v=abcdefghijk";

    fn shown(changes: Value) -> String {
        let bench = Bench::new("info");
        bench.runner.gives(fixture::answered(&fixture::yt_dlp_data(changes)));
        run(&bench.youtube(), &["https://youtu.be/abcdefghijk?t=5".to_string()]).unwrap()
    }

    #[test]
    fn a_video_is_described_in_a_few_lines() {
        let lines = [
            "video  abcdefghijk  Bevel tutorial",
            "channel  Example Channel",
            "length  03:07",
            "picture  1920x1080",
            "sound  yes",
            "language  en",
            "text  subtitles en, read with: wisply pupil captions https://www.youtube.com/watch?v=abcdefghijk",
            "chapters  2",
            "00:00  Intro",
            "00:30  Bevel",
        ];
        assert_eq!(shown(json!({})), lines.join("\n"));
    }

    #[test]
    fn the_text_line_names_the_way_to_the_words() {
        let automatic = shown(json!({"subtitles": {}}));
        assert!(
            automatic.contains(&format!("text  captions en (automatic), read with: wisply pupil captions {LINK}\n"))
        );
        let speech = shown(json!({"subtitles": {}, "automatic_captions": {}}));
        assert!(speech.contains(&format!("text  none, speech with: wisply listen {LINK}\n")), "{speech}");
        let silent =
            json!({"subtitles": {}, "automatic_captions": {}, "formats": [{"acodec": "none", "vcodec": "avc1"}]});
        let silent = shown(silent);
        assert!(silent.contains("sound  no\n") && silent.contains("text  none (no sound)\n"), "{silent}");
    }

    #[test]
    fn missing_details_leave_no_empty_lines() {
        let changes = json!({
            "channel": null, "duration": null, "width": null, "language": null, "chapters": null,
            "availability": "needs_auth", "live_status": "is_upcoming", "formats": [],
        });
        assert_eq!(shown(changes), "video  abcdefghijk  Bevel tutorial\nstate  not started yet\nopen to  needs auth");
        let live = shown(json!({"live_status": "is_live", "duration": null, "chapters": []}));
        assert!(live.contains("state  live now, no fixed timeline yet\n") && !live.contains("text  "), "{live}");
    }

    #[test]
    fn a_failure_comes_back_as_it_is() {
        let bench = Bench::new("info-failure");
        bench.runner.gives(fixture::failed("ERROR: [youtube] abcdefghijk: Private video"));
        let fail = run(&bench.youtube(), &[LINK.to_string()]).unwrap_err();
        assert_eq!(fail.message, "this video is private");
        assert!(run(&bench.youtube(), &[]).unwrap_err().message.contains("no video"));
    }
}
