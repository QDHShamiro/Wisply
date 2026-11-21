import { spawn } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createServer } from "node:http";
import { tmpdir } from "node:os";
import { extname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";
import { synth } from "./synth.mjs";

const args = Object.fromEntries(process.argv.slice(2).map((a) => a.replace(/^--/, "").split("=")).map(([k, v]) => [k, v ?? "1"]));
const SIZES = (args.nodes ?? "500,2000,5000,20000").split(",").map(Number);
const [WIDTH, HEIGHT] = (args.size ?? "1280x820").split("x").map(Number);
const DIST = args.dist ?? fileURLToPath(new URL("../dist", import.meta.url));
const SOFT = args.gpu === "0";
const DRAW = args.draw !== "0";
const THEME = args.theme ?? "dark";
const LIMIT_MS = Number(args.limit ?? 120000);
const STEPS = new Set((args.steps ?? "pan,zoom,hover,focus,again").split(","));
const BROWSERS = [
  process.env.WISPLY_BROWSER,
  "C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe",
  "C:/Program Files/Microsoft/Edge/Application/msedge.exe",
  "C:/Program Files/Google/Chrome/Application/chrome.exe",
  "/usr/bin/chromium",
  "/usr/bin/chromium-browser",
  "/usr/bin/google-chrome",
  "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
].filter(Boolean);
const TYPES = { ".html": "text/html; charset=utf-8", ".js": "text/javascript; charset=utf-8", ".css": "text/css; charset=utf-8", ".svg": "image/svg+xml" };
const CSP = "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'none'; worker-src blob:; object-src 'none'; base-uri 'none'; form-action 'none'";
const PULSE = JSON.stringify({ pid: 1, up: 60, working: 0, queued: 0, drafts: 0, notes: 0, problem: "" });
const ARRANGED = "/\\d/.test(document.querySelector('.graph .info')?.textContent ?? '')";
const PROBE = `(() => {
  const m = (window.__m = { long: [], frames: null, script: 0, draws: 0, first: 0, renderer: "" });
  new PerformanceObserver((l) => { for (const e of l.getEntries()) m.long.push(Math.round(e.duration)); }).observe({ type: "longtask", buffered: true });
  const raf = window.requestAnimationFrame.bind(window);
  window.requestAnimationFrame = (f) => raf((t) => { const s = performance.now(); try { f(t); } finally { m.script += performance.now() - s; } });
  let run = 0;
  m.rec = () => {
    const mine = ++run;
    const frames = (m.frames = []);
    let last = (m.script = 0);
    const tick = (t) => { if (mine !== run) return; frames.push([t, m.script - last]); last = m.script; raf(tick); };
    raf(tick);
  };
  m.stop = () => { run++; return m.frames; };
  const get = HTMLCanvasElement.prototype.getContext;
  HTMLCanvasElement.prototype.getContext = function (kind, ...rest) {
    const gl = get.call(this, kind, ...rest);
    if (!gl || !/webgl/.test(kind) || gl.__probed) return gl;
    gl.__probed = true;
    const ext = gl.getExtension("WEBGL_debug_renderer_info");
    if (ext) m.renderer = gl.getParameter(ext.UNMASKED_RENDERER_WEBGL);
    for (const k of ["drawArrays", "drawElements", "drawArraysInstanced", "drawElementsInstanced"]) {
      const f = gl[k];
      gl[k] = function (...a) { m.draws++; if (!m.first) m.first = performance.now(); if (${DRAW}) return f.apply(this, a); };
    }
    return gl;
  };
  window.ipc = { postMessage: (s) => window.__ipc(s) };
})();`;

const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const round = (v) => Math.round(v * 10) / 10;
const note = (text) => args.verbose && console.error(text);

function serve(dir) {
  return new Promise((resolve) => {
    const server = createServer((req, res) => {
      const path = normalize(decodeURIComponent((req.url ?? "/").split("?")[0])).replace(/^([\\/])+/, "");
      const file = join(dir, path === "" || path === "." ? "index.html" : path);
      if (!file.startsWith(normalize(dir)) || !existsSync(file)) return void res.writeHead(404).end("not found");
      res.writeHead(200, { "Content-Type": TYPES[extname(file)] ?? "application/octet-stream", "Content-Security-Policy": CSP, "Cache-Control": "no-store" });
      res.end(readFileSync(file));
    });
    server.listen(0, "127.0.0.1", () => resolve({ url: `http://127.0.0.1:${server.address().port}/`, close: () => server.close() }));
  });
}

async function browser() {
  const exe = BROWSERS.find((p) => existsSync(p));
  if (!exe) throw new Error("no Chromium browser found, set WISPLY_BROWSER to its path");
  if (!globalThis.WebSocket) throw new Error("needs Node 22 or newer");
  const profile = mkdtempSync(join(tmpdir(), "wisply-graph-bench-"));
  const flags = ["--headless=new", `--user-data-dir=${profile}`, "--remote-debugging-port=0", "--no-first-run", "--no-default-browser-check", "--disable-extensions", "--disable-sync", "--disable-background-networking", "--disable-component-update", "--mute-audio", "--hide-scrollbars", "--lang=en-US", `--window-size=${WIDTH},${HEIGHT}`];
  if (SOFT) flags.push("--use-angle=swiftshader", "--enable-unsafe-swiftshader");
  const child = spawn(exe, [...flags, "about:blank"], { stdio: "ignore", windowsHide: true });
  process.on("exit", () => child.kill());
  const portFile = join(profile, "DevToolsActivePort");
  for (let i = 0; i < 200 && !existsSync(portFile); i++) await sleep(50);
  const [port, path] = readFileSync(portFile, "utf8").trim().split("\n");
  const ws = new WebSocket(`ws://127.0.0.1:${port}${path}`);
  await new Promise((ok, bad) => {
    ws.onopen = ok;
    ws.onerror = bad;
  });
  let next = 0;
  const waiting = new Map();
  const subs = new Map();
  ws.onmessage = (msg) => {
    const d = JSON.parse(msg.data);
    if (d.id === undefined) return void subs.get(d.method)?.(d.params);
    const w = waiting.get(d.id);
    waiting.delete(d.id);
    if (w) d.error ? w.bad(new Error(`${w.method}: ${d.error.message}`)) : w.ok(d.result);
  };
  const raw = (method, params = {}, sessionId) =>
    new Promise((ok, bad) => {
      const id = ++next;
      waiting.set(id, { ok, bad, method });
      ws.send(JSON.stringify({ id, method, params, sessionId }));
    });
  const { targetId } = await raw("Target.createTarget", { url: "about:blank" });
  const { sessionId } = await raw("Target.attachToTarget", { targetId, flatten: true });
  const send = (method, params) => raw(method, params, sessionId);
  const run = async (expression) => {
    const r = await send("Runtime.evaluate", { expression, awaitPromise: true, returnByValue: true });
    if (r.exceptionDetails) throw new Error(r.exceptionDetails.exception?.description ?? r.exceptionDetails.text);
    return r.result.value;
  };
  const close = async () => {
    await Promise.race([raw("Browser.close").catch(() => {}), sleep(1000)]);
    child.kill();
    for (let i = 0; i < 10; i++) {
      try {
        rmSync(profile, { recursive: true, force: true });
        break;
      } catch {
        await sleep(300);
      }
    }
  };
  return { send, run, close, on: (method, f) => subs.set(method, f) };
}

function pace(frames) {
  const gaps = frames.slice(1).map((f, i) => f[0] - frames[i][0]).sort((a, b) => a - b);
  const script = frames.slice(1).map((f) => f[1]).sort((a, b) => a - b);
  if (!gaps.length) return "no frames";
  const p95 = (list) => list[Math.min(list.length - 1, Math.floor(list.length * 0.95))];
  const fps = (gaps.length * 1000) / (frames[frames.length - 1][0] - frames[0][0]);
  return `${round(fps)} fps, p95 ${round(p95(gaps))} ms, worst ${round(gaps[gaps.length - 1])} ms, script p95 ${round(p95(script))} ms`;
}

async function page(nodes, site) {
  const data = JSON.stringify(synth(nodes));
  const b = await browser();
  const guard = setTimeout(() => {
    console.error(`${nodes} nodes: gave up after ${LIMIT_MS} ms, raise --limit or pick fewer --steps`);
    void b.close().then(() => process.exit(2));
  }, LIMIT_MS);
  await b.send("Page.enable");
  await b.send("Runtime.enable");
  await b.send("Performance.enable");
  await b.send("Emulation.setDeviceMetricsOverride", { width: WIDTH, height: HEIGHT, deviceScaleFactor: 1, mobile: false });
  await b.send("Runtime.addBinding", { name: "__ipc" });
  await b.send("Page.addScriptToEvaluateOnNewDocument", { source: PROBE });
  b.on("Runtime.bindingCalled", ({ payload }) => {
    const { id, argv } = JSON.parse(payload);
    const out = argv[0] === "board" && argv[1] === "graph" ? data : argv[0] === "board" && argv[1] === "pulse" ? PULSE : "";
    const code = out || argv[0] === "theme" ? 0 : 1;
    void b.send("Runtime.evaluate", { expression: `window.lt && window.lt.done(${id}, ${code}, ${JSON.stringify(out || (code ? "err: not in the bench" : ""))})` }).catch(() => {});
  });
  const until = async (expression, ms = 90000) => {
    const end = Date.now() + ms;
    while (Date.now() < end) {
      if (await b.run(expression).catch(() => false)) return true;
      await sleep(50);
    }
    return false;
  };
  const longest = async () => (await b.run("(() => { const l = __m.long; __m.long = []; return l; })()")).reduce((a, v) => Math.max(a, v), 0);
  const record = async (name, act) => {
    note(name);
    await b.run("__m.rec()");
    await sleep(60);
    await act();
    await sleep(60);
    return `${pace(await b.run("__m.stop()"))}, longest task ${await longest()} ms`;
  };
  const shot = async (name) => {
    if (!args.shots) return;
    mkdirSync(args.shots, { recursive: true });
    writeFileSync(join(args.shots, `graph-${nodes}-${name}.png`), Buffer.from((await b.send("Page.captureScreenshot", { format: "png" })).data, "base64"));
  };
  const open = async () => {
    note("open");
    await b.send("Page.navigate", { url: "about:blank" });
    await sleep(100);
    await b.send("Page.navigate", { url: `${site.url}#/graph` });
    await until("window.__m && __m.first > 0");
    const first = await b.run("Math.round(__m.first)");
    await until(ARRANGED);
    const arranged = await b.run("Math.round(performance.now())");
    await sleep(600);
    return `first image ${first} ms, arranged ${arranged} ms, longest task ${await longest()} ms`;
  };
  const close = async () => {
    clearTimeout(guard);
    await b.close();
  };
  await b.send("Page.navigate", { url: site.url });
  await sleep(300);
  await b.run(`localStorage.clear(); localStorage.setItem("wisply.theme", ${JSON.stringify(THEME)})`);
  return { b, record, shot, open, close };
}

function hands(b, x, y) {
  const mouse = (type, px, py, extra = {}) => b.send("Input.dispatchMouseEvent", { type, x: px, y: py, button: "none", buttons: 0, ...extra });
  const pan = async () => {
    await mouse("mouseMoved", x, y);
    await mouse("mousePressed", x, y, { button: "left", buttons: 1, clickCount: 1 });
    for (let i = 0; i < 60; i++) {
      const a = (i / 60) * Math.PI * 4;
      await mouse("mouseMoved", x + Math.cos(a) * 3 * i, y + Math.sin(a) * 3 * i, { button: "left", buttons: 1 });
      await sleep(8);
    }
    await mouse("mouseReleased", x, y, { button: "left", buttons: 0, clickCount: 1 });
    await sleep(300);
  };
  const zoom = async (dir) => {
    for (let i = 0; i < 10; i++) {
      await mouse("mouseWheel", x, y, { deltaX: 0, deltaY: dir * 120 });
      await sleep(40);
    }
    await sleep(300);
  };
  const sweep = async () => {
    for (let i = 0; i <= 80; i++) {
      await mouse("mouseMoved", x - 240 + i * 6, y - 160 + i * 4);
      await sleep(8);
    }
    await mouse("mouseMoved", 4, 4);
  };
  return { pan, zoom, sweep };
}

async function measure(nodes, site) {
  const { b, record, shot, open, close } = await page(nodes, site);
  const row = { open: await open() };
  await shot("settled");
  const [x, y] = await b.run("(() => { const r = document.querySelector('.graph .canvas').getBoundingClientRect(); return [Math.round(r.left + r.width / 2), Math.round(r.top + r.height / 2)]; })()");
  const { pan, zoom, sweep } = hands(b, x, y);
  if (STEPS.has("pan")) row.pan = await record("pan", pan);
  if (STEPS.has("zoom")) {
    row["zoom in"] = await record("zoom in", () => zoom(-1));
    await shot("near");
    row["zoom out"] = await record("zoom out", () => zoom(1));
  }
  if (STEPS.has("hover")) row.hover = await record("hover", sweep);
  if (STEPS.has("focus")) {
    row.focus = await record("focus", async () => {
      await b.run("document.querySelector('.graph aside > a')?.click()");
      await sleep(900);
    });
    await shot("focus");
    row["pan in focus"] = await record("pan in focus", pan);
    row["focus off"] = await record("focus off", async () => {
      await b.run("document.querySelector('.graph aside .back')?.click()");
      await sleep(600);
    });
  }
  await b.send("HeapProfiler.collectGarbage").catch(() => {});
  const heap = (await b.send("Performance.getMetrics")).metrics.find((v) => v.name === "JSHeapUsedSize").value;
  row.memory = `${round(heap / 1048576)} MB script heap`;
  const renderer = await b.run("__m.renderer");
  if (STEPS.has("again")) row["open again"] = await open();
  await close();
  return { row, renderer };
}

const site = await serve(DIST);
for (const nodes of SIZES) {
  const { row, renderer } = await measure(nodes, site);
  console.log(`\n${nodes} nodes · ${WIDTH}x${HEIGHT} · ${THEME} · ${DRAW ? renderer : "drawing skipped, script time only"}`);
  for (const [name, text] of Object.entries(row)) console.log(`  ${name.padEnd(13)}${text}`);
}
site.close();
