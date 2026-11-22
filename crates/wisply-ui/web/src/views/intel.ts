import { board, type IntelData } from "../api";
import { h, mount } from "../dom";
import { T } from "../text";

const RANGE_KEY = "wisply.intel";
const RANGES = ["7", "30", "90", "all"];
const LEARN = ["remembered", "learned", "vault", "claude", "lessons", "routines"];
const WORK = ["sessions", "done", "bugs", "ideas"];
const NS = "http://www.w3.org/2000/svg";
const W = 800;
const H = 200;
const PAD = 6;
const LABELS = 8;
const DAY_BARS = 45;
const WEEK_BARS = 200;

type Bucket = { label: string; short: string; values: number[] };

const num = (n: number) => Math.round(n).toLocaleString("en-US");
const sum = (v: number[]) => v.reduce((a, b) => a + b, 0);
const last = (v: number[]) => v[v.length - 1] ?? 0;

function svg<K extends keyof SVGElementTagNameMap>(tag: K, attrs: Record<string, string | number>): SVGElementTagNameMap[K] {
  const el = document.createElementNS(NS, tag);
  for (const [k, v] of Object.entries(attrs)) el.setAttribute(k, String(v));
  return el;
}

function niceDay(day: string | undefined | null): string {
  const [y, m, d] = (day ?? "").split("-").map(Number);
  if (!y || !m || !d) return day ?? "";
  return new Date(y, m - 1, d).toLocaleDateString("en-US", { month: "short", day: "numeric" });
}

function delta(w: [number, number] | undefined): string {
  const [n, prev] = w ?? [0, 0];
  return T.intel.week(n, prev);
}

function tile(label: string, value: string, note: string, extra = "", cls = ""): HTMLElement {
  return h("div", { class: `stat ${cls}` }, h("div", { class: "label" }, label), h("div", { class: "value" }, value), h("div", { class: "note" }, note), extra ? h("div", { class: "note" }, extra) : null);
}

function hover(box: HTMLElement, days: string[], values: number[], at: (i: number) => [number, number]): void {
  const mark = h("div", { class: "mark" });
  const tip = h("div", { class: "tip" });
  box.append(mark, tip);
  box.addEventListener("mousemove", (e) => {
    const r = box.getBoundingClientRect();
    const i = Math.max(0, Math.min(values.length - 1, Math.round(((e.clientX - r.left) / r.width) * (values.length - 1))));
    const [x, y] = at(i);
    mark.style.left = `${(x / W) * 100}%`;
    mark.style.top = `${(y / H) * 100}%`;
    tip.style.left = `${Math.min(80, Math.max(0, (x / W) * 100 - 8))}%`;
    tip.textContent = T.intel.tip(niceDay(days[i]), num(values[i] ?? 0));
    box.classList.add("hover");
  });
  box.addEventListener("mouseleave", () => box.classList.remove("hover"));
}

function line(title: string, days: string[], values: number[], cls: string): HTMLElement {
  const max = Math.max(1, ...values);
  const min = Math.min(...values);
  const lo = min === max ? 0 : Math.max(0, min - (max - min) * 0.15);
  const x = (i: number) => (values.length < 2 ? W / 2 : (i / (values.length - 1)) * W);
  const y = (v: number) => H - PAD - ((v - lo) / (max - lo || 1)) * (H - 2 * PAD);
  const pts = values.map((v, i) => `${x(i).toFixed(1)},${y(v).toFixed(1)}`).join(" ");
  const chart = svg("svg", { viewBox: `0 0 ${W} ${H}`, preserveAspectRatio: "none", class: "line" });
  chart.append(svg("polygon", { points: `0,${H} ${pts} ${W},${H}`, class: "area" }), svg("polyline", { points: pts, class: "stroke", "vector-effect": "non-scaling-stroke" }));
  const box = h("div", { class: "plot" }, chart, h("span", { class: "ymax" }, num(max)), h("span", { class: "ymin" }, num(lo)));
  hover(box, days, values, (i) => [x(i), y(values[i] ?? 0)]);
  const axis = h("div", { class: "axis" }, h("span", {}, niceDay(days[0])), h("span", {}, niceDay(days[days.length - 1])));
  return h("section", { class: `panel intel-chart ${cls}` }, h("h2", { class: "row" }, h("span", { class: "grow" }, title), h("span", { class: "big" }, num(last(values)))), box, axis);
}

function buckets(days: string[], daily: Record<string, number[]>, keys: string[]): Bucket[] {
  const n = days.length;
  const size = n <= DAY_BARS ? 1 : n <= WEEK_BARS ? 7 : 30;
  const out: Bucket[] = [];
  for (let end = n; end > 0; end -= size) {
    const start = Math.max(0, end - size);
    const label = size === 1 ? niceDay(days[start]) : `${niceDay(days[start])} – ${niceDay(days[end - 1])}`;
    out.unshift({ label, short: niceDay(days[start]), values: keys.map((k) => sum((daily[k] ?? []).slice(start, end))) });
  }
  return out;
}

function column(g: Bucket, keys: string[], max: number, showDay: boolean): HTMLElement {
  const lines = keys.map((k, j) => (g.values[j] ? `${T.intel.series[k] ?? k}: ${num(g.values[j])}` : "")).filter(Boolean);
  return h(
    "div",
    { class: "scol", title: [g.label, ...lines].join("\n") },
    h("div", { class: "sstack" }, keys.map((k, j) => (g.values[j] ? h("div", { class: `seg s-${k}`, style: `height:${(g.values[j] / max) * 100}%` }) : null))),
    h("div", { class: "sday" }, showDay ? g.short : ""),
  );
}

function stacked(title: string, days: string[], daily: Record<string, number[]>, keys: string[]): HTMLElement {
  const groups = buckets(days, daily, keys);
  const max = Math.max(1, ...groups.map((g) => sum(g.values)));
  const every = Math.ceil(groups.length / LABELS);
  const cols = groups.map((g, i) => column(g, keys, max, i % every === 0 || i === groups.length - 1));
  const legend = h("div", { class: "legend" }, keys.map((k) => h("span", {}, h("i", { class: `s-${k}` }), T.intel.series[k] ?? k, h("b", {}, num(sum(daily[k] ?? []))))));
  return h("section", { class: "panel intel-chart" }, h("h2", {}, title), h("div", { class: "sbars" }, cols), legend);
}

function tiles(d: IntelData): HTMLElement {
  const t = d.totals;
  const today = last(d.gain);
  const high = today > 0 && today >= d.best;
  const best = high ? T.intel.newHigh : d.best ? T.intel.best(num(d.best), niceDay(d.best_day)) : "";
  const rate = t.ups + t.downs ? Math.round((t.ups / (t.ups + t.downs)) * 100) : 0;
  return h(
    "div",
    { class: "stats intel-stats" },
    tile(T.intel.score, num(last(d.score)), delta(d.weeks.score), best, high ? "good" : "accent"),
    tile(T.intel.knowledge, num(last(d.knowledge)), delta(d.weeks.knowledge), T.intel.knowledgeNote(t.notes ?? 0)),
    tile(T.intel.lessons, num(t.lessons ?? 0), T.intel.lessonsNote(t.ups ?? 0, t.downs ?? 0), rate ? T.intel.rate(rate) : delta(d.weeks.lessons)),
    tile(T.intel.routines, num(t.routines ?? 0), T.intel.routinesNote(t.routine_runs ?? 0, t.routine_ok ?? 0), delta(d.weeks.routines)),
    tile(T.intel.done, num(t.done ?? 0), T.intel.doneNote(t.found ?? 0), delta(d.weeks.done)),
  );
}

function content(d: IntelData): HTMLElement[] {
  const t = d.totals;
  if (!last(d.score) && !t.notes) return [h("div", { class: "empty" }, T.intel.none)];
  const points = d.points.map(([k, w]) => `${w} × ${T.intel.series[k] ?? k}`).join(" + ");
  return [
    tiles(d),
    line(T.intel.growth, d.days, d.score, "score"),
    h("div", { class: "intel-pair" }, line(T.intel.knowledgeChart, d.days, d.knowledge, "knowledge"), line(T.intel.gainChart, d.days, d.gain, "gain")),
    stacked(T.intel.daily, d.days, d.daily, LEARN),
    stacked(T.intel.work, d.days, d.daily, WORK),
    h("div", { class: "footnote" }, T.intel.recallNote(t.hits ?? 0, t.recalled ?? 0, t.notes ?? 0)),
    h("div", { class: "footnote" }, T.intel.how(points)),
  ];
}

export async function intel(view: HTMLElement): Promise<() => void> {
  let range = RANGES.find((r) => r === localStorage.getItem(RANGE_KEY)) ?? "30";
  let live = true;
  let ask = 0;
  const seg = h("div", { class: "seg" });
  const body = h("div", { class: "intel-body" });
  const load = async () => {
    const mine = ++ask;
    mount(seg, ...RANGES.map((r) => h("button", { class: r === range ? "on" : "", onclick: () => pickRange(r) }, T.intel.ranges[r])));
    try {
      const d = await board<IntelData>("intelligence", range);
      if (live && mine === ask) mount(body, ...content(d));
    } catch (e) {
      if (live && mine === ask) mount(body, h("div", { class: "error" }, (e as Error).message));
    }
  };
  const pickRange = (r: string) => {
    range = r;
    localStorage.setItem(RANGE_KEY, r);
    void load();
  };
  mount(view, h("div", { class: "page intel" }, h("div", { class: "row head" }, h("div", { class: "grow" }, h("h1", {}, T.intel.title), h("div", { class: "sub" }, T.intel.sub)), seg), body));
  await load();
  return () => {
    live = false;
  };
}
