const r = String(args.result || "");
const app = String(args.app || "whatsapp").trim().toLowerCase();
if (args.dry === "1") {
  if (!r.includes("dry run") || r.includes("NOT cleared")) throw new Error(`the dry run did not type and clear the text: ${r.slice(0, 160)}`);
  return true;
}
if (r.includes("not visible")) throw new Error(`the message did not show up in the chat: ${r.slice(0, 160)}`);
const OUT = { whatsapp: "#main .message-out", telegram: ".bubble.is-out" };
if (!OUT[app]) return true;
const want = String(args.text || "").trim().split("\n")[0].slice(0, 40);
const last = await wisply(["js", `return [...document.querySelectorAll(${JSON.stringify(OUT[app])})].slice(-3).map((m) => m.innerText).join(" | ")`]);
if (!String(last).includes(want)) throw new Error("the last sent messages in the chat do not contain the text");
return true;
