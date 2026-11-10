import * as C from "./cdp.js";
import * as T from "./tabs.js";
import * as A from "./actions.js";
import { mediaAudio, shot, frames } from "./media.js";
import { SETTLE_MAX_MS, stillBusy, withKeys, snap, text, table, wait, expect } from "./read.js";
import { net, webmcp, pageExtras, js, fetchJson, upload, dialog, downloads, viewport, consoleCmd, dismiss } from "./tools.js";

const HOST = "dev.qdhshamiro.wisply";
const VERSION = chrome.runtime.getManifest().version;
const { cfg, page, formatSnap, header, guard, tabOf, trunc, LOAD_MS, LOADING_WAIT_MS } = A;
let port = null;
let backoff = 300;
let timer = null;

function brandInfo() {
  const brands = (navigator.userAgentData && navigator.userAgentData.brands) || [];
  const pick = (re) => brands.find((b) => re.test(b.brand));
  const b = pick(/brave/i) || pick(/edge/i) || pick(/opera/i) || pick(/vivaldi/i) || pick(/google chrome/i) || pick(/chromium/i);
  const name = b ? b.brand.toLowerCase().replace("microsoft ", "").replace("google ", "") : "chromium";
  return { brand: name, browserVersion: b ? b.version : "" };
}

function post(msg) {
  if (!port) return;
  try {
    port.postMessage(msg);
  } catch (e) {}
}

function schedule() {
  clearTimeout(timer);
  timer = setTimeout(connect, backoff);
  backoff = Math.min(backoff * 2, 30000);
}

async function connect() {
  if (port) return;
  try {
    port = chrome.runtime.connectNative(HOST);
  } catch (e) {
    port = null;
    schedule();
    return;
  }
  port.onMessage.addListener(onMessage);
  port.onDisconnect.addListener(() => {
    port = null;
    void chrome.runtime.lastError;
    schedule();
  });
  let focused = false;
  try {
    const w = await chrome.windows.getLastFocused();
    focused = !!w.focused;
  } catch (e) {}
  post(Object.assign({ event: "hello", ext: VERSION, focused, epoch: T.getEpoch() }, brandInfo()));
}

function event(text) {
  post({ event: "note", text });
}

async function onMessage(msg) {
  backoff = 300;
  if (!msg || !msg.cmd) return;
  const id = msg.id;
  const busy = !NO_TAB.has(msg.cmd) && !msg.cmd.startsWith("record") && msg.cmd !== "where";
  const t0 = performance.now();
  if (busy) T.acts(true);
  try {
    if (busy) await T.paint("orange", msg.group);
    const res = await run(msg.cmd, Object.assign(msg.args || {}, msg.group ? { group: msg.group } : {}), msg.tab);
    if (id) post(Object.assign({ id, ok: true, extMs: performance.now() - t0 }, typeof res === "string" ? { out: res } : res));
  } catch (e) {
    if (busy) T.paint("red", msg.group);
    if (id) post({ id, ok: false, error: String((e && e.message) || e) });
  } finally {
    if (busy) T.acts(false);
  }
}

function laps() {
  const done = [];
  let t = performance.now();
  const lap = (name) => {
    const n = performance.now();
    done.push(name + " " + Math.round(n - t));
    t = n;
  };
  lap.text = () => done.join(", ");
  return lap;
}

async function open(a, tab) {
  a.lap = laps();
  const res = await openTab(a, tab);
  const extras = a.quiet || !a.tabId ? {} : await pageExtras(a.tabId).catch(() => ({}));
  a.lap("extras");
  return Object.assign(typeof res === "string" ? { out: res } : res, extras, { created: a.created, steps: a.lap.text() });
}

async function openTab(a, tab) {
  const lap = a.lap || (() => {});
  let tabId = a.new ? null : await tabOf(tab, false);
  if (tabId === null) {
    tabId = await T.create(a.group);
    a.created = true;
  } else T.setTarget(tabId);
  a.tabId = tabId;
  lap("tab");
  await C.attach(tabId).catch(() => {});
  lap("attach");
  const nav = await T.navigate(tabId, a.url, LOAD_MS, LOADING_WAIT_MS);
  lap("load");
  const calm = await page(tabId, "act.settle", [300, SETTLE_MAX_MS, 0, 150], 0, 8000).catch(() => null);
  lap("settle");
  let t = await chrome.tabs.get(tabId);
  if (nav.error && nav.error !== "net::ERR_ABORTED") throw new Error("load failed: " + nav.error + " (" + T.short(a.url, 60) + ")");
  if (a.quiet) return header(tabId, { title: t.title, url: t.url });
  const narrow = !!(a.filter || a.scope);
  const opts = { force: true, filter: a.filter || null, scope: a.scope || null, media: !!a.media };
  let res = await page(tabId, "snap", [opts]).catch((e) => ({ error: e.message }));
  lap("snap");
  if (!narrow && !res.error && !(res.lines || []).length) {
    const peek = await page(tabId, "act.peek", [300]).catch(() => "");
    if (peek) res.peek = peek;
    else {
      await page(tabId, "act.settle", [400, 2500, 2500], 0, 8000).catch(() => {});
      res = await page(tabId, "snap", [opts]).catch((e) => ({ error: e.message }));
      t = await chrome.tabs.get(tabId);
      if (!res.error && !(res.lines || []).length) res.peek = await page(tabId, "act.peek", [300]).catch(() => "");
    }
  }
  if (res.error) return header(tabId, { title: t.title, url: t.url }) + "\n" + res.error;
  return withKeys(formatSnap(tabId, stillBusy(calm, res), narrow ? 0 : cfg.navLines, narrow), res);
}

async function history(kind, tab) {
  const tabId = await tabOf(tab);
  const nav = T.watchNav(tabId);
  if (kind === "back") await chrome.tabs.goBack(tabId).catch(() => {});
  else if (kind === "forward") await chrome.tabs.goForward(tabId).catch(() => {});
  else await chrome.tabs.reload(tabId);
  await nav.wait(1500, LOAD_MS);
  const calm = await page(tabId, "act.settle", [300, SETTLE_MAX_MS, 0, 150], 0, 8000).catch(() => null);
  const res = await page(tabId, "snap", [{ force: true }]);
  return formatSnap(tabId, stillBusy(calm, res), cfg.navLines);
}

async function tabCmd(a) {
  const id = T.rid(a.id);
  const t = id === null ? null : await chrome.tabs.get(id).catch(() => null);
  if (!t) throw new Error("no tab " + a.id + " -> try: wisply tabs");
  T.setTarget(id);
  return "target t" + T.sid(id) + " " + trunc(t.title, 50) + " - " + T.short(t.url, 60);
}

async function close(a, tab) {
  const id = a.id ? T.rid(a.id) : await tabOf(tab);
  if (id === null) throw new Error("no tab " + a.id + " -> try: wisply tabs");
  if (!a.force && !(await T.inGroup(id))) throw new Error("t" + T.sid(id) + " is not a Wisply tab -> add --force to close it anyway");
  await chrome.tabs.remove(id);
  if (id === T.getTarget()) T.setTarget(await T.previous(id));
  const next = T.getTarget();
  return "closed t" + T.sid(id) + (next ? ", target now t" + T.sid(next) : "");
}

const seen = new Map();
const NO_TAB = new Set(["config", "abort", "reload-extension", "ping", "close-url", "tabs", "downloads", "cleanup", "keep"]);
const BY_TEXT = new Set(["click", "type", "select", "check", "hover", "fill"]);

function pageKey(url) {
  try {
    const u = new URL(url);
    return u.origin + u.pathname + u.search;
  } catch (e) {
    return String(url || "");
  }
}

async function stale(cmd, a, tab) {
  if (!BY_TEXT.has(cmd) || (cmd !== "fill" && !a.text)) return;
  const id = await tabOf(tab, false).catch(() => null);
  if (id === null || !seen.has(id)) return;
  const t = await chrome.tabs.get(id).catch(() => null);
  if (!t || pageKey(t.url) === seen.get(id)) return;
  seen.set(id, pageKey(t.url));
  throw new Error("the page changed since the last command (now " + T.short(t.url, 70) + "), nothing done -> try: wisply snap, then repeat");
}

const BY_REF = new Set(["click", "type", "select", "check", "hover"]);

async function remember(tab) {
  const id = await tabOf(tab, false).catch(() => null);
  if (id === null) return null;
  const t = await chrome.tabs.get(id).catch(() => null);
  if (!t) return null;
  seen.set(id, pageKey(t.url));
  return { url: t.url, title: t.title, tab: "t" + T.sid(id) };
}

async function describeRef(ref, tab) {
  const id = await tabOf(tab, false).catch(() => null);
  if (id === null) return null;
  const info = await page(id, "act.info", [ref]).catch(() => null);
  return info && !info.gone ? { name: info.name, role: info.role, hint: info.hint } : null;
}

async function run(cmd, a, tab) {
  if (NO_TAB.has(cmd) || cmd.startsWith("record")) return route(cmd, a, tab);
  await stale(cmd, a, tab);
  const target = BY_REF.has(cmd) && a.ref ? await describeRef(a.ref, tab) : null;
  let res, where = null;
  try {
    res = await route(cmd, a, tab);
  } finally {
    where = await remember(typeof a.tabId === "number" ? a.tabId : tab);
  }
  const out = typeof res === "string" ? { out: res } : res || {};
  if (target) out.target = target;
  if (where) out.where = where;
  return out;
}

const rec = { on: false, tabs: new Set(), steps: [], started: 0 };

function recPush(step) {
  if (rec.on) rec.steps.push(Object.assign({ t: Date.now() }, step));
}

async function recInject(tabId) {
  await chrome.scripting.executeScript({ target: { tabId }, files: [...A.PAGE_FILES, "page-rec.js"] }).catch(() => {});
}

async function recStart(a, tab) {
  const tabId = await tabOf(tab);
  const t = await chrome.tabs.get(tabId);
  Object.assign(rec, { on: true, tabs: new Set([tabId]), steps: [], started: Date.now() });
  recPush({ cmd: "open", args: [t.url] });
  await recInject(tabId);
  return { out: "t" + T.sid(tabId) + " " + trunc(t.title, 50) + " - " + T.short(t.url, 60) };
}

async function recStop() {
  const was = rec.on;
  for (const id of rec.tabs) {
    await chrome.scripting.executeScript({ target: { tabId: id }, func: () => globalThis.__lt && globalThis.__lt.rec && globalThis.__lt.rec.stop() }).catch(() => {});
  }
  await T.sleep(80);
  rec.on = false;
  const steps = rec.steps;
  rec.steps = [];
  rec.tabs = new Set();
  return { out: was ? steps.length + " browser steps" : "not recording", steps };
}

chrome.runtime.onMessage.addListener((m, sender) => {
  if (!m || m.lt !== "rec" || !rec.on || !sender.tab || !rec.tabs.has(sender.tab.id) || sender.frameId) return;
  recPush(m.step);
});

chrome.webNavigation.onCommitted.addListener((d) => {
  if (!rec.on || d.frameId !== 0 || !rec.tabs.has(d.tabId)) return;
  const q = d.transitionQualifiers || [];
  if (d.transitionType === "reload") recPush({ cmd: "reload", args: [] });
  else if (q.includes("forward_back") || ["typed", "auto_bookmark", "generated", "keyword", "start_page"].includes(d.transitionType)) recPush({ cmd: "open", args: [d.url] });
});

chrome.webNavigation.onDOMContentLoaded.addListener((d) => {
  if (rec.on && d.frameId === 0 && rec.tabs.has(d.tabId)) recInject(d.tabId);
});

async function route(cmd, a, tab) {
  switch (cmd) {
    case "where":
      return { out: "" };
    case "record-start":
      return recStart(a, tab);
    case "record-stop":
      return recStop();
    case "record-status":
      return { out: rec.on ? rec.steps.length + " browser steps" : "not recording" };
    case "config":
      cfg.blocklist = a.blocklist || [];
      cfg.risk = a.risk || [];
      cfg.pointer = a.pointer !== false;
      T.setOwnWindow(a.window !== false);
      if (a.navLines) cfg.navLines = a.navLines;
      if (a.navChars) cfg.navChars = a.navChars;
      return {};
    case "abort":
      cfg.abort = true;
      setTimeout(() => (cfg.abort = false), 800);
      T.paint("red");
      return {};
    case "reload-extension":
      setTimeout(() => chrome.runtime.reload(), 100);
      return { out: "reloading extension" };
    case "ping":
      return { out: "pong" };
    case "close-url": {
      const all = await chrome.tabs.query({});
      const path = String(a.prefix || "").replace(/^[a-z]+:\/\//, "");
      const hits = all.filter((t) => path && (t.url || t.pendingUrl || "").replace(/^[a-z]+:\/\//, "").startsWith(path)).sort((x, y) => y.id - x.id);
      const n = a.all ? hits.length : Math.min(1, hits.length);
      for (const t of hits.slice(0, n)) await chrome.tabs.remove(t.id).catch(() => {});
      return { out: "closed " + n };
    }
    case "cleanup": {
      await T.paint("green", a.group);
      const only = Array.isArray(a.only) ? new Set(a.only.map((x) => T.rid(x)).filter((x) => x !== null)) : null;
      const tabs = (await T.groupTabs(a.group)).filter((t) => !only || only.has(t.id));
      const own = await T.ownWindows();
      const last = await chrome.windows.getLastFocused().catch(() => null);
      const looked = (t) => t.active && (!own.has(t.windowId) || (a.front !== false && last && last.focused && last.id === t.windowId && T.userPicked(t.windowId)));
      const shut = a.close === false ? [] : tabs.filter((t) => !looked(t)).map((t) => t.id);
      if (shut.length) await chrome.tabs.remove(shut).catch(() => {});
      if (shut.includes(T.getTarget())) T.setTarget(null);
      const why = tabs.filter((t) => !shut.includes(t.id)).map((t) => (own.has(t.windowId) ? "front" : "yours"));
      const focus = tabs.filter((t) => !shut.includes(t.id)).map((t) => Object.assign({ window: t.windowId, last: last ? last.id : null, lastFocused: !!(last && last.focused) }, T.focusInfo(t.windowId)));
      return { out: "", closed: shut.length, kept: tabs.length - shut.length, why, focus, shut: shut.map((id) => "t" + T.sid(id)) };
    }
    case "keep": {
      const only = Array.isArray(a.only) ? new Set(a.only.map((x) => T.rid(x)).filter((x) => x !== null)) : null;
      const ids = a.id ? [T.rid(a.id)].filter((x) => x !== null) : (await T.groupTabs(a.group)).map((t) => t.id).filter((id) => !only || only.has(id));
      if (ids.length) await chrome.tabs.ungroup(ids).catch(() => {});
      return { out: "", kept: ids.length };
    }
    case "tabs":
      return a.rows ? T.rows() : T.list(!!a.all);
    case "tab":
      return tabCmd(a);
    case "open":
      return open(a, tab);
    case "back":
    case "forward":
    case "reload":
      return history(cmd, tab);
    case "close":
      return close(a, tab);
    case "snap":
      return snap(a, tab);
    case "text":
      return text(a, tab);
    case "table":
      return table(a, tab);
    case "click":
      return A.click(a, tab);
    case "type":
      return A.type(a, tab);
    case "fill":
      return A.fill(a, tab);
    case "press":
      return A.press(a, tab);
    case "select":
      return A.select(a, tab);
    case "check":
      return A.check(a, tab);
    case "hover":
      return A.hover(a, tab);
    case "drag":
      return A.drag(a, tab);
    case "scroll":
      return A.scroll(a, tab);
    case "wait":
      return wait(a, tab);
    case "search-field":
      return page(await tabOf(tab), "act.searchField", []);
    case "media-audio":
      return mediaAudio(a, tab);
    case "net":
      return net(a, tab);
    case "webmcp":
      return webmcp(a, tab);
    case "expect":
      return expect(a, tab);
    case "js":
      return js(a, tab);
    case "fetch":
      return fetchJson(a, tab);
    case "shot":
      return shot(a, tab);
    case "frames":
      return frames(a, tab);
    case "upload":
      return upload(a, tab);
    case "dialog":
      return dialog(a, tab);
    case "downloads":
      return downloads();
    case "console":
      return consoleCmd(a, tab);
    case "viewport":
      return viewport(a, tab);
    case "dismiss":
      return dismiss(a, tab);
    default:
      throw new Error("unknown browser command " + cmd);
  }
}

C.onEvent((kind, tabId, params) => {
  if (kind === "dialog") event("dialog on t" + T.sid(tabId) + ": " + params.type + ' "' + trunc(params.message, 80) + '" -> wisply dialog accept|dismiss');
});

chrome.webNavigation.onCreatedNavigationTarget.addListener(async (d) => {
  const src = d.sourceTabId;
  if (src !== T.getTarget() && !(await T.inGroup(src))) return;
  if (rec.on && rec.tabs.has(src)) rec.tabs.add(d.tabId);
  await T.addToGroup(d.tabId, (await T.groupOf(src)) || undefined);
  chrome.tabs.update(d.tabId, { autoDiscardable: false }).catch(() => {});
  T.setTarget(d.tabId);
  event("new tab t" + T.sid(d.tabId) + " opened from t" + T.sid(src) + " (now the target; wisply tab t" + T.sid(src) + " to go back)");
});

chrome.downloads.onChanged.addListener(async (d) => {
  if (!d.state || d.state.current !== "complete") return;
  const [item] = await chrome.downloads.search({ id: d.id });
  if (item) event("download done: " + item.filename);
});

chrome.windows.onFocusChanged.addListener((id) => {
  T.noteFocus(id);
  post({ event: "focus", focused: id !== chrome.windows.WINDOW_ID_NONE });
});

chrome.alarms.onAlarm.addListener(() => {
  C.sweep();
  if (!port) connect();
});

const ready = T.restore();
chrome.runtime.onStartup.addListener(async () => {
  await ready;
  await T.fresh();
  if (port) post(Object.assign({ event: "hello", ext: VERSION, focused: false, epoch: T.getEpoch() }, brandInfo()));
  else connect();
});
chrome.runtime.onInstalled.addListener(() => {
  chrome.alarms.create("wisply", { periodInMinutes: 0.5 });
  connect();
});

chrome.alarms.get("wisply").then((a) => {
  if (!a) chrome.alarms.create("wisply", { periodInMinutes: 0.5 });
});
ready.then(connect);
