import 'package:flutter/material.dart';

import 'controller.dart';
import 'screens/board_screen.dart';
import 'screens/pair_screen.dart';
import 'theme.dart';

class WisplyApp extends StatelessWidget {
  const WisplyApp({super.key, required this.controller, this.scanner = true});

  final AppController controller;
  final bool scanner;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Wisply',
      debugShowCheckedModeBanner: false,
      theme: buildTheme(),
      home: ListenableBuilder(
        listenable: controller,
        builder: (context, _) {
          if (!controller.loaded) {
            return const Scaffold(
              body: Center(child: CircularProgressIndicator()),
            );
          }
          if (!controller.paired) {
            return PairScreen(controller: controller, scanner: scanner);
          }
          return BoardScreen(controller: controller);
        },
      ),
    );
  }
}
