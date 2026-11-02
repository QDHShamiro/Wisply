import { h, mount } from "../dom";
import { T } from "../text";
import { type Agent, type Detail, clock, modelLabel, say, show, stop, tokens } from "./agent-data";

const TAIL_MS = 2500;
const ARM_MS = 4000;

export function detail(agent: Agent, onClose: () => void): { el: HTMLElement; close: () => void } {
  const out = h("div", { class: "ag-out", tabindex: "0" });
  const meta = h("div", { class: "ag-meta" });
  const note = h("div", { class: "ag-note" });
  const input = h("input", { type: "text", placeholder: T.agents.say, maxlength: "4000" }) as HTMLInputElement;
  const send = h("button", { type: "button", class: "primary" }, T.agents.send) as HTMLButtonElement;
  const stopBtn = h("button", { type: "button", class: "danger" }, T.agents.stop) as HTMLButtonElement;
  const result = h("pre", { class: "ag-result", hidden: true });
  const tabLive = h("button", { type: "button", class: "ag-chipbtn on" }, T.agents.live);
  const tabResult = h("button", { type: "button", class: "ag-chipbtn", hidden: true }, T.agents.result);
  const tabs = h("div", { class: "ag-chips" }, tabLive, tabResult);
  const showTab = (which: "live" | "result") => {
    out.hidden = which !== "live";
    result.hidden = which !== "result";
    tabLive.classList.toggle("on", which === "live");
    tabResult.classList.toggle("on", which === "result");
  };
  tabLive.addEventListener("click", () => showTab("live"));
  tabResult.addEventListener("click", () => showTab("result"));
  let alive = true;
  let seenResult = false;
  let armed = 0;
  let last: string[] = [];

  const paint = (d: Detail) => {
    mount(
      meta,
      h("span", { class: `ag-dot s-${d.status}` }),
      h("b", {}, d.name || d.dir_short),
      h("span", { class: "ag-chip" }, d.dir_short),
      h("span", { class: "ag-chip" }, modelLabel(d)),
      h("span", { class: `pill s-${d.status}` }, d.status),
      h("span", { class: "ag-chip" }, `${tokens(d.tokens)} · $${d.cost_est.toFixed(2)}`),
      h("span", { class: "ag-chip" }, clock(Date.now() / 1000 - d.since)),
    );
    if (d.tail.join("\n") !== last.join("\n")) {
      const atEnd = out.scrollHeight - out.scrollTop - out.clientHeight < 40;
      const same = last.length > 0 && d.tail.slice(0, last.length).join("\n") === last.join("\n");
      if (same) out.append(...d.tail.slice(last.length).map((l) => h("div", { class: "ag-line fresh" }, l)));
      else mount(out, ...(d.tail.length ? d.tail.map((l) => h("div", { class: "ag-line" }, l)) : [h("div", { class: "ag-line" }, T.agents.noOutput)]));
      last = d.tail;
      if (atEnd || !same) out.scrollTop = out.scrollHeight;
    }
    if (d.result !== undefined && result.textContent !== d.result) result.textContent = d.result;
    const has = d.result !== undefined;
    tabResult.hidden = !has;
    if (has && !seenResult) showTab("result");
    seenResult = has;
    if (!has && out.hidden) showTab("live");
    const idle = d.status !== "running";
    send.disabled = !idle;
    input.disabled = !idle;
    stopBtn.disabled = d.status !== "running";
  };

  let busy = false;
  const pull = async () => {
    if (busy) return;
    busy = true;
    const d = await show(agent.id);
    busy = false;
    if (!alive) return;
    if (d) paint(d);
    else note.textContent = T.agents.gone;
  };

  const submit = async () => {
    const text = input.value.trim();
    if (!text || send.disabled) return;
    send.disabled = true;
    const err = await say(agent.id, text);
    note.textContent = err ?? T.agents.sent;
    if (!err) input.value = "";
    void pull();
  };
  send.addEventListener("click", () => void submit());
  input.addEventListener("keydown", (e) => {
    if (e.key === "Enter") void submit();
  });

  const disarm = () => {
    armed = 0;
    stopBtn.textContent = T.agents.stop;
    stopBtn.classList.remove("armed");
  };
  stopBtn.addEventListener("click", async () => {
    if (!armed) {
      armed = window.setTimeout(disarm, ARM_MS);
      stopBtn.textContent = T.agents.sure;
      stopBtn.classList.add("armed");
      return;
    }
    clearTimeout(armed);
    disarm();
    const err = await stop(agent.id);
    note.textContent = err ?? T.agents.stopped;
    void pull();
  });

  const back = h("button", { type: "button", class: "ag-back", onclick: () => onClose() }, "← ", T.agents.back);
  const el = h(
    "div",
    { class: "ag-detail" },
    h("div", { class: "row" }, back, h("div", { class: "grow" }), stopBtn),
    meta,
    tabs,
    out,
    result,
    h("div", { class: "row ag-send" }, input, send),
    note,
  );
  out.append(h("div", { class: "ag-line skel-line" }, T.agents.noOutput));
  paint({ ...agent, tail: [] });
  void pull();
  const timer = window.setInterval(() => void pull(), TAIL_MS);
  const key = (e: KeyboardEvent) => {
    if (e.key === "Escape" && document.activeElement !== input) onClose();
  };
  document.addEventListener("keydown", key);
  return {
    el,
    close: () => {
      alive = false;
      clearInterval(timer);
      clearTimeout(armed);
      document.removeEventListener("keydown", key);
    },
  };
}
