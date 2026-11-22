import type { Net } from "./graph-data";

const TURN = Math.PI * (3 - Math.sqrt(5));
const STEP = 6;
const HOLE = 4;
const PAD = 12;
const NEST = 3;
const RIM = 0.5;
const HALO_GAP = 18;
const SPREAD = 4;
const BOND = 0.2;
const BOND_MIN = 3;
const BRIDGE = 8;

export type Disc = { r: number; x: number; y: number };
type Family = Disc & { node: string; hole: number; leaves: string[]; heavy: Family[]; ox: number; oy: number };
type Kids = Map<string, string[]>;
type Ties = Map<string, Map<string, number>>;

function arc(r: number, at: number): number {
  return 2 * Math.asin(Math.min(1, r / at));
}

export function rings<T extends Disc>(discs: T[], hold = 0): number {
  const order = [...discs].sort((a, b) => b.r - a.r);
  let inner = hold;
  let i = 0;
  if (!hold && order.length) {
    order[0].x = 0;
    order[0].y = 0;
    inner = order[0].r;
    i = 1;
  }
  let turn = 0;
  while (i < order.length) {
    const at = inner + order[i].r;
    const row: T[] = [];
    let used = 0;
    while (i < order.length && (!row.length || used + arc(order[i].r, at) <= 2 * Math.PI)) {
      used += arc(order[i].r, at);
      row.push(order[i++]);
    }
    const slack = Math.max(0, 2 * Math.PI - used) / row.length;
    let angle = turn;
    for (const d of row) {
      const wide = arc(d.r, at) + slack;
      d.x = at * Math.cos(angle + wide / 2);
      d.y = at * Math.sin(angle + wide / 2);
      angle += wide;
    }
    inner = at + row[0].r;
    turn += TURN;
  }
  return inner;
}

function vote(g: Net, n: string, home: Map<string, string>): string | null {
  const count = new Map<string, number>();
  g.forEachNeighbor(n, (m) => {
    const k = home.get(m);
    if (k !== undefined) count.set(k, (count.get(k) ?? 0) + 1);
  });
  let best: string | null = null;
  let most = 0;
  for (const [k, c] of count) {
    if (c > most || (c === most && best !== null && k < best)) {
      best = k;
      most = c;
    }
  }
  return best;
}

function islands(g: Net, open: string[], home: Map<string, string>): void {
  for (const start of open) {
    if (home.has(start)) continue;
    const key = `\u0000${start}`;
    const queue = [start];
    home.set(start, key);
    for (let at = 0; at < queue.length; at++)
      g.forEachNeighbor(queue[at], (m) => {
        if (home.has(m)) return;
        home.set(m, key);
        queue.push(m);
      });
  }
}

function ties(g: Net, home: Map<string, string>): Ties {
  const out: Ties = new Map();
  const add = (a: string, b: string): void => {
    const row = out.get(a) ?? new Map<string, number>();
    row.set(b, (row.get(b) ?? 0) + 1);
    out.set(a, row);
  };
  g.forEachEdge((_e, _a, s, t) => {
    const a = home.get(s);
    const b = home.get(t);
    if (a === undefined || b === undefined || a === b) return;
    add(a, b);
    add(b, a);
  });
  return out;
}

function partner(row: Map<string, number>): { best: string; most: number; all: number } {
  let all = 0;
  let most = 0;
  let best = "";
  for (const [k, count] of row) {
    all += count;
    if (count > most || (count === most && k < best)) {
      best = k;
      most = count;
    }
  }
  return { best, most, all };
}

export function kin(g: Net, home: Map<string, string>, groups: Map<string, string[]>): string[][] {
  const lead = new Map<string, string>();
  const root = (k: string): string => {
    let at = k;
    for (let up = lead.get(at); up !== undefined; up = lead.get(at)) at = up;
    return at;
  };
  const join = (a: string, b: string): void => {
    const x = root(a);
    const y = root(b);
    if (x !== y) lead.set(y, x);
  };
  const size = (k: string): number => (groups.get(k) ?? []).length;
  for (const [a, row] of ties(g, home)) {
    for (const [b, count] of row) if (a < b && count >= BOND_MIN && count >= BOND * Math.min(size(a), size(b))) join(a, b);
    const { best, most, all } = partner(row);
    if (most * 2 > all && most >= BOND * size(a)) join(best, a);
  }
  const clans = new Map<string, string[]>();
  for (const k of groups.keys()) {
    const r = root(k);
    const list = clans.get(r);
    if (list) list.push(k);
    else clans.set(r, [k]);
  }
  return [...clans.values()];
}

export function homes(g: Net): { groups: Map<string, string[]>; loose: string[]; home: Map<string, string> } {
  const home = new Map<string, string>();
  const loose: string[] = [];
  let open: string[] = [];
  g.forEachNode((n, a) => {
    if (!g.degree(n)) loose.push(n);
    else if (a.cluster) home.set(n, a.cluster);
    else open.push(n);
  });
  for (let round = 0; round < SPREAD && open.length; round++) {
    const found = open.map((n) => [n, vote(g, n, home)] as const);
    open = [];
    for (const [n, k] of found) {
      if (k === null) open.push(n);
      else home.set(n, k);
    }
  }
  islands(g, open, home);
  const groups = new Map<string, string[]>();
  for (const [n, k] of home) {
    const list = groups.get(k);
    if (list) list.push(n);
    else groups.set(k, [n]);
  }
  for (const list of groups.values()) list.sort((a, b) => g.degree(b) - g.degree(a) || (a < b ? -1 : 1));
  return { groups, loose, home };
}

export function elders(g: Net, home: Map<string, string>, groups: Map<string, string[]>): Kids {
  const kids: Kids = new Map();
  for (const [key, nodes] of groups) {
    const hub = nodes[0];
    for (const n of nodes) {
      if (n === hub) continue;
      const own = g.degree(n);
      let elder = hub;
      let least = Infinity;
      g.forEachNeighbor(n, (m) => {
        if (m === hub || home.get(m) !== key) return;
        const deg = g.degree(m);
        if (deg < own || (deg === own && m > n)) return;
        if (deg < least || (deg === least && m < elder)) {
          elder = m;
          least = deg;
        }
      });
      const list = kids.get(elder);
      if (list) list.push(n);
      else kids.set(elder, [n]);
    }
  }
  return kids;
}

function family(g: Net, kids: Kids, n: string, pad: number): Family {
  const hole = HOLE * Math.sqrt(1 + Math.sqrt(g.degree(n)));
  const leaves: string[] = [];
  const heavy: Family[] = [];
  for (const c of kids.get(n) ?? []) {
    if (kids.has(c)) heavy.push(family(g, kids, c, NEST));
    else leaves.push(c);
  }
  const core = Math.sqrt(hole * hole + leaves.length * STEP * STEP);
  let r = core;
  let cx = 0;
  let cy = 0;
  if (heavy.length) {
    rings(heavy, core);
    const box = [-core, core, -core, core];
    for (const h of heavy) {
      box[0] = Math.min(box[0], h.x - h.r);
      box[1] = Math.max(box[1], h.x + h.r);
      box[2] = Math.min(box[2], h.y - h.r);
      box[3] = Math.max(box[3], h.y + h.r);
    }
    cx = (box[0] + box[1]) / 2;
    cy = (box[2] + box[3]) / 2;
    r = Math.hypot(cx, cy) + core;
    for (const h of heavy) r = Math.max(r, Math.hypot(h.x - cx, h.y - cy) + h.r);
  }
  return { node: n, hole, leaves, heavy, r: r + Math.min(pad, r * RIM), x: 0, y: 0, ox: -cx, oy: -cy };
}

function seat(g: Net, f: Family, x: number, y: number): void {
  const a = g.getNodeAttributes(f.node);
  a.x = x + f.ox;
  a.y = y + f.oy;
  f.leaves.forEach((n, i) => {
    const r = Math.sqrt(f.hole * f.hole + (i + 1) * STEP * STEP);
    const b = g.getNodeAttributes(n);
    b.x = a.x + r * Math.cos((i + 1) * TURN);
    b.y = a.y + r * Math.sin((i + 1) * TURN);
  });
  for (const h of f.heavy) seat(g, h, a.x + h.x, a.y + h.y);
}

function shift(g: Net, kids: Kids, n: string, dx: number, dy: number): void {
  const a = g.getNodeAttributes(n);
  a.x += dx;
  a.y += dy;
  for (const c of kids.get(n) ?? []) shift(g, kids, c, dx, dy);
}

function bridges(g: Net, home: Map<string, string>, kids: Kids, hubs: Set<string>): void {
  g.forEachNode((n, a) => {
    if (g.degree(n) < BRIDGE || hubs.has(n)) return;
    const own = home.get(n);
    let all = 0;
    let away = 0;
    let x = 0;
    let y = 0;
    g.forEachNeighbor(n, (m, b) => {
      all++;
      if (home.get(m) !== own) away++;
      x += b.x;
      y += b.y;
    });
    if (away * 2 > all) shift(g, kids, n, x / all - a.x, y / all - a.y);
  });
}

function halo(g: Net, loose: string[]): void {
  let far = 0;
  g.forEachNode((n, a) => {
    if (g.degree(n)) far = Math.max(far, a.x * a.x + a.y * a.y);
  });
  const from = (Math.sqrt(far) + HALO_GAP) ** 2;
  loose.forEach((n, i) => {
    const r = Math.sqrt(from + i * STEP * STEP);
    const a = g.getNodeAttributes(n);
    a.x = r * Math.cos(i * TURN);
    a.y = r * Math.sin(i * TURN);
  });
}

export function scatter(g: Net): Map<string, string> {
  const { groups, loose, home } = homes(g);
  const kids = elders(g, home, groups);
  const discs = new Map<string, Family>();
  for (const [key, nodes] of groups) discs.set(key, family(g, kids, nodes[0], PAD));
  const clans = kin(g, home, groups).map((keys) => {
    const list = keys.flatMap((k) => discs.get(k) ?? []);
    return { list, r: list.length > 1 ? rings(list) : list[0].r, x: 0, y: 0 };
  });
  rings(clans);
  for (const c of clans) for (const d of c.list) seat(g, d, c.x + d.x, c.y + d.y);
  bridges(g, home, kids, new Set([...groups.values()].map((nodes) => nodes[0])));
  halo(g, loose);
  return home;
}
