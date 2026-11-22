import { board, type ListRun, type MemoryData, type MemoryRow } from "../api";
import { ago, debounce, h, mount, pick } from "../dom";
import { T } from "../text";

const SHELF_ORDER = ["", "user", "session", "vault", "claude", "sessions"];
const state = { source: "", project: "", words: "", offset: 0 };

function shelfChips(data: MemoryData, redraw: () => void): HTMLElement {
  const counts = new Map(data.shelves.map((s) => [s.source, s.count]));
  const all = data.shelves.reduce((n, s) => n + s.count, 0);
  const known = [...SHELF_ORDER, ...data.shelves.map((s) => s.source).filter((s) => !SHELF_ORDER.includes(s))];
  const count = (s: string) => (s === "" ? all : s === "sessions" ? data.sessions_total : (counts.get(s) ?? 0));
  return h(
    "div",
    { class: "chips shelves" },
    known
      .filter((s) => s === "" || count(s) > 0)
      .map((s) =>
        h(
          "button",
          {
            class: `chip ${state.source === s ? "on" : ""}`,
            onclick: () => {
              state.source = s;
              state.offset = 0;
              redraw();
            },
          },
          T.memory.shelves[s] ?? s,
          h("span", { class: "n" }, count(s).toLocaleString("en-US")),
        ),
      ),
  );
}

function noteRow(n: MemoryRow, redraw: () => void, say: (e: unknown) => void): HTMLElement {
  let armed = false;
  const forget = h("button", {
    class: "ghost",
    onclick: async (e: Event) => {
      e.preventDefault();
      e.stopPropagation();
      if (!armed) {
        armed = true;
        forget.textContent = T.memory.sure;
        forget.classList.add("danger");
        return;
      }
      try {
        await board("forget", String(n.id));
        redraw();
      } catch (err) {
        say(err);
      }
    },
  }, T.memory.forget);
  return h(
    "a",
    { class: "item mem", href: `#/n/${n.id}` },
    h("span", { class: "kind" }, T.memory.shelves[n.source] ?? (n.source || n.kind)),
    h("div", { class: "grow" }, h("div", { class: "ellipsis" }, n.title), n.snippet ? h("div", { class: "faint small ellipsis" }, n.snippet) : null),
    n.project ? h("span", { class: "tag" }, n.project) : null,
    n.hits ? h("span", { class: "faint small" }, T.memory.hits(n.hits)) : null,
    h("span", { class: "when" }, T.ago(ago(n.updated))),
    h("span", { class: "acts" }, forget),
  );
}

function runRow(r: ListRun): HTMLElement {
  return h(
    "div",
    { class: "item", style: "flex-direction:column;align-items:stretch;gap:2px" },
    h("div", { class: "row" }, r.project ? h("span", { class: "tag" }, r.project) : null, h("span", { class: "grow ellipsis" }, r.title || r.id.slice(0, 8)), h("span", { class: "when" }, T.ago(ago(r.last)))),
    r.summary ? h("div", { class: "body" }, r.summary) : null,
    r.next ? h("div", { class: "body faint" }, `${T.memory.nextLine}: ${r.next}`) : null,
  );
}

function pager(data: MemoryData, redraw: () => void): HTMLElement | null {
  if (data.total <= data.page) return null;
  const to = Math.min(data.total, data.offset + data.page);
  const move = (by: number) => {
    state.offset = Math.max(0, state.offset + by);
    redraw();
  };
  return h(
    "div",
    { class: "row pager" },
    h("button", { onclick: () => move(-data.page), disabled: data.offset === 0 }, T.memory.prev),
    h("span", { class: "faint grow center" }, T.memory.range(data.offset + 1, to, data.total)),
    h("button", { onclick: () => move(data.page), disabled: to >= data.total }, T.memory.next),
  );
}

function rowsOf(data: MemoryData, redraw: () => void, say: (e: unknown) => void): HTMLElement {
  const items = state.source === "sessions" ? (data.sessions ?? []).map(runRow) : (data.rows ?? []).map((n) => noteRow(n, redraw, say));
  return items.length ? h("div", { class: "list" }, items) : h("div", { class: "empty" }, T.memory.none);
}

export async function memory(view: HTMLElement): Promise<() => void> {
  const say = h("div", { class: "state" });
  const top = h("div", {});
  const body = h("div", {});
  let live = true;
  let ask = 0;
  const fail = (e: unknown) => {
    say.className = "state bad";
    say.textContent = (e as Error).message;
  };
  const load = async () => {
    const mine = ++ask;
    const args = Object.entries(state).filter(([, v]) => v !== "" && v !== 0).map(([k, v]) => `--${k}=${v}`);
    const data = await board<MemoryData>("memory", ...args).catch((e) => {
      fail(e);
      return null;
    });
    if (!data || !live || mine !== ask) return;
    mount(top, shelfChips(data, () => void load()), projectPick(data, () => void load()));
    mount(body, rowsOf(data, () => void load(), fail), pager(data, () => void load()));
  };
  const words = h("input", { type: "search", placeholder: T.memory.find, value: state.words });
  words.addEventListener("input", debounce(() => {
    state.words = words.value.trim();
    state.offset = 0;
    void load();
  }, 200));
  mount(view, h("div", { class: "page" }, h("h1", {}, T.memory.title), h("div", { class: "sub" }, T.memory.sub), h("div", { class: "row filters" }, words), top, say, body));
  await load();
  return () => {
    live = false;
  };
}

function projectPick(data: MemoryData, redraw: () => void): HTMLElement | null {
  if (!data.projects.length) return null;
  const choices = [{ value: "", label: T.memory.allProjects }, ...data.projects.map((p) => ({ value: p.name, label: p.name, hint: String(p.count) }))];
  return h(
    "div",
    { class: "row" },
    pick(choices, state.project, (v) => {
      state.project = v;
      state.offset = 0;
      redraw();
    }),
  );
}
