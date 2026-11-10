# Wisply: video, sound and screen clips

## Watch a video
- `watch <link|file|e40>` answers in one call: title, length, chapters, 12 scenes in one image (`frames <path>`, open it with your file-read tool), the text on screen and the speech as a skim with times.
- Then zoom: `--from 4:05 --to 6:00` (denser frames, all speech of that part), `--at 1:23,2:45` (those frames large enough to read code), `--text` (screen text from the real frames), `--full` (all speech).
- The next call about the same video answers from the cache (24 h). Tutorials: read the chapters and the skim first, then `--at` the moments with code or settings.

## Listen: what a video says
- `listen` (the playing video of the current tab), `listen e40`, `listen https://vm.tiktok.com/...` (no browser needed) or `listen C:/clip.mp4`. Answer: `transcript (en, groq whisper-large-v3-turbo, 5.8 s):` and lines like `0:05 the cool thing about these guys`.
- Cheapest first: captions on the page, then captions via yt-dlp, then the audio via yt-dlp, then a live recording in the tab (`--live`, or when the site cannot be downloaded). The live recording plays the video muted for as long as the part lasts (at most 5 minutes) and puts time and pause back.
- Speech-to-text uses the Groq or OpenAI key from `GROQ_API_KEY`/`OPENAI_API_KEY` or `~/.config/watch/.env` (the sound goes to that service); with `--local` or `config audio local` it runs on this computer with faster-whisper (installed once, ~150 MB).
- `--from 1:30 --to 2:00` for a part, `--lang de` if the language is known. Music with singing can be misheard (the header then says `mostly music`).
- `frames e40 --audio` puts the pictures and the words of the same part into one answer. `--small` makes the sheet 768 px wide.

## Film the screen
- `shot screen --seconds 10` (or `shot w3 --seconds 10`) films the screen or a window: an MP4 (up to 60 s) to send to the user. A window is filmed by itself, also while other windows cover it (`--screen-area` films its screen area instead).
