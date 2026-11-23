const url = String(args.url || "").trim();
if (!url) throw new Error("read-form needs url=<page>");
const dry = args.dry === "1";
const fill = String(args.fill || "").split(";").map((p) => p.trim()).filter((p) => p.includes("="));
const submit = String(args.submit || "").trim();
const find = String(args.find || "").trim();
await wisply(["open", url]);
if (fill.length) await wisply(["fill", ...fill]);
const did = [];
if (fill.length) did.push(`filled ${fill.length} field(s)`);
if (submit && dry) did.push(`dry run: "${submit}" not clicked`);
if (submit && !dry) {
  await wisply(["click", submit, ...(args.confirm === "1" ? ["--yes"] : [])]);
  await sleep(800);
  did.push(`clicked "${submit}"`);
}
const page = String(await wisply(find ? ["text", "-f", find] : ["text"]));
const cut = page.length > 3000 ? `${page.slice(0, 3000)}\n... (more: find=<words>)` : page;
return did.length ? `${did.join(", ")}\n${cut}` : cut;
