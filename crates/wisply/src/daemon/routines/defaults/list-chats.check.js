const r = String(args.result || "");
if (String(args.scan || "").trim()) {
  if (!/^found \d+ · /.test(r)) throw new Error(`no scan report in the result: ${r.slice(0, 160)}`);
  return true;
}
const rows = r.split("\n").filter((l) => /^(discord|whatsapp)\t[^\t]+\t[^\t]+$/.test(l));
if (!rows.length) throw new Error(`no chat rows in the result: ${r.slice(0, 160)}`);
if (rows.some((l) => l.startsWith("whatsapp\t") && !/\t(chat|group)$/.test(l))) throw new Error("a whatsapp row has no chat/group mark");
if (rows.some((l) => l.startsWith("discord\t") && !/\t\/channels\/@me\/\d+$/.test(l))) throw new Error("a discord row has no dm link");
return true;
