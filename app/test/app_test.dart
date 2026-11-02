import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:wisply_app/api/models.dart';
import 'package:wisply_app/api/store.dart';
import 'package:wisply_app/app.dart';
import 'package:wisply_app/controller.dart';
import 'package:wisply_app/services/notifier.dart';

import '../tool/mock_server.dart';

class _Sink implements EventSink {
  final shown = <PhoneEvent>[];

  @override
  Future<void> show(PhoneEvent event) async => shown.add(event);
}

Future<void> settle(WidgetTester tester, Finder finder) async {
  for (var i = 0; i < 60; i++) {
    await tester.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 50)),
    );
    await tester.pump();
    if (finder.evaluate().isNotEmpty) return;
  }
  fail('never showed: $finder');
}

Future<void> pullRefresh(WidgetTester tester) async {
  await tester.tap(find.byIcon(Icons.refresh));
  await tester.pump();
}

Future<void> unmount(WidgetTester tester, AppController c) async {
  await tester.runAsync(() async {
    await Future<void>.delayed(const Duration(milliseconds: 300));
  });
  await tester.pump();
  await tester.pumpWidget(const SizedBox());
  await tester.runAsync(() async {
    await Future<void>.delayed(const Duration(milliseconds: 100));
  });
  await tester.pump();
  c.dispose();
}

void main() {
  late MockWisplyServer server;
  late MemoryPairingStore store;
  late _Sink sink;
  late AppController controller;

  setUp(() async {
    server = MockWisplyServer();
    store = MemoryPairingStore();
    sink = _Sink();
    controller = AppController(
      store: store,
      sink: sink,
      useBackground: false,
      deviceName: 'testhandy',
      clientTimeout: null,
      httpFactory: server.client,
    );
  });

  tearDown(() => server.stop());

  Future<void> pairViaLink(WidgetTester tester) async {
    await tester.runAsync(controller.load);
    await tester.pumpWidget(WisplyApp(controller: controller, scanner: false));
    await tester.pump();
    await tester.enterText(find.byType(TextField), server.pairLink);
    await tester.tap(find.text('Koppeln'));
    await settle(tester, find.text('Plugin bauen'));
  }

  testWidgets('pairing with a wrong code shows an error and stays', (
    tester,
  ) async {
    await tester.runAsync(controller.load);
    await tester.pumpWidget(WisplyApp(controller: controller, scanner: false));
    await tester.pump();
    final bad = server.pairLink.replaceFirst('TESTCODE', 'WRONG');
    await tester.enterText(find.byType(TextField), bad);
    await tester.tap(find.text('Koppeln'));
    await settle(tester, find.text('Code abgelaufen oder schon benutzt'));
    expect(controller.paired, isFalse);
    await unmount(tester, controller);
  });

  testWidgets('junk text is rejected without a request', (tester) async {
    await tester.runAsync(controller.load);
    await tester.pumpWidget(WisplyApp(controller: controller, scanner: false));
    await tester.pump();
    await tester.enterText(find.byType(TextField), 'hallo');
    await tester.tap(find.text('Koppeln'));
    await tester.pump();
    expect(find.text('Das ist kein Wisply-Code'), findsOneWidget);
    expect(server.codeUsed, isFalse);
    await unmount(tester, controller);
  });

  testWidgets('pair, board with status chips, detail, reply, stop', (
    tester,
  ) async {
    await pairViaLink(tester);
    expect((await store.load())!.token, server.token);
    expect(find.text('Läuft'), findsOneWidget);
    expect(find.text('Wartet auf dich'), findsOneWidget);
    expect(find.text('Fertig'), findsOneWidget);

    await tester.tap(find.text('Doku schreiben'));
    await tester.pump();
    await settle(tester, find.byType(TextField));
    await tester.enterText(find.byType(TextField), 'README zuerst');
    await tester.tap(find.byIcon(Icons.send));
    await settle(tester, find.text('Antwort gesendet'));
    expect(server.replies['a2'], ['README zuerst']);
    tester
        .state<ScaffoldMessengerState>(find.byType(ScaffoldMessenger))
        .clearSnackBars();
    await tester.pumpAndSettle();

    await tester.tap(find.byIcon(Icons.stop_circle_outlined));
    await tester.pumpAndSettle();
    expect(find.text('Agent stoppen?'), findsOneWidget);
    await tester.tap(find.text('Abbrechen'));
    await tester.pumpAndSettle();
    expect(server.stops, isEmpty);

    await tester.tap(find.byIcon(Icons.stop_circle_outlined));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Stoppen'));
    await tester.pump();
    await settle(tester, find.text('Stopp gesendet'));
    expect(server.stops, ['a2']);
    await unmount(tester, controller);
  });

  testWidgets('pull to refresh picks up a new state', (tester) async {
    await pairViaLink(tester);
    server.agents[0]['status'] = 'error';
    await pullRefresh(tester);
    await settle(tester, find.text('Fehler'));
    await unmount(tester, controller);
  });

  testWidgets('dragging the list down refreshes it', (tester) async {
    await pairViaLink(tester);
    server.agents[0]['status'] = 'error';
    await tester.drag(find.byType(ListView).first, const Offset(0, 500));
    for (var i = 0; i < 6; i++) {
      await tester.pump(const Duration(milliseconds: 300));
    }
    await settle(tester, find.text('Fehler'));
    await unmount(tester, controller);
  });

  testWidgets('settings unpair returns to the pair screen', (tester) async {
    await pairViaLink(tester);
    await tester.tap(find.byIcon(Icons.settings_outlined));
    await tester.pump();
    await settle(tester, find.textContaining('Wisply 3.1.0'));
    await tester.tap(find.text('Gerät entkoppeln'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Entkoppeln'));
    await tester.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 100)),
    );
    await tester.pumpAndSettle();
    expect(find.text('Mit PC koppeln'), findsOneWidget);
    expect(await store.load(), isNull);
    await unmount(tester, controller);
  });

  testWidgets('a revoked token drops back to pairing', (tester) async {
    await pairViaLink(tester);
    server.revoked = true;
    await pullRefresh(tester);
    await settle(tester, find.text('Mit PC koppeln'));
    expect(await store.load(), isNull);
    await unmount(tester, controller);
  });

  testWidgets('an incoming event raises a notification and refreshes', (
    tester,
  ) async {
    await pairViaLink(tester);
    await tester.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 300)),
    );
    server.emit('waiting', 'Agent wartet', 'Doku braucht Antwort');
    await tester.runAsync(
      () => Future<void>.delayed(const Duration(milliseconds: 400)),
    );
    await tester.pump();
    expect(sink.shown.single.title, 'Agent wartet');
    await unmount(tester, controller);
  });

  testWidgets('tablet width shows list and detail side by side', (
    tester,
  ) async {
    tester.view.physicalSize = const Size(1200, 800);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.reset);
    await pairViaLink(tester);
    expect(find.text('Agent wählen'), findsOneWidget);
    await tester.tap(find.text('Aufräumen'));
    await tester.pump();
    await settle(tester, find.byType(TextField));
    expect(find.text('Agent wählen'), findsNothing);
    expect(find.text('Plugin bauen'), findsOneWidget);
    await unmount(tester, controller);
  });
}
