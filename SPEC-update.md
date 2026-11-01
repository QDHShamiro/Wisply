# SPEC: automatic updates

Goal: a running Wisply keeps itself current from GitHub releases, in the background, without taking the screen,
without touching a running game, and with a way back.

## Behaviour

- `config update auto|notify|off`, default `auto`.
- The daemon checks `releases/latest` of `QDHShamiro/Wisply` at most once every 24 h (first check 3 min after start).
  Only stable releases count (`latest` skips pre-releases and drafts); tags with a suffix are ignored.
- `auto`: download the archive for this system (`wisply-<v>-<windows-x64|macos-universal|linux-x64|linux-arm64>`),
  check its SHA-256 against `SHA256SUMS.txt` of the same release, unpack to `~/.wisply/update/<v>`, run the new
  `wisply --version`, then apply.
- `notify`: only record the new version; the start info says so once a day.
- Nothing runs while a game is running. Applying also waits until no command reached the daemon for 5 minutes.
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

## Commands

`wisply update` (check, stage, apply now) · `--check` (say what is there, change nothing) · `--rollback`.
The start info reports an update once (`updated 3.0.1 -> 3.1.0`, or why it failed) and, in `notify`, a new version once a day.
A version that failed 3 times is not tried again until a newer one appears.

## State

`~/.wisply/update.json`: `checked`, `latest`, `staged {version, root}`, `failed {version, count}`,
`last {from, to, at, ok, why, shown}`, `notified`. Log of the helper: `~/.wisply/update.log`.

## Not in this version

- Verifying the GitHub build attestation (needs `gh`); the checksum list comes from the same release, so it
  protects against broken downloads, not against a hijacked release.
- Updating the Claude Code / Codex plugin files; those come through `/plugin` and only hold hooks and the skill.
