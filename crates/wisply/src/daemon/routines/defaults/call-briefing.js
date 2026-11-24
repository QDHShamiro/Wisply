const flags = [];
const only = String(args.channels || "").trim();
if (only) flags.push("--channels", only);
const r = await wisply(["call", "briefing", ...flags]).nothrow();
if (r.code === 0) return r.out;
throw new Error(r.out);
