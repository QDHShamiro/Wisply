import Graph from "graphology";
import forceAtlas2, { inferSettings } from "graphology-layout-forceatlas2";
import FA2Layout from "graphology-layout-forceatlas2/worker";
import Sigma from "sigma";
import { board, type GraphData } from "../api";
import { ago, enc, go, h, mount } from "../dom";
import { T } from "../text";

const PALETTE = ["#4f7cff", "#ff3d8b", "#12c99b", "#ff9f1a", "#a259ff", "#ff5c39", "#1bb8f0", "#8bc34a", "#e040fb", "#00bfa5"];
const css = (name: string) => getComputedStyle(document.documentElement).getPropertyValue(name).trim();
const FONT = "Segoe UI Variable Text, Segoe UI, system-ui, sans-serif";
const STORE = "wisply.graph.v3:";
const SYNC_MAX = 3000;
const NUDGE_MS = 900;
const KINDS = ["note", "project", "session"] as const;
const REPORT_TOP = 10;
const LABELED = 14;
const CLUSTER_MIN = 3;
const ROUNDS = 15;

type Kind = (typeof KINDS)[number];
type Attrs = { label: string; kind: Kind; group: string; cluster: string; t: number; x: number; y: number; size: number; color: string; deg: number };
type Spots = Record<string, [number, number]>;

function loadSpots(key: string): Spots {
  try {
    return JSON.parse(localStorage.getItem(STORE + key) ?? "{}") as Spots;
  } catch {
    return {};
  }
}

function saveSpots(key: string, g: Graph<Attrs>): void {
  const out: Spots = {};
  g.forEachNode((n, a) => {
    out[n] = [Math.round(a.x * 10) / 10, Math.round(a.y * 10) / 10];
  });
  try {
    localStorage.setItem(STORE + key, JSON.stringify(out));
  } catch {
    localStorage.removeItem(STORE + key);
  }
}

function place(g: Graph<Attrs>, spots: Spots): number {
  let fresh = 0;
  g.forEachNode((n, a) => {
    const s = spots[n];
    if (s) {
      a.x = s[0];
      a.y = s[1];
      return;
    }
    fresh++;
    const near = g.neighbors(n).map((m) => spots[m]).filter((x): x is [number, number] => !!x);
    if (near.length) {
      a.x = near.reduce((v, s2) => v + s2[0], 0) / near.length + (Math.random() - 0.5) * 4;
      a.y = near.reduce((v, s2) => v + s2[1], 0) / near.length + (Math.random() - 0.5) * 4;
    }
  });
  return fresh;
}

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

function communities(g: Graph<Attrs>): Map<string, string> {
  const lab = new Map(g.nodes().map((n) => [n, n]));
  for (let r = 0; r < ROUNDS; r++) {
    let moved = false;
    g.forEachNode((n) => {
      const count = new Map<string, number>();
      g.forEachNeighbor(n, (m) => {
        const l = lab.get(m) as string;
        count.set(l, (count.get(l) ?? 0) + 1);
      });
      let best = lab.get(n) as string;
      let most = 0;
      for (const [l, c] of count) {
        if (c > most || (c === most && l < best)) {
          best = l;
          most = c;
        }
      }
      if (most > 0 && best !== lab.get(n)) {
        lab.set(n, best);
        moved = true;
      }
    });
    if (!moved) break;
  }
  return lab;
}

function seed(data: GraphData): { g: Graph<Attrs>; colors: Map<string, string>; names: Map<string, string> } {
  const g = new Graph<Attrs>({ type: "undirected", multi: false, allowSelfLoops: false });
  const deg = new Array<number>(data.nodes.length).fill(0);
  for (const [a, b] of data.edges) {
    deg[a]++;
    deg[b]++;
  }
  data.nodes.forEach(([id, label, kind, group, t], i) => {
    const k = kind as Kind;
    g.mergeNode(id, {
      label: label || id,
      kind: k,
      group,
      cluster: "",
      t,
      x: (Math.random() - 0.5) * 100,
      y: (Math.random() - 0.5) * 100,
      size: k === "project" ? 3 + Math.sqrt(deg[i]) * 0.5 : 1.2 + Math.sqrt(deg[i]) * 0.5,
      color: css("--g-other"),
      deg: deg[i],
    });
  });
  for (const [a, b] of data.edges) {
    const s = data.nodes[a][0];
    const d = data.nodes[b][0];
    if (s !== d && !g.hasEdge(s, d)) g.addEdge(s, d);
  }
  const named = new Set(data.nodes.map((n) => n[3]).filter(Boolean));
  const lab = named.size >= 2 ? null : communities(g);
  g.forEachNode((n, a) => {
    a.cluster = lab ? (lab.get(n) as string) : a.group;
  });
  const sizes = new Map<string, number>();
  g.forEachNode((_, a) => sizes.set(a.cluster, (sizes.get(a.cluster) ?? 0) + 1));
  const top = [...sizes.entries()].filter(([c, n]) => c && n >= CLUSTER_MIN).sort((a, b) => b[1] - a[1]).slice(0, PALETTE.length);
  const colors = new Map<string, string>();
  const names = new Map<string, string>();
  top.forEach(([c], i) => {
    colors.set(c, PALETTE[i]);
    if (lab) {
      const hub = g.filterNodes((_, a) => a.cluster === c).sort((x, y) => g.getNodeAttribute(y, "deg") - g.getNodeAttribute(x, "deg"))[0];
      names.set(c, g.getNodeAttribute(hub, "label"));
    } else names.set(c, c);
  });
  g.forEachNode((_, a) => {
    a.color = colors.get(a.cluster) ?? css("--g-other");
  });
  return { g, colors, names };
}

export async function graph(view: HTMLElement, project?: string): Promise<() => void> {
  const [LABEL, DIM, EDGE, EDGE_ON, BACKDROP, BACKDROP_LINE] = ["--g-label", "--g-dim", "--g-edge", "--g-edge-on", "--g-back", "--g-back-line"].map(css);
  mount(view, h("div", { class: "empty", style: "margin:24px" }, T.graph.loading));
  const data = await board<GraphData>("graph", ...(project ? [project] : []));
  const { g, colors, names } = seed(data);
  const key = project ?? "all";
  const fresh = place(g, loadSpots(key));
  const canvas = h("div", { class: "canvas" });
  const info = h("span", { class: "info" }, T.graph.nodes(g.order, g.size));
  const side = h("aside", {});
  const hidden = new Set<Kind>();
  let orphans = false;
  const named = new Set(g.nodes().sort((a, b) => g.getNodeAttribute(b, "deg") - g.getNodeAttribute(a, "deg")).slice(0, LABELED));
  let focus: string | null = null;
  let hover: string | null = null;
  const times = g.mapNodes((_, a) => a.t).filter((t) => t > 0);
  const [t0, t1] = times.length ? [Math.min(...times), Math.max(...times)] : [0, 0];
  let until = t1;
  const shown = (a: Attrs) => !hidden.has(a.kind) && (orphans || a.deg > 0) && (a.t === 0 || a.t <= until);
  const lit = () => focus ?? hover;
  let near = new Set<string>();
  const light = () => {
    const n = lit();
    near = n ? new Set(g.neighbors(n)) : new Set();
  };

  let layout: FA2Layout | null = null;
  let stop = 0;
  if (fresh && g.order <= SYNC_MAX) {
    forceAtlas2.assign(g, { iterations: fresh === g.order ? 400 : 60, settings: { ...inferSettings(g), scalingRatio: 14, gravity: 0.6, slowDown: 2 } });
    saveSpots(key, g);
  } else if (fresh) {
    info.textContent = T.graph.layout;
    layout = new FA2Layout(g, { settings: { ...inferSettings(g), barnesHutOptimize: true, slowDown: 2 } });
    layout.start();
    stop = window.setTimeout(() => {
      layout?.stop();
      saveSpots(key, g);
      info.textContent = T.graph.nodes(g.order, g.size);
    }, fresh === g.order ? Math.min(8000, 1500 + g.order / 3) : NUDGE_MS);
  }

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
    const d = renderer.getNodeDisplayData(n);
    if (d) void renderer.getCamera().animate({ x: d.x, y: d.y, ratio: 0.6 }, { duration: 350 });
  };

  const pick = (n: string | null) => {
    focus = n;
    light();
    redraw();
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
      h("div", { class: "stage" }, canvas, h("div", { class: "tools" }, find, info), h("div", { class: "hint-box" }, T.graph.focus), t1 > t0 ? h("div", { class: "time" }, slider, label) : null),
      side,
    ),
  );
  overview();
  const renderer = new Sigma(g, canvas, {
    settings: {
      hideEdgesOnMove: g.order > SYNC_MAX,
      hideLabelsOnMove: g.order > SYNC_MAX,
      labelRenderedSizeThreshold: 7,
      labelDensity: 0.35,
      labelGridCellSize: 110,
      minEdgeThickness: 0.8,
      zoomToSizeRatioFunction: () => 1,
    },
    nodeReducer: (n, _d, a) => {
      const out: Record<string, unknown> = {
        labelColor: LABEL,
        labelFont: FONT,
        labelSize: 13,
        backdropColor: BACKDROP,
        backdropBorderColor: BACKDROP_LINE,
        backdropBorderWidth: 1,
        backdropCornerRadius: 6,
        backdropShadowColor: "rgba(0,0,0,0.5)",
        cursor: "pointer",
      };
      if (!shown(a as Attrs)) out.visibility = "hidden";
      else if (lit() && n !== lit() && !near.has(n)) {
        out.color = DIM;
        out.labelVisibility = "hidden";
      } else if (lit()) out.labelVisibility = "visible";
      else if (named.has(n)) out.labelVisibility = "visible";
      if (n === focus) out.backdropVisibility = "visible";
      return out;
    },
    edgeReducer: (e, _d, _a, _s, _gs, gr) => {
      const [s, t] = gr.extremities(e);
      if (!shown(gr.getNodeAttributes(s) as Attrs) || !shown(gr.getNodeAttributes(t) as Attrs)) return { visibility: "hidden" };
      const n = lit();
      if (n) return s === n || t === n ? { color: EDGE_ON, size: 1.6 } : { color: DIM };
      return { color: EDGE };
    },
  });
  const redraw = frame(() => renderer.refresh({ skipIndexation: true }));
  renderer.on("enterNode", ({ node }) => {
    hover = node;
    light();
    redraw();
  });
  renderer.on("leaveNode", () => {
    hover = null;
    light();
    redraw();
  });
  renderer.on("clickNode", ({ node }) => pick(node === focus ? null : node));
  renderer.on("clickStage", () => pick(null));
  renderer.on("doubleClickNode", ({ node }) => open(node));

  return () => {
    clearTimeout(stop);
    layout?.kill();
    renderer.kill();
  };
}
