import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { build } from "esbuild";

globalThis.WebGL2RenderingContext = { UNSIGNED_BYTE: 5121 };

const entry = `
export { NEON, PALETTE, forget, packs, seed, swell, tint } from "./src/views/graph-data";
export { frame, idle } from "./src/views/graph-pace";
export { box, brief, nameless, spread } from "./src/views/graph-names";
export { elders, homes, kin, rings, scatter } from "./src/views/graph-place";
export { curtain, fit, spotlight } from "./src/views/graph-light";
export { LABEL_MIN, crownOptions, lift, rgba, stageOptions } from "./src/views/graph-style";
export { HALO, reach } from "./src/views/graph-glow";
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
const { g, colors, names } = m.seed(data, "#888888");
assert.equal(g.order, data.nodes.length);
assert.equal(g.size, 8);
assert.equal(g.getNodeAttribute("n6", "label"), "n6");
assert.equal(g.getNodeAttribute("n6", "deg"), 0);
assert.equal(g.getNodeAttribute("n6", "color"), "#888888");
assert.equal(g.getNodeAttribute("pA", "deg"), 4);
assert.equal(g.getNodeAttribute("n1", "color"), m.PALETTE[0]);
assert.equal(g.getNodeAttribute("n5", "color"), m.PALETTE[1]);
assert.deepEqual([...colors.keys()], ["alpha", "beta"]);
assert.deepEqual(m.packs(g, colors, names).map((p) => [p.key, p.name, p.color, p.count, p.hub]), [["alpha", "alpha", m.PALETTE[0], 5, "pA"], ["beta", "beta", m.PALETTE[1], 3, "pB"]]);
assert.ok(g.everyNode((_, a) => a.tag === ""));
assert.ok(g.getNodeAttribute("pA", "size") > g.getNodeAttribute("n3", "size"));

const mix = {
  nodes: [
    ["pA", "alpha", "project", "alpha", 0],
    ["a1", "a one", "note", "alpha", 0],
    ["a2", "a two", "note", "alpha", 0],
    ["a3", "a three", "note", "alpha", 0],
    ["pB", "beta", "project", "beta", 0],
    ["b1", "b one", "note", "beta", 0],
    ["b2", "b two", "note", "beta", 0],
    ["v", "voted", "note", "", 0],
    ["d1", "d one", "note", "duo", 0],
    ["d2", "d two", "note", "duo", 0],
  ],
  edges: [[1, 0], [2, 0], [3, 0], [5, 4], [6, 4], [7, 1], [7, 2], [3, 5], [8, 9]],
};
const tinted = m.seed(mix, "#888888");
const home = m.scatter(tinted.g);
assert.equal(home.get("v"), "alpha");
const edgeColor = (a, b) => tinted.g.getEdgeAttribute(tinted.g.edge(a, b), "color");
m.tint(tinted.g, home, tinted.colors, [1, 1, 1, 0.5], 1);
assert.equal(edgeColor("a3", "b1"), "#ffffff80");
m.tint(tinted.g, home, tinted.colors, [1, 1, 1, 0.5], 0.4);
assert.equal(edgeColor("a3", "b1"), "#ffffff33");
assert.equal(edgeColor("a1", "pA"), `${m.PALETTE[0]}80`);
assert.equal(edgeColor("v", "a1"), `${m.PALETTE[0]}80`);
assert.equal(edgeColor("b1", "pB"), `${m.PALETTE[1]}80`);
assert.equal(edgeColor("d1", "d2"), undefined);
assert.equal(tinted.g.getNodeAttribute("v", "color"), m.PALETTE[0]);
assert.equal(tinted.g.getNodeAttribute("v", "cluster"), "");
assert.equal(tinted.g.getNodeAttribute("d1", "color"), "#888888");
assert.deepEqual(m.packs(tinted.g, tinted.colors, tinted.names).map((p) => p.count), [4, 3]);

const plates = [
  { n: "a", x: 0, y: 0, r: 2, text: "alpha" },
  { n: "b", x: 10, y: 2, r: 2, text: "beta" },
  { n: "c", x: 0, y: 40, r: 2, text: "gamma" },
  { n: "d", x: -60, y: 0, r: 2, text: "delta" },
];
const room = (list, x, y) => list.map((s) => ({ ...s, x: s.x + x, y: s.y + y }));
assert.deepEqual(m.spread(room(plates, 100, 100), 13, 400, 300), ["a", "c", "d"]);
assert.deepEqual(m.spread(room(plates, 100, 100), 13, 130, 300), ["d"]);
assert.deepEqual(m.spread(room(plates, 100, 100), 13, 400, 130), ["a", "d"]);
assert.deepEqual(m.spread(room(plates, 0, 20), 13, 400, 300), ["b"]);
assert.deepEqual(m.spread([], 13, 400, 300), []);
const huddle = [{ n: "hub", x: 0, y: 0, r: 25, text: "hub" }, { n: "above", x: 10, y: -24, r: 2, text: "above" }, { n: "row", x: 0, y: 40, r: 2, text: "row" }, { n: "next", x: 0, y: 60, r: 2, text: "next" }];
assert.deepEqual(m.spread(room(huddle, 100, 100), 13, 400, 300), ["hub", "row"]);

assert.equal(m.fit([]), m.fit([0]));
assert.ok(m.fit([0.001]) > 0);
assert.equal(m.fit([0.5, 0.6, 2]), 1);
const near = Array.from({ length: 10 }, (_, i) => 0.01 * (i + 1));
assert.ok(m.fit(near) > m.fit(near.slice(0, 3)));
assert.equal(m.fit([...near, 5]), m.fit([...near.slice(0, 9), 5, 0.1]));
assert.ok(m.fit([...near, 5]) < 1);

const stored = { "wisply.graph.v4:all": "{}", "wisply.graph.v3:all": "{}", "wisply.theme": "dark" };
Object.defineProperty(globalThis, "localStorage", { configurable: true, value: { ...stored, removeItem(k) { delete this[k]; } } });
m.forget();
assert.deepEqual(Object.keys(globalThis.localStorage).filter((k) => k !== "removeItem"), ["wisply.theme"]);
Object.defineProperty(globalThis, "localStorage", { configurable: true, get() { throw new Error("blocked"); } });
m.forget();
delete globalThis.localStorage;

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
assert.deepEqual([...filter.cut].sort(), [edge("n1", "pA"), edge("n2", "pA"), edge("n3", "pA"), edge("s1", "pA"), edge("n4", "pB"), edge("n5", "pB")].sort());

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
assert.equal(filter.cut.size, 0);

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
const crowd = m.seed(big, "#888888");
const wide = crowd.g;
assert.equal(crowd.colors.size, HUBS);
const shades = [...crowd.colors.values()];
assert.equal(shades[m.PALETTE.length], shades[0]);
assert.equal(new Set(shades.slice(0, m.PALETTE.length)).size, m.PALETTE.length);
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
assert.deepEqual(lamp.top.filterNodes((_, a) => a.plate), ["h7"]);
assert.equal(lamp.top.getNodeAttribute("h7", "pin"), true);
assert.equal(lamp.ranked().length, LEAVES + 1);
assert.equal(lamp.ranked()[0], "h7");
assert.equal(lamp.spot("h7", "h3"), LEAVES + 2);
assert.deepEqual(lamp.ranked().slice(0, 2), ["h7", "h3"]);
assert.ok(lamp.ranked().slice(2).every((n, i, all) => i === 0 || wide.getNodeAttribute(all[i - 1], "deg") >= wide.getNodeAttribute(n, "deg")));
assert.equal(lamp.spot("h2", null), 1);
const long = "a very long note title that would cover half of the window";
assert.equal(m.brief(long).length, 30);
assert.ok(m.brief(long).endsWith("…"));
assert.equal(m.brief(" short "), "short");

const tones = { label: "#fff", dim: "rgba(0,0,0,0.5)", edge: "rgba(255,255,255,0.1)", edgeOn: "#fff", back: "#000", backLine: "#333", shadow: "#000" };
for (const options of [m.stageOptions, m.crownOptions]) {
  for (const k of [1, m.HALO, 120]) {
    const o = options(tones, k);
    assert.notEqual(m.analyzeStyleDeclaration(o.styles.nodes).dependency, "graph-state");
    assert.notEqual(m.analyzeStyleDeclaration(o.styles.edges).dependency, "graph-state");
    for (const slow of ["nodeReducer", "edgeReducer", "hideEdgesOnMove", "hideLabelsOnMove"]) assert.ok(!(slow in o.settings), slow);
    assert.equal(o.settings.labelRenderedSizeThreshold, m.LABEL_MIN * k);
    assert.equal("nodes" in o.primitives, k > 1);
    const sized = o.styles.nodes.filter((s) => s.size && s.size.attribute === "bulk");
    assert.equal(sized.length, k > 1 ? 1 : 0);
    if (k > 1) assert.equal(sized[0].size.defaultValue, k);
    for (const rule of [...o.styles.nodes, ...o.styles.edges]) for (const v of Object.values(rule)) assert.notEqual(typeof v, "function");
  }
}

assert.equal(new Set(m.NEON).size, m.NEON.length);
assert.ok(m.NEON.every((c) => /^#[0-9a-f]{6}$/.test(c)));
assert.equal(m.NEON.length, m.PALETTE.length);
const bright = m.seed(data, "#888888", m.NEON);
assert.equal(bright.g.getNodeAttribute("n1", "color"), m.NEON[0]);
assert.equal(bright.g.getNodeAttribute("n5", "color"), m.NEON[1]);
assert.equal(m.seed(data, "#888888").g.getNodeAttribute("n1", "color"), m.PALETTE[0]);
const flat = m.seed(data, "#888888").g;
flat.forEachNode((_, a) => assert.equal(a.bulk, a.size));
const swollen = m.seed(data, "#888888", m.NEON).g;
const grown = m.swell(swollen, 600);
assert.ok(grown >= m.HALO);
flat.forEachNode((n, a) => {
  assert.ok(Math.abs(swollen.getNodeAttribute(n, "bulk") - a.size * grown) < 1e-9);
  assert.equal(swollen.getNodeAttribute(n, "size"), a.size);
});
assert.equal(m.swell(m.seed({ nodes: [], edges: [] }, "#888888").g, 600), 1);
const dense = m.seed(big, "#888888").g;
m.scatter(dense);
const swelled = m.swell(dense, 600);
assert.ok(swelled > m.HALO);
assert.ok(swelled <= 400);
assert.equal(m.reach(1, 600), m.HALO);
assert.equal(m.reach(1e12, 600), 400);
assert.ok(m.reach(10000, 600) > m.reach(5000, 600));
assert.ok(m.reach(10000, 1200) < m.reach(10000, 600));

assert.deepEqual(m.rgba("#fff"), [1, 1, 1, 1]);
assert.deepEqual(m.rgba("#ff000080").map((v) => Math.round(v * 100)), [100, 0, 0, 50]);
assert.deepEqual(m.rgba("rgba(255, 0, 51, 0.25)").map((v) => Math.round(v * 100)), [100, 0, 20, 25]);
assert.deepEqual(m.rgba(" rgb(0 128 255 / 50%) ").map((v) => Math.round(v * 100)), [0, 50, 100, 50]);
assert.equal(m.lift([1, 1, 1, 1]), 0);
assert.equal(m.lift([1, 1, 1, 0.5]), 1);
assert.equal(m.lift([1, 1, 1, 0.12]), 4);
assert.equal(m.lift([1, 1, 1, 0]), 4);

const far = (net, n) => Math.hypot(net.getNodeAttribute(n, "x"), net.getNodeAttribute(n, "y"));
const spots = (net) => net.mapNodes((_, a) => [a.x, a.y]);
const closest = (net) => {
  const list = net.mapNodes((_, a) => a);
  let least = Infinity;
  for (let i = 0; i < list.length; i++) for (let j = i + 1; j < list.length; j++) least = Math.min(least, Math.hypot(list[i].x - list[j].x, list[i].y - list[j].y));
  return least;
};
const small = m.seed(data, "#888888").g;
m.scatter(small);
const laid = small.filterNodes((n) => small.degree(n) > 0);
assert.equal(laid.length, small.order - 1);
assert.ok(far(small, "n6") > Math.max(...laid.map((n) => far(small, n))));
assert.ok(closest(small) > 3);

let t1 = performance.now();
m.scatter(wide);
const scatterMs = performance.now() - t1;
const first = spots(wide);
m.scatter(wide);
assert.deepEqual(spots(wide), first);
const again = m.seed(big, "#888888").g;
m.scatter(again);
assert.deepEqual(spots(again), first);
const homesOf = Array.from({ length: HUBS }, (_, i) => [wide.getNodeAttribute(`h${i}`, "x"), wide.getNodeAttribute(`h${i}`, "y")]);
let strays = 0;
for (let i = 0; i < HUBS; i++)
  for (let j = 0; j < LEAVES; j += 7) {
    const a = wide.getNodeAttributes(`h${i}l${j}`);
    const own = Math.hypot(a.x - homesOf[i][0], a.y - homesOf[i][1]);
    if (homesOf.some(([x, y], k) => k !== i && Math.hypot(a.x - x, a.y - y) < own)) strays++;
  }
assert.equal(strays, 0);

const clashes = (list) => {
  let n = 0;
  for (let i = 0; i < list.length; i++) for (let j = i + 1; j < list.length; j++) if (Math.hypot(list[i].x - list[j].x, list[i].y - list[j].y) < list[i].r + list[j].r - 1e-6) n++;
  return n;
};
const discs = Array.from({ length: 300 }, (_, i) => ({ r: 1 + ((i * 37) % 50), x: 0, y: 0 }));
const rim = m.rings(discs);
assert.ok(discs.every((d) => Math.hypot(d.x, d.y) + d.r <= rim + 1e-6));
assert.equal(clashes(discs), 0);
const held = Array.from({ length: 40 }, (_, i) => ({ r: 1 + ((i * 13) % 9), x: 0, y: 0 }));
const outer = m.rings(held, 25);
assert.ok(held.every((d) => Math.hypot(d.x, d.y) - d.r >= 25 - 1e-6 && Math.hypot(d.x, d.y) + d.r <= outer + 1e-6));
assert.equal(clashes(held), 0);

const TEAMS = 12;
const town = { nodes: [], edges: [] };
const put = (id, kind, group) => town.nodes.push([id, id, kind, group, 0]) - 1;
const heads = [];
const rest = [];
for (let i = 0; i < TEAMS; i++) {
  heads.push(put(`t${i}`, "project", `team ${i}`));
  const size = i === 0 ? 60 : i === TEAMS - 1 ? 12 : 30;
  for (let j = 0; j < size; j++) {
    const leaf = put(`t${i}m${j}`, "note", `team ${i}`);
    town.edges.push([leaf, heads[i]]);
    if (i === TEAMS - 1) rest.push(leaf);
  }
}
const talk = put("talk", "session", "team 0");
for (let j = 0; j < 20; j++) town.edges.push([talk, heads[0] + 1 + j]);
for (let j = 0; j < 8; j++) town.edges.push([talk, rest[j]]);
const hall = put("index", "note", "");
for (const h of heads) town.edges.push([hall, h]);
const city = m.seed(town, "#888888").g;
const { groups: crews, home: homeOf } = m.homes(city);
assert.equal(homeOf.get("index"), "team 0");
assert.deepEqual(m.kin(city, homeOf, crews).map((c) => [...c].sort()).filter((c) => c.length > 1), [["team 0", "team 11"]]);
const kids = m.elders(city, homeOf, crews);
assert.deepEqual([...kids.get("talk")].sort(), Array.from({ length: 20 }, (_, j) => `t0m${j}`).sort());
const up = new Map();
for (const [p, list] of kids)
  for (const c of list) {
    assert.ok(!up.has(c), c);
    up.set(c, p);
  }
for (const list of crews.values())
  for (const n of list.slice(1)) {
    let at = n;
    for (let step = 0; step < list.length && at !== list[0]; step++) at = up.get(at);
    assert.equal(at, list[0], n);
  }
m.scatter(city);
const seat = (n) => [city.getNodeAttribute(n, "x"), city.getNodeAttribute(n, "y")];
const apart = (a, b) => Math.hypot(seat(a)[0] - seat(b)[0], seat(a)[1] - seat(b)[1]);
const smallest = `t${TEAMS - 1}`;
for (let i = 1; i < TEAMS - 1; i++) assert.ok(apart("t0", smallest) < apart("t0", `t${i}`));
const middle = [0, 1].map((k) => heads.reduce((sum, _, i) => sum + seat(`t${i}`)[k], 0) / TEAMS);
assert.ok(Math.hypot(seat("index")[0] - middle[0], seat("index")[1] - middle[1]) < 1e-6);
assert.ok(apart("talk", "t0") < apart("talk", smallest));

const pairs = { nodes: [], edges: [] };
const team = (key, size) => {
  const at = pairs.nodes.push([key, key, "project", key, 0]) - 1;
  for (let j = 0; j < size; j++) pairs.edges.push([pairs.nodes.push([`${key}${j}`, `${key} ${j}`, "note", key, 0]) - 1, at]);
  return at;
};
const [ta, tb, tc, td] = [team("A", 9), team("B", 1), team("C", 1), team("D", 9)];
pairs.edges.push([tb, ta + 1], [tb, ta + 2], [tc, ta + 3], [tc, td + 1]);
const duo = m.seed(pairs, "#888888").g;
const split = m.homes(duo);
assert.deepEqual(m.kin(duo, split.home, split.groups).map((c) => [...c].sort()).sort(), [["A", "B"], ["C"], ["D"]]);

const SESSIONS = 6;
const NOTES = 12;
const solo = { nodes: [["p", "solo", "project", "solo", 0]], edges: [] };
for (let i = 0; i < SESSIONS; i++) {
  const s = solo.nodes.push([`s${i}`, `talk ${i}`, "session", "solo", 0]) - 1;
  solo.edges.push([s, 0]);
  for (let j = 0; j < NOTES; j++) {
    const n = solo.nodes.push([`s${i}n${j}`, `note ${i} ${j}`, "note", "solo", 0]) - 1;
    solo.edges.push([n, 0], [n, s]);
  }
}
for (let j = 0; j < 20; j++) solo.edges.push([solo.nodes.push([`d${j}`, `direct ${j}`, "note", "solo", 0]) - 1, 0]);
const one = m.seed(solo, "#888888").g;
m.scatter(one);
const gap = (a, b) => Math.hypot(one.getNodeAttribute(a, "x") - one.getNodeAttribute(b, "x"), one.getNodeAttribute(a, "y") - one.getNodeAttribute(b, "y"));
for (let i = 0; i < SESSIONS; i++)
  for (let j = 0; j < NOTES; j++)
    for (let k = 0; k < SESSIONS; k++) if (k !== i) assert.ok(gap(`s${i}n${j}`, `s${i}`) < gap(`s${i}n${j}`, `s${k}`), `s${i}n${j}`);
assert.ok(closest(one) > 3);

const whole = m.box([{ x: -1e6, y: -1e6 }, { x: 1e6, y: 1e6 }]);
const hub = wide.getNodeAttribute("h0", "size");
assert.deepEqual(m.nameless(wide, whole, hub, new Set(), 500).length, HUBS);
assert.deepEqual(m.nameless(wide, whole, hub, new Set(["h3"]), 500).length, HUBS - 1);
assert.equal(m.nameless(wide, whole, 0, new Set(), 50).length, 50);
assert.ok(m.nameless(wide, whole, 0, new Set(), 50).every((n) => !n.includes("l")));
wide.setNodeAttribute("h0", "tag", "hub 0");
assert.ok(!m.nameless(wide, whole, hub, new Set(), 500).includes("h0"));
const corner = m.box([{ x: homesOf[5][0] - 0.5, y: homesOf[5][1] - 0.5 }, { x: homesOf[5][0] + 0.5, y: homesOf[5][1] + 0.5 }]);
assert.deepEqual(m.nameless(wide, corner, hub, new Set(), 500), ["h5"]);

const queue = [];
globalThis.requestAnimationFrame = (f) => queue.push(f);
let painted = 0;
const paintOnce = m.frame(() => painted++);
paintOnce();
paintOnce();
assert.equal(queue.length, 1);
queue.shift()();
paintOnce();
assert.equal(queue.length, 1);
queue.shift()();
assert.equal(painted, 2);
delete globalThis.requestAnimationFrame;

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

console.log(`graph ok (${wide.order} nodes: filter ${siftMs.toFixed(1)} ms, focus ${spotMs.toFixed(1)} ms, layout ${scatterMs.toFixed(1)} ms)`);
