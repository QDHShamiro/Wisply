import { board, boardWith, type Change, type EntryFull } from "../api";
import { ago, h, mount, pick } from "../dom";
import { render } from "../md";
import { T } from "../text";

const KINDS = ["bug", "feature", "task", "idea"];
const PRIOS = ["high", "mid", "low"];
const STATUSES = ["todo", "doing", "waiting", "done"];
const WAITS = ["", "restart", "you"];

type Ctx = {
  e: EntryFull;
  set: (field: string, value: string) => Promise<void>;
  run: (work: () => Promise<unknown>) => Promise<void>;
  reload: () => void;
  close: () => void;
};

function when(t: number): string {
  return t ? T.ago(ago(t)) : "–";
}

function choices(values: string[], labels: Record<string, string>) {
  return values.map((v) => ({ value: v, label: labels[v] ?? v }));
}

function field(label: string, control: Node): HTMLElement {
  return h("div", { class: "field-row" }, h("span", { class: "label" }, label), control);
}

function taskBox(c: Ctx): HTMLTextAreaElement {
  const task = h("textarea", { class: "title", rows: 2, spellcheck: "true", "aria-label": T.list.cols.task });
  task.value = c.e.task;
  task.addEventListener("keydown", (k) => {
    if (k.key === "Enter" && !k.shiftKey) {
      k.preventDefault();
      task.blur();
    } else if (k.key === "Escape") {
      task.value = c.e.task;
      task.blur();
    }
  });
  task.addEventListener("change", () => {
    const v = task.value.trim();
    if (v && v !== c.e.task) void c.set("task", v);
  });
  return task;
}

function fields(c: Ctx): HTMLElement {
  const e = c.e;
  const project = h("input", { type: "text", value: e.project, placeholder: T.list.newProject, "aria-label": T.entry.project });
  project.addEventListener("change", () => void c.set("project", project.value.trim()));
  const level = e.level ? h("span", { class: `pill ${e.level === "warn" ? "warn" : "err"}` }, e.level) : null;
  return h(
    "div",
    { class: "fields" },
    field(T.entry.status, pick(choices(STATUSES, T.list.statuses), e.status === "suggested" ? "todo" : e.status, (v) => void c.set("status", v))),
    e.status === "waiting" ? field(T.entry.wait, pick(choices(WAITS, T.list.waits), e.wait, (v) => void c.set("wait", v))) : null,
    field(T.entry.kind, pick(choices(KINDS, T.list.kinds), e.kind, (v) => void c.set("kind", v))),
    field(T.entry.priority, pick(choices(PRIOS, T.list.prios), e.priority, (v) => void c.set("priority", v))),
    field(T.entry.project, project),
    e.who ? field(T.entry.who, h("span", {}, e.who)) : null,
    field(T.entry.source, h("span", {}, T.entry.sources[e.source] ?? e.source)),
    e.server ? field(T.entry.server, h("span", { class: "row" }, e.server, level)) : null,
    e.server ? h("div", { class: "faint small" }, T.entry.seen(when(e.first_seen), when(e.last_seen), e.count)) : null,
  );
}

function bodySection(c: Ctx): HTMLElement[] {
  const box = h("div", {});
  const show = () => {
    const md = h("div", { class: "md" });
    if (c.e.body.trim()) md.innerHTML = render(c.e.body);
    else mount(md, h("span", { class: "faint" }, T.entry.noBody));
    mount(box, md);
  };
  const edit = () => {
    const area = h("textarea", { class: "body-edit", rows: 10, spellcheck: "true" });
    area.value = c.e.body;
    const buttons = h("div", { class: "row" }, h("button", { class: "primary", onclick: () => void c.set("body", area.value) }, T.entry.save), h("button", { onclick: show }, T.entry.cancel));
    mount(box, area, buttons);
    area.focus();
  };
  show();
  return [h("h2", { class: "row" }, h("span", { class: "grow" }, T.entry.body), h("button", { class: "ghost", onclick: edit }, T.entry.edit)), box];
}

function noteSection(c: Ctx): (HTMLElement | null)[] {
  const input = h("input", { type: "text", placeholder: T.entry.addNote });
  input.addEventListener("keydown", (k) => {
    const v = input.value.trim();
    if (k.key === "Enter" && v) void c.run(() => board("entry-note", String(c.e.id), `--value=${v}`));
  });
  return [h("h2", {}, T.entry.notes), c.e.note.trim() ? h("div", { class: "notes" }, c.e.note.trim()) : null, input];
}

function status(v: string, f: string): string {
  return f === "status" ? (T.list.statuses[v] ?? v) : v;
}

function change(c: Change): HTMLElement {
  const diff = c.field === "created" ? null : h("span", { class: "change" }, " ", c.old ? h("s", {}, status(c.old, c.field)) : null, " → ", status(c.new, c.field) || "–");
  return h("li", { class: c.undone ? "undone" : "" }, h("span", { class: "who" }, c.who || "–"), " ", T.entry.field(c.field), diff, h("span", { class: "when" }, when(c.at)));
}

function historySection(c: Ctx, say: (text: string) => void): HTMLElement[] {
  const undo = h("button", {
    onclick: () =>
      void c.run(async () => {
        const r = await board<{ undone: number }>("entry-undo", String(c.e.id));
        say(T.entry.undone(r.undone));
      }),
  }, T.entry.undo);
  return [h("h2", { class: "row" }, h("span", { class: "grow" }, T.entry.history), undo), h("ul", { class: "history" }, c.e.history.map(change))];
}

function footer(c: Ctx): HTMLElement {
  const e = c.e;
  let mute: HTMLElement | null = null;
  if (e.fingerprint && e.muted) mute = h("div", { class: "faint small" }, T.entry.ignored);
  else if (e.fingerprint) mute = h("button", { onclick: () => void c.run(() => board("mute", String(e.id))) }, T.entry.ignore);
  return h("div", { class: "row end" }, mute, h("span", { class: "grow" }), h("button", { class: "danger", onclick: () => void c.set("status", "dropped") }, T.entry.drop));
}

let said = "";

export async function entryPanel(host: HTMLElement, id: number, changed: () => void): Promise<void> {
  host.classList.add("open");
  let e: EntryFull;
  try {
    e = await board<EntryFull>("entry", String(id));
  } catch (err) {
    mount(host, h("div", { class: "drawer-in" }, h("div", { class: "error" }, (err as Error).message)));
    return;
  }
  const state = h("div", { class: `state ${said ? "ok" : ""}` }, said);
  said = "";
  const fail = (err: unknown) => {
    state.className = "state bad";
    state.textContent = (err as Error).message;
  };
  const reload = () => void entryPanel(host, id, changed);
  const run = async (work: () => Promise<unknown>) => {
    try {
      await work();
      changed();
      reload();
    } catch (err) {
      fail(err);
    }
  };
  const set = (f: string, value: string) =>
    run(() => (f === "body" ? boardWith(value, "entry-set", String(id), "body") : board("entry-set", String(id), f, `--value=${value}`)));
  const close = () => {
    host.classList.remove("open");
    mount(host);
  };
  const c: Ctx = { e, set, run, reload, close };
  const top = h("div", { class: "row" }, h("span", { class: "faint mono grow" }, `#${e.id}`), h("button", { class: "ghost", title: T.entry.close, "aria-label": T.entry.close, onclick: close }, "×"));
  const log = e.excerpt.trim() ? [h("h2", {}, T.entry.log), h("pre", { class: "out log" }, e.excerpt)] : [];
  mount(
    host,
    h("div", { class: "drawer-in" }, top, taskBox(c), fields(c), state, bodySection(c), log, noteSection(c), historySection(c, (t) => (said = t)), footer(c)),
  );
}
