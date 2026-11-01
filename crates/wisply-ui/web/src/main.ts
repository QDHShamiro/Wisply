import { board, call, type Pulse } from "./api";
import { enc, go, h, icon, mount } from "./dom";
import { T } from "./text";
import { blind } from "./views/blind";
import { home } from "./views/home";
import { live } from "./views/live";
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
  ["#/", "projects", T.nav.projects],
  ["#/graph", "graph", T.nav.graph],
  ["#/live", "live", T.nav.live],
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

function mark(hash: string): void {
  for (const a of nav.querySelectorAll("a")) {
    const href = a.getAttribute("href") ?? "";
    a.classList.toggle("on", href === "#/" ? hash === "#/" || hash.startsWith("#/p/") : hash.startsWith(href));
  }
}

async function route(): Promise<void> {
  const hash = location.hash || "#/";
  const mine = ++turn;
  cleanup();
  cleanup = () => {};
  mark(hash);
  view.scrollTop = 0;
  const parts = hash.slice(2).split("?")[0].split("/").map(decodeURIComponent);
  try {
    let done: void | (() => void);
    if (parts[0] === "p" && parts[1]) done = await project(view, parts[1], ((parts[2] as Tab) || "notes") as Tab);
    else if (parts[0] === "n" && parts[1]) done = await (await import("./views/note")).note(view, { id: Number(parts[1]) });
    else if (parts[0] === "t" && parts[1]) done = await (await import("./views/note")).note(view, { title: parts.slice(1).join("/") });
    else if (parts[0] === "graph") done = await (await import("./views/graph")).graph(view, parts[1] || undefined);
    else if (parts[0] === "live") done = await live(view);
    else if (parts[0] === "tokens") done = await tokens(view);
    else if (parts[0] === "learned") done = await learned(view);
    else if (parts[0] === "blind") done = await blind(view);
    else if (parts[0] === "settings") done = await settings(view);
    else if (parts[0] === "search" && parts[1]) done = await search(view, parts.slice(1).join("/"));
    else done = await home(view);
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
