const want = String(args.app || "all").trim().toLowerCase();
const apps = want === "all" || !want ? ["discord", "whatsapp"] : want.split(",").map((a) => a.trim());
const cap = Number(args.max) > 0 ? Number(args.max) : 400;

const SITE = {
  discord: {
    host: "discord.com",
    url: "https://discord.com/channels/@me",
    list: '[class*="privateChannels"] a[href^="/channels/@me/"], nav a[href^="/channels/@me/"]',
    steps: 40,
  },
  whatsapp: {
    host: "web.whatsapp.com",
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

async function tabOf(host) {
  const tabs = await wisply(["tabs", "--all"]).nothrow();
  const row = tabs.ok ? String(tabs.out).split("\n").find((l) => l.includes(` - ${host}`)) : undefined;
  return row ? (row.match(/^\s*(t\d+)\s/) || [])[1] : undefined;
}

const PAGE = [clean, until, scroller, sweep, dcRead, waRead, dcReady, dcOpen, dcList, waList].map(String).join("\n");

async function chats(app) {
  const site = SITE[app];
  if (!site) return [`skip\t${app}\tunknown app`];
  const tab = await tabOf(site.host);
  if (tab) await wisply(["tab", tab]);
  else await wisply(["open", site.url]);
  const fn = app === "discord" ? "dcList" : "waList";
  const raw = await wisply(["js", `${PAGE}\nreturn ${fn}(${JSON.stringify({ ...site, cap })});`]);
  let rows;
  try {
    rows = JSON.parse(body(raw));
  } catch {
    return [`skip\t${app}\tthe page answered: ${clean(raw).slice(0, 80)}`];
  }
  if (!rows.length) return [`skip\t${app}\tno chat list (logged out or still loading)`];
  return rows;
}

const out = [];
for (const app of apps) out.push(...(await chats(app)));
if (!out.some((l) => !l.startsWith("skip\t"))) throw new Error(`no chats read: ${out.map((l) => l.replace(/\t/g, " ")).join("; ")}`);
return out.join("\n");
