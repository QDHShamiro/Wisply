const r = String(args.result || "");
if (!/^call:/.test(r)) throw new Error(`no briefing line: ${r.slice(0, 160)}`);
return true;
