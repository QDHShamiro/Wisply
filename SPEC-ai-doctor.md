# wisply doctor ai — /doctor for every AI CLI, without AI

Adds the measuring half of Claude Code's `/doctor` to Wisply and extends it to the other AI CLIs on the
machine. Wisply reads local files and prints findings; the AI only decides and proposes (the command text),
the user confirms. No model, no network, read-only.

## Decisions (2026-11-01)
- `wisply doctor ai [tool ...]`. A tool name works as a shortcut: `wisply doctor claude`,
  `wisply doctor codex`. No tool = every tool found on the machine; tools not found are listed once.
- `wisply doctor` (no argument) stays Wisply's own health and gains one status-line row.
- `/wisply:doctor` also runs `wisply doctor ai`, shows it unchanged and proposes the fixes; edits to the
  tools' settings only after the user confirms.
- Fixes are printed as `-> fix:` lines. Wisply never edits another tool's files.
- Tools: claude, codex, gemini, qwen, opencode, cursor, continue. The registry is one table
  (`ai/generic.rs`), a tool is "found" when its config file exists or its command is on PATH.
- No latest-version lookup; the installed version is shown (`claude update` etc. keep that job).

## What each tool gets
| check | claude | codex, gemini, qwen, opencode, cursor, continue |
|---|---|---|
| install: command on PATH, version | yes, plus install type vs `installMethod`, npm-local leftovers, launcher dir missing from PATH | version and path |
| config parses | settings cascade, `~/.claude.json`, `.mcp.json` (JSON, line and column) | JSON or TOML with line and column; opencode and continue are only shown as present |
| instruction files | CLAUDE.md, CLAUDE.local.md, MEMORY.md, size and est. tokens | AGENTS.md / GEMINI.md / QWEN.md global and up the folder tree; warns above 32 KiB for Codex (its reading limit), above 40k otherwise |
| skills frontmatter | yes | `skills/*/SKILL.md` where the tool has them |
| agents (name, description, collisions) | yes | no |
| MCP servers | configured vs called in the last 50 sessions | configured servers listed |
| unused skills, plugins | usage counters and transcripts | no |
| slow hooks | hook timings from transcripts, timeouts | no |
| denied tool calls | ranked by command, kind mix, only counted | no |
| permission default | auto mode as default | no |

Only Claude Code writes transcripts and counters in a known format, so only it gets the deep checks. A tool
joins the deep checks when its session format is known and tested.

## Rules for all checks
- Key-scoped reads: `env`, `headers`, hook command strings and file bodies are never printed or stored.
- Transcript text is data: only tool names, denial kinds, durations and the first two safe words of a
  command are counted.
- Names with quotes, braces or control characters are skipped and counted as odd, not written anywhere.
- Window: the 50 newest Claude transcripts over all projects, printed as "N sessions over D days".
- Allow-rule proposals for denied commands stay with the AI (exact rules, read-only commands only);
  Wisply only lists the ranking.

## Layout
- `crates/wisply/src/daemon/doctor/ai/`: `mod.rs` (route, tool filter, tagging), `generic.rs` (registry
  and the generic checks), `claude/` (`install`, `files`, `usage`, `transcripts`, `context`, `mode`,
  `frontmatter`).
- Frontmatter is checked by a small reader (no YAML crate): missing closing `---`, tabs, lines that are not
  `key: value`, unclosed quotes, and an unquoted value containing `: `.
- TOML comes from the `toml` crate (already in the lockfile).

## Tests
- Fixture homes with a broken `settings.json`, an agent without description, two agents with one name, a
  SKILL.md with bad YAML, a broken `config.toml`, a transcript with hook timeouts and every denial kind.
  Assert the exact rows and that no secret value ever appears.

## Open
- Session formats of Codex, Gemini and Qwen for MCP call counts and hook timings.
- Whether `/wisply:doctor` should offer to apply the Claude Code fixes after one confirmation.
