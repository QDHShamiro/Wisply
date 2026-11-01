use crate::pat;
use crate::text::{clip, head, rstrip, strip, words};
use std::cmp::Reverse;
use std::collections::HashMap;

const HEAD: (usize, usize, usize) = (5, 10, 20);
const TAIL: (usize, usize, usize) = (10, 20, 10);
const AFTER: usize = 2;
const TRACE_LONG: usize = 12;
const TRACE_TIGHT: usize = 3;
const TRACE_SCAN: usize = 400;
const TRACE_FIRST: usize = 2;
const SAME: usize = 2;
const GAP_KEEP: usize = 2;
const SIMILAR_MIN: usize = 3;
const LINES: usize = 120;
const LINE_CHARS: usize = 300;
const KIND_CHARS: usize = 60;
const CUT_TAIL: usize = 20;

pub(crate) fn clean(text: &str) -> Vec<String> {
    let plain = pat::strip_ansi(text);
    let mut out: Vec<String> = plain
        .split('\n')
        .map(|line| {
            let line = line.strip_suffix('\r').unwrap_or(line);
            let line =
                if line.contains('\r') { line.split('\r').rfind(|p| !strip(p).is_empty()).unwrap_or("") } else { line };
            rstrip(line).to_string()
        })
        .collect();
    while out.last().is_some_and(String::is_empty) {
        out.pop();
    }
    out
}

struct Line<'a> {
    raw: &'a str,
    tmpl: String,
    flow: bool,
    frame: bool,
    hot: bool,
    noise: bool,
    summary: bool,
}

impl<'a> Line<'a> {
    fn new(raw: &'a str) -> Self {
        let body = pat::body(raw);
        let frame = pat::trace(body);
        let flow = frame || body.starts_with([' ', '\t']);
        let kind = pat::kind(raw);
        let hot = !frame && kind.hot;
        let summary = !hot && pat::summary(body);
        let tmpl = words(&pat::numbers(body)).collect::<Vec<_>>().join(" ");
        Line { raw, tmpl, flow, frame, hot, noise: kind.noise && !summary, summary }
    }

    fn kind(&self) -> &str {
        let end = self.tmpl.match_indices(' ').nth(2).map_or(self.tmpl.len(), |(i, _)| i);
        head(&self.tmpl[..end], KIND_CHARS)
    }

    fn runtime(&self) -> bool {
        self.frame && pat::runtime(self.raw)
    }
}

fn span(n: usize, (low, high, per): (usize, usize, usize)) -> usize {
    (n / per).clamp(low, high)
}

fn marked(lines: &[Line], after: usize, trace: usize) -> Vec<bool> {
    let n = lines.len();
    let real: Vec<usize> = (0..n).filter(|&i| !lines[i].noise).collect();
    let mut keep = vec![false; n];
    let (top, end) = (span(real.len(), HEAD), span(real.len(), TAIL));
    for &i in real.iter().take(top).chain(real.iter().rev().take(end)) {
        keep[i] = true;
    }
    let mut seen: HashMap<&str, usize> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        if !line.hot && !line.summary {
            continue;
        }
        let count = seen.entry(line.tmpl.as_str()).or_insert(0);
        *count += 1;
        if *count > SAME {
            continue;
        }
        keep[i] = true;
        if line.summary {
            continue;
        }
        let (mut j, mut shown) = (i + 1, 0);
        while j < n && j - i <= TRACE_SCAN && lines[j].flow {
            if shown < trace && (j - i <= TRACE_FIRST || !lines[j].runtime()) {
                keep[j] = true;
                shown += 1;
            }
            j += 1;
        }
        for k in j..n.min(j + after) {
            keep[k] |= !lines[k].noise;
        }
    }
    keep
}

fn gap(run: &[&Line]) -> String {
    if run.iter().all(|l| l.frame) {
        let indent = &run[0].raw[..run[0].raw.len() - run[0].raw.trim_start().len()];
        return format!("{indent}... {} frames", run.len());
    }
    let mut kinds: Vec<(&str, usize, bool)> = Vec::new();
    let mut at: HashMap<&str, usize> = HashMap::new();
    for line in run.iter().filter(|l| !strip(l.raw).is_empty()) {
        let kind = line.kind();
        match at.get(kind) {
            Some(&i) => kinds[i].1 += 1,
            None => {
                at.insert(kind, kinds.len());
                kinds.push((kind, 1, line.hot));
            }
        }
    }
    kinds.sort_by_key(|k| (Reverse(k.2), Reverse(k.1)));
    let what: Vec<String> = kinds
        .iter()
        .filter(|(k, c, hot)| !k.is_empty() && (*c > 1 || *hot))
        .take(2)
        .map(|(k, c, _)| format!("{c} x {k}"))
        .collect();
    if what.is_empty() {
        format!("... {} lines", run.len())
    } else {
        format!("... {} lines ({})", run.len(), what.join(", "))
    }
}

fn similar(a: &str, b: &str) -> bool {
    let (mut x, mut y) = (a.split(' '), b.split(' '));
    let mut diff = 0;
    loop {
        match (x.next(), y.next()) {
            (None, None) => return diff == 1,
            (Some(p), Some(q)) if p == q => {}
            (Some(_), Some(_)) if diff == 0 => diff = 1,
            _ => return false,
        }
    }
}

enum Item<'l, 'a> {
    Line(&'l Line<'a>),
    Gap(String),
}

fn pieces<'l, 'a>(lines: &'l [Line<'a>], keep: &[bool]) -> Vec<Item<'l, 'a>> {
    let mut out = Vec::new();
    let mut run: Vec<&Line> = Vec::new();
    let flush = |run: &mut Vec<&'l Line<'a>>, out: &mut Vec<Item<'l, 'a>>| {
        if run.len() > GAP_KEEP || (!run.is_empty() && run.iter().all(|l| l.noise)) {
            out.push(Item::Gap(gap(run)));
        } else {
            out.extend(run.iter().filter(|l| !l.noise).map(|l| Item::Line(l)));
        }
        run.clear();
    };
    for (line, &kept) in lines.iter().zip(keep) {
        if kept {
            flush(&mut run, &mut out);
            out.push(Item::Line(line));
        } else {
            run.push(line);
        }
    }
    flush(&mut run, &mut out);
    out
}

fn assemble(lines: &[Line], keep: &[bool]) -> Vec<(String, bool)> {
    let items = pieces(lines, keep);
    let mut out: Vec<(String, bool)> = Vec::new();
    let mut i = 0;
    while i < items.len() {
        let first = match &items[i] {
            Item::Gap(g) => {
                out.push((g.clone(), false));
                i += 1;
                continue;
            }
            Item::Line(l) => *l,
        };
        let mut j = i + 1;
        while let Some(Item::Line(l)) = items.get(j)
            && !l.tmpl.is_empty()
            && l.tmpl == first.tmpl
        {
            j += 1;
        }
        let mut line = clip(first.raw, LINE_CHARS);
        if j - i > 1 {
            line.push_str(&format!(" (x{})", j - i));
        } else if !first.hot && !first.frame && first.tmpl.contains(' ') {
            while let Some(Item::Line(l)) = items.get(j)
                && !l.hot
                && !l.summary
                && !l.frame
                && similar(&first.tmpl, &l.tmpl)
            {
                j += 1;
            }
            if j - i >= SIMILAR_MIN {
                line.push_str(&format!(" (+{} similar)", j - i - 1));
            } else {
                j = i + 1;
            }
        }
        out.push((line, first.hot));
        i = j;
    }
    out
}

pub(crate) fn squeeze(raw: &[String]) -> Vec<String> {
    let lines: Vec<Line> = raw.iter().map(|r| Line::new(r)).collect();
    let mut out = Vec::new();
    for (after, trace) in [(AFTER, TRACE_LONG), (0, TRACE_TIGHT), (0, 0)] {
        out = assemble(&lines, &marked(&lines, after, trace));
        if out.len() <= LINES {
            return out.into_iter().map(|(l, _)| l).collect();
        }
    }
    let top = LINES - CUT_TAIL;
    let end = out.len() - CUT_TAIL;
    let mut cut: Vec<String> = out[..top].iter().map(|(l, _)| l.clone()).collect();
    let dropped = out[top..end].iter().filter(|(_, hot)| !hot).count();
    if dropped > 0 {
        cut.push(format!("... {dropped} more squeezed lines (see the full file)"));
    }
    cut.extend(out[top..end].iter().filter(|(_, hot)| *hot).map(|(l, _)| l.clone()));
    cut.extend(out[end..].iter().map(|(l, _)| l.clone()));
    cut
}

#[cfg(test)]
mod tests {
    use super::*;

    fn owned(lines: &[&str]) -> Vec<String> {
        lines.iter().map(|l| (*l).to_string()).collect()
    }

    #[test]
    fn clean_drops_colors_carriage_returns_and_trailing_blanks() {
        let got = clean("\x1b[32mok\x1b[0m\r\n10%\r50%\r100%\r\n  \r \r\nend  \n\n \n");
        assert_eq!(got, ["ok", "100%", "", "end"]);
        assert!(clean("").is_empty());
    }

    #[test]
    fn kind_is_the_first_three_template_words() {
        let line = Line::new("[12:00:01 INFO]: Player 42 joined from 10.0.0.1 port 5");
        assert_eq!(line.tmpl, "Player # joined from #.#.#.# port #");
        assert_eq!(line.kind(), "Player # joined");
        assert!(Line::new("\tat a.B(B.java:1)").flow);
        assert!(!Line::new("\tat a.B(B.java:1) error").hot);
        assert!(Line::new("  ● Test suite failed to run").hot);
    }

    #[test]
    fn repeated_kept_lines_collapse() {
        let raw: Vec<String> = (0..100).map(|i| format!("tick {i}")).collect();
        let out = squeeze(&raw);
        assert_eq!(out, ["tick 0 (x5)", "... 85 lines (85 x tick #)", "tick 90 (x10)"]);
    }

    #[test]
    fn head_and_tail_grow_with_the_output() {
        assert_eq!((span(40, HEAD), span(40, TAIL)), (5, 10));
        assert_eq!((span(150, HEAD), span(150, TAIL)), (7, 15));
        assert_eq!((span(5000, HEAD), span(5000, TAIL)), (10, 20));
    }

    #[test]
    fn similar_lines_fold_but_errors_never() {
        let mut raw: Vec<String> = ["alpha", "beta", "gamma", "delta", "eps", "zeta", "eta"]
            .iter()
            .map(|c| format!("   Compiling {c} v1.0.0"))
            .collect();
        raw.extend((0..80).map(|i| format!("row {i} fine")));
        raw.push("error: could not find `alpha` in `x`".into());
        raw.push("error: could not find `beta` in `x`".into());
        raw.extend((0..20).map(|i| format!("row {i} fine")));
        let out = squeeze(&raw);
        assert_eq!(out[0], "   Compiling alpha v1.0.0 (+4 similar)");
        assert!(out.contains(&"error: could not find `alpha` in `x`".to_string()), "{out:#?}");
        assert!(out.contains(&"error: could not find `beta` in `x`".to_string()), "{out:#?}");
        assert!(similar("a b c", "a x c") && !similar("a b c", "a b c") && !similar("a b c", "x y c"));
    }

    #[test]
    fn progress_lines_never_take_head_or_tail_room() {
        let mut raw: Vec<String> = (0..50).map(|i| format!("Progress (1): {i}.0 kB")).collect();
        raw.push("[INFO] BUILD FAILURE".into());
        raw.extend((0..60).map(|i| format!("Downloaded from central: https://repo/x{i}.pom (2 kB at 9 kB/s)")));
        raw.extend(owned(&["[ERROR] Failed to execute goal compile on project shop", "[INFO] Total time: 6 s"]));
        let out = squeeze(&raw);
        assert_eq!(
            out,
            [
                "... 50 lines (50 x Progress (#): #.#)",
                "[INFO] BUILD FAILURE",
                "... 60 lines (60 x Downloaded from central:)",
                "[ERROR] Failed to execute goal compile on project shop",
                "[INFO] Total time: 6 s"
            ]
        );
    }

    #[test]
    fn framework_frames_fold_and_app_frames_stay() {
        let mut raw = owned(&[
            "[21:07:05 ERROR]: Could not pass event InventoryClickEvent to HugoShop v2.3.1",
            "java.lang.NullPointerException: item is null",
            "\tat dev.qdhshamiro.hugoshop.gui.ShopListener.onClick(ShopListener.java:88)",
        ]);
        raw.extend((0..20).map(|i| format!("\tat java.base/java.lang.reflect.Method{i}.invoke(Method.java:{i})")));
        raw.push("\tat dev.qdhshamiro.hugoshop.Main.run(Main.java:12)".into());
        raw.extend((0..15).map(|i| format!("\tat org.bukkit.plugin.Loader{i}.call(Loader.java:{i})")));
        raw.push("\t... 14 more".into());
        let mut log: Vec<String> = (0..80).map(|i| format!("[21:00:{i:02} INFO]: tick {i}")).collect();
        log.extend(raw);
        log.extend((0..40).map(|i| format!("[21:09:{i:02} INFO]: tock {i}")));
        let out = squeeze(&log);
        let at = out.iter().position(|l| l.contains("ShopListener.onClick")).expect("app frame");
        assert_eq!(out[at - 1], "java.lang.NullPointerException: item is null");
        assert!(out[at + 1].contains("Method0.invoke"), "{out:#?}");
        assert_eq!(out[at + 2], "\t... 19 frames");
        assert_eq!(out[at + 3], "\tat dev.qdhshamiro.hugoshop.Main.run(Main.java:12)");
        assert_eq!(out[at + 4], "\t... 15 frames");
        assert_eq!(out[at + 5], "\t... 14 more");
    }

    #[test]
    fn summaries_survive_and_hot_repeats_lead_the_gap_note() {
        let mut raw: Vec<String> = (0..60).map(|i| format!("test case_{i} ... ok")).collect();
        for i in 0..6 {
            raw.push(format!("[21:02:{i:02} WARN]: Can't keep up! Running {i}000ms behind"));
            raw.extend((0..5).map(|k| format!("test more_{i}_{k} ... ok")));
        }
        raw.push("test result: ok. 96 passed; 0 failed; 0 ignored; finished in 0.31s".into());
        raw.extend((0..30).map(|i| format!("   Doc-tests crate_{i}")));
        let out = squeeze(&raw);
        assert!(out.iter().any(|l| l.starts_with("test result: ok. 96 passed")), "{out:#?}");
        assert_eq!(out.iter().filter(|l| l.contains("Can't keep up")).count(), 2 + 1, "{out:#?}");
        assert!(out.iter().any(|l| l.contains("(4 x Can't keep up!")), "{out:#?}");
    }
}
