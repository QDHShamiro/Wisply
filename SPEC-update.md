# SPEC: automatic updates

Goal: a running Wisply keeps itself current from GitHub releases, in the background, without taking the screen,
without touching a running game, and with a way back.

## Behaviour

- `config update auto|notify|off`, default `auto`.
- The daemon ticks every 60 s (first tick 3 min after start) and checks `releases/latest` of `QDHShamiro/Wisply`
  at most once an hour.
  Only stable releases count (`latest` skips pre-releases and drafts); tags with a suffix are ignored.
- `auto`: download the archive for this system (`wisply-<v>-<windows-x64|macos-universal|linux-x64|linux-arm64>`),
  check its SHA-256 against `SHA256SUMS.txt` of the same release, unpack to `~/.wisply/update/<v>`, run the new
  `wisply --version`, then apply.
- `notify`: only record the new version; the start info says so once a day.
- Nothing runs while a game is running. Applying waits until no command reached the daemon for 5 minutes; once the
  update has been staged for 6 h, a free moment is enough (no command running and none in the last 10 s). A command
  counts from its start until its end.
- Dev builds are left alone: no auto update unless the daemon runs from `~/.wisply/bin` and the home is the default one.

## Apply (helper process)

The daemon cannot replace itself, so it starts a detached copy of the old program (`update/helper-<pid>`):

1. save the extension folder to `update/extension-prev`
2. run `<new>/wisply setup --manual` (the normal setup: copies the programs, keeps the old ones as `*.old`, copies the
   extension, writes the browser link, stops the old daemon)
3. run `wisply status` (starts the new daemon) and require the first line to be `wisply <v> |`, three tries
4. ok: `ext-reload`, record the result, delete the staged files. Not ok: roll back.

Rollback = stop the daemon, move every `*.old` over its program, restore the extension folder, start the daemon.
`wisply update --rollback` does the same by hand.

Windows does not let a running program be deleted or overwritten, only renamed. A file that is still in use (an
older `*.old` an MCP server runs from, or the program itself on rollback) therefore moves aside to `*.old1` …
`*.old9`; every setup first removes the ones nothing runs any more.

## Commands

`wisply update` (check, stage, apply now) · `--check` (say what is there, change nothing) · `--plugins` (plugin
step by hand) · `--rollback`.
The start info reports an update once (`updated 3.0.1 -> 3.1.0` plus up to three first sentences of the new
CHANGELOG section, which is built into the program, or why it failed) and, in `notify`, a new version once a day.
A version that failed 3 times is not tried again until a newer one appears. `wisply status` has an `update` and a
`plugins` line.

## Plugins

The helper is the old program, so the plugin step runs in the new daemon: on its first tick and on every hourly check.

- Claude Code: every user install `wisply@<m>` in `<CLAUDE_CONFIG_DIR or ~/.claude>/plugins/installed_plugins.json`
  that is older than the program. A git/github marketplace is refreshed (`claude plugin marketplace update <m>`); then
  the version it offers is read (the `ref` tag of the source, or the `plugin.json` of a path source). Only when it is
  exactly the program version: `claude plugin update wisply@<m> --scope user`. A folder marketplace with another
  version is skipped with the reason in `wisply status`.
- The marketplace of this repo pins the plugin to `ref: vX.Y.Z`; a test keeps it equal to the program version.
- Codex: enabled `wisply@<m>` in `<CODEX_HOME or ~/.codex>/config.toml` whose marketplace has `source_type = "git"`:
  `codex plugin marketplace upgrade <m>`, once per version.
- Both run hidden, with the session variables of a calling AI removed and a 2 minute limit; 3 failed tries per version
  stop the automatic runs (`--plugins` still runs).

## State

`~/.wisply/update.json`: `checked`, `latest`, `staged {version, root, at}`, `failed {version, count}`,
`last {from, to, at, ok, why, shown}`, `notified`,
`plugins {claude|codex: {to, from, ok, why, tries, skip, shown}}`. Log of the helper: `~/.wisply/update.log`.

## Not in this version

- Verifying the GitHub build attestation (needs `gh`); the checksum list comes from the same release, so it
  protects against broken downloads, not against a hijacked release.
