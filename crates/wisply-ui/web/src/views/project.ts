import { board, type Project } from "../api";
import { ago, enc, h, mount } from "../dom";
import { render } from "../md";
import { T } from "../text";
import { list, type ListTab } from "./list";

export type Tab = "notes" | "todos" | "sessions" | "graph";

function notes(p: Project): HTMLElement {
  const list = h("div", { class: "list" });
  const draw = (want: string) => {
    const w = want.trim().toLowerCase();
    const shown = p.notes.filter((n) => !w || n.title.toLowerCase().includes(w));
    mount(
      list,
      ...shown.map((n) =>
        h("a", { class: "item", href: `#/n/${n.id}` }, h("span", { class: "kind" }, n.kind || n.source), h("span", { class: "grow ellipsis" }, n.title), h("span", { class: "when" }, T.ago(ago(n.updated)))),
      ),
    );
    if (!shown.length) mount(list, h("div", { class: "empty" }, T.project.noNotes));
  };
  draw("");
  const filter = h("input", { type: "search", placeholder: T.project.filter, oninput: (e: Event) => draw((e.target as HTMLInputElement).value) });
  return h("div", {}, h("div", { class: "row", style: "margin-bottom:10px" }, filter), list);
}

function sessions(p: Project): HTMLElement {
  if (!p.sessions.length) return h("div", { class: "empty" }, T.project.noSessions);
  return h(
    "div",
    { class: "list" },
    p.sessions.map((s) =>
      h(
        "div",
        { class: "item", style: "flex-direction:column;align-items:stretch;gap:2px" },
        h("div", { class: "row" }, h("span", { class: "grow ellipsis" }, s.title || s.id.slice(0, 8)), h("span", { class: "when" }, T.ago(ago(s.last)))),
        s.summary ? h("div", { class: "body" }, s.summary) : null,
        s.next ? h("div", { class: "body faint" }, `${T.project.next}: ${s.next}`) : null,
      ),
    ),
  );
}

export async function project(view: HTMLElement, name: string, tab: Tab, sub: ListTab = "open"): Promise<() => void> {
  const p = await board<Project>("project", name);
  const tabs = (Object.keys(T.project.tabs) as Tab[]).map((k) => h("a", { href: `#/p/${enc(name)}/${k}`, class: k === tab ? "on" : "" }, T.project.tabs[k]));
  const open = p.todos.filter((t) => t.status === "todo" || t.status === "doing").length;
  const about = p.about ? h("div", { class: "md sub" }) : null;
  const lines = p.about.split("\n");
  if (lines[0]?.replace(/^#\s+/, "").trim() === p.name) lines.shift();
  if (about) about.innerHTML = render(lines.slice(0, 6).join("\n"));
  const body = h("div", {});
  mount(
    view,
    h(
      "div",
      { class: tab === "graph" ? "page wide" : "page" },
      tab === "graph" ? null : h("h1", {}, p.name),
      tab === "graph" ? null : h("div", { class: "muted" }, T.home.counts(p.notes.length, open, p.sessions.length).join(" · ")),
      tab === "graph" ? null : about,
      h("div", { class: "tabs", style: tab === "graph" ? "margin:0;padding:0 16px" : "" }, tabs),
      body,
    ),
  );
  if (tab === "notes") mount(body, notes(p));
  else if (tab === "todos") return list(body, sub, name);
  else if (tab === "sessions") mount(body, sessions(p));
  else {
    body.style.height = "calc(100% - 42px)";
    return (await import("./graph")).graph(body, name);
  }
  return () => {};
}
