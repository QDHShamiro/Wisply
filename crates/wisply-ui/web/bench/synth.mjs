const WORDS = "amber anchor atlas basil birch cedar cinder cobalt comet coral delta ember fable fern flint garnet harbor hazel indigo jasper kelp lantern linen maple marble meadow nectar onyx opal orchid pebble pine quartz raven reef saffron sage slate sorrel spruce thistle timber tundra umber velvet walnut willow zephyr".split(" ");

function rng(seed) {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

export function synth(total, seed = 7) {
  const r = rng(seed);
  const pick = (n) => Math.floor(r() * n);
  const title = (words) => Array.from({ length: words }, () => WORDS[pick(WORDS.length)]).join(" ");
  const now = 1_780_000_000;
  const span = 60 * 60 * 24 * 700;
  const projects = Math.max(3, Math.round(total * 0.02));
  const sessions = Math.round(total * 0.075);
  const notes = total - projects - sessions;
  const nodes = [];
  const edges = [];
  const weight = Array.from({ length: projects }, (_, i) => 1 / (i + 1) ** 0.75);
  const sum = weight.reduce((a, b) => a + b, 0);
  const project = () => {
    let x = r() * sum;
    for (let i = 0; i < projects; i++) {
      x -= weight[i];
      if (x <= 0) return i;
    }
    return projects - 1;
  };
  const names = Array.from({ length: projects }, (_, i) => `${WORDS[i % WORDS.length]}-${Math.floor(i / WORDS.length) + 1}`);
  names.forEach((n) => nodes.push([`p${n}`, n, "project", n, 0]));
  const byProject = Array.from({ length: projects }, () => []);
  const sessionAt = projects + notes;
  for (let i = 0; i < notes; i++) {
    const loose = r() < 0.36;
    const p = loose ? -1 : project();
    const at = nodes.length;
    nodes.push([`n${i + 1}`, title(6 + pick(10)), "note", p < 0 ? "" : names[p], Math.round(now - r() * span)]);
    if (p >= 0) {
      edges.push([at, p]);
      byProject[p].push(at);
    }
    if (r() < (loose ? 0.18 : 0.22)) edges.push([at, sessionAt + pick(sessions)]);
  }
  for (let i = 0; i < sessions; i++) nodes.push([`s${i + 1}`, title(4 + pick(6)), "session", names[project()], Math.round(now - r() * span)]);
  for (let p = 0; p < projects; p++) {
    const list = byProject[p];
    for (const at of list) {
      if (r() < 0.1 && list.length > 1) edges.push([at, list[pick(list.length)]]);
      if (r() < 0.012) {
        const other = byProject[project()];
        if (other.length) edges.push([at, other[pick(other.length)]]);
      }
    }
  }
  const seen = new Set();
  const clean = edges
    .filter(([a, b]) => a !== b)
    .map(([a, b]) => (a < b ? [a, b] : [b, a]))
    .filter(([a, b]) => !seen.has(a * 1e7 + b) && seen.add(a * 1e7 + b))
    .sort((x, y) => x[0] - y[0] || x[1] - y[1]);
  return { nodes, edges: clean };
}
