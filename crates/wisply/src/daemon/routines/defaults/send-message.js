const to = String(args.to || "").trim();
const text = String(args.text || "");
const app = String(args.app || "whatsapp").trim().toLowerCase();
const dry = args.dry === "1";
if (!to || !text.trim()) throw new Error("send-message needs to=<exact chat name> and text=<message>");

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

async function clear(box) {
  box.focus();
  document.execCommand("selectAll");
  document.execCommand("delete");
  return !!(await until(() => !norm(box.textContent), 1000));
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
  if (!(await typeIn(box, d.text))) return { ok: false, why: "the text did not land in the message box" };
  if (d.dry) return { ok: true, cleared: await clear(box) };
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

async function dcHere(d) {
  const id = String(d.dm || "").split("/").pop();
  const here = () => !!id && location.pathname === `/channels/@me/${id}` && !!document.querySelector(d.box);
  if (here()) return { here: true, path: location.pathname };
  const link = id ? document.querySelector(`a[href="/channels/@me/${id}"]`) : null;
  if (link) {
    link.click();
    if (await until(here, 3000)) return { here: true, path: location.pathname };
  }
  return { here: false, path: location.pathname };
}

async function dcFind(d) {
  const field = await until(() => {
    const el = document.activeElement;
    return el && el.tagName === "INPUT" && el.closest('[role="dialog"]') ? el : null;
  }, 2000);
  if (!field) return { ok: false, why: "the Discord quick switcher did not open" };
  put(field, `@${d.to}`);
  const dialog = field.closest('[role="dialog"]');
  const hit = await until(() => dialog.querySelector('[role="option"], [aria-selected="true"]'), 2500);
  if (!hit) return { ok: false, why: `Discord found nobody called "${d.to}"` };
  await new Promise((r) => setTimeout(r, 150));
  return { ok: true };
}

async function dcOpened(d) {
  const dm = () => location.pathname.startsWith("/channels/@me/") && !!document.querySelector(d.box);
  const moved = () => dm() && (location.pathname !== d.before || norm(document.title).includes(norm(d.to)));
  if (!(await until(moved, 4000))) return { ok: false, why: `the DM with "${d.to}" did not open` };
  return { ok: true, path: location.pathname };
}

async function dcWrite(d) {
  const box = await until(() => document.querySelector(d.box), 2000);
  if (!box) return { ok: false, why: "no message box in the DM" };
  if (!(await typeIn(box, d.text))) return { ok: false, why: "the text did not land in the message box" };
  if (d.dry) return { ok: true, cleared: await clear(box) };
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

async function tabOf(host) {
  const tabs = await wisply(["tabs", "--all"]).nothrow();
  const row = tabs.ok ? String(tabs.out).split("\n").find((l) => l.includes(` - ${host}`)) : undefined;
  return row ? (row.match(/^\s*(t\d+)\s/) || [])[1] : undefined;
}

async function discord() {
  const dm = String(args.dm || "").trim();
  const code = [norm, until, put, typeIn, clear, dcReady, dcHere, dcFind, dcOpened, dcWrite, dcSeen].map(String).join("\n");
  const base = { to, text, dry, dm, ...DC };
  const page = async (fn, extra) => {
    const raw = await wisply(["js", `${code}\nreturn ${fn}(${JSON.stringify({ ...base, ...extra })});`]);
    try {
      return JSON.parse(raw);
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
    await wisply(["press", "ctrl+k"]);
    const found = await page("dcFind", {});
    if (!found.ok) {
      await wisply(["press", "Escape"]).nothrow();
      throw new Error(found.why);
    }
    await wisply(["press", "Enter"]);
    const opened = await page("dcOpened", { before: at.path });
    if (!opened.ok) throw new Error(opened.why);
    path = opened.path;
  }
  const wrote = await page("dcWrite", {});
  if (!wrote.ok) throw new Error(wrote.why);
  const link = `https://discord.com${path}`;
  if (dry) return `dry run: typed ${text.length} chars into "${to}" on discord, ${wrote.cleared ? "cleared" : "NOT cleared"}, nothing sent (dm ${link})`;
  await wisply(["press", "Enter"]);
  const seen = await page("dcSeen", {});
  return `sent to "${to}" on discord${seen ? "" : " (not visible after 6 s)"} (dm ${link})`;
}

const PAGE = [norm, until, label, press, put, openChat, typeIn, clear, send, main].map(String).join("\n");
if (app === "discord") return await discord();
const site = WEB[app];
if (!site) return await wisply(["reply", to, text, "--exact", ...(dry ? ["--dry"] : [])]);
const tab = await tabOf(site.host);
if (tab) await wisply(["tab", tab]);
else await wisply(["open", site.url]);
const raw = await wisply(["js", `${PAGE}\nreturn main(${JSON.stringify({ to, text, dry, sel: site.sel })});`]);
let out;
try {
  out = JSON.parse(raw);
} catch {
  out = { ok: false, why: `the page answered: ${String(raw).slice(0, 200)}` };
}
if (!out.ok) throw new Error(out.why);
if (dry) return `dry run: typed ${text.length} chars into "${to}" on ${app}, ${out.cleared ? "cleared" : "NOT cleared"}, nothing sent`;
return `sent to "${to}" on ${app}${out.seen ? "" : " (not visible after 6 s)"}`;
