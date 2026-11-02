import 'package:flutter/material.dart';

class Palette {
  static const bg = Color(0xFF0D1117);
  static const card = Color(0xFF161B22);
  static const border = Color(0xFF30363D);
  static const text = Color(0xFFF0F0F0);
  static const muted = Color(0xFF9198A1);
  static const running = Color(0xFF58A6FF);
  static const waiting = Color(0xFFE3B341);
  static const done = Color(0xFF3FB950);
  static const limit = Color(0xFFDB7C3D);
  static const error = Color(0xFFF85149);
}

ThemeData buildTheme() {
  final scheme = const ColorScheme.dark(
    surface: Palette.bg,
    primary: Palette.text,
    onPrimary: Palette.bg,
    secondary: Palette.running,
    error: Palette.error,
    onSurface: Palette.text,
  );
  return ThemeData(
    useMaterial3: true,
    colorScheme: scheme,
    scaffoldBackgroundColor: Palette.bg,
    appBarTheme: const AppBarTheme(
      backgroundColor: Palette.bg,
      elevation: 0,
      scrolledUnderElevation: 0,
      centerTitle: false,
    ),
    cardTheme: CardThemeData(
      color: Palette.card,
      elevation: 0,
      margin: EdgeInsets.zero,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(10),
        side: const BorderSide(color: Palette.border),
      ),
    ),
    dividerColor: Palette.border,
    inputDecorationTheme: InputDecorationTheme(
      filled: true,
      fillColor: Palette.card,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: const BorderSide(color: Palette.border),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: const BorderSide(color: Palette.border),
      ),
    ),
    filledButtonTheme: FilledButtonThemeData(
      style: FilledButton.styleFrom(
        backgroundColor: Palette.text,
        foregroundColor: Palette.bg,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
      ),
    ),
  );
}

Color statusColor(String status) {
  switch (status) {
    case 'running':
      return Palette.running;
    case 'waiting':
      return Palette.waiting;
    case 'done':
      return Palette.done;
    case 'limit':
      return Palette.limit;
    default:
      return Palette.error;
  }
}

String statusLabel(String status) {
  switch (status) {
    case 'running':
      return 'Läuft';
    case 'waiting':
      return 'Wartet auf dich';
    case 'done':
      return 'Fertig';
    case 'limit':
      return 'Limit erreicht';
    default:
      return 'Fehler';
  }
}
