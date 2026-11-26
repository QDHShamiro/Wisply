mod cmd;
mod fallback;
mod pick;
mod providers;
mod reply;
mod serve;
mod server;
mod state;
mod upstream;
mod wire;

#[cfg(test)]
mod flow_tests;

use wisply_core::{Call, Fail};

use super::{Daemon, Out};

pub fn start(d: &Daemon) {
    cmd::start(&d.home);
}

pub fn run(d: &Daemon, c: &Call) -> Result<Out, Fail> {
    cmd::run(&d.home, c).map(Out::Text)
}

pub fn status_line() -> Option<String> {
    state::enabled().then(|| {
        if state::limited() { "fallback: active (Claude limit)".to_string() } else { "fallback: on".to_string() }
    })
}
