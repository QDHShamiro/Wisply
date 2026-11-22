use serde_json::{Value, json};

use crate::config;
use crate::fail::{Fail, Kind};

pub const SUMMARY_KEY: &str = "sum";
pub const POINTS_KEY: &str = "pts";
pub const STEPS_KEY: &str = "steps";
pub const SOUNDS_KEY: &str = "snd";
pub const TIME_KEY: &str = "t";
pub const ACTION_KEY: &str = "do";
pub const KEYS_KEY: &str = "keys";
pub const MENU_KEY: &str = "menu";
pub const VALUES_KEY: &str = "vals";
pub const NAME_KEY: &str = "n";
pub const VALUE_KEY: &str = "v";
pub const CODE_KEY: &str = "code";
pub const RESULT_KEY: &str = "see";
pub const UNSURE_KEY: &str = "unsure";
pub const WHAT_KEY: &str = "what";
pub const TEXT_KEY: &str = "x";

const TIME_TEXT: &str = "Time as MM:SS, or H:MM:SS past one hour.";
const SUMMARY_TEXT: &str = "The gist of the video in at most three sentences.";
const POINTS_TEXT: &str = "The key points in order.";
const POINT_TEXT: &str = "The point in one sentence.";
const SOUNDS_TEXT: &str = "Notable sounds other than speech: music, clicks, alerts, effects, ambience.";
const SOUND_TEXT: &str = "The sound in a few words.";
const STEPS_TEXT: &str = "Every step in order, one per action.";
const ACTION_TEXT: &str = "The action in a few words.";
const KEYS_TEXT: &str = "Shortcuts and mouse actions, only when shown or said.";
const MENU_TEXT: &str = "Menu path, one level per entry.";
const VALUES_TEXT: &str = "Every value typed or set, with the name of its field.";
const NAME_TEXT: &str = "Field name as shown.";
const VALUE_TEXT: &str = "Value exactly as shown.";
const CODE_TEXT: &str = "Code typed on screen, exact.";
const RESULT_TEXT: &str = "What visibly changes afterwards.";
const UNSURE_TEXT: &str = "True when a value or text was not clearly readable.";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Summary,
    Steps,
    Full,
}

impl Mode {
    pub const ALL: [Mode; 3] = [Mode::Summary, Mode::Steps, Mode::Full];

    pub fn name(self) -> &'static str {
        match self {
            Mode::Summary => "summary",
            Mode::Steps => "steps",
            Mode::Full => "full",
        }
    }

    pub fn parse(value: &str) -> Result<Mode, Fail> {
        let cleaned = value.trim().to_lowercase();
        Mode::ALL.into_iter().find(|mode| mode.name() == cleaned).ok_or_else(|| {
            let names: Vec<&str> = Mode::ALL.iter().map(|mode| mode.name()).collect();
            let message = format!("mode '{}' is unknown", value.trim());
            Fail::hint(Kind::Input, message, format!("use one of: {}", names.join(", ")))
        })
    }
}

fn text(description: &str) -> Value {
    json!({"type": "string", "description": description})
}

fn capped(description: &str, most: usize) -> String {
    format!("{description} At most {most}.")
}

fn text_list(description: &str, most: usize) -> Value {
    json!({"type": "array", "description": capped(description, most), "items": {"type": "string"}})
}

fn item_list(description: &str, properties: Value, required: [&str; 2], most: usize) -> Value {
    json!({
        "type": "array",
        "description": capped(description, most),
        "items": {"type": "object", "properties": properties, "required": required},
    })
}

fn steps(settings: &config::Analysis) -> Value {
    let most = settings.max_list_items;
    let fields = json!({NAME_KEY: text(NAME_TEXT), VALUE_KEY: text(VALUE_TEXT)});
    let properties = json!({
        TIME_KEY: text(TIME_TEXT),
        ACTION_KEY: text(ACTION_TEXT),
        KEYS_KEY: text_list(KEYS_TEXT, most),
        MENU_KEY: text_list(MENU_TEXT, most),
        VALUES_KEY: item_list(VALUES_TEXT, fields, [NAME_KEY, VALUE_KEY], most),
        CODE_KEY: text(CODE_TEXT),
        RESULT_KEY: text(RESULT_TEXT),
        UNSURE_KEY: {"type": "boolean", "description": UNSURE_TEXT},
    });
    item_list(STEPS_TEXT, properties, [TIME_KEY, ACTION_KEY], settings.max_steps)
}

fn sounds(settings: &config::Analysis) -> Value {
    let properties = json!({TIME_KEY: text(TIME_TEXT), WHAT_KEY: text(SOUND_TEXT)});
    item_list(SOUNDS_TEXT, properties, [TIME_KEY, WHAT_KEY], settings.max_sounds)
}

fn points(settings: &config::Analysis) -> Value {
    let properties = json!({TIME_KEY: text(TIME_TEXT), TEXT_KEY: text(POINT_TEXT)});
    item_list(POINTS_TEXT, properties, [TIME_KEY, TEXT_KEY], settings.max_points)
}

pub fn answer(mode: Mode, settings: &config::Analysis) -> Value {
    let (properties, required) = match mode {
        Mode::Summary => (
            json!({SUMMARY_KEY: text(SUMMARY_TEXT), POINTS_KEY: points(settings), SOUNDS_KEY: sounds(settings)}),
            vec![SUMMARY_KEY, POINTS_KEY, SOUNDS_KEY],
        ),
        Mode::Steps => (json!({STEPS_KEY: steps(settings), SOUNDS_KEY: sounds(settings)}), vec![STEPS_KEY, SOUNDS_KEY]),
        Mode::Full => (
            json!({SUMMARY_KEY: text(SUMMARY_TEXT), STEPS_KEY: steps(settings), SOUNDS_KEY: sounds(settings)}),
            vec![SUMMARY_KEY, STEPS_KEY, SOUNDS_KEY],
        ),
    };
    json!({"type": "object", "properties": properties, "required": required})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    fn keys(value: &Value) -> Vec<String> {
        value.as_object().map(|map| map.keys().cloned().collect()).unwrap_or_default()
    }

    #[test]
    fn every_mode_has_a_name_that_parses_back() {
        for mode in Mode::ALL {
            assert_eq!(Mode::parse(&format!(" {} ", mode.name().to_uppercase())).expect("mode"), mode);
        }
        let fail = Mode::parse("deep").expect_err("unknown");
        assert_eq!(fail.kind, Kind::Input);
        assert!(fail.hint.contains("summary, steps, full"), "{}", fail.hint);
    }

    #[test]
    fn the_answer_names_every_part_as_required_in_the_order_it_is_written() {
        let settings = Config::standard().analysis;
        for (mode, wanted) in [
            (Mode::Summary, vec!["sum", "pts", "snd"]),
            (Mode::Steps, vec!["steps", "snd"]),
            (Mode::Full, vec!["sum", "steps", "snd"]),
        ] {
            let schema = answer(mode, &settings);
            assert_eq!(keys(&schema["properties"]), wanted);
            assert_eq!(schema["required"], json!(wanted));
        }
    }

    #[test]
    fn a_step_keeps_time_and_action_first_and_carries_the_limits() {
        let settings = Config::standard().analysis;
        let schema = answer(Mode::Steps, &settings);
        let list = &schema["properties"]["steps"];
        assert_eq!(list["description"], "Every step in order, one per action. At most 80.");
        let step = &list["items"];
        assert_eq!(keys(&step["properties"]), ["t", "do", "keys", "menu", "vals", "code", "see", "unsure"]);
        assert_eq!(step["required"], json!(["t", "do"]));
        assert!(step["properties"]["keys"]["description"].as_str().expect("text").ends_with("At most 8."));
        assert!(schema["properties"]["snd"]["description"].as_str().expect("text").ends_with("At most 25."));
    }
}
