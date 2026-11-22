use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Digest, Sha256};

use crate::config::Talk;
use crate::http::{self, Request};

const KEY: &str = "x-pupil-key";
const SALTS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Route {
    Shell,
    Logic,
    Mark,
    Link,
    Setup,
    State,
    File,
    Say,
    Forget,
    Hear,
}

const ROUTES: [(&str, &str, Route); 10] = [
    ("/", "GET", Route::Shell),
    ("/page.js", "GET", Route::Logic),
    ("/mark.js", "GET", Route::Mark),
    ("/wire.js", "GET", Route::Link),
    ("/config", "GET", Route::Setup),
    ("/state", "GET", Route::State),
    ("/file", "POST", Route::File),
    ("/say", "POST", Route::Say),
    ("/forget", "POST", Route::Forget),
    ("/hear", "POST", Route::Hear),
];

impl Route {
    pub fn open(self) -> bool {
        matches!(self, Route::Shell | Route::Logic | Route::Mark | Route::Link)
    }
}

pub struct Gate {
    host: String,
    origin: String,
    key: String,
}

fn same(given: &str, key: &str) -> bool {
    given.len() == key.len() && given.bytes().zip(key.bytes()).fold(0, |all, (a, b)| all | (a ^ b)) == 0
}

impl Gate {
    pub fn new(talk: &Talk, key: String) -> Gate {
        let host = format!("{}:{}", talk.page_host, talk.page_port);
        Gate { origin: format!("http://{host}"), host, key }
    }

    pub fn url(&self) -> String {
        format!("{}/?k={}", self.origin, self.key)
    }

    fn local(&self, request: &Request) -> bool {
        request.header("host") == Some(self.host.as_str())
            && request.header("origin").is_none_or(|origin| origin == self.origin)
    }

    fn keyed(&self, request: &Request) -> bool {
        !self.key.is_empty() && request.header(KEY).is_some_and(|given| same(given, &self.key))
    }

    pub fn admit(&self, request: &Request) -> Result<Route, u16> {
        if !self.local(request) {
            return Err(http::FORBIDDEN);
        }
        let Some((_, method, route)) = ROUTES.iter().find(|known| known.0 == request.path) else {
            return Err(http::MISSING);
        };
        if request.method != *method {
            return Err(http::METHOD);
        }
        if !route.open() && !self.keyed(request) {
            return Err(http::LOCKED);
        }
        Ok(*route)
    }
}

pub fn fresh_key() -> String {
    let mut hash = Sha256::new();
    for _ in 0..SALTS {
        hash.update(RandomState::new().build_hasher().finish().to_le_bytes());
    }
    let clock = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |since| since.as_nanos());
    hash.update(clock.to_le_bytes());
    hash.update(std::process::id().to_le_bytes());
    hash.finalize().iter().map(|byte| format!("{byte:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::{Gate, Route, fresh_key, same};
    use crate::config::Config;
    use crate::http::{self, parse_head};

    fn config() -> Config {
        toml::from_str(include_str!("../pupil.toml")).expect("pupil.toml parses")
    }

    fn gate(key: &str) -> Gate {
        let mut talk = config().talk;
        talk.page_host = "127.0.0.1".to_string();
        talk.page_port = 9953;
        Gate::new(&talk, key.to_string())
    }

    fn asked(first: &str, lines: &[&str]) -> Result<Route, u16> {
        let head = format!("{first} HTTP/1.1\r\n{}", lines.join("\r\n"));
        gate("secret").admit(&parse_head(&head).expect("head"))
    }

    #[test]
    fn only_the_own_page_with_the_start_key_gets_in() {
        let host = "Host: 127.0.0.1:9953";
        let key = "X-Pupil-Key: secret";
        let own = "Origin: http://127.0.0.1:9953";
        assert_eq!(asked("GET /", &[host]), Ok(Route::Shell));
        assert_eq!(asked("GET /?k=secret", &[host]), Ok(Route::Shell));
        assert_eq!(asked("GET /page.js", &[host]), Ok(Route::Logic));
        assert_eq!(asked("GET /mark.js", &[host]), Ok(Route::Mark));
        assert_eq!(asked("GET /wire.js", &[host]), Ok(Route::Link));
        assert_eq!(asked("GET /config", &[host, key]), Ok(Route::Setup));
        assert_eq!(asked("GET /state", &[host, key]), Ok(Route::State));
        assert_eq!(asked("POST /say", &[host, key, own]), Ok(Route::Say));
        assert_eq!(asked("POST /forget", &[host, key, own]), Ok(Route::Forget));
        assert_eq!(asked("POST /hear?name=a.webm", &[host, key, own]), Ok(Route::Hear));
        assert!(asked("POST /hear?name=a.webm", &[host, own]).is_err());
        assert_eq!(asked("POST /file?name=a.png", &[host, key, own]), Ok(Route::File));
        assert_eq!(asked("GET /state", &[host]), Err(http::LOCKED));
        assert_eq!(asked("GET /state?k=secret", &[host]), Err(http::LOCKED));
        for wrong in ["secreT", "secret1", "secre", ""] {
            let line = format!("X-Pupil-Key: {wrong}");
            assert_eq!(asked("GET /state", &[host, &line]), Err(http::LOCKED));
        }
        assert_eq!(asked("GET /", &[]), Err(http::FORBIDDEN));
        for foreign in ["evil.example:9953", "127.0.0.1:9952", "127.0.0.1", "localhost:9953"] {
            let line = format!("Host: {foreign}");
            assert_eq!(asked("GET /", &[&line]), Err(http::FORBIDDEN));
            assert_eq!(asked("GET /state", &[&line, key]), Err(http::FORBIDDEN));
        }
        for origin in ["http://evil.example", "null", "https://127.0.0.1:9953"] {
            let line = format!("Origin: {origin}");
            assert_eq!(asked("POST /say", &[host, key, &line]), Err(http::FORBIDDEN));
        }
        assert_eq!(asked("OPTIONS /say", &[host, key, own]), Err(http::METHOD));
        assert_eq!(asked("GET /say", &[host, key]), Err(http::METHOD));
        assert_eq!(asked("POST /", &[host, own]), Err(http::METHOD));
        assert_eq!(asked("GET /pupil.toml", &[host, key]), Err(http::MISSING));
        assert_eq!(asked("GET /../pupil.toml", &[host, key]), Err(http::MISSING));
        let keyless = gate("");
        let head = format!("GET /state HTTP/1.1\r\n{host}\r\nX-Pupil-Key: ");
        assert_eq!(keyless.admit(&parse_head(&head).expect("head")), Err(http::LOCKED));
    }

    #[test]
    fn every_start_gets_its_own_long_key() {
        let (one, two) = (fresh_key(), fresh_key());
        assert_eq!(one.len(), 64);
        assert!(one.bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert_ne!(one, two);
        assert!(same(&one, &one.clone()));
        assert!(!same(&one, &two));
        assert!(!same(&one, &one[1..]));
        assert!(!same("", "x"));
        assert_eq!(gate(&one).url(), format!("http://127.0.0.1:9953/?k={one}"));
    }
}
