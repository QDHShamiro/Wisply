import 'package:flutter/material.dart';

import 'api/store.dart';
import 'app.dart';
import 'controller.dart';
import 'services/background.dart';
import 'services/notifier.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  Background.configure();
  final notifier = LocalNotifier();
  await notifier.init();
  final controller = AppController(store: SecurePairingStore(), sink: notifier);
  controller.load();
  runApp(WisplyApp(controller: controller));
}
