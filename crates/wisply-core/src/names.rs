use crate::{Fail, alias, close_match};

const SYSTEM: &[&str] = &[
    "do",
    "go",
    "contact",
    "contacts",
    "channels",
    "ping",
    "stop",
    "status",
    "config",
    "log",
    "version",
    "help",
    "selftest",
    "bench",
    "demo",
    "board",
    "ui",
    "autoload",
    "ext-reload",
    "run",
    "routine",
    "routines",
    "learn",
    "record",
    "done",
    "keep",
    "keys",
    "suggest",
    "brain",
    "improve",
    "access-prompt",
    "tokens",
    "grep",
    "squeeze",
    "secure",
    "think",
    "update",
    "pc",
    "headless",
    "agents",
    "mobile",
    "notify",
    "local-url",
    "insight",
];
const MEMORY: &[&str] = &["remember", "recall", "forget", "todo", "bug", "list", "memory", "hook", "wiki", "code"];
const MEMORY_LANE: &[&str] = &["suggest", "tokens", "grep", "squeeze", "board", "pc", "agents", "mobile", "notify"];
const HOOK_LANE: &[&str] = &["suggest", "hook"];
pub const SHARED: &[&str] = &["snap", "click", "type", "press", "shot", "scroll", "hover", "drag"];
pub const APP_ONLY: &[&str] = &["windows", "focus", "clip", "launch"];
pub const CHAT: &[&str] = &["inbox", "reply", "unread"];
pub const WEB_ONLY: &[&str] = &[
    "open",
    "text",
    "fill",
    "select",
    "check",
    "wait",
    "expect",
    "table",
    "fetch",
    "js",
    "dismiss",
    "search",
    "net",
    "tools",
    "call",
    "upload",
    "tabs",
    "tab",
    "close",
    "back",
    "forward",
    "reload",
    "dialog",
    "downloads",
    "console",
    "viewport",
    "frames",
];
const MORE: &[&str] = &["read", "research", "listen", "watch", "voice", "phone", "claude", "desktop"];
pub const CLIENT: &[&str] = &[
    "more",
    "peek",
    "kit",
    "skill",
    "progress",
    "codemode",
    "call",
    "local",
    "setup",
    "install",
    "uninstall",
    "doctor",
    "pupil",
    "lint",
    "safe",
    "review",
    "analyze",
    "statusline",
    "mcp",
    "host",
    "daemon",
    "voice-session",
    "voice-watch",
    "testapp",
    "claude-start",
];
pub const INTERNAL: &[&str] = &["host", "daemon", "voice-session", "voice-watch", "testapp", "claude-start"];
const HIDDEN: &[&str] = &["hook", "suggest", "board", "access-prompt", "autoload", "ping", "local-url"];

pub const QUIET: &[&str] = &[
    "done",
    "suggest",
    "hook",
    "ping",
    "status",
    "log",
    "version",
    "help",
    "config",
    "brain",
    "improve",
    "access-prompt",
    "tokens",
    "board",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lane {
    Main,
    Memory,
    Hook,
}

pub fn lane(name: &str) -> Lane {
    let name = same(name).unwrap_or(name).to_lowercase();
    if HOOK_LANE.contains(&name.as_str()) {
        Lane::Hook
    } else if MEMORY_LANE.contains(&name.as_str()) || MEMORY.contains(&name.as_str()) {
        Lane::Memory
    } else {
        Lane::Main
    }
}

fn all() -> impl Iterator<Item = &'static str> {
    [SYSTEM, MEMORY, SHARED, APP_ONLY, CHAT, WEB_ONLY, MORE, CLIENT].into_iter().flatten().copied()
}

fn known() -> Vec<&'static str> {
    let mut names: Vec<&str> = all().filter(|n| !HIDDEN.contains(n) && !INTERNAL.contains(n)).collect();
    names.sort_unstable();
    names.dedup();
    names
}

pub(crate) fn known_name(name: &str) -> Option<&'static str> {
    known().into_iter().find(|n| *n == name)
}

pub fn built_later(name: &str) -> bool {
    all().any(|n| n == name)
}

pub fn same(name: &str) -> Option<&'static str> {
    alias::same(name, built_later)
}

pub fn unknown(name: &str) -> Fail {
    let hint = alias::guess(name).or_else(|| close_match(name, &known())).unwrap_or("help");
    Fail::hint(format!("unknown command '{name}'"), format!("wisply {hint}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lanes_match_the_2x_daemon() {
        assert_eq!(lane("suggest"), Lane::Hook);
        assert_eq!(lane("HOOK"), Lane::Hook);
        assert_eq!(lane("recall"), Lane::Memory);
        assert_eq!(lane("squeeze"), Lane::Memory);
        assert_eq!(lane("snap"), Lane::Main);
        assert_eq!(lane(""), Lane::Main);
        assert_eq!(lane("memorize"), Lane::Memory);
        assert_eq!(lane("screenshot"), Lane::Main);
    }

    #[test]
    fn help_and_close_matches_know_the_commands_the_client_runs() {
        for name in ["setup", "doctor", "pupil", "insight", "mcp", "statusline", "review", "lint"] {
            assert!(built_later(name) && known_name(name).is_some(), "{name}");
        }
        assert!(INTERNAL.iter().all(|name| built_later(name) && known_name(name).is_none()));
        assert_eq!(same("screenshot"), Some("shot"));
        assert_eq!(same("shot"), None);
        assert_eq!(unknown("doctr").text(), "err: unknown command 'doctr' -> try: wisply doctor");
    }

    #[test]
    fn unknown_commands_get_the_2x_hints() {
        let cases = [
            ("snapp", "snap"),
            (
                "exec",
                "codemode --file x.js (shell, files and wisply steps in one call) or js <code> (JavaScript in the page)",
            ),
            ("zzzz", "help"),
            ("pinng", "help"),
            ("stauts", "status"),
            ("recal", "recall"),
            ("clik", "click"),
            ("verison", "version"),
            ("lgo", "go"),
            ("snaps", "snap"),
            ("tab", "tab"),
            ("t", "help"),
        ];
        for (word, hint) in cases {
            assert_eq!(unknown(word).text(), format!("err: unknown command '{word}' -> try: wisply {hint}"));
        }
        assert!(built_later("snap") && built_later("recall") && built_later("ping") && !built_later("snapp"));
        assert!(CLIENT.iter().all(|name| built_later(name)));
    }

    #[test]
    fn words_agents_typed_for_keys_apps_and_chats_name_the_real_command() {
        let cases = [
            ("sleep", "wait <ms|\"text\"> (milliseconds: wait 2000)"),
            ("apps", "windows (open apps) or launch <app>"),
            ("chat", "unread (waiting chats), inbox <w5|\"@Name\"> or reply <w5|\"@Name\"> \"text\""),
        ];
        for (word, hint) in cases {
            assert_eq!(unknown(word).text(), format!("err: unknown command '{word}' -> try: wisply {hint}"));
        }
    }

    #[test]
    fn the_known_names_are_the_2x_list_secure_and_the_window() {
        let want = "agents analyze back bench brain bug call channels check claude click clip close code codemode config console contact \
                    contacts demo desktop dialog dismiss do doctor done downloads drag expect ext-reload fetch fill focus forget forward \
                    frames go grep headless help hover improve inbox insight install js keep keys kit launch learn lint list listen local \
                    log mcp memory mobile more net notify open pc peek phone press progress pupil read recall record reload remember \
                    reply research review routine routines run safe scroll search secure select selftest setup shot skill snap squeeze \
                    status statusline stop tab table tabs text think todo tokens tools type ui uninstall unread update upload version \
                    viewport voice wait watch wiki windows";
        assert_eq!(known(), want.split(' ').collect::<Vec<_>>());
    }
}
