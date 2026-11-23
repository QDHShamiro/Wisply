const flags = [];
const pr = String(args.pr || "").trim();
const repo = String(args.repo || "").trim();
if (pr) flags.push("--pr", pr);
if (repo) flags.push("--repo", repo);
flags.push("--timeout", String(args.timeout || "8m").trim());
const r = await wisply(["wait", "ci", ...flags]).nothrow();
if (r.code === 0 || r.out.startsWith("red:")) return r.out;
throw new Error(r.out);
