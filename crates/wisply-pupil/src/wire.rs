use std::io::Write;

use serde_json::{Value, json};

use crate::http;
use crate::job::Failure;
use crate::talk::Out;

pub fn escape(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

pub struct Wire<W: Write> {
    wire: W,
    open: bool,
}

impl<W: Write> Wire<W> {
    pub fn new(wire: W) -> Wire<W> {
        Wire { wire, open: true }
    }

    fn raw(&mut self, line: &str) -> bool {
        if self.open {
            let sent = self.wire.write_all(line.as_bytes());
            self.open = sent.and_then(|()| self.wire.flush()).is_ok();
        }
        self.open
    }

    pub fn line(&mut self, value: &Value) -> bool {
        self.raw(&format!("{value}\n"))
    }
}

impl<W: Write> Out for Wire<W> {
    fn note(&mut self, line: &str) {
        if !line.trim().is_empty() {
            self.line(&json!({"note": line}));
        }
    }

    fn word(&mut self, piece: &str) -> bool {
        self.line(&json!({"word": piece}))
    }

    fn thought(&mut self, piece: &str) -> bool {
        self.line(&json!({"thought": piece}))
    }

    fn gone(&mut self) -> bool {
        !self.raw("\n")
    }
}

pub fn give(wire: &mut impl Write, value: &Value) {
    let _ = http::respond(wire, http::OK, http::JSON, &[], value.to_string().as_bytes());
}

pub fn deny(wire: &mut impl Write, status: u16, failure: &Failure) {
    let body = serde_json::to_vec(failure).unwrap_or_default();
    let _ = http::respond(wire, status, http::JSON, &[], &body);
}

#[cfg(test)]
mod tests {
    use super::{Wire, escape};
    use crate::talk::Out;

    struct Shut;

    impl std::io::Write for Shut {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::ErrorKind::BrokenPipe.into())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn the_answer_goes_out_as_json_lines_until_the_page_is_gone() {
        let mut out = Wire::new(Vec::new());
        out.note("Reading a.png ...");
        out.note("  ");
        assert!(out.word("Hal\"lo\n"));
        assert!(out.thought("hm"));
        assert!(!out.gone());
        let text = String::from_utf8(out.wire).expect("utf8");
        let lines: Vec<&str> = text.split('\n').collect();
        assert_eq!(
            lines,
            [r#"{"note":"Reading a.png ..."}"#, r#"{"word":"Hal\"lo\n"}"#, r#"{"thought":"hm"}"#, "", ""]
        );
        let mut out = Wire::new(Shut);
        assert!(!out.word("a"));
        assert!(!out.thought("b"));
        assert!(out.gone());
    }

    #[test]
    fn the_locked_text_cannot_break_out_of_the_page() {
        assert_eq!(escape("<b a=\"1\">Tom & Co</b>"), "&lt;b a=&quot;1&quot;&gt;Tom &amp; Co&lt;/b&gt;");
    }
}
