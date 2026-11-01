---
description: Check every part of Wisply and your AI CLIs and repair what is broken (like /doctor, for Wisply)
---
Run `wisply doctor` and show its lines unchanged.
If its last line says `wisply doctor --fix repairs N`, run `wisply doctor --fix` and show that output unchanged too.
Then run `wisply doctor ai` and show its lines unchanged (it checks Claude Code, Codex, Gemini, Qwen and the other AI CLIs it finds, read-only).
For every line that still says `warn` or `FAIL`:
- `-> fix: wisply setup ...`: run it and report its last line.
- a fix in another tool's settings (Claude Code, Codex, Gemini ...): tell the user the exact edit in one line and apply it only after the user says yes. Never edit `env`, `headers` or hook commands, never add allow rules for commands.
- any other `-> fix:` (installing ffmpeg or uv, updating the plugin, Windows settings, `/wisply:setup`): tell the user that exact step in one line, do not do it yourself.
End with one line: healthy, or what is still open.
