import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:http/http.dart' as http;
import 'package:http/testing.dart';

class MockWisplyServer {
  MockWisplyServer({this.pairCode = 'TESTCODE', this.token = 'tok-test'});

  final String pairCode;
  final String token;

  HttpServer? _server;
  bool codeUsed = false;
  bool revoked = false;
  final replies = <String, List<String>>{};
  final stops = <String>[];
  final _streams = <StreamController<List<int>>>[];
  final events = <Map<String, dynamic>>[];
  int _seq = 0;

  final agents = <Map<String, dynamic>>[
    {
      'id': 'a1',
      'name': 'Plugin bauen',
      'dir_short': 'wisply',
      'status': 'running',
      'since': '2026-11-02T10:00:00Z',
      'last_line': 'cargo test laeuft',
      'tokens': 12000,
      'cost_est': 0.42,
    },
    {
      'id': 'a2',
      'name': 'Doku schreiben',
      'dir_short': 'docs',
      'status': 'waiting',
      'since': '2026-11-02T09:30:00Z',
      'last_line': 'Soll ich README oder CHANGELOG zuerst machen?',
      'tokens': 4000,
      'cost_est': 0.1,
    },
    {
      'id': 'a3',
      'name': 'Aufräumen',
      'dir_short': 'tools',
      'status': 'done',
      'since': '2026-11-02T08:00:00Z',
      'last_line': 'Alles erledigt',
      'tokens': 800,
      'cost_est': 0.02,
    },
  ];

  int get port => _server!.port;
  String get url =>
      _server == null ? 'http://mock.local' : 'http://127.0.0.1:$port';
  String get pairLink =>
      'wisply://pair?u=${Uri.encodeComponent(url)}&c=$pairCode';

  Future<void> start({int port = 0}) async {
    _server = await HttpServer.bind(InternetAddress.loopbackIPv4, port);
    _server!.listen(_handle);
  }

  Future<void> stop() async {
    for (final c in List.of(_streams)) {
      await c.close();
    }
    _streams.clear();
    await _server?.close(force: true);
  }

  void emit(String kind, String title, String body, {String? agentId}) {
    final e = {
      'seq': ++_seq,
      'kind': kind,
      'title': title,
      'body': body,
      'agent_id': ?agentId,
    };
    events.add(e);
    for (final c in _streams) {
      c.add(utf8.encode('data: ${jsonEncode(e)}\n\n'));
    }
  }

  void _json(HttpResponse res, int code, Object? body) {
    res.statusCode = code;
    res.headers.contentType = ContentType.json;
    res.write(jsonEncode(body));
    res.close();
  }

  Future<void> _handle(HttpRequest req) async {
    final body = await utf8.decoder.bind(req).join();
    final path = req.uri.path;
    if (req.method == 'GET' &&
        path == '/v1/events' &&
        _authorized(req.headers.value('authorization'))) {
      req.response.headers.contentType = ContentType('text', 'event-stream');
      req.response.bufferOutput = false;
      final since = int.tryParse(req.uri.queryParameters['since'] ?? '') ?? 0;
      await req.response.flush();
      final sub = eventStream(since).listen(req.response.add);
      sub.onDone(() => req.response.close());
      req.response.done.whenComplete(sub.cancel);
      return;
    }
    final res = handle(
      req.method,
      path,
      req.headers.value('authorization'),
      body,
    );
    _json(req.response, res.status, res.body);
  }

  bool _authorized(String? auth) => !revoked && auth == 'Bearer $token';

  ({int status, Object? body}) handle(
    String method,
    String path,
    String? auth,
    String body,
  ) {
    if (method == 'POST' && path == '/v1/pair') {
      final j = jsonDecode(body) as Map;
      if (codeUsed || j['code'] != pairCode) return (status: 401, body: {});
      codeUsed = true;
      return (status: 200, body: {'token': token, 'name': j['device']});
    }
    if (!_authorized(auth)) return (status: 401, body: {});
    if (method == 'GET' && path == '/v1/agents') {
      return (status: 200, body: agents);
    }
    if (method == 'GET' && path == '/v1/status') {
      return (
        status: 200,
        body: {
          'version': '3.1.0',
          'pc': {'free_gb_c': 21.5, 'game_mode': false},
          'limit': {'hit': false, 'reset_at': null},
        },
      );
    }
    final m = RegExp(r'^/v1/agents/([^/]+)(/reply|/stop)?$').firstMatch(path);
    if (m != null) {
      final id = Uri.decodeComponent(m.group(1)!);
      final agent = agents.where((a) => a['id'] == id).firstOrNull;
      if (agent == null) return (status: 404, body: {});
      final tail = m.group(2);
      if (method == 'GET' && tail == null) {
        return (
          status: 200,
          body: {
            ...agent,
            'tail': [
              for (var i = 1; i <= 30; i++) 'Zeile $i von ${agent['name']}',
            ],
          },
        );
      }
      if (method == 'POST' && tail == '/reply') {
        final j = jsonDecode(body) as Map;
        replies.putIfAbsent(id, () => []).add('${j['text']}');
        return (status: 202, body: {});
      }
      if (method == 'POST' && tail == '/stop') {
        stops.add(id);
        agent['status'] = 'done';
        return (status: 202, body: {});
      }
    }
    return (status: 404, body: {});
  }

  Stream<List<int>> eventStream(int since) {
    late StreamController<List<int>> c;
    c = StreamController<List<int>>(
      onListen: () {
        c.add(utf8.encode(': hello\n\n'));
        for (final e in events.where((e) => (e['seq'] as int) > since)) {
          c.add(utf8.encode('data: ${jsonEncode(e)}\n\n'));
        }
        _streams.add(c);
      },
      onCancel: () => _streams.remove(c),
    );
    return c.stream;
  }

  http.Client client() => MockClient.streaming((request, bodyStream) async {
    final body = await bodyStream.bytesToString();
    final path = request.url.path;
    if (request.method == 'GET' &&
        path == '/v1/events' &&
        _authorized(request.headers['Authorization'])) {
      final since =
          int.tryParse(request.url.queryParameters['since'] ?? '') ?? 0;
      return http.StreamedResponse(eventStream(since), 200);
    }
    final res = handle(
      request.method,
      path,
      request.headers['Authorization'],
      body,
    );
    return http.StreamedResponse(
      Stream.value(utf8.encode(jsonEncode(res.body))),
      res.status,
      headers: {'content-type': 'application/json'},
    );
  });
}

Future<void> main(List<String> args) async {
  final port = args.isNotEmpty ? int.parse(args[0]) : 47841;
  final server = MockWisplyServer();
  await server.start(port: port);
  stdout.writeln('mock on ${server.url}');
  stdout.writeln('pair link: ${server.pairLink}');
  Timer.periodic(const Duration(seconds: 20), (_) {
    server.emit(
      'waiting',
      'Agent wartet',
      'Doku schreiben braucht eine Antwort',
      agentId: 'a2',
    );
  });
  await ProcessSignal.sigint.watch().first;
  await server.stop();
}
