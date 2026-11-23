# Wisply: routines, recording and what it learns

## Routines (learned tasks)
- Wisply keeps every successful action with a stable target (`click e12` as `click "Sign in"`, `d5` by its name). A task seen twice becomes a routine; what differed between the runs becomes a parameter. It is announced once: `! learned routine spotify-wiedergeben ...`.
- Offers come by themselves: `! routine X can finish this in one call: wisply run X search=...` after the first matching step, or one line in the prompt: `Wisply routine fits this request (one call, swap the values): wisply run discord-markieren user=Luis text='hi'`. Run it instead of doing the steps.
- `wisply run X param=value` replays with fresh lookups; elements that are not there yet are retried for up to 6 s. Without the exact name, the words before the `k=v` pairs pick the routine: `wisply run discord markieren user=Tom text=hi`. Several fit equally: the error names them.
- Steps that needed `--yes` stop the run unless the run itself gets `--yes`.
- The end state is checked; a run that does not verify counts as failed and says which step broke. Finish the task by hand: when it ends, Wisply rebuilds the routine from what worked (`! repaired routine X (v2)`). Three failures in the last five runs mark it `FLAKY`; flaky routines are not offered.
- `routine save <name> [param=value]` saves the task just done. Name it by intent in the user's words: `routine save discord-markieren user=Tom text=hi`. A step `run gh-open page=Tools` uses another routine (at most 3 deep).
- `wisply routines [-f word]` lists them (`spotify-wiedergeben search= | 2x ok | 1.4 s`), `routine show X` shows steps and stats, `routine rm|rename`, `routine forget` clears the task history (routines stay), `config learn off` stops learning.
- Tasks that only navigate are not learned. Learned routines never run within 14 days are deleted; saved and recorded ones stay.
- Script routines: a working `wisply codemode` script with 2+ calls is saved whole (loops, shell, files, logic), once per script, offered only in its project. `wisply run X pr=101` hands the values to the script as `args.pr`; `--yes` is never saved. It is marked `| script` in the list, `routine show X` prints the code. A failed run: fix the script and run it with `wisply codemode`, the next working one replaces X (v2). Gone after 30 days unused, 90 days from 5 runs on; `config codemode_save off` stops saving.
- Files: `~/.wisply/routines/<name>.json` and `~/.wisply/episodes.jsonl` (last 300 tasks, typed text included; `--env` values are stored as `@secret`).

## Skills (taught once, then one call)
- A skill is a task that repeats with other values (send a message, fill a form, check a PR). The AI writes it once, Wisply practices and checks it without tokens.
- `wisply learn "<task>"` prints the short how-to and names skills that may do it already. Write `run.js` (values as `args.x`; with `args.dry === "1"` stop right before the last real step) and `check.js` (gets the same args plus `args.result`, throws or returns false when the outcome is wrong).
- `wisply learn save <name> "<what it does>" --file run.js --check check.js [--guide "<args>"] [--before 2m] [--tokens 3000] k=v` saves it and practices at once: 3 dry runs, each checked. All green: ask the user, then `wisply learn ok <name>`. Red: the error is kept as a hint for the next AI (`learn show <name>`); fix and save again (v2).
- Learned skills are offered in the prompt (`Wisply skill fits this request (one call, checked after every run): wisply run send-message to=<…> text=<…>`) and checked after every real run. `learn practice <name> [k=v]` practices again, `learn drop <name>` removes one.
- `wisply learn` lists them with runs, time against by hand and tokens saved; `WEAK` (often fails or barely faster) = ask the user, then drop. Script routines that keep coming back are named as candidates.
- Default skills on every PC, local and editable (`learn show`, then `learn save` with your own file): `send-message to= text= app=` (WhatsApp Web default, Telegram Web, chat apps; sends at once, only on an exact chat name, then checks the message is in the chat), `read-form url= fill="A=1;B=2" submit= find=`, `find-file name= in=`, `pr-ci pr= repo=`. A dropped default stays away; a changed one is never overwritten by updates.

## Recording (the user shows it once)
- `wisply record start <name>` records the Wisply tab (clicks, form fields, keys, scrolling, typed-in URLs) and Windows apps (clicks by control name, typing, shortcuts). A red `REC` badge shows in the page and on the screen.
- Browsers and terminals are skipped on the desktop side; password fields become `@secret`. Recording stops by itself after 15 minutes (`--max` minutes).
- `record stop [name] [param=value ...]` saves a routine; every `value` in the steps becomes `{param}`. `record cancel` throws it away, `record status` shows the progress.
- `--web` or `--desktop` records only one side.

## The brain (learns on this computer)
- Wisply remembers which controls you use on each page and app and shows only those on the next visit (`snap --full` shows all).
- It learns from misses: when `click "Checkout"` finds nothing and `click "Go to checkout"` works right after, the next miss says so, and from the second time on it uses the name that worked (`! nothing called "Checkout" here; used "Go to checkout" ...`).
- `wisply brain` shows what it learned, `brain forget github.com` or `brain forget app:spotify.exe` drops a site, `brain reset --yes` everything (only when the user asks), `config brain off` turns it off. Nothing leaves this computer; blocklisted sites are never learned.
- `wisply improve [days]` is a self-check: the most frequent errors of the last days with what to do, loops that kept going after an error, and what the brain saved.
