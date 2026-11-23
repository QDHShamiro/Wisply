const r = String(args.result || "");
if (!r.trim()) throw new Error("the page gave no text");
const fill = String(args.fill || "").split(";").filter((p) => p.includes("="));
if (fill.length && !r.includes(`filled ${fill.length} field`)) throw new Error("not every field was filled");
if (args.dry === "1" && String(args.submit || "").trim() && !r.includes("not clicked")) {
  throw new Error(`the dry run did not stop before the submit click: ${r.slice(0, 160)}`);
}
return true;
