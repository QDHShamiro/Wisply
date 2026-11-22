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
- Six quick single calls in a row bring a one-time tip. Off for this PC: `wisply config codemode off`.
