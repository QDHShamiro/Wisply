import { board, type GraphData } from "../api";
import { ago, enc, go, h, mount } from "../dom";
import { T } from "../text";
import { KINDS, NEON, forget, packs, seed, swell, tint, type Attrs, type Kind } from "./graph-data";
import { curtain, fit, spotlight } from "./graph-light";
import { motion } from "./graph-motion";
import { box, brief, nameless, spread, type Spot } from "./graph-names";
import { frame, idle } from "./graph-pace";
import { scatter } from "./graph-place";
import { sky } from "./graph-sky";
import { crown, grow, LABEL_MIN, LABEL_SIZE, lift, rgba, stage, veil, type Crown } from "./graph-style";

const css = (name: string) => getComputedStyle(document.documentElement).getPropertyValue(name).trim();
const REPORT_TOP = 10;
const LABELED = 14;
const GRID_MS = 200;
const WARM_MS = 2000;
const HUSH_MIN = 120;
const HUSH_MS = 160;
const NAME_MS = 140;
const NAME_BATCH = 400;
const LEGEND = 12;
const CROSS = 0.4;
const PLATES = 24;
const CENTER_MS = 350;
const VIEW_FALLBACK = 640;

export async function graph(view: HTMLElement, project?: string): Promise<() => void> {
  const tones = { label: css("--g-label"), dim: css("--g-dim"), edge: css("--g-edge"), edgeOn: css("--g-edge-on"), back: css("--g-back"), backLine: css("--g-back-line"), shadow: css("--g-shadow") };
  mount(view, h("div", { class: "empty", style: "margin:24px" }, T.graph.loading));
  const data = await board<GraphData>("graph", ...(project ? [project] : []));
  const big = document.documentElement.dataset.theme !== "light";
  const { g, colors, names } = seed(data, css("--g-other"), big ? NEON : undefined);
  tint(g, scatter(g), colors, rgba(tones.edge), CROSS);
  const k = big ? swell(g, Math.min(view.clientWidth || VIEW_FALLBACK, view.clientHeight || VIEW_FALLBACK)) : 1;
  forget();
  const dome = big ? h("canvas", { class: "sky" }) : null;
  const canvas = h("div", { class: "canvas" });
  const upper = h("div", { class: "canvas", style: "pointer-events:none" });
  const info = h("span", { class: "info" });
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
  for (const n of loudKeys) g.setNodeAttribute(n, "tag", brief(g.getNodeAttribute(n, "label")));
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
  let naming = 0;
  let wide = false;
  const count = () => {
    info.textContent = T.graph.nodes(g.order - filter.gone.size, g.order, g.size - filter.cut.size);
  };
  let plates = "";
  const seen = (spots: Spot[]) => {
    const { width, height } = live.getDimensions();
    return spread(spots, LABEL_SIZE, width, height);
  };
  const place = () => {
    const ratio = live.getCamera().ratio;
    const spots = [...named]
      .filter((n) => !filter.gone.has(n))
      .map((n) => {
        const a = g.getNodeAttributes(n);
        const p = live.graphToViewport(a);
        return { n, x: p.x, y: p.y, r: live.scaleSize(a.size, ratio) * k, text: a.tag };
      });
    const keep = new Set(seen(spots));
    const now = [...keep].join("\u0000");
    if (now === plates) return;
    plates = now;
    live.setNodesState([...named].filter((n) => !keep.has(n)), { named: false });
    live.setNodesState([...keep], { named: true });
  };
  const name = () => {
    place();
    plate();
    const { width, height } = live.getDimensions();
    const view = box([[0, 0], [width, 0], [0, height], [width, height]].map(([x, y]) => live.viewportToGraph({ x, y })));
    const add = nameless(g, view, LABEL_MIN * grow(live.getCamera().ratio), filter.gone, NAME_BATCH);
    if (!add.length) return;
    for (const n of add) {
      const a = g.getNodeAttributes(n);
      a.tag = brief(a.label);
      loud.add(n);
      loudKeys.push(n);
    }
    live.refresh({ partialGraph: { nodes: add }, schedule: true });
    if (glow) live.setNodesState(add, { mute: true });
  };
  const hush = (on: boolean) => {
    clearTimeout(rest);
    if (on) rest = window.setTimeout(() => hush(false), HUSH_MS);
    if (on === quiet || !over || (on && top.order <= HUSH_MIN)) return;
    quiet = on;
    over.setNodesState(top.nodes(), { hush: on });
  };
  const plate = () => {
    if (!glow) return;
    const ratio = live.getCamera().ratio;
    const spots = ranked().map((n) => {
      const a = top.getNodeAttributes(n);
      const p = live.graphToViewport(a);
      return { n, x: p.x, y: p.y, r: live.scaleSize(a.size, ratio) * k, text: a.tag };
    });
    const keep = new Set(seen(spots).slice(0, PLATES));
    top.forEachNode((n, a) => {
      const on = a.pin || keep.has(n);
      if (a.plate !== on) top.setNodeAttribute(n, "plate", on);
    });
  };
  const light = () => {
    const lit = spot(focus ?? hover, focus === null ? null : hover) > 0;
    if (lit !== glow) live.setNodesState(loudKeys, { mute: lit });
    glow = lit;
    quiet = false;
    if (lit) {
      over ??= crown(top, upper, tones, k);
      over.setCustomBBox(live.getBBox());
      over.getCamera().setState(live.getCamera().getState());
      plate();
    }
    live.scheduleRender();
  };
  const redraw = frame(() => {
    const moved = filter.sift(shown);
    if (!moved.length) return;
    count();
    name();
    if (glow) light();
    if (!moved.some((n) => loud.has(n))) return;
    clearTimeout(grid);
    grid = window.setTimeout(() => live.refresh({ partialGraph: { nodes: [] }, schedule: true }), GRID_MS);
  });

  const dye = (c: string) => `background:${c};--c:${c}`;
  const swatch = (n: string) => h("span", { class: "swatch", style: dye(g.getNodeAttribute(n, "color")) });
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
    const groups = (wide ? teams : teams.slice(0, LEGEND)).map((p) =>
      h(
        "a",
        {
          href: "#",
          class: "team",
          style: `--c:${p.color}`,
          onclick: (e: Event) => {
            e.preventDefault();
            pick(p.hub);
          },
        },
        h("span", { class: "swatch", style: dye(p.color) }),
        p.name,
        h("span", { class: "faint" }, ` · ${p.count.toLocaleString("en-US")}`),
      ),
    );
    const extra = teams.length - LEGEND;
    const more =
      extra > 0
        ? h(
            "a",
            {
              href: "#",
              class: "more",
              onclick: (e: Event) => {
                e.preventDefault();
                wide = !wide;
                overview();
              },
            },
            wide ? T.graph.fewer : T.graph.more(extra),
          )
        : null;
    mount(
      side,
      h("h2", { style: "margin-top:0" }, T.graph.show),
      toggles,
      groups.length ? h("h2", {}, T.graph.groups) : null,
      ...groups,
      more,
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
    if (!d) return;
    const reach = g.filterNeighbors(n, (m) => !filter.gone.has(m)).map((m) => {
      const e = live.getNodeDisplayData(m);
      return e ? Math.hypot(e.x - d.x, e.y - d.y) : 0;
    });
    void live.getCamera().animate({ x: d.x, y: d.y, ratio: fit(reach) }, { duration: CENTER_MS });
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
      h("div", { class: "stage" }, dome, canvas, upper, h("div", { class: "tools" }, find, info), h("div", { class: "hint-box" }, T.graph.focus), t1 > t0 ? h("div", { class: "time" }, slider, label) : null),
      side,
    ),
  );
  const teams = packs(g, colors, names);
  overview();

  const live = stage(g, canvas, tones, k);
  const filter = curtain(g, live);
  const { top, spot, ranked } = spotlight(g, filter.gone);
  live.addCustomLayerProgram("veil", "veil", veil(rgba(tones.dim), lift(rgba(tones.edge)), () => glow));
  filter.sift(shown);
  place();
  count();
  const heaven = dome ? sky(dome, live, { ring: css("--g-ring"), dust: css("--g-dust") }) : null;
  const drive = big
    ? motion({
        live,
        canvas,
        busy: () => glow,
        paint: (seconds) => {
          heaven?.draw(seconds);
          if (glow) over?.scheduleRender();
        },
        settle: name,
      })
    : null;
  heaven?.draw();
  naming = window.setTimeout(name, NAME_MS);
  live.getCamera().on("updated", (state) => {
    if (drive?.own()) return;
    drive?.poke();
    heaven?.draw();
    clearTimeout(naming);
    naming = window.setTimeout(name, NAME_MS);
    if (!glow) return;
    over?.getCamera().setState(state);
    hush(true);
  });
  const cool = idle(() => (over ??= crown(top, upper, tones, k)), WARM_MS);
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

  return () => {
    clearTimeout(grid);
    clearTimeout(rest);
    clearTimeout(naming);
    drive?.stop();
    cool();
    over?.kill();
    live.kill();
  };
}
