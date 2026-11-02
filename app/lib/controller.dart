import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:http/http.dart' as http;

import 'api/client.dart';
import 'api/models.dart';
import 'api/store.dart';
import 'services/background.dart';
import 'services/event_loop.dart';
import 'services/notifier.dart';

class AppController extends ChangeNotifier {
  AppController({
    required this.store,
    required this.sink,
    this.useBackground = true,
    this.deviceName = 'handy',
    this.clientTimeout = const Duration(seconds: 12),
    this.httpFactory,
  });

  final PairingStore store;
  final EventSink sink;
  final bool useBackground;
  final String deviceName;
  final Duration? clientTimeout;
  final http.Client Function()? httpFactory;

  Pairing? pairing;
  bool loaded = false;
  EventLoop? _loop;
  final eventTicks = ValueNotifier<int>(0);

  bool get paired => pairing != null;

  WisplyClient? get client {
    final p = pairing;
    if (p == null) return null;
    return _build(p);
  }

  WisplyClient _build(Pairing p) => WisplyClient(
    baseUrl: p.url,
    token: p.token,
    timeout: clientTimeout,
    clientFactory: httpFactory,
  );

  Future<void> load() async {
    pairing = await store.load();
    loaded = true;
    notifyListeners();
    if (pairing != null) await _startEvents();
  }

  Future<void> pair(PairInfo info) async {
    final res = await WisplyClient(
      baseUrl: info.url,
      timeout: clientTimeout,
      clientFactory: httpFactory,
    ).pair(info.code, deviceName);
    final p = Pairing(url: info.url, token: res.token, name: res.name);
    await store.save(p);
    pairing = p;
    notifyListeners();
    await _startEvents();
  }

  Future<void> unpair() async {
    _loop?.stop();
    _loop = null;
    if (useBackground) await Background.stop();
    await store.clear();
    pairing = null;
    notifyListeners();
  }

  void handleUnauthorized() {
    unpair();
  }

  Future<void> _startEvents() async {
    if (useBackground && Background.supported) {
      final sink = this.sink;
      if (sink is LocalNotifier) await sink.requestPermission();
      await Background.start();
      return;
    }
    if (useBackground && Platform.isIOS) {
      final sink = this.sink;
      if (sink is LocalNotifier) await sink.requestPermission();
    }
    _loop?.stop();
    final loop = EventLoop(
      store: store,
      sink: sink,
      builder: (p) => _build(p),
      onEvent: (_) => eventTicks.value++,
      onUnpaired: () {
        if (pairing != null) unpair();
      },
    );
    _loop = loop;
    loop.run();
  }

  @override
  void dispose() {
    _loop?.stop();
    eventTicks.dispose();
    super.dispose();
  }
}
