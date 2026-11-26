use super::providers::Provider;
use super::state;

pub const SMALL_TOKENS: u64 = 6_000;
const CHARS_PER_TOKEN: usize = 3;

#[derive(Clone, Copy)]
pub struct Need {
    pub tokens: u64,
    pub tools: bool,
    pub small: bool,
    pub private: bool,
    pub game: bool,
    pub free_only_cloud: bool,
}

pub fn tokens_of(chars: usize) -> u64 {
    (chars / CHARS_PER_TOKEN) as u64
}

pub fn small(model: &str, tokens: u64) -> bool {
    model.to_lowercase().contains("haiku") || tokens < SMALL_TOKENS
}

fn usable(p: &Provider, need: &Need) -> bool {
    p.ready()
        && state::cooling(&p.name) == 0
        && need.tokens <= p.max_prompt
        && !(need.tools && !p.code_ok)
        && !(need.private && !p.local)
        && !(need.game && p.local)
        && !(need.free_only_cloud && p.local && need.game)
}

#[derive(Debug, PartialEq)]
pub enum Game {
    Open,
    Blocked,
    Ask,
}

pub fn game_rule(policy: &str, allowed: bool, local_wanted: bool) -> Game {
    match policy.trim() {
        "always" => Game::Open,
        "never" => Game::Blocked,
        _ if allowed => Game::Open,
        _ if !local_wanted => Game::Blocked,
        _ => Game::Ask,
    }
}

pub fn order<'a>(all: &'a [Provider], chain: &[String], need: &Need) -> Vec<&'a Provider> {
    chain.iter().filter_map(|name| all.iter().find(|p| &p.name == name)).filter(|p| usable(p, need)).collect()
}

pub fn why_none(all: &[Provider], chain: &[String], need: &Need) -> String {
    if need.private && need.game {
        return "private text may only go to the local model and a game is running".into();
    }
    let known: Vec<&Provider> = chain.iter().filter_map(|n| all.iter().find(|p| &p.name == n)).collect();
    if known.iter().all(|p| !p.ready()) {
        return "no provider has a key (wisply fallback keys)".into();
    }
    if known.iter().filter(|p| p.ready()).all(|p| need.tokens > p.max_prompt) {
        return format!("the prompt is about {} tokens and no ready provider carries that much", need.tokens);
    }
    "every provider is cooling down or excluded for this request".into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn prov(name: &str, local: bool, key: bool, max: u64, code: bool) -> Provider {
        Provider {
            name: name.into(),
            url: String::new(),
            model: String::new(),
            key_env: String::new(),
            key: key.then(|| "k".to_string()),
            max_prompt: max,
            max_output: 1000,
            vision: false,
            local,
            code_ok: code,
            strip: vec![],
            extra: json!({}),
        }
    }

    fn need() -> Need {
        Need { tokens: 20_000, tools: true, small: false, private: false, game: false, free_only_cloud: false }
    }

    fn names(v: Vec<&Provider>) -> Vec<String> {
        v.iter().map(|p| p.name.clone()).collect()
    }

    fn chain(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn during_a_game_the_local_model_needs_a_yes_unless_the_rule_says_otherwise() {
        assert_eq!(game_rule("ask", false, true), Game::Ask);
        assert_eq!(game_rule("ask", true, true), Game::Open);
        assert_eq!(game_rule("ask", false, false), Game::Blocked);
        assert_eq!(game_rule("", false, true), Game::Ask);
        assert_eq!(game_rule("always", false, true), Game::Open);
        assert_eq!(game_rule("never", true, true), Game::Blocked);
    }

    #[test]
    fn a_big_session_skips_groq_and_keeps_the_chain_order() {
        let all = [
            prov("local", true, false, 57_000, true),
            prov("groq", false, true, 6_000, true),
            prov("or", false, true, 60_000, true),
        ];
        assert_eq!(names(order(&all, &chain(&["local", "groq", "or"]), &need())), ["local", "or"]);
    }

    #[test]
    fn a_game_removes_the_local_model_and_private_text_leaves_only_local() {
        let all = [prov("local", true, false, 57_000, true), prov("or", false, true, 60_000, true)];
        let c = chain(&["local", "or"]);
        let gaming = Need { game: true, ..need() };
        assert_eq!(names(order(&all, &c, &gaming)), ["or"]);
        let secret = Need { private: true, ..need() };
        assert_eq!(names(order(&all, &c, &secret)), ["local"]);
        let both = Need { private: true, game: true, ..need() };
        assert!(order(&all, &c, &both).is_empty());
        assert!(why_none(&all, &c, &both).contains("game"));
    }

    #[test]
    fn providers_without_a_key_or_not_for_code_are_left_out_and_small_means_haiku_or_short() {
        let all = [prov("g", false, true, 200_000, false), prov("n", false, false, 100_000, true)];
        assert!(order(&all, &chain(&["g", "n"]), &need()).is_empty());
        assert_eq!(names(order(&all, &chain(&["g"]), &Need { tools: false, ..need() })), ["g"]);
        assert!(
            small("claude-haiku-5-5", 50_000) && small("claude-opus-5-5", 3_000) && !small("claude-opus-5-5", 9_000)
        );
        assert!(why_none(&all, &chain(&["n"]), &need()).contains("no provider has a key"));
    }
}
