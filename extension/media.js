import * as C from "./cdp.js";
import * as T from "./tabs.js";
import { page, tabOf } from "./actions.js";

const PLAY_MARKED = "(() => { const v = document.querySelector('[data-lt-play]'); return v ? v.play().then(() => true, () => false) : false; })()";

async function mediaAudio(a, tab) {
  const tabId = await tabOf(tab);
  let ref = a.ref;
  if (!ref) {
    const found = await page(tabId, "firstMedia", []);
    if (found.none) throw new Error("no video or audio on this page -> try: wisply listen <url>");
    ref = found.ref;
  }
  if (a.op === "cues") return Object.assign({ ref }, await page(tabId, "media", [ref, "cues"], 0, 8000));
  const t = await chrome.tabs.get(tabId);
  const muted = !!(t.mutedInfo && t.mutedInfo.muted);
  if (!muted) await chrome.tabs.update(tabId, { muted: true }).catch(() => {});
  let started = false;
  try {
    if (typeof a.from === "number") await page(tabId, "media", [ref, "seek", a.from], 0, 12000);
    const st = await page(tabId, "media", [ref, "mark"], 0, 8000).catch(() => null);
    if (st && st.paused) {
      await C.attach(tabId).catch(() => {});
      const r = await C.send(tabId, "Runtime.evaluate", { expression: PLAY_MARKED, awaitPromise: true, returnByValue: true, userGesture: true, timeout: 5000 }).catch(() => null);
      started = !!(r && r.result && r.result.value);
    }
    await page(tabId, "media", [ref, "unmark"], 0, 8000).catch(() => {});
    return Object.assign({ ref }, await page(tabId, "media", [ref, "record", a.ms], 0, a.ms + 20000));
  } finally {
    if (started) await page(tabId, "media", [ref, "pause"], 0, 8000).catch(() => {});
    if (!muted) await chrome.tabs.update(tabId, { muted: false }).catch(() => {});
  }
}

async function shot(a, tab) {
  const tabId = await tabOf(tab);
  const t = await chrome.tabs.get(tabId);
  const win = await chrome.windows.get(t.windowId);
  if (win.state === "minimized") throw new Error("browser window is minimized -> restore it, or use: wisply shot screen");
  return T.showBriefly(tabId, async () => {
    await C.attach(tabId);
    if (a.ref && !a.marks) return { image: await clipShot(tabId, a.ref, a.quality || 70) };
    let crop = null;
    if (a.ref) {
      const r = await page(tabId, "rect", [a.ref]);
      if (r.gone) throw new Error(a.ref + " is gone -> try: wisply snap");
      crop = { x: r.x - r.w / 2, y: r.y - r.h / 2, w: r.w, h: r.h };
    }
    let marks = 0;
    if (a.marks) {
      await page(tabId, "snap", [{ force: true }]);
      marks = (await page(tabId, "act.marks", [true])).n;
    }
    try {
      const m = await C.send(tabId, "Page.getLayoutMetrics");
      const vp = m.cssVisualViewport || m.visualViewport || {};
      const img = await C.send(tabId, "Page.captureScreenshot", { format: "jpeg", quality: a.quality || 70 });
      return { image: img.data, crop, vw: vp.clientWidth, vh: vp.clientHeight, marks };
    } finally {
      if (a.marks) await page(tabId, "act.marks", [false]).catch(() => {});
    }
  });
}

async function frames(a, tab) {
  const tabId = await tabOf(tab);
  const t = await chrome.tabs.get(tabId);
  const win = await chrome.windows.get(t.windowId);
  if (win.state === "minimized") throw new Error("browser window is minimized -> restore it");
  const muted = !!(t.mutedInfo && t.mutedInfo.muted);
  if (!muted) await chrome.tabs.update(tabId, { muted: true }).catch(() => {});
  return T.showBriefly(tabId, async () => {
    await C.attach(tabId);
    const info = await page(tabId, "media", [a.ref, "info"]);
    if (info.gone) throw new Error(a.ref + " is gone -> try: wisply snap");
    if (info.notMedia) throw new Error(a.ref + " is not a video -> try: wisply snap -f video");
    if (!info.seekable && !a.live) Object.assign(info, await page(tabId, "media", [a.ref, "wait"], 0, 8000));
    const live = a.live || !info.seekable;
    const out = [];
    try {
      if (live) {
        for (let i = 0; i < a.count; i++) {
          if (i) await T.sleep(a.every);
          const st = await page(tabId, "media", [a.ref, "state"]);
          out.push({ image: await clipShot(tabId, a.ref, 70), media: st.media, cap: st.cap });
        }
      } else {
        const from = Math.max(0, Math.min(a.from || 0, info.duration - 0.5));
        const to = Math.max(from + 0.5, Math.min(a.to || info.duration, info.duration));
        const n = a.samples || a.count;
        await page(tabId, "media", [a.ref, "prime"], 0, 8000);
        for (let i = 0; i < n; i++) {
          const st = await page(tabId, "media", [a.ref, "seek", from + ((i + 0.5) * (to - from)) / n], 0, 12000);
          out.push({ image: await clipShot(tabId, a.ref, 70), media: st.media, cap: st.cap });
        }
      }
    } finally {
      await page(tabId, "media", [a.ref, "restore", live ? -1 : 0]).catch(() => {});
      if (!muted) await chrome.tabs.update(tabId, { muted: false }).catch(() => {});
    }
    return { frames: out, live, duration: info.duration };
  });
}

async function clipShot(tabId, ref, quality) {
  const r = await page(tabId, "rect", [ref]);
  if (r.gone) throw new Error(ref + " is gone -> try: wisply snap");
  const m = await C.send(tabId, "Page.getLayoutMetrics");
  const vp = m.cssVisualViewport || m.visualViewport || {};
  const clip = { x: r.x - r.w / 2 + (vp.pageX || 0), y: r.y - r.h / 2 + (vp.pageY || 0), width: Math.max(1, r.w), height: Math.max(1, r.h), scale: 1 };
  const img = await C.send(tabId, "Page.captureScreenshot", { format: "jpeg", quality, clip });
  return img.data;
}

export { mediaAudio, shot, frames };
