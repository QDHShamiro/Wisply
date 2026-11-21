use serde_json::{Map, Value, json};

const THINKING_LEVEL: &str = "low";
const JSON_MIME: &str = "application/json";
const AGENTIC: &str = "agentic";
const STATIC: &str = "static";

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sampling {
    Agentic,
    Fixed { start: Option<u64>, end: Option<u64>, fps: Option<f64> },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Clip<'a> {
    pub url: &'a str,
    pub resolution: &'a str,
    pub sampling: Sampling,
    pub mime_type: Option<&'a str>,
}

fn offset(seconds: u64) -> Value {
    Value::from(format!("{seconds}s"))
}

fn processing(sampling: Sampling) -> Option<Value> {
    let Sampling::Fixed { start, end, fps } = sampling else {
        return Some(Value::from(AGENTIC));
    };
    let mut fixed = Map::new();
    fixed.insert("type".to_string(), Value::from(STATIC));
    let named = [("start_offset", start.map(offset)), ("end_offset", end.map(offset)), ("fps", fps.map(Value::from))];
    fixed.extend(named.into_iter().filter_map(|(name, value)| Some((name.to_string(), value?))));
    (fixed.len() > 1).then_some(Value::Object(fixed))
}

pub fn video_part(clip: &Clip) -> Value {
    let mut part = Map::new();
    part.insert("type".to_string(), Value::from("video"));
    part.insert("uri".to_string(), Value::from(clip.url));
    part.insert("resolution".to_string(), Value::from(clip.resolution));
    let more = [("mime_type", clip.mime_type.map(Value::from)), ("processing", processing(clip.sampling))];
    part.extend(more.into_iter().filter_map(|(name, value)| Some((name.to_string(), value?))));
    Value::Object(part)
}

pub fn request(
    video: Option<Value>,
    prompt: &str,
    max_output_tokens: u64,
    schema: Option<&Value>,
) -> Map<String, Value> {
    let input: Vec<Value> = video.into_iter().chain([json!({"type": "text", "text": prompt})]).collect();
    let mut body = Map::new();
    body.insert("input".to_string(), Value::Array(input));
    let generation = json!({"thinking_level": THINKING_LEVEL, "max_output_tokens": max_output_tokens});
    body.insert("generation_config".to_string(), generation);
    body.insert("store".to_string(), Value::Bool(false));
    if let Some(schema) = schema {
        body.insert("response_format".to_string(), json!({"type": "text", "mime_type": JSON_MIME, "schema": schema}));
    }
    body
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINK: &str = "https://www.youtube.com/watch?v=abcdefghijk";

    fn clip(sampling: Sampling) -> Clip<'static> {
        Clip { url: LINK, resolution: "low", sampling, mime_type: None }
    }

    #[test]
    fn a_whole_video_is_sent_without_a_window() {
        let whole = video_part(&clip(Sampling::Fixed { start: None, end: None, fps: None }));
        assert_eq!(whole, json!({"type": "video", "uri": LINK, "resolution": "low"}));
    }

    #[test]
    fn a_window_and_a_frame_rate_are_named_in_seconds() {
        let part = video_part(&clip(Sampling::Fixed { start: Some(30), end: Some(95), fps: Some(0.5) }));
        let wanted = json!({"type": "video", "uri": LINK, "resolution": "low",
            "processing": {"type": "static", "start_offset": "30s", "end_offset": "95s", "fps": 0.5}});
        assert_eq!(part, wanted);
        assert_eq!(part.to_string(), wanted.to_string());
        let open = video_part(&clip(Sampling::Fixed { start: None, end: Some(600), fps: None }));
        assert_eq!(open["processing"], json!({"type": "static", "end_offset": "600s"}));
    }

    #[test]
    fn an_agentic_video_lets_the_model_pick_its_frames() {
        let file =
            Clip { url: "files/abc", resolution: "high", sampling: Sampling::Agentic, mime_type: Some("video/mp4") };
        let wanted = json!({"type": "video", "uri": "files/abc", "resolution": "high", "mime_type": "video/mp4",
            "processing": "agentic"});
        assert_eq!(video_part(&file).to_string(), wanted.to_string());
    }

    #[test]
    fn a_request_holds_the_video_the_question_and_the_answer_form() {
        let video = video_part(&clip(Sampling::Agentic));
        let schema = json!({"type": "object", "properties": {"summary": {"type": "string"}}});
        let body = Value::Object(request(Some(video.clone()), "What happens?", 3000, Some(&schema)));
        let wanted = json!({
            "input": [video, {"type": "text", "text": "What happens?"}],
            "generation_config": {"thinking_level": "low", "max_output_tokens": 3000},
            "store": false,
            "response_format": {"type": "text", "mime_type": "application/json", "schema": schema}
        });
        assert_eq!(body.to_string(), wanted.to_string());
        let plain = Value::Object(request(None, "Hi", 10, None));
        assert_eq!(plain["input"], json!([{"type": "text", "text": "Hi"}]));
        assert!(plain.get("response_format").is_none());
    }
}
