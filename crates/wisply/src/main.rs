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
