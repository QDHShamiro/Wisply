import { h, mount } from "./dom";
import { T } from "./text";
import { TOKEN_KEY, pair } from "./views/agent-data";
import { agents } from "./views/agents";

const view = document.getElementById("view")!;

function gate(failed: boolean): void {
  const input = h("input", { type: "text", placeholder: T.agents.pairCode, autocomplete: "off", autocapitalize: "off", spellcheck: false }) as HTMLInputElement;
  const note = h("div", { class: "ag-note" }, failed ? T.agents.pairFail : "");
  const go = async () => {
    const code = input.value.trim();
    if (!code) return;
    if (await pair(code)) location.reload();
    else note.textContent = T.agents.pairFail;
  };
  input.addEventListener("keydown", (e) => {
    if (e.key === "Enter") void go();
  });
  mount(
    view,
    h("div", { class: "page ag-page ag-pair" }, h("h1", {}, T.agents.pairTitle), h("div", { class: "sub" }, T.agents.pairHelp), h("div", { class: "row ag-send" }, input, h("button", { type: "button", class: "primary", onclick: () => void go() }, T.agents.pairGo)), note),
  );
}

async function start(): Promise<void> {
  const code = new URLSearchParams(location.hash.slice(1)).get("pair");
  if (code) {
    history.replaceState(null, "", location.pathname);
    if (!(await pair(code))) return gate(true);
  }
  if (!localStorage.getItem(TOKEN_KEY)) return gate(false);
  await agents(view);
}

void start();
