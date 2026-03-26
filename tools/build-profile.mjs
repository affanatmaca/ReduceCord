import { cpSync, existsSync, mkdirSync, rmSync } from "node:fs";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";

const profile = process.argv[2];
const configMap = {
  normal: "src-tauri/tauri.conf.json",
  lite: "src-tauri/tauri.lite.conf.json",
  "no-gpu": "src-tauri/tauri.no-gpu.conf.json"
};

if (!profile || !configMap[profile]) {
  console.error("Usage: node tools/build-profile.mjs <normal|lite|no-gpu>");
  process.exit(1);
}

const configPath = configMap[profile];
const tauriResult = spawnSync(
  `npm run tauri -- build --config ${configPath}`,
  { stdio: "inherit", shell: true }
);

if (tauriResult.error) {
  console.error(tauriResult.error.message);
  process.exit(1);
}

if (tauriResult.status !== 0) {
  process.exit(tauriResult.status ?? 1);
}

const distRoot = resolve("dist");
const profileRoot = resolve(distRoot, profile);
const releaseRoot = resolve("src-tauri", "target", "release");
const nsisRoot = resolve(releaseRoot, "bundle", "nsis");
const setupName = "Discord Web_0.1.0_x64-setup.exe";

if (existsSync(profileRoot)) {
  rmSync(profileRoot, { recursive: true, force: true });
}
mkdirSync(profileRoot, { recursive: true });

cpSync(resolve(releaseRoot, "discord-web.exe"), resolve(profileRoot, "discord-web.exe"));
if (existsSync(resolve(nsisRoot, setupName))) {
  cpSync(resolve(nsisRoot, setupName), resolve(profileRoot, setupName));
}

console.log(`Build artifacts copied to: ${profileRoot}`);
