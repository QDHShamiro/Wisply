import 'package:flutter_local_notifications/flutter_local_notifications.dart';

import '../api/models.dart';

abstract class EventSink {
  Future<void> show(PhoneEvent event);
}

class LocalNotifier implements EventSink {
  final _plugin = FlutterLocalNotificationsPlugin();
  bool _ready = false;

  Future<void> init() async {
    if (_ready) return;
    await _plugin.initialize(
      settings: const InitializationSettings(
        android: AndroidInitializationSettings('@mipmap/ic_launcher'),
        iOS: DarwinInitializationSettings(
          requestAlertPermission: false,
          requestBadgePermission: false,
          requestSoundPermission: false,
        ),
      ),
    );
    _ready = true;
  }

  Future<void> requestPermission() async {
    await init();
    await _plugin
        .resolvePlatformSpecificImplementation<
          AndroidFlutterLocalNotificationsPlugin
        >()
        ?.requestNotificationsPermission();
    await _plugin
        .resolvePlatformSpecificImplementation<
          IOSFlutterLocalNotificationsPlugin
        >()
        ?.requestPermissions(alert: true, badge: true, sound: true);
  }

  @override
  Future<void> show(PhoneEvent event) async {
    await init();
    await _plugin.show(
      id: event.seq & 0x7fffffff,
      title: event.title,
      body: event.body,
      notificationDetails: const NotificationDetails(
        android: AndroidNotificationDetails(
          'wisply_events',
          'Wisply',
          channelDescription: 'Meldungen vom PC',
          importance: Importance.high,
          priority: Priority.high,
        ),
        iOS: DarwinNotificationDetails(),
      ),
      payload: event.agentId,
    );
  }
}
