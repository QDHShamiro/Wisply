---
description: Switch the Wisply status line on or off and choose what it shows
---
The user's argument is: $ARGUMENTS
- empty or `status`: run `wisply statusline status` and show its lines unchanged.
- `on`, `off`: run `wisply statusline <argument>` and show its output unchanged. If `on` says the user's own status line is set, tell the user that `wisply statusline on --force` replaces it (the old one is saved and `off` brings it back) and do not run it yourself.
- `set <segment> on|off`: run `wisply statusline set <segment> on|off` and show its output unchanged. Segments: branch, sync, github, repo, model, effort, context, tokens, limit5h, limitweek, ascii.
- `preview`: run `wisply statusline preview` and show the line.
End with one line: what changed, or that nothing changed. Claude Code reads the status line setting at the next refresh, a restart is not needed.
