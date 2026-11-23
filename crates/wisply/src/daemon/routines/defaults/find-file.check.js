const r = String(args.result || "");
const name = String(args.name || "").trim().toLowerCase();
if (r.startsWith("nothing named") || /[*?{]/.test(name)) return true;
const bad = r
  .split("\n")
  .filter((l) => l && !l.startsWith("... "))
  .find((l) => !l.toLowerCase().split("/").pop().includes(name));
if (bad) throw new Error(`a found path does not match the name: ${bad}`);
return true;
