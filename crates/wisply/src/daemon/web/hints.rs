use wisply_core::Fail;

const EATEN: &str =
    "the shell eats `backticks` and $names inside \"...\": quote the code with '...', or wisply js --file x.js";
const TAB_REF: &str = "wisply tabs (pick a tab by its ref like t3 or a word of its title, not a number id)";
const TAB_ALL: &str = "wisply tabs --all";

pub fn js(fail: Fail, code: &str) -> Fail {
    let text = fail.text();
    let undefined = text.contains("ReferenceError") && text.contains("is not defined");
    if code.contains('`') || !undefined || text.contains(" -> try:") {
        return fail;
    }
    Fail::hint(text.trim_start_matches("err: ").to_string(), EATEN)
}

pub fn tab_filter(f: &str) -> &'static str {
    if !f.is_empty() && f.chars().all(|c| c.is_ascii_digit()) { TAB_REF } else { TAB_ALL }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_the_shell_ate_gets_a_quoting_hint() {
        let fail = Fail::new("js: ReferenceError: webgl2 is not defined");
        let code = "[webgl2, webgl].map(k => k).join(' ')";
        assert_eq!(js(fail, code).text(), format!("err: js: ReferenceError: webgl2 is not defined -> try: {EATEN}"));
        let kept = Fail::new("js: ReferenceError: x is not defined");
        assert_eq!(js(kept, "`a` + x").text(), "err: js: ReferenceError: x is not defined");
        let other = Fail::new("js: TypeError: y is null");
        assert_eq!(js(other, "y.z").text(), "err: js: TypeError: y is null");
    }

    #[test]
    fn a_number_as_tab_filter_names_the_tab_refs() {
        assert_eq!(tab_filter("1533544133417893970"), TAB_REF);
        assert_eq!(tab_filter("github"), TAB_ALL);
        assert_eq!(tab_filter(""), TAB_ALL);
    }
}
