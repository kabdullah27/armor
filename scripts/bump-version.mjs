// Usage: bun run bump 1.0.16
// package.json is the single source of truth: tauri.conf.json reads it directly,
// the UI reads it at runtime via getVersion(), and this script syncs Cargo.toml/Cargo.lock.
import { readFileSync, writeFileSync } from "node:fs";
import { execSync } from "node:child_process";

const version = process.argv[2];
if (!/^\d+\.\d+\.\d+(-[\w.]+)?$/.test(version ?? "")) {
  console.error("Usage: bun run bump <x.y.z>");
  process.exit(1);
}

const pkg = JSON.parse(readFileSync("package.json", "utf8"));
pkg.version = version;
writeFileSync("package.json", JSON.stringify(pkg, null, 2) + "\n");

const cargoPath = "src-tauri/Cargo.toml";
const cargo = readFileSync(cargoPath, "utf8");
writeFileSync(cargoPath, cargo.replace(/^version = ".*"$/m, `version = "${version}"`));

// Refresh Cargo.lock without touching dependency versions
execSync("cargo update -p armor --offline", { cwd: "src-tauri", stdio: "inherit" });

console.log(`Version set to ${version}. Release with: git tag v${version} && git push origin v${version}`);
