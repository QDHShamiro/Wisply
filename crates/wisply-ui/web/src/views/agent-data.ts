import { call } from "../api";

export type Status = "running" | "waiting" | "done" | "limit" | "error";
export type Agent = {
  id: string;
  name: string;
  dir_short: string;
  status: Status;
  since: number;
  last_line: string;
  tokens: number;
  cost_est: number;
  model: string;
  effort: string;
  has_result?: boolean;
  reset_at?: number;
};
export type Detail = Agent & { tail: string[]; result?: string };
export type Overview = {
  version: string;
  pc: { free_gb_c: number | null; free_ram_gb: number | null; game_mode: boolean };
  limit: { hit: boolean; reset_at: number | null };
  build: string | null;
};

export const STATUSES: Status[] = ["running", "waiting", "done", "limit", "error"];
export const TOKEN_KEY = "wisply.phone.token";
export const viaBridge = typeof window.ipc !== "undefined";

const WORDS: Record<number, string> = {
  400: "That text cannot be sent.",
  401: "Not paired. Open the pairing link again.",
  404: "This agent is gone.",
  409: "The agent is not ready for a reply.",
  429: "Too many requests, wait a second.",
  502: "Claude is not answering.",
};

function clean(out: string): string {
  return out.replace(/^err: /, "").split(" -> try:")[0];
}

type Wire = { ok: boolean; status: number; data: unknown };

async function http(method: string, path: string, body?: unknown): Promise<Wire> {
  const token = localStorage.getItem(TOKEN_KEY) ?? "";
  try {
    const res = await fetch(path, {
      method,
      headers: { Authorization: `Bearer ${token}`, ...(body === undefined ? {} : { "Content-Type": "application/json" }) },
      body: body === undefined ? undefined : JSON.stringify(body),
    });
    if (res.status === 401) {
      localStorage.removeItem(TOKEN_KEY);
      location.reload();
    }
    const text = await res.text();
    return { ok: res.ok, status: res.status, data: text ? JSON.parse(text) : null };
  } catch {
    return { ok: false, status: 0, data: null };
  }
}

async function read<T>(argv: string[], path: string): Promise<T | null> {
  if (viaBridge) {
    const r = await call(argv);
    return r.code === 0 ? (JSON.parse(r.out) as T) : null;
  }
  const r = await http("GET", path);
  return r.ok ? (r.data as T) : null;
}

export const list = () => read<Agent[]>(["agents", "--json"], "/v1/agents");
export const show = (id: string) => read<Detail>(["agents", "show", id], `/v1/agents/${id}`);
export const overview = () => read<Overview>(["agents", "status"], "/v1/status");

async function act(argv: string[], path: string, body?: string): Promise<string | null> {
  if (viaBridge) {
    const r = await call(argv, body);
    return r.code === 0 ? null : clean(r.out);
  }
  const r = await http("POST", path, body === undefined ? undefined : { text: body });
  return r.ok ? null : (WORDS[r.status] ?? "Failed.");
}

export const say = (id: string, text: string) => act(["agents", "say", id], `/v1/agents/${id}/reply`, text);
export const stop = (id: string) => act(["agents", "stop", id], `/v1/agents/${id}/stop`);

export async function pair(code: string): Promise<boolean> {
  const r = await http("POST", "/v1/pair", { code, device: "browser" });
  const token = (r.data as { token?: string } | null)?.token;
  if (!r.ok || !token) return false;
  localStorage.setItem(TOKEN_KEY, token);
  return true;
}

export function watch(onEvent: () => void): () => void {
  if (viaBridge) return () => {};
  const ctl = new AbortController();
  let alive = true;
  const run = async () => {
    while (alive) {
      try {
        const res = await fetch("/v1/events", { headers: { Authorization: `Bearer ${localStorage.getItem(TOKEN_KEY) ?? ""}` }, signal: ctl.signal });
        if (!res.ok || !res.body) throw new Error("stream");
        const reader = res.body.getReader();
        const dec = new TextDecoder();
        let buf = "";
        for (;;) {
          const { done, value } = await reader.read();
          if (done) break;
          buf += dec.decode(value, { stream: true });
          const parts = buf.split("\n\n");
          buf = parts.pop() ?? "";
          if (parts.some((p) => p.includes("data:"))) onEvent();
        }
      } catch {
        if (!alive) return;
      }
      await new Promise((r) => setTimeout(r, 3000));
    }
  };
  let release = () => {};
  const held = new Promise<void>((resolve) => (release = resolve));
  if (navigator.locks) void navigator.locks.request("wisply-agent-events", () => (alive ? run().then(() => held) : undefined));
  else void run();
  return () => {
    alive = false;
    ctl.abort();
    release();
  };
}

export function tokens(n: number): string {
  if (n >= 1_000_000) return `${(n / 1_000_000).toFixed(1)}M`;
  if (n >= 1_000) return `${(n / 1_000).toFixed(1)}k`;
  return String(n);
}

export function clock(secs: number): string {
  const s = Math.max(0, Math.floor(secs));
  const h = Math.floor(s / 3600);
  const m = Math.floor((s % 3600) / 60);
  const pad = (n: number) => String(n).padStart(2, "0");
  return h ? `${h}:${pad(m)}:${pad(s % 60)}` : `${m}:${pad(s % 60)}`;
}

export function modelLabel(a: Agent): string {
  return [a.model, a.effort].filter(Boolean).join(" · ") || "—";
}
