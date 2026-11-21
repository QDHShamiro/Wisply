use std::io::Read;

use serde_json::{Value, json};

use crate::chat;
use crate::config::Config;
use crate::inputs;
use crate::job::{Failure, Job};
use crate::long;
use crate::watch;

fn record(model: &str, frames: &[f64], body: Value) -> Value {
    json!({"ok": true, "model": model, "frames": frames, "body": body})
}

pub fn prepare(config: &Config, job: &Job) -> Result<Value, Failure> {
    let model = watch::check(config, job)?;
    let (info, end) = watch::window(config, job)?;
    if long::parts(job.start, end, &config.long).len() > 1 {
        let (start, end) = (job.start.to_string(), end.to_string());
        return Err(Failure::new(config, "prep_long", &[("start", &start), ("end", &end)]));
    }
    let hears = model.hears && config.audio.enabled;
    let plan = watch::plan(config, job, &info, end, hears)?;
    let pictures = inputs::pictures(config, job, &plan)?;
    let mut body = chat::body(config, &model.name, &plan.tail, &pictures.list, &job.schema);
    if let Some(sound) = &pictures.sound {
        chat::add_sound(config, &mut body, sound);
    }
    let frames: Vec<f64> = pictures.list.iter().map(|p| p.at).collect();
    Ok(record(&model.name, &frames, body))
}

fn one(config: &Config, raw: &str) -> Value {
    match Job::parse(config, raw).and_then(|job| prepare(config, &job)) {
        Ok(value) => value,
        Err(failure) => serde_json::to_value(&failure).unwrap_or_default(),
    }
}

fn many(config: &Config, jobs: &[Value]) -> Value {
    let made: Vec<Value> = std::thread::scope(|scope| {
        let running: Vec<_> = jobs.iter().map(|job| scope.spawn(move || one(config, &job.to_string()))).collect();
        running.into_iter().map(|handle| handle.join().unwrap_or_default()).collect()
    });
    Value::Array(made)
}

pub fn run(config: &Config) -> i32 {
    let mut raw = String::new();
    if let Err(e) = std::io::stdin().read_to_string(&mut raw) {
        let failure = Failure::new(config, "bad_job", &[("detail", &e.to_string())]);
        println!("{}", serde_json::to_string(&failure).unwrap_or_default());
        return 1;
    }
    let value = match serde_json::from_str::<Value>(&raw) {
        Ok(Value::Array(jobs)) => many(config, &jobs),
        _ => one(config, &raw),
    };
    println!("{value}");
    i32::from(value["ok"] == false)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::record;

    #[test]
    fn a_prepared_job_keeps_the_exact_request_and_frame_times() {
        let body = json!({"model": "2b", "messages": [{"role": "user", "content": []}]});
        let made = record("2b", &[0.0, 12.5], body.clone());
        assert_eq!(made["ok"], json!(true));
        assert_eq!(made["frames"], json!([0.0, 12.5]));
        assert_eq!(made["body"], body);
    }
}
