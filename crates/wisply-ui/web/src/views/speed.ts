import { board, type Insight, type InsightCmd } from "../api";
import { every, h, mount } from "../dom";
import { T } from "../text";
import { lapse, SORTS, type SortKey, sortCommands } from "./insight-logic";
import { pct, short, stat } from "./side";

const SORT_KEY = "wisply.speed";
const POLL_MS = 15000;

function commandTable(rows: InsightCmd[], key: SortKey): HTMLElement {
  const slowest = Math.max(1, ...rows.map((r) => r.avg_ms));
  return h(
    "section",
    { class: "panel" },
    h(
      "div",
      { class: "krows" },
      h("div", { class: "krow head" }, h("span", {}, T.speed.cmd), h("span", {}, T.speed.runs), h("span", {}, T.speed.errors), h("span", {}, T.speed.avg), h("span", {}, T.speed.tokens), h("span", {})),
      sortCommands(rows, key).map((r) =>
        h(
          "div",
          { class: "krow" },
          h("span", { class: "name" }, r.cmd, r.waits ? h("span", { class: "faint" }, ` · ${T.speed.waits}`) : null),
          h("span", {}, r.runs.toLocaleString("en-US")),
          h("span", { class: r.err_pct >= 20 && !r.waits ? "bad" : "faint" }, `${r.err_pct}%`),
          h("span", {}, lapse(r.avg_ms)),
          h("span", { class: "faint" }, r.avg_tokens ? short(r.avg_tokens) : "–"),
          h("span", { class: "meter" }, h("span", { class: "used", style: `width:${(100 * r.avg_ms) / slowest}%` })),
        ),
      ),
    ),
  );
}

function repeated(d: Insight): HTMLElement {
  return h(
    "section",
    { class: "panel" },
    h("h2", { style: "margin-top:0" }, T.speed.repeated),
    d.failures.length
      ? h("div", { class: "list" }, d.failures.slice(0, 10).map((f) => h("div", { class: "item" }, h("span", { class: "kind" }, f.cmd), h("span", { class: "grow ellipsis" }, f.error), h("span", { class: "when" }, T.speed.times(f.count)))))
      : h("div", { class: "faint" }, T.speed.noRepeated),
  );
}

function eaters(d: Insight): HTMLElement {
  const rows = [...d.commands].filter((r) => r.tokens > 0).sort((a, b) => b.tokens - a.tokens).slice(0, 6);
  const most = Math.max(1, ...rows.map((r) => r.tokens));
  return h(
    "section",
    { class: "panel" },
    h("h2", { style: "margin-top:0" }, T.speed.eaters),
    h("div", { class: "sub", style: "margin-bottom:10px" }, T.speed.eatersNote),
    rows.map((r) => h("div", { class: "krow", style: "grid-template-columns: 120px 80px 1fr" }, h("span", { class: "name" }, r.cmd), h("span", {}, short(r.tokens)), h("span", { class: "meter" }, h("span", { class: "used", style: `width:${(100 * r.tokens) / most}%` })))),
  );
}

export async function speed(view: HTMLElement): Promise<() => void> {
  let key = (SORTS.find((s) => s === localStorage.getItem(SORT_KEY)) ?? "runs") as SortKey;
  let data: Insight | null = null;
  const body = h("div", {});
  const seg = h("div", { class: "seg" });
  const draw = () => {
    localStorage.setItem(SORT_KEY, key);
    mount(seg, ...SORTS.map((s) => h("button", { class: s === key ? "on" : "", onclick: () => { key = s; draw(); } }, T.speed.sorts[s])));
    if (!data) return;
    if (!data.commands.length) {
      mount(body, h("div", { class: "empty" }, T.speed.none));
      return;
    }
    const runs = data.commands.reduce((n, r) => n + r.runs, 0);
    const err = data.commands.reduce((n, r) => n + r.err, 0);
    const tokens = data.commands.reduce((n, r) => n + r.tokens, 0);
    mount(
      body,
      h(
        "div",
        { class: "stats" },
        stat(T.speed.runsStat, runs.toLocaleString("en-US"), ""),
        stat(T.speed.errStat, `${pct(err, runs)}%`, `${err.toLocaleString("en-US")} / ${runs.toLocaleString("en-US")}`, data.light.tone === "good" ? "" : "bad"),
        stat(T.speed.slowStat, lapse(data.light.avg_ms), T.side.lightNote(data.light.err_pct, data.light.avg_ms, data.light.runs)),
        stat(T.speed.tokStat, short(tokens), ""),
      ),
      commandTable(data.commands, key),
      repeated(data),
      eaters(data),
    );
  };
  const load = async () => {
    try {
      data = await board<Insight>("insight");
    } catch (e) {
      mount(body, h("div", { class: "error" }, (e as Error).message));
      return;
    }
    draw();
  };
  mount(view, h("div", { class: "page" }, h("div", { class: "row head" }, h("div", { class: "grow" }, h("h1", {}, T.speed.title), h("div", { class: "sub" }, T.speed.sub)), seg), body));
  draw();
  await load();
  return every(() => void load(), POLL_MS);
}
