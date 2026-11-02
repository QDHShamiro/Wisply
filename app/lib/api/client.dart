import 'dart:async';
import 'dart:convert';

import 'package:http/http.dart' as http;

import 'models.dart';

class WisplyClient {
  WisplyClient({
    required String baseUrl,
    this.token,
    http.Client Function()? clientFactory,
    this.timeout = const Duration(seconds: 12),
  }) : baseUrl = baseUrl.endsWith('/')
           ? baseUrl.substring(0, baseUrl.length - 1)
           : baseUrl,
       _factory = clientFactory ?? http.Client.new;

  final String baseUrl;
  final String? token;
  final http.Client Function() _factory;

  final Duration? timeout;

  Map<String, String> get _headers => {
    if (token != null) 'Authorization': 'Bearer $token',
    'Content-Type': 'application/json',
  };

  Future<http.Response> _send(
    String method,
    String path, {
    Object? body,
  }) async {
    final client = _factory();
    try {
      final req = http.Request(method, Uri.parse('$baseUrl$path'))
        ..headers.addAll(_headers);
      if (body != null) req.body = jsonEncode(body);
      final sent = client.send(req);
      final res = await http.Response.fromStream(
        timeout == null ? await sent : await sent.timeout(timeout!),
      );
      if (res.statusCode == 401) {
        throw ApiException(401, 'Nicht mehr gekoppelt');
      }
      if (res.statusCode >= 400) {
        throw ApiException(res.statusCode, 'Fehler ${res.statusCode}');
      }
      return res;
    } on TimeoutException {
      throw ApiException(0, 'PC nicht erreichbar');
    } on http.ClientException {
      throw ApiException(0, 'PC nicht erreichbar');
    } finally {
      client.close();
    }
  }

  dynamic _json(http.Response res) => jsonDecode(utf8.decode(res.bodyBytes));

  Future<({String token, String name})> pair(String code, String device) async {
    final res = await _send(
      'POST',
      '/v1/pair',
      body: {'code': code, 'device': device},
    );
    final j = _json(res) as Map<String, dynamic>;
    return (token: '${j['token']}', name: '${j['name'] ?? device}');
  }

  Future<List<Agent>> agents() async {
    final res = await _send('GET', '/v1/agents');
    return (_json(res) as List)
        .map((e) => Agent.fromJson(e as Map<String, dynamic>))
        .toList();
  }

  Future<Agent> agent(String id) async {
    final res = await _send('GET', '/v1/agents/${Uri.encodeComponent(id)}');
    return Agent.fromJson(_json(res) as Map<String, dynamic>);
  }

  Future<void> reply(String id, String text) async {
    await _send(
      'POST',
      '/v1/agents/${Uri.encodeComponent(id)}/reply',
      body: {'text': text},
    );
  }

  Future<void> stop(String id) async {
    await _send('POST', '/v1/agents/${Uri.encodeComponent(id)}/stop');
  }

  Future<PcStatus> status() async {
    final res = await _send('GET', '/v1/status');
    return PcStatus.fromJson(_json(res) as Map<String, dynamic>);
  }

  Object _mapError(Object e) {
    if (e is TimeoutException || e is http.ClientException) {
      return ApiException(0, 'PC nicht erreichbar');
    }
    return e;
  }

  Stream<PhoneEvent> events({int since = 0}) {
    final client = _factory();
    StreamSubscription<String>? sub;
    var closed = false;
    late final StreamController<PhoneEvent> out;

    void shutdown() {
      if (closed) return;
      closed = true;
      sub?.cancel();
      client.close();
    }

    void fail(Object e) {
      if (!out.isClosed) {
        out.addError(_mapError(e));
        out.close();
      }
      shutdown();
    }

    Future<void> start() async {
      try {
        final req =
            http.Request('GET', Uri.parse('$baseUrl/v1/events?since=$since'))
              ..headers.addAll({
                if (token != null) 'Authorization': 'Bearer $token',
                'Accept': 'text/event-stream',
              });
        final sent = client.send(req);
        final res = timeout == null ? await sent : await sent.timeout(timeout!);
        if (closed) return;
        if (res.statusCode == 401) {
          throw ApiException(401, 'Nicht mehr gekoppelt');
        }
        if (res.statusCode != 200) {
          throw ApiException(res.statusCode, 'Fehler ${res.statusCode}');
        }
        final data = StringBuffer();
        sub = res.stream
            .transform(utf8.decoder)
            .transform(const LineSplitter())
            .listen(
              (line) {
                if (line.isEmpty) {
                  if (data.isEmpty) return;
                  final text = data.toString();
                  data.clear();
                  try {
                    out.add(
                      PhoneEvent.fromJson(
                        jsonDecode(text) as Map<String, dynamic>,
                      ),
                    );
                  } on FormatException {
                    return;
                  }
                } else if (line.startsWith('data:')) {
                  if (data.isNotEmpty) data.write('\n');
                  data.write(line.substring(5).trimLeft());
                }
              },
              onError: fail,
              onDone: () {
                if (!out.isClosed) out.close();
                shutdown();
              },
            );
      } catch (e) {
        fail(e);
      }
    }

    out = StreamController<PhoneEvent>(onListen: start, onCancel: shutdown);
    return out.stream;
  }
}
