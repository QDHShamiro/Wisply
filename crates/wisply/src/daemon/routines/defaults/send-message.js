const text = String(args.text || "");
const dry = args.dry === "1";
const many = [].concat(JSON.parse(String(args.list || "[]").trim() || "[]"));
const list = (many.length ? many : [{ to: args.to, app: args.app || "whatsapp", dm: args.dm || 0 }]).map((r) => ({
  name: String(r.name || "").trim(),
  to: String(r.to || "").trim(),
  app: String(r.app || "whatsapp").trim().toLowerCase(),
  dm: String(r.dm || "").trim(),
}));
if (!list.length || list.some((r) => !r.to) || !text.trim()) {
  throw new Error("send-message needs to=<exact chat name> and text=<message>");
}

const WEB = {
  whatsapp: {
    host: "web.whatsapp.com",
    url: "https://web.whatsapp.com/",
    sel: {
      ready: "#pane-side",
      list: "#pane-side span[title]",
      search: '#side input[type="text"], #side div[contenteditable="true"]',
      title: '#main header span[title], #main header span[dir="auto"]',
      box: '#main footer div[contenteditable="true"]',
      send: '#main footer [data-icon*="send"], #main footer button[aria-label="Send"]',
      out: "#main .message-out",
    },
  },
  telegram: {
    host: "web.telegram.org",
    url: "https://web.telegram.org/k/",
    sel: {
      ready: ".chatlist",
      list: ".chatlist-chat .peer-title",
      search: ".input-search input",
      title: ".chat-info .peer-title",
      box: '.input-message-input[contenteditable="true"]',
      send: ".btn-send",
      out: ".bubble.is-out",
    },
  },
};

function norm(v) {
  return String(v || "").replace(/\p{Cf}/gu, "").trim().toLowerCase();
}

async function until(test, ms) {
  for (let t = 0; t < ms; t += 100) {
    const v = test();
    if (v) return v;
    await new Promise((r) => setTimeout(r, 100));
  }
  return test();
}

function label(el) {
  return norm(el.getAttribute("title") || el.textContent);
}

function press(el) {
  const hit = el.closest('[role="listitem"], [role="row"], .chatlist-chat, a') || el;
  for (const type of ["mousedown", "mouseup", "click"]) {
    hit.dispatchEvent(new MouseEvent(type, { bubbles: true, cancelable: true, view: window }));
  }
}

function put(box, value) {
  box.focus();
  if (box.tagName === "INPUT") {
    Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, "value").set.call(box, value);
    box.dispatchEvent(new Event("input", { bubbles: true }));
    return;
  }
  document.execCommand("selectAll");
  document.execCommand("insertText", false, value);
}

async function openChat(d) {
  const want = norm(d.to);
  const open = () => [...document.querySelectorAll(d.sel.title)].some((el) => label(el) === want);
  const row = () => [...document.querySelectorAll(d.sel.list)].find((el) => label(el) === want);
  if (open()) return "";
  let hit = row();
  if (!hit) {
    const search = document.querySelector(d.sel.search);
    if (!search) return "no search box on the chat page";
    put(search, d.to);
    hit = await until(row, 3000);
  }
  if (!hit) return `no chat named exactly "${d.to}"`;
  press(hit);
  return (await until(open, 3000)) ? "" : `the chat "${d.to}" did not open`;
}

async function typeIn(box, value) {
  box.focus();
  const data = new DataTransfer();
  data.setData("text/plain", value);
  box.dispatchEvent(new ClipboardEvent("paste", { clipboardData: data, bubbles: true, cancelable: true }));
  if (!(await until(() => norm(box.textContent), 1000))) document.execCommand("insertText", false, value);
  return !!(await until(() => norm(box.textContent), 1000));
}

function extra(box, value) {
  const flat = (v) => norm(v).replace(/\s+/g, "");
  return flat(box.textContent).length > flat(value).length + 2;
}

function boxOf(d) {
  return document.querySelector(d.box || d.sel.box);
}

function boxFocus(d) {
  const box = boxOf(d);
  if (box) box.focus();
  return !!box;
}

async function boxEmpty(d) {
  return !!(await until(() => {
    const box = boxOf(d);
    return !!box && !norm(box.textContent);
  }, 1500));
}

async function send(d, box) {
  const before = document.querySelectorAll(d.sel.out).length;
  const button = await until(() => document.querySelector(d.sel.send), 1000);
  if (button) (button.closest("button") || button).click();
  else box.dispatchEvent(new KeyboardEvent("keydown", { key: "Enter", code: "Enter", keyCode: 13, which: 13, bubbles: true }));
  return !!(await until(() => document.querySelectorAll(d.sel.out).length > before, 6000));
}

async function main(d) {
  if (!(await until(() => document.querySelector(d.sel.ready), 8000))) {
    return { ok: false, why: "the chat page is not ready (logged out or still loading)" };
  }
  const missed = await openChat(d);
  if (missed) return { ok: false, why: missed };
  const box = await until(() => document.querySelector(d.sel.box), 2000);
  if (!box) return { ok: false, why: "no message box in the chat" };
  if (norm(box.textContent)) return { ok: false, why: `the chat with "${d.to}" already holds a draft, nothing typed` };
  if (!(await typeIn(box, d.text))) return { ok: false, why: "the text did not land in the message box" };
  if (extra(box, d.text)) return { ok: false, why: `the chat with "${d.to}" holds more than the message, nothing sent` };
  if (d.dry) return { ok: true };
  return { ok: true, seen: await send(d, box) };
}

const DC = {
  host: "discord.com",
  url: "https://discord.com/channels/@me",
  ready: 'nav[aria-label], [data-list-id="guildsnav"]',
  box: 'div[role="textbox"][data-slate-editor="true"]',
  msg: 'li[id^="chat-messages-"]',
};

async function dcReady(d) {
  return !!(await until(() => document.querySelector(d.ready), 15000));
}

function dcTitle(d) {
  return norm(document.title).endsWith(norm(`@${d.to}`));
}

async function dcHere(d) {
  const id = String(d.dm || "").split("/").pop();
  const here = () => !!id && location.pathname === `/channels/@me/${id}` && !!document.querySelector(d.box) && dcTitle(d);
  if (here()) return { here: true, path: location.pathname };
  if (!id) return { here: false, path: location.pathname };
  const link = document.querySelector(`a[href="/channels/@me/${id}"]`);
  if (link) link.click();
  else {
    history.pushState(null, "", `/channels/@me/${id}`);
    dispatchEvent(new PopStateEvent("popstate", { state: null }));
  }
  if (await until(here, 3000)) return { here: true, path: location.pathname };
  return { here: false, path: location.pathname };
}

async function dcEnter(d) {
  const box = document.querySelector(d.box);
  if (!box) return { sent: false };
  box.focus();
  const key = { key: "Enter", code: "Enter", keyCode: 13, which: 13, bubbles: true, cancelable: true };
  box.dispatchEvent(new KeyboardEvent("keydown", key));
  return { sent: !!(await until(() => !norm(box.innerText), 800)) };
}

function dcKey(el, key, extra) {
  const k = { key, code: extra.code || key, keyCode: extra.keyCode || 0, which: extra.keyCode || 0 };
  el.dispatchEvent(new KeyboardEvent("keydown", { ...k, ctrlKey: !!extra.ctrl, bubbles: true, cancelable: true }));
}

function dcField() {
  const el = document.activeElement;
  return el && el.tagName === "INPUT" && el.closest('[role="dialog"]') ? el : null;
}

async function dcSwitch() {
  dcKey(document.activeElement || document.body, "k", { code: "KeyK", keyCode: 75, ctrl: true });
  return { open: !!(await until(dcField, 600)) };
}

async function dcPick(d) {
  const field = dcField();
  if (!field) return { ok: false };
  dcKey(field, "Enter", { keyCode: 13 });
  if (!(await until(() => !field.isConnected, 800))) return { ok: false };
  await until(() => document.activeElement && document.activeElement.matches(d.box), 800);
  return { ok: true };
}

async function dcFind(d) {
  const field = await until(dcField, 2000);
  if (!field) return { ok: false, why: "the Discord quick switcher did not open" };
  put(field, `@${d.to}`);
  const dialog = field.closest('[role="dialog"]');
  const picked = () => {
    const sel = dialog.querySelector('[role="option"][aria-selected="true"]');
    return !!sel && sel.innerText.split("\n").some((l) => norm(l) === norm(d.to));
  };
  if (!(await until(picked, 2500))) return { ok: false, why: `Discord found nobody called "${d.to}"` };
  await new Promise((r) => setTimeout(r, 150));
  if (!picked()) return { ok: false, why: `Discord found nobody called "${d.to}"` };
  return { ok: true };
}

async function dcOpened(d) {
  const dm = () => location.pathname.startsWith("/channels/@me/") && !!document.querySelector(d.box);
  if (!(await until(() => dm() && dcTitle(d), 4000))) {
    return { ok: false, why: `the DM with "${d.to}" did not open, nothing typed` };
  }
  return { ok: true, path: location.pathname };
}

async function dcWrite(d) {
  let box = null;
  for (let i = 0; i < 2 && !box; i++) {
    const el = await until(() => document.querySelector(d.box), 2000);
    if (!el) return { ok: false, why: "no message box in the DM" };
    if (!dcTitle(d)) return { ok: false, why: `left the DM with "${d.to}", nothing typed` };
    if (!i && norm(el.textContent)) return { ok: false, why: `the DM with "${d.to}" already holds a draft, nothing typed` };
    if ((await typeIn(el, d.text)) && el.isConnected) box = el;
  }
  if (!box) return { ok: false, why: "the text did not land in the message box" };
  if (extra(box, d.text)) return { ok: false, why: `the DM with "${d.to}" holds more than the message, nothing sent` };
  if (d.dry) return { ok: true };
  return { ok: true };
}

async function dcSeen(d) {
  const want = norm(d.text).split("\n")[0].slice(0, 40);
  return !!(await until(() => {
    const all = document.querySelectorAll(d.msg);
    const last = all[all.length - 1];
    return !!last && norm(last.innerText).includes(want);
  }, 6000));
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

async function wipe(call) {
  if (!(await call("boxFocus", {}))) return false;
  await wisply(["press", "ctrl+a", "Backspace"]).nothrow();
  return !!(await call("boxEmpty", {}));
}

async function discord(to, dm) {
  const code = [norm, until, put, typeIn, extra, boxOf, boxFocus, boxEmpty, dcReady, dcTitle, dcHere, dcKey, dcField, dcSwitch, dcPick, dcFind, dcOpened, dcWrite, dcEnter, dcSeen].map(String).join("\n");
  const base = { to, text, dry, dm, ...DC };
  const page = async (fn, extra) => {
    const raw = await wisply(["js", `${code}\nreturn ${fn}(${JSON.stringify({ ...base, ...extra })});`]);
    try {
      return JSON.parse(body(raw));
    } catch {
      return { ok: false, why: `the page answered: ${String(raw).slice(0, 200)}` };
    }
  };
  const tab = await tabOf(DC.host);
  if (tab) await wisply(["tab", tab]);
  else await wisply(["open", dm.startsWith(`${DC.url}/`) ? dm : DC.url]);
  if (!(await page("dcReady", {}))) throw new Error("Discord is not ready (logged out or still loading)");
  const at = await page("dcHere", {});
  let path = at.path;
  if (!at.here) {
    if (!(await page("dcSwitch", {})).open) await wisply(["press", "ctrl+k"]);
    const found = await page("dcFind", {});
    if (!found.ok) {
      await wisply(["press", "Escape"]).nothrow();
      throw new Error(found.why);
    }
    if (!(await page("dcPick", {})).ok) await wisply(["press", "Enter"]);
    const opened = await page("dcOpened", {});
    if (!opened.ok) throw new Error(opened.why);
    path = opened.path;
  }
  const wrote = await page("dcWrite", {});
  if (!wrote.ok) throw new Error(wrote.why);
  const link = `https://discord.com${path}`;
  if (dry) return `dry run: typed ${text.length} chars into "${to}" on discord, ${(await wipe(page)) ? "cleared" : "NOT cleared"}, nothing sent (dm ${link})`;
  if (!(await page("dcEnter", {})).sent) await wisply(["press", "Enter"]);
  const seen = await page("dcSeen", {});
  return `sent to "${to}" on discord${seen ? "" : " (not visible after 6 s)"} (dm ${link})`;
}

const PAGE = [norm, until, label, press, put, openChat, typeIn, extra, boxOf, boxFocus, boxEmpty, send, main].map(String).join("\n");
async function one({ to, app, dm }) {
  if (app === "discord") return await discord(to, dm);
  const site = WEB[app];
  if (!site) return await wisply(["reply", to, text, "--exact", ...(dry ? ["--dry"] : [])]);
  const tab = await tabOf(site.host);
  if (tab) await wisply(["tab", tab]);
  else await wisply(["open", site.url]);
  const call = async (fn, extra) => {
    const raw = await wisply(["js", `${PAGE}\nreturn ${fn}(${JSON.stringify({ to, text, dry, sel: site.sel, ...extra })});`]);
    try {
      return JSON.parse(body(raw));
    } catch {
      return { ok: false, why: `the page answered: ${String(raw).slice(0, 200)}` };
    }
  };
  const out = await call("main", {});
  if (!out.ok) throw new Error(out.why);
  if (dry) return `dry run: typed ${text.length} chars into "${to}" on ${app}, ${(await wipe(call)) ? "cleared" : "NOT cleared"}, nothing sent`;
  return `sent to "${to}" on ${app}${out.seen ? "" : " (not visible after 6 s)"}`;
}

if (!many.length) return await one(list[0]);
const done = [];
for (const r of list) {
  let out;
  try {
    out = String(await one(r));
  } catch (e) {
    const why = String((e && e.message) || e);
    throw new Error(done.length ? `${why} (already sent to ${done.join(", ")})` : why);
  }
  const link = (out.match(/https:\/\/discord\.com\/channels\/@me\/\d+/) || [])[0];
  const name = r.name || r.to;
  if (!dry) await wisply(["contact", "sent", name, `app=${r.app}`, `handle=${r.to}`, ...(link ? [`dm=${link}`] : [])]).nothrow();
  const note = out.includes("not visible") ? ", not visible after 6 s" : out.includes("NOT cleared") ? ", NOT cleared" : "";
  done.push(`${name} (${r.app}${note})`);
}
return `${dry ? "dry run, typed and cleared, nothing sent:" : "sent to"} ${done.join(", ")}`;
