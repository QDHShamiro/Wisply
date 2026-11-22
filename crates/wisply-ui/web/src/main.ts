import { board, call, type Pulse } from "./api";
import { enc, go, h, icon, mount } from "./dom";
import { hashParts } from "./hash";
import { T } from "./text";
import { agents } from "./views/agents";
import { blind } from "./views/blind";
import { home } from "./views/home";
import { intel } from "./views/intel";
import { list, type ListTab, TABS } from "./views/list";
import { live } from "./views/live";
import { memory } from "./views/memory";
import { project, type Tab } from "./views/project";
import { search } from "./views/search";
import { learned, settings, tokens } from "./views/side";

const STATUS_MS = 5000;
const THEME_KEY = "wisply.theme";
const THEMES = ["light", "dark", "glass"] as const;
type Theme = (typeof THEMES)[number];
let theme: Theme = THEMES.find((t) => t === localStorage.getItem(THEME_KEY)) ?? "dark";
document.documentElement.dataset.theme = theme;
void call(["theme", theme]);
const side = document.getElementById("side") as HTMLElement;
const view = document.getElementById("view") as HTMLElement;
const status = h("div", { id: "status" });
let cleanup: () => void = () => {};
let turn = 0;

const links: [string, string, string][] = [
  ["#/", "todos", T.nav.todos],
  ["#/projects", "projects", T.nav.projects],
  ["#/memory", "memory", T.nav.memory],
  ["#/intelligence", "intelligence", T.nav.intelligence],
  ["#/graph", "graph", T.nav.graph],
  ["#/live", "live", T.nav.live],
  ["#/agents", "agents", T.nav.agents],
  ["#/tokens", "tokens", T.nav.tokens],
  ["#/learned", "learned", T.nav.learned],
  ["#/blind", "blind", T.nav.blind],
  ["#/settings", "settings", T.nav.settings],
];
const nav = h("nav", { id: "nav" }, links.map(([href, name, text]) => h("a", { href, title: text }, icon(name), h("span", {}, text))));
const find = h("input", {
  type: "search",
  placeholder: T.find,
  onkeydown: (e: Event) => {
    const k = e as KeyboardEvent;
    const q = (k.target as HTMLInputElement).value.trim();
    if (k.key === "Enter" && q) go(`#/search/${enc(q)}`);
  },
});
const themes = h("div", { class: "themes", role: "group", "aria-label": T.theme.label });
function drawThemes(): void {
  mount(
    themes,
    ...THEMES.map((t) =>
      h(
        "button",
        {
          type: "button",
          class: t === theme ? "on" : "",
          title: T.theme[t],
          "aria-label": T.theme[t],
          "aria-pressed": String(t === theme),
          onclick: () => {
            if (t === theme) return;
            theme = t;
            localStorage.setItem(THEME_KEY, t);
            document.documentElement.dataset.theme = t;
            void call(["theme", t]);
            drawThemes();
            void route();
          },
        },
        icon(t),
      ),
    ),
  );
}
drawThemes();
mount(
  side,
  h("a", { class: "brand", href: "#/" }, h("img", { src: "icon.svg", alt: "" }), h("span", {}, T.app)),
  h("div", { class: "search" }, icon("search"), find, h("kbd", {}, "Ctrl K")),
  nav,
  status,
  themes,
);
window.addEventListener("keydown", (e) => {
  if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
    e.preventDefault();
    find.focus();
    find.select();
  }
});

function lit(href: string, hash: string): boolean {
  if (href === "#/") return hash === "#/" || hash.startsWith("#/l/");
  if (href === "#/projects") return hash === href || hash.startsWith("#/p/");
  return hash.startsWith(href);
}

function mark(hash: string): void {
  for (const a of nav.querySelectorAll("a")) a.classList.toggle("on", lit(a.getAttribute("href") ?? "", hash));
}

function listTab(name: string | undefined): ListTab {
  return TABS.find((t) => t === name) ?? "open";
}

type Page = (parts: string[]) => Promise<void | (() => void)>;
const NEEDS_ARG = new Set(["p", "n", "t", "search"]);
const rest = (parts: string[]) => parts.slice(1).join("/");
const PAGES: Record<string, Page> = {
  p: (p) => project(view, p[1], ((p[2] as Tab) || "notes") as Tab, listTab(p[3])),
  n: async (p) => (await import("./views/note")).note(view, { id: Number(p[1]) }),
  t: async (p) => (await import("./views/note")).note(view, { title: rest(p) }),
  search: (p) => search(view, rest(p)),
  projects: () => home(view),
  memory: () => memory(view),
  intelligence: () => intel(view),
  graph: async (p) => (await import("./views/graph")).graph(view, p[1] || undefined),
  live: () => live(view),
  agents: () => agents(view),
  tokens: () => tokens(view),
  learned: () => learned(view),
  blind: () => blind(view),
  settings: () => settings(view),
};

async function route(): Promise<void> {
  const hash = location.hash || "#/";
  const mine = ++turn;
  cleanup();
  cleanup = () => {};
  mark(hash);
  view.scrollTop = 0;
  const parts = hashParts(hash);
  try {
    const key = parts[0] ?? "";
    const page = PAGES[key] && (parts[1] || !NEEDS_ARG.has(key)) ? PAGES[key] : null;
    const done = page ? await page(parts) : await list(view, listTab(key === "l" ? parts[1] : undefined));
    if (mine === turn && done) cleanup = done;
    else if (done) done();
  } catch (e) {
    if (mine === turn) mount(view, h("div", { class: "page" }, h("div", { class: "error" }, String((e as Error).message ?? e))));
  }
}

async function pulse(): Promise<void> {
  const data = await board<Pulse>("pulse").catch(() => null);
  if (!data) {
    mount(status, h("span", {}, h("span", { class: "dot off" }), T.status.offline));
    return;
  }
  mount(
    status,
    h("span", {}, h("span", { class: "dot on" }), T.status.running),
    h("span", {}, T.status.working(data.working)),
    h("span", {}, T.status.queued(data.queued)),
    data.drafts ? h("span", {}, T.status.drafts(data.drafts)) : null,
    h("span", {}, T.status.notes(data.notes)),
    data.problem ? h("span", { class: "error", style: "padding:0" }, data.problem) : null,
  );
}

window.addEventListener("hashchange", () => void route());
void route();
void pulse();
setInterval(() => void pulse(), STATUS_MS);
