import * as T from "./tabs.js";
import { cfg, page, formatSnap, header, tabOf } from "./actions.js";

const FRAME_WAIT_MS = 3000;
const SETTLE_MAX_MS = 1500;
const BUSY_NOTE = "page still changing after " + SETTLE_MAX_MS / 1000 + " s: it may still be loading (a second snap shows only what is new)";

function stillBusy(calm, res) {
  if (calm && calm.changed && calm.ms >= SETTLE_MAX_MS - 30 && res && !res.error) res.notes = (res.notes || []).concat(BUSY_NOTE);
  return res;
}
function withKeys(out, res) {
  return res && res.keys && res.keys.length ? { out, keys: res.keys } : out;
}

function hostOf(url) {
  try {
    return new URL(url).host.toLowerCase();
  } catch (e) {
    return "";
  }
}

async function frameId(tabId, host) {
  const want = String(host || "").replace(/^[a-z]+:\/\//i, "").replace(/[….]+$/, "").toLowerCase();
  if (!want) throw new Error("--frame needs a host -> try: wisply snap (iframe lines name it)");
  const end = Date.now() + FRAME_WAIT_MS;
  let frames = [];
  for (;;) {
    frames = ((await chrome.webNavigation.getAllFrames({ tabId })) || []).filter((f) => f.frameId !== 0 && f.url);
    const hit = frames.find((f) => f.url.toLowerCase().includes(want)) || frames.find((f) => {
      const h = hostOf(f.url);
      return h && (h.startsWith(want) || want.startsWith(h));
    });
    if (hit) return hit.frameId;
    if (Date.now() >= end) break;
    await new Promise((r) => setTimeout(r, 200));
  }
  const hosts = [...new Set(frames.map((f) => hostOf(f.url)).filter(Boolean))].slice(0, 6);
  throw new Error("no frame matching " + host + (hosts.length ? " (frames now: " + hosts.join(", ") + ")" : " (no frames yet: the page may still be loading)") + " -> try: wisply snap");
}

async function snap(a, tab) {
  const tabId = await tabOf(tab);
  const opts = { all: !!a.all, scope: a.scope || null, filter: a.filter || null, diff: !!a.diff, force: !!a.force || !!a.full || !!a.links, media: !!a.media, links: !!a.links };
  opts.plain = !a.full && !a.frame && !opts.all && !opts.scope && !opts.filter && !opts.diff && !opts.force && !opts.media;
  if (a.frame) {
    const fid = await frameId(tabId, a.frame);
    const res = await page(tabId, "snap", [Object.assign(opts, { force: true })], fid);
    return formatSnap(tabId, res, 0, true).replace(/^e(\d+) /gm, "f" + fid + ".e$1 ").replace(/^\[t\d+\]/, "[t" + T.sid(tabId) + " frame " + a.frame + "]");
  }
  const res = await page(tabId, "snap", [opts]);
  return withKeys(formatSnap(tabId, res, 0, opts.all || !!opts.scope || !!opts.filter || opts.force || opts.diff || opts.media), res);
}

async function text(a, tab) {
  const tabId = await tabOf(tab);
  let why = "";
  if (!a.raw) {
    try {
      const [probe] = await chrome.scripting.executeScript({ target: { tabId }, func: () => !!globalThis.Defuddle, injectImmediately: true });
      if (!probe.result) await chrome.scripting.executeScript({ target: { tabId }, files: ["defuddle.js"], injectImmediately: true });
    } catch (e) {
      why = String((e && e.message) || e).slice(0, 120);
    }
  }
  const res = await page(tabId, "act.text", [{ filter: a.filter || null, raw: !!a.raw, links: !!a.links, why }], 0, 25000);
  return header(tabId, { title: res.title, url: res.url, loading: res.loading }) + "\n" + res.text;
}

async function table(a, tab) {
  const tabId = await tabOf(tab);
  const res = await page(tabId, "act.table", [a.target || null]);
  if (res.none) throw new Error("no table found -> try: wisply snap");
  return res.ref + " table " + res.rows.length + " rows\n" + res.rows.join("\n");
}

async function wait(a, tab) {
  const tabId = await tabOf(tab);
  const ms = Number(a.timeout || 10000);
  const t0 = Date.now();
  if (a.ms) {
    await T.sleep(Math.min(Number(a.ms), 60000));
    return "ok (" + a.ms + " ms)";
  }
  if (a.url) {
    while (Date.now() - t0 < ms) {
      if (cfg.abort) throw new Error("stopped by hotkey");
      const t = await chrome.tabs.get(tabId);
      const hit = (t.url || "").toLowerCase().includes(a.url.toLowerCase());
      if (a.gone ? !hit : hit) return "ok (" + (Date.now() - t0) + " ms) -> " + T.short(t.url, 80);
      await T.sleep(100);
    }
    throw new Error("timeout after " + ms + " ms waiting for url " + (a.gone ? "to leave " : "") + a.url);
  }
  const spec = { text: a.text || null, ref: a.ref || null, css: a.css || null, gone: !!a.gone };
  while (Date.now() - t0 < ms) {
    const left = ms - (Date.now() - t0);
    try {
      const r = await page(tabId, "act.waitFor", [spec, Math.min(left, 4000)], 0, Math.min(left, 4000) + 3000);
      if (r.ok) return "ok (" + (Date.now() - t0) + " ms)";
    } catch (e) {
      if (/dialog open/.test(e.message)) throw e;
      await T.sleep(150);
    }
    if (cfg.abort) throw new Error("stopped by hotkey");
  }
  throw new Error("timeout after " + ms + " ms waiting for " + (a.gone ? "gone " : "") + (a.text || a.ref || a.css));
}

async function expect(a, tab) {
  const tabId = await tabOf(tab);
  if (a.url) {
    const t = await chrome.tabs.get(tabId);
    const hit = (t.url || "").toLowerCase().includes(a.url.toLowerCase());
    if (a.gone ? !hit : hit) return "ok";
    throw new Error("fail: url is " + T.short(t.url, 80));
  }
  const spec = { text: a.text || null, ref: a.ref || null, css: a.css || null, gone: !!a.gone };
  const r = await page(tabId, "act.waitFor", [spec, 0]);
  if (r.ok) return "ok";
  throw new Error("fail: " + (a.gone ? "still present: " : "not found: ") + (a.text || a.ref || a.css));
}

export { SETTLE_MAX_MS, stillBusy, withKeys, snap, text, table, wait, expect };
