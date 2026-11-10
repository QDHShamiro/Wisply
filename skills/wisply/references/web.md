# Wisply: pages in depth

## Snapshots
- Only visible, usable elements are listed. Hidden, covered (behind a modal) and aria-hidden elements are left out, which also blocks hidden prompt-injection text. With a modal open, the snapshot shows only the modal.
- A big nav/footer collapses to one line: `e1 nav "Main": 34 items (wisply snap -s e1)`. Long lists show 5 items plus `... +45 similar items (wisply snap -s e40 --all)`. `more: 23 below` means scroll or use `--all`.
- `unchanged since last snap`: the page did not change and your refs still work. A repeated plain `snap` answers only what changed: `changed since your last snap: +2, -1 gone (all 40 lines: wisply snap --full)`. After a big change, a new page or with any flag you get the full view.
- On a page Wisply knows, `open`, plain `snap`, `back`, `forward` and `reload` show only the controls used there before (up to 15) plus notes, headings and alerts: `brain: 3 controls you use here, 30 others hidden (wisply snap --full shows all)`. Need another one: `snap -f word` or `click "Name"` (searches the whole page).
- Refs stay valid until the page navigates. A stale ref answers `e12 is gone -> try: wisply snap`; when the same element is still there once, Wisply uses it and says so (`! e12 was gone (the page changed); used the same button "Save" instead`).
- `snap --links` lists all link targets. A page with no controls shows its text: `(no controls in view) text: ...`. `(nothing drawn yet ...)` means the page only draws in a visible window: `wisply shot`.

## Frames
- Same-origin iframes are part of the normal snapshot.
- Cross-origin iframes appear as `iframe host.com (cross-origin, wisply snap --frame host.com)`. Refs there look like `f5.e3` and work like any ref (`click f5.e3`, `type f5.e3 text`), payment and login frames included.

## Data without the UI
- `wisply read <url>` asks the server for markdown and falls back to cleaned HTML. It sees no JavaScript content and no login: use `open` + `text` for those.
- Wisply keeps the JSON answers (GET) of its tabs. `open` says `data: 3 JSON calls (wisply net)`; `net [-f word]` lists them, `net 2` shows the shape (`items[] (40) {id, name, price}, total: 40`), then `fetch <url> --pick items[].name` gets exactly that, often instead of reading the page.
- `fetch /api/me --pick login` runs fetch() inside the page with the user's cookies and returns compact JSON. `--method POST --body '{"a":1}'` for writes (not on blocked sites).
- `table e8` or `table #prices` returns a table as tab-separated rows.
- `search youtube "lofi"` (or `search "words"` on the current site) opens the result page directly when the search URL is known; otherwise it uses the search field once and learns the URL.
- WebMCP (pages that offer tools to AI): `open` says `webmcp: 2 tools (wisply tools)`; `tools` lists them, `call <tool> '{"q": "x"}'` runs one. Tools that can change something need `--yes`.

## Tabs and browsers
- Every Claude Code session works in its own browser window (the same logged-in profile) with its own tab group `Wisply #1`, `Wisply #2`, ...; calls without a session id share the group "Wisply". The window opens behind the user's window and closes with its last tab. `config window off` puts the tabs into the user's window. `open -n` opens another tab.
- A session's commands go to its own last tab. A session without a tab gets `this session has no tab yet`: `open <url>` first. `done` at the end of a reply closes only that session's tabs and apps.
- `tabs` lists Wisply's tabs plus the active tab of each window, `--all` (or `-f x`) every tab (`L` = Wisply group, `*` = current target, `a` = active). `tab t3` works in any tab, including the user's own.
- Links that open a new tab switch the target (`! new tab t9 opened from t4`).
- `close` only closes Wisply tabs and tabs this session opened, unless you add `--force`. `keep t3` takes a tab out of the cleanup.
- Several browsers connected: the last focused one is used. `config browser brave|chrome|edge|auto`.

## Waiting
- Actions already wait for navigation and for the page to go quiet.
- `wait "Order placed" --timeout 20s`, `wait url:/dashboard`, `wait "Loading" --gone`, `wait 1500` (times: `90s`, `10m` or milliseconds).
- `wait "All checks have passed" --reload 15s --timeout 10m` reloads the tab every 15 s until the text is there (CI checks, deploys, queues). It waits inside the Wisply daemon and other Wisply commands keep working. Longer than 2 minutes through a shell tool: give that call a longer timeout, or use the MCP tool.

## Pages Wisply cannot script
- `cannot script this page` (brave://, chrome://, Web Store, PDF viewer): use desktop control on the browser window, `wisply snap w1 --web`.
- `dialog open: confirm "..."`: `wisply dialog accept` or `dismiss`.
- Blocklisted sites (`~/.wisply/blocklist.txt`, banking and payment) are read-only for Wisply.
