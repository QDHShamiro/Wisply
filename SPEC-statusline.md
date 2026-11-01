# wisply statusline — Claude Code status line from Wisply

Wisply ships its own status line for Claude Code. It installs itself with Wisply, can be switched on
and off with one command, and shows model, context, limits, git branch and GitHub state. It replaces
hand-made scripts (PowerShell `statusline.ps1`, caveman/ponytail badges). No PowerShell, no Python, no
extra runtime: it is a subcommand of the `wisply` binary.

## Decisions (2026-11-01)
- Claude Code calls `wisply statusline` for every refresh (JSON on stdin, one line on stdout).
- Comes with Wisply: `wisply setup` turns it on by default. Off and on at any time with
  `wisply statusline on|off`. From inside Claude: `/wisply:statusline on|off|status`.
- Segments, on by default: branch, GitHub (PR + checks), model, effort, context bar, tokens,
  5-hour limit, weekly limit. Each can be switched with `wisply statusline set <segment> on|off`.
- No caveman / ponytail / tool-specific badges. Nothing reads `.caveman-active` or `.ponytail-active`.
- Build starts after `feat/auto-update` is merged (it touches `setup/` and `daemon/`). New branch off
  main, one PR. Same rule as `SPEC-claude-doctor.md`.

## What it shows
| segment | source | shown as |
|---|---|---|
| branch | `.git/HEAD` read directly (no process), found upward from `workspace.current_dir` | `feat/x`, detached head as short sha |
| dirty / sync | `git status --porcelain=v2 --branch`, cached | `*` dirty, `↑2 ↓1` ahead/behind |
| github | `gh pr view` for the branch, cached by the daemon | `#41 open ✓` (checks pass), `✗` failing, `…` pending, `draft`, `merged`; nothing when the branch has no PR |
| repo | `origin` URL parsed locally | `owner/repo` only when the segment is on; default off |
| model | stdin `model.display_name` | plain |
| effort | stdin `effort.level` | `effort:HIGH` |
| context | stdin `context_window.used_percentage` | 10-char bar + `%`, green < 60, amber < 85, red above |
| tokens | stdin `context_window.current_usage` | `34.2k tokens` |
| limit 5h | stdin `rate_limits.five_hour` | `12% used (88% left) resets in 3h10m` |
| limit week | stdin `rate_limits.seven_day` | `week 40% used (60% left) resets in 2d4h` |

Segments with no data are left out; the separator is `|` in dark grey. Missing fields never error.

## Speed (it runs on every refresh)
- Render path: parse stdin, read `.git/HEAD`, read the cache file, print. Target < 20 ms, no child
  process in the render path.
- Dirty/ahead-behind and the PR state are never computed while rendering. A short-lived detached
  `wisply statusline refresh <root> <branch>` does it (no daemon needed, hidden window): git state when
  the cache is older than 5 s, PR state when older than 60 s, both at once for a new branch. The renderer
  shows the cached value and starts the refresher when it is stale; one refresher at a time per branch
  (`started` marker, 4 s gap).
- Cache: `~/.wisply/statusline/cache.json`, keyed by repo root + branch: `{dirty, ahead, behind, pr
  {number, state, draft, checks}, at}`. A value older than 10 minutes is dropped, not shown.
- `gh` missing or not logged in: the github segment is silently absent and `wisply statusline status`
  says why. Nothing is ever written to GitHub.
- Never blocks, never prints errors into the line; a panic prints nothing.

## Install and switch
- Claude Code setting: `statusLine: {"type":"command","command":"<wisply> statusline"}` in
  `settings.json`. Extends the existing `setup/claude_env.rs` pattern (`load`/`save` with a tmp file
  and rename; `point`/`unpoint`).
- Ownership: Wisply only treats the setting as its own when the command starts with its own binary
  path. Anything else is the user's.
  - No `statusLine` set: `setup` and `statusline on` write ours.
  - A foreign `statusLine` is set: `setup` leaves it alone and says so once; `statusline on` asks
    for `--force`, which saves the old value to `~/.wisply/statusline/previous.json` first.
  - `statusline off` removes ours and restores `previous.json` if there is one.
- `wisply setup --uninstall` runs the same as `off`.
- Settings keys other than `statusLine` are never touched; parse error in `settings.json` = left alone
  and reported.
- Config: `~/.wisply/statusline/config.json`: `enabled` (default true), `ascii`, `segments`. Disabled:
  `wisply statusline` prints nothing. `statusline off` writes `enabled: false`, so a later `setup` (for
  example from auto-update) does not switch it on again.

## Commands
`wisply statusline` (render, stdin) · `on [--force]` · `off` · `status` (enabled, owner of the setting,
segments, last cache age, gh state) · `set <segment> on|off` · `preview` (renders with sample data) ·
`doctor` row in `wisply doctor` (ok / warn if the setting points to a missing binary) comes after
`feat/auto-update`, it touches the same file.

## Layout
- `crates/wisply/src/statusline/`: `mod.rs` (route, args), `render.rs` (segments, colors, limits),
  `git.rs` (HEAD reader, status parser), `github.rs` (gh runner and parser, daemon side),
  `cache.rs`, `install.rs` (settings.json edit, ownership, previous.json).
- `commands/statusline.md` for `/wisply:statusline`.
- Colors and thresholds come from `config` with the defaults above; no hardcoded layout in code paths
  beyond the defaults table.

## Tests
- Render: golden lines for full stdin, missing fields, over-limit colors, no repo, detached head.
- Git: HEAD reader on branch, detached, worktree (`.git` file), bare path; porcelain v2 parser.
- GitHub: parser for open / draft / merged / failing / pending checks; `gh` absent.
- Install: no setting, own setting, foreign setting (no change, `--force` saves previous, `off`
  restores), broken JSON left alone, other keys preserved byte-for-byte in meaning.
- Speed: render with warm cache under 20 ms on the real machine (Windows path, HDD home).

## Open
- Windows: the `↑↓✓✗…` glyphs in the user's terminal; `wisply statusline set ascii on` gives `+2 -1 ok`.
- Whether the plugin (`wisply@wisply`) can ship the setting itself; until then `setup` writes it.
