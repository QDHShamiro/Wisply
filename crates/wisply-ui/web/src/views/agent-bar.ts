import { h } from "../dom";
import { T } from "../text";
import { type Agent, type Overview } from "./agent-data";

const COUNT_MS = 650;
const still = () => window.matchMedia("(prefers-reduced-motion: reduce)").matches;

function stamp(at: number): string {
  const d = new Date(at * 1000);
  return `${String(d.getHours()).padStart(2, "0")}:${String(d.getMinutes()).padStart(2, "0")}`;
}

type Cell = { el: HTMLElement; num: HTMLElement; shown: number; frame: number };

function count(cell: Cell, to: number): void {
  cancelAnimationFrame(cell.frame);
  const from = cell.shown;
  cell.shown = to;
  if (from === to || still()) {
    cell.num.textContent = String(to);
    return;
  }
  const t0 = performance.now();
  const step = (now: number) => {
    const k = Math.min(1, (now - t0) / COUNT_MS);
    cell.num.textContent = String(Math.round(from + (to - from) * (1 - (1 - k) ** 3)));
    if (k < 1) cell.frame = requestAnimationFrame(step);
  };
  cell.frame = requestAnimationFrame(step);
}

export function bar(): { el: HTMLElement; update: (rows: Agent[], info: Overview | null) => void } {
  const el = h("div", { class: "ag-bar" });
  const cells = new Map<string, Cell>();
  const cell = (key: string, label: string): Cell => {
    let c = cells.get(key);
    if (!c) {
      const num = h("span", { class: "ag-num" }, "0");
      const wrap = h("div", { class: "ag-stat", style: `--i:${cells.size}` }, num, h("span", { class: "ag-lab" }, label));
      c = { el: wrap, num, shown: 0, frame: 0 };
      cells.set(key, c);
      el.append(wrap);
    }
    return c;
  };
  const text = (key: string, label: string, value: string, tone: string) => {
    const c = cell(key, label);
    if (c.num.textContent !== value) {
      c.num.textContent = value;
      c.num.classList.remove("tick");
      void c.num.offsetWidth;
      c.num.classList.add("tick");
    }
    c.el.className = `ag-stat ${tone}`;
  };
  const number = (key: string, label: string, value: number, tone: string) => {
    const c = cell(key, label);
    count(c, value);
    c.el.className = `ag-stat ${tone}`;
  };
  const update = (rows: Agent[], info: Overview | null) => {
    const n = (s: string) => rows.filter((r) => r.status === s).length;
    number("run", T.agents.running, n("running"), "s-running");
    number("wait", T.agents.waiting, n("waiting"), "s-waiting");
    number("done", T.agents.finished, n("done"), "s-done");
    number("bad", T.agents.failed, n("error") + n("limit"), "s-error");
    if (!info) return;
    const limit = info.limit.hit ? (info.limit.reset_at ? T.agents.limitUntil(stamp(info.limit.reset_at)) : T.agents.limitHit) : T.agents.limitOk;
    text("limit", T.agents.limit, limit, info.limit.hit ? "s-error" : "s-done");
    if (info.pc.free_ram_gb !== null) text("ram", T.agents.ram, `${info.pc.free_ram_gb} GB`, info.pc.free_ram_gb < 3 ? "s-waiting" : "s-done");
    text("build", T.agents.build, info.build ?? T.agents.buildFree, info.build ? "s-waiting" : "s-done");
    text("game", T.agents.game, info.pc.game_mode ? T.agents.on : T.agents.off, info.pc.game_mode ? "s-waiting" : "s-done");
  };
  return { el, update };
}
