import { copyFileSync, mkdirSync, readdirSync, rmSync } from "node:fs";
if (process.argv[2] === "clean") {
  for (const f of readdirSync("dist")) if (f.endsWith(".js")) rmSync(`dist/${f}`);
  rmSync("dist/phone", { recursive: true, force: true });
} else {
  for (const f of ["index.html", "app.css", "icon.svg"]) copyFileSync(`static/${f}`, `dist/${f}`);
  mkdirSync("dist/phone", { recursive: true });
  copyFileSync("static/phone.html", "dist/phone/phone.html");
  copyFileSync("static/app.css", "dist/phone/phone.css");
  copyFileSync("static/icon.svg", "dist/phone/icon.svg");
}
