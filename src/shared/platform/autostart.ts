import { isEnabled } from "@tauri-apps/plugin-autostart";

export const getAutostartEnabled = () => isEnabled();
