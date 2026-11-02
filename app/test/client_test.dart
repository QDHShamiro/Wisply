import 'package:flutter_test/flutter_test.dart';
import 'package:wisply_app/api/client.dart';
import 'package:wisply_app/api/models.dart';
import 'package:wisply_app/api/store.dart';
import 'package:wisply_app/services/event_loop.dart';
import 'package:wisply_app/services/notifier.dart';

import '../tool/mock_server.dart';

class RecordingSink implements EventSink {
  final shown = <PhoneEvent>[];

  @override
  Future<void> show(PhoneEvent event) async => shown.add(event);
}

void main() {
  late MockWisplyServer server;

  setUp(() async {
    server = MockWisplyServer();
    await server.start();
  });

  tearDown(() => server.stop());

  group('PairInfo', () {
    test('accepts a wisply pair link', () {
      final info = PairInfo.parse(server.pairLink);
      expect(info, isNotNull);
      expect(info!.url, server.url);
      expect(info.code, 'TESTCODE');
    });

    test('rejects foreign links and junk', () {
      expect(PairInfo.parse('https://example.com'), isNull);
      expect(PairInfo.parse('wisply://pair?u=http%3A%2F%2Fa'), isNull);
      expect(PairInfo.parse('wisply://pair?u=ftp%3A%2F%2Fa&c=x'), isNull);
      expect(PairInfo.parse('wisply://other?u=http%3A%2F%2Fa&c=x'), isNull);
      expect(PairInfo.parse(''), isNull);
    });
  });

  group('WisplyClient', () {
    test('pair gives a token once and the code dies afterwards', () async {
      final anon = WisplyClient(baseUrl: server.url);
      final res = await anon.pair('TESTCODE', 'handy');
      expect(res.token, server.token);
      expect(res.name, 'handy');
      expect(
        () => anon.pair('TESTCODE', 'handy'),
        throwsA(isA<ApiException>()),
      );
    });

    test('lists agents, detail has a tail, reply and stop arrive', () async {
      final c = WisplyClient(baseUrl: server.url, token: server.token);
      final list = await c.agents();
      expect(list.map((a) => a.status), ['running', 'waiting', 'done']);
      final detail = await c.agent('a1');
      expect(detail.tail, hasLength(30));
      await c.reply('a1', 'weiter');
      await c.stop('a1');
      expect(server.replies['a1'], ['weiter']);
      expect(server.stops, ['a1']);
      expect((await c.status()).version, '3.1.0');
    });

    test('a wrong token is a 401 and says nothing else', () async {
      final c = WisplyClient(baseUrl: server.url, token: 'nope');
      await expectLater(
        c.agents(),
        throwsA(isA<ApiException>().having((e) => e.unauthorized, 'u', true)),
      );
    });

    test('an unreachable PC is reported, not thrown raw', () async {
      final c = WisplyClient(baseUrl: 'http://127.0.0.1:1', token: 'x');
      await expectLater(
        c.agents(),
        throwsA(isA<ApiException>().having((e) => e.status, 's', 0)),
      );
    });

    test('events stream replays from since and delivers live ones', () async {
      server.emit('done', 'Fertig', 'a', agentId: 'a1');
      server.emit('waiting', 'Wartet', 'b', agentId: 'a2');
      final c = WisplyClient(baseUrl: server.url, token: server.token);
      final stream = c.events(since: 1);
      final got = <PhoneEvent>[];
      final sub = stream.listen(got.add);
      await Future<void>.delayed(const Duration(milliseconds: 300));
      server.emit('ci_red', 'CI rot', 'c');
      await Future<void>.delayed(const Duration(milliseconds: 300));
      await sub.cancel();
      expect(got.map((e) => e.seq), [2, 3]);
      expect(got.first.agentId, 'a2');
    });
  });

  group('EventLoop', () {
    test('shows each event once and remembers the last seq', () async {
      final store = MemoryPairingStore()
        ..value = Pairing(url: server.url, token: server.token, name: 'h');
      final sink = RecordingSink();
      server.emit('done', 'Fertig', 'x');
      final loop = EventLoop(
        store: store,
        sink: sink,
        minDelay: const Duration(milliseconds: 50),
      );
      loop.run();
      await Future<void>.delayed(const Duration(milliseconds: 300));
      server.emit('waiting', 'Wartet', 'y');
      await Future<void>.delayed(const Duration(milliseconds: 300));
      loop.stop();
      expect(sink.shown.map((e) => e.seq), [1, 2]);
      expect(await store.lastSeq(), 2);
    });

    test('a revoked token wipes the pairing', () async {
      final store = MemoryPairingStore()
        ..value = Pairing(url: server.url, token: 'bad', name: 'h');
      var unpaired = false;
      final loop = EventLoop(
        store: store,
        sink: RecordingSink(),
        minDelay: const Duration(milliseconds: 50),
        onUnpaired: () => unpaired = true,
      );
      await loop.run();
      expect(unpaired, isTrue);
      expect(await store.load(), isNull);
    });

    test('reconnects after the server drops the stream', () async {
      final store = MemoryPairingStore()
        ..value = Pairing(url: server.url, token: server.token, name: 'h');
      final sink = RecordingSink();
      final loop = EventLoop(
        store: store,
        sink: sink,
        minDelay: const Duration(milliseconds: 50),
      );
      loop.run();
      await Future<void>.delayed(const Duration(milliseconds: 200));
      final port = server.port;
      await server.stop();
      server = MockWisplyServer();
      await server.start(port: port);
      server.emit('done', 'Fertig', 'x');
      await Future<void>.delayed(const Duration(milliseconds: 600));
      loop.stop();
      expect(sink.shown.map((e) => e.seq), [1]);
    });
  });
}
