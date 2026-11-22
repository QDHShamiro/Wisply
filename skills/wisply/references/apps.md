# Wisply: desktop apps

## Find the window
- Name the app on the first call: `snap app:Discord` (matches title or exe) or a ref from `wisply windows` (`w3 Discord - Discord.exe *`; `*` = in front, `(min)` = minimized). Later commands without a ref use that window; `--on web` goes back to the page.
- `windows -f spotify` with exactly one match chooses that window (not browsers or terminals). Without a chosen window an app command fails with `no window chosen`.
- A `w` ref whose window was closed moves to the app's new window when there is exactly one; otherwise `window w4 is gone (closed)`.

## Start and close apps
- `launch spotify` starts an app (Start menu name, then Win+R names like `calc`, `notepad`, then the closest name), waits for the window and answers with its first look: `ok (Spotify) -> [w1] Spotify Premium - Spotify.exe (84 controls)` plus the controls (`-f word` narrows them). That window becomes the target; `done` closes it at the end of the task (apps that were already open stay).
- URIs open the app that handles them: `launch "spotify:search:SOS"`, `launch ms-settings:display`, `open obsidian://open?vault=V&file=Note`. A path opens the file; programs (`.exe`, `.bat`, ...) and risky links (`file:`, `ms-msdt:`, `search-ms:`, Office links) need `--yes`.
- `close w4` asks the window to close (like the X button); a save dialog may appear.

## Read it
- `snap w3` gives the controls: `d5 button "Save"`, `d6 edit "Search" ="abc"`, `d7 checkbox "Wrap" [x]`, `d8 tab "Home" *`, `(disabled)`, `(collapsed)`. A second `snap w3` without changes answers `unchanged since last snap (refs still valid)`.
- Controls that repeat in every item (the reply/react buttons on each chat message) show once: `d87 button "Antworten" (x13, same in each item)`. Their refs all still work; `--all` or `-f word` show every line.
- `--text` also lists static text (labels, status lines). `-f word` filters the output.
- Browser windows hide the web page part (use the web commands for the page); `--web` includes it.
- Apps without accessible controls (games, canvas UIs): `read w3` runs OCR and prints `o4 "Play" @960,420`. `--lang de` picks the OCR language (default English, `config ocr_lang de`).
- `shot w3` saves a small JPEG of just that window. Read it only if you really need pixels.

## Act
- `click d5` uses the control's own action in the background. The user's mouse and focus stay where they are. Wisply never brings a window to the front by itself (Windows).
- `click`, `type` and `press` wait until the app stops changing (up to 0.6 s) and answer with what appeared: `ok d5 (invoked in background) (+3 new, -1 gone)` and up to 5 new lines whose refs work right away. `-f word` adds a filtered look.
- `type d6 hello`, `type "Search" hello` (field by name) or `type focused hello` sets the value in the background. Without a value pattern it pastes via the clipboard (the window comes to the front for a moment, the clipboard is restored).
- Electron apps (Discord, Slack, Spotify, VS Code) always paste. `--tab` accepts an autocomplete (Discord `@name` mention) once the list has appeared, `--enter` sends: `type d6 "@name" --tab --enter`.
- `press ctrl+s` brings the window to the front for the key press, then gives focus back. `--stay` keeps the window in front, `--game` sends scan codes (DirectInput games). Shortcuts with Ctrl/Alt other than Ctrl+A/C/V/X/Z fail with a hint while the user is at the PC; `config focus away` lets them take the front when the user has been away for a minute.
- Key names: letters, digits, `f1`-`f24`, `enter tab esc space backspace delete insert home end pgup pgdn up down left right`, `plus minus comma period`, `numpad0`-`numpad9`, modifiers `ctrl shift alt win altgr`. Media keys need no window: `playpause nexttrack prevtrack stop volumeup volumedown mute`.
- `click o4` or `click "Play"` (OCR text) does a real mouse click: it waits until the user has not moved the mouse for 0.3 s, clicks and puts the cursor back exactly.
- `scroll d9` / `scroll up` scroll a list or the window. `drag d3 d9` drags with the real mouse (cursor restored).
- `wait "Done" w3` waits until an app shows a text (control names and values, else OCR). `--gone` waits until it disappears, `--timeout 60s` (default 30 s).
- A pointer shows where Wisply acts; `config pointer off` hides it.

## Keyboard shortcuts
- Wisply remembers shortcuts per app and site: the ones the app shows, the ones that worked (`press ctrl+k` that opened something) and a few well-known ones.
- The first look at an app or site has one line: `keys: ctrl+k quick switcher | ctrl+/ list of all shortcuts | alt+up channel above (wisply keys)`.
- `wisply keys` (current app or site), `keys discord`, `keys windows` (system-wide), `keys obsidian -f search`. `keys add discord.exe "ctrl+k=quick switcher"`, `keys rm discord.exe ctrl+k`.
- A shortcut is often one step where clicking takes three: Discord `press ctrl+k`, `type focused "Luis" --enter` opens the DM.

## Limits
- Windows that run as administrator cannot be controlled from a normal terminal (Windows UIPI). Wisply says so.
- Some apps expose few controls (older games, custom-drawn UIs): use `read` + `click o<n>`.
