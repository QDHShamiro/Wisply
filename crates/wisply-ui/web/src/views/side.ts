import { board, call, plain } from "../api";
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

export async function tokens(view: HTMLElement): Promise<void> {
  const out = h("pre", { class: "out" });
  const days = (n: string) => async () => {
    out.textContent = T.tokens.loading;
    out.textContent = await plain("tokens", n).catch((e: Error) => e.message);
  };
  const claude = h(
    "button",
    {
      onclick: async () => {
        out.textContent = T.tokens.slow;
        out.textContent = await plain("tokens", "claude", "7").catch((e: Error) => e.message);
      },
    },
    T.tokens.claude,
  );
  mount(
    view,
    h(
      "div",
      { class: "page" },
      h("h1", {}, T.tokens.title),
      h("div", { class: "sub" }, T.tokens.sub),
      h("div", { class: "row", style: "margin-bottom:14px" }, h("button", { onclick: days("1") }, T.tokens.today), h("button", { onclick: days("7") }, T.tokens.week), h("button", { onclick: days("30") }, T.tokens.month), claude),
      out,
    ),
  );
  await days("7")();
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
