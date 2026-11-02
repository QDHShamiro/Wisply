mod api;
mod auth;
mod board;
mod cmd;
mod events;
mod extra;
mod limit;
mod model;
mod net;
mod price;
mod qr;
mod secret;
mod server;
mod transcript;
mod watch;
mod web;

use std::path::PathBuf;

use wisply_core::{Call, Fail};

use super::ctx::Ctx;
use super::{Daemon, Out};

pub fn start(home: PathBuf) {
    server::start(&home);
    watch::start(home);
}

pub fn run(d: &Daemon, ctx: &Ctx, c: &Call) -> Result<Out, Fail> {
    match c.name.as_str() {
        "agents" => cmd::agents(&d.home, c, ctx.secret.as_deref()).map(Out::Text),
        "mobile" => cmd::mobile(&d.home, c).map(Out::Text),
        _ => cmd::notify(&d.home, c).map(Out::Text),
    }
}

pub fn status_line(d: &Daemon) -> Option<String> {
    watch::status_line(&d.home)
}
