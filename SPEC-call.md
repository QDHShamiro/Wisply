# SPEC: wisply call

Eine kostenlose, eingebaute Art, wie jede KI QDHShamiro anruft. Dieses Dokument hält die Entscheidungen fest und den Plan für den Sekretär (nicht gebaut).

## Klingelweg (gebaut)

| Kanal | Kosten | klingelt gesperrt | Antwort zurück | Entscheidung |
|---|---|---|---|---|
| ntfy.sh, Priorität 5 | 0, kein Account | ja (Android: Nicht-stören-Override pro Topic), iOS: zeitkritisch | Knöpfe (HTTP-Action auf Antwort-Topic) und Text | **Standard** |
| Telegram-Anruf über CallMeBot | 0, einmal `/start` an @CallMeBot_txtbot | ja, echter Anruf, Text wird vorgelesen | nein (einseitig, Text ≤ 256 Zeichen) | Zusatz (`callmebot`) |
| Discord-Ping (Webhook) | 0 | nur normale Benachrichtigung | per Text in Discord, hier nicht ausgewertet | Zusatz (`discord`) |
| Wisply-Handy-App | 0 | nur Wichtigkeit high, Antworten nur für Agents | eingeschränkt | Zusatz (`app`) |

ntfy gewinnt, weil es als Einziges ohne Account und ohne Tailscale klingelt, Knöpfe für Freigaben hat und die Antwort per Poll zurückkommt (`GET /<topic>/json?poll=1&since=...`). CallMeBot ist der einzige "richtige" Anruf, kann aber nichts zurückhören.

Antwortweg: jede Frage ist eine Datei `~/.wisply/call/asks/<id>.json`; ntfy-Poller, Voice-Modus und `wisply call answer` schreiben dieselbe Datei, die erste Antwort gewinnt. `call ask` pollt, bis sie da ist, und druckt sie.

## Ideen, nicht gebaut

- **Gmail und Kalender im Briefing**: bräuchten einen neuen OAuth-Login. Ohne Login ginge es nur über den schon eingeloggten Browser (`wisply read https://mail.google.com`, Kalenderseite) – langsam und brüchig. Besser später als Routine mit dem Gmail-MCP, wenn QDHShamiro das will.
- **Telegram-Bot mit Antwort**: ein eigener Bot (BotFather, ein Token) würde Antworten per Chat erlauben. Braucht ein Konto und ein Geheimnis, deshalb nicht im Standardweg.
- **Fuzzy-Antworten im Voice-Modus**: heute zählt der Call-Modus Wörter (ja/nein/Auswahl, sonst freier Text). Ein freies Modell könnte unklare Sätze deuten ("mach mal, aber erst morgen").

## Sekretär: andere Leute anrufen (Plan)

Ziel: Wisply ruft eine fremde Nummer an, spricht mit einem Menschen (Termin, Nachfrage), meldet das Ergebnis. Kostenlos nur über die eigene FRITZ!Box 7490 als SIP-Registrar, weil der Anruf dann über den vorhandenen Festnetzanschluss geht (kostenlos nur mit Flatrate des Anschlusses; Mobilfunkziele kosten je nach Tarif). Hier wird nichts gebaut und nichts angerufen.

### Was gebraucht wird

1. **FRITZ!Box**: Heimnetz → Telefonie → Telefoniegeräte → Neues Gerät → Telefon → "LAN/WLAN (IP-Telefon)". Ergibt Benutzername und Passwort, Registrar `fritz.box:5060` (UDP, im LAN). Die 7490 erlaubt mehrere IP-Telefone und bietet G.711 (PCMA/PCMU), teils G.722. Ausgehende Nummer wählbar. Zugangsdaten nur in `~/.config/watch/.env` (`WISPLY_SIP_USER`, `WISPLY_SIP_PASS`), nie im Repo.
2. **SIP-User-Agent** in Wisply: registriert sich an der Box, wählt aus, beantwortet Re-INVITE/BYE, schickt RTP.
3. **Audio**: Mikrofon-Seite = Edge-TTS (vorhanden in `voice/mouth.rs`) → 8-kHz-Mono-PCM (Resampling mit `rubato`) → G.711 → RTP. Gegenseite = RTP → G.711 → 16 kHz → vorhandene ASR (Groq Whisper oder lokal) in `voice/run.rs`.
4. **Gesprächsführung**: Brain mit Rolle "Sekretär" (Ziel, erlaubte Aussagen, Abbruchwörter), harte Grenzen: nur eine freigegebene Nummer-Liste, vorher `wisply call ask` an QDHShamiro, am Anfang Ansage "Hier ist eine KI im Auftrag von …", Aufzeichnung nur als Text, Ergebnis per `call notice`.
5. **Recht und Anstand**: Anrufe an Fremde sind erst nach Freigabe pro Nummer und Ziel erlaubt; Ansage als KI Pflicht; keine Mehrfachanrufe; Testen nur mit einer eigenen zweiten Nummer.

### Rust-Crates (Stand 24.11.2026, crates.io)

| Crate | Rolle | Eindruck |
|---|---|---|
| `rsipstack` 0.8.8 | SIP-Stack (Transaktionen, Dialoge, Registrierung, UDP/TCP/TLS) | aktiv (Update 23.11.2026), ~100k Downloads, **erste Wahl** |
| `rvoip` 0.5.0 (`rvoip-sip`, `rvoip-sip-core`) | Komplettpaket SIP + RTP + Codecs, auch Voice-Agent-Beispiele | sehr jung, groß, gut zum Abgucken |
| `ezk-sip-ua` 0.9.1 / `ezk-sip-core` / `ezk-rtp` 0.2.1 | SIP-User-Agent plus RTP-Session | solide Alternative, SIP und RTP getrennt |
| `rsip` 0.4.0 | nur Parser und Generator | zu klein, ohne Zustandsmaschine |
| `oxideav-g711`, `nanoaudio-g711` | G.711 A-law/µ-law | reicht für die Box; sonst 40 Zeilen selbst |
| `rubato` 5.1 | Resampling 24/16 kHz ↔ 8 kHz | Standard |
| `audiopus` | Opus (nur nötig, wenn die Box nicht G.711 nimmt) | selten gebraucht |
| `pjsip-sys`, `baresip-rs` | C-Bibliotheken per FFI | nur wenn die reinen Rust-Stacks bei der Box zicken (NAT-frei im LAN, daher unwahrscheinlich) |

Empfehlung: `rsipstack` für Registrierung und INVITE, `ezk-rtp` oder eigenes RTP (12-Byte-Header, 20-ms-Pakete, G.711), `rubato` fürs Resampling. Kein C-Code, passt zu Wisply.

### Aufwand (grob)

| Schritt | Tage |
|---|---|
| FRITZ!Box-Gerät anlegen, mit `baresip`/Softphone von Hand testen | 0,5 |
| `sip.rs`: Register, Invite, Bye, Digest-Auth gegen die Box | 2 |
| `rtp.rs`: Senden/Empfangen, G.711, Jitter-Puffer, Resampling | 2 |
| Anbindung an Mouth/Ears/ASR (andere Audioquelle statt Discord/Cable) | 2 |
| Rolle, Freigabeliste, Ansage, Ergebnisbericht, Tests mit Mock-SIP | 2 |
| Echt-Test mit einer eigenen zweiten Nummer, Feinschliff (Echo, Sprechpausen) | 2 |
| **Summe** | **ca. 10 bis 11 Tage** |

Größtes Risiko: Echo und Sprechpausen (TTS-Latenz plus ASR ergeben etwa 1,5 bis 2 s pro Antwort) und die Box-Eigenheiten bei Rufnummern und Codecs. Zweitgrößtes: Rechtliches bei Anrufen an Fremde.
