# SPEC: wisply pc (PC-Optimizer)

Entschieden mit QDHShamiro am 2026-11-01 (grill-me). Für alle Nutzer, Windows + macOS + Linux gleich.

## Befehle
- `wisply pc` = Check: Note 0-100, die 3 größten Bremsen, je ein Satz "was tun" + Nummer. Beispiel:
  `pc 72/100 · RAM 87 % (brave 6.1 GB) · C: 18 GB frei · 6 Autostarts · ping hugosmp 34 ms`
- `wisply pc fix <n>` führt Vorschlag n aus. `wisply pc fix all` = alle sicheren.
- `wisply pc clean [--dry]` aufräumen. `wisply pc startup` Autostarts listen, `startup off <n>` abschalten.
- `wisply pc game` Spiel-Modus-Status. `wisply pc afk|play` von Hand umschalten.
- `wisply pc net [host]` Ping + welcher Prozess die Leitung belegt. `wisply pc disk` SMART/Gesundheit.
- `wisply pc updates` veraltete Grafiktreiber + offene System-Updates (nur anzeigen, Installieren nur nach Ja).
- `wisply pc on|off` Handeln an/aus. `wisply pc log [n]` was er getan hat.
- Statuszeile: Note + größte Bremse als kurzes Feld (vorhandene statusline erweitern, abschaltbar).

## Rechte (wichtigste Regel)
- Selbst (wenn `pc on`): Temp/Caches löschen, Priorität senken, RAM-Arbeitsspeicher leeren, Spiel-Modus.
- Nur nach Ja (Toast-Klick oder `pc fix n`): Prozesse beenden, Autostart abschalten, Löschen außerhalb Temp/Cache/Build-Müll, Treiber/Updates.
- Standard neue Nutzer: Anzeigen + Lag-Warnung an, Handeln aus bis `wisply pc on`. Beim Entwickler (Config-Schalter) an.

## Hintergrund (Daemon)
- Leiser Takt (z.B. 5 s), Ziel < 1 % CPU, < 15 MB RAM extra. Messen und in Tests belegen.
- Lag-Finder: RAM > 85 %, ein Prozess wächst stark, CPU > 80 % über 30 s, Platte 100 % aktiv über 30 s (HDD!). Ursache = konkreter Prozess.
- Meldung: System-Benachrichtigung (Windows-Toast, macOS Notification Center, Linux notify-send/D-Bus). Holt nie ein Fenster nach vorne, auch nicht über Vollbild-Spiele. Klick = Ja für den Vorschlag. Gleiche Meldung höchstens 1x pro 30 min.

## Spiel-Modus
- An, wenn ein Spiel läuft (vorhandene Spiel-Erkennung daemon/game.rs nutzen) UND Maus/Tastatur in den letzten 5 min benutzt. 5 min ohne Eingabe = AFK = gelockert. Eingabe = sofort wieder voll.
- Dann: Builds/Tests/Agents (cargo, rustc, gradle/java-Builds, node, claude-Agent-Sessions außer der Vordergrund-Sitzung) niedrigste Priorität, bei RAM-Knappheit kurz pausieren (suspend) und danach fortsetzen. Hintergrund (OneDrive, Suche/Indexer, Updates, Nutzer-Liste aus Config) leiser. Spiel höhere Priorität + Höchstleistungs-Energieplan. RAM ungenutzter Prozesse freigeben.
- Beim Ende alles exakt wiederherstellen (Prioritäten, Energieplan, pausierte Prozesse). Auch nach Absturz des Daemons: Zustand in Datei, beim Start wiederherstellen.

## Aufräumen
- Auto bei < 25 GB frei auf einem Laufwerk + 1x pro Woche wenn AFK/idle.
- Darf selbst: Temp + OS-Caches + Crash-Dumps; Build-Müll (target/, build/, node_modules, .gradle) in Projekten ohne Änderung seit 30 Tagen; Logs > 14 Tage (u.a. ~/agents-work/*.log|*.json, alte ~/.wisply/out); Paket-Caches (npm, pip, cargo registry, gradle caches).
- NIE: laufende/geöffnete Projekte, Git-Repos mit ungespeicherten Änderungen, `build/libs` (Live-Mod-Jars), Minecraft/ModrinthApp/.minecraft, Spielordner, ~/.claude, ~/.wisply/memory, Vault, Dokumente/Bilder/Downloads, alles mit Secrets. Liste in Config erweiterbar.
- Direkt löschen (kein Papierkorb), aber Protokoll: was, wo, wie groß (`pc log`). `--dry` zeigt nur.
- Nach Einführung DiskWatch-C beim Entwickler ablösen (nur Hinweis in RESULT, Task nicht selbst löschen).

## Autostart & Hintergrund
- Listen: Autostart-Einträge, geplante Aufgaben, Dienste von Dritten, Prozesse die Konsolen-Popups öffnen. Mit Größe der Bremse.
- Abschalten nur nach Ja, immer umkehrbar (`startup on <n>`), Original sichern.

## Regeln
- Null Kommentare, Config statt Hardcodes, Dateien < 500 Zeilen, Stil wie umgebender Code.
- Neue Crate `crates/wisply-pc` (plattformneutrale Logik + Tests) und dünne Anbindung in daemon/ + Hilfe-Text + README-Abschnitt + CHANGELOG.
- Tests: Logik (Regeln, Aufräum-Ausschlüsse, Wiederherstellen, AFK-Übergang) als Unit-Tests ohne echte Prozesse. CI muss auf allen 3 Systemen grün sein.

## Offen
- Toast-Klick = Ja ist nicht gebaut: die Benachrichtigung nennt `wisply pc fix <n>`, ein Klick tut nichts. Dafuer braucht
  Windows einen registrierten Protokoll-Handler/COM-Activator, macOS ein Notification-Helper-Bundle, Linux
  notify-send mit Aktionen (D-Bus). Alles drei kann ein Fenster nach vorne holen, deshalb bewusst weggelassen.
- Netz pro Prozess: das Betriebssystem liefert keine Zaehler pro Programm ohne Admin/ETW; `pc net` schaetzt aus offenen
  Verbindungen und Platten-/Netz-Bytes des Programms in 1,5 s.
- Konsolen-Popups: nur Heuristik (cmd/powershell/wscript mit ungewoehnlichem Elternprozess), keine Fenster-Pruefung.
- Dienste von Dritten: noch nicht gelistet (Autostarts und geplante Aufgaben ja); Abschalten braucht Admin.
- Linux Fenster-Vordergrund (xdotool) und Leerlauf (xprintidle) sind optional; fehlen sie, gilt Eingabe als aktiv und der
  Vordergrund wird nicht geschont. macOS: kein Energieplan, keine Vordergrund-Erkennung, kein Speicher-Freigeben.
- macOS und Linux sind nur kompiliert und mit Parser-Tests (echte Beispiel-Ausgaben) geprueft, nicht auf echten Geraeten.
- DiskWatch-C beim Entwickler nach Einfuehrung ablösen: nur Hinweis, Task nicht angefasst.
