use std::io::Read;
use std::path::Path;
use std::time::Instant;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::chat::{self, Answer};
use crate::config::Config;
use crate::dry;
use crate::game;
use crate::job::Failure;
use crate::server::{self, Busy};
use crate::watch::{request_failure, start_failure};

#[derive(Debug, Deserialize)]
pub struct Question {
    pub prompt: String,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub afk: bool,
}

#[derive(Debug, Serialize)]
pub struct Said {
    pub ok: bool,
    pub text: String,
    pub finish: String,
    pub model: String,
    pub prompt_tokens: u64,
    pub answer_tokens: u64,
    pub prompt_per_second: f64,
    pub answer_per_second: f64,
    pub server_seconds: f64,
    pub model_seconds: f64,
    pub total_seconds: f64,
    pub wrapped: bool,
}

fn fail(config: &Config, code: &str, fields: &[(&str, &str)]) -> Failure {
    Failure::new(config, code, fields)
}

pub fn parse(config: &Config, raw: &str) -> Result<Question, Failure> {
    let question: Question =
        serde_json::from_str(raw).map_err(|e| fail(config, "bad_job", &[("detail", &e.to_string())]))?;
    if question.prompt.trim().is_empty() {
        return Err(fail(config, "bad_job", &[("detail", "prompt is empty")]));
    }
    Ok(question)
}

fn messages(config: &Config, question: &Question) -> Vec<Value> {
    let mut messages = Vec::new();
    if !config.write.system.trim().is_empty() {
        messages.push(json!({"role": "system", "content": config.write.system}));
    }
    messages.push(json!({"role": "user", "content": question.prompt}));
    messages
}

fn limit(config: &Config, question: &Question) -> u32 {
    question.max_tokens.filter(|n| *n > 0).unwrap_or(if config.write.max_tokens > 0 {
        config.write.max_tokens
    } else {
        config.sampling.max_tokens
    })
}

fn reserve(config: &Config, limit: u32) -> u32 {
    let write = &config.write;
    let on = write.wrap_tokens > 0 && !write.wrap_prompt.trim().is_empty();
    if on && limit >= write.wrap_from_tokens && limit > write.wrap_tokens { write.wrap_tokens } else { 0 }
}

fn send(config: &Config, model: &str, messages: Vec<Value>, limit: u32) -> Value {
    let mut sent = chat::base(config, model, Value::Array(messages), limit);
    if let Some(multiplier) = config.write.dry_multiplier {
        dry::set(&mut sent, &config.sampling, multiplier);
    }
    sent
}

pub fn body(config: &Config, model: &str, question: &Question) -> Value {
    let limit = limit(config, question);
    send(config, model, messages(config, question), limit - reserve(config, limit))
}

pub fn wrap_body(config: &Config, model: &str, question: &Question, cut: &str) -> Value {
    let mut messages = messages(config, question);
    messages.push(json!({"role": "assistant", "content": cut}));
    messages.push(json!({"role": "user", "content": config.write.wrap_prompt}));
    send(config, model, messages, reserve(config, limit(config, question)))
}

fn spent(answer: &Answer) -> f64 {
    if answer.answer_per_second > 0.0 { answer.answer_tokens as f64 / answer.answer_per_second } else { 0.0 }
}

fn join(config: &Config, first: Answer, last: Answer) -> Answer {
    let seconds = spent(&first) + spent(&last);
    let tokens = first.answer_tokens + last.answer_tokens;
    Answer {
        text: format!("{}{}{}", first.text.trim_end(), config.write.wrap_join, last.text.trim()),
        finish: last.finish,
        prompt_tokens: first.prompt_tokens,
        answer_tokens: tokens,
        prompt_per_second: first.prompt_per_second,
        answer_per_second: if seconds > 0.0 { tokens as f64 / seconds } else { 0.0 },
        probs: None,
    }
}

fn needs_wrap(config: &Config, question: &Question, first: &Answer) -> bool {
    first.finish != chat::FINISH_DONE && !first.text.trim().is_empty() && reserve(config, limit(config, question)) > 0
}

fn finish(config: &Config, model: &str, question: &Question, first: Answer) -> Result<(Answer, bool), String> {
    if !needs_wrap(config, question, &first) {
        return Ok((first, false));
    }
    let last = chat::ask(config, &wrap_body(config, model, question, &first.text))?;
    Ok((join(config, first, last), true))
}

pub fn ask(config: &Config, config_path: &Path, question: &Question) -> Result<Said, Failure> {
    let total = Instant::now();
    let Some(model) = config.model(question.model.as_deref()).cloned() else {
        let name = question.model.clone().unwrap_or_default();
        return Err(fail(config, "model_unknown", &[("name", &name), ("names", &config.model_names())]));
    };
    if !model.file.is_file() {
        return Err(fail(config, "model_missing", &[("path", &model.file.display().to_string())]));
    }
    if let Some(name) = game::running(&config.game, question.afk) {
        return Err(fail(config, "game_running", &[("game", &name)]));
    }
    server::set_afk(config, question.afk);
    let begun = Instant::now();
    server::ensure(config, config_path, &model).map_err(|e| start_failure(config, question.afk, e))?;
    let server_seconds = begun.elapsed().as_secs_f64();
    let sent = body(config, &model.name, question);
    let begun = Instant::now();
    let answer = {
        let _busy = Busy::mark(config);
        chat::ask(config, &sent).and_then(|first| finish(config, &model.name, question, first))
    };
    let model_seconds = begun.elapsed().as_secs_f64();
    let (answer, wrapped) = answer.map_err(|detail| request_failure(config, question.afk, detail))?;
    if answer.text.trim().is_empty() {
        return Err(fail(config, "empty_answer", &[]));
    }
    Ok(Said {
        ok: true,
        text: answer.text.trim().to_string(),
        finish: answer.finish,
        model: model.name,
        prompt_tokens: answer.prompt_tokens,
        answer_tokens: answer.answer_tokens,
        prompt_per_second: answer.prompt_per_second,
        answer_per_second: answer.answer_per_second,
        server_seconds,
        model_seconds,
        total_seconds: total.elapsed().as_secs_f64(),
        wrapped,
    })
}

pub fn run(config: &Config, config_path: &Path) -> i32 {
    let mut raw = String::new();
    let result = match std::io::stdin().read_to_string(&mut raw) {
        Ok(_) => parse(config, &raw).and_then(|q| ask(config, config_path, &q)),
        Err(e) => Err(fail(config, "bad_job", &[("detail", &e.to_string())])),
    };
    match result {
        Ok(said) => {
            println!("{}", serde_json::to_string(&said).unwrap_or_default());
            0
        }
        Err(failure) => {
            println!("{}", serde_json::to_string(&failure).unwrap_or_default());
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{body, join, needs_wrap, parse, wrap_body};
    use crate::chat::{Answer, FINISH_DONE};
    use crate::config::Config;

    fn config() -> Config {
        toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses")
    }

    fn unwrapped() -> Config {
        let mut config = config();
        config.write.wrap_tokens = 0;
        config
    }

    #[test]
    fn the_shipped_config_keeps_room_for_a_conclusion() {
        let config = config();
        let write = &config.write;
        assert!(write.wrap_tokens > 0);
        assert!(!write.wrap_prompt.trim().is_empty());
        assert!(write.max_tokens >= write.wrap_from_tokens);
        let question = parse(&config, r#"{"prompt": "Explain."}"#).expect("parses");
        assert_eq!(body(&config, "2b", &question)["max_tokens"], write.max_tokens - write.wrap_tokens);
        assert_eq!(wrap_body(&config, "2b", &question, "Step 1")["max_tokens"], write.wrap_tokens);
    }

    #[test]
    fn a_question_is_plain_text_without_json_rules() {
        let config = unwrapped();
        let question = parse(&config, r#"{"prompt": "Write a haiku."}"#).expect("parses");
        let sent = body(&config, "2b", &question);
        assert!(sent.get("response_format").is_none());
        assert!(sent.get("grammar").is_none());
        let last = sent["messages"].as_array().and_then(|m| m.last()).expect("message");
        assert_eq!(last["role"], "user");
        assert_eq!(last["content"], "Write a haiku.");
        assert_eq!(sent["max_tokens"], config.write.max_tokens);
        assert_eq!(sent["seed"], config.sampling.seed.expect("seed"));
    }

    #[test]
    fn writing_uses_its_own_dry_setting() {
        let mut config = config();
        let question = parse(&config, r#"{"prompt": "Hi"}"#).expect("parses");
        config.write.dry_multiplier = Some(0.0);
        assert!(body(&config, "2b", &question).get("dry_multiplier").is_none());
        config.write.dry_multiplier = None;
        assert_eq!(
            body(&config, "2b", &question).get("dry_multiplier").is_some(),
            config.sampling.dry_multiplier > 0.0
        );
    }

    #[test]
    fn the_job_may_set_its_own_answer_limit() {
        let config = config();
        let question = parse(&config, r#"{"prompt": "Hi", "max_tokens": 64}"#).expect("parses");
        assert_eq!(body(&config, "2b", &question)["max_tokens"], 64);
    }

    fn wrapping() -> Config {
        let mut config = config();
        config.write.max_tokens = 1024;
        config.write.wrap_tokens = 96;
        config.write.wrap_from_tokens = 512;
        config.write.wrap_prompt = "Finish now.".into();
        config.write.wrap_join = " | ".into();
        config
    }

    fn answer(text: &str, finish: &str, tokens: u64, speed: f64) -> Answer {
        Answer {
            text: text.into(),
            finish: finish.into(),
            prompt_tokens: 40,
            answer_tokens: tokens,
            prompt_per_second: 400.0,
            answer_per_second: speed,
            probs: None,
        }
    }

    #[test]
    fn a_long_answer_keeps_room_for_its_conclusion() {
        let config = wrapping();
        let question = parse(&config, r#"{"prompt": "Explain."}"#).expect("parses");
        assert_eq!(body(&config, "2b", &question)["max_tokens"], 928);
        let sent = wrap_body(&config, "2b", &question, "Step 1");
        assert_eq!(sent["max_tokens"], 96);
        let messages = sent["messages"].as_array().expect("messages");
        let count = messages.len();
        assert_eq!(messages[count - 2]["role"], "assistant");
        assert_eq!(messages[count - 2]["content"], "Step 1");
        assert_eq!(messages[count - 1]["role"], "user");
        assert_eq!(messages[count - 1]["content"], "Finish now.");
    }

    #[test]
    fn short_limits_and_a_switched_off_wrap_get_no_wrap_up() {
        let wrap = wrapping();
        let short = parse(&wrap, r#"{"prompt": "Hi", "max_tokens": 64}"#).expect("parses");
        assert_eq!(body(&wrap, "2b", &short)["max_tokens"], 64);
        assert!(!needs_wrap(&wrap, &short, &answer("Hi", "length", 64, 50.0)));
        let plain = unwrapped();
        let question = parse(&plain, r#"{"prompt": "Explain."}"#).expect("parses");
        assert_eq!(body(&plain, "2b", &question)["max_tokens"], plain.write.max_tokens);
        assert!(!needs_wrap(&plain, &question, &answer("Step", "length", 1024, 50.0)));
    }

    #[test]
    fn only_a_cut_answer_is_wrapped_up() {
        let config = wrapping();
        let question = parse(&config, r#"{"prompt": "Explain."}"#).expect("parses");
        assert!(needs_wrap(&config, &question, &answer("Step", "length", 928, 50.0)));
        assert!(!needs_wrap(&config, &question, &answer("Done.", FINISH_DONE, 20, 50.0)));
        assert!(!needs_wrap(&config, &question, &answer("  ", "length", 928, 50.0)));
    }

    #[test]
    fn the_conclusion_is_joined_and_counted() {
        let config = wrapping();
        let whole = join(
            &config,
            answer("Step 1 \n", "length", 900, 50.0),
            answer(" The answer is (B). ", FINISH_DONE, 20, 10.0),
        );
        assert_eq!(whole.text, "Step 1 | The answer is (B).");
        assert_eq!(whole.finish, FINISH_DONE);
        assert_eq!(whole.answer_tokens, 920);
        assert_eq!(whole.prompt_tokens, 40);
        assert_eq!(whole.answer_per_second, 46.0);
    }

    #[test]
    fn an_empty_prompt_is_refused() {
        let config = config();
        let failure = parse(&config, r#"{"prompt": "  "}"#).expect_err("refused");
        assert_eq!(failure.error, "bad_job");
    }
}
