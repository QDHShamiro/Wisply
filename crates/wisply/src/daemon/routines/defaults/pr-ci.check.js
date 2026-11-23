const r = String(args.result || "");
if (!/^(green|red):/.test(r)) throw new Error(`no CI verdict: ${r.slice(0, 160)}`);
return true;
