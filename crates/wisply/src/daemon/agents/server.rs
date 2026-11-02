use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

use serde_json::{Value, json};
use wisply_core::Fail;

use super::api::{Api, Backend};
use super::auth::Auth;
use super::board;
use super::limit::Guard;
use super::model::Row;
use super::net::{Server, parse_binds, serve, tailscale_ips};
use crate::daemon::config;
use crate::daemon::pc;

const PORT: i64 = 47841;
const LOOPBACK: &str = "127.0.0.1";

static API: OnceLock<Arc<Api>> = OnceLock::new();
static RUNNING: Mutex<Option<Server>> = Mutex::new(None);

struct Live {
    home: std::path::PathBuf,
}

impl Backend for Live {
    fn rows(&self) -> Result<Vec<Row>, Fail> {
        board::snapshot(false)
    }

    fn tail(&self, row: &Row) -> Vec<String> {
        board::tail(row)
    }

    fn reply(&self, row: &Row, text: &str) -> Result<(), Fail> {
        board::resume(&row.sid, &row.dir, text)
    }

    fn stop(&self, row: &Row) -> Result<(), Fail> {
        board::stop(&row.id)
    }

    fn status(&self) -> Value {
        let facts = pc::facts(&self.home);
        let guard = Guard::load(&self.home);
        let reset = guard.waiting.iter().filter(|w| !w.resumed).map(|_| guard.reset_at).max().filter(|a| *a > 0);
        json!({
            "version": env!("CARGO_PKG_VERSION"),
            "pc": {"free_gb_c": facts.free_gb, "free_ram_gb": facts.free_ram_gb, "game_mode": facts.playing},
            "build": super::extra::build_owner(&self.home),
            "limit": {"hit": guard.hit(), "reset_at": reset},
        })
    }
}

fn api(home: &Path) -> Arc<Api> {
    Arc::clone(API.get_or_init(|| {
        Arc::new(Api {
            home: home.to_path_buf(),
            auth: Auth::new(home),
            backend: Box::new(Live { home: home.to_path_buf() }),
        })
    }))
}

pub fn shared(home: &Path) -> Arc<Api> {
    api(home)
}

fn port(home: &Path) -> u16 {
    let cfg = config::load(home);
    cfg.get("phone_port").and_then(Value::as_i64).filter(|p| (1..=65535).contains(p)).unwrap_or(PORT) as u16
}

fn addresses(home: &Path) -> (Vec<IpAddr>, Vec<String>) {
    let cfg = config::load(home);
    let (mut ips, refused) = parse_binds(&config::text(&cfg, "phone_bind"));
    if config::on(&cfg, "phone_tailscale") {
        for ip in tailscale_ips() {
            if !ips.contains(&ip) {
                ips.push(ip);
            }
        }
    }
    (ips, refused)
}

pub fn ensure(home: &Path) -> Result<Vec<String>, String> {
    let mut slot = RUNNING.lock().unwrap_or_else(PoisonError::into_inner);
    if let Some(s) = slot.as_ref() {
        return Ok(s.addrs.iter().map(ToString::to_string).collect());
    }
    let (ips, _) = addresses(home);
    let server = serve(api(home), &ips, port(home)).map_err(|e| format!("phone API could not listen ({e})"))?;
    let list = server.addrs.iter().map(ToString::to_string).collect();
    *slot = Some(server);
    Ok(list)
}

pub fn stop() {
    if let Some(s) = RUNNING.lock().unwrap_or_else(PoisonError::into_inner).take() {
        s.stop();
    }
}

pub fn start(home: &Path) {
    if config::on(&config::load(home), "phone_api") {
        let _ = ensure(home);
    }
}

pub fn pair_url(addrs: &[String]) -> String {
    let parsed: Vec<SocketAddr> = addrs.iter().filter_map(|a| a.parse().ok()).collect();
    let pick = parsed.iter().find(|a| !a.ip().is_loopback()).or_else(|| parsed.first());
    pick.map_or_else(
        || format!("http://{LOOPBACK}:{PORT}"),
        |a| match a.ip() {
            IpAddr::V6(ip) => format!("http://[{ip}]:{}", a.port()),
            IpAddr::V4(ip) => format!("http://{ip}:{}", a.port()),
        },
    )
}

pub fn describe(home: &Path) -> String {
    let cfg = config::load(home);
    let on = config::on(&cfg, "phone_api");
    let (_, refused) = addresses(home);
    let live = RUNNING.lock().unwrap_or_else(PoisonError::into_inner);
    let mut out = vec![match (on, live.as_ref()) {
        (_, Some(s)) => format!(
            "phone API on, listening on {}",
            s.addrs.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
        ),
        (true, None) => "phone API on in config but not listening (see the daemon log)".to_string(),
        (false, None) => "phone API off: wisply mobile pair".to_string(),
    }];
    let paired = api(home).auth.list();
    out.push(format!("{} phone(s) paired", paired.len()));
    if !refused.is_empty() {
        out.push(format!("refused phone_bind: {} (only localhost and Tailscale are allowed)", refused.join(", ")));
    }
    out.join("\n")
}
