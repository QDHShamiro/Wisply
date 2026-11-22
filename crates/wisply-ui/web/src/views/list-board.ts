import { board, plain, type Entry, type ServerRow } from "../api";
import { ago, h } from "../dom";
import { T } from "../text";
import { prioPill, statusPill, type Ctx } from "./list";

const COLS = ["todo", "doing", "waiting", "done"];
const DONE_ON_BOARD = 20;

function card(c: Ctx, e: Entry): HTMLElement {
  const el = h(
    "div",
    { class: `kcard k-${e.kind}`, draggable: "true", tabindex: "0", onclick: () => c.open(e.id), onkeydown: (k: Event) => (k as KeyboardEvent).key === "Enter" && c.open(e.id) },
    h("div", { class: "task" }, e.task),
    h(
      "div",
      { class: "row meta" },
      h("span", { class: `kind-tag k-${e.kind}` }, T.list.kinds[e.kind] ?? e.kind),
      prioPill(e),
      e.count > 1 ? h("span", { class: "faint" }, T.list.times(e.count)) : null,
      h("span", { class: "grow" }),
      h("span", { class: "faint ellipsis" }, e.server || e.project),
    ),
    e.status === "doing" || e.status === "waiting" ? statusPill(e) : null,
  );
  el.addEventListener("dragstart", (ev) => {
    ev.dataTransfer?.setData("text/plain", String(e.id));
    if (ev.dataTransfer) ev.dataTransfer.effectAllowed = "move";
    el.classList.add("dragging");
  });
  el.addEventListener("dragend", () => el.classList.remove("dragging"));
  return el;
}

function column(c: Ctx, col: string, rows: Entry[]): HTMLElement {
  const shown = col === "done" ? rows.slice(0, DONE_ON_BOARD) : rows;
  const el = h(
    "section",
    { class: `kcol c-${col}` },
    h("h3", {}, T.list.statuses[col], h("span", { class: "n" }, rows.length)),
    h("div", { class: "kcards" }, shown.length ? shown.map((e) => card(c, e)) : h("div", { class: "kdrop" }, T.list.drop)),
  );
  el.addEventListener("dragover", (ev) => {
    ev.preventDefault();
    el.classList.add("over");
  });
  el.addEventListener("dragleave", (ev) => {
    if (!el.contains(ev.relatedTarget as Node)) el.classList.remove("over");
  });
  el.addEventListener("drop", (ev) => {
    ev.preventDefault();
    el.classList.remove("over");
    const id = Number(ev.dataTransfer?.getData("text/plain"));
    const from = c.data.rows.find((r) => r.id === id);
    if (from && from.status !== col) void c.set(id, "status", col);
  });
  return el;
}

export function boardView(c: Ctx, rows: Entry[]): HTMLElement {
  return h("div", { class: "kanban" }, COLS.map((col) => column(c, col, rows.filter((e) => e.status === col))));
}

function health(s: ServerRow): HTMLElement[] {
  const S = T.list.servers;
  const out: HTMLElement[] = [];
  if (!s.usable) out.push(h("span", { class: "pill err" }, S.noLogin));
  if (s.error) out.push(h("span", { class: "pill err", title: s.error }, s.error));
  else if (s.ok) out.push(h("span", { class: "pill on" }, S.ok(T.ago(ago(s.ok)))));
  else if (!s.checked) out.push(h("span", { class: "pill" }, S.notChecked));
  if (s.started) out.push(h("span", { class: "pill" }, S.restarted(T.ago(ago(s.started)))));
  return out;
}

function serverCard(c: Ctx, s: ServerRow): HTMLElement {
  const S = T.list.servers;
  const said = h("div", { class: "state" });
  const run = async (...args: string[]) => {
    said.className = "state";
    said.textContent = S.checking;
    try {
      const r = await board<{ said: string }>("server", s.name, ...args);
      said.className = "state ok";
      said.textContent = r.said;
      await c.refresh();
    } catch (e) {
      said.className = "state bad";
      said.textContent = (e as Error).message;
    }
  };
  const on = h("input", { type: "checkbox", checked: s.enabled, "aria-label": S.enabled });
  on.addEventListener("change", () => void run(on.checked ? "on" : "off"));
  const log = h("input", { type: "text", value: s.log, placeholder: S.log, class: "grow mono", "aria-label": S.log });
  log.addEventListener("change", () => void run("log", `--value=${log.value.trim()}`));
  return h(
    "div",
    { class: `panel server-card ${s.enabled ? "" : "off"}` },
    h("div", { class: "row" }, h("label", { class: "row grow" }, on, h("b", {}, s.name)), health(s), h("button", { onclick: () => void run("check"), disabled: !s.enabled || !s.usable }, S.check)),
    h("div", { class: "row" }, h("span", { class: "label" }, S.log), log),
    h("div", { class: "row faint small" }, S.counts(s.bugs, s.warns)),
    said,
  );
}

export function serversView(c: Ctx): HTMLElement {
  const S = T.list.servers;
  const s = c.data.servers;
  const toggle = (v: string) => void plain("config", "servers", v).then(c.refresh, c.fail);
  const banner = s.on ? null : h("div", { class: "banner" }, h("span", { class: "grow" }, S.off), h("button", { class: "primary", onclick: () => toggle("on") }, S.turnOn));
  if (s.problem) return h("div", {}, banner, h("div", { class: "empty" }, s.problem));
  const all = h("button", { onclick: () => void board("server", "all", "check").then(c.refresh, c.fail) }, S.checkAll);
  const top = h("div", { class: "row" }, h("span", { class: "faint grow" }, S.every), all, s.on ? h("button", { class: "ghost", onclick: () => toggle("off") }, S.turnOff) : null);
  return h("div", { class: "servers" }, banner, top, s.rows.length ? s.rows.map((r) => serverCard(c, r)) : h("div", { class: "empty" }, S.none));
}
