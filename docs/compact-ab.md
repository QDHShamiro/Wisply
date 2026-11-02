# Auto-compact window: 250k against 400k, a two-week comparison

Question: does compacting earlier (250k tokens of context) make Claude Code cheaper without making the work worse
than compacting at 400k? Nothing here runs by itself. You switch the window by hand every two days, Wisply measures
both arms from the transcripts you produce anyway.

## What the settings are

Claude Code takes the window from `autoCompactWindow` in `settings.json` (100000 to 1000000), from
`/autocompact 250k`, from `claude --autocompact 250k` for one launch, or from the environment variable
`CLAUDE_CODE_AUTO_COMPACT_WINDOW`. The variable beats everything else, so do not set it while the test runs.
Sources: https://code.claude.com/docs/en/model-config#set-the-auto-compact-window and
https://code.claude.com/docs/en/settings-reference#autocompactwindow.

Wisply writes the same key:

```
wisply config compact 250k
wisply config compact 400k
wisply config compact off
```

`wisply setup --compact 250k` does the same. A running session keeps its window, so switch before you start a session.

## Design

- Length: 14 days, 7 blocks of 2 days, alternating arms. Start with 250k on a Monday: A B A B A B A.
  Alternating blocks keep weekday and project mix about equal in both arms.
- Switch at the start of a block, never during one. Write the block, the date and the arm in a note
  (`wisply remember "compact test: block 3 = 400k"`), because the transcripts do not record the setting.
- Same model, same effort level and the same Wisply hooks in both arms. Do not change `settings.json` otherwise.
- Work as usual. Do not avoid long sessions in either arm; the point is how the window changes the same kind of work.

## Cost, from the `usage` fields

Every assistant line in `~/.claude/projects/*/*.jsonl` has `message.usage`. Per request:

- context = `input_tokens` + `cache_creation_input_tokens` + `cache_read_input_tokens`
- cost weight (relative to the input price) = 1.0 x fresh input + 0.1 x cache read + 2.0 x cache write (1 h cache)
  + 5.0 x output

Use the weights of your plan if they differ. Sum per day, assign each day to its block, then compare the arms by
cost per finished work item, not per day: use commits merged (`git log --since`), or finished to-dos
(`wisply todo --all`), as the denominator. Costs per day vary too much with the amount of work.

Also record per arm:

| Number | Where it comes from |
|---|---|
| Requests above 200k and above 400k context | `wisply doctor ai claude`, row `context length` |
| Compactions | transcript lines where the context drops below half of the previous request |
| Cost weight per merged commit | usage fields, git log |
| Cache write share | usage fields (compacting rewrites the cache) |

Expected from the earlier measurement (2.8k requests/day, 36% of cache reads above 400k): 250k saves about 8 to 12%
of the weighted cost. The test shows whether that holds once the compactions themselves are paid for.

## Quality, from the transcripts

Compacting loses detail. Two signals show it, both counted by hand or with a short script over the user lines
after a compaction (the line that starts with "This session is being continued from a previous conversation"):

1. Follow-up questions: within the next 10 user messages after a compaction, count messages that ask for something
   Claude had before (for example "which file was that", "what did we decide", "you already knew this").
2. Repeats: within the next 20 tool calls after a compaction, count reads of a file that was already read in the
   200 requests before it, and repeated identical commands.

Report per compaction: follow-ups, repeats. Divide by the number of compactions in the arm. The handoff note that
Wisply adds after a compaction (goal, decisions, open points, changed files) is on in both arms, so it does not
distort the comparison; to measure it on its own, run a third pair of blocks with `wisply memory off`.

## Decision rule

- Choose 250k if the weighted cost per merged commit is at least 5% lower and the follow-ups plus repeats per
  compaction are not more than 25% higher than in the 400k arm.
- Otherwise stay at 400k, or try 320k.
- With fewer than 10 compactions in the 250k arm the quality numbers are too thin; extend by one week.

## Undo

`wisply config compact off` removes the key. Claude Code then uses its own default window again. The real
`settings.json` is only touched by those commands, and `wisply doctor ai claude` names every place that sets a
window (settings files, the variable, per-model settings) so nothing silently overrides the test.
