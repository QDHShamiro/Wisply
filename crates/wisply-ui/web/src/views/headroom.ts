import { board, type Room, type RoomRow } from "../api";
import { ago, h, mount } from "../dom";
import { T } from "../text";
import { pct, short, stat } from "./side";
import { took } from "./seen";

const RANGES = ["today", "7", "30", "all"] as const;
type Range = (typeof RANGES)[number];
const RANGE_KEY = "wisply.headroom";
const TOKEN_CHARS = 4;

function perDay(d: Room): HTMLElement {
  const top = Math.max(1, ...d.days.map((p) => p.saved));
  const cols = d.days.map((p) => {
    const day = p.name.slice(5).replace("-", "/");
    return h(
      "div",
      { class: "col", title: T.room.tip(day, short(p.tokens), p.n) },
      h("div", { class: "stack" }, h("div", { class: "saved", style: `height:${(100 * p.saved) / top}%` })),
      h("div", { class: "day" }, day),
    );
  });
  return h("section", { class: "panel" }, h("h2", { style: "margin:0" }, T.room.perDay), cols.length ? h("div", { class: "bars" }, cols) : h("div", { class: "faint" }, T.room.none));
}

function table(title: string, label: string, rows: RoomRow[], names: Record<string, string> = {}): HTMLElement {
  const most = Math.max(1, ...rows.map((r) => r.in));
  return h(
    "section",
    { class: "panel" },
    h("h2", { style: "margin-top:0" }, title),
    h(
      "div",
      { class: "krows" },
      h("div", { class: "krow head" }, h("span", {}, label), h("span", {}, T.room.times), h("span", {}, T.room.in), h("span", {}, T.room.out), h("span", {}, T.room.saved), h("span", {})),
      rows.map((r) =>
        h(
          "div",
          { class: "krow" },
          h("span", { class: "name" }, names[r.name] ?? r.name),
          h("span", {}, r.n.toLocaleString("en-US")),
          h("span", {}, short(r.in / TOKEN_CHARS)),
          h("span", { class: "faint" }, short(r.out / TOKEN_CHARS)),
          h("span", { class: r.saved ? "good" : "faint" }, r.saved ? `${short(r.tokens)} · ${pct(r.saved, r.in)}%` : "–"),
          h("span", { class: "meter" }, h("span", { class: "used", style: `width:${(100 * r.out) / most}%` }), r.saved ? h("span", { class: "saved", style: `width:${(100 * r.saved) / most}%` }) : null),
        ),
      ),
    ),
  );
}

function biggest(d: Room): HTMLElement | null {
  if (!d.top.length) return null;
  return h(
    "section",
    { class: "panel" },
    h("h2", { style: "margin-top:0" }, T.room.top),
    h(
      "div",
      { class: "list" },
      d.top.map((c) =>
        h(
          "div",
          { class: "item" },
          h("span", { class: "kind" }, c.tool),
          h("span", { class: "grow ellipsis" }, h("code", {}, c.ref ?? ""), h("span", { class: "muted" }, ` ${T.room.acts[c.act] ?? c.act}`)),
          h("span", { class: "good" }, short(c.saved / TOKEN_CHARS)),
          h("span", { class: "when" }, T.ago(ago(c.t / 1000))),
        ),
      ),
    ),
  );
}

export async function headroom(view: HTMLElement): Promise<void> {
  let range = (RANGES.find((r) => r === localStorage.getItem(RANGE_KEY)) ?? "7") as Range;
  const body = h("div", { class: "tokens" });
  const seg = h("div", { class: "seg" });
  const draw = async () => {
    localStorage.setItem(RANGE_KEY, range);
    mount(seg, ...RANGES.map((r) => h("button", { class: r === range ? "on" : "", onclick: () => { range = r; void draw(); } }, T.tokens.ranges[r])));
    mount(body, h("div", { class: "faint", style: "padding:24px 0" }, T.room.loading));
    let d: Room;
    try {
      d = await board<Room>("headroom", range);
    } catch (e) {
      mount(body, h("div", { class: "error" }, (e as Error).message));
      return;
    }
    if (!d.rows) {
      mount(body, h("div", { class: "empty" }, T.room.none));
      return;
    }
    mount(
      body,
      h(
        "div",
        { class: "stats" },
        stat(T.room.saved, short(d.tokens), T.room.savedNote(pct(d.saved, d.in)), "good"),
        stat(T.room.squeezed, d.squeezed.toLocaleString("en-US"), T.room.squeezedNote(d.rows)),
        stat(T.room.reloads, d.reloads.toLocaleString("en-US"), T.room.reloadsNote(d.reload_pct), d.reload_pct > 20 ? "bad" : ""),
        stat(T.room.speed, took(d.avg_ms), T.room.speedNote(d.errors), d.errors ? "bad" : ""),
      ),
      perDay(d),
      table(T.room.byAct, T.room.act, d.acts, T.room.acts),
      table(T.room.byTool, T.room.tool, d.tools),
      biggest(d),
      d.last_error ? h("div", { class: "error" }, T.room.lastError(d.last_error)) : null,
      h("p", { class: "footnote" }, T.room.how),
    );
  };
  mount(view, h("div", { class: "page" }, h("div", { class: "row head" }, h("div", { class: "grow" }, h("h1", {}, T.room.title), h("div", { class: "sub" }, T.room.sub)), seg), body));
  await draw();
}
