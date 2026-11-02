import 'package:flutter/material.dart';
import 'package:mobile_scanner/mobile_scanner.dart';

import '../api/models.dart';
import '../controller.dart';
import '../theme.dart';

class PairScreen extends StatefulWidget {
  const PairScreen({super.key, required this.controller, this.scanner = true});

  final AppController controller;
  final bool scanner;

  @override
  State<PairScreen> createState() => _PairScreenState();
}

class _PairScreenState extends State<PairScreen> {
  final _manual = TextEditingController();
  bool _busy = false;
  bool _manualMode = false;
  String? _error;

  @override
  void initState() {
    super.initState();
    _manualMode = !widget.scanner;
  }

  @override
  void dispose() {
    _manual.dispose();
    super.dispose();
  }

  Future<void> _submit(String raw) async {
    if (_busy) return;
    final info = PairInfo.parse(raw);
    if (info == null) {
      setState(() => _error = 'Das ist kein Wisply-Code');
      return;
    }
    setState(() {
      _busy = true;
      _error = null;
    });
    try {
      await widget.controller.pair(info);
    } on ApiException catch (e) {
      if (mounted) {
        setState(
          () => _error = e.status == 0
              ? e.message
              : 'Code abgelaufen oder schon benutzt',
        );
      }
    } catch (_) {
      if (mounted) setState(() => _error = 'PC nicht erreichbar');
    } finally {
      if (mounted) setState(() => _busy = false);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Mit PC koppeln')),
      body: SafeArea(
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 520),
            child: Padding(
              padding: const EdgeInsets.all(20),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  const Text(
                    'Auf dem PC im Terminal "wisply mobile pair" ausführen und den QR-Code scannen. Handy und PC müssen im gleichen Tailscale sein.',
                    style: TextStyle(color: Palette.muted),
                  ),
                  const SizedBox(height: 16),
                  if (!_manualMode)
                    Expanded(
                      child: ClipRRect(
                        borderRadius: BorderRadius.circular(12),
                        child: MobileScanner(
                          onDetect: (capture) {
                            for (final code in capture.barcodes) {
                              final v = code.rawValue;
                              if (v != null) {
                                _submit(v);
                                break;
                              }
                            }
                          },
                          errorBuilder: (context, error) => Center(
                            child: Padding(
                              padding: const EdgeInsets.all(16),
                              child: Text(
                                'Kamera nicht verfügbar. Link unten einfügen.',
                                textAlign: TextAlign.center,
                                style: const TextStyle(color: Palette.muted),
                              ),
                            ),
                          ),
                        ),
                      ),
                    )
                  else
                    TextField(
                      controller: _manual,
                      decoration: const InputDecoration(
                        labelText: 'wisply://pair?... Link',
                      ),
                      onSubmitted: _submit,
                    ),
                  if (_error != null) ...[
                    const SizedBox(height: 12),
                    Text(_error!, style: const TextStyle(color: Palette.error)),
                  ],
                  const SizedBox(height: 12),
                  if (_busy) const LinearProgressIndicator(),
                  if (_manualMode)
                    FilledButton(
                      onPressed: _busy ? null : () => _submit(_manual.text),
                      child: const Text('Koppeln'),
                    ),
                  if (widget.scanner)
                    TextButton(
                      onPressed: () =>
                          setState(() => _manualMode = !_manualMode),
                      child: Text(
                        _manualMode ? 'QR scannen' : 'Link einfügen',
                      ),
                    ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
