use std::sync::LazyLock;
use std::time::Duration;

use ureq::tls::{RootCerts, TlsConfig};

pub const ANTHROPIC: &str = "https://api.anthropic.com";
const CONNECT: Duration = Duration::from_secs(15);
const FIRST_BYTE: Duration = Duration::from_secs(600);
const SKIP_REQUEST: &[&str] = &[
    "host",
    "content-length",
    "connection",
    "accept-encoding",
    "transfer-encoding",
    "proxy-connection",
    "keep-alive",
    "expect",
];
const SKIP_REPLY: &[&str] = &["content-length", "connection", "transfer-encoding", "content-encoding", "keep-alive"];

pub type Reply = ureq::http::Response<ureq::Body>;

static AGENT: LazyLock<ureq::Agent> = LazyLock::new(|| {
    ureq::Agent::config_builder()
        .timeout_connect(Some(CONNECT))
        .timeout_recv_response(Some(FIRST_BYTE))
        .http_status_as_error(false)
        .max_redirects(0)
        .tls_config(TlsConfig::builder().root_certs(RootCerts::PlatformVerifier).build())
        .build()
        .into()
});

pub fn agent() -> &'static ureq::Agent {
    &AGENT
}

pub fn forwarded(headers: &[(String, String)]) -> Vec<(&str, &str)> {
    headers
        .iter()
        .filter(|(k, _)| !SKIP_REQUEST.contains(&k.to_lowercase().as_str()))
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect()
}

pub fn reply_headers(resp: &Reply) -> Vec<(String, String)> {
    resp.headers()
        .iter()
        .filter(|(k, _)| !SKIP_REPLY.contains(&k.as_str()))
        .filter_map(|(k, v)| Some((k.as_str().to_string(), v.to_str().ok()?.to_string())))
        .collect()
}

pub fn send(base: &str, method: &str, path: &str, headers: &[(String, String)], body: &[u8]) -> Result<Reply, String> {
    let mut req = ureq::http::Request::builder().method(method).uri(format!("{}{path}", base.trim_end_matches('/')));
    for (k, v) in forwarded(headers) {
        req = req.header(k, v);
    }
    req = req.header("accept-encoding", "identity");
    let sent = if body.is_empty() { req.body(()).map(|r| AGENT.run(r)) } else { req.body(body).map(|r| AGENT.run(r)) };
    sent.map_err(|e| e.to_string())?.map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hop_by_hop_and_length_headers_are_not_forwarded_but_auth_is() {
        let headers: Vec<(String, String)> = [
            ("Host", "127.0.0.1:9947"),
            ("Content-Length", "10"),
            ("Accept-Encoding", "gzip"),
            ("authorization", "Bearer x"),
            ("anthropic-version", "2023-06-01"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
        let kept: Vec<&str> = forwarded(&headers).iter().map(|(k, _)| *k).collect();
        assert_eq!(kept, ["authorization", "anthropic-version"]);
    }
}
