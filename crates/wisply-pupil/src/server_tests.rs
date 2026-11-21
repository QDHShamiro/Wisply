use super::{
    Busy, Files, State, args_context, fit_context, live_busy, marker_pid, others_busy, same_server, server_args, shares,
};

#[test]
fn the_chat_never_counts_on_more_context_than_the_running_server_has() {
    let config: crate::config::Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
    let model = config.model(None).expect("default model").clone();
    let args = server_args(&config, &model);
    assert_eq!(args_context(&args), Some(config.server.context));
    assert_eq!(args_context(&["-m".to_string()]), None);
    assert_eq!(args_context(&["-c".to_string(), "x".to_string()]), None);
    assert_eq!(fit_context(131072, Some(32768)), 32768);
    assert_eq!(fit_context(32768, Some(131072)), 32768);
    assert_eq!(fit_context(131072, None), 131072);
}

#[test]
fn nested_busy_marks_stay_until_the_last_one_ends_and_own_marks_are_not_others() {
    let dir = std::env::temp_dir().join(format!("pupil-busy-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
    config.paths.run_dir = dir.clone();
    let files = Files::new(&config);
    assert_eq!(live_busy(&files), 0);
    let turn = Busy::mark(&config);
    let part = Busy::mark(&config);
    assert_eq!(live_busy(&files), 2);
    assert_eq!(others_busy(&files), 0);
    drop(part);
    assert_eq!(live_busy(&files), 1);
    drop(turn);
    assert_eq!(live_busy(&files), 0);
    assert_eq!(marker_pid("123-4"), 123);
    assert_eq!(marker_pid("123"), 123);
    assert_eq!(marker_pid("x-1"), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_shared_server_is_used_only_when_it_runs_our_model_and_answers() {
    assert!(shares(true, Some("2b"), "2b", || true));
    assert!(!shares(false, Some("2b"), "2b", || true));
    assert!(!shares(true, Some("4b"), "2b", || true));
    assert!(!shares(true, None, "2b", || true));
    assert!(!shares(true, Some("2b"), "2b", || false));
    assert!(!shares(false, Some("2b"), "2b", || panic!("no health call when off")));
}
use crate::config::Config;

#[test]
fn a_server_with_other_start_options_is_not_reused() {
    let mut config: Config = toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses");
    let model = config.model(None).expect("model").clone();
    let state = State {
        keeper: 1,
        server: 2,
        model: model.name.clone(),
        port: config.server.port,
        started: 0.0,
        args: server_args(&config, &model),
    };
    assert!(same_server(&state, &config, &model));
    config.server.extra_args = vec!["--spec-type".to_string(), "ngram-mod".to_string()];
    assert!(!same_server(&state, &config, &model));
    let old = State { args: Vec::new(), ..state };
    assert!(!same_server(&old, &config, &model));
}
