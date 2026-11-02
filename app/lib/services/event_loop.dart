import 'dart:async';

import '../api/client.dart';
import '../api/models.dart';
import '../api/store.dart';
import 'notifier.dart';

typedef ClientBuilder = WisplyClient Function(Pairing pairing);

class EventLoop {
  EventLoop({
    required this.store,
    required this.sink,
    ClientBuilder? builder,
    this.minDelay = const Duration(seconds: 2),
    this.maxDelay = const Duration(seconds: 60),
    this.onEvent,
    this.onUnpaired,
  }) : _builder =
           builder ?? ((p) => WisplyClient(baseUrl: p.url, token: p.token));

  final PairingStore store;
  final EventSink sink;
  final ClientBuilder _builder;
  final Duration minDelay;
  final Duration maxDelay;
  final void Function(PhoneEvent event)? onEvent;
  final void Function()? onUnpaired;

  bool _running = false;
  Completer<void>? _wake;
  Completer<void>? _done;

  bool get running => _running;

  void stop() {
    _running = false;
    final wake = _wake;
    if (wake != null && !wake.isCompleted) wake.complete();
    final done = _done;
    if (done != null && !done.isCompleted) done.complete();
  }

  Future<void> run() async {
    if (_running) return;
    _running = true;
    var delay = minDelay;
    while (_running) {
      final pairing = await store.load();
      if (pairing == null) {
        onUnpaired?.call();
        break;
      }
      try {
        final since = await store.lastSeq();
        final client = _builder(pairing);
        final done = Completer<void>();
        _done = done;
        final sub = client
            .events(since: since)
            .asyncMap((event) async {
              delay = minDelay;
              await _handle(event);
            })
            .listen(
              null,
              onError: (Object e) {
                if (!done.isCompleted) done.completeError(e);
              },
              onDone: () {
                if (!done.isCompleted) done.complete();
              },
            );
        try {
          await done.future;
        } finally {
          await sub.cancel();
        }
      } on ApiException catch (e) {
        if (e.unauthorized) {
          await store.clear();
          onUnpaired?.call();
          break;
        }
      } catch (_) {}
      if (!_running) break;
      await _sleep(delay);
      delay = delay * 2 > maxDelay ? maxDelay : delay * 2;
    }
    _running = false;
  }

  Future<void> _handle(PhoneEvent event) async {
    final last = await store.lastSeq();
    if (event.seq <= last) return;
    await store.saveSeq(event.seq);
    await sink.show(event);
    onEvent?.call(event);
  }

  Future<void> _sleep(Duration d) async {
    final wake = Completer<void>();
    _wake = wake;
    final timer = Timer(d, () {
      if (!wake.isCompleted) wake.complete();
    });
    await wake.future;
    timer.cancel();
  }
}
