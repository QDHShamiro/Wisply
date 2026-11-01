import { board, call, plain, type TokenData } from "../api";
import { h, mount, pick } from "../dom";
import { T } from "../text";

type Lesson = { sig: string; cmd: string; target: string; kind: string; fix: string; ups: number; downs: number };
type Field = { key: string; kind: "bool" | "text" | "pick" | "path"; options?: string[] };

const FIELDS: Field[] = [
  { key: "gemini", kind: "bool" },
  { key: "gemini_model", kind: "text" },
  { key: "memory", kind: "bool" },
  { key: "memory_learn", kind: "bool" },
  { key: "learn", kind: "bool" },
  { key: "brain", kind: "bool" },
  { key: "cleanup", kind: "bool" },
  { key: "pointer", kind: "bool" },
  { key: "window", kind: "bool" },
  { key: "focus", kind: "pick", options: ["never", "away", "idle"] },
  { key: "browser", kind: "pick", options: ["auto", "brave", "chrome", "edge"] },
  { key: "vault", kind: "path" },
  { key: "vault_inbox", kind: "path" },
];

const RANGES = ["today", "7", "30", "all"] as const;
type Range = (typeof RANGES)[number] | "claude";
const RANGE_KEY = "wisply.tokens";

function short(n: number): string {
  for (const [size, unit] of [[1e9, "B"], [1e6, "M"], [1e3, "k"]] as const) {
    if (n >= size) return `${(n / size).toFixed(n >= 10 * size ? 0 : 1)}${unit}`;
  }
  return String(Math.round(n));
}

function pct(part: number, whole: number): number {
  return whole > 0 ? Math.round((100 * part) / whole) : 0;
}

function stat(label: string, value: string, note: string, tone = ""): HTMLElement {
  return h("div", { class: `stat ${tone}` }, h("div", { class: "label" }, label), h("div", { class: "value" }, value), h("div", { class: "note" }, note));
}

function days(d: TokenData, range: Range): TokenData["series"] {
  const n = range === "today" ? 1 : Number(range);
  if (!d.daily || !n) return d.series;
  const have = new Map(d.series.map((p) => [p.date, p]));
  const out: TokenData["series"] = [];
  for (let i = n - 1; i >= 0; i--) {
    const t = new Date();
    t.setDate(t.getDate() - i);
    const date = `${t.getFullYear()}-${String(t.getMonth() + 1).padStart(2, "0")}-${String(t.getDate()).padStart(2, "0")}`;
    out.push(have.get(date) ?? { date, used: 0, saved: 0 });
  }
  return out;
}

function chart(d: TokenData, range: Range): HTMLElement {
  const series = days(d, range);
  const top = Math.max(1, ...series.map((p) => p.used + p.saved));
  const cols = series.map((p) => {
    const day = p.date.slice(5).replace("-", "/");
    return h(
      "div",
      { class: "col", title: T.tokens.tip(day, short(p.used), short(p.saved)) },
      h("div", { class: "stack" }, h("div", { class: "saved", style: `height:${(100 * p.saved) / top}%` }), h("div", { class: "used", style: `height:${(100 * p.used) / top}%` })),
      h("div", { class: "day" }, day),
    );
  });
  return h(
    "section",
    { class: "panel" },
    h("div", { class: "row" }, h("h2", { class: "grow", style: "margin:0" }, d.daily ? T.tokens.perDay : T.tokens.perWeek), h("span", { class: "key saved" }, T.tokens.saved), h("span", { class: "key used" }, T.tokens.used)),
    cols.length ? h("div", { class: "bars" }, cols) : h("div", { class: "faint" }, T.tokens.none),
  );
}

function kinds(d: TokenData): HTMLElement {
  const most = Math.max(1, ...d.groups.map((g) => g.without ?? g.used));
  return h(
    "section",
    { class: "panel" },
    h("h2", { style: "margin-top:0" }, T.tokens.byKind),
    h(
      "div",
      { class: "krows" },
      h("div", { class: "krow head" }, h("span", {}, T.tokens.kind), h("span", {}, T.tokens.runs), h("span", {}, T.tokens.used), h("span", {}, T.tokens.without), h("span", {}, T.tokens.saved), h("span", {})),
      d.groups.map((g) =>
        h(
          "div",
          { class: "krow" },
          h("span", { class: "name" }, g.name.charAt(0).toUpperCase() + g.name.slice(1)),
          h("span", {}, g.runs.toLocaleString("en-US")),
          h("span", {}, short(g.used)),
          h("span", { class: "faint" }, g.without === null ? "–" : short(g.without)),
          h("span", { class: g.saved ? "good" : "faint" }, g.saved === null ? "–" : `${short(g.saved)} · ${pct(g.saved, g.without ?? 0)}%`),
          h("span", { class: "meter" }, h("span", { class: "used", style: `width:${(100 * g.used) / most}%` }), g.saved ? h("span", { class: "saved", style: `width:${(100 * g.saved) / most}%` }) : null),
        ),
      ),
    ),
  );
}

function extras(d: TokenData): HTMLElement | null {
  const lines = [
    d.routines ? T.tokens.routines(d.routines.runs, short(d.routines.saved)) : "",
    d.brain ? T.tokens.brain(d.brain.snaps, short(d.brain.saved)) : "",
    d.trimmed ? T.tokens.trimmed(short(d.trimmed.tokens), d.trimmed.since?.slice(5) ?? "") : "",
  ].filter(Boolean);
  if (!d.top.length && !lines.length) return null;
  return h(
    "section",
    { class: "panel" },
    d.top.length ? h("h2", { style: "margin-top:0" }, T.tokens.top) : null,
    d.top.length ? h("div", { class: "chips" }, d.top.map((t) => h("span", { class: "chip" }, h("code", {}, t.cmd), " ", h("span", { class: "good" }, short(t.saved))))) : null,
    lines.length ? h("ul", { class: "facts" }, lines.map((l) => h("li", {}, l))) : null,
  );
}

export async function tokens(view: HTMLElement): Promise<void> {
  let range = (localStorage.getItem(RANGE_KEY) as Range | null) ?? "7";
  const body = h("div", { class: "tokens" });
  const seg = h("div", { class: "seg" });
  const draw = async () => {
    localStorage.setItem(RANGE_KEY, range);
    mount(
      seg,
      ...[...RANGES, "claude" as const].map((r) =>
        h(
          "button",
          {
            class: r === range ? "on" : "",
            onclick: () => {
              range = r;
              void draw();
            },
          },
          T.tokens.ranges[r],
        ),
      ),
    );
    mount(body, h("div", { class: "faint", style: "padding:24px 0" }, range === "claude" ? T.tokens.slow : T.tokens.loading));
    if (range === "claude") {
      const text = await plain("tokens", "claude", "7").catch((e: Error) => e.message);
      mount(body, h("pre", { class: "out" }, text));
      return;
    }
    let d: TokenData;
    try {
      d = await board<TokenData>("tokens", range);
    } catch (e) {
      mount(body, h("div", { class: "error" }, (e as Error).message));
      return;
    }
    if (!d.runs) {
      mount(body, h("div", { class: "empty" }, T.tokens.none));
      return;
    }
    mount(
      body,
      h(
        "div",
        { class: "stats" },
        stat(T.tokens.saved, short(d.saved), T.tokens.savedNote(pct(d.saved, d.without)), "good"),
        stat(T.tokens.used, short(d.used), T.tokens.usedNote),
        stat(T.tokens.without, short(d.without), T.tokens.withoutNote),
        stat(T.tokens.commands, d.runs.toLocaleString("en-US"), d.title.replace(/^Wisply tokens, /, "")),
      ),
      chart(d, range),
      kinds(d),
      extras(d),
      h("p", { class: "footnote" }, T.tokens.how(d.page.toLocaleString("en-US"), d.screenshot.toLocaleString("en-US"), d.video.toLocaleString("en-US"))),
    );
  };
  mount(view, h("div", { class: "page" }, h("div", { class: "row head" }, h("div", { class: "grow" }, h("h1", {}, T.tokens.title), h("div", { class: "sub" }, T.tokens.sub)), seg), body));
  await draw();
}

export async function learned(view: HTMLElement): Promise<void> {
  const { lessons } = await board<{ lessons: Lesson[] }>("lessons");
  const redraw = () => void learned(view);
  mount(
    view,
    h(
      "div",
      { class: "page" },
      h("h1", {}, T.learned.title),
      h("div", { class: "sub" }, T.learned.sub),
      h(
        "div",
        { class: "list" },
        lessons.length
          ? lessons.map((l) =>
              h(
                "div",
                { class: "item todo" },
                h("span", { class: "kind" }, l.cmd),
                h("span", { class: "grow" }, l.target ? `${l.target}: ` : "", l.kind, h("span", { class: "muted" }, " → "), h("code", {}, `wisply ${l.fix}`)),
                h("span", { class: "when" }, T.learned.score(l.ups, l.downs)),
                h("span", { class: "acts" }, h("button", { class: "danger", onclick: async () => { await board("unlearn", l.sig); redraw(); } }, T.learned.forget)),
              ),
            )
          : h("div", { class: "empty" }, T.learned.none),
      ),
    ),
  );
}

export async function settings(view: HTMLElement): Promise<void> {
  const cfg = await board<Record<string, unknown>>("settings");
  const row = (f: Field) => {
    const v = cfg[f.key];
    const state = h("div", { class: "state" });
    const say = (text: string, ok: boolean) => {
      state.className = `state ${ok ? "ok" : "bad"}`;
      state.textContent = text;
    };
    const set = async (value: string, input?: HTMLInputElement) => {
      try {
        await plain("config", f.key, value);
        input?.classList.remove("bad");
        say(T.settings.saved, true);
      } catch (e) {
        input?.classList.add("bad");
        say((e as Error).message, false);
      }
    };
    const head = h("div", { class: "grow" }, h("div", {}, T.settings.labels[f.key] ?? f.key), T.settings.help[f.key] ? h("div", { class: "help" }, T.settings.help[f.key]) : null, h("div", { class: "key" }, f.key));
    if (f.kind === "bool") {
      const box = h("input", { type: "checkbox", checked: v === true, "aria-label": T.settings.labels[f.key] ?? f.key, onchange: (e: Event) => void set((e.target as HTMLInputElement).checked ? "on" : "off") });
      return h("div", { class: "setting" }, h("div", { class: "grow" }, head, state), box);
    }
    if (f.kind === "pick") {
      const choices = (f.options ?? []).map((o) => ({ value: o, label: T.settings.options[`${f.key}.${o}`] ?? o }));
      return h("div", { class: "setting" }, h("div", { class: "grow" }, head, state), pick(choices, String(v ?? ""), (value) => void set(value)));
    }
    const input = h("input", {
      type: "text",
      value: String(v ?? ""),
      spellcheck: "false",
      placeholder: T.settings.placeholder[f.key] ?? "",
      "aria-label": T.settings.labels[f.key] ?? f.key,
      onkeydown: (e: Event) => {
        if ((e as KeyboardEvent).key === "Enter") (e.target as HTMLInputElement).blur();
      },
      onchange: (e: Event) => void set((e.target as HTMLInputElement).value.trim(), e.target as HTMLInputElement),
    });
    const browse =
      f.kind === "path"
        ? h(
            "button",
            {
              type: "button",
              onclick: async () => {
                const r = await call(["pick-folder", input.value]);
                if (r.code !== 0 || !r.out) return;
                input.value = r.out;
                await set(r.out, input);
              },
            },
            T.settings.browse,
          )
        : null;
    return h("div", { class: "setting stack" }, head, h("div", { class: "field" }, input, browse), state);
  };
  mount(view, h("div", { class: "page" }, h("h1", {}, T.settings.title), h("div", { class: "sub" }, T.settings.sub), h("div", { class: "list" }, FIELDS.map(row))));
}
