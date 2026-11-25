import type { Entry, InsightCmd, Light, Side } from "../api";

export type SortKey = "runs" | "err" | "slow" | "tokens";
export const SORTS: SortKey[] = ["runs", "err", "slow", "tokens"];

const SORT_BY: Record<SortKey, (r: InsightCmd) => number> = {
  runs: (r) => r.runs,
  err: (r) => r.err_pct,
  slow: (r) => r.avg_ms,
  tokens: (r) => r.tokens,
};

export function sortCommands(rows: InsightCmd[], key: SortKey): InsightCmd[] {
  const by = SORT_BY[key];
  return [...rows].sort((a, b) => by(b) - by(a) || b.runs - a.runs);
}

export function lapse(ms: number): string {
  if (ms < 1000) return `${Math.round(ms)} ms`;
  if (ms < 60_000) return `${(ms / 1000).toFixed(1)} s`;
  return `${(ms / 60_000).toFixed(1)} min`;
}

export type NavGroup = { title: string; names: string[] };

export const GROUPS: NavGroup[] = [
  { title: "work", names: ["todos", "projects", "agents", "live"] },
  { title: "knowledge", names: ["memory", "intelligence", "graph"] },
  { title: "improve", names: ["improve", "speed", "tokens", "learned", "blind"] },
  { title: "system", names: ["settings"] },
];

export function groupLinks<T extends [string, string, string]>(links: T[], groups: NavGroup[] = GROUPS): { title: string; links: T[] }[] {
  const byName = new Map(links.map((l) => [l[1], l]));
  const out = groups.map((g) => ({ title: g.title, links: g.names.map((n) => byName.get(n)).filter((l): l is T => l !== undefined) }));
  const placed = new Set(groups.flatMap((g) => g.names));
  const rest = links.filter((l) => !placed.has(l[1]));
  if (rest.length) out.push({ title: "more", links: rest });
  return out.filter((g) => g.links.length > 0);
}

export function badgeOf(name: string, side: Pick<Side, "badges"> | null): number {
  if (!side) return 0;
  if (name === "todos") return side.badges.todos;
  if (name === "improve") return side.badges.insight;
  return 0;
}

export function lightWord(light: Light): "good" | "warn" | "bad" {
  return light.tone;
}

export type Proposals = { waiting: Entry[]; taken: Entry[]; dropped: number };

export function proposalsOf(rows: Entry[]): Proposals {
  const mine = rows.filter((e) => e.origin === "insight");
  return {
    waiting: mine.filter((e) => e.status === "suggested").sort((a, b) => b.updated - a.updated),
    taken: mine.filter((e) => e.status !== "suggested" && e.status !== "dropped").sort((a, b) => b.updated - a.updated),
    dropped: mine.filter((e) => e.status === "dropped").length,
  };
}

export function findPrompt(): string {
  return 'Run `wisply insight`, check every suspicious item against the real code or log, and send each real finding with `wisply insight add "title | why | proof"`. Skip anything you cannot prove.';
}
