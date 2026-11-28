mod call;
mod client;
mod codemode;
mod daemon;
mod headless;
mod hook;
mod host;
mod junk;
mod kit;
#[cfg(windows)]
mod launch;
mod lint;
mod local;
mod mcp;
mod more;
mod peek;
mod pipe;
mod prewarn;
mod progress;
mod pupil;
mod reads;
mod readstate;
mod readview;
mod review;
mod risk;
mod setup;
mod skill;
mod spawn;
mod statusline;
#[cfg(windows)]
mod testapp;
mod update;
mod v2;
#[cfg(windows)]
mod voice;
mod waitfor;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    wisply_proto::pin_home();
    let args: Vec<String> = env::args_os().skip(1).map(|a| a.to_string_lossy().into_owned()).collect();
    let first = args.first().map(String::as_str).unwrap_or("");
    if let Some(text) = own_help(&args) {
        println!("{text}");
        return ExitCode::SUCCESS;
    }
    match first {
        "mcp" => mcp::serve(),
        "host" => host::serve(args.get(1).map(String::as_str).unwrap_or("")),
        "hook" if args.get(1).is_some_and(|a| a == "post") => hook::post(),
        origin if origin.starts_with("chrome-extension://") => host::serve(origin),
        "statusline" => return statusline::run(&args[1..]),
        "version" | "--version" => println!("{}", env!("CARGO_PKG_VERSION")),
        "daemon" if daemon::may_serve() => return daemon::serve(),
        #[cfg(windows)]
        "voice-session" | "voice-watch" => return voice::main(&args),
        #[cfg(windows)]
        "testapp" => testapp::main(&args),
        #[cfg(windows)]
        "claude-start" => return launch::main(&args),
        _ => return client::run(&args),
    }
    ExitCode::SUCCESS
}

const RUN_HERE: &[&str] = &["update", "headless", "ui", "squeeze", "version"];

fn own_help(args: &[String]) -> Option<String> {
    let first = args.first()?.as_str();
    let here = wisply_core::CLIENT.contains(&first)
        || RUN_HERE.contains(&first)
        || (first == "wait" && args.get(1).is_some_and(|a| a == "ci" || a == "until"))
        || (first == "memory" && args.get(1).is_some_and(|a| a == "bench"));
    if !here || wisply_core::wants_help(args).as_deref() != Some(first) {
        return None;
    }
    Some(daemon::help_for(Some(first)).unwrap_or_else(|fail| fail.text()))
}

#[cfg(test)]
mod help_tests {
    use super::own_help;

    fn argv(line: &str) -> Vec<String> {
        line.split(' ').map(String::from).collect()
    }

    #[test]
    fn help_flags_on_local_commands_print_help_and_never_run_them() {
        for line in
            ["uninstall --help", "setup --help", "setup -h", "doctor --fix --help", "update --help", "wait ci --help"]
        {
            assert!(own_help(&argv(line)).is_some_and(|text| !text.starts_with("err:")), "{line}");
        }
        assert!(own_help(&argv("uninstall --help")).is_some_and(|text| text.contains("uninstall [--purge]")));
        assert!(own_help(&argv("daemon --help")).is_some_and(|text| text.contains("nothing to run by hand")));
        for line in
            ["setup", "uninstall --purge", "kit add x y -- tool --help", "snap --help", "help setup", "wait 200 --help"]
        {
            assert_eq!(own_help(&argv(line)), None, "{line}");
        }
    }
}
