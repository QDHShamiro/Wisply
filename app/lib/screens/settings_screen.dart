import 'package:flutter/material.dart';

import '../api/models.dart';
import '../controller.dart';
import '../theme.dart';

class SettingsScreen extends StatefulWidget {
  const SettingsScreen({super.key, required this.controller});

  final AppController controller;

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  PcStatus? _status;
  String? _error;

  @override
  void initState() {
    super.initState();
    _load();
  }

  Future<void> _load() async {
    final client = widget.controller.client;
    if (client == null) return;
    try {
      final s = await client.status();
      if (mounted) setState(() => _status = s);
    } catch (e) {
      if (mounted) setState(() => _error = '$e');
    }
  }

  Future<void> _unpair() async {
    final ok = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        backgroundColor: Palette.card,
        title: const Text('Gerät entkoppeln?'),
        content: const Text(
          'Das Handy bekommt danach keine Meldungen mehr. Auf dem PC mit "wisply mobile revoke" den Zugang auch sperren.',
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('Abbrechen'),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: const Text(
              'Entkoppeln',
              style: TextStyle(color: Palette.error),
            ),
          ),
        ],
      ),
    );
    if (ok != true) return;
    await widget.controller.unpair();
    if (mounted) Navigator.of(context).popUntil((r) => r.isFirst);
  }

  @override
  Widget build(BuildContext context) {
    final p = widget.controller.pairing;
    final s = _status;
    return Scaffold(
      appBar: AppBar(title: const Text('Einstellungen')),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  const Text(
                    'Gekoppelt mit',
                    style: TextStyle(color: Palette.muted),
                  ),
                  const SizedBox(height: 4),
                  Text(p?.url ?? '-'),
                  const SizedBox(height: 12),
                  const Text(
                    'Gerätename',
                    style: TextStyle(color: Palette.muted),
                  ),
                  const SizedBox(height: 4),
                  Text(p?.name ?? '-'),
                ],
              ),
            ),
          ),
          const SizedBox(height: 12),
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: s == null
                  ? Text(
                      _error ?? 'Lade PC-Status ...',
                      style: TextStyle(
                        color: _error != null ? Palette.error : Palette.muted,
                      ),
                    )
                  : Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text('Wisply ${s.version}'),
                        const SizedBox(height: 6),
                        Text(
                          'C: frei ${s.freeGbC.toStringAsFixed(1)} GB',
                          style: const TextStyle(color: Palette.muted),
                        ),
                        Text(
                          s.gameMode ? 'Spielmodus an' : 'Spielmodus aus',
                          style: const TextStyle(color: Palette.muted),
                        ),
                        Text(
                          s.limitHit
                              ? 'Limit erreicht${s.limitResetAt != null ? ', Reset ${s.limitResetAt}' : ''}'
                              : 'Kein Limit erreicht',
                          style: TextStyle(
                            color: s.limitHit ? Palette.limit : Palette.muted,
                          ),
                        ),
                      ],
                    ),
            ),
          ),
          const SizedBox(height: 24),
          OutlinedButton(
            onPressed: _unpair,
            style: OutlinedButton.styleFrom(
              foregroundColor: Palette.error,
              side: const BorderSide(color: Palette.error),
              padding: const EdgeInsets.symmetric(vertical: 14),
            ),
            child: const Text('Gerät entkoppeln'),
          ),
        ],
      ),
    );
  }
}
