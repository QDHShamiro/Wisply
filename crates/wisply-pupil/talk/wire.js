const KEY_PARAM = "k";
const KEY_HEADER = "X-Pupil-Key";
const KEY_STORE = "pupil-talk-key";
const PATH = { config: "/config", state: "/state", file: "/file", say: "/say", forget: "/forget", hear: "/hear" };
const LOCKED = 401;
const STOPPED = "AbortError";
const JSON_TYPE = "application/json";
const PICTURE = "picture";

let C = {};
let T = {};
let key = "";
let locked = false;

export const isLocked = () => locked;

function api(path, options = {}) {
  const headers = { ...(options.headers || {}), [KEY_HEADER]: key };
  return fetch(path, { ...options, headers, cache: "no-store" });
}

function post(path, body, signal) {
  return api(path, { method: "POST", headers: { "Content-Type": JSON_TYPE }, body: JSON.stringify(body), signal });
}

export function forget(chat) {
  post(PATH.forget, { chat }).catch(() => {});
}

function step(bot, key, text) {
  bot.steps = bot.steps || [];
  if (bot.steps.some((s) => s.key === key)) return;
  bot.steps.push({ key, text: text || "", at: (Date.now() - (bot.began || Date.now())) / 1000 });
}

function flow(bot) {
  bot.count = (bot.count || 0) + 1;
  bot.firstAt = bot.firstAt || Date.now();
}

const why = (failure) => [failure.message, failure.hint].filter(Boolean).join(" ").slice(0, C.error_chars);

async function alive() {
  try {
    return (await api(PATH.state)).ok;
  } catch (_) {
    return false;
  }
}

async function broke(bot) {
  bot.stats = bot.stats || { total: (Date.now() - (bot.began || Date.now())) / 1000 };
  bot.error = (await alive()) ? T.cut_off : T.unreachable;
}

async function reason(response) {
  locked = locked || response.status === LOCKED;
  try {
    return why(await response.json()) || T.unreachable;
  } catch (_) {
    return T.unreachable;
  }
}

export async function connect() {
  const given = new URLSearchParams(window.location.search).get(KEY_PARAM);
  if (given) {
    localStorage.setItem(KEY_STORE, given);
    window.history.replaceState(null, "", window.location.pathname);
  }
  key = localStorage.getItem(KEY_STORE) || "";
  try {
    const response = await api(PATH.config);
    C = response.ok ? await response.json() : null;
  } catch (_) {
    C = null;
  }
  T = C ? C.text : {};
  return C;
}

export async function look() {
  try {
    const response = await api(PATH.state);
    locked = response.status === LOCKED;
    return response.ok ? await response.json() : null;
  } catch (_) {
    return null;
  }
}

export function pairs(list) {
  const done = [];
  for (let i = 0; i + 1 < list.length; i += 1) {
    const [user, bot] = [list[i], list[i + 1]];
    if (user.role === "user" && bot.role === "assistant" && bot.text && !bot.error) {
      done.push([user.text || (user.files || []).map((file) => file.name).join(" "), bot.text]);
    }
  }
  return done;
}

function tell(bot, told) {
  bot.text = told.text || bot.text;
  bot.finish = told.finish;
  bot.stopped = Boolean(told.stopped);
  bot.stats = { total: told.total_seconds, prep: told.prep_seconds, first: told.first_seconds, speed: told.answer_per_second, tokens: told.answer_tokens, read: told.prompt_tokens, cached: told.cached_tokens };
  step(bot, "done");
}

function take(line, bot) {
  if (line.note !== undefined) bot.note = line.note;
  if (line.note) step(bot, "note:" + line.note, line.note);
  if (line.thought) {
    step(bot, "think");
    bot.thought += line.thought;
    flow(bot);
  }
  if (line.word) {
    step(bot, "write");
    bot.text += line.word;
    flow(bot);
    bot.note = "";
  }
  if (line.told) tell(bot, line.told);
  if (line.error) bot.error = why(line.error) || T.unreachable;
  return Boolean(line.told || line.error);
}

async function stream(response, bot, tick) {
  const reader = response.body.getReader();
  const decoder = new TextDecoder();
  let rest = "";
  let ended = false;
  for (;;) {
    let chunk;
    try {
      chunk = await reader.read();
    } catch (error) {
      if (ended && error.name !== STOPPED) break;
      throw error;
    }
    const { value, done } = chunk;
    rest += decoder.decode(value || new Uint8Array(), { stream: !done });
    const lines = rest.split("\n");
    rest = done ? "" : lines.pop();
    for (const line of lines) {
      if (line.trim()) {
        ended = take(JSON.parse(line), bot) || ended;
        tick();
      }
    }
    if (done) break;
  }
  if (!ended) await broke(bot);
}

export async function ask(body, signal, bot, tick) {
  try {
    const response = await post(PATH.say, body, signal);
    if (response.ok) await stream(response, bot, tick);
    else bot.error = await reason(response);
  } catch (error) {
    if (error.name === STOPPED) bot.stopped = true;
    else await broke(bot);
  }
  if (bot.error) bot.stats = bot.stats || { total: (Date.now() - (bot.began || Date.now())) / 1000 };
}

async function thumb(file) {
  try {
    const picture = await createImageBitmap(file);
    const scale = Math.min(1, C.thumb.long_side / Math.max(picture.width, picture.height));
    const canvas = document.createElement("canvas");
    canvas.width = Math.max(1, Math.round(picture.width * scale));
    canvas.height = Math.max(1, Math.round(picture.height * scale));
    const pen = canvas.getContext("2d");
    pen.fillStyle = C.thumb.matte;
    pen.fillRect(0, 0, canvas.width, canvas.height);
    pen.drawImage(picture, 0, 0, canvas.width, canvas.height);
    return canvas.toDataURL(C.thumb.type, C.thumb.quality);
  } catch (_) {
    return "";
  }
}

export function kindOf(name) {
  const ext = name.includes(".") ? name.split(".").pop().toLowerCase() : "";
  const hit = Object.entries(C.kinds).find(([, list]) => list.includes(ext));
  return hit ? hit[0] : "";
}

export async function hear(blob, now) {
  const query = "?name=" + encodeURIComponent(C.dictate.name) + (now ? "&now=" + encodeURIComponent(C.dictate.now) : "");
  try {
    const response = await api(PATH.hear + query, { method: "POST", body: blob });
    if (response.ok) return await response.json();
    locked = locked || response.status === LOCKED;
    const failure = await response.json().catch(() => ({}));
    return { error: why(failure) || T.unreachable, held: failure.error === C.dictate.held_code };
  } catch (_) {
    return { error: T.unreachable };
  }
}

export async function send(entry, file, chat) {
  const query = "?name=" + encodeURIComponent(file.name) + "&chat=" + encodeURIComponent(chat);
  const sending = api(PATH.file + query, { method: "POST", body: file, signal: entry.stop.signal });
  try {
    const [response, small] = await Promise.all([sending, entry.kind === PICTURE ? thumb(file) : ""]);
    if (!response.ok) return await reason(response);
    Object.assign(entry, await response.json(), { thumb: small, pending: false });
    return "";
  } catch (error) {
    return error.name === STOPPED ? "" : T.unreachable;
  }
}
