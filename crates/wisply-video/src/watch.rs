use serde_json::Value;

use crate::analysis::{self, Analysis};
use crate::asked::Asked;
use crate::bodies::{self, Clip, Sampling};
use crate::captions::Segment;
use crate::config::Config;
use crate::fail::{Fail, Kind};
use crate::gemini::{Gemini, Reply, Request, Transport};
use crate::guard::{Ask, Guard};
use crate::planner::{self, Plan, Span};
use crate::prompt::{self, Facts};
use crate::render::{self, Heading};
use crate::schema::{self, Mode};
use crate::store::{self, AnalysisKey};
use crate::timecode::{self, Range};
use crate::transcript;
use crate::youtube::{Video, VideoInfo, YouTube};

pub const VERB: &str = "watch";
pub const USAGE: &str = "wisply pupil watch <YouTube link> [--mode summary|steps|full] [--focus \"words\"] \
                         [--start S] [--end E] [--page N]";
const MODE: &str = "mode";
const FOCUS: &str = "focus";
const START: &str = "start";
const END: &str = "end";
const PAGE: &str = "page";
const FLAGS: [&str; 5] = [MODE, FOCUS, START, END, PAGE];
const STANDARD: Mode = Mode::Steps;
const FIRST_PAGE: usize = 1;
const NO_TRANSCRIPT: &str = "No transcript, so quiet parts were not pointed out to Gemini";
const NOT_KEPT: &str = "The answer could not be kept for the next call";

pub struct Watcher<'a> {
    pub youtube: &'a YouTube<'a>,
    pub config: &'a Config,
    pub connect: &'a dyn Fn() -> Result<Box<dyn Transport + 'a>, Fail>,
    pub jitter: &'a dyn Fn() -> f64,
}

struct Job<'a> {
    video: &'a Video,
    info: &'a VideoInfo,
    mode: Mode,
    focus: Option<&'a str>,
    seen: Option<Range>,
    span: Span,
}

struct Answer {
    analysis: Analysis,
    model: String,
    requests: usize,
    notes: Vec<String>,
}

fn page_number(asked: Option<&str>) -> Result<usize, Fail> {
    let Some(text) = asked else { return Ok(FIRST_PAGE) };
    text.trim().parse::<usize>().ok().filter(|page| *page >= FIRST_PAGE).ok_or_else(|| {
        Fail::hint(Kind::Input, "page has to be a whole number from 1 up", "leave it out for the first page")
    })
}

fn sentence(start: &str, reason: &str) -> String {
    format!("{start}: {}.", reason.trim_end_matches('.'))
}

fn handed_over(reply: &Reply) -> Vec<String> {
    let mut named = vec![reply.model.as_str()];
    let mut notes = Vec::new();
    for miss in reply.misses.iter().rev() {
        if !named.contains(&miss.model.as_str()) {
            named.push(&miss.model);
            notes.push(format!("{} did not answer ({}), {} took over.", miss.model, miss.problem, reply.model));
        }
    }
    notes.reverse();
    notes
}

fn offsets(span: &Span) -> (Option<u64>, Option<u64>) {
    if span.whole { (None, None) } else { (Some(span.start.floor() as u64), span.end.map(|end| end.ceil() as u64)) }
}

fn sampling(plan: &Plan, span: &Span) -> Sampling {
    let (start, end) = offsets(span);
    plan.fps.map_or(Sampling::Agentic, |fps| Sampling::Fixed { start, end, fps: Some(fps) })
}

impl Watcher<'_> {
    fn speech(&self, video: &Video) -> (Vec<Segment>, Vec<String>) {
        match transcript::load(self.youtube, &self.config.transcript, video) {
            Ok(found) if found.segments.is_empty() && !found.note.is_empty() => {
                (Vec::new(), vec![sentence(NO_TRANSCRIPT, &found.note)])
            }
            Ok(found) => (found.segments, Vec::new()),
            Err(fail) => (Vec::new(), vec![sentence(NO_TRANSCRIPT, &fail.message)]),
        }
    }

    fn quiet(&self, segments: &[Segment], span: &Span) -> Vec<(f64, f64)> {
        if segments.is_empty() {
            return Vec::new();
        }
        let end = span.end.unwrap_or_else(|| segments.iter().map(|part| part.end).fold(span.start, f64::max));
        prompt::quiet_spans(segments, span.start, end, &self.config.analysis)
    }

    fn key(&self, job: &Job, plan: &Plan, model: &str) -> AnalysisKey {
        AnalysisKey {
            video_id: job.video.id.clone(),
            section: job.seen.map_or_else(|| store::WHOLE_VIDEO.to_string(), |seen| seen.label()),
            mode: job.mode.name().to_string(),
            focus: store::normalize_focus(job.focus.unwrap_or_default()),
            prompt_version: prompt::fingerprint(job.mode, &self.config.analysis),
            model: model.to_string(),
            profile: plan.tag.clone(),
        }
    }

    fn kept(&self, job: &Job, plan: &Plan) -> Result<Option<Answer>, Fail> {
        for model in &self.config.gemini.model_ladder {
            let Some(found) = self.youtube.store.analysis(&self.key(job, plan, model))? else { continue };
            if let Ok(analysis) = analysis::read(&found.payload, job.mode, 0.0, false) {
                return Ok(Some(Answer { analysis, model: model.clone(), requests: 0, notes: Vec::new() }));
            }
        }
        Ok(None)
    }

    fn send(&self, job: &Job, plan: &Plan, prompt: &str, guard: &Guard) -> Result<Reply, Fail> {
        let transport = (self.connect)()?;
        let clip = Clip {
            url: &job.video.url,
            resolution: &plan.resolution,
            sampling: sampling(plan, &job.span),
            mime_type: None,
        };
        let form = schema::answer(job.mode, &self.config.analysis);
        let output = u64::try_from(plan.output).unwrap_or_default();
        let body = bodies::request(Some(bodies::video_part(&clip)), prompt, output, Some(&form));
        let tokens = plan.video_tokens + planner::text_tokens(prompt, &self.config.tokens) + plan.output;
        let youtube_seconds = job.span.end.map_or(0.0, |end| (end - job.span.start).max(0.0));
        let estimate = Ask { tokens, youtube_seconds, ..Ask::default() };
        let gemini = Gemini {
            settings: &self.config.gemini,
            guard,
            clock: self.youtube.clock,
            transport: transport.as_ref(),
            jitter: self.jitter,
        };
        gemini.send(&Request { body: &body, estimate, models: &self.config.gemini.model_ladder })
    }

    fn asked(&self, job: &Job, plan: &Plan, prompt: &str) -> Result<Answer, Fail> {
        let store = self.youtube.store;
        let guard = Guard::new(store, &self.config.limits, self.youtube.clock);
        let reply = self.send(job, plan, prompt, &guard)?;
        let analysis = analysis::parse(&reply.text, job.mode, 0.0, !reply.complete)?;
        let mut notes = handed_over(&reply);
        let key = self.key(job, plan, &reply.model);
        let usage = Value::Object(reply.usage.clone());
        let saved = store.put_analysis(&key, &analysis::data(&analysis), &usage, self.youtube.clock.now());
        if let Err(fail) = saved.and_then(|()| guard.prune()) {
            notes.push(sentence(NOT_KEPT, &fail.message));
        }
        if let Ok(today) = guard.usage_today(&reply.model) {
            let (used, allowed) = (today.used, today.allowed);
            notes.push(format!("Today {used} of {allowed} requests on {} are used.", reply.model));
        }
        Ok(Answer { analysis, model: reply.model, requests: reply.requests as usize, notes })
    }

    fn answer(&self, job: &Job) -> Result<(Answer, Vec<String>), Fail> {
        let (segments, mut notes) = self.speech(job.video);
        let quiet = self.quiet(&segments, &job.span);
        let facts = Facts {
            title: &job.info.title,
            channel: &job.info.channel,
            duration: job.info.duration,
            chapters: &job.info.chapters,
            description: &job.info.description,
            window: job.seen,
            quiet: &quiet,
            focus: job.focus,
        };
        let prompt = prompt::build(job.mode, &facts, &self.config.analysis);
        let prompt_tokens = planner::text_tokens(&prompt, &self.config.tokens);
        let plan = planner::plan(job.mode, &job.span, prompt_tokens, self.config)?;
        notes.extend(planner::notes(&plan, job.mode, &self.config.watch));
        let answer = match self.kept(job, &plan)? {
            Some(answer) => answer,
            None => self.asked(job, &plan, &prompt)?,
        };
        Ok((answer, notes))
    }
}

pub fn run(watcher: &Watcher, args: &[String]) -> Result<String, Fail> {
    let asked = Asked::read(VERB, USAGE, args, &FLAGS)?;
    let mode = asked.flag(MODE).map_or(Ok(STANDARD), Mode::parse)?;
    let focus = prompt::clean_focus(asked.flag(FOCUS), &watcher.config.analysis)?;
    let page = page_number(asked.flag(PAGE))?;
    let video = asked.video()?;
    timecode::range(asked.flag(START), asked.flag(END), None)?;
    let info = watcher.youtube.info(&video)?;
    transcript::check_finished(&info)?;
    let window = timecode::range(asked.flag(START), asked.flag(END), info.duration)?;
    let start = window.map_or(0.0, |window| window.start);
    let end = window.and_then(|window| window.end).or(info.duration);
    let job = Job {
        video: &video,
        info: &info,
        mode,
        focus: focus.as_deref(),
        seen: window.map(|_| Range { start, end }),
        span: Span { start, end, whole: window.is_none() },
    };
    let (answer, mut notes) = watcher.answer(&job)?;
    notes.extend(answer.notes);
    let title = if info.title.trim().is_empty() { video.url.as_str() } else { info.title.as_str() };
    let heading = Heading {
        title,
        mode,
        model: &answer.model,
        requests: answer.requests,
        duration: info.duration,
        window: job.seen,
    };
    render::page(&answer.analysis, &heading, page, &watcher.config.render, &notes)
}

#[cfg(test)]
#[path = "watch_tests.rs"]
mod tests;
