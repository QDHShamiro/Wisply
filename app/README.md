# Wisply phone app

Flutter app for Android, iPhone and tablets. It shows the agents running on your PC, lets you answer or stop them and
rings your phone when one is done or waits for you. No cloud, no Firebase, no account: the phone talks straight to the
Wisply daemon on your PC over [Tailscale](https://tailscale.com). The contract is "Wisply Phone API v1" (`/v1/agents`,
`/v1/events`, `/v1/status`, `/v1/pair`).

Screens: pair (scan the QR), board (agents with status colours, pull to refresh), agent detail (last lines, reply field,
stop with confirmation), settings (PC status, unpair). On a tablet or a wide window the board and the detail sit side by
side.

## Set up Tailscale

1. Install Tailscale on the PC and on the phone and sign in to the same tailnet on both.
2. Check that the phone reaches the PC: open `http://<tailscale-ip-of-pc>:47841/` in the phone browser (a 401 or 404 is
   fine, it only has to answer).
3. The daemon listens only on `127.0.0.1` and the Tailscale addresses, never on your normal network.

## Pair the phone

1. On the PC: `wisply mobile pair --name handy`. A QR code appears (code valid 10 minutes, one use).
2. In the app: scan it. Without a camera, paste the `wisply://pair?...` link instead.
3. The token goes into the Android Keystore or the iOS Keychain. Revoke it any time with `wisply mobile revoke handy`
   on the PC, or "Gerät entkoppeln" in the app settings.

## Notifications

- Android: after pairing a foreground service keeps one Server-Sent-Events connection to the PC open and raises local
  notifications. It reconnects by itself and starts again after a reboot. Allow notifications, and exclude the app from
  battery optimisation when the system asks, otherwise some phones kill the service.
- iPhone: iOS does not allow a permanent connection without Apple push. Notifications arrive while the app is open and
  for a short time after you leave it. Open the app to catch up; missed events are replayed from the last one you saw.

## Install on Android

1. Download `wisply-app-android` from the latest `app` workflow run on GitHub (Actions, then Artifacts) and unzip it.
2. Copy `app-release.apk` to the phone, open it and allow "install unknown apps" for your file manager when asked.
3. Updates: install the new APK over the old one, pairing stays.

The APK is signed with the debug key, which is fine for sideloading on your own phone.

## Install on iPhone (Sideloadly, every 7 days)

Apple only lets a free developer account run a self-signed app for 7 days, so the app has to be signed again every week.

1. Download `wisply-app-ios-unsigned` from the latest `app` workflow run and unzip it to get `wisply-app-unsigned.ipa`.
2. Install [Sideloadly](https://sideloadly.io) on the PC and connect the iPhone by cable (trust the computer).
3. Drag the IPA into Sideloadly, enter your Apple ID, press Start.
4. On the iPhone: Settings, General, VPN & Device Management, trust your Apple ID. Turn on Developer Mode if asked.
5. Repeat step 3 every 7 days (before the app stops opening). Sideloadly can refresh automatically while the PC and the
   phone are on the same network. Your pairing stays across re-signing with the same Apple ID.

## Build it yourself

```
cd app
flutter pub get
flutter analyze
flutter test
flutter build apk --release
```

`tool/mock_server.dart` is a stand-in for the daemon. `dart run tool/mock_server.dart` serves the v1 API on port 47841
with three made-up agents and prints a pairing link; the widget tests use the same mock in memory.

CI (`.github/workflows/app.yml`) runs on every change in `app/`: analyze, tests, the APK on Ubuntu and the unsigned IPA
on macOS, both as artifacts.
