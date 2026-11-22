import type { Net } from "./graph-data";

const NAME_MAX = 30;
const CHAR = 0.56;
const LINE = 1.3;
const GAP = 4;
const ROOM = 2.5;

export type Box = { left: number; right: number; low: number; high: number };

export function brief(label: string): string {
  const t = label.trim();
  return t.length > NAME_MAX ? `${t.slice(0, NAME_MAX - 1).trimEnd()}…` : t;
}

export function box(corners: { x: number; y: number }[]): Box {
  const xs = corners.map((c) => c.x);
  const ys = corners.map((c) => c.y);
  return { left: Math.min(...xs), right: Math.max(...xs), low: Math.min(...ys), high: Math.max(...ys) };
}

export type Spot = { n: string; x: number; y: number; r: number; text: string };

export function spread(spots: Spot[], size: number, width: number, height: number): string[] {
  const kept: Box[] = [];
  const out: string[] = [];
  for (const s of spots) {
    const half = Math.max(s.r, (size * LINE) / 2) + ROOM;
    const b = { left: s.x - s.r - ROOM, right: s.x + s.r + GAP + s.text.length * size * CHAR + ROOM, low: s.y - half, high: s.y + half };
    if (b.left < 0 || b.right > width || b.low < 0 || b.high > height) continue;
    if (kept.some((k) => b.left < k.right && k.left < b.right && b.low < k.high && k.low < b.high)) continue;
    kept.push(b);
    out.push(s.n);
  }
  return out;
}

export function nameless(g: Net, view: Box, least: number, gone: ReadonlySet<string>, limit: number): string[] {
  const out: string[] = [];
  g.forEachNode((n, a) => {
    if (a.tag || a.size < least || a.x < view.left || a.x > view.right || a.y < view.low || a.y > view.high || gone.has(n)) return;
    out.push(n);
  });
  if (out.length <= limit) return out;
  return out.sort((x, y) => g.getNodeAttribute(y, "size") - g.getNodeAttribute(x, "size") || (x < y ? -1 : 1)).slice(0, limit);
}
