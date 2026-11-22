# Code mode (many steps in one call)

`wisply codemode "<js>"`, `--file x.js`, `-` (stdin) or the MCP `code` argument runs one JavaScript in a sandbox
(QuickJS, no network, no require). Only the result and `console.log` lines come back, so the tokens of every step in
between are saved.

```
const [st, tests] = await Promise.all([$`git status --short`.lines(), $`cargo test -q`.nothrow()]);
for (const f of await glob("src/**/*.rs")) if ((await read(f)).includes("todo!")) console.log(f);
({ changed: st.length, tests: tests.ok, page: await wisply`snap -f "Sign in"` })
```

- `` $`cmd ${v}` `` runs in bash (Git Bash on Windows, else PowerShell); values are quoted, arrays become words.
  `.text() .lines() .json() .nothrow() .cwd(d) .env({}) .timeout(ms) .stdin(t)`; a failing exit throws.
- `` wisply`...` `` is any wisply command; `read`, `write`, `edit(path, from, to)`, `glob`, `sleep`; up to 8 calls run
  in parallel; default and maximum time 10 minutes (`--timeout 2m`).
- An error names the line, the source and how many calls already finished (they stay done).
- Safety: the deny and ask rules of Claude Code, Codex, Gemini and Cursor and the four risk classes apply to every
  step. Deleting inside the project passes; outside it, keys, rights, money or autostart stop the script before the
  step runs. Ask the user, then run the same code with `--yes` (`"yes": true` over MCP). A denied step never runs.
- Plain form steps without logic can also be one `wisply do "fill Email=a@b.c; click Continue; expect Welcome"`.
- Three single calls within two minutes bring a tip, every time. Off for this PC: `wisply config codemode off`.
- Values: `--arg pr=100` (MCP `args: {"pr": "100"}`) is `args.pr` in the code. Pass changing values this way, never
  paste them into the code, so the same script can be saved and replayed.
- Saved: a working script with 2+ calls becomes a local routine, once per script, named from the request (never
  with secrets, `--yes` never kept). A value from the request left in the code (e.g. `2026-10`) is saved as
  `args.month`. The reply names it: `wisply run <name> pr=101` replays it in one call,
  `wisply routine show <name>` prints the code. When a saved script fails, fix it and run the fixed one with
  `wisply codemode`; it replaces the routine as a new version (so does a corrected script for the same task in the
  same session). Unused: gone after 30 days (90 from 5 runs).
  Off: `wisply config codemode_save off`.
- Heavy or hot work as a Rust tool: `wisply kit add <name> "what it does" --rust tool.rs` (or a Cargo folder),
  built once, rebuilt only when the code changes; `wisply kit run <name> [args]`.
