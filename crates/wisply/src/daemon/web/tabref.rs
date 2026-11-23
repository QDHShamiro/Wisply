use serde_json::json;
use wisply_core::Fail;

use super::super::Daemon;
use super::super::ctx::Ctx;
use super::{SNAP_LINES, call, head, obj, out, tab_url, text_of};

const TAB_WAIT: f64 = 15.0;
const TWIN_MIN: usize = 12;

pub fn tab(d: &Daemon, ctx: &mut Ctx, id: &str) -> Result<String, Fail> {
    match out(d, ctx, "tab", obj(json!({"id": id})), TAB_WAIT, SNAP_LINES) {
        Err(e) if e.msg.split(" -> ").next() == Some(format!("no tab {id}").as_str()) => moved(d, ctx, id, e),
        done => done,
    }
}

fn moved(d: &Daemon, ctx: &mut Ctx, id: &str, e: Fail) -> Result<String, Fail> {
    let Some(url) = tab_url(ctx, id) else { return Err(e) };
    let key = page_key(&url);
    let listing = call(d, ctx, "tabs", obj(json!({"all": true})), TAB_WAIT).ok();
    let Some(twin) = listing.as_ref().and_then(|m| twin(text_of(m, "out"), &key, id)) else {
        return Err(Fail::hint(
            format!("no tab {id}: it was closed (it showed {})", head(&url, 80)),
            format!("wisply open {url}"),
        ));
    };
    let res = out(d, ctx, "tab", obj(json!({"id": twin})), TAB_WAIT, SNAP_LINES)?;
    ctx.note(&format!("{id} was closed; {twin} shows the same page, so Wisply used {twin}"));
    Ok(res)
}

fn page_key(url: &str) -> String {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let rest = rest.strip_prefix("www.").unwrap_or(rest);
    let end = rest.find(['?', '#']).unwrap_or(rest.len());
    rest[..end].trim_end_matches('/').to_lowercase()
}

fn twin(listing: &str, key: &str, gone: &str) -> Option<String> {
    listing.lines().find_map(|line| {
        let id = line.split_whitespace().next()?;
        let shown = line.rsplit(" - ").next()?;
        let cut = shown.strip_suffix("...");
        let page = page_key(cut.unwrap_or(shown));
        let same = match cut {
            Some(_) => page.len() >= TWIN_MIN && key.starts_with(&page),
            None => !page.is_empty() && page == key,
        };
        (same && id != gone && id.starts_with('t')).then(|| id.to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_closed_tab_finds_its_twin_by_page() {
        let key = page_key("https://www.discord.com/channels/@me/1533544133417893970?x=1#top");
        assert_eq!(key, "discord.com/channels/@me/1533544133417893970");
        let listing = "t54 L1* Old - discord.com/channels/@me/1533544133417893970\n\
                       t61 a Discord | DEV - discord.com/channels/@me/1533544133417893970\n\
                       t62 Other - github.com/QDHShamiro/Wisply";
        assert_eq!(twin(listing, &key, "t54").as_deref(), Some("t61"));
        let cut = "t70 Long - discord.com/channels/@me/15335441...";
        assert_eq!(twin(cut, &key, "t54").as_deref(), Some("t70"));
        assert_eq!(twin("t71 Short - disc...", &key, "t54"), None);
        assert_eq!(twin("t72 Home - discord.com/channels/@me", &key, "t54"), None);
        assert_eq!(twin("+3 other tabs of yours (wisply tabs --all lists them)", &key, "t54"), None);
    }
}
