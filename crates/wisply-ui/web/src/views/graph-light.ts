import Graph from "graphology";
import type { Attrs, Net } from "./graph-data";
import { brief } from "./graph-names";

const RANK = 240;
const FIT_SHARE = 0.9;
const FIT_ROOM = 2.4;
const FIT_NEAR = 0.05;
const FIT_FAR = 1;

export const NODE_FLAGS = { off: false, named: false, mute: false };
export const EDGE_FLAGS = { off: false };
export const GLOW_FLAGS = { hush: false };

export type Lit = Attrs & { pin: boolean; plate: boolean };
export type Glow = Graph<Lit>;
export type Paint = {
  setNodesState(keys: string[], state: Partial<typeof NODE_FLAGS>): unknown;
  setEdgesState(keys: string[], state: Partial<typeof EDGE_FLAGS>): unknown;
};

export function spotlight(g: Net, gone: ReadonlySet<string>): { top: Glow; spot: (center: string | null, extra: string | null) => number; ranked: () => string[] } {
  const top: Glow = new Graph<Lit>({ type: g.type, multi: g.multi, allowSelfLoops: g.allowSelfLoops });
  let order: string[] = [];
  const spot = (center: string | null, extra: string | null) => {
    if (top.order) top.clear();
    order = [];
    const add = (n: string, pin: boolean) => {
      if (gone.has(n) || top.hasNode(n)) return;
      const a = g.getNodeAttributes(n);
      top.addNode(n, { ...a, tag: brief(a.label), pin, plate: pin });
      if (pin) order.unshift(n);
    };
    if (extra !== null) add(extra, true);
    if (center !== null) {
      add(center, true);
      const near = g.filterNeighbors(center, (m) => !gone.has(m));
      near.sort((x, y) => g.getNodeAttribute(y, "deg") - g.getNodeAttribute(x, "deg") || (x < y ? -1 : 1));
      for (const m of near) add(m, false);
      order.push(...near.filter((m) => !order.includes(m)).slice(0, RANK));
      g.forEachEdge(center, (e, _a, s, t) => {
        if (top.hasNode(s) && top.hasNode(t)) top.addEdgeWithKey(e, s, t);
      });
    }
    return top.order;
  };
  return { top, spot, ranked: () => order };
}

export function fit(reach: number[]): number {
  const sorted = [...reach].sort((a, b) => a - b);
  const far = sorted[Math.floor(sorted.length * FIT_SHARE)] ?? sorted[sorted.length - 1] ?? 0;
  return Math.min(FIT_FAR, Math.max(FIT_NEAR, far * FIT_ROOM));
}

export function curtain(g: Net, paint: Paint): { gone: ReadonlySet<string>; cut: ReadonlySet<string>; sift: (shown: (a: Attrs) => boolean) => string[] } {
  const gone = new Set<string>();
  const lost = new Set<string>();
  const sift = (shown: (a: Attrs) => boolean) => {
    const hide: string[] = [];
    const show: string[] = [];
    g.forEachNode((n, a) => {
      const off = !shown(a);
      if (off === gone.has(n)) return;
      if (off) {
        gone.add(n);
        hide.push(n);
      } else {
        gone.delete(n);
        show.push(n);
      }
    });
    const moved = hide.concat(show);
    if (!moved.length) return moved;
    paint.setNodesState(hide, { off: true });
    paint.setNodesState(show, { off: false });
    const seen = new Set<string>();
    const cut: string[] = [];
    const join: string[] = [];
    for (const n of moved)
      g.forEachEdge(n, (e, _a, s, t) => {
        if (seen.has(e)) return;
        seen.add(e);
        (gone.has(s) || gone.has(t) ? cut : join).push(e);
      });
    for (const e of cut) lost.add(e);
    for (const e of join) lost.delete(e);
    paint.setEdgesState(cut, { off: true });
    paint.setEdgesState(join, { off: false });
    return moved;
  };
  return { gone, cut: lost, sift };
}
