import { invoke } from "@tauri-apps/api/core";
import { writable } from "svelte/store";

export const toastMessage = writable("");
export const toastVisible = writable(false);

let hideTimer: ReturnType<typeof setTimeout> | null = null;

function show(msg: string) {
  if (hideTimer) clearTimeout(hideTimer);
  toastMessage.set(msg);
  toastVisible.set(true);
}

function showTimed(msg: string, duration = 2000) {
  show(msg);
  hideTimer = setTimeout(() => toastVisible.set(false), duration);
}

export async function setAsDesktop(url: string) {
  show("正在设置壁纸...");
  try {
    await invoke("set_as_desktop", { service: "Bing", url });
    showTimed("壁纸已设置 ✓", 2000);
  } catch {
    showTimed("设置失败，请重试", 3000);
  }
}
