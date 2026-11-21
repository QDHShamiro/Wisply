import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { build } from "esbuild";

const entry = `
export { PALETTE, place, seed } from "./src/views/graph-data";
export { idle, matrices, plan, settle } from "./src/views/graph-layout";
export { curtain, spotlight } from "./src/views/graph-light";
export { crownOptions, lift, rgba, stageOptions } from "./src/views/graph-style";
export { analyzeStyleDeclaration } from "sigma/types";
`;
const out = await build({ stdin: { contents: entry, resolveDir: fileURLToPath(new URL("..", import.meta.url)), loader: "ts" }, bundle: true, format: "esm", write: false, logLevel: "silent" });
const m = await import("data:text/javascript;base64," + Buffer.from(out.outputFiles[0].text).toString("base64"));

const data = {
  nodes: [
    ["pA", "alpha", "project", "alpha", 0],
    ["n1", "one", "note", "alpha", 100],
    ["n2", "two", "note", "alpha", 200],
    ["n3", "three", "note", "alpha", 300],
    ["s1", "talk", "session", "alpha", 250],
    ["pB", "beta", "project", "beta", 0],
    ["n4", "four", "note", "beta", 400],
    ["n5", "five", "note", "beta", 500],
    ["n6", "", "note", "", 600],
  ],
  edges: [[1, 0], [2, 0], [3, 0], [4, 0], [1, 2], [2, 1], [6, 5], [7, 5], [3, 3], [4, 6]],
};
const { g, colors } = m.seed(data, "#888888");
assert.equal(g.order, data.nodes.length);
assert.equal(g.size, 8);
assert.equal(g.getNodeAttribute("n6", "label"), "n6");
assert.equal(g.getNodeAttribute("n6", "deg"), 0);
assert.equal(g.getNodeAttribute("n6", "color"), "#888888");
assert.equal(g.getNodeAttribute("pA", "deg"), 4);
assert.equal(g.getNodeAttribute("n1", "color"), m.PALETTE[0]);
assert.equal(g.getNodeAttribute("n5", "color"), m.PALETTE[1]);
assert.deepEqual([...colors.keys()], ["alpha", "beta"]);
assert.ok(g.everyNode((_, a) => a.tag === ""));
assert.ok(g.getNodeAttribute("pA", "size") > g.getNodeAttribute("n3", "size"));

assert.equal(m.place(g, { pA: [1, 2], n1: [3, 4] }), data.nodes.length - 2);
assert.deepEqual([g.getNodeAttribute("pA", "x"), g.getNodeAttribute("pA", "y")], [1, 2]);
assert.ok(Math.abs(g.getNodeAttribute("n2", "x") - 2) <= 2);

const calls = [];
const paint = {
  setNodesState: (keys, state) => calls.push(["node", [...keys].sort(), state]),
  setEdgesState: (keys, state) => calls.push(["edge", [...keys].sort(), state]),
};
const asked = (kind, off) => calls.filter((c) => c[0] === kind && c[2].off === off).flatMap((c) => c[1]);
const edge = (a, b) => g.edge(a, b);
const filter = m.curtain(g, paint);

assert.deepEqual(filter.sift((a) => a.deg > 0), ["n6"]);
assert.deepEqual(asked("node", true), ["n6"]);
assert.deepEqual(asked("edge", true), []);
assert.deepEqual([...filter.gone], ["n6"]);

calls.length = 0;
assert.deepEqual(filter.sift((a) => a.deg > 0), []);
assert.equal(calls.length, 0);

assert.deepEqual(filter.sift((a) => a.deg > 0 && a.kind === "note").sort(), ["pA", "pB", "s1"]);
assert.deepEqual(asked("edge", true), [edge("n1", "pA"), edge("n2", "pA"), edge("n3", "pA"), edge("s1", "pA"), edge("n4", "pB"), edge("n5", "pB"), edge("s1", "n4")].sort());
assert.deepEqual(asked("edge", false), []);

calls.length = 0;
assert.deepEqual(filter.sift((a) => a.deg > 0 && a.kind !== "project"), ["s1"]);
assert.deepEqual(asked("node", false), ["s1"]);
assert.deepEqual(asked("edge", false), [edge("s1", "n4")]);
assert.deepEqual(asked("edge", true), [edge("s1", "pA")]);

const { top, spot } = m.spotlight(g, filter.gone);
assert.equal(spot("n1", null), 2);
assert.deepEqual(top.nodes().sort(), ["n1", "n2"]);
assert.deepEqual(top.edges(), [edge("n1", "n2")]);
assert.equal(top.getNodeAttribute("n1", "pin"), true);
assert.equal(top.getNodeAttribute("n2", "pin"), false);
assert.equal(top.getNodeAttribute("n2", "label"), "two");
assert.equal(spot("n1", "n5"), 3);
assert.equal(top.getNodeAttribute("n5", "pin"), true);
assert.equal(top.degree("n5"), 0);
assert.equal(spot("n1", "n6"), 2);
assert.equal(spot(null, null), 0);
assert.equal(top.size, 0);
filter.sift(() => true);
assert.equal(spot("pA", null), 5);
assert.equal(top.size, 4);

const HUBS = 200;
const LEAVES = 99;
const big = { nodes: [], edges: [] };
for (let i = 0; i < HUBS; i++) {
  const at = big.nodes.length;
  big.nodes.push([`h${i}`, `hub ${i}`, "project", `hub ${i}`, 0]);
  for (let j = 0; j < LEAVES; j++) {
    big.edges.push([big.nodes.length, at]);
    big.nodes.push([`h${i}l${j}`, `leaf ${i} ${j}`, j % 9 ? "note" : "session", `hub ${i}`, 1000 + i * LEAVES + j]);
  }
}
const wide = m.seed(big, "#888888").g;
assert.equal(wide.order, HUBS * (LEAVES + 1));
assert.equal(wide.size, HUBS * LEAVES);
let touched = 0;
const count = { setNodesState: (keys) => (touched += keys.length), setEdgesState: (keys) => (touched += keys.length) };
const sieve = m.curtain(wide, count);
const cut = 1000 + 5 * LEAVES;
let t = performance.now();
assert.equal(sieve.sift((a) => a.t === 0 || a.t >= cut).length, 5 * LEAVES);
const siftMs = performance.now() - t;
assert.equal(touched, 5 * LEAVES * 2);
const lamp = m.spotlight(wide, sieve.gone);
t = performance.now();
assert.equal(lamp.spot("h7", null), LEAVES + 1);
const spotMs = performance.now() - t;
assert.equal(lamp.spot("h2", null), 1);

for (const options of [m.stageOptions, m.crownOptions]) {
  const o = options({ label: "#fff", dim: "rgba(0,0,0,0.5)", edge: "rgba(255,255,255,0.1)", edgeOn: "#fff", back: "#000", backLine: "#333", shadow: "#000" });
  assert.notEqual(m.analyzeStyleDeclaration(o.styles.nodes).dependency, "graph-state");
  assert.notEqual(m.analyzeStyleDeclaration(o.styles.edges).dependency, "graph-state");
  for (const slow of ["nodeReducer", "edgeReducer", "hideEdgesOnMove", "hideLabelsOnMove"]) assert.ok(!(slow in o.settings), slow);
}

assert.deepEqual(m.rgba("#fff"), [1, 1, 1, 1]);
assert.deepEqual(m.rgba("#ff000080").map((v) => Math.round(v * 100)), [100, 0, 0, 50]);
assert.deepEqual(m.rgba("rgba(255, 0, 51, 0.25)").map((v) => Math.round(v * 100)), [100, 0, 20, 25]);
assert.deepEqual(m.rgba(" rgb(0 128 255 / 50%) ").map((v) => Math.round(v * 100)), [0, 50, 100, 50]);
assert.equal(m.lift([1, 1, 1, 1]), 0);
assert.equal(m.lift([1, 1, 1, 0.5]), 1);
assert.equal(m.lift([1, 1, 1, 0.12]), 4);
assert.equal(m.lift([1, 1, 1, 0]), 4);

const cells = m.matrices(g);
assert.equal(cells.nodes.length, g.order * 10);
assert.equal(cells.edges.length, g.size * 3);
const at = g.nodes().indexOf("pA") * 10;
assert.deepEqual([cells.nodes[at], cells.nodes[at + 1], cells.nodes[at + 6]], [1, 2, 1 + g.degree("pA")]);
cells.nodes[at] = 40;
cells.nodes[at + 1] = -7;
m.settle(g, cells.nodes);
assert.deepEqual([g.getNodeAttribute("pA", "x"), g.getNodeAttribute("pA", "y")], [40, -7]);

assert.equal(m.plan(400, 400).settings.barnesHutOptimize, false);
assert.equal(m.plan(501, 501).settings.barnesHutOptimize, true);
assert.ok(m.plan(400, 3).rounds < m.plan(400, 400).rounds);
assert.equal(m.plan(20000, 20000).rounds, Infinity);
assert.ok(m.plan(20000, 5).ms < m.plan(20000, 20000).ms);

assert.equal(typeof globalThis.requestIdleCallback, "undefined");
let woke = 0;
m.idle(() => woke++, 1);
m.idle(() => (woke += 10), 1)();
await new Promise((r) => setTimeout(r, 30));
assert.equal(woke, 1);
const idled = [];
globalThis.requestIdleCallback = (_run, o) => idled.push(["ask", o.timeout]) + 6;
globalThis.cancelIdleCallback = (id) => idled.push(["drop", id]);
m.idle(() => {}, 50)();
delete globalThis.requestIdleCallback;
delete globalThis.cancelIdleCallback;
assert.deepEqual(idled, [["ask", 50], ["drop", 7]]);

console.log(`graph ok (${wide.order} nodes: filter ${siftMs.toFixed(1)} ms, focus ${spotMs.toFixed(1)} ms)`);
