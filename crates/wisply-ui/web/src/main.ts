import { board, call, plain, type Pulse, type Side } from "./api";
import { enc, go, h, icon, mount } from "./dom";
import { hashParts } from "./hash";
import { T } from "./text";
import { agents } from "./views/agents";
import { blind } from "./views/blind";
import { headroom } from "./views/headroom";
import { home } from "./views/home";
import { improve } from "./views/improve";
import { badgeOf, findPrompt, groupLinks } from "./views/insight-logic";
import { intel } from "./views/intel";
import { list, type ListTab, TABS } from "./views/list";
import { live } from "./views/live";
import { memory } from "./views/memory";
import { project, type Tab } from "./views/project";
import { search } from "./views/search";
import { learned, settings, tokens } from "./views/side";
import { speed } from "./views/speed";

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
  ["#/improve", "improve", T.nav.improve],
  ["#/speed", "speed", T.nav.speed],
  ["#/learned", "learned", T.nav.learned],
  ["#/blind", "blind", T.nav.blind],
  ["#/settings", "settings", T.nav.settings],
];
const nav = h("nav", { id: "nav" });
const badges = new Map<string, HTMLElement>();
function drawNav(): void {
  mount(
    nav,
    ...groupLinks(links).flatMap((g) => [
      h("div", { class: "group" }, T.groups[g.title as keyof typeof T.groups] ?? g.title),
      ...g.links.map(([href, name, text]) => {
        const badge = h("em", { class: "badge", hidden: "" });
        badges.set(name, badge);
        return h("a", { href, title: text }, icon(name), h("span", {}, text), badge);
      }),
    ]),
  );
}
drawNav();
const nowBox = h("div", { id: "now" });
const light = h("div", { id: "light" });
const actions = h("div", { id: "actions" });
let sideData: Side | null = null;
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
  nowBox,
  actions,
  light,
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
  headroom: () => headroom(view),
  improve: () => improve(view),
  speed: () => speed(view),
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

const remember = h("input", {
  type: "text",
  placeholder: T.side.remember,
  onkeydown: async (e: Event) => {
    const k = e as KeyboardEvent;
    const input = k.target as HTMLInputElement;
    const text = input.value.trim();
    if (k.key !== "Enter" || !text) return;
    input.value = "";
    input.placeholder = await plain("remember", text).then(() => T.side.remembered).catch((x: Error) => x.message);
    window.setTimeout(() => (input.placeholder = T.side.remember), 2500);
  },
});
const finder = h("button", {
  type: "button",
  onclick: async () => {
    const ok = await navigator.clipboard.writeText(findPrompt()).then(() => true, () => false);
    finder.textContent = ok ? T.side.findDone : T.side.findFail;
    window.setTimeout(() => (finder.textContent = T.side.find), 2500);
    go("#/improve");
  },
}, T.side.find);
const pauser = h("button", {
  type: "button",
  onclick: async () => {
    const next = sideData?.memory === false ? "on" : "off";
    await plain("config", "memory", next).catch(() => "");
    void sidebar();
  },
});
mount(actions, h("div", { class: "group" }, T.side.actions), finder, remember, pauser);

function drawSide(data: Side | null): void {
  sideData = data;
  for (const [name, el] of badges) {
    const n = badgeOf(name, data);
    el.textContent = String(n);
    el.hidden = n === 0;
  }
  if (!data) {
    mount(nowBox);
    mount(light);
    return;
  }
  pauser.textContent = data.memory ? T.side.pause : T.side.resume;
  mount(
    nowBox,
    h("div", { class: "group" }, T.side.live),
    ...(data.live.length
      ? data.live.map((r) => h("a", { class: "run", href: "#/live", title: r.title ?? r.cmd }, h("span", { class: "dot on" }), h("span", {}, r.title ?? r.cmd), r.project ? h("small", {}, r.project) : null))
      : [h("div", { class: "faint idle" }, T.side.idle)]),
    data.working > data.live.length ? h("div", { class: "faint idle" }, T.side.working(data.working)) : null,
  );
  mount(light, h("a", { href: "#/speed", class: `lamp ${data.light.tone}`, title: T.side.lightNote(data.light.err_pct, data.light.avg_ms, data.light.runs) }, h("span", { class: "dot" }), h("span", {}, T.side.light[data.light.tone])));
}

async function sidebar(): Promise<void> {
  drawSide(await board<Side>("side").catch(() => null));
}

window.addEventListener("hashchange", () => void route());
void route();
void pulse();
void sidebar();
setInterval(() => {
  void pulse();
  void sidebar();
}, STATUS_MS);
