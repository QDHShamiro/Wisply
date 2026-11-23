use std::fs;
use std::path::Path;

const MARKER: &str = "v3";
const UNKNOWN: &str = "err: unknown ";
const TOP: &str = "err: unknown command '";

fn typo(out: &str, known: fn(&str) -> bool) -> bool {
    out.strip_prefix(TOP).and_then(|rest| rest.split_once('\'')).is_some_and(|(name, _)| !known(name))
}

fn parts(version: &str) -> Option<Vec<u64>> {
    let version = version.trim();
    if version.is_empty() {
        return None;
    }
    version.split('.').map(|p| p.parse().ok()).collect()
}

fn older(running: &str, mine: &str) -> bool {
    matches!((parts(running), parts(mine)), (Some(a), Some(b)) if a < b)
}

pub fn older_daemon(home: &Path, code: i32, out: &str, mine: &str, known: fn(&str) -> bool) -> Option<String> {
    if code == 0 || !out.starts_with(UNKNOWN) || !out.contains("command") || typo(out, known) {
        return None;
    }
    let running = fs::read_to_string(home.join(MARKER)).ok()?;
    let running = running.trim();
    if !older(running, mine) {
        return None;
    }
    let what = out.split(" -> try:").next().unwrap_or(out).trim_end();
    Some(format!(
        "{what} (the running daemon is the older wisply {running}, this wisply is {mine}) -> try: wisply stop, then run it again (that starts the {mine} daemon, notes and skills stay)"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn home(version: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("wisply-skew-{}-{version}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(MARKER), version).unwrap();
        dir
    }

    #[test]
    fn versions_compare_by_number_not_by_text() {
        assert!(older("3.0.1", "3.6.0"));
        assert!(older("3.9.0", "3.10.0"));
        assert!(!older("3.6.0", "3.6.0"));
        assert!(!older("3.6.1", "3.6.0"));
        assert!(!older("", "3.6.0"));
        assert!(!older("3.6.0-rc1", "3.7.0"));
    }

    fn known(name: &str) -> bool {
        matches!(name, "codemode" | "learn" | "snap")
    }

    #[test]
    fn an_older_daemon_turns_unknown_command_into_a_restart_hint() {
        let old = home("3.0.1");
        let said =
            older_daemon(&old, 1, "err: unknown command 'codemode' -> try: wisply help", "3.6.0", known).unwrap();
        assert_eq!(
            said,
            "err: unknown command 'codemode' (the running daemon is the older wisply 3.0.1, this wisply is 3.6.0) -> try: wisply stop, then run it again (that starts the 3.6.0 daemon, notes and skills stay)"
        );
        let sub =
            older_daemon(&old, 1, "err: unknown routine command 'script' -> try: wisply routine list", "3.6.0", known);
        assert!(
            sub.unwrap()
                .starts_with("err: unknown routine command 'script' (the running daemon is the older wisply 3.0.1")
        );
        assert_eq!(older_daemon(&old, 1, "err: unknown command 'snapp' -> try: wisply snap", "3.6.0", known), None);
        assert_eq!(older_daemon(&old, 0, "err: unknown command 'learn'", "3.6.0", known), None);
        assert_eq!(older_daemon(&old, 1, "err: no tab matches \"x\"", "3.6.0", known), None);
        let same = home("3.6.0");
        assert_eq!(older_daemon(&same, 1, "err: unknown command 'learn' -> try: wisply help", "3.6.0", known), None);
        let blank = home("");
        assert_eq!(older_daemon(&blank, 1, "err: unknown command 'learn' -> try: wisply help", "3.6.0", known), None);
        for dir in [old, same, blank] {
            let _ = fs::remove_dir_all(dir);
        }
    }
}
