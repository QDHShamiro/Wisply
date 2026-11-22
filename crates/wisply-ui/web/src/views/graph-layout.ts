import run from "graphology-layout-forceatlas2/webworker.js";
import { BIG, type Net } from "./graph-data";

const NODE_CELLS = 10;
const EDGE_CELLS = 3;
const TREE_MIN = 500;
const FULL_ROUNDS = 400;
const NUDGE_ROUNDS = 60;
const FULL_MS = 8000;
const NUDGE_MS = 900;
const FRAME_MS = 16;
const REST = 4;
const BASE = { linLogMode: false, outboundAttractionDistribution: false, adjustSizes: false, edgeWeightInfluence: 1, strongGravityMode: true, slowDown: 2, barnesHutTheta: 0.5 };

export type Plan = { settings: Record<string, number | boolean>; rounds: number; ms: number };

export function plan(order: number, fresh: number): Plan {
  const all = fresh >= order;
  if (order <= BIG) return { settings: { ...BASE, scalingRatio: 14, gravity: 0.6, barnesHutOptimize: order > TREE_MIN }, rounds: all ? FULL_ROUNDS : NUDGE_ROUNDS, ms: FULL_MS };
  return { settings: { ...BASE, scalingRatio: 10, gravity: 0.05, barnesHutOptimize: true }, rounds: Infinity, ms: all ? Math.min(FULL_MS, 1500 + order / 3) : NUDGE_MS };
}

export function matrices(g: Net): { nodes: Float32Array; edges: Float32Array } {
  const nodes = new Float32Array(g.order * NODE_CELLS);
  const edges = new Float32Array(g.size * EDGE_CELLS);
  const at = new Map<string, number>();
  let j = 0;
  g.forEachNode((n, a) => {
    at.set(n, j);
    nodes[j] = a.x;
    nodes[j + 1] = a.y;
    nodes[j + 6] = 1;
    nodes[j + 7] = 1;
    nodes[j + 8] = a.size || 1;
    j += NODE_CELLS;
  });
  j = 0;
  g.forEachEdge((_e, _a, s, t) => {
    const sj = at.get(s) as number;
    const tj = at.get(t) as number;
    nodes[sj + 6]++;
    nodes[tj + 6]++;
    edges[j] = sj;
    edges[j + 1] = tj;
    edges[j + 2] = 1;
    j += EDGE_CELLS;
  });
  return { nodes, edges };
}

export function settle(g: Net, m: Float32Array): void {
  let i = 0;
  g.forEachNode((_n, a) => {
    a.x = m[i];
    a.y = m[i + 1];
    i += NODE_CELLS;
  });
}

export function arrange(g: Net, p: Plan, show: () => void, done: () => void): () => void {
  const { nodes, edges } = matrices(g);
  const url = URL.createObjectURL(new Blob([`(${run.toString()}).call(this);`], { type: "text/javascript" }));
  const worker = new Worker(url);
  const free = () => URL.revokeObjectURL(url);
  worker.addEventListener("message", free, { once: true });
  const end = performance.now() + p.ms;
  let round = 0;
  let next = 0;
  worker.onmessage = (e: MessageEvent<{ nodes: ArrayBuffer }>) => {
    const m = new Float32Array(e.data.nodes);
    const now = performance.now();
    round++;
    const last = round >= p.rounds || now >= end;
    const due = last || now >= next;
    if (due) settle(g, m);
    if (last) worker.terminate();
    else worker.postMessage({ settings: p.settings, nodes: m.buffer }, [m.buffer]);
    if (due) {
      show();
      const after = performance.now();
      next = after + Math.max(FRAME_MS, (after - now) * REST);
    }
    if (last) done();
  };
  worker.postMessage({ settings: p.settings, nodes: nodes.buffer, edges: edges.buffer }, [nodes.buffer, edges.buffer]);
  return () => {
    free();
    worker.terminate();
  };
}

export function idle(run: () => void, ms: number): () => void {
  if (typeof requestIdleCallback === "function") {
    const id = requestIdleCallback(run, { timeout: ms });
    return () => cancelIdleCallback(id);
  }
  const id = setTimeout(run, ms);
  return () => clearTimeout(id);
}
