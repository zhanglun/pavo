import { confirm, message } from "@tauri-apps/plugin-dialog";
import { relaunch } from "@tauri-apps/plugin-process";
import { check } from "@tauri-apps/plugin-updater";

let inFlight: Promise<void> | null = null;

async function runUpdateCheck() {
  try {
    const update = await check();
    if (!update) {
      await message("当前已是最新版本", { title: "Pavo", kind: "info" });
      return;
    }
    const approved = await confirm(`发现新版本 ${update.version}，是否现在安装？`, { title: "Pavo", kind: "info" });
    if (!approved) return;
    await update.downloadAndInstall();
    await relaunch();
  } catch {
    await message("检查更新失败，请稍后再试", { title: "Pavo", kind: "error" });
  }
}

export function checkForUpdates() {
  if (inFlight) return inFlight;
  inFlight = runUpdateCheck().finally(() => { inFlight = null; });
  return inFlight;
}
