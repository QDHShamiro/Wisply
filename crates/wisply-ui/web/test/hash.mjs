import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { transformSync } from "esbuild";

const source = readFileSync(new URL("../src/hash.ts", import.meta.url), "utf8");
const { code } = transformSync(source, { loader: "ts", format: "esm" });
const { hashParts } = await import("data:text/javascript," + encodeURIComponent(code));

assert.deepEqual(hashParts("#/"), [""]);
assert.deepEqual(hashParts("#/p/My%20Project/todos"), ["p", "My Project", "todos"]);
assert.deepEqual(hashParts("#/t/Folder/Note%20one?x=1"), ["t", "Folder", "Note one"]);
assert.deepEqual(hashParts("#/t/50%"), ["t", "50%"]);
assert.deepEqual(hashParts("#/search/100%25%20sure/%E0%A4%A"), ["search", "100% sure", "%E0%A4%A"]);
console.log("hash ok");
