# wisply call: jede KI kann dich anrufen

Kostenlos, ohne Account, ohne Tailscale. Standardweg ist **ntfy** mit Priorität 5: Das Handy klingelt und vibriert auch gesperrt und im Nicht-stören-Modus (wenn du es in der App erlaubst), und die Nachricht hat Knöpfe (Ja / Nein).

## Einmal einrichten (2 Minuten)

1. `wisply call setup` legt ein geheimes Topic an und kopiert es in die Zwischenablage (es wird nie ausgegeben).
2. ntfy-App (Android oder iOS) installieren, das Topic abonnieren (Server `ntfy.sh`, Topic aus der Zwischenablage).
3. Android: in der App für das Topic "Nicht stören überschreiben" erlauben, dann klingelt Priorität 5 auch gesperrt.
4. `wisply call test` – das Handy muss klingeln.

Extras, alles optional und per `wisply call set key=wert`:

| Kanal | Wofür | Einrichtung |
|---|---|---|
| `callmebot` | echter Telefonanruf über Telegram (liest den Text vor, einseitig) | in Telegram `/start` an @CallMeBot_txtbot schicken, dann `call set callmebot_user=@deinname channels=ntfy,callmebot` |
| `discord` | Ping in einen Kanal | Webhook-URL als `WISPLY_CALL_DISCORD_WEBHOOK=...` in `~/.config/watch/.env`, dazu `call set discord_user_id=<deine ID>` |
| `app` | Wisply-Handy-App | läuft über den Daemon (`notify event`), nur normale Wichtigkeit |

## Fragen und Antworten

```
wisply call ask "Soll ich deployen?" --choices ja,nein --wait 120
```

Wartet bis zur Antwort und gibt sie **nur** auf stdout aus (`ja`). Fehler: `err: ...`, Exit 1 (z. B. niemand geht ran). Antworten kannst du:

- per **Knopf** in der ntfy-Nachricht,
- per **Text**: an das Antwort-Topic (`<topic>-re`) schreiben, `<id> <Antwort>` oder nur die Antwort (gilt dann für die jüngste offene Frage),
- per **Sprache** im Discord-Call (siehe unten),
- von Hand: `wisply call answer <id|last> "ja"`.

Ohne `--choices` ist jeder Text eine gültige Antwort. Mit Auswahl zählt "klar", "mach" usw. als ja, "lieber nicht" als nein (Wörter in `yes_words` / `no_words`).

Weitere Verben: `call notice "Build fertig"` (klingelt, wartet nicht), `call list`, `call result <id>`, `call status`, `call config`.

Auch als MCP-Tool: `call ask "..." --choices ja,nein --wait 90` über das Wisply-MCP, und in Codemode als `wisply\`call ask ...\``.

## Sprachgespräch im Discord-Call

`wisply call set voice=on voice_target="<Call-Name>" voice_join_url=<Link zum Sprachkanal>`

Setzt `wisply voice` voraus (Audiogeräte und Keys, siehe `wisply voice`). Beim Klingeln bekommt die Nachricht einen Knopf "Call beitreten"; Wisply geht in den Call, im eigenen Call-Modus (keine freie Plauderei): liest die Frage mit den Antworten vor, versteht dein "ja", "nein" oder die Auswahl, bestätigt und legt nach `voice_linger_s` Sekunden Stille auf.

## Claude-Hook: ruft nur an, wenn du weg bist

`wisply call away on` (oder `off`). Automatisch gilt "weg" nach `away_after_min` (15) Minuten ohne Maus und Tastatur. Nachts (`quiet_from`/`quiet_to`, 23:00 bis 07:00) klingelt nichts, Hinweise haben einen Mindestabstand (`min_gap_s`, 300).

`wisply call hook-config` druckt den fertigen `hooks`-Block für `settings.json` (PermissionRequest, Notification idle_prompt, Stop). Das Hook ist ein dünner Zusatz:

- **Rechteabfrage** (z. B. `Bash`): Frage mit Ja/Nein ans Handy; die Antwort wird die Entscheidung. Keine Antwort in `hook_wait_s` (90 s) oder irgendein Fehler: ganz normaler Dialog am PC.
- **Fertig** und **wartet auf dich**: nur ein Hinweis, blockiert nie.

## Morgen-Briefing

`wisply call briefing` (`--dry` zeigt nur den Text) fasst offene To-dos, laufende und wartende Agents und neue Albion-Nachrichten (nur lesen) zusammen. Täglich ohne sichtbares Fenster: `wisply call schedule on 07:30`, aus mit `schedule off`, Stand mit `schedule status`. Gmail und Kalender sind nicht drin (bräuchten einen neuen Login, siehe `SPEC-call.md`).

## Einstellungen und Sicherheit

`wisply call config` listet alles, `call set key=wert` ändert es. Texte und Zeiten stehen in `~/.wisply/call/config.json`, nicht im Code. Topic, Telegram-Name und Discord-ID werden nie angezeigt. Texte laufen vor dem Senden durch den Wisply-Scrubber (Schlüssel werden geschwärzt). Ein Fremder, der dein Topic kennt, könnte Antworten einschleusen – deshalb ist es 20 zufällige Zeichen lang und geheim.
