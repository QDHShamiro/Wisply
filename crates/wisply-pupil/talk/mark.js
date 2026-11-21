let C = {};
let T = {};

const ESC = { "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" };
const FENCE = "```";
const IMAGE = "data:image/";
const SECOND_MS = 1000;

export const esc = (s) => String(s).replace(/[&<>"']/g, (ch) => ESC[ch]);
export const fill = (s, map) => String(s).replace(/\{(\w+)\}/g, (_, key) => (key in map ? map[key] : ""));
export const num = (n, digits) => Number(n).toLocaleString(C.locale, { minimumFractionDigits: digits, maximumFractionDigits: digits });

export function setup(config) {
  C = config;
  T = config.text;
}

export function size(bytes) {
  let value = Number(bytes) || 0;
  let unit = 0;
  while (value >= C.unit_step && unit < C.units.length - 1) {
    value /= C.unit_step;
    unit += 1;
  }
  return num(value, unit ? 1 : 0) + " " + C.units[unit];
}

function inline(text) {
  return text.split(/(`[^`\n]+`)/).map((part, i) => {
    if (i % 2 === 1) return "<code>" + part.slice(1, -1) + "</code>";
    return part
      .replace(/\*\*([^*\n]+)\*\*/g, "<b>$1</b>")
      .replace(/(^|[^*])\*([^*\s][^*\n]*)\*(?!\*)/g, "$1<i>$2</i>")
      .replace(/\[([^\]\n]+)\]\((https?:\/\/[^\s)]+)\)/g, '<a href="$2" target="_blank" rel="noopener noreferrer">$1</a>');
  }).join("");
}

function table(rows) {
  const cells = (row) => row.trim().replace(/^\||\|$/g, "").split("|").map((cell) => inline(esc(cell.trim())));
  const head = rows.length > 1 && /^[\s|:-]+$/.test(rows[1]);
  const body = head ? rows.slice(2) : rows;
  const top = head ? "<tr>" + cells(rows[0]).map((c) => "<th>" + c + "</th>").join("") + "</tr>" : "";
  return "<table>" + top + body.map((r) => "<tr>" + cells(r).map((c) => "<td>" + c + "</td>").join("") + "</tr>").join("") + "</table>";
}

function line(raw) {
  let hit;
  if (!raw.trim()) return { kind: "gap" };
  if ((hit = raw.match(/^(#{1,4})\s+(.*)$/))) {
    return { kind: "solo", html: '<div class="h h' + hit[1].length + '">' + inline(esc(hit[2])) + "</div>" };
  }
  if (/^\s*([-*_])\1{2,}\s*$/.test(raw)) return { kind: "solo", html: "<hr>" };
  if ((hit = raw.match(/^\s*[-*•]\s+(.*)$/))) return { kind: "ul", open: "ul", item: inline(esc(hit[1])) };
  if ((hit = raw.match(/^\s*(\d+)[.)]\s+(.*)$/))) {
    return { kind: "ol", open: 'ol start="' + Number(hit[1]) + '"', item: inline(esc(hit[2])) };
  }
  if (/^\s*\|.*\|\s*$/.test(raw)) return { kind: "rows", item: raw };
  return { kind: "p", item: inline(esc(raw)) };
}

function shut(group) {
  if (!group) return "";
  if (group.kind === "p") return "<p>" + group.items.join("<br>") + "</p>";
  if (group.kind === "rows") return table(group.items);
  return "<" + group.open + ">" + group.items.map((item) => "<li>" + item + "</li>").join("") + "</" + group.kind + ">";
}

function blocks(source) {
  let html = "";
  let group = null;
  for (const raw of source.split("\n")) {
    const found = line(raw);
    if (group && group.kind !== found.kind) {
      html += shut(group);
      group = null;
    }
    if (found.kind === "solo") {
      html += found.html;
    } else if (found.kind !== "gap") {
      group = group || { kind: found.kind, open: found.open, items: [] };
      group.items.push(found.item);
    }
  }
  return html + shut(group);
}

export function markdown(text) {
  return text.split(FENCE).map((part, i) => {
    if (i % 2 === 0) return blocks(part);
    const cut = part.indexOf("\n");
    const name = cut >= 0 ? part.slice(0, cut).trim() : "";
    const code = cut >= 0 ? part.slice(cut + 1) : part;
    return '<div class="code"><div class="codebar"><span>' + esc(name || T.code) + '</span><button data-act="code">' + esc(T.copy)
      + "</button></div><pre><code>" + esc(code.replace(/\n$/, "")) + "</code></pre></div>";
  }).join("");
}

const pill = (text, key) => '<span class="pill' + (key ? " key" : "") + '">' + esc(text) + "</span>";

function stats(m) {
  const s = m.stats;
  if (!s) return "";
  const rows = [
    [s.total, T.stat_total, 1, true],
    [s.first, T.stat_first, 1, false],
    [s.speed, T.stat_speed, 0, true],
    [s.tokens, T.stat_tokens, 0, false],
    [s.read, T.stat_read, 0, false],
    [s.cached, T.stat_cached, 0, false],
    [s.prep, T.stat_prep, 1, false],
  ];
  return rows.filter(([v]) => v).map(([v, text, digits, key]) => pill(fill(text, { v: num(v, digits) }), key)).join("");
}

function clock(m) {
  const now = Date.now();
  const parts = [fill(T.live_clock, { v: num((now - (m.began || now)) / SECOND_MS, 1) })];
  const flowing = m.firstAt ? (now - m.firstAt) / SECOND_MS : 0;
  if (m.count > 1 && flowing > C.speed_after_s) parts.push(fill(T.stat_speed, { v: num((m.count - 1) / flowing, 0) }));
  const phase = m.text ? T.phase_write : m.thought ? T.phase_think : "";
  if (phase) parts.push(phase);
  return '<span class="pill clock"><i></i>' + esc(parts.join(T.stat_join)) + "</span>";
}

const shown = (file) => Boolean(file.thumb) && file.thumb.startsWith(IMAGE);

export function chip(file, off) {
  const kind = T.kinds[file.kind] || file.kind || "";
  const face = shown(file) ? '<img src="' + esc(file.thumb) + '" alt="">' : "<i>" + esc(kind) + "</i>";
  const cross = off === undefined ? "" : '<b data-off="' + off + '">' + esc(T.remove_mark) + "</b>";
  return '<span class="chip" title="' + esc(file.name) + '">' + face + "<span>" + esc(file.name) + "</span><u>"
    + esc(file.pending ? T.uploading : size(file.bytes)) + "</u>" + cross + "</span>";
}

function sent(files) {
  const list = files || [];
  const pics = list.filter(shown);
  const rest = list.filter((file) => !shown(file));
  const row = pics.map((file) => '<img src="' + esc(file.thumb) + '" alt="" title="' + esc(file.name) + '">').join("");
  return (pics.length ? '<div class="pics">' + row + "</div>" : "")
    + (rest.length ? '<div class="chips">' + rest.map((file) => chip(file)).join("") + "</div>" : "");
}

function mine(m) {
  const lost = m.lost ? '<div class="lost">' + esc(fill(T.lost, { n: m.lost })) + "</div>" : "";
  const said = m.text ? '<div class="plain">' + esc(m.text) + "</div>" : "";
  return '<div class="msg user"><div class="bub">' + sent(m.files) + lost + said + "</div></div>";
}

function thoughtFor(m) {
  const list = m.steps || [];
  const begun = list.find((s) => s.key === "think");
  const ended = list.find((s) => s.key === "write") || list.find((s) => s.key === "done");
  if (!begun) return "";
  const until = ended ? ended.at : m.live ? (Date.now() - m.began) / SECOND_MS : null;
  return until === null ? "" : fill(T.thought_for, { v: num(until - begun.at, 1) });
}

function steps(m, open) {
  if (!C.show_steps || !m.steps || !m.steps.length) return "";
  const rows = m.steps.map((s) => '<li><span class="at">' + esc(fill(T.step_at, { v: num(s.at, 1) })) + "</span>"
    + esc(s.text || T["step_" + s.key] || "") + "</li>").join("");
  return '<details class="steps"' + (open ? " open" : "") + "><summary>" + esc(T.steps) + "</summary><ol>" + rows + "</ol></details>";
}

export function bubble(m, index, last, open) {
  if (m.role === "user") return mine(m);
  const shut = open || {};
  const thought = m.thought
    ? '<details class="thought"' + (shut.thought ? " open" : "") + "><summary>" + esc(T.thought + thoughtFor(m)) + '</summary><div class="plain">' + esc(m.thought) + "</div></details>"
    : "";
  const wait = m.live && (m.note || !m.text) ? '<div class="wait">' + esc(m.note || T.waiting) + "</div>" : "";
  const flags = [m.error ? fill(T.failed, { why: m.error }) : "", m.stopped ? T.stopped : "", m.finish === C.cut_finish ? T.cut : ""]
    .filter(Boolean).map((flag) => '<div class="flag">' + esc(flag) + "</div>").join("");
  const copy = m.text ? '<button data-act="copy" data-i="' + index + '">' + esc(T.copy) + "</button>" : "";
  const acts = m.live ? "" : copy + (last ? '<button data-act="again">' + esc(T.again) + "</button>" : "");
  return '<div class="msg bot' + (m.live ? " live" : "") + (m.live && m.text ? " typing" : "") + '"' + (m.live ? ' id="live"' : "") + '><div class="who">' + esc(T.name) + "</div>" + steps(m, shut.steps) + thought
    + '<div class="body">' + (m.text ? markdown(m.text) : "") + "</div>" + wait + flags
    + '<div class="meta">' + (m.live ? clock(m) : stats(m)) + acts + "</div></div>";
}

export function welcome() {
  return '<div class="hello"><h1>' + esc(T.hello) + "</h1><p>" + esc(T.lead) + '</p><div class="starters">'
    + C.starters.map((s) => '<button data-act="starter">' + esc(s) + "</button>").join("") + "</div></div>";
}
