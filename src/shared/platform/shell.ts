import { open } from "@tauri-apps/plugin-shell";
export const openExternal = (url: string) => open(url);
