use serde_json::{Map, Value, json};

use crate::fail::{Fail, Kind};
use crate::schema::{self, Mode};
use crate::timecode;

const CUT_KEY: &str = "cut";
const DROPPED_KEY: &str = "dropped";
const MAX_SALVAGE_TRIES: usize = 64;
const WHOLE_ITEM_DEPTH: usize = 2;
const UNREADABLE: &str = "Gemini sent an answer that is not readable JSON";
const NOT_AN_OBJECT: &str = "Gemini sent JSON that is not an object";
const UNREADABLE_HINT: &str = "ask again";

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Step {
    pub at: Option<f64>,
    pub action: String,
    pub keys: Vec<String>,
    pub menus: Vec<String>,
    pub values: Vec<Field>,
    pub code: String,
    pub result: String,
    pub unsure: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Moment {
    pub at: Option<f64>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Analysis {
    pub mode: Mode,
    pub summary: String,
    pub points: Vec<Moment>,
    pub steps: Vec<Step>,
    pub sounds: Vec<Moment>,
    pub dropped: usize,
    pub cut: bool,
}

impl Analysis {
    pub fn unsure_times(&self) -> Vec<f64> {
        self.steps.iter().filter(|step| step.unsure).filter_map(|step| step.at).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.summary.is_empty() && self.points.is_empty() && self.steps.is_empty() && self.sounds.is_empty()
    }
}

fn list(value: Option<&Value>) -> &[Value] {
    value.and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default()
}

fn one_line(value: Option<&Value>) -> String {
    let words: Vec<&str> = value.and_then(Value::as_str).unwrap_or_default().split_whitespace().collect();
    words.join(" ")
}

fn code_text(value: Option<&Value>) -> String {
    let unix = value.and_then(Value::as_str).unwrap_or_default().replace("\r\n", "\n");
    let lines: Vec<&str> = unix.split('\n').map(str::trim_end).collect();
    lines.join("\n").trim_matches('\n').to_string()
}

fn unique_texts(value: Option<&Value>) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for entry in list(value) {
        let cleaned = one_line(Some(entry));
        if !cleaned.is_empty() && !seen.contains(&cleaned) {
            seen.push(cleaned);
        }
    }
    seen
}

fn read_values(value: Option<&Value>) -> Vec<Field> {
    let field = |entry: &Value| {
        let entry = entry.as_object()?;
        let value = one_line(entry.get(schema::VALUE_KEY));
        (!value.is_empty()).then(|| Field { name: one_line(entry.get(schema::NAME_KEY)), value })
    };
    list(value).iter().filter_map(field).collect()
}

fn read_time(value: Option<&Value>, shift: f64) -> Option<f64> {
    value.and_then(timecode::find).map(|seconds| seconds + shift)
}

fn read_step(entry: &Value, shift: f64) -> Option<Step> {
    let entry = entry.as_object()?;
    let action = one_line(entry.get(schema::ACTION_KEY));
    if action.is_empty() {
        return None;
    }
    Some(Step {
        at: read_time(entry.get(schema::TIME_KEY), shift),
        action,
        keys: unique_texts(entry.get(schema::KEYS_KEY)),
        menus: unique_texts(entry.get(schema::MENU_KEY)),
        values: read_values(entry.get(schema::VALUES_KEY)),
        code: code_text(entry.get(schema::CODE_KEY)),
        result: one_line(entry.get(schema::RESULT_KEY)),
        unsure: entry.get(schema::UNSURE_KEY) == Some(&Value::Bool(true)),
    })
}

fn read_moment(entry: &Value, text_key: &str, shift: f64) -> Option<Moment> {
    let entry = entry.as_object()?;
    let text = one_line(entry.get(text_key));
    (!text.is_empty()).then(|| Moment { at: read_time(entry.get(schema::TIME_KEY), shift), text })
}

fn salvage_points(text: &str) -> Vec<(usize, Vec<char>)> {
    let mut points = Vec::new();
    let mut open: Vec<char> = Vec::new();
    let (mut in_string, mut escaped) = (false, false);
    for (index, sign) in text.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if sign == '\\' {
                escaped = true;
            } else if sign == '"' {
                in_string = false;
            }
            continue;
        }
        match sign {
            '"' => in_string = true,
            '{' => open.push('}'),
            '[' => open.push(']'),
            '}' | ']' if !open.is_empty() => {
                open.pop();
                if open.len() <= WHOLE_ITEM_DEPTH {
                    points.push((index + 1, open.clone()));
                }
            }
            ',' if !open.is_empty() && open.len() <= WHOLE_ITEM_DEPTH => points.push((index, open.clone())),
            _ => {}
        }
    }
    points
}

fn salvage(text: &str) -> Option<Value> {
    let points = salvage_points(text);
    let first = points.len().saturating_sub(MAX_SALVAGE_TRIES);
    points[first..].iter().rev().find_map(|(end, open)| {
        let closing: String = open.iter().rev().collect();
        serde_json::from_str(&format!("{}{closing}", &text[..*end])).ok()
    })
}

fn load(text: &str) -> Result<(Value, bool), Fail> {
    let stripped = text.trim();
    if let Ok(whole) = serde_json::from_str(stripped) {
        return Ok((whole, false));
    }
    let rescued = salvage(stripped).ok_or_else(|| Fail::hint(Kind::Service, UNREADABLE, UNREADABLE_HINT))?;
    Ok((rescued, true))
}

pub fn read(data: &Value, mode: Mode, shift: f64, cut: bool) -> Result<Analysis, Fail> {
    let Some(data) = data.as_object() else {
        return Err(Fail::hint(Kind::Service, NOT_AN_OBJECT, UNREADABLE_HINT));
    };
    let raw_steps = list(data.get(schema::STEPS_KEY));
    let raw_points = list(data.get(schema::POINTS_KEY));
    let raw_sounds = list(data.get(schema::SOUNDS_KEY));
    let steps: Vec<Step> = raw_steps.iter().filter_map(|entry| read_step(entry, shift)).collect();
    let points: Vec<Moment> =
        raw_points.iter().filter_map(|entry| read_moment(entry, schema::TEXT_KEY, shift)).collect();
    let sounds: Vec<Moment> =
        raw_sounds.iter().filter_map(|entry| read_moment(entry, schema::WHAT_KEY, shift)).collect();
    let wanted = raw_steps.len() + raw_points.len() + raw_sounds.len();
    let kept = steps.len() + points.len() + sounds.len();
    let stored = data.get(DROPPED_KEY).and_then(Value::as_u64).and_then(|count| usize::try_from(count).ok());
    Ok(Analysis {
        mode,
        summary: one_line(data.get(schema::SUMMARY_KEY)),
        points,
        steps,
        sounds,
        dropped: wanted - kept + stored.unwrap_or(0),
        cut: cut || data.get(CUT_KEY) == Some(&Value::Bool(true)),
    })
}

pub fn parse(text: &str, mode: Mode, shift: f64, cut: bool) -> Result<Analysis, Fail> {
    let (data, rescued) = load(text)?;
    read(&data, mode, shift, cut || rescued)
}

fn timed(at: Option<f64>) -> Map<String, Value> {
    let mut data = Map::new();
    if let Some(at) = at {
        data.insert(schema::TIME_KEY.to_string(), json!(at));
    }
    data
}

fn step_data(step: &Step) -> Value {
    let mut data = timed(step.at);
    let mut put = |key: &str, value: Value| data.insert(key.to_string(), value);
    put(schema::ACTION_KEY, json!(step.action));
    if !step.keys.is_empty() {
        put(schema::KEYS_KEY, json!(step.keys));
    }
    if !step.menus.is_empty() {
        put(schema::MENU_KEY, json!(step.menus));
    }
    if !step.values.is_empty() {
        let fields = step.values.iter().map(|item| {
            let mut field = Map::new();
            field.insert(schema::NAME_KEY.to_string(), json!(item.name));
            field.insert(schema::VALUE_KEY.to_string(), json!(item.value));
            Value::Object(field)
        });
        put(schema::VALUES_KEY, fields.collect());
    }
    if !step.code.is_empty() {
        put(schema::CODE_KEY, json!(step.code));
    }
    if !step.result.is_empty() {
        put(schema::RESULT_KEY, json!(step.result));
    }
    if step.unsure {
        put(schema::UNSURE_KEY, json!(true));
    }
    Value::Object(data)
}

fn moments_data(moments: &[Moment], text_key: &str) -> Value {
    let one = |moment: &Moment| {
        let mut data = timed(moment.at);
        data.insert(text_key.to_string(), json!(moment.text));
        Value::Object(data)
    };
    moments.iter().map(one).collect()
}

pub fn data(analysis: &Analysis) -> Value {
    let mut data = Map::new();
    let mut put = |key: &str, value: Value| data.insert(key.to_string(), value);
    if !analysis.summary.is_empty() {
        put(schema::SUMMARY_KEY, json!(analysis.summary));
    }
    if !analysis.points.is_empty() {
        put(schema::POINTS_KEY, moments_data(&analysis.points, schema::TEXT_KEY));
    }
    if !analysis.steps.is_empty() {
        put(schema::STEPS_KEY, analysis.steps.iter().map(step_data).collect());
    }
    if !analysis.sounds.is_empty() {
        put(schema::SOUNDS_KEY, moments_data(&analysis.sounds, schema::WHAT_KEY));
    }
    if analysis.dropped > 0 {
        put(DROPPED_KEY, json!(analysis.dropped));
    }
    if analysis.cut {
        put(CUT_KEY, json!(true));
    }
    Value::Object(data)
}

#[cfg(test)]
#[path = "analysis_tests.rs"]
mod tests;
