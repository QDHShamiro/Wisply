import * as C from "./cdp.js";
import * as T from "./tabs.js";
import { page, guard, tabOf, trunc } from "./actions.js";

async function net(a, tab) {
  const tabId = await tabOf(tab);
  const list = C.netList(tabId);
  if (a.op !== "body") return { calls: list.map((x) => ({ id: x.id, url: x.url, status: x.status, size: x.size })) };
  const hit = list.find((x) => x.id === a.id);
  if (!hit) throw new Error("that call is gone -> try: wisply net");
  const b = await C.netBody(tabId, hit.id).catch((e) => {
    throw new Error("the browser no longer has that answer (" + String(e.message || e).slice(0, 60) + ") -> reload and try again");
  });
  const text = b.base64Encoded ? new TextDecoder().decode(Uint8Array.from(atob(b.body), (c) => c.charCodeAt(0))) : b.body;
  return { url: hit.url, size: hit.size, body: text.slice(0, 200000) };
}

const TOOL_LIST = "JSON.stringify({ api: !!(document.modelContext || navigator.modelContext), tools: [...(window.__ltWebMCP || new Map()).values()].map((t) => ({ name: t.name, description: String(t.description || '').slice(0, 200), readOnly: !!(t.annotations && t.annotations.readOnlyHint), schema: t.inputSchema || null })) })";

async function webmcp(a, tab) {
  const tabId = await tabOf(tab);
  if (a.op === "list") return JSON.parse(await C.evaluate(tabId, TOOL_LIST));
  const name = JSON.stringify(String(a.name || ""));
  const args = JSON.stringify(a.args || {});
  const code = "(async () => { const t = (window.__ltWebMCP || new Map()).get(" + name + "); if (!t) throw new Error('no tool ' + " + name + "); const r = await t.execute(" + args + ", { requestUserInteraction: async (cb) => cb() }); return JSON.stringify(r === undefined ? null : r); })()";
  return { result: await C.evaluate(tabId, code) };
}

async function pageExtras(tabId) {
  const bigJson = C.netList(tabId).filter((x) => x.size > 1024).length;
  const tools = await C.evaluate(tabId, "window.__ltWebMCP ? window.__ltWebMCP.size : 0").catch(() => 0);
  return { net: bigJson, webmcp: tools || 0 };
}

function wrap(code) {
  if (/\breturn\b/.test(code)) return "(async()=>{" + code + "\n})()";
  if (/(^|[;{}\n])\s*(const|let|class)\s/.test(code)) return "{" + code + "\n}";
  return code;
}

async function js(a, tab) {
  const tabId = await tabOf(tab);
  await guard(tabId, "js");
  await C.attach(tabId);
  const replMode = /\bawait\b/.test(a.code) && !/\breturn\b/.test(a.code);
  const r = await C.send(tabId, "Runtime.evaluate", { expression: wrap(a.code), awaitPromise: true, returnByValue: true, userGesture: true, replMode, timeout: 15000 });
  if (r.exceptionDetails) {
    const d = r.exceptionDetails;
    throw new Error("js: " + String((d.exception && d.exception.description) || d.text).split("\n")[0]);
  }
  const v = r.result;
  if (v.type === "undefined") return "undefined";
  if (v.value === undefined) return v.description || v.type;
  return typeof v.value === "string" ? v.value : JSON.stringify(v.value);
}

async function fetchJson(a, tab) {
  const tabId = await tabOf(tab);
  await guard(tabId, "fetch");
  await C.attach(tabId);
  const init = { method: a.method || "GET", credentials: a.cookies ? "include" : "same-origin" };
  if (a.body) {
    init.body = a.body;
    init.headers = { "content-type": /^\s*[{[]/.test(a.body) ? "application/json" : "application/x-www-form-urlencoded" };
  }
  const expr = "fetch(" + JSON.stringify(a.url) + "," + JSON.stringify(init) + ").then(async r=>({status:r.status,type:r.headers.get('content-type')||'',text:await r.text(),url:r.url}))";
  const r = await C.send(tabId, "Runtime.evaluate", { expression: expr, awaitPromise: true, returnByValue: true, timeout: 20000 });
  if (r.exceptionDetails) throw new Error("fetch failed: " + String((r.exceptionDetails.exception && r.exceptionDetails.exception.description) || r.exceptionDetails.text).split("\n")[0]);
  return { fetch: r.result.value };
}

function markedIn(doc, id) {
  const el = doc.querySelector('[data-lt-mark="' + id + '"]');
  if (el) {
    el.removeAttribute("data-lt-mark");
    return el;
  }
  for (const f of doc.querySelectorAll("iframe, frame")) {
    try {
      const hit = f.contentDocument && markedIn(f.contentDocument, id);
      if (hit) return hit;
    } catch (e) {}
  }
  return null;
}

async function upload(a, tab) {
  const tabId = await tabOf(tab);
  await guard(tabId, "upload");
  await C.attach(tabId);
  const id = "u" + Date.now();
  const m = await page(tabId, "act.mark", [a.ref, id, true]);
  if (m.gone) throw new Error(a.ref + " is gone -> try: wisply snap");
  if (m.nofile) throw new Error("no file input at or near " + a.ref + " -> try: the upload button or its label");
  const fr = /^f(\d+)\.e\d+$/.exec(a.ref);
  if (fr) {
    if (!a.data) throw new Error("file too big for a frame upload (max 8 MB)");
    const [r] = await chrome.scripting.executeScript({ target: { tabId, frameIds: [Number(fr[1])] }, args: [id, a.data], func: (mark, data) => {
      const el = document.querySelector('[data-lt-mark="' + mark + '"]');
      el && el.removeAttribute("data-lt-mark");
      const inp = el && el.matches('input[type=file]') ? el : null;
      if (!inp) return "no file input near the ref";
      const dt = new DataTransfer();
      for (const f of data) dt.items.add(new File([Uint8Array.from(atob(f.b64), (c) => c.charCodeAt(0))], f.name, { type: f.mime }));
      inp.files = dt.files;
      inp.dispatchEvent(new Event("input", { bubbles: true }));
      inp.dispatchEvent(new Event("change", { bubbles: true }));
      return "";
    } });
    if (r && r.result) throw new Error(r.result);
    return "ok (" + a.data.length + " file" + (a.data.length === 1 ? "" : "s") + " set on " + a.ref + ")";
  }
  try {
    await C.send(tabId, "DOM.getDocument", { depth: 0 });
    const r = await C.send(tabId, "Runtime.evaluate", { expression: "(" + markedIn + ")(document, " + JSON.stringify(id) + ")" });
    if (!r.result || !r.result.objectId) throw new Error("file input not reachable (inside a frame?)");
    await C.send(tabId, "DOM.setFileInputFiles", { objectId: r.result.objectId, files: a.files });
  } finally {
    await page(tabId, "act.unmark", []).catch(() => {});
  }
  return "ok (" + a.files.length + " file" + (a.files.length === 1 ? "" : "s") + " set on " + a.ref + ")";
}

async function dialog(a, tab) {
  const tabId = await tabOf(tab);
  const d = C.dialog(tabId);
  if (!d) return "no dialog open";
  await C.answerDialog(tabId, a.accept !== false, a.text);
  return "ok (" + (a.accept !== false ? "accepted" : "dismissed") + " " + d.type + ' "' + trunc(d.message, 60) + '")';
}

async function downloads() {
  const items = await chrome.downloads.search({ limit: 8, orderBy: ["-startTime"] });
  if (!items.length) return "no downloads";
  return items.map((d) => {
    const st = d.state === "complete" ? "done" : d.state === "interrupted" ? "failed" : Math.round((d.bytesReceived / Math.max(1, d.totalBytes)) * 100) + "%";
    const size = d.fileSize > 0 ? " " + (d.fileSize / 1048576).toFixed(1) + " MB" : "";
    return st + size + " " + (d.filename || T.short(d.url, 80));
  }).join("\n");
}

async function innerWidth(tabId) {
  const r = await C.send(tabId, "Runtime.evaluate", { expression: "innerWidth", returnByValue: true }).catch(() => null);
  return r && r.result ? r.result.value : null;
}

async function viewport(a, tab) {
  const tabId = await tabOf(tab);
  await C.attach(tabId);
  const before = await innerWidth(tabId);
  const mobile = a.width <= 600;
  const changed = async (ms) => {
    const t0 = Date.now();
    while (Date.now() - t0 < ms) {
      if ((await innerWidth(tabId)) !== before) return true;
      await T.sleep(25);
    }
    return false;
  };
  if (!a.width) {
    await C.send(tabId, "Emulation.clearDeviceMetricsOverride");
    await C.send(tabId, "Emulation.setTouchEmulationEnabled", { enabled: false }).catch(() => {});
  } else {
    await C.send(tabId, "Emulation.setDeviceMetricsOverride", { width: a.width, height: a.height, deviceScaleFactor: 0, mobile });
    await C.send(tabId, "Emulation.setTouchEmulationEnabled", { enabled: mobile, maxTouchPoints: mobile ? 5 : 1 }).catch(() => {});
  }
  if (!(await changed(250))) await T.showBriefly(tabId, () => changed(500));
  if (!a.width) return "ok (viewport reset)";
  return "ok (viewport " + a.width + "x" + a.height + (mobile ? ", mobile" : "") + ", until wisply viewport reset or 5 min idle)";
}

async function consoleCmd(a, tab) {
  const tabId = await tabOf(tab);
  const fresh = !C.isAttached(tabId);
  await C.enableConsole(tabId);
  const lines = C.consoleLines(tabId).slice(-30);
  if (!lines.length) return fresh ? "console capture is on now; no errors yet (reload to catch load errors)" : "no console errors";
  return lines.join("\n");
}

async function dismiss(a, tab) {
  const tabId = await tabOf(tab);
  await guard(tabId, "dismiss");
  const r = await page(tabId, "act.dismiss", []);
  if (r.none) return "no cookie banner found";
  await page(tabId, "act.settle", [200, 1500]).catch(() => {});
  return 'ok (' + r.kind + ' "' + r.clicked + '")';
}

export { net, webmcp, pageExtras, js, fetchJson, upload, dialog, downloads, viewport, consoleCmd, dismiss };
