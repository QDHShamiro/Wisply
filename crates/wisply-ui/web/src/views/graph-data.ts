import Graph from "graphology";
import type { GraphData } from "../api";

export const PALETTE = ["#4f7cff", "#ff3d8b", "#12c99b", "#ff9f1a", "#a259ff", "#ff5c39", "#1bb8f0", "#8bc34a", "#e040fb", "#00bfa5"];
export const KINDS = ["note", "project", "session"] as const;
export const BIG = 3000;
const STORE = "wisply.graph.v3:";
const CLUSTER_MIN = 3;
const ROUNDS = 15;

export type Kind = (typeof KINDS)[number];
export type Attrs = { label: string; tag: string; kind: Kind; group: string; cluster: string; t: number; x: number; y: number; size: number; color: string; deg: number };
export type Net = Graph<Attrs>;
export type Spots = Record<string, [number, number]>;

export function loadSpots(key: string): Spots {
  try {
    return JSON.parse(localStorage.getItem(STORE + key) ?? "{}") as Spots;
  } catch {
    return {};
  }
}

export function saveSpots(key: string, g: Net): void {
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

export function place(g: Net, spots: Spots): number {
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
      x: (Math.random() - 0.5) * 100,
      y: (Math.random() - 0.5) * 100,
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
    a.color = colors.get(a.cluster) ?? other;
  });
  return { g, colors, names };
}
