import Graph from "graphology";
import type { Attrs, Net } from "./graph-data";

export const NODE_FLAGS = { off: false, named: false, mute: false };
export const EDGE_FLAGS = { off: false };
export const GLOW_FLAGS = { hush: false };

export type Lit = Attrs & { pin: boolean };
export type Glow = Graph<Lit>;
export type Paint = {
  setNodesState(keys: string[], state: Partial<typeof NODE_FLAGS>): unknown;
  setEdgesState(keys: string[], state: Partial<typeof EDGE_FLAGS>): unknown;
};

export function spotlight(g: Net, gone: ReadonlySet<string>): { top: Glow; spot: (center: string | null, extra: string | null) => number } {
  const top: Glow = new Graph<Lit>({ type: g.type, multi: g.multi, allowSelfLoops: g.allowSelfLoops });
  const spot = (center: string | null, extra: string | null) => {
    if (top.order) top.clear();
    const add = (n: string, pin: boolean) => {
      if (!gone.has(n) && !top.hasNode(n)) top.addNode(n, { ...g.getNodeAttributes(n), pin });
    };
    if (center !== null) {
      add(center, true);
      g.forEachNeighbor(center, (m) => add(m, false));
      g.forEachEdge(center, (e, _a, s, t) => {
        if (top.hasNode(s) && top.hasNode(t)) top.addEdgeWithKey(e, s, t);
      });
    }
    if (extra !== null) add(extra, true);
    return top.order;
  };
  return { top, spot };
}

export function curtain(g: Net, paint: Paint): { gone: ReadonlySet<string>; sift: (shown: (a: Attrs) => boolean) => string[] } {
  const gone = new Set<string>();
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
    paint.setEdgesState(cut, { off: true });
    paint.setEdgesState(join, { off: false });
    return moved;
  };
  return { gone, sift };
}
