use std::collections::BTreeMap;
use std::path::Path;

use serde_json::json;

pub const INSIDE: &str =
    "a saved script routine cannot run inside code mode -> try: put its steps into this script, or run it on its own";
const SECRET_KEYS: [&str; 6] = ["token", "secret", "pass", "key", "auth", "cookie"];
const SECRET_CODE: [&str; 2] = ["--env", "@secret"];
const MIN_CALLS: usize = 2;

pub struct Report<'a> {
    pub code: &'a str,
    pub args: &'a BTreeMap<String, String>,
    pub routine: Option<&'a str>,
    pub calls: usize,
    pub ms: u64,
    pub ok: bool,
    pub secret: bool,
    pub error: Option<&'a str>,
    pub checked: bool,
    pub rounds: usize,
    pub dry: bool,
}

pub fn code_of(home: &Path, name: &str) -> Result<String, String> {
    crate::daemon::script_code(home, name)
        .ok_or_else(|| format!("no saved script routine \"{name}\" -> try: wisply routines"))
}

fn secret_args(args: &BTreeMap<String, String>) -> bool {
    args.keys().any(|k| {
        let k = k.to_lowercase();
        SECRET_KEYS.iter().any(|w| k.contains(w))
    })
}

pub fn worth_saving(r: &Report<'_>) -> bool {
    r.routine.is_none()
        && r.ok
        && r.calls >= MIN_CALLS
        && !r.secret
        && !secret_args(r.args)
        && !SECRET_CODE.iter().any(|w| r.code.contains(w))
}

pub fn tell(home: &Path, cwd: &Path, sid: &str, r: &Report<'_>) -> Option<String> {
    let body = match r.routine {
        Some(name) => json!({
            "routine": name, "ok": r.ok, "ms": r.ms, "error": r.error, "checked": r.checked, "rounds": r.rounds,
            "dry": r.dry
        }),
        None if worth_saving(r) && crate::daemon::config_on(&crate::daemon::load_config(home), "codemode_save") => {
            json!({
                "code": r.code,
                "args": r.args,
                "calls": r.calls,
                "ms": r.ms,
                "project": crate::daemon::script_project(cwd).display().to_string(),
            })
        }
        None => return None,
    };
    let argv = ["routine".to_string(), "script".to_string(), body.to_string()];
    let (code, out) = crate::client::ask_as(home, &argv, sid, None, cwd).ok()?;
    (code == 0 && !out.trim().is_empty()).then(|| out.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_clean_multi_step_scripts_are_worth_saving() {
        let mut args = BTreeMap::new();
        args.insert("pr".to_string(), "101".to_string());
        let base = Report {
            code: "await $`a`; await $`b`",
            args: &args,
            routine: None,
            calls: 2,
            ms: 9,
            ok: true,
            secret: false,
            error: None,
            checked: false,
            rounds: 0,
            dry: false,
        };
        assert!(worth_saving(&base));
        assert!(!worth_saving(&Report { calls: 1, ..base }));
        assert!(!worth_saving(&Report { ok: false, ..base }));
        assert!(!worth_saving(&Report { secret: true, ..base }));
        assert!(!worth_saving(&Report { routine: Some("x"), ..base }));
        assert!(!worth_saving(&Report { code: "await wisply`fill Pass @secret --env PW`", ..base }));
        let mut keyed = BTreeMap::new();
        keyed.insert("api_token".to_string(), "abc".to_string());
        assert!(!worth_saving(&Report { args: &keyed, ..base }));
    }
}
