const SAME: &[(&str, &str)] = &[
    ("screenshot", "shot"),
    ("take_screenshot", "shot"),
    ("snapshot", "snap"),
    ("take_snapshot", "snap"),
    ("read_page", "snap"),
    ("navigate", "open"),
    ("navigate_page", "open"),
    ("goto", "open"),
    ("go_to", "open"),
    ("visit", "open"),
    ("open_url", "open"),
    ("eval", "js"),
    ("evaluate", "js"),
    ("evaluate_script", "js"),
    ("javascript", "js"),
    ("javascript_tool", "js"),
    ("run_js", "js"),
    ("execute_js", "js"),
    ("get_text", "text"),
    ("gettext", "text"),
    ("page_text", "text"),
    ("get_page_text", "text"),
    ("press_key", "press"),
    ("key", "press"),
    ("keypress", "press"),
    ("hotkey", "press"),
    ("wait_for", "wait"),
    ("waitfor", "wait"),
    ("select_option", "select"),
    ("file_upload", "upload"),
    ("upload_file", "upload"),
    ("handle_dialog", "dialog"),
    ("navigate_back", "back"),
    ("go_back", "back"),
    ("navigate_forward", "forward"),
    ("go_forward", "forward"),
    ("refresh", "reload"),
    ("list_pages", "tabs"),
    ("list_tabs", "tabs"),
    ("tabs_context", "tabs"),
    ("select_page", "tab"),
    ("switch_tab", "tab"),
    ("close_page", "close"),
    ("close_tab", "close"),
    ("fill_form", "fill"),
    ("form_input", "fill"),
    ("console_messages", "console"),
    ("read_console_messages", "console"),
    ("list_console_messages", "console"),
    ("network_requests", "net"),
    ("read_network_requests", "net"),
    ("list_network_requests", "net"),
    ("resize", "viewport"),
    ("resize_page", "viewport"),
    ("resize_window", "viewport"),
    ("type_text", "type"),
    ("hover_element", "hover"),
    ("drag_and_drop", "drag"),
    ("scroll_to", "scroll"),
    ("memorize", "remember"),
    ("search_memory", "recall"),
    ("todos", "todo"),
];

const GUESS: &[(&str, &str)] = &[
    ("find", "snap -f \"word\" (page), code <name> (code) or recall \"words\" (memory)"),
    ("url", "tabs (every tab with its URL)"),
    ("sleep", "wait <ms|\"text\"> (milliseconds: wait 2000)"),
    ("apps", "windows (open apps) or launch <app>"),
    ("chat", "unread (waiting chats), inbox <w5|\"@Name\"> or reply <w5|\"@Name\"> \"text\""),
    ("exec", "codemode --file x.js (shell, files and wisply steps in one call) or js <code> (JavaScript in the page)"),
    (
        "execute",
        "codemode --file x.js (shell, files and wisply steps in one call) or js <code> (JavaScript in the page)",
    ),
    ("shell", "codemode --file x.js (shell, files and wisply steps in one call)"),
    ("bash", "codemode --file x.js (shell, files and wisply steps in one call)"),
    ("run_command", "codemode --file x.js (shell, files and wisply steps in one call)"),
    ("note", "remember \"fact\" (keep it) or recall \"words\" (find it)"),
    ("notes", "remember \"fact\" (keep it) or recall \"words\" (find it)"),
    ("search_code", "code <Name> or code \"what it does\" (run it in the project folder)"),
    ("code_search", "code <Name> or code \"what it does\" (run it in the project folder)"),
    ("symbols", "code <Name> or code \"what it does\" (run it in the project folder)"),
    ("hear", "listen <link|file> (speech to text, --local stays on this PC) or pupil hear <file>"),
    ("transcribe", "listen <link|file> (speech to text, --local stays on this PC) or pupil hear <file>"),
    ("speech_to_text", "listen <link|file> (speech to text, --local stays on this PC) or pupil hear <file>"),
    ("ocr", "read app:Name (the text of an app window) or read <url|file.pdf>"),
    ("new_tab", "open <url> -n (a new tab)"),
    ("new_page", "open <url> -n (a new tab)"),
    ("open_tab", "open <url> -n (a new tab)"),
    ("computer", "snap, click, type, press or shot (one wisply command per action)"),
    ("fix", "doctor --fix"),
    ("repair", "doctor --fix"),
    ("init", "setup"),
    ("start", "status (any command starts the daemon by itself)"),
    ("restart", "stop (the next command starts the daemon again)"),
];

pub fn normal(name: &str) -> String {
    let low = name.trim().to_lowercase();
    let tail = low.rsplit("__").next().unwrap_or(&low).replace('-', "_");
    let tail = tail.strip_prefix("browser_").unwrap_or(&tail);
    tail.strip_prefix("wisply_").unwrap_or(tail).to_string()
}

pub fn same(name: &str, known: impl Fn(&str) -> bool) -> Option<&'static str> {
    if known(name) {
        return None;
    }
    let low = normal(name);
    if let Some((_, real)) = SAME.iter().find(|(word, _)| *word == low) {
        return Some(real);
    }
    crate::names::known_name(&low).or_else(|| crate::names::known_name(&low.replace('_', "-")))
}

pub fn guess(name: &str) -> Option<&'static str> {
    let low = normal(name);
    GUESS.iter().find(|(word, _)| *word == low).map(|(_, hint)| *hint)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn never(_: &str) -> bool {
        false
    }

    #[test]
    fn names_of_other_browser_tools_run_the_wisply_command() {
        let cases = [
            ("screenshot", "shot"),
            ("browser_take_screenshot", "shot"),
            ("mcp__playwright__browser_navigate", "open"),
            ("Navigate", "open"),
            ("take-snapshot", "snap"),
            ("evaluate_script", "js"),
            ("get-text", "text"),
            ("browser_press_key", "press"),
            ("browser_click", "click"),
            ("browser_type", "type"),
            ("browser_tabs", "tabs"),
            ("wisply_snap", "snap"),
            ("javascript_tool", "js"),
            ("todos", "todo"),
        ];
        for (word, real) in cases {
            assert_eq!(same(word, never), Some(real), "{word}");
        }
    }

    #[test]
    fn real_commands_and_unclear_words_are_never_renamed() {
        assert_eq!(same("snap", |n| n == "snap"), None);
        for word in ["sleep", "exec", "find", "zzz", "hear", ""] {
            assert_eq!(same(word, never), None, "{word}");
        }
    }

    #[test]
    fn unclear_words_get_a_hint_not_a_run() {
        assert!(guess("exec").is_some_and(|h| h.starts_with("codemode --file")));
        assert!(guess("browser-new-tab").is_some_and(|h| h.starts_with("open <url> -n")));
        assert!(guess("Transcribe").is_some_and(|h| h.contains("pupil hear")));
        assert!(guess("sleep").is_some_and(|h| h.contains("wait 2000")));
        assert_eq!(guess("snap"), None);
    }
}
