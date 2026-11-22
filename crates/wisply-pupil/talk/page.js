import { setup, esc, fill, num, size, bubble, welcome, chip } from "/mark.js";
import { connect, look, isLocked, forget, pairs, ask, kindOf, send, hear } from "/wire.js";

const SECOND = 1000;

const $ = (id) => document.getElementById(id);

let C = {};
let T = {};
let book = { chats: [], current: "" };
let draft = "";
let files = [];
let live = null;
let health = null;
let pending = null;
let packing = false;
let paint = 0;
let seen = { chat: "", count: 0 };
let rec = null;

const chatNow = () => book.chats.find((chat) => chat.id === book.current) || null;
const said = () => (chatNow() ? chatNow().messages : []);
const clock = (seconds) => (seconds ? new Date(seconds * SECOND).toLocaleTimeString(C.locale, C.clock) : "");
const kept = ({ id, name, kind, bytes, thumb }) => ({ id, name, kind, bytes, thumb });

function chatId() {
  if (book.current) return book.current;
  draft = draft || crypto.randomUUID();
  return draft;
}

function load() {
  try {
    const saved = JSON.parse(localStorage.getItem(C.storage.chats) || "null");
    if (saved && Array.isArray(saved.chats)) book = saved;
  } catch (_) {
    book = { chats: [], current: "" };
  }
  const kept = localStorage.getItem(C.storage.think);
  const mode = C.think.legacy[kept] || kept;
  $("think").value = C.think.modes.includes(mode) ? mode : C.think.default;
}

function lighten() {
  for (let at = book.chats.length - 1; at >= 0; at -= 1) {
    for (const m of book.chats[at].messages) {
      const pics = (m.files || []).filter((file) => file.thumb);
      if (pics.length) {
        pics.forEach((file) => delete file.thumb);
        m.lost = pics.length;
        return true;
      }
    }
  }
  return false;
}

function save() {
  book.chats.splice(C.storage.max_chats).forEach((chat) => forget(chat.id));
  const clean = (name, value) => (name === "live" || name === "note" ? undefined : value);
  for (;;) {
    try {
      localStorage.setItem(C.storage.chats, JSON.stringify(book, clean));
      return;
    } catch (_) {
      if (!lighten()) return;
    }
  }
}

function nearEnd() {
  const feed = $("feed");
  return feed.scrollHeight - feed.scrollTop - feed.clientHeight < C.stick_px;
}

function renderFeed(stick) {
  const feed = $("feed");
  const list = said();
  const html = list.length ? list.map((m, i) => bubble(m, i, i === list.length - 1, m.live ? C.open_live : (i === list.length - 1 ? C.open_last : false))).join("") : welcome();
  feed.innerHTML = '<div class="inner">' + html + "</div>";
  const nodes = feed.querySelectorAll(".msg");
  if (seen.chat === book.current) nodes.forEach((node, i) => node.classList.toggle("fresh", i >= seen.count));
  seen = { chat: book.current, count: nodes.length };
  phase($("live"));
  if (stick) feed.scrollTop = feed.scrollHeight;
}

function phase(node) {
  if (node) node.style.setProperty("--t", String(Date.now() % C.anim_wrap_ms));
}

function paintLive() {
  paint = 0;
  const node = $("live");
  if (!live || !node || live.chat !== book.current) return;
  const stick = nearEnd();
  const was = (name) => {
    const box = node.querySelector("details." + name);
    return box ? box.open : Boolean(C.open_live && C.open_live[name]);
  };
  const shell = document.createElement("div");
  shell.innerHTML = bubble(live.bot, said().length - 1, true, { thought: was("thought"), steps: was("steps") });
  const next = shell.firstElementChild;
  next.classList.toggle("fresh", node.classList.contains("fresh"));
  const inside = node.querySelector(".thought > div");
  const scrolled = inside ? inside.scrollHeight - inside.scrollTop - inside.clientHeight < C.stick_px : true;
  node.className = next.className;
  phase(node);
  node.innerHTML = next.innerHTML;
  const after = node.querySelector(".thought > div");
  if (after && scrolled) after.scrollTop = after.scrollHeight;
  if (stick) $("feed").scrollTop = $("feed").scrollHeight;
}

function soon() {
  if (!paint) paint = setTimeout(paintLive, C.render_ms);
}

function renderChats() {
  $("chats").innerHTML = book.chats.map((chat) => {
    const on = chat.id === book.current ? " on" : "";
    const id = esc(chat.id);
    return '<div class="chat' + on + '" data-id="' + id + '"><span>' + esc(chat.title || T.untitled) + '</span><b data-del="' + id
      + '" title="' + esc(T.remove) + '">' + esc(T.remove_mark) + "</b></div>";
  }).join("");
}

function renderThumbs() {
  $("thumbs").innerHTML = files.map((file, i) => chip(file, i)).join("");
}

function fault(text) {
  $("fault").textContent = text;
  $("fault").hidden = !text;
}

function testName(label) {
  const hit = C.test_names.find((pair) => label.startsWith(pair[0]));
  return hit ? hit[1] : T.test_other;
}

function holdMap() {
  const test = (health && health.test) || {};
  const label = (health && health.held) || "";
  return { name: testName(label), label, end: clock(test.end) };
}

function status() {
  if (isLocked()) return ["", T.off, $("locked").textContent];
  if (!health) return ["", T.off, T.off_long];
  if (health.game) return ["", T.paused, health.game];
  const test = health.test;
  if (test) {
    const label = test.label || "";
    return ["test", fill(T.testing, { name: testName(label), label, done: test.case, all: test.cases, end: clock(test.end) }), ""];
  }
  if (health.held) return ["test", fill(T.held, holdMap()), ""];
  return health.up ? ["up", T.ready, ""] : ["", T.asleep, ""];
}

function renderState() {
  const [dot, text, notice] = status();
  $("dot").className = dot;
  $("state").textContent = text;
  $("notice").textContent = notice;
  $("notice").hidden = !notice;
  $("model").textContent = C.model_names[C.model] || C.model || T.name;
}

function waiting() {
  return Boolean(live && !live.rush && !live.bot.count && health && health.held);
}

function renderHold() {
  const box = $("hold");
  const stuck = !pending && waiting();
  box.hidden = !pending && !stuck;
  if (box.hidden) return;
  const acts = stuck ? [["rush", T.hold_send]] : [["now", T.hold_send], ["later", T.hold_later], ["cancel", T.hold_cancel]];
  const buttons = acts
    .map(([act, label]) => '<button data-hold="' + act + '">' + esc(label) + "</button>").join("");
  box.innerHTML = esc(fill(T.hold, holdMap())) + "<div>" + buttons + "</div>";
}

function setBusy(on) {
  $("send").textContent = on ? T.stop : T.send;
}

function grow() {
  const input = $("input");
  input.style.height = "auto";
  input.style.height = Math.min(input.scrollHeight, C.input_max_px) + "px";
}

async function poll() {
  health = await look();
  renderState();
  if (pending && health && !health.held) release(false);
  else renderHold();
}

function finish(bot) {
  const stick = nearEnd();
  delete bot.live;
  delete bot.note;
  live = null;
  clearTimeout(paint);
  paint = 0;
  setBusy(false);
  save();
  renderFeed(stick);
  renderState();
  renderHold();
}

async function answer(chat, flags) {
  const list = chat.messages;
  const user = list[list.length - 1];
  const done = pairs(list.slice(0, -1));
  const recent = C.history > 0 ? done.slice(-C.history) : [];
  const bot = { role: "assistant", text: "", thought: "", live: true, began: Date.now(), steps: [{ key: "sent", text: "", at: 0 }] };
  const stop = new AbortController();
  const ids = (user.files || []).map((file) => file.id);
  const body = { chat: chat.id, text: user.text, files: ids, ...C.think.sends[$("think").value], ...flags, before: done.length - recent.length, history: recent };
  list.push(bot);
  const mine = { chat: chat.id, bot, stop, rush: false };
  live = mine;
  setBusy(true);
  save();
  renderChats();
  renderFeed(true);
  const ticker = setInterval(soon, C.clock_ms);
  await ask(body, stop.signal, bot, soon);
  clearInterval(ticker);
  finish(bot);
  if (mine.rush && list[list.length - 1] === bot) {
    list.pop();
    answer(chat, { now: true, redo: true });
  }
}

function release(now) {
  const { text, files: sent } = pending;
  const chat = chatNow() || { id: chatId(), title: "", messages: [] };
  pending = null;
  draft = "";
  book.chats = [chat, ...book.chats.filter((other) => other !== chat)];
  book.current = chat.id;
  chat.messages.push({ role: "user", text, files: sent.map(kept) });
  chat.title = chat.title || (text || sent[0].name).slice(0, C.storage.title_chars);
  renderHold();
  answer(chat, { now, redo: false });
}

function cancel() {
  $("input").value = pending.text;
  files = pending.files;
  pending = null;
  grow();
  renderThumbs();
  renderHold();
}

async function submit() {
  if (live || pending || packing) return;
  const count = files.length;
  packing = true;
  await Promise.all(files.map((file) => file.done));
  packing = false;
  const text = $("input").value.trim();
  if (live || files.length !== count || (!text && !count)) return;
  pending = { text, files };
  files = [];
  $("input").value = "";
  grow();
  renderThumbs();
  if (health && health.held) renderHold();
  else release(false);
}

function again() {
  const chat = chatNow();
  if (live || pending || !chat) return;
  const list = chat.messages;
  while (list.length && list[list.length - 1].role === "assistant") list.pop();
  if (list.length) answer(chat, { now: false, redo: true });
}

async function upload(entry, file) {
  const failed = await send(entry, file, chatId());
  if (failed) {
    files = files.filter((other) => other !== entry);
    fault(fill(T.bad_upload, { name: entry.name, why: failed }));
  }
  renderThumbs();
}

function addFiles(list) {
  fault("");
  for (const file of list) {
    const kind = kindOf(file.name);
    if (!kind) {
      fault(fill(T.bad_kind, { name: file.name }));
    } else if (file.size > C.upload_max_bytes) {
      fault(fill(T.too_big, { name: file.name, max: size(C.upload_max_bytes) }));
    } else {
      const entry = { id: "", name: file.name, kind, bytes: file.size, thumb: "", pending: true, stop: new AbortController() };
      files.push(entry);
      entry.done = upload(entry, file);
    }
  }
  renderThumbs();
}

function leave() {
  files.forEach((file) => file.stop.abort());
  if (draft) forget(draft);
  draft = "";
  files = [];
  pending = null;
  fault("");
  renderThumbs();
  renderHold();
}

function switchTo(id) {
  if (id === book.current) return;
  leave();
  book.current = id;
  save();
  renderChats();
  renderFeed(true);
}

function remove(id) {
  if (!window.confirm(T.remove_ask)) return;
  if (live && live.chat === id) live.stop.abort();
  book.chats = book.chats.filter((chat) => chat.id !== id);
  forget(id);
  if (book.current === id) {
    leave();
    book.current = "";
  }
  save();
  renderChats();
  renderFeed(true);
}

function copy(button, text) {
  navigator.clipboard.writeText(text).then(() => {
    button.textContent = T.copied;
    setTimeout(() => {
      button.textContent = T.copy;
    }, C.copied_ms);
  }).catch(() => {});
}

function onFeed(event) {
  const button = event.target.closest("[data-act]");
  if (!button) return;
  const act = button.dataset.act;
  if (act === "again") {
    again();
  } else if (act === "starter") {
    $("input").value = button.textContent;
    grow();
    $("input").focus();
  } else if (act === "code") {
    copy(button, button.closest(".code").querySelector("code").textContent);
  } else {
    copy(button, said()[Number(button.dataset.i)].text);
  }
}

function onChats(event) {
  const del = event.target.closest("[data-del]");
  const row = event.target.closest("[data-id]");
  if (del) remove(del.dataset.del);
  else if (row) switchTo(row.dataset.id);
}

function onHold(event) {
  const act = event.target.dataset.hold;
  if (act === "rush" && waiting()) {
    live.rush = true;
    renderHold();
    live.stop.abort();
    return;
  }
  if (!act || !pending) return;
  if (act === "cancel") cancel();
  else release(act === "now");
}

function onThumbs(event) {
  const off = event.target.dataset.off;
  if (off === undefined) return;
  const [gone] = files.splice(Number(off), 1);
  if (gone) gone.stop.abort();
  renderThumbs();
}

function onKey(event) {
  if (event.key !== "Enter" || event.shiftKey || event.isComposing) return;
  event.preventDefault();
  submit();
}

function onPaste(event) {
  const list = Array.from(event.clipboardData.files);
  if (!list.length) return;
  event.preventDefault();
  addFiles(list);
}

function insert(text) {
  const input = $("input");
  const value = String(text || "").trim();
  if (!value) return;
  const at = input.selectionStart ?? input.value.length;
  const end = input.selectionEnd ?? at;
  const before = input.value.slice(0, at);
  const gap = before && !/\s$/.test(before) ? " " : "";
  input.value = before + gap + value + input.value.slice(end);
  input.focus();
  grow();
}

function dropped(data) {
  for (const type of C.drop.types) {
    const lines = (data.getData(type) || "").split(/\r?\n/).map((line) => line.trim()).filter((line) => line && !line.startsWith(C.drop.comment));
    if (lines.length) return lines.join(" ");
  }
  return "";
}

function onDrag(event) {
  event.preventDefault();
  const over = event.type === "dragover";
  if (over || !event.relatedTarget) {
    $("composer").classList.toggle("drop", over);
    document.body.classList.toggle("dragging", over);
  }
  if (event.type !== "drop") return;
  const list = Array.from(event.dataTransfer.files);
  if (list.length) addFiles(list);
  else insert(dropped(event.dataTransfer));
}

function micText() {
  const mic = $("mic");
  mic.classList.toggle("on", Boolean(rec));
  mic.textContent = rec ? fill(T.dictate_stop, { v: num((Date.now() - rec.began) / SECOND, 0) }) : T.dictate;
}

async function transcribe(blob, now) {
  const mic = $("mic");
  mic.disabled = true;
  mic.textContent = T.dictate_busy;
  const got = await hear(blob, now);
  mic.disabled = false;
  micText();
  if (got.held && !now && confirm(fill(T.dictate_hold, { why: got.error }))) return transcribe(blob, true);
  if (got.error) return fault(got.error);
  if (!String(got.text || "").trim()) return fault(T.dictate_empty);
  fault("");
  insert(got.text);
  if (C.dictate.send) submit();
}

async function onMic() {
  if (rec) {
    rec.media.stop();
    return;
  }
  let input;
  try {
    input = await navigator.mediaDevices.getUserMedia({ audio: true });
  } catch (_) {
    fault(T.mic_denied);
    return;
  }
  const options = MediaRecorder.isTypeSupported(C.dictate.type) ? { mimeType: C.dictate.type } : {};
  const media = new MediaRecorder(input, options);
  const parts = [];
  media.ondataavailable = (event) => {
    if (event.data.size) parts.push(event.data);
  };
  media.onstop = () => {
    input.getTracks().forEach((track) => track.stop());
    clearInterval(rec.timer);
    rec = null;
    micText();
    transcribe(new Blob(parts, { type: media.mimeType }), false);
  };
  rec = { media, began: Date.now(), timer: setInterval(micText, C.dictate.tick_ms) };
  media.start();
  fault("");
  micText();
}

function onPick(event) {
  addFiles(Array.from(event.target.files));
  event.target.value = "";
}

function listen() {
  $("feed").addEventListener("click", onFeed);
  $("chats").addEventListener("click", onChats);
  $("hold").addEventListener("click", onHold);
  $("thumbs").addEventListener("click", onThumbs);
  $("new").addEventListener("click", () => switchTo(""));
  $("send").addEventListener("click", () => (live ? live.stop.abort() : submit()));
  $("attach").addEventListener("click", () => $("file").click());
  $("mic").addEventListener("click", onMic);
  $("file").addEventListener("change", onPick);
  $("think").addEventListener("change", (event) => localStorage.setItem(C.storage.think, event.target.value));
  $("input").addEventListener("input", grow);
  $("input").addEventListener("keydown", onKey);
  $("input").addEventListener("paste", onPaste);
  ["dragover", "dragleave", "drop"].forEach((name) => document.addEventListener(name, onDrag));
}

function dress() {
  const root = document.documentElement;
  const texts = { brand: T.name, new: T.new_chat, attach: T.attach, mic: T.dictate, dropText: T.drop_here, thinkText: T.think, hint: T.hint, send: T.send };
  root.lang = C.lang;
  for (const [name, value] of Object.entries(C.theme)) root.style.setProperty("--" + name, value);
  root.style.setProperty("--grow", C.input_max_px + "px");
  document.title = T.title;
  for (const [id, text] of Object.entries(texts)) $(id).textContent = text;
  $("input").placeholder = T.placeholder;
  $("think").replaceChildren(...C.think.modes.map((mode) => new Option(T.think_modes[mode], mode)));
  $("mic").hidden = !(C.dictate.on && navigator.mediaDevices && window.MediaRecorder);
  $("file").accept = Object.values(C.kinds).flat().map((ext) => "." + ext).join(",");
}

async function boot() {
  const given = await connect();
  if (!given) {
    $("locked").hidden = false;
    return;
  }
  C = given;
  T = C.text;
  setup(C);
  dress();
  load();
  listen();
  $("app").hidden = false;
  renderChats();
  renderFeed(true);
  renderThumbs();
  grow();
  poll();
  setInterval(poll, C.poll_ms);
  $("input").focus();
}

boot();
