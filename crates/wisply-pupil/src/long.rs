use serde_json::{Map, Value};

use crate::config::{Config, Long, Text};
use crate::schema::{field_guide, harden};
use crate::timecode;

pub fn parts(start: f64, end: f64, long: &Long) -> Vec<(f64, f64)> {
    let length = end - start;
    if !long.enabled || long.part_seconds <= 0.0 || length <= long.min_seconds {
        return vec![(start, end)];
    }
    let count = (length / long.part_seconds).ceil().max(1.0) as usize;
    let step = length / count as f64;
    (0..count)
        .map(|i| {
            let to = if i + 1 == count { end } else { start + step * (i + 1) as f64 };
            (start + step * i as f64, to)
        })
        .collect()
}

pub fn share(limit: u64, index: usize, count: usize, least: u64) -> u64 {
    let count = count.max(1) as u64;
    let index = index as u64;
    let own = (index + 1) * limit / count - index * limit / count;
    own.max(least).min(limit.max(least))
}

pub fn part_frames(long: &Long, index: usize, count: usize) -> Option<usize> {
    (long.frames_total > 0).then(|| share(long.frames_total, index, count, long.least_frames) as usize)
}

fn with_limit(description: &str, phrase: &str, limit: u64) -> String {
    let lower = description.to_lowercase();
    let Some(found) = lower.find(&phrase.to_lowercase()) else {
        return description.to_string();
    };
    let after = found + phrase.len();
    let rest = &description[after..];
    let gap = rest.len() - rest.trim_start().len();
    let digits = rest[gap..].chars().take_while(char::is_ascii_digit).count();
    if digits == 0 {
        return description.to_string();
    }
    format!("{}{}{limit}{}", &description[..after], &rest[..gap], &rest[gap + digits..])
}

pub fn part_schema(schema: &Value, text: &Text, long: &Long, index: usize, count: usize) -> Value {
    let mut hard = harden(schema, text);
    if count < 2 {
        return hard;
    }
    let Some(Value::Object(properties)) = hard.get_mut("properties") else {
        return hard;
    };
    for field in properties.values_mut() {
        let Value::Object(map) = field else {
            continue;
        };
        if map.get("type").and_then(Value::as_str) != Some("array") {
            continue;
        }
        let Some(limit) = map.get("maxItems").and_then(Value::as_u64) else {
            continue;
        };
        let total = match long.items_total {
            0 => limit,
            cap => limit.min(cap),
        };
        let own = share(total, index, count, long.least_items);
        map.insert("maxItems".into(), Value::from(own));
        if let Some(lowest) = map.get("minItems").and_then(Value::as_u64) {
            map.insert("minItems".into(), Value::from(lowest.min(own)));
        }
        if let Some(Value::String(description)) = map.get_mut("description") {
            *description = with_limit(description, &text.max_items_phrase, own);
        }
    }
    hard
}

fn is_list(field: &Value) -> bool {
    field.get("type").and_then(Value::as_str) == Some("array")
}

pub fn spread_pick(list: Vec<Value>, limit: usize) -> Vec<Value> {
    if limit == 0 || list.len() <= limit {
        return list;
    }
    let total = list.len();
    (0..limit).map(|i| list[i * total / limit].clone()).collect()
}

pub struct Merged {
    pub value: Map<String, Value>,
    pub open: Vec<String>,
}

pub fn merge(schema: &Value, text: &Text, answers: &[Value]) -> Merged {
    let hard = harden(schema, text);
    let empty = Map::new();
    let properties = hard.get("properties").and_then(Value::as_object).unwrap_or(&empty);
    let mut value = Map::new();
    let mut open = Vec::new();
    for (name, field) in properties {
        if !is_list(field) {
            open.push(name.clone());
            continue;
        }
        let mut list: Vec<Value> = Vec::new();
        for item in answers.iter().filter_map(|a| a.get(name)).filter_map(Value::as_array).flatten() {
            if !list.contains(item) {
                list.push(item.clone());
            }
        }
        let limit = field.get("maxItems").and_then(Value::as_u64).map_or(0, |n| n as usize);
        value.insert(name.clone(), Value::Array(spread_pick(list, limit)));
    }
    if answers.len() == 1 {
        for name in open.drain(..) {
            if let Some(found) = answers[0].get(&name) {
                value.insert(name, found.clone());
            }
        }
    }
    Merged { value, open }
}

pub fn open_schema(schema: &Value, text: &Text, open: &[String]) -> Value {
    let hard = harden(schema, text);
    let mut out = Map::new();
    for (key, item) in hard.as_object().into_iter().flatten() {
        match key.as_str() {
            "properties" => {
                let kept: Map<String, Value> = item
                    .as_object()
                    .into_iter()
                    .flatten()
                    .filter(|(name, _)| open.contains(name))
                    .map(|(name, field)| (name.clone(), field.clone()))
                    .collect();
                out.insert(key.clone(), Value::Object(kept));
            }
            "required" => {
                let kept: Vec<Value> = item
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter(|name| name.as_str().is_some_and(|n| open.iter().any(|o| o == n)))
                    .cloned()
                    .collect();
                out.insert(key.clone(), Value::Array(kept));
            }
            _ => {
                out.insert(key.clone(), item.clone());
            }
        }
    }
    Value::Object(out)
}

pub fn notes(long: &Long, ranges: &[(f64, f64)], answers: &[Value], open: &[String]) -> String {
    let lines: Vec<String> = ranges
        .iter()
        .zip(answers)
        .map(|((from, to), answer)| {
            let kept: Map<String, Value> =
                open.iter().filter_map(|name| answer.get(name).map(|v| (name.clone(), v.clone()))).collect();
            long.notes_line
                .replace("{from}", &timecode::format(*from))
                .replace("{to}", &timecode::format(*to))
                .replace("{notes}", &Value::Object(kept).to_string())
        })
        .collect();
    format!("{}\n{}", long.notes_header, lines.join("\n"))
}

pub fn merge_tail(config: &Config, notes: &str, prompt: &str, schema: &Value) -> String {
    let guide = field_guide(schema, &config.text);
    [notes, prompt.trim(), guide.as_str(), config.long.merge_rule.as_str()]
        .iter()
        .filter(|p| !p.is_empty())
        .copied()
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{merge, notes, open_schema, part_frames, part_schema, parts, share, spread_pick, with_limit};
    use crate::config::Long;
    use crate::schema::tests::text;

    fn long() -> Long {
        Long {
            enabled: true,
            min_seconds: 900.0,
            part_seconds: 600.0,
            least_items: 1,
            items_total: 0,
            ahead: 1,
            frames_total: 0,
            least_frames: 1,
            skip_failed: false,
            notes_header: "Notes:".into(),
            notes_line: "{from} to {to}: {notes}".into(),
            merge_rule: "Merge.".into(),
        }
    }

    fn summary() -> serde_json::Value {
        json!({
            "type": "object",
            "required": ["sum", "pts"],
            "properties": {
                "sum": {"type": "string", "description": "The gist."},
                "pts": {"type": "array", "description": "The key points in order. At most 10.",
                    "items": {"type": "object", "properties": {"t": {"type": "string"}}}}
            }
        })
    }

    #[test]
    fn a_frame_budget_is_spread_over_the_parts() {
        let mut setup = long();
        assert_eq!(part_frames(&setup, 0, 6), None);
        setup.frames_total = 64;
        let each: Vec<usize> = (0..6).filter_map(|i| part_frames(&setup, i, 6)).collect();
        assert_eq!(each.iter().sum::<usize>(), 64);
        assert!(each.iter().all(|n| (10..=11).contains(n)));
    }

    #[test]
    fn short_windows_stay_one_part() {
        assert_eq!(parts(0.0, 900.0, &long()), vec![(0.0, 900.0)]);
        let mut off = long();
        off.enabled = false;
        assert_eq!(parts(0.0, 4000.0, &off), vec![(0.0, 4000.0)]);
    }

    #[test]
    fn long_windows_split_evenly_and_end_exactly() {
        let split = parts(100.0, 3820.0, &long());
        assert_eq!(split.len(), 7);
        assert_eq!(split[0].0, 100.0);
        assert_eq!(split[6].1, 3820.0);
        for pair in split.windows(2) {
            assert_eq!(pair[0].1, pair[1].0);
        }
        let lengths: Vec<f64> = split.iter().map(|(a, b)| b - a).collect();
        assert!(lengths.iter().all(|l| (l - lengths[0]).abs() < 1e-6));
    }

    #[test]
    fn shares_add_up_and_spread() {
        let own: Vec<u64> = (0..7).map(|i| share(10, i, 7, 1)).collect();
        assert_eq!(own.iter().sum::<u64>(), 10);
        assert_eq!(own, vec![1, 1, 2, 1, 2, 1, 2]);
        let few: Vec<u64> = (0..7).map(|i| share(3, i, 7, 1)).collect();
        assert!(few.iter().all(|n| *n == 1));
        assert_eq!(share(80, 0, 1, 1), 80);
    }

    #[test]
    fn limit_text_is_rewritten_in_place() {
        assert_eq!(
            with_limit("The key points in order. At most 10.", "At most", 2),
            "The key points in order. At most 2."
        );
        assert_eq!(with_limit("no limit here", "At most", 2), "no limit here");
        assert_eq!(with_limit("at most  80 steps", "At most", 12), "at most  12 steps");
    }

    #[test]
    fn part_schema_scales_lists_only() {
        let schema = part_schema(&summary(), &text(), &long(), 2, 7);
        assert_eq!(schema["properties"]["pts"]["maxItems"], 2);
        assert_eq!(schema["properties"]["pts"]["minItems"], 1);
        assert_eq!(schema["properties"]["pts"]["description"], "The key points in order. At most 2.");
        assert_eq!(schema["properties"]["sum"]["type"], "string");
        let whole = part_schema(&summary(), &text(), &long(), 0, 1);
        assert_eq!(whole["properties"]["pts"]["maxItems"], 10);
    }

    #[test]
    fn a_total_item_cap_is_shared_over_the_parts() {
        let mut setup = long();
        setup.items_total = 4;
        let own: Vec<u64> = (0..4)
            .map(|i| {
                part_schema(&summary(), &text(), &setup, i, 4)["properties"]["pts"]["maxItems"].as_u64().expect("limit")
            })
            .collect();
        assert_eq!(own, vec![1, 1, 1, 1]);
        setup.items_total = 40;
        let wide = part_schema(&summary(), &text(), &setup, 0, 2);
        assert_eq!(wide["properties"]["pts"]["maxItems"], 5);
    }

    #[test]
    fn merge_joins_lists_in_order_without_twins() {
        let answers = vec![
            json!({"sum": "a", "pts": [{"t": "00:10"}, {"t": "00:20"}]}),
            json!({"sum": "b", "pts": [{"t": "00:20"}, {"t": "10:05"}]}),
        ];
        let merged = merge(&summary(), &text(), &answers);
        assert_eq!(merged.value["pts"], json!([{"t": "00:10"}, {"t": "00:20"}, {"t": "10:05"}]));
        assert_eq!(merged.open, vec!["sum".to_string()]);
        assert!(merged.value.get("sum").is_none());
    }

    #[test]
    fn merge_of_one_part_keeps_everything() {
        let answers = vec![json!({"sum": "a", "pts": [{"t": "00:10"}]})];
        let merged = merge(&summary(), &text(), &answers);
        assert!(merged.open.is_empty());
        assert_eq!(merged.value["sum"], "a");
    }

    #[test]
    fn too_many_items_are_thinned_evenly() {
        let list: Vec<_> = (0..14).map(|i| json!(i)).collect();
        let kept = spread_pick(list, 10);
        assert_eq!(kept.len(), 10);
        assert_eq!(kept[0], json!(0));
        assert_eq!(kept[9], json!(12));
        let short = spread_pick(vec![json!(1)], 10);
        assert_eq!(short, vec![json!(1)]);
    }

    #[test]
    fn open_schema_keeps_only_open_fields() {
        let schema = open_schema(&summary(), &text(), &["sum".to_string()]);
        assert!(schema["properties"].get("pts").is_none());
        assert_eq!(schema["required"], json!(["sum"]));
        assert_eq!(schema["type"], "object");
    }

    #[test]
    fn notes_list_each_part_with_its_time() {
        let answers = vec![json!({"sum": "a", "pts": []}), json!({"sum": "b"})];
        let text = notes(&long(), &[(0.0, 600.0), (600.0, 1200.0)], &answers, &["sum".to_string()]);
        assert_eq!(text, "Notes:\n00:00 to 10:00: {\"sum\":\"a\"}\n10:00 to 20:00: {\"sum\":\"b\"}");
    }
}
