const PAGES: &[(&str, &str, &[u8])] = &[
    ("/", "text/html; charset=utf-8", include_bytes!("../../../../wisply-ui/web/dist/phone/phone.html")),
    ("/phone.js", "text/javascript; charset=utf-8", include_bytes!("../../../../wisply-ui/web/dist/phone/phone.js")),
    ("/phone.css", "text/css; charset=utf-8", include_bytes!("../../../../wisply-ui/web/dist/phone/phone.css")),
    ("/icon.svg", "image/svg+xml", include_bytes!("../../../../wisply-ui/web/dist/phone/icon.svg")),
];

pub const CSP: &str = "default-src 'none'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";

pub fn page(method: &str, target: &str) -> Option<(&'static str, &'static [u8])> {
    if method != "GET" && method != "HEAD" {
        return None;
    }
    let path = target.split(['?', '#']).next().unwrap_or("");
    PAGES.iter().find(|(p, ..)| *p == path).map(|(_, kind, body)| (*kind, *body))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_four_shell_files_are_public_and_never_data() {
        assert!(page("GET", "/").is_some_and(|(k, b)| k.starts_with("text/html") && !b.is_empty()));
        assert!(page("GET", "/phone.js?x=1").is_some());
        for (m, p) in
            [("POST", "/"), ("GET", "/v1/agents"), ("GET", "/v1/status"), ("GET", "/../x"), ("GET", "/app.js")]
        {
            assert!(page(m, p).is_none(), "{m} {p}");
        }
    }
}
