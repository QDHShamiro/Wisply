use crate::config::{Config, Profile, Tokens, Watch};
use crate::fail::{Fail, Kind};
use crate::schema::Mode;
use crate::timecode;

const HIGH: &str = "high";
const AGENTIC: &str = "agentic";
const ONE_FRAME: &str = "1 frame per second";
const SUMMARY_TIP: &str = ", or use --mode summary for an overview";
const UNKNOWN_LENGTH: &str = "The video length is unknown, so the whole video went in one request. If the answer is \
                              cut off, watch parts with --start and --end.";
const SKIMMED: &str = "Gemini skimmed the video on its own (agentic), so times can be far off. Ask for a part with \
                       --start and --end for exact times.";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: f64,
    pub end: Option<f64>,
    pub whole: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Plan {
    pub name: String,
    pub resolution: String,
    pub fps: Option<f64>,
    pub tag: String,
    pub output: i64,
    pub video_tokens: i64,
    pub note: String,
}

fn frames(profile: &Profile) -> Option<f64> {
    if profile.agentic { None } else { profile.fps }
}

fn frame_tokens(profile: &Profile, tokens: &Tokens) -> f64 {
    if profile.resolution == HIGH { tokens.video_frame_high } else { tokens.video_frame_low }
}

pub fn tokens_per_second(profile: &Profile, tokens: &Tokens) -> f64 {
    frames(profile).map_or(0.0, |fps| fps * frame_tokens(profile, tokens) + tokens.audio_per_second)
}

pub fn tag(name: &str, profile: &Profile, output: i64) -> String {
    let seen = frames(profile).map_or_else(|| AGENTIC.to_string(), |fps| format!("{fps}fps"));
    format!("{name}:{}:{seen}:o{output}", profile.resolution)
}

pub fn text_tokens(text: &str, tokens: &Tokens) -> i64 {
    (text.chars().count() as f64 / tokens.text_chars_per_token).ceil() as i64
}

pub fn budget(config: &Config) -> i64 {
    let limits = &config.limits;
    let known = config.gemini.model_ladder.iter().filter_map(|model| limits.models.get(model));
    let smallest = known.map(|model| model.tokens_per_minute).reduce(f64::min).unwrap_or_default();
    (smallest * limits.usage_share).floor() as i64
}

fn ladder(mode: Mode, watch: &Watch) -> Vec<(&str, &Profile)> {
    let names = watch.ladders.get(mode.name()).map(Vec::as_slice).unwrap_or_default();
    names.iter().filter_map(|name| Some((name.as_str(), watch.profiles.get(name)?))).collect()
}

fn built(name: &str, profile: &Profile, output: i64, video_tokens: i64, note: &str) -> Plan {
    Plan {
        name: name.to_string(),
        resolution: profile.resolution.clone(),
        fps: frames(profile),
        tag: tag(name, profile, output),
        output,
        video_tokens,
        note: note.to_string(),
    }
}

fn no_room(needed: i64) -> Fail {
    let message = format!("the prompt and the answer alone need {needed} tokens, more than one request may use");
    let hint = format!("lower watch.output_tokens or raise limits.models tokens_per_minute in {}", crate::CONFIG_FILE);
    Fail::hint(Kind::Input, message, hint)
}

fn no_profile(mode: Mode) -> Fail {
    let message = format!("mode {} has no watch profile with a frame rate", mode.name());
    Fail::hint(Kind::Input, message, format!("check watch.ladders in {}", crate::CONFIG_FILE))
}

fn too_long(mode: Mode, seconds: f64, name: &str, longest: f64) -> Fail {
    let length = timecode::format(seconds);
    let message = format!("{length} of video does not fit in one request, even with the leanest profile ({name})");
    let tip = if mode == Mode::Summary { "" } else { SUMMARY_TIP };
    let hint = format!("watch a part of at most {} with --start and --end{tip}", timecode::format(longest));
    Fail::hint(Kind::Input, message, hint)
}

pub fn plan(mode: Mode, span: &Span, prompt_tokens: i64, config: &Config) -> Result<Plan, Fail> {
    let watch = &config.watch;
    let output = watch.output_tokens.get(mode.name()).copied().unwrap_or_default();
    let room = budget(config) - prompt_tokens - output;
    if room <= 0 {
        return Err(no_room(prompt_tokens + output));
    }
    let ladder = ladder(mode, watch);
    let Some((lean_name, lean)) = ladder.iter().rev().find(|(_, profile)| !profile.agentic).copied() else {
        return Err(no_profile(mode));
    };
    let Some(end) = span.end else {
        return Ok(built(lean_name, lean, output, room, UNKNOWN_LENGTH));
    };
    let seconds = (end - span.start).max(0.0);
    for &(name, profile) in &ladder {
        if profile.agentic {
            if span.whole {
                return Ok(built(name, profile, output, watch.agentic_tokens, ""));
            }
            continue;
        }
        let needed = seconds * tokens_per_second(profile, &config.tokens);
        if needed <= room as f64 {
            return Ok(built(name, profile, output, needed.ceil() as i64, ""));
        }
    }
    Err(too_long(mode, seconds, lean_name, room as f64 / tokens_per_second(lean, &config.tokens)))
}

fn rate(fps: f64) -> String {
    if (fps - 1.0).abs() < f64::EPSILON { ONE_FRAME.to_string() } else { format!("{fps} frames per second") }
}

pub fn notes(plan: &Plan, mode: Mode, watch: &Watch) -> Vec<String> {
    let mut notes: Vec<String> = Vec::new();
    if !plan.note.is_empty() {
        notes.push(plan.note.clone());
    }
    let Some(fps) = plan.fps else {
        notes.push(SKIMMED.to_string());
        return notes;
    };
    let richest = watch.ladders.get(mode.name()).and_then(|names| names.first());
    if richest.is_some_and(|name| *name != plan.name) {
        let seen = format!("{} resolution, {}", plan.resolution, rate(fps));
        notes.push(format!("Watched with the {} profile ({seen}) so it fits; tiny text can be missed.", plan.name));
    }
    if fps < watch.coarse_sound_fps {
        notes.push(format!("At {} the sound list is only coarse.", rate(fps)));
    }
    notes
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROMPT: i64 = 1000;

    fn whole(seconds: f64) -> Span {
        Span { start: 0.0, end: Some(seconds), whole: true }
    }

    fn planned(mode: Mode, span: Span) -> Plan {
        plan(mode, &span, PROMPT, &Config::standard()).expect("plan")
    }

    #[test]
    fn a_short_video_gets_the_richest_profile() {
        let plan = planned(Mode::Steps, whole(600.0));
        assert_eq!(plan.name, "high_1");
        assert_eq!((plan.resolution.as_str(), plan.fps), ("high", Some(1.0)));
        assert_eq!(plan.tag, "high_1:high:1fps:o8000");
        assert_eq!((plan.output, plan.video_tokens), (8000, 173_400));
        assert!(plan.note.is_empty());
        assert_eq!(planned(Mode::Full, whole(600.0)).tag, "high_1:high:1fps:o10000");
    }

    #[test]
    fn a_longer_video_steps_down_the_ladder() {
        let half = planned(Mode::Steps, whole(1200.0));
        assert_eq!((half.tag.as_str(), half.video_tokens), ("high_half:high:0.5fps:o8000", 188_400));
        let low = planned(Mode::Steps, Span { start: 100.0, end: Some(2100.0), whole: false });
        assert_eq!((low.tag.as_str(), low.video_tokens), ("low_1:low:1fps:o8000", 182_000));
        assert_eq!(planned(Mode::Summary, whole(3600.0)).tag, "low_tenth:low:0.1fps:o3000");
    }

    #[test]
    fn a_video_past_the_leanest_profile_is_refused_with_the_longest_part() {
        let fail = plan(Mode::Steps, &whole(3600.0), PROMPT, &Config::standard()).expect_err("too long");
        assert_eq!(fail.kind, Kind::Input);
        let wanted = "1:00:00 of video does not fit in one request, even with the leanest profile (low_1)";
        assert_eq!(fail.message, wanted);
        assert_eq!(
            fail.hint,
            "watch a part of at most 34:58 with --start and --end, or use --mode summary for an overview"
        );
    }

    #[test]
    fn a_whole_long_video_is_skimmed_in_summary_mode() {
        let skimmed = planned(Mode::Summary, whole(7200.0));
        assert_eq!((skimmed.name.as_str(), skimmed.fps), ("agentic", None));
        assert_eq!((skimmed.tag.as_str(), skimmed.video_tokens), ("agentic:low:agentic:o3000", 15_000));
        let part = Span { start: 0.0, end: Some(7200.0), whole: false };
        let fail = plan(Mode::Summary, &part, PROMPT, &Config::standard()).expect_err("a part is never skimmed");
        assert!(
            fail.message.starts_with("2:00:00 of video") && fail.message.ends_with("(low_tenth)"),
            "{}",
            fail.message
        );
        assert_eq!(fail.hint, "watch a part of at most 1:43:22 with --start and --end");
    }

    #[test]
    fn an_unknown_length_goes_out_in_one_request_with_the_leanest_profile() {
        let plan = planned(Mode::Steps, Span { start: 0.0, end: None, whole: true });
        assert_eq!((plan.name.as_str(), plan.video_tokens), ("low_1", 191_000));
        assert_eq!(plan.note, UNKNOWN_LENGTH);
        let summary = planned(Mode::Summary, Span { start: 30.0, end: None, whole: false });
        assert_eq!((summary.name.as_str(), summary.video_tokens), ("low_tenth", 196_000));
    }

    #[test]
    fn a_prompt_that_fills_the_budget_is_refused() {
        let fail = plan(Mode::Steps, &whole(60.0), 195_000, &Config::standard()).expect_err("no room");
        assert_eq!(fail.kind, Kind::Input);
        assert_eq!(fail.message, "the prompt and the answer alone need 203000 tokens, more than one request may use");
        assert!(
            fail.hint.starts_with("lower watch.output_tokens") && fail.hint.ends_with("video.toml"),
            "{}",
            fail.hint
        );
    }

    #[test]
    fn a_ladder_without_a_frame_rate_is_named() {
        let mut config = Config::standard();
        config.watch.ladders.insert("summary".to_string(), vec!["agentic".to_string()]);
        let fail = plan(Mode::Summary, &whole(60.0), PROMPT, &config).expect_err("no profile");
        assert_eq!(fail.message, "mode summary has no watch profile with a frame rate");
    }

    #[test]
    fn text_is_counted_by_its_characters() {
        let tokens = Config::standard().tokens;
        assert_eq!(text_tokens("", &tokens), 0);
        assert_eq!(text_tokens("abc", &tokens), 1);
        assert_eq!(text_tokens("abcdefg", &tokens), 3);
        assert_eq!(text_tokens("äöü", &tokens), 1);
    }

    #[test]
    fn the_budget_follows_the_tightest_model_of_the_ladder() {
        let mut config = Config::standard();
        assert_eq!(budget(&config), 200_000);
        if let Some(model) = config.limits.models.get_mut("gemini-3.6-flash") {
            model.tokens_per_minute = 100_000.0;
        }
        assert_eq!(budget(&config), 80_000);
        config.gemini.model_ladder = vec!["gemini-3.8-flash".to_string()];
        assert_eq!(budget(&config), 200_000);
    }

    #[test]
    fn a_second_of_video_costs_its_frames_and_its_sound() {
        let config = Config::standard();
        let cost = |name: &str| tokens_per_second(&config.watch.profiles[name], &config.tokens);
        assert_eq!(cost("high_1"), 289.0);
        assert_eq!(cost("high_half"), 157.0);
        assert_eq!(cost("low_1"), 91.0);
        assert_eq!(cost("agentic"), 0.0);
    }

    #[test]
    fn the_notes_say_what_was_given_up() {
        let watch = Config::standard().watch;
        let told = |mode: Mode, span: Span| notes(&planned(mode, span), mode, &watch);
        assert!(told(Mode::Steps, whole(600.0)).is_empty());
        let half = told(Mode::Steps, whole(1200.0));
        let leaner = "Watched with the high_half profile (high resolution, 0.5 frames per second) so it fits; tiny \
                      text can be missed.";
        assert_eq!(half, [leaner, "At 0.5 frames per second the sound list is only coarse."]);
        let low = told(Mode::Full, whole(2000.0));
        assert_eq!(low.len(), 1);
        assert!(low[0].contains("(low resolution, 1 frame per second)"), "{}", low[0]);
        assert_eq!(told(Mode::Summary, whole(600.0)), ["At 0.1 frames per second the sound list is only coarse."]);
        assert_eq!(told(Mode::Summary, whole(7200.0)), [SKIMMED]);
        let unknown = told(Mode::Steps, Span { start: 0.0, end: None, whole: true });
        assert_eq!(unknown.len(), 2);
        assert_eq!(unknown[0], UNKNOWN_LENGTH);
        assert!(unknown[1].starts_with("Watched with the low_1 profile"), "{}", unknown[1]);
    }
}
