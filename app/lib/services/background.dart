import 'dart:io';

import 'package:flutter_foreground_task/flutter_foreground_task.dart';

import '../api/store.dart';
import 'event_loop.dart';
import 'notifier.dart';

@pragma('vm:entry-point')
void backgroundEntry() {
  FlutterForegroundTask.setTaskHandler(_EventTaskHandler());
}

class _EventTaskHandler extends TaskHandler {
  EventLoop? _loop;

  @override
  Future<void> onStart(DateTime timestamp, TaskStarter starter) async {
    final notifier = LocalNotifier();
    await notifier.init();
    final loop = EventLoop(
      store: SecurePairingStore(),
      sink: notifier,
      onUnpaired: () => FlutterForegroundTask.stopService(),
    );
    _loop = loop;
    loop.run();
  }

  @override
  void onRepeatEvent(DateTime timestamp) {}

  @override
  Future<void> onDestroy(DateTime timestamp, bool isTimeout) async {
    _loop?.stop();
  }
}

class Background {
  static bool get supported => Platform.isAndroid;

  static void configure() {
    if (!supported) return;
    FlutterForegroundTask.init(
      androidNotificationOptions: AndroidNotificationOptions(
        channelId: 'wisply_service',
        channelName: 'Wisply Verbindung',
        channelDescription: 'Hält die Verbindung zum PC offen',
        onlyAlertOnce: true,
      ),
      iosNotificationOptions: const IOSNotificationOptions(
        showNotification: false,
        playSound: false,
      ),
      foregroundTaskOptions: ForegroundTaskOptions(
        eventAction: ForegroundTaskEventAction.nothing(),
        autoRunOnBoot: true,
        autoRunOnMyPackageReplaced: true,
        allowWakeLock: true,
        allowWifiLock: true,
      ),
    );
  }

  static Future<void> start() async {
    if (!supported) return;
    if (await FlutterForegroundTask.isRunningService) return;
    await FlutterForegroundTask.startService(
      serviceId: 4784,
      notificationTitle: 'Wisply',
      notificationText: 'Verbunden mit dem PC',
      callback: backgroundEntry,
    );
  }

  static Future<void> stop() async {
    if (!supported) return;
    if (await FlutterForegroundTask.isRunningService) {
      await FlutterForegroundTask.stopService();
    }
  }
}
