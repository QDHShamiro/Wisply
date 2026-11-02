import { h, mount, pick } from "../dom";
import { T } from "../text";
import { bar } from "./agent-bar";
import { type Agent, type Overview, STATUSES, type Status, clock, list, modelLabel, overview, tokens, watch } from "./agent-data";
import { detail } from "./agent-detail";

const POLL_MS = 4000;
const TICK_MS = 1000;
const STAGGER_MAX = 24;
const HERO = "ag-hero";
type Morph = { finished: Promise<unknown> };
type Doc = Document & { startViewTransition?: (change: () => void) => Morph };
type Sort = "status" | "name" | "time" | "cost";

const SORTS: Record<Sort, (a: Agent, b: Agent) => number> = {
  status: () => 0,
  name: (a, b) => (a.name || a.dir_short).localeCompare(b.name || b.dir_short),
  time: (a, b) => b.since - a.since,
  cost: (a, b) => b.cost_est - a.cost_est,
};

export async function agents(view: HTMLElement): Promise<() => void> {
  let rows: Agent[] = [];
  let info: Overview | null = null;
  let loaded = false;
  let failed = false;
  let filter: Status | "all" = "all";
  let query = "";
  let sort: Sort = "status";
  let grouped = true;
  let open: { close: () => void } | null = null;
  const tiles = new Map<string, HTMLElement>();
  const grid = h("div", { class: "ag-groups" });
  const chips = h("div", { class: "ag-chips", role: "group" });
  const stats = bar();
  const sum = h("div", { class: "sub" }, T.agents.sub);
  const search = h("input", {
    type: "search",
    placeholder: T.agents.search,
    oninput: (e: Event) => {
      query = (e.target as HTMLInputElement).value.trim().toLowerCase();
      draw();
    },
  });
  const sorter = pick(
    (Object.keys(SORTS) as Sort[]).map((s) => ({ value: s, label: `${T.agents.sortBy}: ${T.agents.sorts[s]}` })),
    sort,
    (v) => {
      sort = v as Sort;
      draw();
    },
  );
  const groupBtn = h("button", {
    type: "button",
    class: "ag-chipbtn on",
    onclick: () => {
      grouped = !grouped;
      groupBtn.classList.toggle("on", grouped);
      draw();
    },
  }, T.agents.byProject);
  const page = h("div", { class: "page ag-page" });
  const tools = h("div", { class: "ag-tools" }, search, sorter, groupBtn);
  const board = h("div", { class: "ag-board" }, h("div", { class: "row" }, h("div", { class: "grow" }, h("h1", {}, T.agents.title), sum)), stats.el, tools, chips, grid);

  const count = (s: Status | "all") => (s === "all" ? rows.length : rows.filter((r) => r.status === s).length);

  const drawChips = () =>
    mount(
      chips,
      ...(["all", ...STATUSES] as const).map((s) =>
        h(
          "button",
          {
            type: "button",
            class: `ag-chipbtn ${s === filter ? "on" : ""} s-${s}`,
            onclick: () => {
              filter = s;
              draw();
            },
          },
          s === "all" ? T.agents.all : s,
          h("span", { class: "n" }, String(count(s))),
        ),
      ),
    );

  const tile = (a: Agent): HTMLElement => {
    const el = h(
      "button",
      { type: "button", class: `ag-tile s-${a.status}`, onclick: () => openDetail(a) },
      h("div", { class: "ag-top" }, h("span", { class: "ag-dot" }), h("b", { class: "ag-name ellipsis" }, a.name || a.dir_short), h("span", { class: "pill" }, a.status)),
      h("div", { class: "ag-sub ellipsis" }, `${a.dir_short} · ${modelLabel(a)}`),
      h("div", { class: "ag-line1" }, a.last_line || "—"),
      h("div", { class: "ag-foot" }, h("span", { class: "ag-time", "data-since": a.since }, clock(Date.now() / 1000 - a.since)), h("span", { class: "ag-cost" }, `${tokens(a.tokens)} · $${a.cost_est.toFixed(2)}`)),
      h("div", { class: `ag-prog ${a.has_result ? "full" : ""}` }, h("i", {})),
    );
    el.addEventListener("animationend", (e) => {
      if (e.target === el) el.classList.add("seen");
    });
    return el;
  };

  const patch = (el: HTMLElement, a: Agent) => {
    const prev = STATUSES.find((s) => el.classList.contains(`s-${s}`));
    if (prev !== a.status) {
      if (prev) el.classList.remove(`s-${prev}`);
      el.classList.add(`s-${a.status}`);
      el.classList.remove("flip");
      void el.offsetWidth;
      el.classList.add("flip");
    }
    el.onclick = () => openDetail(a);
    el.querySelector(".pill")!.textContent = a.status;
    el.querySelector(".ag-line1")!.textContent = a.last_line || "—";
    el.querySelector(".ag-sub")!.textContent = `${a.dir_short} · ${modelLabel(a)}`;
    (el.querySelector(".ag-time") as HTMLElement).dataset.since = String(a.since);
    el.querySelector(".ag-cost")!.textContent = `${tokens(a.tokens)} · $${a.cost_est.toFixed(2)}`;
    el.querySelector(".ag-prog")!.classList.toggle("full", !!a.has_result);
  };

  const section = (name: string, items: HTMLElement[]) =>
    h("section", { class: "ag-group" }, name ? h("h3", { class: "ag-ghead" }, name, h("span", { class: "n" }, String(items.length))) : null, h("div", { class: "ag-grid" }, ...items));

  const draw = () => {
    drawChips();
    if (!loaded) return;
    sum.textContent = failed ? T.agents.offline : T.agents.sub;
    stats.update(rows, info);
    const shown = rows
      .filter((r) => (filter === "all" || r.status === filter) && (!query || `${r.name} ${r.dir_short} ${r.model} ${r.last_line}`.toLowerCase().includes(query)))
      .sort(SORTS[sort]);
    if (!shown.length) {
      tiles.clear();
      mount(grid, h("div", { class: "empty" }, rows.length ? T.agents.nomatch : T.agents.none));
      return;
    }
    const first = tiles.size === 0;
    const keep = new Set(shown.map((r) => r.id));
    for (const id of [...tiles.keys()]) if (!keep.has(id)) tiles.delete(id);
    const made = new Map<string, HTMLElement>();
    shown.forEach((a, i) => {
      let el = tiles.get(a.id);
      if (el) patch(el, a);
      else {
        el = tile(a);
        if (first) el.style.setProperty("--i", String(Math.min(i, STAGGER_MAX)));
        else el.classList.add("pop");
        tiles.set(a.id, el);
      }
      made.set(a.id, el);
    });
    if (!grouped) return mount(grid, section("", shown.map((a) => made.get(a.id)!)));
    const by = new Map<string, HTMLElement[]>();
    for (const a of shown) by.set(a.dir_short || "—", [...(by.get(a.dir_short || "—") ?? []), made.get(a.id)!]);
    mount(grid, ...[...by].map(([name, items]) => section(name, items)));
  };

  const morph = (change: () => void, hero: () => HTMLElement | undefined) => {
    const doc = document as Doc;
    if (!doc.startViewTransition || document.visibilityState !== "visible" || window.matchMedia("(prefers-reduced-motion: reduce)").matches) return change();
    const before = hero();
    if (before) before.style.viewTransitionName = HERO;
    const run = doc.startViewTransition(() => {
      change();
      const after = hero();
      if (after) after.style.viewTransitionName = HERO;
    });
    void run.finished.finally(() => {
      for (const el of grid.querySelectorAll<HTMLElement>(".ag-tile")) el.style.viewTransitionName = "";
    });
  };

  const openDetail = (a: Agent) => {
    open?.close();
    const back = () => {
      open?.close();
      open = null;
      morph(
        () => {
          mount(page, board);
          draw();
        },
        () => tiles.get(a.id),
      );
      void pull();
    };
    const d = detail(a, back);
    open = d;
    morph(
      () => mount(page, d.el),
      () => tiles.get(a.id),
    );
  };

  const pull = async () => {
    const [next, status] = await Promise.all([list(), overview()]);
    failed = next === null;
    if (next) rows = next;
    info = status;
    loaded = true;
    if (!open) draw();
  };

  mount(page, board);
  mount(view, page);
  mount(grid, h("div", { class: "ag-grid" }, ...Array.from({ length: 8 }, () => h("div", { class: "ag-tile skel" }))));
  drawChips();
  await pull();
  const poll = window.setInterval(() => void pull(), POLL_MS);
  const unwatch = watch(() => void pull());
  const tick = window.setInterval(() => {
    const now = Date.now() / 1000;
    for (const el of grid.querySelectorAll<HTMLElement>(".s-running .ag-time")) el.textContent = clock(now - Number(el.dataset.since));
  }, TICK_MS);
  return () => {
    clearInterval(poll);
    clearInterval(tick);
    unwatch();
    open?.close();
  };
}
