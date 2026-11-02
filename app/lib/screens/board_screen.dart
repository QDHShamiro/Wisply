import 'package:flutter/material.dart';

import '../api/models.dart';
import '../controller.dart';
import '../theme.dart';
import 'agent_detail.dart';
import 'settings_screen.dart';
import 'status_chip.dart';

class BoardScreen extends StatefulWidget {
  const BoardScreen({super.key, required this.controller});

  final AppController controller;

  @override
  State<BoardScreen> createState() => _BoardScreenState();
}

class _BoardScreenState extends State<BoardScreen> {
  List<Agent> _agents = const [];
  String? _error;
  bool _loading = true;
  String? _selected;

  @override
  void initState() {
    super.initState();
    widget.controller.eventTicks.addListener(_refresh);
    _refresh();
  }

  @override
  void dispose() {
    widget.controller.eventTicks.removeListener(_refresh);
    super.dispose();
  }

  Future<void> _refresh() async {
    final client = widget.controller.client;
    if (client == null) return;
    try {
      final list = await client.agents();
      if (mounted) {
        setState(() {
          _agents = list;
          _error = null;
          _loading = false;
        });
      }
    } on ApiException catch (e) {
      if (e.unauthorized) {
        widget.controller.handleUnauthorized();
        return;
      }
      if (mounted) {
        setState(() {
          _error = e.message;
          _loading = false;
        });
      }
    }
  }

  void _open(Agent a, bool wide) {
    if (wide) {
      setState(() => _selected = a.id);
      return;
    }
    final client = widget.controller.client;
    if (client == null) return;
    Navigator.of(context)
        .push(
          MaterialPageRoute<void>(
            builder: (_) => Scaffold(
              appBar: AppBar(title: Text(a.name)),
              body: AgentDetail(
                client: client,
                id: a.id,
                onUnauthorized: widget.controller.handleUnauthorized,
              ),
            ),
          ),
        )
        .then((_) => _refresh());
  }

  void _settings() {
    Navigator.of(context).push(
      MaterialPageRoute<void>(
        builder: (_) => SettingsScreen(controller: widget.controller),
      ),
    );
  }

  Widget _list(bool wide) {
    return RefreshIndicator(
      onRefresh: _refresh,
      child: _agents.isEmpty
          ? ListView(
              children: [
                const SizedBox(height: 120),
                Center(
                  child: _loading
                      ? const CircularProgressIndicator()
                      : Text(
                          _error ?? 'Keine Agents aktiv',
                          style: TextStyle(
                            color: _error != null
                                ? Palette.error
                                : Palette.muted,
                          ),
                        ),
                ),
              ],
            )
          : ListView.separated(
              physics: const AlwaysScrollableScrollPhysics(),
              padding: const EdgeInsets.all(16),
              itemCount: _agents.length + (_error != null ? 1 : 0),
              separatorBuilder: (_, _) => const SizedBox(height: 10),
              itemBuilder: (context, i) {
                if (_error != null) {
                  if (i == 0) {
                    return Text(
                      _error!,
                      style: const TextStyle(color: Palette.error),
                    );
                  }
                  i -= 1;
                }
                final a = _agents[i];
                return _AgentCard(
                  agent: a,
                  selected: wide && a.id == _selected,
                  onTap: () => _open(a, wide),
                );
              },
            ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, box) {
        final wide = box.maxWidth >= 720;
        final client = widget.controller.client;
        return Scaffold(
          appBar: AppBar(
            title: const Text('Wisply'),
            actions: [
              IconButton(
                onPressed: _refresh,
                icon: const Icon(Icons.refresh),
                tooltip: 'Aktualisieren',
              ),
              IconButton(
                onPressed: _settings,
                icon: const Icon(Icons.settings_outlined),
                tooltip: 'Einstellungen',
              ),
            ],
          ),
          body: wide
              ? Row(
                  children: [
                    SizedBox(width: 360, child: _list(true)),
                    const VerticalDivider(width: 1),
                    Expanded(
                      child: _selected == null || client == null
                          ? const Center(
                              child: Text(
                                'Agent wählen',
                                style: TextStyle(color: Palette.muted),
                              ),
                            )
                          : AgentDetail(
                              key: ValueKey(_selected),
                              client: client,
                              id: _selected!,
                              onUnauthorized:
                                  widget.controller.handleUnauthorized,
                            ),
                    ),
                  ],
                )
              : _list(false),
        );
      },
    );
  }
}

class _AgentCard extends StatelessWidget {
  const _AgentCard({
    required this.agent,
    required this.selected,
    required this.onTap,
  });

  final Agent agent;
  final bool selected;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return Card(
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(10),
        side: BorderSide(color: selected ? Palette.text : Palette.border),
      ),
      child: InkWell(
        borderRadius: BorderRadius.circular(10),
        onTap: onTap,
        child: Padding(
          padding: const EdgeInsets.all(14),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Row(
                children: [
                  Expanded(
                    child: Text(
                      agent.name,
                      style: Theme.of(context).textTheme.titleMedium,
                      overflow: TextOverflow.ellipsis,
                    ),
                  ),
                  StatusChip(status: agent.status),
                ],
              ),
              const SizedBox(height: 4),
              Text(
                agent.dirShort,
                style: const TextStyle(color: Palette.muted, fontSize: 12),
              ),
              if (agent.lastLine.isNotEmpty) ...[
                const SizedBox(height: 8),
                Text(
                  agent.lastLine,
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                  style: const TextStyle(fontSize: 13),
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }
}
