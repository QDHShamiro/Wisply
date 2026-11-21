mod ask;
mod bench;
mod cache;
mod card;
mod chat;
mod clock;
mod config;
mod door;
mod dry;
mod ear;
mod game;
mod gate;
mod gauge;
mod goals;
mod grab;
mod grammar;
mod history;
mod http;
mod inputs;
mod job;
mod load;
mod long;
mod loops;
mod media;
mod merge;
mod page;
mod pick;
mod post;
mod prep;
mod quality;
mod repair;
mod route;
mod rules;
mod schema;
mod score;
mod server;
mod session;
mod site;
mod spawn;
mod stream;
mod talk;
mod term;
mod think;
mod timecode;
mod trace;
mod video;
mod warm;
mod watch;
mod wire;
mod write;

use std::path::Path;

use serde_json::json;

use crate::config::Config;

fn status(config: &Config) -> i32 {
    let files = server::Files::new(config);
    let state = server::read_state(&files);
    let alive = state.as_ref().is_some_and(server::keeper_alive);
    let used = std::fs::read_to_string(files.used()).ok().and_then(|t| t.trim().parse::<f64>().ok());
    let report = json!({
        "ok": true,
        "loaded": alive,
        "ready": alive && server::healthy(config),
        "model": state.as_ref().map(|s| s.model.clone()),
        "idle_seconds": used.map(|u| (server::now() - u).max(0.0).round()),
        "busy": server::live_busy(&files),
        "keep_loaded_seconds": config.keeper.keep_loaded_seconds,
        "log": files.log(),
    });
    println!("{report}");
    0
}

pub fn run(path: &Path, args: &[String]) -> i32 {
    spawn::seal_std_handles();
    let config = match Config::load(path) {
        Ok(config) => config,
        Err(error) => {
            println!("{}", json!({"ok": false, "error": "config", "message": error, "hint": path}));
            return 2;
        }
    };
    let flag = |name: &str| args.iter().any(|a| a == name);
    match args.first().map(String::as_str) {
        Some("watch") => watch::run(&config, path),
        Some("prep") => prep::run(&config),
        Some("ask") => ask::run(&config, path),
        Some("chat") => talk::run(&config, path, &args[1..]),
        Some("write") => write::run(&config, path, &args[1..]),
        Some("think") => think::run(&config, path, &args[1..]),
        Some("route") => route::run(&config, &args[1..]),
        Some("bench") => bench::run(&config, path, &args[1..]),
        Some("card") => card::run(&config, path, &args[1..]),
        Some("keeper") => server::keeper(&config, args.get(1).map(String::as_str).unwrap_or(&config.models.default)),
        Some("status") => status(&config),
        Some("stop") => {
            println!("{}", json!({"ok": true, "stopped": server::stop(&config)}));
            0
        }
        Some("game") => {
            println!(
                "{}",
                json!({
                    "ok": true,
                    "game": game::running(&config.game, flag("--afk")),
                    "idle_seconds": game::system::idle_seconds().round(),
                    "foreground": game::system::foreground().1,
                })
            );
            0
        }
        _ => {
            eprintln!(
                "wisply pupil watch | prep | ask | chat [--page] [--afk] [--json] [--now] [--think] [--model m] [message, files, links] | write --label x [--runs n] [--model m] [--afk] | think --label x [--think] [--budget n] [--runs n] [--limit n] [--model m] [--afk] | route --set file | bench [cases.json] [--model 2b,4b] [--only a,b] | card [card.json] | status | stop | game [--afk] | hear <video> [--start S] [--end E] [--speech MODEL] [--sound MODEL] [--only speech|sound] | info <YouTube link> | captions <YouTube link> [--start S] [--end E] [--find \"words\"] | limits"
            );
            2
        }
    }
}
