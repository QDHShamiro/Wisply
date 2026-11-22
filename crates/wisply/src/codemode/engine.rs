use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::{Duration, Instant};

use rquickjs::context::EvalOptions;
use rquickjs::promise::PromiseState;
use rquickjs::{Context, Ctx, Function, Object, Promise, Runtime, Value};
use serde_json::Value as Json;

const PRELUDE: &str = include_str!("prelude.js");
const FILE: &str = "code.js";
const MEMORY: usize = 256 << 20;
const STACK: usize = 8 << 20;
const PRINT_MAX: usize = 400_000;
const TICK: Duration = Duration::from_millis(100);
const NEEDS_FUNCTION: &str = "return not in a function";

pub struct Done {
    pub id: i32,
    pub ok: bool,
    pub text: String,
}

pub enum Step {
    Going,
    Now(bool, String),
    Halt(String),
}

pub trait Host {
    fn start(&self, id: i32, kind: &str, payload: &str) -> Step;
    fn facts(&self) -> String;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Thrown {
    pub name: String,
    pub message: String,
    pub line: Option<usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Verdict {
    Value(Option<String>),
    Threw(Thrown),
    Halted(String),
    TimedOut,
    Stuck,
    Broken(String),
}

pub struct Ran {
    pub verdict: Verdict,
    pub prints: String,
    pub lost: Vec<String>,
    pub started: usize,
    pub finished: usize,
}

#[derive(Default)]
struct State {
    prints: String,
    ready: Vec<Done>,
    timers: Vec<(Instant, i32)>,
    halt: Option<String>,
    lost: Vec<String>,
    started: usize,
    finished: usize,
    waiting: usize,
}

struct Api<'js> {
    settle: Function<'js>,
    finish: Function<'js>,
    explain: Function<'js>,
    lost: Function<'js>,
}

struct Loop<'a, 'js> {
    ctx: &'a Ctx<'js>,
    api: &'a Api<'js>,
    rx: &'a Receiver<Done>,
    deadline: Instant,
    state: &'a Rc<RefCell<State>>,
}

pub fn run(source: &str, host: Rc<dyn Host>, rx: &Receiver<Done>, deadline: Instant) -> Ran {
    let state = Rc::new(RefCell::new(State::default()));
    let halted = Rc::new(Cell::new(false));
    let verdict = match boot(deadline, &halted) {
        Ok((_rt, context)) => context.with(|ctx| drive(&ctx, source, &host, rx, deadline, &state, &halted)),
        Err(why) => Verdict::Broken(why),
    };
    let mut st = state.borrow_mut();
    Ran {
        verdict,
        prints: std::mem::take(&mut st.prints),
        lost: std::mem::take(&mut st.lost),
        started: st.started,
        finished: st.finished,
    }
}

fn boot(deadline: Instant, halted: &Rc<Cell<bool>>) -> Result<(Runtime, Context), String> {
    let rt = Runtime::new().map_err(|e| format!("the script engine did not start ({e})"))?;
    rt.set_memory_limit(MEMORY);
    rt.set_max_stack_size(STACK);
    let halted = halted.clone();
    rt.set_interrupt_handler(Some(Box::new(move || halted.get() || Instant::now() >= deadline)));
    let context = Context::full(&rt).map_err(|e| format!("the script engine did not start ({e})"))?;
    Ok((rt, context))
}

fn options(file: &str, promise: bool) -> EvalOptions {
    let mut options = EvalOptions::default();
    options.strict = false;
    options.promise = promise;
    options.filename = Some(file.to_string());
    options
}

fn install<'js>(
    ctx: &Ctx<'js>,
    host: &Rc<dyn Host>,
    state: &Rc<RefCell<State>>,
    halted: &Rc<Cell<bool>>,
) -> rquickjs::Result<Api<'js>> {
    let make: Function = ctx.eval_with_options(PRELUDE, options("prelude.js", false))?;
    let start = {
        let (host, state, halted) = (host.clone(), state.clone(), halted.clone());
        Function::new(ctx.clone(), move |id: i32, kind: String, payload: String| {
            let mut st = state.borrow_mut();
            if st.halt.is_some() || halted.get() {
                return;
            }
            st.waiting += 1;
            if kind == "sleep" {
                let ms = payload.parse::<f64>().unwrap_or(0.0).clamp(0.0, 86_400_000.0);
                st.timers.push((Instant::now() + Duration::from_millis(ms as u64), id));
                return;
            }
            st.started += 1;
            match host.start(id, &kind, &payload) {
                Step::Going => {}
                Step::Now(ok, text) => st.ready.push(Done { id, ok, text }),
                Step::Halt(text) => {
                    st.started -= 1;
                    st.waiting -= 1;
                    st.halt = Some(text);
                    halted.set(true);
                }
            }
        })?
    };
    let print = {
        let state = state.clone();
        Function::new(ctx.clone(), move |text: String| {
            let mut st = state.borrow_mut();
            if st.prints.len() < PRINT_MAX {
                st.prints.push_str(&text);
                st.prints.push('\n');
            }
        })?
    };
    let api: Object = make.call((start, print, host.facts()))?;
    Ok(Api {
        settle: api.get("settle")?,
        finish: api.get("finish")?,
        explain: api.get("explain")?,
        lost: api.get("lost")?,
    })
}

fn drive<'js>(
    ctx: &Ctx<'js>,
    source: &str,
    host: &Rc<dyn Host>,
    rx: &Receiver<Done>,
    deadline: Instant,
    state: &Rc<RefCell<State>>,
    halted: &Rc<Cell<bool>>,
) -> Verdict {
    let api = match install(ctx, host, state, halted) {
        Ok(api) => api,
        Err(e) => return Verdict::Broken(format!("the script engine did not start ({e})")),
    };
    let turn = Loop { ctx, api: &api, rx, deadline, state };
    let promise = match begin(&turn, source) {
        Ok(promise) => promise,
        Err(verdict) => return verdict,
    };
    let done: Promise = match api.finish.call((promise, true)) {
        Ok(done) => done,
        Err(_) => return Verdict::Threw(explain(ctx, &api)),
    };
    if let Err(verdict) = turn.wait(&done, false) {
        return verdict;
    }
    let verdict = match done.result::<String>() {
        Some(Ok(envelope)) => read(&envelope),
        _ => Verdict::Threw(explain(ctx, &api)),
    };
    if matches!(verdict, Verdict::Value(_))
        && let Err(late) = turn.wait(&done, true)
    {
        return late;
    }
    let lost = api.lost.call::<_, String>(()).unwrap_or_default();
    state.borrow_mut().lost = lost.lines().map(str::to_string).collect();
    verdict
}

fn begin<'a, 'js>(turn: &Loop<'a, 'js>, source: &str) -> Result<Promise<'js>, Verdict> {
    let first = turn.ctx.eval_with_options::<Promise, _>(source, options(FILE, true));
    let thrown = match first {
        Ok(promise) => return Ok(promise),
        Err(_) => explain(turn.ctx, turn.api),
    };
    if let Some(verdict) = turn.stopped() {
        return Err(verdict);
    }
    if thrown.name != "SyntaxError" || !thrown.message.contains(NEEDS_FUNCTION) {
        return Err(Verdict::Threw(thrown));
    }
    let wrapped = format!("(async () => {{ {source}\n}})()");
    match turn.ctx.eval_with_options::<Promise, _>(wrapped, options(FILE, true)) {
        Ok(promise) => Ok(promise),
        Err(_) => {
            let thrown = explain(turn.ctx, turn.api);
            Err(turn.stopped().unwrap_or(Verdict::Threw(thrown)))
        }
    }
}

fn explain<'js>(ctx: &Ctx<'js>, api: &Api<'js>) -> Thrown {
    let value: Value = ctx.catch();
    let text = api.explain.call::<_, String>((value,)).unwrap_or_default();
    let _ = ctx.catch();
    match read(&text) {
        Verdict::Threw(thrown) => thrown,
        _ => Thrown { name: "Error".into(), message: "the code failed and gave no reason".into(), line: None },
    }
}

fn read(envelope: &str) -> Verdict {
    let Ok(v) = serde_json::from_str::<Json>(envelope) else {
        return Verdict::Broken("the script engine gave an unreadable result".into());
    };
    let text = |key: &str| v[key].as_str().unwrap_or_default().to_string();
    if v["ok"].as_bool() == Some(true) {
        return Verdict::Value((v["none"].as_bool() != Some(true)).then(|| text("text")));
    }
    Verdict::Threw(Thrown { name: text("name"), message: text("message"), line: line_of(&text("stack")) })
}

fn line_of(stack: &str) -> Option<usize> {
    let mark = format!("{FILE}:");
    let rest = &stack[stack.find(&mark)? + mark.len()..];
    let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
    digits.parse().ok()
}

impl<'js> Loop<'_, 'js> {
    fn stopped(&self) -> Option<Verdict> {
        if let Some(text) = self.state.borrow_mut().halt.take() {
            return Some(Verdict::Halted(text));
        }
        (Instant::now() >= self.deadline).then_some(Verdict::TimedOut)
    }

    fn due(&self) -> (Vec<Done>, Vec<i32>, usize, usize, Option<Instant>) {
        let mut st = self.state.borrow_mut();
        let now = Instant::now();
        let ready = std::mem::take(&mut st.ready);
        let (due, later): (Vec<_>, Vec<_>) = st.timers.drain(..).partition(|(at, _)| *at <= now);
        st.timers = later;
        let next = st.timers.iter().map(|(at, _)| *at).min();
        (ready, due.into_iter().map(|(_, id)| id).collect(), st.waiting, st.timers.len(), next)
    }

    fn settle(&self, id: i32, ok: bool, text: String, counted: bool) {
        {
            let mut st = self.state.borrow_mut();
            st.waiting = st.waiting.saturating_sub(1);
            if counted {
                st.finished += 1;
            }
        }
        if self.api.settle.call::<_, ()>((id, ok, text)).is_err() {
            let _ = self.ctx.catch();
        }
    }

    fn wait(&self, promise: &Promise<'js>, all: bool) -> Result<(), Verdict> {
        loop {
            while self.ctx.execute_pending_job() {}
            if let Some(verdict) = self.stopped() {
                return Err(verdict);
            }
            let (ready, timers, waiting, sleeping, next) = self.due();
            if !ready.is_empty() || !timers.is_empty() {
                for done in ready {
                    self.settle(done.id, done.ok, done.text, true);
                }
                for id in timers {
                    self.settle(id, true, String::new(), false);
                }
                continue;
            }
            let pending = matches!(promise.state(), PromiseState::Pending);
            if !pending && (!all || waiting <= sleeping) {
                return Ok(());
            }
            if pending && waiting == 0 {
                return Err(Verdict::Stuck);
            }
            let now = Instant::now();
            let until = next.unwrap_or(now + TICK).min(now + TICK).min(self.deadline);
            match self.rx.recv_timeout(until.saturating_duration_since(now)) {
                Ok(done) => self.state.borrow_mut().ready.push(done),
                Err(RecvTimeoutError::Timeout) => {}
                Err(RecvTimeoutError::Disconnected) if waiting > self.state.borrow().timers.len() => {
                    return Err(Verdict::Broken("the workers of code mode stopped".into()));
                }
                Err(RecvTimeoutError::Disconnected) => std::thread::sleep(until.saturating_duration_since(now)),
            }
        }
    }
}
