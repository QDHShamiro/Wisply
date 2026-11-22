(function (start, print, facts) {
  "use strict";
  const g = globalThis;
  const info = JSON.parse(facts);
  const waiting = new Map();
  const calls = [];
  let seq = 0;

  const here = () => String(new Error().stack || "");
  const placed = (e, at) => {
    if (at) e.stack = at;
    return e;
  };

  const call = (kind, payload, at) =>
    new Promise((resolve, reject) => {
      const id = ++seq;
      waiting.set(id, [resolve, reject, at === undefined ? here() : at]);
      start(id, kind, typeof payload === "string" ? payload : JSON.stringify(payload));
    });

  const settle = (id, ok, text) => {
    const pair = waiting.get(id);
    if (!pair) return;
    waiting.delete(id);
    if (ok) pair[0](text);
    else pair[1](placed(new Error(text), pair[2]));
  };

  class Output {
    constructor(r) {
      this.stdout = r.stdout;
      this.stderr = r.stderr;
      this.exitCode = r.code;
    }
    get ok() {
      return this.exitCode === 0;
    }
    text() {
      return this.stdout;
    }
    lines() {
      return lines(this.stdout);
    }
    json() {
      return JSON.parse(this.stdout);
    }
    toString() {
      return this.stdout;
    }
    toJSON() {
      if (this.exitCode === 0 && !this.stderr.trim()) return this.stdout.trimEnd();
      return { exitCode: this.exitCode, stdout: this.stdout.trimEnd(), stderr: this.stderr.trimEnd() };
    }
  }

  class ShellError extends Error {
    constructor(out, label) {
      const tail = lines(out.stderr.trim() ? out.stderr : out.stdout).slice(-6).join("\n");
      super(`exit ${out.exitCode} from: ${label}` + (tail ? `\n${tail}` : ""));
      this.name = "ShellError";
      this.exitCode = out.exitCode;
      this.stdout = out.stdout;
      this.stderr = out.stderr;
    }
  }

  const lines = (text) => text.split(/\r?\n/).filter((l) => l !== "");
  const swap = (k, v) =>
    v instanceof Map ? Object.fromEntries(v) : v instanceof Set ? [...v] : typeof v === "bigint" ? String(v) : v;

  const str = (v) => {
    if (v === null || v === undefined) return "";
    if (typeof v === "string") return v;
    if (v instanceof Output) return v.stdout.replace(/(\r?\n)+$/, "");
    if (typeof v === "object") {
      try {
        return JSON.stringify(v, swap);
      } catch (e) {
        return String(v);
      }
    }
    return String(v);
  };

  const one = (v) => {
    if (typeof v === "string") return v;
    if (v === undefined) return "undefined";
    if (typeof v === "function") return `[function ${v.name || "anonymous"}]`;
    if (v instanceof Error) return `${v.name}: ${v.message}`;
    try {
      const text = JSON.stringify(v, swap);
      return text === undefined ? String(v) : text;
    } catch (e) {
      return String(v);
    }
  };

  const show = (v) => (Array.isArray(v) ? v.map(one).join("\n") : one(v instanceof Output ? v.toJSON() : v));

  const pieces = (raws, values) => {
    const out = [];
    raws.forEach((raw, i) => {
      const text = raw.replace(/\\(\$\{|`)/g, "$1");
      if (text) out.push({ raw: text });
      if (i >= values.length) return;
      const v = values[i];
      if (Array.isArray(v)) out.push({ args: v.map(str) });
      else if (v && typeof v === "object" && typeof v.raw === "string" && !(v instanceof Output)) out.push({ raw: v.raw });
      else out.push({ arg: str(v) });
    });
    return out;
  };

  const label = (parts) =>
    parts
      .map((p) => (p.raw !== undefined ? p.raw : p.args !== undefined ? p.args.join(" ") : p.arg))
      .join("")
      .trim()
      .slice(0, 160);

  class Call {
    constructor(kind, payload, name) {
      this.kind = kind;
      this.payload = payload;
      this.name = name;
      this.throws = true;
      this.seen = false;
      this.failed = "";
      this.promise = null;
      this.at = here();
      calls.push(this);
      Promise.resolve().then(() =>
        this.run().catch((e) => {
          this.failed = String((e && e.message) || e).split("\n")[0];
        }),
      );
    }
    run() {
      if (!this.promise) this.promise = call(this.kind, this.payload, this.at).then((text) => this.done(JSON.parse(text)));
      return this.promise;
    }
    then(a, b) {
      this.seen = true;
      return this.run().then(a, b);
    }
    catch(b) {
      this.seen = true;
      return this.run().catch(b);
    }
    finally(f) {
      this.seen = true;
      return this.run().finally(f);
    }
    nothrow() {
      this.throws = false;
      return this;
    }
  }

  class ShellCall extends Call {
    done(r) {
      const out = new Output(r);
      if (out.exitCode !== 0 && this.throws) throw placed(new ShellError(out, this.name), this.at);
      return out;
    }
    quiet() {
      return this;
    }
    cwd(dir) {
      this.payload.cwd = str(dir);
      return this;
    }
    env(vars) {
      const env = this.payload.env || {};
      for (const key of Object.keys(vars || {})) env[key] = str(vars[key]);
      this.payload.env = env;
      return this;
    }
    timeout(ms) {
      this.payload.timeout = Number(ms) || 0;
      return this;
    }
    stdin(text) {
      this.payload.stdin = str(text);
      return this;
    }
    text() {
      return this.then((o) => o.stdout);
    }
    lines() {
      return this.then((o) => o.lines());
    }
    json() {
      return this.then((o) => o.json());
    }
  }

  class WisplyCall extends Call {
    done(r) {
      if (r.code !== 0 && this.throws) {
        const e = new Error(r.out);
        e.name = "WisplyError";
        e.code = r.code;
        throw placed(e, this.at);
      }
      return this.throws ? r.out : { ok: r.code === 0, code: r.code, out: r.out };
    }
    text() {
      return this.then((r) => (typeof r === "string" ? r : r.out));
    }
    lines() {
      return this.text().then(lines);
    }
    json() {
      return this.text().then((t) => JSON.parse(t));
    }
  }

  const tagged = (first) => Array.isArray(first) && Array.isArray(first.raw);

  const $ = (first, ...values) => {
    const parts = tagged(first) ? pieces(first.raw, values) : [{ raw: [first, ...values].map(str).join(" ") }];
    return new ShellCall("sh", { parts }, label(parts));
  };

  const wisply = (first, ...rest) => {
    if (tagged(first)) {
      const parts = pieces(first.raw, rest);
      return new WisplyCall("wisply", { parts }, "wisply " + label(parts));
    }
    if (Array.isArray(first) || rest.length) {
      const argv = (Array.isArray(first) ? first : [first, ...rest]).map(str);
      return new WisplyCall("wisply", { argv }, "wisply " + argv.join(" ").slice(0, 160));
    }
    return new WisplyCall("wisply", { line: str(first) }, "wisply " + str(first).slice(0, 160));
  };

  const read = (path) => call("read", { path: str(path) });
  const write = (path, text) => call("write", { path: str(path), text: str(text) }).then(() => undefined);
  const edit = (path, from, to, all) =>
    call("edit", { path: str(path), from: str(from), to: str(to), all: all === true || (!!all && all.all === true) }).then(Number);
  const glob = (pattern, dir) =>
    call("glob", { pattern: str(pattern), cwd: dir === undefined ? "" : str(dir && dir.cwd !== undefined ? dir.cwd : dir) }).then(
      (text) => JSON.parse(text),
    );
  const sleep = (ms) => call("sleep", String(Math.max(0, Number(ms) || 0))).then(() => undefined);

  const timers = new Map();
  let timerSeq = 0;
  g.setTimeout = (fn, ms, ...args) => {
    const id = ++timerSeq;
    timers.set(id, true);
    sleep(ms).then(() => {
      if (timers.delete(id) && typeof fn === "function") fn(...args);
    });
    return id;
  };
  g.clearTimeout = (id) => {
    timers.delete(id);
  };

  const log = (...args) => print(args.map((a) => (typeof a === "string" ? a : show(a))).join(" "));
  g.console = { log, info: log, warn: log, error: log, debug: log };

  const deep = async (v, depth) => {
    if (depth > 8 || v === null || typeof v !== "object") return v;
    if (typeof v.then === "function") return deep(await v, depth + 1);
    if (Array.isArray(v)) return Promise.all(v.map((x) => deep(x, depth + 1)));
    const proto = Object.getPrototypeOf(v);
    if (proto !== Object.prototype && proto !== null) return v;
    const out = {};
    for (const key of Object.keys(v)) out[key] = await deep(v[key], depth + 1);
    return out;
  };

  const explain = (e) => {
    if (e instanceof Error || (e && typeof e === "object" && "message" in e)) {
      return { ok: false, name: String(e.name || "Error"), message: String(e.message), stack: String(e.stack || "") };
    }
    return { ok: false, name: "Thrown", message: one(e), stack: "" };
  };

  const finish = async (promise, boxed) => {
    try {
      let v = await promise;
      if (boxed && v && typeof v === "object") v = v.value;
      v = await deep(v, 0);
      const none = v === undefined || v === null;
      return JSON.stringify({ ok: true, none, text: none ? "" : show(v) });
    } catch (e) {
      return JSON.stringify(explain(e));
    }
  };

  const args = Object.freeze(Object.assign({}, info.args || {}));
  Object.assign(g, { $, wisply, read, write, edit, glob, sleep, args, shell: info.shell, os: info.os, cwd: info.cwd });

  return {
    settle,
    finish,
    explain: (e) => JSON.stringify(explain(e)),
    lost: () =>
      calls
        .filter((c) => c.failed && !c.seen)
        .map((c) => c.failed)
        .join("\n"),
  };
})
