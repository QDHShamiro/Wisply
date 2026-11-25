import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { transformSync } from "esbuild";

const source = readFileSync(new URL("../src/views/insight-logic.ts", import.meta.url), "utf8");
const { code } = transformSync(source, { loader: "ts", format: "esm" });
const m = await import("data:text/javascript," + encodeURIComponent(code));

const cmd = (name, runs, err_pct, avg_ms, tokens) => ({ cmd: name, runs, err: 0, err_pct, avg_ms, avg_tokens: 0, tokens, snap_after: null, waits: false });
const rows = [cmd("a", 5, 10, 900, 50), cmd("b", 9, 40, 100, 10), cmd("c", 9, 0, 4000, 900)];
assert.deepEqual(m.sortCommands(rows, "runs").map((r) => r.cmd), ["b", "c", "a"]);
assert.deepEqual(m.sortCommands(rows, "err").map((r) => r.cmd), ["b", "a", "c"]);
assert.deepEqual(m.sortCommands(rows, "slow").map((r) => r.cmd), ["c", "a", "b"]);
assert.deepEqual(m.sortCommands(rows, "tokens").map((r) => r.cmd), ["c", "a", "b"]);
assert.equal(rows[0].cmd, "a");

assert.equal(m.lapse(250), "250 ms");
assert.equal(m.lapse(1500), "1.5 s");
assert.equal(m.lapse(90000), "1.5 min");

const links = [["#/", "todos", "To-dos"], ["#/x", "extra", "Extra"], ["#/speed", "speed", "Speed"]];
const groups = m.groupLinks(links);
assert.deepEqual(groups.map((g) => g.title), ["work", "improve", "more"]);
assert.deepEqual(groups[2].links.map((l) => l[1]), ["extra"]);
assert.ok(!groups.some((g) => g.links.some((l) => l[1] === "headroom")));

const side = { badges: { todos: 3, insight: 2 } };
assert.equal(m.badgeOf("todos", side), 3);
assert.equal(m.badgeOf("improve", side), 2);
assert.equal(m.badgeOf("memory", side), 0);
assert.equal(m.badgeOf("todos", null), 0);

const e = (id, origin, status, updated) => ({ id, origin, status, updated });
const p = m.proposalsOf([e(1, "insight", "suggested", 1), e(2, "insight", "suggested", 5), e(3, "insight", "todo", 2), e(4, "insight", "dropped", 3), e(5, "user", "suggested", 9)]);
assert.deepEqual(p.waiting.map((x) => x.id), [2, 1]);
assert.deepEqual(p.taken.map((x) => x.id), [3]);
assert.equal(p.dropped, 1);

assert.ok(m.findPrompt().includes("wisply insight add"));
