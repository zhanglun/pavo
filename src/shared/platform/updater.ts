import { check } from "@tauri-apps/plugin-updater";
export async function checkForUpdates() { const update = await check(); if (update) await update.downloadAndInstall(); return Boolean(update); }
