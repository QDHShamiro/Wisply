# wisply list — one Notion-style list for bugs, features, tasks and ideas

Wisply's to-dos become one list in `wisply ui`: table and board, a page per entry, filled by the
user, by any AI (Claude, Codex, Gemini, any MCP client), by server logs and by session notes.
The local model keeps it tidy. Works for every AI through the `wisply` command and MCP, no
Claude-only hooks.

## Decisions (2026-11-21, grill-me with QDHShamiro)
- To-dos become the list. One system, `wisply todo` keeps working, the open to-dos migrate.
- Kinds: `bug`, `feature`, `task`, `idea` (coloured tag). Old to-dos become `task`.
- Status: `open` · `working` · `waiting` · `done`. `working` shows who (user or which AI session).
  `waiting` has a reason: `restart` or `you`. `dropped` stays, only hidden.
- Priority: `high` · `mid` · `low`. AI suggests it, crashes are `high`, warnings `low`.
- View: table and board, switchable. Board columns per status, drag to move.
- Click on an entry opens a side panel on the right: free markdown text (user and AI write),
  log excerpt for log bugs, history ("AI fixed, 14:20"). The list stays visible on the left.
- Nav: new item "List" at the top, it is the start page. Each project's to-do tab shows the
  same list filtered to that project.
- Entries by hand: "+ Entry" in the window and `wisply bug add <server> "text"` /
  `wisply todo add ... --kind --prio --project` for any AI.
- Suggestions: session notes' "next" lines become greyed `suggestion` entries on top, one click
  accepts or drops them.
- Tidy: both. The AI in chat moves its own entries (working/done). The local model tidies when
  idle: merges duplicates, clear titles, sets kind/project/priority. Every change in the history
  and undoable, never deletes, never while a game runs.
- AI work: an entry set to `working` from a session stores that session; the entry shows its
  name (click opens it in Agents). The List page has a top bar with sessions active in the last 24 h.
- Notifications: only crashes and `high`, via `notify event` (never takes focus), phone feed too;
  while a game runs phone only.
- Ships in one piece: list, server logs and tidy together, one PR.

## Server logs
- Servers: all profiles of the user's sftp profile file (9 today), each can be switched off in
  Settings. The path to that file lives in private config (`~/.wisply`), never in the repo.
  Wisply reads logins from it at connect time, never copies or prints them.
- Fetch every 5 min in the daemon, paused while a game runs (`daemon/game.rs`). Only new bytes
  since the last read (offset per file).
- Minecraft default paths: `logs/latest.log`, `crash-reports/`. Other servers (VPS, Node
  backend): log paths set once per server in Settings; without a path the server shows
  "no log set".
- What lands: ERROR/SEVERE lines and exceptions with their stack trace as `bug` (`mid`),
  crash reports as `bug` (`high`), WARN as `bug` (`low`) in a separate "Warnings" tab under the
  errors.
- Same error = one entry with a counter and first/last seen. Fingerprint: logger/plugin +
  exception class + first own stack frame, with numbers, UUIDs, player names and coordinates
  normalised.
- "Ignore" on an entry mutes that fingerprint for that server for good (private config).
- Restart detection: log file shrinks/rotates or a new server start line appears.
  On restart every entry of that server with `waiting: restart` becomes `done`.
- A `done` log bug whose fingerprint shows up again reopens as `open` with "came back"
  in the history.

## Data
Entry: `id, title, kind, status, wait_reason, priority, project, assignee, source
(hand|ai|log|suggestion), body (md), fingerprint, count, first_seen, last_seen, server,
excerpt, created, updated`. History table: `entry_id, at, who, field, old, new`.
Migration from `todos`: `todo→open, doing→working, done→done, dropped→dropped`,
`list→project`, `note→body`, kind `task`, priority `mid`. Ids stay the same.

## CLI (all AIs)
- `wisply todo [--all] [--kind k] [--project p] [--status s]`
- `wisply todo add "title" [--kind k] [--prio p] [--project p] [--doing]`
- `wisply todo doing|done|open|drop <id> [note]`, `wisply todo wait <id> restart|you`
- `wisply todo prio <id> high|mid|low`, `wisply todo kind <id> <kind>`, `wisply todo note <id> "text"`
- `wisply bug add <server> "text"` = `todo add --kind bug --project <server>`
- `wisply list servers` (state per server), `wisply list ignore <id>`
- New verbs go into `client.rs` and `fast.rs` LOCAL, smoke-tested through the real `wisply`.

## UI
- `#/` = List, projects move to `#/projects`.
- Table columns: kind, title, status, priority, project, who, updated. Sort and filter by each.
- Board: four status columns, drag and drop, cards show kind, priority, project, who.
- Side panel: editable title and body (same save/mtime path as notes), log excerpt, history,
  buttons for status/priority/kind/ignore.
- Suggestions block on top, Warnings tab, server row with last fetch time and state.
- Sidebar: To-dos (start page), Projects, Memory, Intelligence, then the rest.
- Memory (`#/memory`): every shelf (remembered, learned, vault, Claude memory, sessions) with
  counts, search, project filter, pages of 200, forget with a second click.
- Intelligence (`#/intelligence`): score, knowledge, lessons, routines and done work as tiles with
  the week before; score, knowledge and points per day as line charts, learned and work per day
  as stacked bars; 7/30/90 days or all. Best day is the high-water mark, a new one gets ⬆.

## Ship
Branch off main, one PR. `cargo fmt`, `clippy -D warnings`, `cargo test --workspace`
(git on PATH + `wisply-task` first), web tests, live test via Haiku, Windows selftest before merge.
