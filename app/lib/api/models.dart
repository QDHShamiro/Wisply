class Agent {
  Agent({
    required this.id,
    required this.name,
    required this.dirShort,
    required this.status,
    required this.since,
    required this.lastLine,
    required this.tokens,
    required this.costEst,
    this.resetAt,
    this.tail = const [],
  });

  final String id;
  final String name;
  final String dirShort;
  final String status;
  final String since;
  final String lastLine;
  final int tokens;
  final double costEst;
  final String? resetAt;
  final List<String> tail;

  factory Agent.fromJson(Map<String, dynamic> j) => Agent(
    id: '${j['id']}',
    name: '${j['name'] ?? ''}',
    dirShort: '${j['dir_short'] ?? ''}',
    status: '${j['status'] ?? 'error'}',
    since: '${j['since'] ?? ''}',
    lastLine: '${j['last_line'] ?? ''}',
    tokens: (j['tokens'] as num?)?.toInt() ?? 0,
    costEst: (j['cost_est'] as num?)?.toDouble() ?? 0,
    resetAt: j['reset_at'] as String?,
    tail: ((j['tail'] as List?) ?? const []).map((e) => '$e').toList(),
  );
}

class PhoneEvent {
  PhoneEvent({
    required this.seq,
    required this.kind,
    required this.title,
    required this.body,
    this.agentId,
  });

  final int seq;
  final String kind;
  final String title;
  final String body;
  final String? agentId;

  factory PhoneEvent.fromJson(Map<String, dynamic> j) => PhoneEvent(
    seq: (j['seq'] as num?)?.toInt() ?? 0,
    kind: '${j['kind'] ?? ''}',
    title: '${j['title'] ?? ''}',
    body: '${j['body'] ?? ''}',
    agentId: j['agent_id'] == null ? null : '${j['agent_id']}',
  );
}

class PcStatus {
  PcStatus({
    required this.version,
    required this.freeGbC,
    required this.gameMode,
    required this.limitHit,
    this.limitResetAt,
  });

  final String version;
  final double freeGbC;
  final bool gameMode;
  final bool limitHit;
  final String? limitResetAt;

  factory PcStatus.fromJson(Map<String, dynamic> j) {
    final pc = (j['pc'] as Map?) ?? const {};
    final limit = (j['limit'] as Map?) ?? const {};
    return PcStatus(
      version: '${j['version'] ?? ''}',
      freeGbC: (pc['free_gb_c'] as num?)?.toDouble() ?? 0,
      gameMode: pc['game_mode'] == true,
      limitHit: limit['hit'] == true,
      limitResetAt: limit['reset_at'] as String?,
    );
  }
}

class PairInfo {
  PairInfo({required this.url, required this.code});

  final String url;
  final String code;

  static PairInfo? parse(String raw) {
    final uri = Uri.tryParse(raw.trim());
    if (uri == null || uri.scheme != 'wisply' || uri.host != 'pair') {
      return null;
    }
    final u = uri.queryParameters['u'];
    final c = uri.queryParameters['c'];
    if (u == null || c == null || u.isEmpty || c.isEmpty) return null;
    final target = Uri.tryParse(u);
    if (target == null || !target.hasAuthority) return null;
    if (target.scheme != 'http' && target.scheme != 'https') return null;
    return PairInfo(url: u, code: c);
  }
}

class ApiException implements Exception {
  ApiException(this.status, this.message);

  final int status;
  final String message;

  bool get unauthorized => status == 401;

  @override
  String toString() => message;
}
