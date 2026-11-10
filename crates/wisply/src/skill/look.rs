use std::fs;
use std::path::Path;

use wisply_core::Fail;
use wisply_think::skill::{head, lint};

use super::{ABOUT_SHORT, BYTES_MAX, FILES_MAX, MARK, SCRIPT_FOLDER, SCRIPT_KINDS, SKILL, WHEN_WORDS, leaf, slash};
use crate::junk::size;

pub(super) fn walk(root: &Path, dir: &Path, out: &mut Vec<(String, u64)>) {
    for e in fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = e.path();
        if e.file_type().is_ok_and(|t| t.is_dir()) {
            walk(root, &path, out);
        } else if let Ok(rel) = path.strip_prefix(root) {
            out.push((slash(rel), e.metadata().map_or(0, |m| m.len())));
        }
    }
}

pub(super) fn is_script(rel: &str) -> bool {
    let kind = Path::new(rel).extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
    rel != MARK && (rel.starts_with(SCRIPT_FOLDER) || SCRIPT_KINDS.contains(&kind.as_str()))
}

fn unseen(c: char) -> bool {
    matches!(
        c,
        '\u{200B}' | '\u{200E}' | '\u{200F}' | '\u{202A}'..='\u{202E}' | '\u{2060}'..='\u{2064}' | '\u{2066}'..='\u{2069}'
            | '\u{E0000}'..='\u{E007F}'
    )
}

pub(super) struct Checked {
    pub(super) name: String,
    pub(super) errs: Vec<String>,
    pub(super) warns: Vec<String>,
    pub(super) scripts: Vec<String>,
    pub(super) code: String,
    pub(super) veiled: Option<&'static str>,
}

pub(super) fn inspect(dir: &Path) -> Result<Checked, Fail> {
    let text = fs::read_to_string(dir.join(SKILL))
        .map_err(|_| Fail::hint(format!("no SKILL.md in {}", slash(dir)), "wisply skill new (what a skill needs)"))?;
    let (mut errs, mut warns) = lint(dir);
    let h = head(&text);
    let name = h.as_ref().map(|h| h.name.clone()).filter(|n| !n.is_empty()).unwrap_or_else(|| leaf(dir));
    if let Some(h) = &h {
        let about = h.description.to_lowercase();
        if h.body.trim().is_empty() {
            errs.push("no text below the frontmatter".into());
        }
        if !about.is_empty() && about.chars().count() < ABOUT_SHORT {
            warns.push(format!("description under {ABOUT_SHORT} characters: say what it does and when to use it"));
        } else if !about.is_empty() && !WHEN_WORDS.iter().any(|w| about.contains(w)) {
            warns.push("description never says when to use it (add: Use when ...)".into());
        }
    }
    let mut files = Vec::new();
    walk(dir, dir, &mut files);
    files.sort();
    let bytes: u64 = files.iter().map(|(_, n)| n).sum();
    if files.len() > FILES_MAX || bytes > BYTES_MAX {
        let most = size(BYTES_MAX);
        errs.push(format!("{} files with {} are too much (up to {FILES_MAX} files, {most})", files.len(), size(bytes)));
    }
    let scripts: Vec<String> = files.iter().map(|(rel, _)| rel).filter(|rel| is_script(rel)).cloned().collect();
    let read = |rel: &String| fs::read(dir.join(rel)).map(|b| String::from_utf8_lossy(&b).into_owned()).ok();
    let code = scripts.iter().filter_map(read).collect::<Vec<_>>().join("\n");
    let docs: Vec<String> =
        files.iter().map(|(rel, _)| rel).filter(|rel| rel.ends_with(".md")).filter_map(read).collect();
    let veiled = if docs.iter().any(|d| d.chars().any(unseen)) {
        Some("has invisible characters in its text")
    } else if docs.iter().any(|d| d.contains("<!--")) {
        Some("has text hidden in comments")
    } else {
        None
    };
    Ok(Checked { name, errs, warns, scripts, code, veiled })
}
