import { getCurrentWindow } from "@tauri-apps/api/window";
export const hideWindow = () => getCurrentWindow().hide();
