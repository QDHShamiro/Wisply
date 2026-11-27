import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { build } from "esbuild";

globalThis.WebGL2RenderingContext = { UNSIGNED_BYTE: 5121 };
globalThis.window = { matchMedia: () => ({ matches: false }), devicePixelRatio: 2 };

const entry = `
export { HALO, TAU, clock, halo } from "./src/views/graph-glow";
export { sky } from "./src/views/graph-sky";
export { motion } from "./src/views/graph-motion";
`;
const out = await build({ stdin: { contents: entry, resolveDir: fileURLToPath(new URL("..", import.meta.url)), loader: "ts" }, bundle: true, format: "esm", write: false, logLevel: "silent" });
const m = await import("data:text/javascript;base64," + Buffer.from(out.outputFiles[0].text).toString("base64"));

const layer = m.halo();
assert.equal(layer.name, "halo");
assert.deepEqual(layer.uniforms, []);
assert.equal(layer.attributes.length, 1);
assert.equal(layer.attributes[0].source, "color");
assert.equal(layer.attributes[0].type, 5121);
assert.ok(layer.glsl.includes("vec4 layer_halo(vec4 v_fillColor)"));
assert.ok(layer.glsl.includes("uniform float u_phase;"));
assert.ok(!layer.glsl.includes("//") && !layer.glsl.includes("/*"));
assert.ok(layer.glsl.includes(`#ifdef PICKING_MODE`));
assert.ok(!/NaN|undefined|Infinity/.test(layer.glsl));
assert.ok(layer.glsl.includes(`, ${1 / m.HALO}, d)`));
const sent = [];
const hooks = layer.lifecycle({ gl: { uniform1f: (at, v) => sent.push([at, v]) }, getUniformLocation: (n) => (n === "u_phase" ? "loc" : null) });
m.clock.phase = 1.25;
hooks.beforeRender();
m.clock.phase = 2.5;
hooks.beforeRender();
assert.deepEqual(sent, [["loc", 1.25], ["loc", 2.5]]);
const lost = m.halo().lifecycle({ gl: { uniform1f: () => assert.fail("no location") }, getUniformLocation: () => null });
lost.beforeRender();

const calls = [];
const pen = new Proxy({}, { get: (_t, k) => (...a) => calls.push([k, ...a]), set: (_t, k, v) => (calls.push(["set", k, v]), true) });
const cloth = { getContext: () => pen, clientWidth: 800, clientHeight: 600, width: 0, height: 0 };
const lens = {
  getBBox: () => ({ x: [-10, 10], y: [-10, 10] }),
  graphToViewport: ({ x, y }) => ({ x: 400 + x * 10, y: 300 + y * 10 }),
  getCamera: () => ({ angle: 0.5 }),
};
const dome = m.sky(cloth, lens, { ring: "#123456", dust: "#abcdef" });
dome.draw(3);
assert.equal(cloth.width, 1600);
assert.equal(cloth.height, 1200);
const arcs = calls.filter((c) => c[0] === "arc");
assert.equal(arcs.length, 111);
const ringArc = arcs[arcs.length - 1];
assert.equal(ringArc[1], 400);
assert.equal(ringArc[2], 300);
assert.ok(Math.abs(ringArc[3] - 100 * 1.22) < 1e-9);
const dash = calls.find((c) => c[0] === "setLineDash")[1];
assert.equal(dash.length, 2);
const dots = (m.TAU * ringArc[3]) / (dash[0] + dash[1]);
assert.ok(Math.abs(dots - Math.round(dots)) < 1e-9);
assert.ok(calls.some((c) => c[0] === "set" && c[1] === "strokeStyle" && c[2] === "#123456"));
assert.ok(calls.some((c) => c[0] === "set" && c[1] === "fillStyle" && c[2] === "#abcdef"));
const offsetA = calls.filter((c) => c[0] === "set" && c[1] === "lineDashOffset").at(-1)[2];
calls.length = 0;
dome.draw(13);
const offsetB = calls.filter((c) => c[0] === "set" && c[1] === "lineDashOffset").at(-1)[2];
assert.ok(offsetB < offsetA);
calls.length = 0;
cloth.clientWidth = 0;
dome.draw(14);
assert.equal(calls.length, 0);

let now = 100000;
const realClock = Object.getOwnPropertyDescriptor(globalThis, "performance");
Object.defineProperty(globalThis, "performance", { value: { now: () => now }, configurable: true, writable: true });
const rafs = new Map();
let rafId = 0;
globalThis.requestAnimationFrame = (f) => (rafs.set(++rafId, f), rafId);
globalThis.cancelAnimationFrame = (id) => rafs.delete(id);
globalThis.document = { hidden: false, addEventListener() {}, removeEventListener() {} };
const turned = [];
const cam = { angle: 0, setState(s) { cam.angle = s.angle; turned.push(wire.own()); } };
const handlers = new Map();
const gl = { renders: 0, on: (e, f) => handlers.set(e, f), off: (e) => handlers.delete(e), getCamera: () => cam, scheduleRender: () => gl.renders++ };
const touch = new Map();
const pad = { addEventListener: (e, f) => touch.set(e, f), removeEventListener: (e) => touch.delete(e) };
const seen = [];
let busy = false;
let settled = 0;
let report = () => {};
let quit = 0;
const wire = m.motion({ live: gl, canvas: pad, busy: () => busy, paint: (s) => seen.push(s), settle: () => settled++ }, (change) => ((report = change), () => quit++));
assert.equal(rafs.size, 0);
assert.deepEqual([...touch.keys()].sort(), ["pointerdown", "touchstart", "wheel"]);
report(false);
assert.equal(rafs.size, 1);
const tick = (at) => {
  now = at;
  const [id, f] = [...rafs][0];
  rafs.delete(id);
  f(at);
};
const t0 = now + 6000;
tick(t0);
assert.ok(cam.angle > 0);
assert.deepEqual(turned, [true]);
assert.equal(wire.own(), false);
assert.equal(seen.length, 1);
assert.ok(Math.abs(m.clock.phase - (((t0 / 1000) * 1.4) % m.TAU)) < 1e-9);
const angle1 = cam.angle;
tick(t0 + 10);
assert.equal(cam.angle, angle1);
assert.equal(seen.length, 1);
tick(t0 + 40);
assert.ok(cam.angle > angle1);
assert.equal(seen.length, 2);
busy = true;
const angle2 = cam.angle;
const drawn = gl.renders;
tick(t0 + 80);
assert.equal(cam.angle, angle2);
assert.equal(gl.renders, drawn + 1);
busy = false;
touch.get("pointerdown")();
tick(t0 + 120);
assert.equal(cam.angle, angle2);
tick(t0 + 61000);
assert.ok(cam.angle > angle2);
assert.ok(settled >= 0);
for (let i = 0; i < 80; i++) {
  tick(t0 + 62000 + i * 40);
}
assert.ok(settled >= 1);
report(true);
assert.equal(rafs.size, 0);
report(false);
assert.equal(rafs.size, 1);
globalThis.document.hidden = true;
report(false);
assert.equal(rafs.size, 0);
globalThis.document.hidden = false;
report(false);
assert.equal(rafs.size, 1);

for (let i = 0; i < 30; i++) {
  handlers.get("beforeRender")();
  now += 12;
  handlers.get("afterRender")();
}
const before = seen.length;
const base = t0 + 200000;
tick(base);
assert.equal(seen.length, before + 1);
tick(base + 40);
assert.equal(seen.length, before + 1);
tick(base + 70);
assert.equal(seen.length, before + 2);
for (let i = 0; i < 30; i++) {
  handlers.get("beforeRender")();
  handlers.get("afterRender")();
}
tick(base + 110);
assert.equal(seen.length, before + 3);
wire.stop();
assert.equal(quit, 1);
assert.equal(rafs.size, 0);
assert.equal(touch.size, 0);
assert.equal(handlers.size, 0);
report(false);
assert.equal(rafs.size, 0);
delete globalThis.requestAnimationFrame;
delete globalThis.cancelAnimationFrame;
delete globalThis.document;
Object.defineProperty(globalThis, "performance", realClock);

console.log("galaxy ok");
