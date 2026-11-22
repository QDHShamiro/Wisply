# Wisply: memory, to-dos and the code map

## Recall and keep
- `recall "words"` searches notes, the vault and session notes by words and meaning; notes of the current project rank first, `--all` shows other projects with `[project]`, `recall 42` prints one note in full, `--max n` limits the hits.
- `remember "fact"` keeps a finding, `forget <id>` deletes one note.
- `todo` lists the open steps, `todo add "task" --list x`, `todo done|doing|drop|open 12`, `todo note 12 "text"`.
- `wiki` lists drafts in the vault inbox, `wiki show 1`, `wiki accept|drop 1`.
- By itself: the session start prints a summary, a prompt can show up to 2 fitting notes of the same project (`Wisply memory may help`), and a session is learned once it has been quiet for 10 minutes or ended.

## Code, before reading many files
- `code` (overview of the project), `code Name` (find a function or class by name), `code "what it does"` (closest by name and body), `code callers Name`, `code body Name`, `code outline file`, `code impact [--base main]` (what the changes against HEAD or a base touch). `--project <path>` for another folder.
- `peek a.rs b.rs src/ [--outline] [--head 10]` gives size, lines and advice (`whole`, `parts`, `grep`, `skip`) for all of them in one call. Read `parts` files with offset and limit, leave `skip` files (lock files, build output, binaries).
- A tool output that Wisply shortened names a ref: `more <ref>` shows it in full, `--lines a-b` or `--grep "word|other"` a part.

## The local model
- Store: `~/.wisply/memory/memory.db` (SQLite) on this computer.
- Learning uses a local model that Wisply downloads by itself with a checksum check; it never runs or downloads while a game runs. `memory model` shows it or starts the download, `config models_dir <folder>` puts the files on another drive.
- An own model instead: `config memory_api http://127.0.0.1:9930/v1` + `config memory_api_model <name>` + `config memory_api_embed <name>` (any OpenAI-compatible server), or own files: `config memory_server <llama-server>`, `config memory_model <gguf>`, `config memory_embed <gguf>`.
- `memory learn` learns the waiting sessions now, `memory import claude-mem` moves notes, summaries and open to-dos over (safe to repeat), `memory off` stops the hooks, `config memory_learn off` keeps the notes but learns nothing new.
