import { copyFileSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
const STYLES = ["app.css", "glass.css", "agents.css"];
if (process.argv[2] === "clean") {
  for (const f of readdirSync("dist")) if (f.endsWith(".js")) rmSync(`dist/${f}`);
  rmSync("dist/phone", { recursive: true, force: true });
} else {
  for (const f of ["index.html", ...STYLES, "icon.svg"]) copyFileSync(`static/${f}`, `dist/${f}`);
  mkdirSync("dist/phone", { recursive: true });
  copyFileSync("static/phone.html", "dist/phone/phone.html");
  writeFileSync("dist/phone/phone.css", STYLES.map((f) => readFileSync(`static/${f}`, "utf8")).join(""));
  copyFileSync("static/icon.svg", "dist/phone/icon.svg");
}
