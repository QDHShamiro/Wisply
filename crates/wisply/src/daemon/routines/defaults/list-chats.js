const want = String(args.app || "all").trim().toLowerCase();
const apps = want === "all" || !want ? ["discord", "whatsapp"] : want.split(",").map((a) => a.trim());
const cap = Number(args.max) > 0 ? Number(args.max) : 400;

const SITE = {
  discord: {
    host: "discord.com",
    tab: "discord.com/channels",
    prefer: "discord.com/channels/@me",
    url: "https://discord.com/channels/@me",
    list: '[class*="privateChannels"] a[href^="/channels/@me/"], nav a[href^="/channels/@me/"]',
    steps: 40,
  },
  whatsapp: {
    host: "web.whatsapp.com",
    tab: "web.whatsapp.com",
    prefer: "(",
    url: "https://web.whatsapp.com/",
    ready: "#pane-side",
    list: "#pane-side span[title]",
    probe: "#pane-side",
    steps: 40,
  },
};

function clean(v) {
  return String(v || "")
    .replace(/\p{Cf}/gu, "")
    .replace(/\s+/g, " ")
    .trim();
}

async function until(test, ms) {
  for (let t = 0; t < ms; t += 100) {
    const v = test();
    if (v) return v;
    await new Promise((r) => setTimeout(r, 100));
  }
  return test();
}

function scroller(el) {
  for (let n = el; n; n = n.parentElement) {
    if (n.scrollHeight > n.clientHeight + 4 && /(auto|scroll)/.test(getComputedStyle(n).overflowY)) return n;
  }
  return null;
}

async function sweep(read, d) {
  const out = new Map();
  const first = document.querySelector(d.list);
  const box = first ? scroller(first) : null;
  const start = box ? box.scrollTop : 0;
  if (box) box.scrollTop = 0;
  for (let i = 0; i < d.steps; i++) {
    await new Promise((r) => setTimeout(r, 200));
    for (const row of read(d)) if (!out.has(row.key)) out.set(row.key, row.line);
    if (!box || box.scrollTop + box.clientHeight >= box.scrollHeight - 4) break;
    box.scrollTop += Math.max(box.clientHeight * 0.8, 200);
  }
  if (box) box.scrollTop = start;
  return [...out.values()].slice(0, d.cap);
}

function dcRead(d) {
  const rows = [];
  for (const a of document.querySelectorAll(d.list)) {
    const path = a.getAttribute("href") || "";
    const label = clean(a.getAttribute("aria-label"));
    if (!/^\/channels\/@me\/\d+$/.test(path) || !label) continue;
    rows.push({ key: path, line: `discord\t${label}\t${path}` });
  }
  return rows;
}

function waRead(d) {
  const seen = new Set();
  const rows = [];
  for (const t of document.querySelectorAll(d.list)) {
    const row = t.closest('[role="listitem"], [role="row"]');
    const title = clean(t.getAttribute("title") || t.textContent);
    if (!row || !title || seen.has(row)) continue;
    seen.add(row);
    const lines = String(row.innerText || "")
      .split("\n")
      .map((l) => l.trim());
    const prefixed = lines.slice(1).some((l) => l.startsWith(":") || /^\S.{0,40}\s:\s/.test(l));
    const group = !!row.querySelector('[data-icon*="group"]') || prefixed;
    rows.push({ key: title, line: `whatsapp\t${title}\t${group ? "group" : "chat"}` });
  }
  return rows;
}

function dcReady(d) {
  return !!document.querySelector(d.list);
}

async function dcOpen(d) {
  if (await until(() => dcReady(d), 6000)) return true;
  history.pushState(null, "", "/channels/@me");
  dispatchEvent(new PopStateEvent("popstate", { state: null }));
  return !!(await until(() => dcReady(d), 8000));
}

async function dcList(d) {
  if (!(await dcOpen(d))) return [];
  return await sweep(dcRead, d);
}

async function waList(d) {
  if (!(await until(() => document.querySelector(d.ready), 10000))) return [];
  await until(() => document.querySelector(d.list), 5000);
  return await sweep(waRead, d);
}

function body(raw) {
  return String(raw)
    .split("\n")
    .filter((l) => !l.startsWith("! "))
    .join("\n");
}

async function tabsOf(site) {
  const tabs = await wisply(["tabs", "--all"]).nothrow();
  if (!tabs.ok) return [];
  return String(tabs.out)
    .split("\n")
    .filter((l) => l.includes(` - ${site.tab}`))
    .sort((a, b) => Number(b.includes(site.prefer)) - Number(a.includes(site.prefer)))
    .map((l) => (l.match(/^\s*(t\d+)\s/) || [])[1])
    .filter(Boolean);
}

const PAGE = [clean, until, scroller, sweep, dcRead, waRead, dcReady, dcOpen, dcList, waList].map(String).join("\n");
const CHUNK = 8000;
const SLICE = `const a = window.__wisplyChats || []; let n = AT, size = 0; const rows = [];
while (n < a.length && (!rows.length || size + a[n].length < ${CHUNK})) { size += a[n].length + 4; rows.push(a[n]); n++; }
if (n >= a.length) delete window.__wisplyChats;
return { next: n, done: n >= a.length, rows };`;

function answered(raw) {
  return `the page answered: ${clean(raw).slice(0, 80)}`;
}

async function pull() {
  const rows = [];
  for (let at = 0; ; ) {
    const raw = await wisply(["js", SLICE.replace("AT", String(at))]);
    let part;
    try {
      part = JSON.parse(body(raw));
    } catch {
      return { rows, why: answered(raw) };
    }
    rows.push(...part.rows);
    if (part.done || part.next <= at) return { rows };
    at = part.next;
  }
}

async function read(app, site) {
  const fn = app === "discord" ? "dcList" : "waList";
  const arg = JSON.stringify({ ...site, cap });
  const raw = await wisply(["js", `${PAGE}\nreturn ${fn}(${arg}).then((r) => { window.__wisplyChats = r; return r.length; });`]);
  const count = Number(body(raw).trim());
  if (Number.isNaN(count)) return { rows: [], why: answered(raw) };
  return count > 0 ? await pull() : { rows: [] };
}

async function chats(app) {
  const site = SITE[app];
  if (!site) return [`skip\t${app}\tunknown app`];
  const tabs = await tabsOf(site);
  if (!tabs.length) {
    await wisply(["open", site.url]);
    tabs.push("");
  }
  let why = "";
  for (let pass = 0; pass < 2; pass++) {
    for (const tab of tabs) {
      if (tab) await wisply(["tab", tab]);
      const got = await read(app, site);
      if (got.rows.length) return got.rows;
      why = got.why || why;
    }
    await sleep(1500);
  }
  return [`skip\t${app}\t${why || "no chat list (logged out or still loading)"}`];
}

const out = [];
for (const app of apps) out.push(...(await chats(app)));
if (!out.some((l) => !l.startsWith("skip\t"))) throw new Error(`no chats read: ${out.map((l) => l.replace(/\t/g, " ")).join("; ")}`);
const scan = String(args.scan || "").trim().toLowerCase();
if (scan) return await wisply(["contacts", "scan", `--rows=${out.join("\n")}`, ...(scan === "add" ? ["--add"] : [])]);
return out.join("\n");
