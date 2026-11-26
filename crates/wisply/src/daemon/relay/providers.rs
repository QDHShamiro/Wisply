use serde_json::{Value, json};

use super::super::config;
use super::super::memory::chat::{GEMINI_URL, api_key};

pub const DEFAULT_PORT: u64 = 9947;
pub const DEFAULT_LOCAL_URL: &str = "http://127.0.0.1:9930/v1/chat/completions";
pub const DEFAULT_LOCAL_MODEL: &str = "auto";
pub const DEFAULT_LOCAL_CTX: u64 = 65_536;
pub const DEFAULT_SMALL_CHAIN: &str = "groq,openrouter,nvidia,local";
pub const DEFAULT_BIG_CHAIN: &str = "local,openrouter,nvidia";
const LOCAL_RESERVE: u64 = 8_192;

#[derive(Clone, Debug)]
pub struct Provider {
    pub name: String,
    pub url: String,
    pub model: String,
    pub key_env: String,
    pub key: Option<String>,
    pub max_prompt: u64,
    pub max_output: u64,
    pub vision: bool,
    pub local: bool,
    pub code_ok: bool,
    pub strip: Vec<String>,
    pub extra: Value,
}

impl Provider {
    pub fn ready(&self) -> bool {
        self.local || self.key.is_some()
    }
}

fn cloud(name: &str, url: &str, model: &str, key_env: &str) -> Provider {
    Provider {
        name: name.into(),
        url: url.into(),
        model: model.into(),
        key_env: key_env.into(),
        key: api_key(key_env),
        max_prompt: 30_000,
        max_output: 4_096,
        vision: false,
        local: false,
        code_ok: true,
        strip: Vec::new(),
        extra: json!({}),
    }
}

fn presets(cfg: &serde_json::Map<String, Value>) -> Vec<Provider> {
    let ctx = number(cfg, "fallback_local_ctx", DEFAULT_LOCAL_CTX);
    let mut local = cloud("local", &text_or(cfg, "fallback_local_url", DEFAULT_LOCAL_URL), "", "");
    local.model = text_or(cfg, "fallback_local_model", DEFAULT_LOCAL_MODEL);
    local.key = None;
    local.local = true;
    local.max_prompt = ctx.saturating_sub(LOCAL_RESERVE);
    local.max_output = LOCAL_RESERVE;
    let mut groq =
        cloud("groq", "https://api.groq.com/openai/v1/chat/completions", "openai/gpt-oss-120b", "GROQ_API_KEY");
    groq.max_prompt = 6_000;
    groq.max_output = 2_048;
    groq.extra = json!({"reasoning_effort": "low"});
    let mut router =
        cloud("openrouter", "https://openrouter.ai/api/v1/chat/completions", "openrouter/free", "OPENROUTER_API_KEY");
    router.max_prompt = 60_000;
    router.extra = json!({"provider": {"data_collection": "deny"}});
    let mut nvidia = cloud(
        "nvidia",
        "https://integrate.api.nvidia.com/v1/chat/completions",
        "meta/llama-3.3-70b-instruct",
        "NVIDIA_API_KEY",
    );
    nvidia.max_prompt = 100_000;
    let mut gemini = cloud("gemini", GEMINI_URL, "gemini-2.5-flash", "GEMINI_API_KEY");
    gemini.max_prompt = 200_000;
    gemini.vision = true;
    gemini.code_ok = false;
    gemini.strip = vec!["additionalProperties".into(), "$ref".into(), "$defs".into()];
    vec![local, groq, router, nvidia, gemini]
}

fn text_or(cfg: &serde_json::Map<String, Value>, key: &str, fallback: &str) -> String {
    let t = config::text(cfg, key);
    if t.trim().is_empty() { fallback.to_string() } else { t.trim().to_string() }
}

fn number(cfg: &serde_json::Map<String, Value>, key: &str, fallback: u64) -> u64 {
    cfg.get(key).and_then(Value::as_u64).filter(|n| *n > 0).unwrap_or(fallback)
}

fn custom(cfg: &serde_json::Map<String, Value>) -> Vec<Provider> {
    let raw = config::text(cfg, "fallback_custom");
    let Ok(Value::Array(items)) = serde_json::from_str::<Value>(&raw) else { return Vec::new() };
    items
        .iter()
        .filter_map(|v| {
            let name = v["name"].as_str()?;
            let url = v["url"].as_str()?;
            let key_env = v["key_env"].as_str().unwrap_or("");
            let mut p = cloud(name, url, v["model"].as_str().unwrap_or(""), key_env);
            p.local = v["local"].as_bool().unwrap_or(false);
            p.code_ok = v["code_ok"].as_bool().unwrap_or(true);
            p.vision = v["vision"].as_bool().unwrap_or(false);
            p.max_prompt = v["max_prompt_tokens"].as_u64().unwrap_or(p.max_prompt);
            p.max_output = v["max_output_tokens"].as_u64().unwrap_or(p.max_output);
            if let Some(extra) = v.get("extra").filter(|e| e.is_object()) {
                p.extra = extra.clone();
            }
            Some(p)
        })
        .collect()
}

pub fn all(cfg: &serde_json::Map<String, Value>) -> Vec<Provider> {
    let mut list = presets(cfg);
    for extra in custom(cfg) {
        list.retain(|p| p.name != extra.name);
        list.push(extra);
    }
    list
}

pub fn chain(cfg: &serde_json::Map<String, Value>, key: &str, fallback: &str) -> Vec<String> {
    text_or(cfg, key, fallback)
        .split([',', ' ', ';'])
        .map(str::trim)
        .filter(|w| !w.is_empty())
        .map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(pairs: &[(&str, Value)]) -> serde_json::Map<String, Value> {
        pairs.iter().map(|(k, v)| ((*k).to_string(), v.clone())).collect()
    }

    #[test]
    fn the_local_window_keeps_room_for_the_answer() {
        let list = all(&cfg(&[("fallback_local_ctx", json!(65_536))]));
        let local = list.iter().find(|p| p.name == "local").unwrap();
        assert!(local.local && local.ready() && local.max_prompt == 65_536 - LOCAL_RESERVE);
    }

    #[test]
    fn gemini_never_carries_code_and_custom_entries_replace_presets() {
        let raw = r#"[{"name":"omni","url":"http://127.0.0.1:20128/v1/chat/completions","model":"auto","local":true,"max_prompt_tokens":40000},{"name":"groq","url":"u","model":"m"}]"#;
        let list = all(&cfg(&[("fallback_custom", json!(raw))]));
        assert!(!list.iter().find(|p| p.name == "gemini").unwrap().code_ok);
        let omni = list.iter().find(|p| p.name == "omni").unwrap();
        assert!(omni.local && omni.max_prompt == 40_000);
        assert_eq!(list.iter().filter(|p| p.name == "groq").count(), 1);
        assert_eq!(list.iter().find(|p| p.name == "groq").unwrap().url, "u");
    }

    #[test]
    fn chains_split_on_commas_and_spaces() {
        let c = cfg(&[("fallback_small_chain", json!("groq, local;nvidia"))]);
        assert_eq!(chain(&c, "fallback_small_chain", DEFAULT_SMALL_CHAIN), ["groq", "local", "nvidia"]);
        assert_eq!(chain(&c, "fallback_big_chain", DEFAULT_BIG_CHAIN), ["local", "openrouter", "nvidia"]);
    }
}
