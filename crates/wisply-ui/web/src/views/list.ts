import { board, boardWith, type Entry, type ListData } from "../api";
import { ago, debounce, enc, h, mount, pick } from "../dom";
import { T } from "../text";
import { entryPanel } from "./entry";
import { boardView, serversView } from "./list-board";

export const TABS = ["open", "board", "warnings", "servers", "done", "suggestions"] as const;
export type ListTab = (typeof TABS)[number];
const POLL_MS = 10000;
const DONE_DAYS = 30;
const OPEN = ["todo", "doing", "waiting"];
const PRIO_ORDER: Record<string, number> = { high: 0, mid: 1, low: 2 };
const STATUS_ORDER: Record<string, number> = { doing: 0, waiting: 1, todo: 2, suggested: 3, done: 4 };
const filters = { q: "", kind: "", prio: "", project: "", server: "" };

export type Ctx = {
  data: ListData;
  open: (id: number) => void;
  refresh: () => Promise<void>;
  set: (id: number, field: string, value: string) => Promise<void>;
  fail: (e: unknown) => void;
};

export function byWeight(a: Entry, b: Entry): number {
  return (STATUS_ORDER[a.status] ?? 9) - (STATUS_ORDER[b.status] ?? 9) || (PRIO_ORDER[a.priority] ?? 9) - (PRIO_ORDER[b.priority] ?? 9) || b.updated - a.updated;
}

function isWarn(e: Entry): boolean {
  return e.level === "warn";
}

function inTab(e: Entry, tab: ListTab): boolean {
  if (tab === "open") return OPEN.includes(e.status) && !isWarn(e);
  if (tab === "board") return [...OPEN, "done"].includes(e.status) && !isWarn(e);
  if (tab === "warnings") return OPEN.includes(e.status) && isWarn(e);
  if (tab === "done") return e.status === "done";
  if (tab === "suggestions") return e.status === "suggested";
  return false;
}

function matches(e: Entry): boolean {
  const q = filters.q.trim().toLowerCase();
  if (q && !`${e.task} ${e.project} ${e.server} ${e.last_note} #${e.id}`.toLowerCase().includes(q)) return false;
  if (filters.kind && e.kind !== filters.kind) return false;
  if (filters.prio && e.priority !== filters.prio) return false;
  if (filters.project && e.project !== filters.project) return false;
  return !filters.server || e.server === filters.server;
}

export function rowsOf(data: ListData, tab: ListTab): Entry[] {
  const recent = (e: Entry) => tab !== "done" || data.now - e.updated < DONE_DAYS * 86400;
  return data.rows.filter((e) => inTab(e, tab) && recent(e) && matches(e)).sort(tab === "done" ? (a, b) => b.updated - a.updated : byWeight);
}

export function statusPill(e: Entry): HTMLElement {
  const label = T.list.statuses[e.status] ?? e.status;
  const extra = e.status === "doing" && e.who ? ` · ${e.who}` : e.status === "waiting" && e.wait ? ` · ${T.list.waitShort[e.wait] ?? e.wait}` : "";
  return h("span", { class: `pill st-${e.status}` }, label + extra);
}

export function prioPill(e: Entry): HTMLElement {
  return h("span", { class: `prio p-${e.priority}`, title: T.entry.priority }, T.list.prios[e.priority] ?? e.priority);
}

function where(e: Entry): string {
  return [e.project, e.server].filter(Boolean).join(" · ");
}

function row(c: Ctx, e: Entry): HTMLElement {
  const done = e.status === "done";
  const box = h("span", {
    class: "box",
    title: done ? T.todo.open : T.todo.done,
    onclick: (ev: Event) => {
      ev.stopPropagation();
      void c.set(e.id, "status", done ? "todo" : "done");
    },
  });
  const sub = [e.count > 1 ? T.list.times(e.count) : "", e.last_note].filter(Boolean).join(" · ");
  return h(
    "div",
    { class: `entry-row todo ${e.status} k-${e.kind}`, tabindex: "0", onclick: () => c.open(e.id), onkeydown: (k: Event) => (k as KeyboardEvent).key === "Enter" && c.open(e.id) },
    box,
    h("div", { class: "grow" }, h("div", { class: "task ellipsis" }, e.task), sub ? h("div", { class: "faint small ellipsis" }, sub) : null),
    h("span", { class: `kind-tag k-${e.kind}` }, T.list.kinds[e.kind] ?? e.kind),
    prioPill(e),
    statusPill(e),
    h("span", { class: "where ellipsis" }, where(e)),
    h("span", { class: "when" }, T.ago(ago(e.updated))),
  );
}

function table(c: Ctx, rows: Entry[], empty: string, extra?: (e: Entry) => HTMLElement | null): HTMLElement {
  if (!rows.length) return h("div", { class: "empty" }, empty);
  return h(
    "div",
    { class: "entries" },
    h("div", { class: "entry-row thead" }, h("span", { class: "box-gap" }),h("span", { class: "grow" }, T.list.cols.task), h("span", {}, T.list.cols.kind), h("span", {}, T.list.cols.priority), h("span", {}, T.list.cols.status), h("span", {}, T.list.cols.where), h("span", {}, T.list.cols.updated)),
    rows.map((e) => {
      const r = row(c, e);
      const more = extra?.(e);
      if (more) r.append(more);
      return r;
    }),
  );
}

function warnings(c: Ctx): HTMLElement {
  const ignore = (e: Entry) =>
    h("span", { class: "acts" }, h("button", { onclick: (ev: Event) => { ev.stopPropagation(); void board("mute", String(e.id)).then(c.refresh, c.fail); } }, T.list.ignore));
  const mutes = c.data.mutes.length
    ? h(
        "section",
        {},
        h("h2", {}, T.list.ignored, h("span", { class: "n" }, c.data.mutes.length)),
        h("div", { class: "list" }, c.data.mutes.map((m) => h("div", { class: "item" }, h("span", { class: "kind" }, m.server), h("span", { class: "grow ellipsis" }, m.title), h("span", { class: "when" }, T.ago(ago(m.at))), h("button", { onclick: () => void board("unmute", m.fingerprint).then(c.refresh, c.fail) }, T.list.unmute)))),
      )
    : null;
  return h("div", {}, table(c, rowsOf(c.data, "warnings"), T.list.noWarnings, ignore), mutes);
}

function suggestions(c: Ctx): HTMLElement {
  const acts = (e: Entry) =>
    h(
      "span",
      { class: "acts" },
      h("button", { class: "primary", onclick: (ev: Event) => { ev.stopPropagation(); void c.set(e.id, "status", "todo"); } }, T.list.accept),
      h("button", { onclick: (ev: Event) => { ev.stopPropagation(); void c.set(e.id, "status", "dropped"); } }, T.list.dismiss),
    );
  return table(c, rowsOf(c.data, "suggestions"), T.list.noSuggestions, acts);
}

function work(data: ListData): HTMLElement {
  const runs = data.sessions.slice(0, 12);
  return h(
    "div",
    { class: "work" },
    h("span", { class: "label" }, T.list.working),
    runs.length
      ? runs.map((r) => h("span", { class: `run ${r.status}`, title: [r.summary, r.next ? `${T.project.next}: ${r.next}` : ""].filter(Boolean).join("\n") }, h("span", { class: "dot" }), r.project ? h("b", {}, r.project) : null, h("span", { class: "ellipsis" }, r.title || r.id.slice(0, 8)), h("span", { class: "faint" }, T.ago(ago(r.last)))))
      : h("span", { class: "faint" }, T.list.noWork),
  );
}

function filterBar(data: ListData, redraw: () => void): HTMLElement {
  const opt = (all: string, values: string[], labels?: Record<string, string>) => [{ value: "", label: all }, ...values.map((v) => ({ value: v, label: labels?.[v] ?? v }))];
  const projects = [...new Set([...data.projects, ...data.rows.map((e) => e.project).filter(Boolean)])].sort();
  const servers = data.servers.rows.map((s) => s.name);
  const q = h("input", { type: "search", placeholder: T.list.find, value: filters.q });
  q.addEventListener("input", debounce(() => { filters.q = q.value; redraw(); }, 150));
  const setter = (k: keyof typeof filters) => (v: string) => { filters[k] = v; redraw(); };
  return h(
    "div",
    { class: "row filters" },
    q,
    pick(opt(T.list.allKinds, data.kinds, T.list.kinds), filters.kind, setter("kind")),
    pick(opt(T.list.allPrios, data.prios, T.list.prios), filters.prio, setter("prio")),
    projects.length ? pick(opt(T.list.allProjects, projects), filters.project, setter("project")) : null,
    servers.length ? pick(opt(T.list.allServers, servers), filters.server, setter("server")) : null,
  );
}

function addForm(c: Ctx, scope: string, done: () => void): HTMLElement {
  let kind = "task";
  let prio = "mid";
  const task = h("input", { type: "text", placeholder: T.list.newTask, class: "grow" });
  const project = h("input", { type: "text", placeholder: T.list.newProject, value: scope || filters.project, list: "list-projects" });
  const body = h("textarea", { rows: 3, placeholder: T.list.newBody });
  const save = async () => {
    const t = task.value.trim();
    if (!t) return task.focus();
    try {
      await boardWith(body.value, "entry-add", `--task=${t}`, `--kind=${kind}`, `--priority=${prio}`, `--project=${project.value.trim()}`);
      done();
      await c.refresh();
    } catch (e) {
      c.fail(e);
    }
  };
  task.addEventListener("keydown", (k) => k.key === "Enter" && void save());
  const form = h(
    "div",
    { class: "panel add-form" },
    h("div", { class: "row" }, task, pick(c.data.kinds.map((v) => ({ value: v, label: T.list.kinds[v] ?? v })), kind, (v) => (kind = v)), pick(c.data.prios.map((v) => ({ value: v, label: T.list.prios[v] ?? v })), prio, (v) => (prio = v))),
    h("div", { class: "row" }, project, h("datalist", { id: "list-projects" }, c.data.projects.map((p) => h("option", { value: p })))),
    body,
    h("div", { class: "row end" }, h("button", { onclick: done }, T.list.cancel), h("button", { class: "primary", onclick: () => void save() }, T.list.save)),
  );
  setTimeout(() => task.focus(), 0);
  return form;
}

type Parts = { drawer: HTMLElement; say: HTMLElement; work: HTMLElement; extra: HTMLElement; tabs: HTMLElement; filters: HTMLElement; body: HTMLElement };

const BUSY = ".add-form:focus-within, .drawer:focus-within, .filters:focus-within, .dragging, .server-card:focus-within";

function bodyOf(c: Ctx, tab: ListTab): HTMLElement {
  const views: Record<ListTab, () => HTMLElement> = {
    open: () => table(c, rowsOf(c.data, "open"), T.list.none),
    board: () => boardView(c, rowsOf(c.data, "board")),
    warnings: () => warnings(c),
    servers: () => serversView(c),
    done: () => table(c, rowsOf(c.data, "done"), T.list.noDone),
    suggestions: () => suggestions(c),
  };
  return views[tab]();
}

function tabLinks(data: ListData, tab: ListTab, scope: string): HTMLElement[] {
  const href = (t: ListTab) => (scope ? `#/p/${enc(scope)}/todos/${t}` : `#/l/${t}`);
  const count = (t: ListTab) => (t === "servers" ? data.servers.rows.length : t === "board" ? 0 : rowsOf(data, t).length);
  return TABS.filter((t) => !scope || t !== "servers").map((t) => h("a", { href: href(t), class: t === tab ? "on" : "" }, T.list.tabs[t], count(t) ? h("span", { class: "n" }, count(t)) : null));
}

function context(data: ListData, args: string[], p: Parts, redraw: () => void): Ctx {
  let seen = JSON.stringify(data);
  let opened = 0;
  const c: Ctx = {
    data,
    open: (id) => {
      opened = id;
      void entryPanel(p.drawer, id, () => void c.refresh());
    },
    refresh: async () => {
      const next = await board<ListData>(...args).catch(() => null);
      const text = next ? JSON.stringify(next) : seen;
      if (!next || text === seen) return;
      seen = text;
      c.data = next;
      redraw();
    },
    set: async (id, field, value) => {
      try {
        await board("entry-set", String(id), field, `--value=${value}`);
        await c.refresh();
        if (opened === id && p.drawer.classList.contains("open")) void entryPanel(p.drawer, id, () => void c.refresh());
      } catch (e) {
        c.fail(e);
      }
    },
    fail: (e) => {
      p.say.className = "state bad";
      p.say.textContent = (e as Error).message;
    },
  };
  return c;
}

function watch(c: Ctx, drawer: HTMLElement): () => void {
  const timer = window.setInterval(() => {
    if (!document.querySelector(BUSY)) void c.refresh();
  }, POLL_MS);
  const keys = (k: KeyboardEvent) => {
    if (k.key !== "Escape" || !drawer.classList.contains("open")) return;
    drawer.classList.remove("open");
    mount(drawer);
  };
  window.addEventListener("keydown", keys);
  return () => {
    clearInterval(timer);
    window.removeEventListener("keydown", keys);
  };
}

function header(c: Ctx, p: Parts, scope: string): HTMLElement {
  const add = h("button", { class: "primary", onclick: () => mount(p.extra, addForm(c, scope, () => mount(p.extra))) }, T.list.add);
  if (scope) return h("div", { class: "row end" }, add);
  return h("div", { class: "row head" }, h("div", { class: "grow" }, h("h1", {}, T.list.title), h("div", { class: "sub" }, T.list.sub)), add);
}

export async function list(view: HTMLElement, tab: ListTab, scope = ""): Promise<() => void> {
  const args = ["list", ...(scope ? [`--project=${scope}`] : [])];
  const p: Parts = { drawer: h("aside", { class: "drawer" }), say: h("div", { class: "state" }), work: h("div", {}), extra: h("div", {}), tabs: h("div", { class: "tabs" }), filters: h("div", {}), body: h("div", {}) };
  const drawBody = () => {
    mount(p.tabs, ...tabLinks(c.data, tab, scope));
    mount(p.body, bodyOf(c, tab));
  };
  const drawAll = () => {
    if (!scope) mount(p.work, work(c.data));
    mount(p.filters, tab === "servers" ? null : filterBar(c.data, drawBody));
    drawBody();
  };
  const c = context(await board<ListData>(...args), args, p, drawAll);
  drawAll();
  mount(view, h("div", { class: scope ? "listing" : "page listing" }, header(c, p, scope), p.work, p.extra, p.tabs, p.say, p.filters, p.body, p.drawer));
  return watch(c, p.drawer);
}
