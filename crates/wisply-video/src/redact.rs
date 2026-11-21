use std::sync::LazyLock;

use regex::Regex;

const LINK_MARK: &str = "<link>";

static LINK: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"https?://\S+").expect("link"));

pub fn hide_links(text: &str) -> String {
    LINK.replace_all(text, LINK_MARK).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_are_replaced_by_a_mark() {
        let text = "failed at https://media.example.test/v?sig=abc and http://x.test/y, twice";
        assert_eq!(hide_links(text), "failed at <link> and <link> twice");
        assert_eq!(hide_links("no link here"), "no link here");
    }
}
