use std::io::Read;
use std::thread;
use std::time::{Duration, Instant};

pub const GAP: Duration = Duration::from_secs(1);
const WAIT: Duration = Duration::from_secs(12);
const BODY: u64 = 262_144;
const AGENT: &str = "wisply-secure/1 (passive check of your own site)";

pub struct Resp {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Resp {
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
    }

    pub fn all(&self, name: &str) -> Vec<&str> {
        self.headers.iter().filter(|(k, _)| k == name).map(|(_, v)| v.as_str()).collect()
    }

    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }
}

pub struct Client {
    agent: ureq::Agent,
    last: Option<Instant>,
    pub sent: usize,
}

impl Client {
    pub fn new() -> Client {
        let agent = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .max_redirects(0)
            .max_redirects_will_error(false)
            .user_agent(AGENT)
            .timeout_global(Some(WAIT))
            .build()
            .into();
        Client { agent, last: None, sent: 0 }
    }

    pub fn get(&mut self, url: &str) -> Result<Resp, String> {
        if let Some(t) = self.last {
            let since = t.elapsed();
            if since < GAP {
                thread::sleep(GAP - since);
            }
        }
        self.last = Some(Instant::now());
        self.sent += 1;
        let mut resp = self.agent.get(url).call().map_err(|e| e.to_string())?;
        let status = resp.status().as_u16();
        let headers = resp
            .headers()
            .iter()
            .map(|(k, v)| (k.as_str().to_lowercase(), String::from_utf8_lossy(v.as_bytes()).into_owned()))
            .collect();
        let mut body = Vec::new();
        let _ = resp.body_mut().as_reader().take(BODY).read_to_end(&mut body);
        Ok(Resp { status, headers, body })
    }
}
