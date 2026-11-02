import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:shared_preferences/shared_preferences.dart';

class Pairing {
  Pairing({required this.url, required this.token, required this.name});

  final String url;
  final String token;
  final String name;
}

abstract class PairingStore {
  Future<Pairing?> load();
  Future<void> save(Pairing pairing);
  Future<void> clear();
  Future<int> lastSeq();
  Future<void> saveSeq(int seq);
}

class SecurePairingStore implements PairingStore {
  SecurePairingStore([FlutterSecureStorage? storage])
    : _storage = storage ?? const FlutterSecureStorage();

  final FlutterSecureStorage _storage;

  @override
  Future<Pairing?> load() async {
    final token = await _storage.read(key: 'token');
    final url = await _storage.read(key: 'url');
    if (token == null || url == null) return null;
    return Pairing(
      url: url,
      token: token,
      name: await _storage.read(key: 'name') ?? '',
    );
  }

  @override
  Future<void> save(Pairing pairing) async {
    await _storage.write(key: 'token', value: pairing.token);
    await _storage.write(key: 'url', value: pairing.url);
    await _storage.write(key: 'name', value: pairing.name);
  }

  @override
  Future<void> clear() async {
    await _storage.delete(key: 'token');
    await _storage.delete(key: 'url');
    await _storage.delete(key: 'name');
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove('seq');
  }

  @override
  Future<int> lastSeq() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.reload();
    return prefs.getInt('seq') ?? 0;
  }

  @override
  Future<void> saveSeq(int seq) async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.setInt('seq', seq);
  }
}

class MemoryPairingStore implements PairingStore {
  Pairing? value;
  int seq = 0;

  @override
  Future<Pairing?> load() async => value;

  @override
  Future<void> save(Pairing pairing) async => value = pairing;

  @override
  Future<void> clear() async {
    value = null;
    seq = 0;
  }

  @override
  Future<int> lastSeq() async => seq;

  @override
  Future<void> saveSeq(int s) async => seq = s;
}
