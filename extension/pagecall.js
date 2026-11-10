import * as C from "./cdp.js";
import * as T from "./tabs.js";

export const cfg = { blocklist: [], risk: [], pointer: true, navLines: 40, navChars: 1600, abort: false };

const LOAD_MS = 20000;
const LOADING_WAIT_MS = 8000;
const EARLY = new Set(["snap", "diff", "act.text", "act.visibleText", "act.table", "act.peek", "act.settle"]);
const PAGE_FILES = ["page.js", "page-media.js", "page-snap.js", "page-read.js", "page-act.js"];

function trunc(s, n) {
  s = String(s || "");
  return s.length > n ? s.slice(0, n - 3) + "..." : s;
}

function blockedHost(url) {
  try {
    const h = new URL(url).hostname.replace(/^www\./, "").toLowerCase();
    return cfg.blocklist.find((d) => h === d || h.endsWith("." + d)) || null;
  } catch (e) {
    return null;
  }
}

function risky(name) {
  if (!cfg.risk.length || !name) return false;
  const n = name.toLowerCase();
  return cfg.risk.some((w) => new RegExp("(^|[^\\p{L}])" + w.replace(/[.*+?^${}()|[\]\\]/g, "\\$&") + "([^\\p{L}]|$)", "u").test(n));
}

function callLt(fn, args, wait) {
  const L = globalThis.__lt;
  if (!L || !L.act) return { __missing: true };
  const nav = wait ? performance.getEntriesByType("navigation")[0] : null;
  if (nav && !(nav.domContentLoadedEventStart > 0)) {
    return new Promise((ok) => {
      let done = false;
      const go = (late) => {
        if (done) return;
        done = true;
        Promise.resolve(callLt(fn, args, 0)).then((r) => ok(late && r && typeof r === "object" ? Object.assign(r, { loading: true }) : r));
      };
      document.addEventListener("DOMContentLoaded", () => go(false), { once: true });
      setTimeout(() => go(true), Math.max(0, wait - performance.now()));
    });
  }
  const failed = (e) => ({ __error: String((e && e.message) || e).slice(0, 200) });
  try {
    const dot = fn.indexOf(".");
    const f = dot > 0 ? L[fn.slice(0, dot)][fn.slice(dot + 1)] : L[fn];
    const r = f.apply(null, args || []);
    return r && typeof r.then === "function" ? r.then((v) => v, failed) : r;
  } catch (e) {
    return failed(e);
  }
}

function friendly(e, url) {
  const m = String((e && e.message) || e);
  if (/cannot access|cannot be scripted|chrome:\/\/|brave:\/\/|edge:\/\/|extensions gallery|Missing host permission/i.test(m)) {
    return new Error("cannot script this page (" + T.short(url || "", 60) + ") -> try: wisply windows, then wisply snap w<N> (desktop control)");
  }
  if (/No tab with id|Tab .* not found/i.test(m)) return new Error("tab is gone -> try: wisply tabs");
  if (/Frame with ID .* was removed|document.*unloaded|Execution context was destroyed/i.test(m)) return new Error("page navigated during the command -> try: wisply snap");
  return e instanceof Error ? e : new Error(m);
}

async function withDialog(tabId, promise, ms) {
  let timer;
  const guard = new Promise((_, reject) => {
    const t0 = Date.now();
    const tick = () => {
      const d = C.dialog(tabId);
      if (d) return reject(new Error("dialog open: " + d.type + ' "' + trunc(d.message, 80) + '" -> try: wisply dialog accept|dismiss'));
      if (Date.now() - t0 > ms) return reject(new Error("page did not answer within " + Math.round(ms / 1000) + "s"));
      timer = setTimeout(tick, 100);
    };
    timer = setTimeout(tick, 100);
  });
  try {
    return await Promise.race([promise, guard]);
  } finally {
    clearTimeout(timer);
  }
}

const FRAME_REF = /^f(\d+)\.(e\d+)$/;

async function frameOrigin(tabId, fid) {
  const frames = (await chrome.webNavigation.getAllFrames({ tabId })) || [];
  const me = frames.find((f) => f.frameId === fid);
  if (!me) throw new Error("frame f" + fid + " is gone -> try: wisply snap");
  const size = await page(tabId, "act.viewSize", [], fid);
  const twins = frames.filter((f) => f.parentFrameId === me.parentFrameId && f.url === me.url).sort((x, y) => x.frameId - y.frameId);
  const box = await page(tabId, "act.frameBox", [me.url, size, twins.indexOf(me)], me.parentFrameId);
  if (!box) throw new Error("frame f" + fid + " is not on the page any more -> try: wisply snap");
  const up = me.parentFrameId > 0 ? await frameOrigin(tabId, me.parentFrameId) : { x: 0, y: 0 };
  return { x: box.x + up.x, y: box.y + up.y };
}

async function inFrame(tabId, fid, fn, args, ms) {
  const tag = (s) => (typeof s === "string" ? s.replace(/^e(\d+) /, "f" + fid + ".e$1 ") : s);
  let res;
  try {
    res = await page(tabId, fn, args, fid, ms);
  } catch (e) {
    if (/no frame with id/i.test(String(e && e.message))) throw new Error("frame f" + fid + " is gone -> try: wisply snap");
    throw e;
  }
  if (res && typeof res === "object") {
    if (typeof res.x === "number" && typeof res.y === "number") {
      const o = await frameOrigin(tabId, fid);
      res.x = Math.round(res.x + o.x);
      res.y = Math.round(res.y + o.y);
    }
    if (res.line) res.line = tag(res.line);
  }
  return tag(res);
}

export async function page(tabId, fn, args, frameId, ms) {
  const fr = !frameId && args && typeof args[0] === "string" ? FRAME_REF.exec(args[0]) : null;
  if (fr) return inFrame(tabId, Number(fr[1]), fn, [fr[2]].concat(args.slice(1)), ms);
  const target = { tabId, frameIds: [frameId || 0] };
  const early = EARLY.has(fn);
  const call = () => chrome.scripting.executeScript({ target, func: callLt, args: [fn, args || [], early ? LOADING_WAIT_MS : 0], injectImmediately: early });
  let tab = null;
  try {
    tab = await chrome.tabs.get(tabId);
    let [r] = await withDialog(tabId, call(), ms || 15000);
    if (r && r.result && r.result.__missing) {
      await chrome.scripting.executeScript({ target, files: PAGE_FILES, injectImmediately: early });
      [r] = await withDialog(tabId, call(), ms || 15000);
    }
    if (!r) throw new Error("page did not answer");
    if (r.result && r.result.__error) throw new Error("page script error: " + r.result.__error + " -> try: wisply text or wisply shot");
    return r.result;
  } catch (e) {
    throw friendly(e, tab && tab.url);
  }
}

async function tabOf(requested, need) {
  const id = await T.resolve(requested);
  if (id === null && need !== false) throw new Error("no tab yet -> try: wisply open <url>  (or wisply tabs + wisply tab <id>)");
  return id;
}

async function guard(tabId, what) {
  const tab = await chrome.tabs.get(tabId);
  const b = blockedHost(tab.url || tab.pendingUrl || "");
  if (b) throw new Error("blocked: " + b + " is read-only for wisply (" + what + ") - edit ~/.wisply/blocklist.txt to change");
  return tab;
}

export { trunc, blockedHost, risky, FRAME_REF, tabOf, guard, LOAD_MS, LOADING_WAIT_MS, PAGE_FILES };
