import { board, type Learning, type Seen, type Skill } from "../api";
import { ago, enc, h, mount } from "../dom";
import { T } from "../text";

const USES_SHOWN = 8;

export function took(ms: number): string {
  if (ms < 1000) return `${Math.round(ms)} ms`;
  if (ms < 60_000) return `${(ms / 1000).toFixed(1)} s`;
  return `${(ms / 60_000).toFixed(1)} min`;
}

function fact(label: string, value: Node | string | null): HTMLElement | null {
  if (!value) return null;
  return h("div", { class: "fact" }, h("span", { class: "faint" }, `${label} `), value);
}

function folder(cwd: string): string {
  return cwd.split(/[\\/]/).filter(Boolean).pop() ?? "";
}

function where(project: string, title: string, session: string, cwd = ""): (Node | string)[] {
  const place = project ? "" : folder(cwd);
  const who = title || [place, session.slice(0, 8)].filter(Boolean).join(" · ");
  return project ? [h("a", { href: `#/p/${enc(project)}`, style: "display:inline;padding:0" }, project), ` · ${who}`] : [who];
}

export function seenPanel(s: Seen | undefined): (HTMLElement | null)[] {
  if (!s) return [];
  const o = s.origin;
  const origin = [
    h("h2", {}, T.seen.origin),
    o ? fact(T.seen.session, h("span", { title: o.cwd }, ...where(o.project, o.title, o.session, o.cwd))) : h("div", { class: "faint" }, T.seen.noSession),
    fact(T.seen.ai, o?.ai ?? ""),
    fact(T.seen.via, s.via),
    fact(T.seen.source, [s.source, s.kind].filter(Boolean).join(" · ")),
    fact(T.seen.created, s.created ? T.ago(ago(s.created)) : ""),
    s.files.length ? fact(T.seen.files, h("span", {}, s.files.map((f) => h("code", { class: "file" }, f)))) : null,
  ];
  const counts = s.counts.map((c) => T.seen.count(T.seen.how[c.how] ?? c.how, c.sessions, c.times));
  const uses = s.uses.slice(0, USES_SHOWN).map((u) =>
    h(
      "div",
      { class: "fact", title: [u.ai, u.cwd].filter(Boolean).join(" · ") },
      h("span", { class: "chip" }, T.seen.how[u.how] ?? u.how),
      " ",
      ...where(u.project, u.title, u.session, u.cwd),
      h("span", { class: "faint" }, ` · ${T.ago(ago(u.t))}`),
    ),
  );
  return [
    ...origin,
    h("h2", {}, T.seen.used),
    counts.length ? h("div", { class: "muted" }, counts.join(" · ")) : h("div", { class: "faint" }, T.seen.never),
    ...uses,
  ];
}

export function learningBlock(): { el: HTMLElement; draw: () => Promise<number> } {
  const el = h("section", { class: "panel" });
  const draw = async () => {
    const d = await board<Learning>("learning").catch(() => null);
    if (!d) return 0;
    const head = h("h2", { style: "margin-top:0" }, T.learning.title);
    if (!d.on) {
      mount(el, head, h("div", { class: "faint" }, T.learning.off));
      return d.poll_ms;
    }
    const now = d.now
      ? h(
          "div",
          { class: "item" },
          h("span", { class: "pill on" }, T.live.working),
          h("span", { class: "grow ellipsis" }, ...where(d.now.project, d.now.title, d.now.session)),
          h("span", { class: "cmd" }, T.learning.now(d.now.part, d.now.via)),
          h("span", { class: "when" }, T.ago(ago(d.now.since))),
        )
      : h("div", { class: "faint" }, T.learning.idle(d.queued));
    const done = d.done.map((x) =>
      h(
        "div",
        { class: "item" },
        h("span", { class: "pill" }, T.learning.added(x.added)),
        h("span", { class: "grow ellipsis" }, ...where(x.project, x.title, x.session)),
        h("span", { class: "when" }, T.ago(ago(x.t))),
      ),
    );
    mount(
      el,
      head,
      h("div", { class: "list live" }, now, ...done),
      d.error ? h("div", { class: "error", style: "padding:6px 0" }, d.error) : null,
      d.chain.length ? h("div", { class: "faint", style: "padding-top:8px" }, `${T.learning.chain}: ${d.chain.map((c) => c.trim()).join(" · ")}`) : null,
    );
    return d.poll_ms;
  };
  return { el, draw };
}

function skillRow(s: Skill): HTMLElement {
  const runs = s.runs ?? 0;
  const hand = s.before_ms ? took(s.before_ms) : "";
  const facts = [
    T.skills.runs(s.ok ?? 0, runs),
    runs && s.ms ? T.skills.speed(took(s.ms), hand) : "",
    s.saved ? T.skills.saved(String(s.saved)) : "",
  ].filter(Boolean);
  return h(
    "div",
    { class: "item todo", title: s.hint ? T.skills.hint(s.hint) : "" },
    h("span", { class: "kind" }, s.state),
    h(
      "span",
      { class: "grow" },
      h("code", {}, `wisply run ${s.name}${s.params.map((p) => ` ${p}=`).join("")}`),
      h("span", { class: "muted" }, ` ${s.about}`),
      s.default ? h("span", { class: "chip", style: "margin-left:8px" }, T.skills.default) : null,
      s.weak ? h("span", { class: "pill err", style: "margin-left:8px" }, T.skills.weak) : null,
    ),
    h("span", { class: "when" }, facts.join(" · ")),
  );
}

export async function skillsSection(): Promise<HTMLElement> {
  const { skills } = await board<{ skills: Skill[] }>("skills").catch(() => ({ skills: [] as Skill[] }));
  return h(
    "div",
    {},
    h("h2", {}, T.skills.title),
    h("div", { class: "sub" }, T.skills.sub),
    h("div", { class: "list" }, skills.length ? skills.map(skillRow) : h("div", { class: "empty" }, T.skills.none)),
  );
}
