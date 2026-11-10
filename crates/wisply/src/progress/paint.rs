const STEP_SHOWN: usize = 36;
const LAST_SHOWN: usize = 28;
const BAR_CELLS: usize = 10;
const MIDWAY_AT: i64 = 34;
const LATE_AT: i64 = 67;
const ESC: char = '\u{1b}';
const EARLY: u8 = 220;
const MIDWAY: u8 = 39;
const LATE: u8 = 70;
const FULL: u8 = 46;
const BRIGHT: u8 = 252;
const GREY: u8 = 245;
const DIM: u8 = 238;

fn short(text: &str, max: usize, ascii: bool) -> String {
    if text.chars().count() <= max {
        return text.to_string();
    }
    let dots = if ascii { "..." } else { "\u{2026}" };
    let head: String = text.chars().take(max).collect();
    format!("{}{dots}", head.trim_end())
}

fn paint(code: u8, text: &str) -> String {
    format!("{ESC}[38;5;{code}m{text}{ESC}[0m")
}

pub fn plain(text: &str) -> String {
    let mut out = String::new();
    let mut skipping = false;
    for c in text.chars() {
        match (skipping, c) {
            (true, 'm') => skipping = false,
            (true, _) => {}
            (false, ESC) => skipping = true,
            (false, _) => out.push(c),
        }
    }
    out
}

fn shade(percent: i64) -> u8 {
    if percent >= 100 {
        FULL
    } else if percent >= LATE_AT {
        LATE
    } else if percent >= MIDWAY_AT {
        MIDWAY
    } else {
        EARLY
    }
}

fn bar(percent: i64, ascii: bool) -> String {
    let filled = usize::try_from(percent.clamp(0, 100)).unwrap_or(0) * BAR_CELLS / 100;
    let (full, empty) = if ascii { ("#", "-") } else { ("\u{2501}", "\u{2501}") };
    format!("{}{}", paint(shade(percent), &full.repeat(filled)), paint(DIM, &empty.repeat(BAR_CELLS - filled)))
}

pub fn draw(percent: Option<i64>, count: &str, text: &str, last: Option<&str>, ascii: bool) -> Option<String> {
    let mut parts: Vec<String> = Vec::new();
    if let Some(percent) = percent {
        parts.push(bar(percent, ascii));
        parts.push(paint(shade(percent), &format!("{percent}%")));
    }
    if !count.is_empty() {
        parts.push(paint(GREY, count));
    }
    if !text.is_empty() {
        parts.push(paint(BRIGHT, &short(text, STEP_SHOWN, ascii)));
    } else if percent.is_some_and(|p| p >= 100) {
        parts.push(paint(FULL, if ascii { "done" } else { "\u{2713}" }));
    }
    if let Some(last) = last {
        let arrow = if ascii { ">" } else { "\u{203a}" };
        parts.push(paint(GREY, &format!("{arrow} {}", short(last, LAST_SHOWN, ascii))));
    }
    (!parts.is_empty()).then(|| parts.join(" "))
}
