use std::collections::HashMap;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::config::{Config, Talk};
use crate::door;
use crate::gate::{Gate, Route, fresh_key};
use crate::http::{self, Request};
use crate::job::Failure;
use crate::load::Temp;
use crate::media::{self, Kind};
use crate::post::{MB, Upload};
use crate::server::{self, Files};
use crate::session::Session;
use crate::spawn;
use crate::talk::{Options, note, refuse};
use crate::watch;
use crate::wire::{deny, escape, give};

type Asset = (&'static str, &'static str);

const SHELL: Asset = ("page.html", include_str!("../talk/page.html"));
const LOGIC: Asset = ("page.js", include_str!("../talk/page.js"));
const MARK: Asset = ("mark.js", include_str!("../talk/mark.js"));
const LINK: Asset = ("wire.js", include_str!("../talk/wire.js"));
const SETUP: Asset = ("page.json", include_str!("../talk/page.json"));
const POLICY: (&str, &str) = (
    "Content-Security-Policy",
    "default-src 'self'; img-src 'self' data: blob:; style-src 'unsafe-inline'; connect-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'",
);
const LOCKED: &str = "{locked}";

#[derive(Serialize, Deserialize)]
struct Tab {
    pid: u32,
    url: String,
}

pub fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|broken| broken.into_inner())
}

fn state_path(talk: &Talk, temp: &Temp) -> Option<PathBuf> {
    let name = Path::new(&talk.page_state);
    if talk.page_state.is_empty() {
        None
    } else if name.is_absolute() {
        Some(name.to_path_buf())
    } else {
        temp.root().map(|root| root.join(name))
    }
}

fn running(path: &Path) -> Option<String> {
    let raw = std::fs::read_to_string(path).ok()?;
    let tab: Tab = serde_json::from_str(&raw).ok()?;
    let exe = std::env::current_exe().ok()?;
    server::is_process(tab.pid, &exe).then_some(tab.url)
}

fn mark(path: &Path, url: &str) {
    let tab = Tab { pid: std::process::id(), url: url.to_string() };
    let _ = path.parent().map(std::fs::create_dir_all);
    let _ = std::fs::write(path, serde_json::to_vec(&tab).unwrap_or_default());
}

fn open(talk: &Talk, line: &str, url: &str) {
    note(&line.replace("{url}", url));
    let args = media::fill(&talk.page_open, &[("url", url)]);
    if let Some((program, rest)) = args.split_first() {
        let _ = spawn::detached(program)
            .args(rest)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
    }
}

pub struct Site<'a> {
    pub config: &'a Config,
    pub config_path: &'a Path,
    pub temp: &'a Temp,
    pub options: &'a Options,
    pub sessions: Mutex<Vec<(String, Session<'a>)>>,
    pub uploads: Mutex<HashMap<String, Upload>>,
    pub count: AtomicU64,
    pub writing: AtomicBool,
    gate: Gate,
    model: String,
    seen: Mutex<Instant>,
}

impl<'a> Site<'a> {
    pub fn fail(&self, code: &str, fields: &[(&str, &str)]) -> Failure {
        Failure::new(self.config, code, fields)
    }

    pub fn touch(&self) {
        *lock(&self.seen) = Instant::now();
    }

    fn idle(&self) -> bool {
        let limit = self.config.talk.page_idle_seconds;
        limit > 0.0 && !self.writing.load(Ordering::Acquire) && lock(&self.seen).elapsed().as_secs_f64() > limit
    }

    fn asset(&self, (name, built): Asset) -> String {
        let dir = &self.config.talk.page_dir;
        let own = (!dir.as_os_str().is_empty()).then(|| std::fs::read_to_string(dir.join(name)).ok()).flatten();
        own.unwrap_or_else(|| built.to_string())
    }

    fn setup(&self) -> Value {
        let talk = &self.config.talk;
        let parsed = |raw: &str| serde_json::from_str::<Value>(raw).ok().filter(Value::is_object);
        let mut page = parsed(&self.asset(SETUP)).or_else(|| parsed(SETUP.1)).unwrap_or_else(|| json!({}));
        let kinds: Map<String, Value> = [
            (Kind::Picture, &talk.pictures),
            (Kind::Video, &talk.videos),
            (Kind::Sound, &talk.sounds),
            (Kind::Text, &talk.texts),
            (Kind::Page, &talk.pages),
        ]
        .into_iter()
        .filter_map(|(kind, list)| {
            let name = serde_json::to_value(kind).ok()?.as_str()?.to_string();
            Some((name, json!(list)))
        })
        .collect();
        page["model"] = json!(self.model);
        page["upload_max_bytes"] = json!(talk.upload_max_mb.saturating_mul(MB));
        page["history"] = json!(talk.page_history);
        page["links"] = json!(talk.links);
        page["kinds"] = Value::Object(kinds);
        page
    }

    fn state(&self) -> Value {
        let config = self.config;
        let game = watch::game_failure(config, self.options.afk).map(|failure| failure.message);
        json!({
            "up": server::healthy(config),
            "model": self.model,
            "held": door::held(&config.talk),
            "busy": self.writing.load(Ordering::Acquire),
            "jobs": server::others_busy(&Files::new(config)),
            "game": game,
            "test": door::test(&config.talk),
        })
    }

    fn refused(&self, status: u16, request: &Request) -> Failure {
        let detail = format!("{} {}", request.method, request.path);
        let code = match status {
            http::FORBIDDEN | http::LOCKED => "talk_page_key",
            _ => "talk_page_request",
        };
        self.fail(code, &[("detail", detail.as_str())])
    }

    fn page(&self, route: Route, wire: &mut &TcpStream) {
        let (kind, extra, body) = match route {
            Route::Shell => {
                let locked = escape(&self.config.talk.text.page_locked);
                let html = self.asset(SHELL).replace(LOCKED, &locked);
                (http::HTML, vec![POLICY], html)
            }
            Route::Mark => (http::SCRIPT, Vec::new(), self.asset(MARK)),
            Route::Link => (http::SCRIPT, Vec::new(), self.asset(LINK)),
            _ => (http::SCRIPT, Vec::new(), self.asset(LOGIC)),
        };
        let _ = http::respond(wire, http::OK, kind, &extra, body.as_bytes());
    }

    fn handle(&self, stream: TcpStream) {
        let talk = &self.config.talk;
        let wait = (talk.page_read_seconds > 0).then(|| Duration::from_secs(talk.page_read_seconds));
        let ready = stream.set_nonblocking(false).is_ok()
            && stream.set_read_timeout(wait).is_ok()
            && stream.set_write_timeout(wait).is_ok();
        let _ = stream.set_nodelay(true);
        let Some(read) = stream.try_clone().ok().filter(|_| ready) else {
            return;
        };
        let mut reader = BufReader::new(read);
        let mut wire = &stream;
        let head = http::read_head(&mut reader, talk.page_head_bytes);
        let Some(request) = head.as_deref().and_then(http::parse_head) else {
            let detail = http::BAD.to_string();
            let failure = self.fail("talk_page_request", &[("detail", detail.as_str())]);
            return deny(&mut wire, http::BAD, &failure);
        };
        let route = match self.gate.admit(&request) {
            Ok(route) => route,
            Err(status) => return deny(&mut wire, status, &self.refused(status, &request)),
        };
        if !route.open() {
            self.touch();
        }
        match route {
            Route::Shell | Route::Logic | Route::Mark | Route::Link => self.page(route, &mut wire),
            Route::Setup => give(&mut wire, &self.setup()),
            Route::State => give(&mut wire, &self.state()),
            Route::Say => self.say(&request, &mut reader, &stream),
            Route::File | Route::Forget | Route::Hear => {
                let done = match route {
                    Route::File => self.file(&request, &mut reader),
                    Route::Hear => self.hear(&request, &mut reader),
                    _ => self.forget(&request, &mut reader),
                };
                match done {
                    Ok(value) => give(&mut wire, &value),
                    Err((status, failure)) => deny(&mut wire, status, &failure),
                }
            }
        }
    }
}

fn bind(talk: &Talk) -> std::io::Result<TcpListener> {
    let listener = TcpListener::bind((talk.page_host.as_str(), talk.page_port))?;
    if !listener.local_addr()?.ip().is_loopback() {
        return Err(std::io::ErrorKind::AddrNotAvailable.into());
    }
    listener.set_nonblocking(true)?;
    Ok(listener)
}

fn taken(config: &Config, options: &Options, state: Option<&Path>, error: &std::io::Error) -> i32 {
    let talk = &config.talk;
    if let Some(url) = state.and_then(running) {
        open(talk, &talk.text.page_again, &url);
        return 0;
    }
    let (port, detail) = (talk.page_port.to_string(), error.to_string());
    let fields = [("port", port.as_str()), ("detail", detail.as_str())];
    refuse(config, options.json, &Failure::new(config, "talk_page_port", &fields));
    1
}

fn listen(site: &Site, listener: &TcpListener) {
    let poll = Duration::from_millis(site.config.talk.page_poll_ms.max(1));
    std::thread::scope(|scope| {
        while !site.idle() {
            match listener.accept() {
                Ok((stream, _)) => {
                    scope.spawn(move || site.handle(stream));
                }
                Err(_) => std::thread::sleep(poll),
            }
        }
    });
}

pub fn serve(config: &Config, config_path: &Path, temp: &Temp, options: &Options) -> i32 {
    let talk = &config.talk;
    let state = state_path(talk, temp);
    let listener = match bind(talk) {
        Ok(listener) => listener,
        Err(error) => return taken(config, options, state.as_deref(), &error),
    };
    let model = config.model(options.model.as_deref());
    let site = Site {
        config,
        config_path,
        temp,
        options,
        sessions: Mutex::new(Vec::new()),
        uploads: Mutex::new(HashMap::new()),
        count: AtomicU64::new(0),
        writing: AtomicBool::new(false),
        gate: Gate::new(talk, fresh_key()),
        model: model.map(|model| model.name.clone()).unwrap_or_default(),
        seen: Mutex::new(Instant::now()),
    };
    let url = site.gate.url();
    if let Some(path) = &state {
        mark(path, &url);
    }
    open(talk, &talk.text.page_ready, &url);
    listen(&site, &listener);
    if let Some(path) = &state {
        let _ = std::fs::remove_file(path);
    }
    0
}
