import { TAU } from "./graph-glow";
import type { Stage } from "./graph-style";

const DUST = 110;
const DUST_MIN = 0.5;
const DUST_MAX = 1.8;
const DRIFT_MIN = 0.002;
const DRIFT_MAX = 0.009;
const DUST_BASE = 0.3;
const DUST_TWINKLE = 1.1;
const SEED = 20261127;
const REACH = 1.22;
const DOT = 0.01;
const DOT_GAP = 12;
const DOT_MIN = 24;
const DOT_MAX = 1500;
const RING_WIDTH = 2.2;
const RING_SPIN = 0.01;
const RING_PULSE = 0.5;
const RING_BASE = 0.75;
const RING_SWAY = 0.25;
const FAR = 40000;

export type Look = { ring: string; dust: string };
export type Sky = { draw: (seconds?: number) => void };

function prng(seed: number): () => number {
  let s = seed >>> 0;
  return () => {
    s = (s + 0x6d2b79f5) >>> 0;
    let t = s;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

const wrap = (v: number) => ((v % 1) + 1) % 1;

export function sky(canvas: HTMLCanvasElement, live: Stage, look: Look): Sky {
  const ctx = canvas.getContext("2d") as CanvasRenderingContext2D;
  const next = prng(SEED);
  const specks = Array.from({ length: DUST }, () => ({
    x: next(),
    y: next(),
    r: DUST_MIN + next() * (DUST_MAX - DUST_MIN),
    v: DRIFT_MIN + next() * (DRIFT_MAX - DRIFT_MIN),
    p: next() * TAU,
  }));
  let at = 0;
  const ring = (w: number, h: number) => {
    const box = live.getBBox();
    const x = (box.x[0] + box.x[1]) / 2;
    const y = (box.y[0] + box.y[1]) / 2;
    const reach = (Math.max(box.x[1] - box.x[0], box.y[1] - box.y[0]) / 2) * REACH;
    const mid = live.graphToViewport({ x, y });
    const edge = live.graphToViewport({ x: x + reach, y });
    const r = Math.hypot(edge.x - mid.x, edge.y - mid.y);
    if (!Number.isFinite(r) || r < 1 || r > FAR || !w || !h) return;
    const dots = Math.min(DOT_MAX, Math.max(DOT_MIN, Math.round((TAU * r) / DOT_GAP)));
    ctx.save();
    ctx.globalAlpha = RING_BASE + RING_SWAY * Math.sin(at * RING_PULSE);
    ctx.strokeStyle = look.ring;
    ctx.lineWidth = RING_WIDTH;
    ctx.lineCap = "round";
    ctx.setLineDash([DOT, (TAU * r) / dots - DOT]);
    ctx.lineDashOffset = -(live.getCamera().angle + at * RING_SPIN) * r;
    ctx.beginPath();
    ctx.arc(mid.x, mid.y, r, 0, TAU);
    ctx.stroke();
    ctx.restore();
  };
  const draw = (seconds = at) => {
    at = seconds;
    const w = canvas.clientWidth;
    const h = canvas.clientHeight;
    if (!w || !h) return;
    const dpr = window.devicePixelRatio || 1;
    if (canvas.width !== Math.round(w * dpr) || canvas.height !== Math.round(h * dpr)) {
      canvas.width = Math.round(w * dpr);
      canvas.height = Math.round(h * dpr);
    }
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    ctx.clearRect(0, 0, w, h);
    ctx.fillStyle = look.dust;
    for (const s of specks) {
      ctx.globalAlpha = DUST_BASE + (1 - DUST_BASE) * (0.5 + 0.5 * Math.sin(at * DUST_TWINKLE + s.p));
      ctx.beginPath();
      ctx.arc(wrap(s.x + Math.sin(s.p) * s.v * at) * w, wrap(s.y - s.v * at) * h, s.r, 0, TAU);
      ctx.fill();
    }
    ctx.globalAlpha = 1;
    ring(w, h);
  };
  return { draw };
}
