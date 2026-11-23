const name = String(args.name || "").trim();
if (!name) throw new Error("find-file needs name=<part of the file or folder name>");
const root = String(args.in || "").trim().replace(/\\/g, "/").replace(/\/+$/, "");
const found = await glob(`${root ? `${root}/` : ""}**/*${name}*`);
if (!found.length) return `nothing named *${name}* under ${root || cwd}`;
const shown = found.slice(0, 40).join("\n");
return found.length > 40 ? `${shown}\n... ${found.length - 40} more (narrow it with in=<folder>)` : shown;
