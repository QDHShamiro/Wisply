use serde_json::Value;

use super::{TEXT_MAX, cut, word};

const LAST_WORDS: usize = 3;
const SETUP_WORDS: [&str; 6] = ["cd", "export", "set", "source", "pushd", "."];
const LOUD: [char; 10] = ['=', '@', ':', '"', '\'', '|', '>', '<', '$', '`'];
const FILE_TOOLS: [&str; 5] = ["Read", "Edit", "Write", "MultiEdit", "NotebookEdit"];

fn assignment(word: &str) -> bool {
    let Some((name, _)) = word.split_once('=') else { return false };
    !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn quiet(word: &str) -> bool {
    !word.starts_with('-') && !word.contains(LOUD)
}

fn words(part: &str) -> Vec<&str> {
    let real = part.split_whitespace().skip_while(|w| *w == "rtk" || assignment(w));
    real.take_while(|w| quiet(w)).take(LAST_WORDS).collect()
}

fn command_words(command: &str) -> String {
    let line = command.lines().find(|l| !l.trim().is_empty()).unwrap_or_default();
    let mut parts = line.split("&&").flat_map(|part| part.split(';')).map(words);
    parts.find(|w| w.first().is_some_and(|first| !SETUP_WORDS.contains(first))).unwrap_or_default().join(" ")
}

fn file_name(path: &str) -> String {
    path.rsplit(['/', '\\']).next().unwrap_or(path).to_string()
}

fn host(url: &str) -> String {
    let rest = url.split_once("://").map_or(url, |(_, rest)| rest);
    let name = rest.split(['/', '?', '#']).next().unwrap_or_default();
    name.rsplit('@').next().unwrap_or(name).to_string()
}

pub fn activity(tool: &str, input: &Value) -> String {
    let detail = match tool {
        "Bash" => word(input, &["command"]).map(command_words),
        "Grep" | "Glob" => word(input, &["pattern"]).map(str::to_string),
        "WebSearch" => word(input, &["query"]).map(str::to_string),
        "WebFetch" => word(input, &["url"]).map(host),
        _ if FILE_TOOLS.contains(&tool) => word(input, &["file_path", "notebook_path"]).map(file_name),
        _ => None,
    };
    let label = match detail {
        Some(detail) if !detail.is_empty() => format!("{tool} {detail}"),
        _ => tool.to_string(),
    };
    cut(&label, TEXT_MAX)
}
