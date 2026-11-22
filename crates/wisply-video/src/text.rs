use crate::asked::Asked;
use crate::captions::{self, Segment};
use crate::config;
use crate::fail::Fail;
use crate::timecode::{self, Range};
use crate::transcript::{self, Transcript};
use crate::youtube::{Video, YouTube};

pub const VERB: &str = "captions";
pub const USAGE: &str = "wisply pupil captions <YouTube link> [--start S] [--end E] [--find \"words\"]";
const START: &str = "start";
const END: &str = "end";
const FIND: &str = "find";
const FLAGS: [&str; 3] = [START, END, FIND];

struct Wish<'a> {
    range: Option<Range>,
    end: Option<&'a str>,
    find: Option<&'a str>,
    limit: usize,
}

struct Shown {
    lines: Vec<Segment>,
    best_only: bool,
}

fn chosen(youtube: &YouTube<'_>, video: &Video, found: &Transcript, wish: &Wish) -> Result<Shown, Fail> {
    let (start, end) = wish.range.as_ref().map_or((0.0, None), |range| (range.start, range.end));
    let Some(words) = wish.find else {
        return Ok(Shown { lines: captions::within(&found.segments, Some(start), end), best_only: false });
    };
    let mut hits = youtube.store.search_transcript(&video.id, words, start, end, wish.limit + 1)?;
    let best_only = hits.len() > wish.limit;
    hits.truncate(wish.limit);
    hits.sort_by(|first, second| first.start.total_cmp(&second.start));
    Ok(Shown { lines: hits, best_only })
}

fn nothing(wish: &Wish) -> String {
    match wish.find {
        Some(words) if wish.range.is_some() => format!("no line in this part matches \"{words}\""),
        Some(words) => format!("no line matches \"{words}\""),
        None => "no line in this part of the video".to_string(),
    }
}

fn more(rest: usize, from: &Segment, video: &Video, wish: &Wish) -> String {
    let at = timecode::format(from.start);
    let end = wish.end.map(|end| format!(" --{END} {end}")).unwrap_or_default();
    let find = wish.find.map(|words| format!(" --{FIND} \"{words}\"")).unwrap_or_default();
    format!("{rest} more lines from {at}: wisply pupil {VERB} {} --{START} {at}{end}{find}", video.url)
}

fn report(found: &Transcript, shown: &Shown, video: &Video, wish: &Wish, output_chars: usize) -> String {
    let (total, best_only, shown) = (found.segments.len(), shown.best_only, &shown.lines);
    let count =
        if shown.len() == total { format!("{total} lines") } else { format!("{} of {total} lines", shown.len()) };
    let note = if best_only {
        format!(", the {} best matches (--{START} and --{END} narrow it)", wish.limit)
    } else {
        String::new()
    };
    let mut lines = vec![format!("{}  {}  {count}{note}", found.kind, found.language)];
    let mut used = 0;
    for (index, segment) in shown.iter().enumerate() {
        let line = format!("{}  {}", timecode::format(segment.start), segment.text);
        used += line.chars().count() + 1;
        if used > output_chars && index > 0 {
            lines.push(more(shown.len() - index, segment, video, wish));
            break;
        }
        lines.push(line);
    }
    lines.join("\n")
}

pub fn run(youtube: &YouTube<'_>, settings: &config::Transcript, args: &[String]) -> Result<String, Fail> {
    let asked = Asked::read(VERB, USAGE, args, &FLAGS)?;
    let video = asked.video()?;
    timecode::range(asked.flag(START), asked.flag(END), None)?;
    let found = transcript::load(youtube, settings, &video)?;
    if found.kind == transcript::NO_TEXT {
        return Ok(format!("no text  {}", found.note));
    }
    let duration = youtube.cached(&video.id)?.and_then(|info| info.duration);
    let range = timecode::range(asked.flag(START), asked.flag(END), duration)?;
    let wish = Wish { range, end: asked.flag(END), find: asked.flag(FIND), limit: settings.search_results };
    let shown = chosen(youtube, &video, &found, &wish)?;
    if shown.lines.is_empty() {
        return Ok(nothing(&wish));
    }
    Ok(report(&found, &shown, &video, &wish, settings.output_chars))
}

#[cfg(test)]
#[path = "text_tests.rs"]
mod tests;
