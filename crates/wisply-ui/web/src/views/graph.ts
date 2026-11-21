import { board, type GraphData } from "../api";
import { ago, enc, go, h, mount } from "../dom";
import { T } from "../text";
import { KINDS, loadSpots, place, saveSpots, seed, type Attrs, type Kind } from "./graph-data";
import { arrange, plan } from "./graph-layout";
import { curtain, spotlight } from "./graph-light";
import { crown, LABEL_MIN, lift, rgba, stage, veil, type Crown } from "./graph-style";

const css = (name: string) => getComputedStyle(document.documentElement).getPropertyValue(name).trim();
const REPORT_TOP = 10;
const LABELED = 14;
const GRID_MS = 200;
const WARM_MS = 2000;
const HUSH_MIN = 120;
const HUSH_MS = 160;

function frame(f: () => void): () => void {
  let queued = false;
  return () => {
    if (queued) return;
    queued = true;
    requestAnimationFrame(() => {
      queued = false;
      f();
    });
  };
}

export async function graph(view: HTMLElement, project?: string): Promise<() => void> {
  const tones = { label: css("--g-label"), dim: css("--g-dim"), edge: css("--g-edge"), edgeOn: css("--g-edge-on"), back: css("--g-back"), backLine: css("--g-back-line"), shadow: css("--g-shadow") };
  mount(view, h("div", { class: "empty", style: "margin:24px" }, T.graph.loading));
  const data = await board<GraphData>("graph", ...(project ? [project] : []));
  const { g, colors, names } = seed(data, css("--g-other"));
  const key = project ?? "all";
  const fresh = place(g, loadSpots(key));
  const canvas = h("div", { class: "canvas" });
  const upper = h("div", { class: "canvas", style: "pointer-events:none" });
  const info = h("span", { class: "info" }, T.graph.nodes(g.order, g.size));
  const side = h("aside", {});
  const hidden = new Set<Kind>();
  let orphans = false;
  const keys = g.nodes();
  const named = new Set([...keys].sort((a, b) => g.getNodeAttribute(b, "deg") - g.getNodeAttribute(a, "deg")).slice(0, LABELED));
  const loud = new Set(named);
  g.forEachNode((n, a) => {
    if (a.size >= LABEL_MIN) loud.add(n);
  });
  const loudKeys = [...loud];
  for (const n of loudKeys) g.setNodeAttribute(n, "tag", g.getNodeAttribute(n, "label"));
  let focus: string | null = null;
  let hover: string | null = null;
  const times = g.mapNodes((_, a) => a.t).filter((t) => t > 0);
  const [t0, t1] = times.length ? [Math.min(...times), Math.max(...times)] : [0, 0];
  let until = t1;
  const shown = (a: Attrs) => !hidden.has(a.kind) && (orphans || a.deg > 0) && (a.t === 0 || a.t <= until);
  let glow = false;
  let grid = 0;
  let over: Crown | null = null;
  let quiet = false;
  let rest = 0;
  const hush = (on: boolean) => {
    clearTimeout(rest);
    if (on) rest = window.setTimeout(() => hush(false), HUSH_MS);
    if (on === quiet || !over || (on && top.order <= HUSH_MIN)) return;
    quiet = on;
    over.setNodesState(top.nodes(), { hush: on });
  };
  const light = () => {
    const lit = spot(focus ?? hover, focus === null ? null : hover) > 0;
    if (lit !== glow) live.setNodesState(loudKeys, { mute: lit });
    glow = lit;
    quiet = false;
    if (lit) {
      over ??= crown(top, upper, tones);
      over.setCustomBBox(live.getBBox());
      over.getCamera().setState(live.getCamera().getState());
    }
    live.scheduleRender();
  };
  const redraw = frame(() => {
    const moved = filter.sift(shown);
    if (!moved.length) return;
    if (glow) light();
    if (!moved.some((n) => loud.has(n))) return;
    clearTimeout(grid);
    grid = window.setTimeout(() => live.refresh({ partialGraph: { nodes: [] }, schedule: true }), GRID_MS);
  });

  const swatch = (n: string) => h("span", { class: "swatch", style: `background:${g.getNodeAttribute(n, "color")}` });
  const link = (n: string) =>
    h(
      "a",
      {
        href: "#",
        onclick: (e: Event) => {
          e.preventDefault();
          pick(n);
        },
      },
      swatch(n),
      g.getNodeAttribute(n, "label"),
    );
  const open = (n: string) => {
    const a = g.getNodeAttributes(n);
    if (a.kind === "note") go(`#/n/${n.slice(1)}`);
    else if (a.kind === "project") go(`#/p/${enc(a.group)}`);
  };

  const toggles = h(
    "div",
    { class: "legend" },
    KINDS.map((k) =>
      h(
        "label",
        {},
        h("input", {
          type: "checkbox",
          checked: true,
          onchange: (e: Event) => {
            if ((e.target as HTMLInputElement).checked) hidden.delete(k);
            else hidden.add(k);
            redraw();
          },
        }),
        T.graph.kinds[k],
      ),
    ),
    h(
      "label",
      {},
      h("input", {
        type: "checkbox",
        checked: orphans,
        onchange: (e: Event) => {
          orphans = (e.target as HTMLInputElement).checked;
          redraw();
        },
      }),
      T.graph.orphans,
    ),
  );

  const overview = () => {
    const hubs = g
      .filterNodes((_, a) => a.kind === "note")
      .sort((a, b) => g.getNodeAttribute(b, "deg") - g.getNodeAttribute(a, "deg"))
      .slice(0, REPORT_TOP);
    const groups = [...colors.keys()].map((c) => {
      const hub = g.findNode((_, a) => a.cluster === c && a.label === names.get(c)) ?? g.findNode((_, a) => a.cluster === c);
      const count = g.filterNodes((_, a) => a.cluster === c).length;
      return h(
        "a",
        {
          href: "#",
          onclick: (e: Event) => {
            e.preventDefault();
            if (hub) pick(hub);
          },
        },
        h("span", { class: "swatch", style: `background:${colors.get(c)}` }),
        names.get(c) ?? c,
        h("span", { class: "faint" }, ` · ${count}`),
      );
    });
    mount(
      side,
      h("h2", { style: "margin-top:0" }, T.graph.show),
      toggles,
      groups.length ? h("h2", {}, T.graph.groups) : null,
      ...groups,
      h("h2", {}, T.graph.hubs),
      ...hubs.map(link),
    );
  };

  const detail = (n: string) => {
    const a = g.getNodeAttributes(n);
    const links = g.neighbors(n).sort((x, y) => g.getNodeAttribute(y, "deg") - g.getNodeAttribute(x, "deg"));
    mount(
      side,
      h("div", { class: "detail" },
        h("button", { class: "back", onclick: () => pick(null) }, `← ${T.graph.back}`),
        h("h3", {}, a.label),
        h("div", { class: "meta" }, [T.graph.kinds[a.kind], a.t ? T.ago(ago(a.t)) : "", colors.has(a.cluster) ? names.get(a.cluster) : ""].filter(Boolean).join(" · ")),
        h("div", { class: "actions" },
          a.kind === "session" ? null : h("button", { class: "primary", onclick: () => open(n) }, T.graph.open),
          h("button", { onclick: () => center(n) }, T.graph.center),
        ),
        h("h2", {}, T.graph.connections, h("span", { class: "n" }, links.length)),
        links.length ? links.map(link) : h("div", { class: "faint" }, T.graph.alone),
      ),
    );
  };

  const center = (n: string) => {
    const d = live.getNodeDisplayData(n);
    if (d) void live.getCamera().animate({ x: d.x, y: d.y, ratio: 0.3 }, { duration: 350 });
  };

  const pick = (n: string | null) => {
    focus = n;
    light();
    if (n) {
      detail(n);
      center(n);
    } else overview();
  };

  const find = h("input", {
    type: "search",
    class: "find",
    placeholder: T.graph.find,
    onkeydown: (e: Event) => {
      if ((e as KeyboardEvent).key !== "Enter") return;
      const w = (e.target as HTMLInputElement).value.trim().toLowerCase();
      if (!w) return pick(null);
      const hits = g.filterNodes((_, a) => a.label.toLowerCase().includes(w));
      hits.sort((x, y) => Number(g.getNodeAttribute(y, "kind") === "project") - Number(g.getNodeAttribute(x, "kind") === "project") || g.getNodeAttribute(y, "deg") - g.getNodeAttribute(x, "deg"));
      if (hits.length) pick(hits[0]);
    },
  });
  const label = h("span", {}, T.graph.all);
  const slider = h("input", {
    type: "range",
    min: 0,
    max: 1000,
    value: 1000,
    "aria-label": T.graph.until,
    oninput: (e: Event) => {
      const v = Number((e.target as HTMLInputElement).value) / 1000;
      until = t0 + (t1 - t0) * v;
      label.textContent = v >= 1 ? T.graph.all : `${T.graph.until} ${new Date(until * 1000).toLocaleDateString("en-US", { year: "numeric", month: "short", day: "numeric" })}`;
      redraw();
    },
  });
  mount(
    view,
    h(
      "div",
      { class: "graph" },
      h("div", { class: "stage" }, canvas, upper, h("div", { class: "tools" }, find, info), h("div", { class: "hint-box" }, T.graph.focus), t1 > t0 ? h("div", { class: "time" }, slider, label) : null),
      side,
    ),
  );
  overview();

  const live = stage(g, canvas, tones);
  const filter = curtain(g, live);
  const { top, spot } = spotlight(g, filter.gone);
  live.addCustomLayerProgram("veil", "veil", veil(rgba(tones.dim), lift(rgba(tones.edge)), () => glow));
  live.setNodesState([...named], { named: true });
  filter.sift(shown);
  live.getCamera().on("updated", (state) => {
    if (!glow) return;
    over?.getCamera().setState(state);
    hush(true);
  });
  const warm = requestIdleCallback(() => (over ??= crown(top, upper, tones)), { timeout: WARM_MS });
  live.on("enterNode", ({ node }) => {
    hover = node;
    light();
  });
  live.on("leaveNode", () => {
    hover = null;
    light();
  });
  live.on("clickNode", ({ node }) => pick(node === focus ? null : node));
  live.on("clickStage", () => pick(null));
  live.on("doubleClickNode", ({ node }) => open(node));

  let halt = () => {};
  if (fresh) {
    info.textContent = T.graph.layout;
    halt = arrange(
      g,
      plan(g.order, fresh),
      () => {
        live.refresh({ partialGraph: { nodes: keys } });
        if (glow) light();
      },
      () => {
        saveSpots(key, g);
        info.textContent = T.graph.nodes(g.order, g.size);
      },
    );
  }

  return () => {
    clearTimeout(grid);
    clearTimeout(rest);
    cancelIdleCallback(warm);
    halt();
    over?.kill();
    live.kill();
  };
}
