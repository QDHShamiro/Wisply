type Child = Node | string | number | null | undefined | false;
type Attrs = Record<string, string | number | boolean | ((e: Event) => void) | undefined>;

export function h<K extends keyof HTMLElementTagNameMap>(tag: K, attrs: Attrs = {}, ...kids: (Child | Child[])[]): HTMLElementTagNameMap[K] {
  const el = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs)) {
    if (v === undefined || v === false) continue;
    if (typeof v === "function") el.addEventListener(k.replace(/^on/, ""), v as EventListener);
    else if (v === true) el.setAttribute(k, "");
    else el.setAttribute(k, String(v));
  }
  for (const kid of kids.flat()) {
    if (kid === null || kid === undefined || kid === false) continue;
    el.append(kid instanceof Node ? kid : String(kid));
  }
  return el;
}

const ICONS: Record<string, string> = {
  projects: "M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z",
  graph: "M4 7a2 2 0 1 0 4 0a2 2 0 1 0-4 0M16 6a2 2 0 1 0 4 0a2 2 0 1 0-4 0M10 18a2 2 0 1 0 4 0a2 2 0 1 0-4 0M8 7l8-1M7 9l4 7M17 8l-4 8",
  live: "M3 12h4l3-7 4 14 3-7h4",
  tokens: "M5 20V10M12 20V4M19 20v-7",
  learned: "M4 5.5A2.5 2.5 0 0 1 6.5 3H20v14H6.5A2.5 2.5 0 0 0 4 19.5zM4 19.5A2.5 2.5 0 0 0 6.5 22H20v-5",
  blind: "M4 4h16v16H4zM12 4v16",
  settings: "M4 7h10M18 7h2M4 17h2M10 17h10M16 4v6M8 14v6",
  search: "M11 4a7 7 0 1 0 0 14a7 7 0 0 0 0-14zM20 20l-4-4",
};

export function icon(name: string): SVGElement {
  const ns = "http://www.w3.org/2000/svg";
  const svg = document.createElementNS(ns, "svg");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.setAttribute("fill", "none");
  svg.setAttribute("stroke", "currentColor");
  svg.setAttribute("stroke-linecap", "round");
  svg.setAttribute("stroke-linejoin", "round");
  svg.setAttribute("aria-hidden", "true");
  const path = document.createElementNS(ns, "path");
  path.setAttribute("d", ICONS[name] ?? "");
  svg.append(path);
  return svg;
}

export type Choice = { value: string; label: string; hint?: string };

export function pick(choices: Choice[], value: string, onPick: (value: string) => void): HTMLElement {
  let current = value;
  let active = Math.max(0, choices.findIndex((c) => c.value === value));
  const label = () => choices.find((c) => c.value === current)?.label ?? current;
  const text = h("span", {}, label());
  const list = h("ul", { role: "listbox", tabindex: "-1" });
  const root = h("div", { class: "pick" });
  const button = h("button", { type: "button", "aria-haspopup": "listbox", "aria-expanded": "false" }, text);
  const outside = (e: Event) => {
    if (!root.contains(e.target as Node)) close();
  };
  const draw = () =>
    mount(
      list,
      ...choices.map((c, i) =>
        h(
          "li",
          {
            role: "option",
            "aria-selected": String(c.value === current),
            class: i === active ? "active" : "",
            onmousemove: () => {
              if (active !== i) {
                active = i;
                draw();
              }
            },
            onmousedown: (e: Event) => {
              e.preventDefault();
              choose(i);
            },
          },
          c.label,
          c.hint ? h("span", { class: "hint" }, c.hint) : null,
        ),
      ),
    );
  const open = () => {
    if (root.classList.contains("open")) return;
    active = Math.max(0, choices.findIndex((c) => c.value === current));
    draw();
    root.append(list);
    root.classList.add("open");
    button.setAttribute("aria-expanded", "true");
    document.addEventListener("mousedown", outside);
  };
  function close(): void {
    list.remove();
    root.classList.remove("open");
    button.setAttribute("aria-expanded", "false");
    document.removeEventListener("mousedown", outside);
  }
  function choose(i: number): void {
    const c = choices[i];
    close();
    button.focus();
    if (!c || c.value === current) return;
    current = c.value;
    text.textContent = label();
    onPick(current);
  }
  button.addEventListener("click", () => (root.classList.contains("open") ? close() : open()));
  button.addEventListener("keydown", (e) => {
    const k = e.key;
    const isOpen = root.classList.contains("open");
    if (k === "ArrowDown" || k === "ArrowUp") {
      e.preventDefault();
      if (!isOpen) return open();
      active = (active + (k === "ArrowDown" ? 1 : choices.length - 1)) % choices.length;
      draw();
    } else if ((k === "Enter" || k === " ") && isOpen) {
      e.preventDefault();
      choose(active);
    } else if (k === "Escape" && isOpen) {
      e.preventDefault();
      close();
    } else if (k === "Tab" && isOpen) close();
  });
  root.append(button);
  return root;
}

export function mount(parent: HTMLElement, ...kids: Child[]): void {
  parent.replaceChildren(...kids.filter((k): k is Node | string => k !== null && k !== undefined && k !== false).map((k) => (typeof k === "number" ? String(k) : k)));
}

export function ago(t: number): number {
  return Math.max(0, Date.now() / 1000 - t);
}

export function go(hash: string): void {
  if (location.hash !== hash) location.hash = hash;
}

export function enc(s: string): string {
  return encodeURIComponent(s);
}

export function debounce<A extends unknown[]>(f: (...a: A) => void, ms: number): (...a: A) => void {
  let timer = 0;
  return (...a: A) => {
    clearTimeout(timer);
    timer = window.setTimeout(() => f(...a), ms);
  };
}
