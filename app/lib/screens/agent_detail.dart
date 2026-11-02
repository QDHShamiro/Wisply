import 'dart:async';

import 'package:flutter/material.dart';

import '../api/client.dart';
import '../api/models.dart';
import '../theme.dart';
import 'status_chip.dart';

class AgentDetail extends StatefulWidget {
  const AgentDetail({
    super.key,
    required this.client,
    required this.id,
    this.onUnauthorized,
    this.refreshEvery = const Duration(seconds: 5),
  });

  final WisplyClient client;
  final String id;
  final VoidCallback? onUnauthorized;
  final Duration refreshEvery;

  @override
  State<AgentDetail> createState() => _AgentDetailState();
}

class _AgentDetailState extends State<AgentDetail> {
  final _reply = TextEditingController();
  Agent? _agent;
  String? _error;
  bool _sending = false;
  Timer? _timer;

  @override
  void initState() {
    super.initState();
    _load();
    _timer = Timer.periodic(widget.refreshEvery, (_) => _load());
  }

  @override
  void didUpdateWidget(AgentDetail old) {
    super.didUpdateWidget(old);
    if (old.id != widget.id) {
      _agent = null;
      _load();
    }
  }

  @override
  void dispose() {
    _timer?.cancel();
    _reply.dispose();
    super.dispose();
  }

  void _fail(Object e) {
    if (e is ApiException && e.unauthorized) {
      widget.onUnauthorized?.call();
      return;
    }
    if (mounted) setState(() => _error = e is ApiException ? e.message : '$e');
  }

  Future<void> _load() async {
    try {
      final a = await widget.client.agent(widget.id);
      if (mounted) {
        setState(() {
          _agent = a;
          _error = null;
        });
      }
    } catch (e) {
      _fail(e);
    }
  }

  Future<void> _send() async {
    final text = _reply.text.trim();
    if (text.isEmpty || _sending) return;
    setState(() => _sending = true);
    try {
      await widget.client.reply(widget.id, text);
      _reply.clear();
      if (mounted) {
        ScaffoldMessenger.of(context)
            .showSnackBar(const SnackBar(content: Text('Antwort gesendet')));
      }
      await _load();
    } catch (e) {
      _fail(e);
    } finally {
      if (mounted) setState(() => _sending = false);
    }
  }

  Future<void> _stop() async {
    final ok = await showDialog<bool>(
      context: context,
      builder: (context) => AlertDialog(
        backgroundColor: Palette.card,
        title: const Text('Agent stoppen?'),
        content: Text('${_agent?.name ?? widget.id} wird beendet.'),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context, false),
            child: const Text('Abbrechen'),
          ),
          TextButton(
            onPressed: () => Navigator.pop(context, true),
            child: const Text(
              'Stoppen',
              style: TextStyle(color: Palette.error),
            ),
          ),
        ],
      ),
    );
    if (ok != true) return;
    try {
      await widget.client.stop(widget.id);
      if (mounted) {
        ScaffoldMessenger.of(context)
            .showSnackBar(const SnackBar(content: Text('Stopp gesendet')));
      }
      await _load();
    } catch (e) {
      _fail(e);
    }
  }

  @override
  Widget build(BuildContext context) {
    final a = _agent;
    if (a == null) {
      return Center(
        child: _error != null
            ? Text(_error!, style: const TextStyle(color: Palette.error))
            : const CircularProgressIndicator(),
      );
    }
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
          child: Row(
            children: [
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(a.name, style: Theme.of(context).textTheme.titleLarge),
                    Text(
                      a.dirShort,
                      style: const TextStyle(color: Palette.muted),
                    ),
                  ],
                ),
              ),
              StatusChip(status: a.status),
            ],
          ),
        ),
        if (_error != null)
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 16),
            child: Text(_error!, style: const TextStyle(color: Palette.error)),
          ),
        Expanded(
          child: Container(
            width: double.infinity,
            margin: const EdgeInsets.symmetric(horizontal: 16),
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              color: Palette.card,
              borderRadius: BorderRadius.circular(10),
              border: Border.all(color: Palette.border),
            ),
            child: ListView.builder(
              reverse: true,
              itemCount: a.tail.length,
              itemBuilder: (context, i) {
                final line = a.tail[a.tail.length - 1 - i];
                return Text(
                  line,
                  style: const TextStyle(
                    fontFamily: 'monospace',
                    fontSize: 12,
                    height: 1.4,
                  ),
                );
              },
            ),
          ),
        ),
        Padding(
          padding: const EdgeInsets.all(16),
          child: Row(
            children: [
              Expanded(
                child: TextField(
                  controller: _reply,
                  minLines: 1,
                  maxLines: 4,
                  textInputAction: TextInputAction.send,
                  onSubmitted: (_) => _send(),
                  decoration: const InputDecoration(hintText: 'Antwort'),
                ),
              ),
              const SizedBox(width: 8),
              IconButton.filled(
                onPressed: _sending ? null : _send,
                icon: const Icon(Icons.send),
                tooltip: 'Senden',
              ),
              const SizedBox(width: 4),
              IconButton(
                onPressed: _stop,
                icon: const Icon(Icons.stop_circle_outlined),
                color: Palette.error,
                tooltip: 'Stopp',
              ),
            ],
          ),
        ),
      ],
    );
  }
}
