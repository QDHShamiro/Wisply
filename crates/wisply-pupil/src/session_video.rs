use std::path::Path;
use std::time::Instant;

use serde_json::json;

use super::{Clip, Draft, Seen, Session};
use crate::chat;
use crate::door;
use crate::ear;
use crate::inputs;
use crate::job::{Failure, Job, Reply};
use crate::long;
use crate::media::Kind;
use crate::talk::{Out, Spent, Told, Turn, answer_of, crowded, heard_room, picture_part, push_text, told};
use crate::timecode;
use crate::trace;
use crate::video;
use crate::watch;

impl Session<'_> {
    fn job(&self, path: &Path, asked: &str) -> Job {
        let talk = &self.config.talk;
        let key = talk.answer_key.as_str();
        Job {
            video: path.to_path_buf(),
            start: 0.0,
            end: None,
            prompt: asked.to_string(),
            schema: json!({
                "type": "object",
                "properties": {key: {"type": "string", "description": talk.answer_note}},
                "required": [key],
            }),
            transcript: Vec::new(),
            audio: None,
            model: Some(self.model.name.clone()),
            afk: self.mode.afk,
            max_frames: None,
            probs: None,
        }
    }

    fn video(&self, name: &str, path: &Path, asked: &str, used: u32, out: &mut dyn Out) -> Result<Seen, Failure> {
        let config = self.config;
        let talk = &config.talk;
        if !self.model.mmproj.is_file() {
            let shown = self.model.mmproj.display().to_string();
            return Err(Failure::new(config, "model_missing", &[("path", &shown)]));
        }
        let job = self.job(path, asked);
        let (info, end) = watch::window(config, &job)?;
        let mut local = config.clone();
        local.ear.enabled = talk.ear;
        let reserved = if ear::needed(&local.ear, &job, end) {
            heard_room(end - job.start, talk.heard_tokens_per_minute)
        } else {
            0
        };
        let context = self.context();
        let room = context.saturating_sub(used);
        let mut whole = config.clone();
        whole.server.context = context;
        whole.long.enabled |= crowded(reserved, room, talk.heard_share);
        whole.server.share = talk.share_server;
        if long::parts(job.start, end, &whole.long).len() > 1 {
            if whole.server.share {
                let (afk, now) = (self.mode.afk, self.mode.now);
                door::attach(&whole, self.config_path, &self.model, afk, now)?;
            } else {
                door::own_server(&whole, &self.model, self.mode.now)?;
            }
            let text = &talk.text;
            let reply = watch::watch_told(&whole, self.config_path, &job, &mut |stage| told(text, &mut *out, stage))?;
            let answer = answer_of(&reply.text, &talk.answer_key);
            return Ok(Seen::Whole(answer, Box::new(reply)));
        }
        local.server.context = room.saturating_sub(reserved);
        local.sampling.max_tokens = self.limit();
        let plan = watch::plan(&local, &job, &info, end, false)?;
        let (heard, pictures, cut) = std::thread::scope(|scope| {
            let listener = scope.spawn(|| ear::fill(&local, &job, end));
            let begun = Instant::now();
            let pictures = inputs::pictures(&local, &job, &plan);
            let cut = begun.elapsed().as_secs_f64();
            (listener.join().ok(), pictures, cut)
        });
        let spent =
            Spent { load_seconds: 0.0, ear_seconds: heard.as_ref().map_or(0.0, |found| found.1), frames_seconds: cut };
        let pictures = pictures?;
        let heard = heard.and_then(|found| found.0);
        let job = heard.as_ref().unwrap_or(&job);
        let frames = &config.frames;
        let size = video::frame_size(info.width, info.height, frames.long_side, frames.patch);
        let heard = chat::heard_block(&local, &job.transcript, job.start, end, false);
        let label = talk.text.video.replace("{name}", name);
        let shots = u32::try_from(pictures.list.len()).unwrap_or(u32::MAX);
        let mut cost = shots.saturating_mul(self.frame_cost(size));
        let mut parts = Vec::new();
        for line in [label.as_str(), config.text.frames_header.as_str()] {
            cost += self.count(line);
            push_text(&mut parts, line);
        }
        for picture in &pictures.list {
            let stamp = timecode::format(picture.at);
            push_text(&mut parts, &config.text.frame_label.replace("{time}", &stamp));
            parts.push(picture_part(&picture.jpeg));
        }
        cost += self.count(&heard);
        push_text(&mut parts, &heard);
        Ok(Seen::Parts(parts, cost, spent))
    }

    pub(super) fn clips(
        &mut self,
        clips: &[Clip],
        question: &str,
        draft: &mut Draft,
        out: &mut dyn Out,
    ) -> Result<Option<(String, Box<Reply>)>, Failure> {
        if clips.is_empty() {
            return Ok(None);
        }
        let share = (f64::from(self.room()) * self.config.talk.history_share) as u32;
        self.forget(0, share.max(self.base()))?;
        for (name, path) in clips {
            let mut asked = draft.notes.clone();
            asked.push(question.to_string());
            let used = self.known() + draft.cost + self.count(question);
            match self.video(name, path, &asked.join("\n\n"), used, out)? {
                Seen::Parts(parts, cost, spent) => {
                    draft.parts.extend(parts);
                    draft.cost += cost;
                    draft.spent.add(spent);
                    draft.kinds.push(Kind::Video);
                }
                Seen::Whole(answer, reply) => return Ok(Some((answer, reply))),
            }
        }
        Ok(None)
    }

    pub(super) fn whole(
        &mut self,
        question: &str,
        answer: String,
        reply: &Reply,
        draft: Draft,
        out: &mut dyn Out,
    ) -> Told {
        if trace::on(&self.config.talk.trace) {
            self.log.reply = serde_json::to_value(reply).ok();
        }
        out.word(&answer);
        out.end();
        let mut kinds = draft.kinds;
        kinds.push(Kind::Video);
        let cost = self.count(question) + self.count(&answer);
        self.turns.push(Turn {
            user: json!({"role": "user", "content": question}),
            reply: json!({"role": "assistant", "content": answer}),
            cost,
        });
        Told {
            ok: true,
            text: answer,
            finish: reply.finish.clone(),
            model: reply.model.clone(),
            kinds,
            prep_seconds: reply.seconds.total - reply.seconds.model,
            spent: draft.spent,
            first_seconds: 0.0,
            total_seconds: reply.seconds.total,
            prompt_tokens: reply.prompt_tokens,
            cached_tokens: 0,
            answer_tokens: reply.answer_tokens,
            answer_per_second: reply.answer_per_second,
            stopped: false,
        }
    }
}
