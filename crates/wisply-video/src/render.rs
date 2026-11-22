use crate::analysis::{Analysis, Moment, Step};
use crate::config;
use crate::fail::{Fail, Kind};
use crate::schema::Mode;
use crate::timecode::{self, Range};

const SEPARATOR: &str = " · ";
const CODE_INDENT: &str = "   ";
const MENU_JOINER: &str = " > ";
const CODE_FENCE_MIN: usize = 3;
const NOTHING_FOUND: &str = "Gemini found nothing to report here.";
const STEPS_HEADING: &str = "Steps:";
const POINTS_HEADING: &str = "Key points:";
const SOUNDS_HEADING: &str = "Sounds:";
const NOTE_PREFIX: &str = "Note: ";
const CUT_NOTE: &str =
    "Gemini's answer was cut off, later parts may be missing. Watch the rest with --start and --end.";
const UNTITLED: &str = "Video";
const CACHED: &str = "cached";

#[derive(Debug, Clone)]
pub struct Heading<'a> {
    pub title: &'a str,
    pub mode: Mode,
    pub model: &'a str,
    pub requests: usize,
    pub duration: Option<f64>,
    pub window: Option<Range>,
}

struct Block {
    text: String,
    unsure: Option<f64>,
}

fn plain(text: String) -> Block {
    Block { text, unsure: None }
}

fn longest_backticks(text: &str) -> usize {
    text.split(|sign| sign != '`').map(str::len).max().unwrap_or(0)
}

fn code_span(text: &str) -> String {
    let fence = "`".repeat(longest_backticks(text) + 1);
    let pad = if text.starts_with('`') || text.ends_with('`') { " " } else { "" };
    format!("{fence}{pad}{text}{pad}{fence}")
}

fn code_block(text: &str) -> String {
    let fence = "`".repeat(CODE_FENCE_MIN.max(longest_backticks(text) + 1));
    let lines = [fence.as_str()].into_iter().chain(text.split('\n')).chain([fence.as_str()]);
    let indented: Vec<String> =
        lines.map(|line| if line.is_empty() { String::new() } else { format!("{CODE_INDENT}{line}") }).collect();
    indented.join("\n")
}

fn stamp(at: Option<f64>) -> String {
    at.map(|at| format!("[{}] ", timecode::format(at))).unwrap_or_default()
}

fn heading_line(heading: &Heading) -> String {
    let words: Vec<&str> = heading.title.split_whitespace().collect();
    let mut title = if words.is_empty() { UNTITLED.to_string() } else { words.join(" ") };
    if let Some(length) = heading.duration.filter(|length| *length != 0.0) {
        title = format!("{title} ({})", timecode::format(length));
    }
    let mut parts = vec![title];
    parts.extend(heading.window.map(|window| window.label()));
    parts.push(heading.mode.name().to_string());
    parts.push(heading.model.to_string());
    parts.push(match heading.requests {
        0 => CACHED.to_string(),
        1 => "1 request".to_string(),
        many => format!("{many} requests"),
    });
    format!("# {}", parts.join(SEPARATOR))
}

fn moment_line(moment: &Moment) -> String {
    format!("- {}{}", stamp(moment.at), moment.text)
}

fn step_text(number: usize, step: &Step, sounds: &[&Moment]) -> String {
    let mut parts = vec![format!("{number}. {}{}", stamp(step.at), step.action)];
    if !step.keys.is_empty() {
        let keys: Vec<String> = step.keys.iter().map(|key| code_span(key)).collect();
        parts.push(format!("keys {}", keys.join(", ")));
    }
    if !step.menus.is_empty() {
        parts.push(format!("menu {}", code_span(&step.menus.join(MENU_JOINER))));
    }
    if !step.values.is_empty() {
        let named = |name: &str, value: &str| format!("{name} {}", code_span(value)).trim().to_string();
        let listed: Vec<String> = step.values.iter().map(|item| named(&item.name, &item.value)).collect();
        parts.push(format!("vals {}", listed.join(", ")));
    }
    let multi_line = step.code.contains('\n');
    if !step.code.is_empty() && !multi_line {
        parts.push(format!("code {}", code_span(&step.code)));
    }
    if !step.result.is_empty() {
        parts.push(format!("see: {}", step.result));
    }
    if step.unsure {
        parts.push("unsure".to_string());
    }
    let mut lines = vec![parts.join(SEPARATOR)];
    if multi_line {
        lines.push(code_block(&step.code));
    }
    let heard = |sound: &&Moment| format!("{CODE_INDENT}snd {}{}", stamp(sound.at), sound.text).trim_end().to_string();
    lines.extend(sounds.iter().map(heard));
    lines.join("\n")
}

fn owning_step(timed: &[(f64, usize)], at: Option<f64>, reach: f64) -> Option<usize> {
    let at = at?;
    let mut owner = None;
    for (position, (start, index)) in timed.iter().enumerate() {
        if *start > at {
            break;
        }
        let following = timed.get(position + 1).map(|next| next.0);
        if at - start <= reach && following.is_none_or(|next| at < next) {
            owner = Some(*index);
        }
    }
    owner
}

fn attach_sounds<'a>(steps: &[Step], sounds: &'a [Moment], reach: f64) -> (Vec<Vec<&'a Moment>>, Vec<&'a Moment>) {
    let mut timed: Vec<(f64, usize)> =
        steps.iter().enumerate().filter_map(|(index, step)| step.at.map(|at| (at, index))).collect();
    timed.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
    let mut attached = vec![Vec::new(); steps.len()];
    let mut left = Vec::new();
    for sound in sounds {
        match owning_step(&timed, sound.at, reach) {
            Some(owner) => attached[owner].push(sound),
            None => left.push(sound),
        }
    }
    (attached, left)
}

fn headed(heading: &str, mut blocks: Vec<Block>) -> Vec<Block> {
    if let Some(first) = blocks.first_mut() {
        first.text = format!("{heading}\n{}", first.text);
    }
    blocks
}

fn result_blocks(analysis: &Analysis, settings: &config::Render) -> Vec<Block> {
    let mut blocks = Vec::new();
    if !analysis.summary.is_empty() {
        blocks.push(plain(analysis.summary.clone()));
    }
    blocks.extend(headed(POINTS_HEADING, analysis.points.iter().map(|point| plain(moment_line(point))).collect()));
    let (attached, left) = attach_sounds(&analysis.steps, &analysis.sounds, settings.sound_attach_seconds);
    let numbered = analysis.steps.iter().zip(&attached).enumerate();
    let steps: Vec<Block> = numbered
        .map(|(index, (step, sounds))| Block {
            text: step_text(index + 1, step, sounds),
            unsure: step.at.filter(|_| step.unsure),
        })
        .collect();
    blocks.extend(if analysis.summary.is_empty() { steps } else { headed(STEPS_HEADING, steps) });
    blocks.extend(headed(SOUNDS_HEADING, left.into_iter().map(|sound| plain(moment_line(sound))).collect()));
    if blocks.is_empty() {
        blocks.push(plain(NOTHING_FOUND.to_string()));
    }
    blocks
}

fn paginate(blocks: Vec<Block>, page_chars: usize) -> Vec<Vec<Block>> {
    let mut pages = Vec::new();
    let mut page: Vec<Block> = Vec::new();
    let mut size = 0;
    for block in blocks {
        let cost = block.text.chars().count() + 1;
        if !page.is_empty() && size + cost > page_chars {
            pages.push(std::mem::take(&mut page));
            size = 0;
        }
        page.push(block);
        size += cost;
    }
    pages.push(page);
    pages
}

fn quality_notes(analysis: &Analysis) -> Vec<String> {
    let mut notes = Vec::new();
    if analysis.dropped > 0 {
        notes.push(format!("{} unreadable item(s) from Gemini were left out.", analysis.dropped));
    }
    if analysis.cut {
        notes.push(CUT_NOTE.to_string());
    }
    notes
}

fn footer_line(blocks: &[Block], number: usize, total: usize, most_times: usize) -> String {
    let mut parts = Vec::new();
    if total > 1 {
        parts.push(format!("Page {number}/{total}"));
        if number < total {
            parts.push(format!("next: --page {}", number + 1));
        }
    }
    let mut unsure: Vec<f64> = blocks.iter().filter_map(|block| block.unsure).collect();
    unsure.sort_by(f64::total_cmp);
    unsure.dedup();
    unsure.truncate(most_times);
    if !unsure.is_empty() {
        let times: Vec<String> = unsure.into_iter().map(timecode::format).collect();
        parts.push(format!("unsure at {}", times.join(", ")));
    }
    parts.join(SEPARATOR)
}

pub fn page(
    analysis: &Analysis,
    heading: &Heading,
    number: usize,
    settings: &config::Render,
    notes: &[String],
) -> Result<String, Fail> {
    let mut pages = paginate(result_blocks(analysis, settings), settings.page_chars);
    let total = pages.len();
    if number == 0 || number > total {
        let message = format!("page {number} does not exist, this result has {total} page(s)");
        return Err(Fail::hint(Kind::Input, message, format!("use --page 1 to {total}")));
    }
    let blocks = pages.swap_remove(number - 1);
    let mut lines = vec![heading_line(heading)];
    if number == 1 {
        let all = notes.iter().cloned().chain(quality_notes(analysis));
        lines.extend(all.map(|note| format!("{NOTE_PREFIX}{note}")));
    }
    lines.push(String::new());
    lines.extend(blocks.iter().map(|block| block.text.clone()));
    let footer = footer_line(&blocks, number, total, settings.unsure_times);
    if !footer.is_empty() {
        lines.push(String::new());
        lines.push(footer);
    }
    Ok(lines.join("\n"))
}

#[cfg(test)]
#[path = "render_tests.rs"]
mod tests;
