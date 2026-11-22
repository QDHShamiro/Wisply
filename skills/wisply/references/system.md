# Wisply: settings, logs, own tools and trouble

## Status, config, logs
- `wisply status` shows the daemon, connected browsers and the current target.
- `wisply config` lists settings (`browser`, `pointer`, `ocr_lang`, `shot_width`, ...); `config <key> on|off|<value>` sets one.
- `wisply log [n]` shows the last actions (typed text and secrets are never logged). `log stats` sums them up per command: runs, error rate, time, output tokens and how often a `snap` followed an action.
- `wisply tokens` shows the tokens used and saved. `--stats` after any command prints its time and token estimate.
- Big outputs are cut and the full text is saved to `~/.wisply/out/` (path printed): search that file instead of running the command again.
- The status line shows how far the task is. With a plan tool (TodoWrite, TaskCreate/TaskUpdate, `update_plan`) that happens by itself; without one say it: `wisply progress 3/8 "what you are doing"` or `wisply progress 40`, and `wisply progress clear` when the task is over.

## Own tools and skills
- A step you keep repeating by hand becomes a tool: `wisply kit add <name> "what it does" [--file script] -- <program> [args]` (`{1}`..`{9}`, `{all}`, `{file}` stand for arguments and the script), then `wisply kit run <name> [args]` in every session. `wisply kit` lists them.
- Know-how: `wisply skill find <words>` (this computer first, `--web` for GitHub), `skill get <owner/repo/path>` installs one for every AI here, `skill new` explains and builds one.
- An answer that says "ask them first" means the user decides (deleting, AI permissions, money/accounts/keys/uploads, autostart or code from the internet): ask, and only repeat the call with `--yes` after they agreed.

## Before a commit, and improving work
- `wisply secure` checks keys, risky code and dependencies of the folder.
- `wisply think help` lists the steps that find, fix and prove improvements.

## Safety
- Buttons whose name looks irreversible (buy, pay, delete, ...) need `--yes`.
- Blocklist: `~/.wisply/blocklist.txt` (banking and payment domains). Those sites are read-only for Wisply.
- Ctrl+Alt+End stops the running and queued commands immediately; `wisply abort` does the same.

## Trouble
- `err: no browser with the Wisply extension is connected`: `wisply setup` (it opens the extensions page and loads the extension by itself).
- Something hangs: `wisply stop`; the background process starts again on the next call.
- Broken or offline: `wisply doctor --fix`. `wisply selftest` checks browser, desktop, chat and sessions.

## A new Claude Code session (only when the user asks)
- `wisply claude "the task" --yes` opens a new tab in Windows Terminal, starts Claude Code with Remote Control (so the user can follow it on the phone or at claude.ai/code), waits until it is ready and pastes the task. `--bg` runs it in the background.
- `--dir E:/project` picks the folder (default: the current one), `--name "Shop fix"` the tab and Remote Control name, `--model opus|sonnet|haiku|<id>` and `--effort low|medium|high|xhigh|max` set model and effort, `--no-remote` leaves Remote Control off. Without a task it only starts the session.
- If Claude asks whether to trust the folder, Wisply stops and says so: the user answers in the terminal.
