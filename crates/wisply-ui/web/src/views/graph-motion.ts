import { every } from "../dom";
import { overview } from "./agent-data";
import { TAU, clock } from "./graph-glow";
import type { Stage } from "./graph-style";

const FRAME_MS = [33, 66, 133];
const COST_HIGH = 9;
const COST_LOW = 3;
const SAMPLE = 30;
const STILL_MS = 4000;
const TURN = 0.012;
const SETTLE_S = 1.5;
const GAME_MS = 15000;
const TWINKLE = 1.4;
const SEC = 1000;
const MAX_STEP = 0.25;
const TOUCH = ["pointerdown", "wheel", "touchstart"] as const;

export type Wire = { live: Stage; canvas: HTMLElement; busy: () => boolean; paint: (seconds: number) => void; settle: () => void };
export type Motion = { own: () => boolean; poke: () => void; stop: () => void };

const still = () => window.matchMedia("(prefers-reduced-motion: reduce)").matches;

function governor(live: Stage): { gap: () => number; stop: () => void } {
  let level = 0;
  let cost = 0;
  let count = 0;
  let began = 0;
  const open = () => {
    began = performance.now();
  };
  const close = () => {
    cost += performance.now() - began;
    if (++count < SAMPLE) return;
    const mean = cost / count;
    cost = 0;
    count = 0;
    if (mean > COST_HIGH) level = Math.min(FRAME_MS.length - 1, level + 1);
    else if (mean < COST_LOW) level = Math.max(0, level - 1);
  };
  live.on("beforeRender", open);
  live.on("afterRender", close);
  return {
    gap: () => FRAME_MS[level],
    stop: () => {
      live.off("beforeRender", open);
      live.off("afterRender", close);
    },
  };
}

export type Watch = (change: (on: boolean) => void) => () => void;

const gaming: Watch = (change) => {
  const poll = () =>
    overview().then(
      (o) => change(o?.pc.game_mode ?? false),
      () => change(false),
    );
  const quit = every(poll, GAME_MS);
  void poll();
  return quit;
};

export function motion(w: Wire, watch: Watch = gaming): Motion {
  const cam = w.live.getCamera();
  const pace = governor(w.live);
  let raf = 0;
  let last = 0;
  let drift = 0;
  let poked = performance.now();
  let gamer = true;
  let own = false;
  let dead = false;
  const turn = (dt: number) => {
    own = true;
    cam.setState({ angle: cam.angle + TURN * dt });
    own = false;
    drift += dt;
    if (drift < SETTLE_S) return;
    drift = 0;
    w.settle();
  };
  const step = (now: number) => {
    raf = requestAnimationFrame(step);
    if (now - last < pace.gap()) return;
    const dt = Math.min(MAX_STEP, (now - last) / SEC);
    last = now;
    clock.phase = ((now / SEC) * TWINKLE) % TAU;
    if (now - poked > STILL_MS && !w.busy()) turn(dt);
    else w.live.scheduleRender();
    w.paint(now / SEC);
  };
  const sync = () => {
    if (!dead && !document.hidden && !gamer && !still()) {
      raf ||= requestAnimationFrame(step);
      return;
    }
    cancelAnimationFrame(raf);
    raf = 0;
  };
  const poke = () => {
    poked = performance.now();
  };
  for (const e of TOUCH) w.canvas.addEventListener(e, poke, { passive: true });
  document.addEventListener("visibilitychange", sync);
  const quit = watch((on) => {
    gamer = on;
    sync();
  });
  return {
    own: () => own,
    poke,
    stop: () => {
      dead = true;
      sync();
      quit();
      pace.stop();
      document.removeEventListener("visibilitychange", sync);
      for (const e of TOUCH) w.canvas.removeEventListener(e, poke);
    },
  };
}
