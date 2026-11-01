import { copyFileSync, readdirSync, rmSync } from "node:fs";
if (process.argv[2] === "clean") {
  for (const f of readdirSync("dist")) if (f.endsWith(".js")) rmSync(`dist/${f}`);
} else {
  for (const f of ["index.html", "app.css", "icon.svg"]) copyFileSync(`static/${f}`, `dist/${f}`);
}
