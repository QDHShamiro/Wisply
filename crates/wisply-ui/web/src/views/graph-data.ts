import Graph from "graphology";
import type { GraphData } from "../api";

export const PALETTE = ["#4f7cff", "#ff3d8b", "#12c99b", "#ff9f1a", "#a259ff", "#ff5c39", "#1bb8f0", "#8bc34a", "#e040fb", "#00bfa5"];
export const KINDS = ["note", "project", "session"] as const;
const FAMILY = "wisply.graph.";
const CLUSTER_MIN = 3;
const ROUNDS = 15;

export type Kind = (typeof KINDS)[number];
export type Attrs = { label: string; tag: string; kind: Kind; group: string; cluster: string; t: number; x: number; y: number; size: number; color: string; deg: number };
export type Net = Graph<Attrs>;
export type Pack = { key: string; name: string; color: string; count: number; hub: string };

export function forget(): void {
  try {
    for (const k of Object.keys(localStorage)) if (k.startsWith(FAMILY)) localStorage.removeItem(k);
  } catch {
    return;
  }
}

function hex(v: number): string {
  return Math.round(Math.min(1, Math.max(0, v)) * 255)
    .toString(16)
    .padStart(2, "0");
}

export function tint(g: Net, home: Map<string, string>, colors: Map<string, string>, edge: readonly [number, number, number, number], cross: number): void {
  g.forEachNode((n, a) => {
    const c = colors.get(home.get(n) ?? "");
    if (c !== undefined && !colors.has(a.cluster)) a.color = c;
  });
  const own = hex(edge[3]);
  const apart = `#${hex(edge[0])}${hex(edge[1])}${hex(edge[2])}${hex(edge[3] * cross)}`;
  g.forEachEdge((_e, a, s, t) => {
    const k = home.get(s);
    const c = colors.get(k ?? "");
    if (k === undefined || k !== home.get(t)) a.color = apart;
    else if (c !== undefined) a.color = c + own;
  });
}

function communities(g: Net): Map<string, string> {
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

export function seed(data: GraphData, other: string): { g: Net; colors: Map<string, string>; names: Map<string, string> } {
  const g: Net = new Graph<Attrs>({ type: "undirected", multi: false, allowSelfLoops: false });
  const deg = new Array<number>(data.nodes.length).fill(0);
  for (const [a, b] of data.edges) {
    deg[a]++;
    deg[b]++;
  }
  data.nodes.forEach(([id, label, kind, group, t], i) => {
    const k = kind as Kind;
    g.mergeNode(id, {
      label: label || id,
      tag: "",
      kind: k,
      group,
      cluster: "",
      t,
      x: 0,
      y: 0,
      size: k === "project" ? 3 + Math.sqrt(deg[i]) * 0.5 : 1.2 + Math.sqrt(deg[i]) * 0.5,
      color: other,
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
  const top = [...sizes.entries()].filter(([c, n]) => c && n >= CLUSTER_MIN).sort((a, b) => b[1] - a[1] || (a[0] < b[0] ? -1 : 1));
  const colors = new Map<string, string>();
  const names = new Map<string, string>();
  const hubs = new Map<string, string>();
  g.forEachNode((n, a) => {
    const hub = hubs.get(a.cluster);
    if (hub === undefined || a.deg > g.getNodeAttribute(hub, "deg")) hubs.set(a.cluster, n);
  });
  top.forEach(([c], i) => {
    colors.set(c, PALETTE[i % PALETTE.length]);
    names.set(c, lab ? g.getNodeAttribute(hubs.get(c) as string, "label") : c);
  });
  g.forEachNode((_, a) => {
    a.color = colors.get(a.cluster) ?? other;
  });
  return { g, colors, names };
}

export function packs(g: Net, colors: Map<string, string>, names: Map<string, string>): Pack[] {
  const out = new Map<string, Pack & { lead: boolean }>();
  for (const [key, color] of colors) out.set(key, { key, name: names.get(key) ?? key, color, count: 0, hub: "", lead: false });
  g.forEachNode((n, a) => {
    const p = out.get(a.cluster);
    if (!p) return;
    p.count++;
    const lead = a.label === p.name;
    if (p.hub && ((p.lead && !lead) || (p.lead === lead && g.getNodeAttribute(p.hub, "deg") >= a.deg))) return;
    p.hub = n;
    p.lead = lead;
  });
  return [...out.values()];
}
