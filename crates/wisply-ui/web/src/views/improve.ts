import { board, type Entry, type Insight, type ListData } from "../api";
import { ago, every, h, mount } from "../dom";
import { T } from "../text";
import { findPrompt, proposalsOf } from "./insight-logic";
import { stat } from "./side";

const POLL_MS = 10000;
const PROJECT = "--project=wisply";

function detail(e: Entry): HTMLElement | null {
  return e.last_note ? h("div", { class: "body" }, e.last_note) : null;
}

function waitingRow(e: Entry, act: (id: number, status: string) => void): HTMLElement {
  return h(
    "div",
    { class: "item todo" },
    h("span", { class: "kind" }, `#${e.id}`),
    h("div", { class: "grow" }, h("div", {}, e.task), detail(e)),
    h("span", { class: "when" }, T.ago(ago(e.updated))),
    h(
      "span",
      { class: "acts", style: "display:flex" },
      h("button", { class: "primary", onclick: () => act(e.id, "todo") }, T.improve.accept),
      h("button", { onclick: () => act(e.id, "dropped") }, T.improve.dismiss),
    ),
  );
}

function takenRow(e: Entry): HTMLElement {
  const label = T.list.statuses[e.status] ?? e.status;
  return h("div", { class: "item" }, h("span", { class: "kind" }, `#${e.id}`), h("span", { class: "grow" }, e.task), h("span", { class: `pill st-${e.status}` }, label), h("span", { class: "when" }, T.ago(ago(e.updated))));
}

function suspicious(d: Insight): HTMLElement {
  if (!d.findings.length) return h("div", { class: "empty" }, T.improve.noSuspicious);
  return h(
    "div",
    { class: "list" },
    d.findings.map((f) =>
      h(
        "div",
        { class: "item" },
        h("span", { class: "kind" }, T.improve.kinds[f.kind] ?? f.kind),
        h("div", { class: "grow" }, h("div", {}, f.text), f.proof ? h("div", { class: "body" }, f.proof) : null),
      ),
    ),
  );
}

function copyButton(): HTMLElement {
  const button = h("button", { class: "primary" }, T.improve.find);
  button.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText(findPrompt());
      button.textContent = T.improve.copied;
    } catch {
      button.textContent = T.side.findFail;
    }
    window.setTimeout(() => (button.textContent = T.improve.find), 2500);
  });
  return button;
}

export async function improve(view: HTMLElement): Promise<() => void> {
  const body = h("div", {});
  mount(view, h("div", { class: "page" }, h("div", { class: "row head" }, h("div", { class: "grow" }, h("h1", {}, T.improve.title), h("div", { class: "sub" }, T.improve.sub)), copyButton()), body));
  let seen = "";
  const draw = async () => {
    let list: ListData;
    let insight: Insight;
    try {
      [list, insight] = await Promise.all([board<ListData>("list", PROJECT), board<Insight>("insight")]);
    } catch (e) {
      mount(body, h("div", { class: "error" }, (e as Error).message));
      return;
    }
    const text = JSON.stringify([list.rows, insight]);
    if (text === seen) return;
    seen = text;
    const p = proposalsOf(list.rows);
    const act = async (id: number, status: string) => {
      try {
        await board("entry-set", String(id), "status", `--value=${status}`);
      } catch (e) {
        mount(body, h("div", { class: "error" }, (e as Error).message));
        return;
      }
      seen = "";
      await draw();
    };
    mount(
      body,
      h(
        "div",
        { class: "stats" },
        stat(T.improve.stats.waiting, String(p.waiting.length), T.improve.dismissed(p.dropped), p.waiting.length ? "good" : ""),
        stat(T.improve.stats.taken, String(p.taken.length), ""),
        stat(T.improve.stats.suspicious, String(insight.findings.length), ""),
        stat(T.improve.stats.errors, `${insight.light.err_pct}%`, T.improve.errNote, insight.light.tone === "good" ? "" : "bad"),
      ),
      h("h2", {}, T.improve.waiting, h("span", { class: "n" }, p.waiting.length)),
      p.waiting.length ? h("div", { class: "list" }, p.waiting.map((e) => waitingRow(e, (id, status) => void act(id, status)))) : h("div", { class: "empty" }, T.improve.none),
      h("h2", {}, T.improve.taken, h("span", { class: "n" }, p.taken.length)),
      p.taken.length ? h("div", { class: "list" }, p.taken.map(takenRow)) : h("div", { class: "faint" }, T.improve.noneTaken),
      h("h2", {}, T.improve.suspicious, h("span", { class: "n" }, insight.findings.length)),
      suspicious(insight),
    );
  };
  await draw();
  return every(() => void draw(), POLL_MS);
}
